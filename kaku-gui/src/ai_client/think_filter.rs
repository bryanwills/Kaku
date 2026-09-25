//! Splits inline `<think>` / `<thinking>` blocks out of streamed model text.

// ─── Inline <think> / <thinking> tag filter ─────────────────────────────────

const THINK_TAG_NAMES: &[&str] = &["thinking", "think"];

pub(super) enum ThinkSegment {
    Token(String),
    Reasoning(String),
}

pub(super) struct InlineThinkFilter {
    inside_think: bool,
    tag_name: &'static str,
    pending: String,
}

impl InlineThinkFilter {
    pub(super) fn new() -> Self {
        Self {
            inside_think: false,
            tag_name: "",
            pending: String::new(),
        }
    }

    fn find_open_tag(s: &str) -> Option<(usize, usize, &'static str)> {
        for (pos, _) in s.match_indices('<') {
            if let Some((end, name)) = parse_think_tag_at(s, pos, false, None) {
                return Some((pos, end, name));
            }
        }
        None
    }

    fn find_close_tag(s: &str, tag_name: &str) -> Option<(usize, usize)> {
        for (pos, _) in s.match_indices('<') {
            if let Some((end, _)) = parse_think_tag_at(s, pos, true, Some(tag_name)) {
                return Some((pos, end));
            }
        }
        None
    }

    fn safe_emit_len(pending: &str, closing: bool) -> usize {
        partial_think_tag_start(pending, closing).unwrap_or(pending.len())
    }

    pub(super) fn feed(&mut self, chunk: &str) -> Vec<ThinkSegment> {
        self.pending.push_str(chunk);
        let mut out = Vec::new();
        loop {
            if self.inside_think {
                if let Some((pos, end)) = Self::find_close_tag(&self.pending, self.tag_name) {
                    let reasoning = &self.pending[..pos];
                    if !reasoning.is_empty() {
                        out.push(ThinkSegment::Reasoning(reasoning.to_string()));
                    }
                    self.pending = self.pending[end..].to_string();
                    self.inside_think = false;
                } else {
                    let safe = Self::safe_emit_len(&self.pending, true);
                    if safe > 0 {
                        out.push(ThinkSegment::Reasoning(self.pending[..safe].to_string()));
                        self.pending = self.pending[safe..].to_string();
                    }
                    break;
                }
            } else if let Some((pos, end, name)) = Self::find_open_tag(&self.pending) {
                let text = &self.pending[..pos];
                if !text.is_empty() {
                    out.push(ThinkSegment::Token(text.to_string()));
                }
                self.pending = self.pending[end..].to_string();
                self.tag_name = name;
                self.inside_think = true;
            } else {
                let safe = Self::safe_emit_len(&self.pending, false);
                if safe > 0 {
                    out.push(ThinkSegment::Token(self.pending[..safe].to_string()));
                    self.pending = self.pending[safe..].to_string();
                }
                break;
            }
        }
        out
    }

    pub(super) fn flush(&mut self) -> Vec<ThinkSegment> {
        let mut out = Vec::new();
        if !self.pending.is_empty() {
            let text = std::mem::take(&mut self.pending);
            if self.inside_think {
                out.push(ThinkSegment::Reasoning(text));
            } else {
                out.push(ThinkSegment::Token(text));
            }
        }
        out
    }
}

fn parse_think_tag_at(
    s: &str,
    start: usize,
    closing: bool,
    expected_name: Option<&str>,
) -> Option<(usize, &'static str)> {
    let bytes = s.as_bytes();
    if bytes.get(start) != Some(&b'<') {
        return None;
    }

    let mut i = start + 1;
    i = skip_ascii_whitespace(bytes, i);
    if closing {
        if bytes.get(i) != Some(&b'/') {
            return None;
        }
        i += 1;
        i = skip_ascii_whitespace(bytes, i);
    } else if bytes.get(i) == Some(&b'/') {
        return None;
    }

    let (name, next) = parse_think_tag_name(bytes, i)?;
    if let Some(expected) = expected_name {
        if name != expected {
            return None;
        }
    }
    i = skip_ascii_whitespace(bytes, next);
    if bytes.get(i) != Some(&b'>') {
        return None;
    }
    Some((i + 1, name))
}

fn parse_think_tag_name(bytes: &[u8], start: usize) -> Option<(&'static str, usize)> {
    for name in THINK_TAG_NAMES {
        let raw = name.as_bytes();
        if bytes.len() < start + raw.len() {
            continue;
        }
        if bytes[start..start + raw.len()].eq_ignore_ascii_case(raw) {
            let next = start + raw.len();
            match bytes.get(next) {
                Some(b'>') | Some(b' ' | b'\t' | b'\n' | b'\r' | 0x0c) => {
                    return Some((name, next));
                }
                _ => {}
            }
        }
    }
    None
}

fn partial_think_tag_start(s: &str, closing: bool) -> Option<usize> {
    let pos = s.rfind('<')?;
    let tail = &s[pos..];
    if tail.contains('>') {
        return None;
    }

    let bytes = tail.as_bytes();
    let mut i = 1;
    i = skip_ascii_whitespace(bytes, i);
    if closing {
        match bytes.get(i) {
            None => return Some(pos),
            Some(b'/') => {
                i += 1;
                i = skip_ascii_whitespace(bytes, i);
            }
            Some(c) if c.is_ascii_whitespace() => return Some(pos),
            _ => return None,
        }
    } else {
        match bytes.get(i) {
            None => return Some(pos),
            Some(b'/') => return None,
            Some(c) if c.is_ascii_whitespace() => return Some(pos),
            _ => {}
        }
    }

    let name = &tail[i..];
    if name.is_empty() || name.as_bytes().iter().all(|b| b.is_ascii_whitespace()) {
        return Some(pos);
    }

    let trimmed = name.trim_end_matches(|c: char| c.is_ascii_whitespace());
    if name.len() != trimmed.len() {
        return THINK_TAG_NAMES
            .iter()
            .any(|tag| trimmed.eq_ignore_ascii_case(tag))
            .then_some(pos);
    }

    THINK_TAG_NAMES
        .iter()
        .any(|tag| {
            tag.as_bytes()
                .starts_with(&trimmed.to_ascii_lowercase().into_bytes())
        })
        .then_some(pos)
}

fn skip_ascii_whitespace(bytes: &[u8], mut i: usize) -> usize {
    while matches!(bytes.get(i), Some(b' ' | b'\t' | b'\n' | b'\r' | 0x0c)) {
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::{InlineThinkFilter, ThinkSegment};
    use crate::ai_client::tests::collect_segments;
    #[test]
    fn think_filter_single_block() {
        let mut f = InlineThinkFilter::new();
        let segs = f.feed("<think>reasoning</think>visible");
        let mut tokens = Vec::new();
        let mut reasoning = Vec::new();
        for s in segs {
            match s {
                ThinkSegment::Token(t) => tokens.push(t),
                ThinkSegment::Reasoning(r) => reasoning.push(r),
            }
        }
        assert_eq!(reasoning.join(""), "reasoning");
        assert_eq!(tokens.join(""), "visible");
    }

    #[test]
    fn think_filter_split_across_chunks() {
        let mut f = InlineThinkFilter::new();
        let mut tokens = Vec::new();
        let mut reasoning = Vec::new();
        let collect =
            |segs: Vec<ThinkSegment>, tokens: &mut Vec<String>, reasoning: &mut Vec<String>| {
                for s in segs {
                    match s {
                        ThinkSegment::Token(t) => tokens.push(t),
                        ThinkSegment::Reasoning(r) => reasoning.push(r),
                    }
                }
            };
        collect(f.feed("<thi"), &mut tokens, &mut reasoning);
        collect(f.feed("nk>deep thought</thi"), &mut tokens, &mut reasoning);
        collect(f.feed("nk>hello"), &mut tokens, &mut reasoning);
        collect(f.flush(), &mut tokens, &mut reasoning);
        assert_eq!(reasoning.join(""), "deep thought");
        assert_eq!(tokens.join(""), "hello");
    }

    #[test]
    fn think_filter_no_tags() {
        let mut f = InlineThinkFilter::new();
        let segs = f.feed("plain text");
        assert!(segs.iter().all(|s| matches!(s, ThinkSegment::Token(_))));
        let text: String = segs
            .into_iter()
            .map(|s| match s {
                ThinkSegment::Token(t) => t,
                _ => String::new(),
            })
            .collect();
        assert_eq!(text, "plain text");
    }

    #[test]
    fn think_filter_repeated_tags() {
        let mut f = InlineThinkFilter::new();
        let segs = f.feed("<think>a</think>x<think>b</think>y");
        let mut tokens = String::new();
        let mut reasoning = String::new();
        for s in segs {
            match s {
                ThinkSegment::Token(t) => tokens.push_str(&t),
                ThinkSegment::Reasoning(r) => reasoning.push_str(&r),
            }
        }
        assert_eq!(reasoning, "ab");
        assert_eq!(tokens, "xy");
    }

    #[test]
    fn think_filter_thinking_tags() {
        let mut f = InlineThinkFilter::new();
        let segs = f.feed("<thinking>deep</thinking>answer");
        let mut tokens = String::new();
        let mut reasoning = String::new();
        for s in segs {
            match s {
                ThinkSegment::Token(t) => tokens.push_str(&t),
                ThinkSegment::Reasoning(r) => reasoning.push_str(&r),
            }
        }
        assert_eq!(reasoning, "deep");
        assert_eq!(tokens, "answer");
    }

    #[test]
    fn think_filter_is_case_and_spacing_tolerant() {
        let mut f = InlineThinkFilter::new();
        let (tokens, reasoning) = collect_segments(f.feed("< THINKING >deep</ THINKING >answer"));
        assert_eq!(reasoning, "deep");
        assert_eq!(tokens, "answer");
    }

    #[test]
    fn think_filter_mixed_tag_variants() {
        let mut f = InlineThinkFilter::new();
        let segs = f.feed("<think>a</think>x<thinking>b</thinking>y");
        let mut tokens = String::new();
        let mut reasoning = String::new();
        for s in segs {
            match s {
                ThinkSegment::Token(t) => tokens.push_str(&t),
                ThinkSegment::Reasoning(r) => reasoning.push_str(&r),
            }
        }
        assert_eq!(reasoning, "ab");
        assert_eq!(tokens, "xy");
    }

    #[test]
    fn think_filter_thinking_split_across_chunks() {
        let mut f = InlineThinkFilter::new();
        let mut tokens = Vec::new();
        let mut reasoning = Vec::new();
        let collect =
            |segs: Vec<ThinkSegment>, tokens: &mut Vec<String>, reasoning: &mut Vec<String>| {
                for s in segs {
                    match s {
                        ThinkSegment::Token(t) => tokens.push(t),
                        ThinkSegment::Reasoning(r) => reasoning.push(r),
                    }
                }
            };
        collect(f.feed("<thinki"), &mut tokens, &mut reasoning);
        collect(f.feed("ng>reason</thinki"), &mut tokens, &mut reasoning);
        collect(f.feed("ng>visible"), &mut tokens, &mut reasoning);
        collect(f.flush(), &mut tokens, &mut reasoning);
        assert_eq!(reasoning.join(""), "reason");
        assert_eq!(tokens.join(""), "visible");
    }

    // ─── SSE rough-input rubustness ──────────────────────────────────────
    // Real providers occasionally return malformed SSE: HTML error pages
    // from CDNs, truncated chunks, empty choices arrays, comment frames.
    // The contract is: parse what we can, skip what we can't, never panic.
}
