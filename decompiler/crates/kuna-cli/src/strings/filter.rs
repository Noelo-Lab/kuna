//! The string filter's bounded matcher and existing pattern grammar.

/// One alternative-free element of a character class.
#[derive(Debug)]
enum ClassItem {
    Char(char),
    Range(char, char),
    Digit(bool),
    Word(bool),
    Space(bool),
}

impl ClassItem {
    fn matches(&self, c: char) -> bool {
        match self {
            ClassItem::Char(x) => *x == c,
            ClassItem::Range(lo, hi) => *lo <= c && c <= *hi,
            ClassItem::Digit(want) => c.is_ascii_digit() == *want,
            ClassItem::Word(want) => (c.is_alphanumeric() || c == '_') == *want,
            ClassItem::Space(want) => c.is_whitespace() == *want,
        }
    }
}

#[derive(Debug)]
enum Node {
    Char(char),
    Any,
    Class { neg: bool, items: Vec<ClassItem> },
    Start,
    End,
    Concat(Vec<Node>),
    Alt(Vec<Node>),
    Repeat(Repetition),
}

#[derive(Debug)]
struct Repetition {
    node: Box<Node>,
    min: u32,
    max: u32,
    greedy: bool,
}

/// A compiled `--filter` pattern.
pub(super) struct Regex {
    root: Node,
    icase: bool,
}

/// How many node visits one candidate string may cost before the match is
/// abandoned. A pathological pattern (`(a*)*b`) must not hang the command; the
/// caller is told when the budget was hit rather than being handed a silent
/// "no match".
const MATCH_BUDGET: i64 = 2_000_000;

impl Regex {
    pub(super) fn compile(pattern: &str) -> Result<Regex, String> {
        let (body, icase) = match pattern.strip_prefix("(?i)") {
            Some(rest) => (rest, true),
            None => (pattern, false),
        };
        let chars: Vec<char> = body.chars().collect();
        let mut p = Parser {
            src: &chars,
            pos: 0,
            depth: 0,
        };
        let root = p.alt()?;
        if p.pos != chars.len() {
            return Err(format!("unexpected {:?} at offset {}", chars[p.pos], p.pos));
        }
        Ok(Regex { root, icase })
    }

    /// Does the pattern match anywhere in `text`? The second element is `true`
    /// when the search ran out of backtracking budget (the answer is then a
    /// conservative "no").
    pub(super) fn is_match(&self, text: &str) -> (bool, bool) {
        let chars: Vec<char> = text.chars().collect();
        let m = Matcher {
            text: &chars,
            icase: self.icase,
            budget: std::cell::Cell::new(MATCH_BUDGET),
        };
        for start in 0..=chars.len() {
            if m.node(&self.root, start, &mut |_| true) {
                return (true, false);
            }
            if m.budget.get() <= 0 {
                return (false, true);
            }
        }
        (false, false)
    }
}

struct Matcher<'t> {
    text: &'t [char],
    /// Fold case on both sides of every character comparison (a leading `(?i)`).
    icase: bool,
    budget: std::cell::Cell<i64>,
}

impl Matcher<'_> {
    /// Does a pattern character match a text character, honoring `(?i)`?
    fn same(&self, pat: char, c: char) -> bool {
        pat == c || (self.icase && pat.to_lowercase().eq(c.to_lowercase()))
    }

    /// The forms of `c` a character class is tried against: itself, plus its
    /// other case under `(?i)`, so `[A-Z]` and `[a-z]` both match either.
    fn variants(&self, c: char) -> [char; 2] {
        if !self.icase {
            return [c, c];
        }
        [
            c.to_lowercase().next().unwrap_or(c),
            c.to_uppercase().next().unwrap_or(c),
        ]
    }

    /// Charge one node visit; `false` once the budget is gone.
    fn spend(&self) -> bool {
        let left = self.budget.get() - 1;
        self.budget.set(left);
        left > 0
    }

    /// Match `nodes` in order from `pos`, handing every end position to `k`.
    fn seq(&self, nodes: &[Node], pos: usize, k: &mut dyn FnMut(usize) -> bool) -> bool {
        match nodes.split_first() {
            None => k(pos),
            Some((head, rest)) => self.node(head, pos, &mut |p| self.seq(rest, p, k)),
        }
    }

    fn node(&self, node: &Node, pos: usize, k: &mut dyn FnMut(usize) -> bool) -> bool {
        if !self.spend() {
            return false;
        }
        match node {
            Node::Char(c) => self.text.get(pos).is_some_and(|&t| self.same(*c, t)) && k(pos + 1),
            Node::Any => pos < self.text.len() && k(pos + 1),
            Node::Class { neg, items } => match self.text.get(pos) {
                Some(&c) => {
                    let variants = self.variants(c);
                    let inside = items.iter().any(|i| variants.iter().any(|&v| i.matches(v)));
                    (inside != *neg) && k(pos + 1)
                }
                None => false,
            },
            Node::Start => pos == 0 && k(pos),
            Node::End => pos == self.text.len() && k(pos),
            Node::Concat(nodes) => self.seq(nodes, pos, k),
            Node::Alt(branches) => branches.iter().any(|b| self.node(b, pos, k)),
            Node::Repeat(repetition) => self.repeat(repetition, pos, 0, k),
        }
    }

    fn repeat(
        &self,
        repetition: &Repetition,
        pos: usize,
        count: u32,
        k: &mut dyn FnMut(usize) -> bool,
    ) -> bool {
        if !self.spend() {
            return false;
        }
        // A zero-width body would repeat forever, so one empty iteration ends the
        // loop rather than extending it.
        let more = |k: &mut dyn FnMut(usize) -> bool| {
            count < repetition.max
                && self.node(&repetition.node, pos, &mut |p| {
                    p != pos && self.repeat(repetition, p, count + 1, k)
                })
        };
        let done = |k: &mut dyn FnMut(usize) -> bool| count >= repetition.min && k(pos);
        if repetition.greedy {
            more(k) || done(k)
        } else {
            done(k) || more(k)
        }
    }
}

/// Recursive-descent parser for the supported flavor.
struct Parser<'p> {
    src: &'p [char],
    pos: usize,
    depth: u32,
}

/// Nesting cap: a hand-rolled recursive-descent parser must refuse a pattern deep
/// enough to overflow the stack rather than crash on it.
const MAX_DEPTH: u32 = 64;

impl Parser<'_> {
    fn peek(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    fn alt(&mut self) -> Result<Node, String> {
        let mut branches = vec![self.concat()?];
        while self.peek() == Some('|') {
            self.pos += 1;
            branches.push(self.concat()?);
        }
        Ok(if branches.len() == 1 {
            branches.pop().unwrap()
        } else {
            Node::Alt(branches)
        })
    }

    fn concat(&mut self) -> Result<Node, String> {
        let mut nodes = Vec::new();
        while let Some(c) = self.peek() {
            if c == '|' || c == ')' {
                break;
            }
            nodes.push(self.repeat()?);
        }
        Ok(Node::Concat(nodes))
    }

    fn repeat(&mut self) -> Result<Node, String> {
        let atom = self.atom()?;
        let (min, max) = match self.peek() {
            Some('*') => (0, u32::MAX),
            Some('+') => (1, u32::MAX),
            Some('?') => (0, 1),
            Some('{') => return self.counted(atom),
            _ => return Ok(atom),
        };
        self.pos += 1;
        Ok(self.quantified(atom, min, max))
    }

    fn quantified(&mut self, atom: Node, min: u32, max: u32) -> Node {
        let greedy = if self.peek() == Some('?') {
            self.pos += 1;
            false
        } else {
            true
        };
        Node::Repeat(Repetition {
            node: Box::new(atom),
            min,
            max,
            greedy,
        })
    }

    /// `{n}` / `{n,}` / `{n,m}`. A `{` that does not open a valid counter is a
    /// literal brace, the way an operator's shell-quoted pattern usually means it.
    fn counted(&mut self, atom: Node) -> Result<Node, String> {
        let save = self.pos;
        self.pos += 1;
        let min = match self.number() {
            Some(n) => n,
            None => {
                self.pos = save;
                return Ok(atom);
            }
        };
        let max = match self.peek() {
            Some('}') => min,
            Some(',') => {
                self.pos += 1;
                self.number().unwrap_or(u32::MAX)
            }
            _ => {
                self.pos = save;
                return Ok(atom);
            }
        };
        if self.peek() != Some('}') {
            self.pos = save;
            return Ok(atom);
        }
        self.pos += 1;
        if min > max {
            return Err(format!("{{{min},{max}}} counts down"));
        }
        Ok(self.quantified(atom, min, max))
    }

    fn number(&mut self) -> Option<u32> {
        let start = self.pos;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 1;
        }
        if self.pos == start {
            return None;
        }
        self.src[start..self.pos]
            .iter()
            .try_fold(0u32, |value, &digit| {
                value
                    .checked_mul(10)?
                    .checked_add(digit as u32 - '0' as u32)
            })
    }

    fn atom(&mut self) -> Result<Node, String> {
        let c = self
            .peek()
            .ok_or("pattern ends where an expression was expected")?;
        self.pos += 1;
        match c {
            '(' => {
                if self.depth >= MAX_DEPTH {
                    return Err("pattern nests too deeply".into());
                }
                // `(?:` is accepted and means the same thing: nothing here
                // captures, so a group is always non-capturing.
                if self.src[self.pos..].starts_with(&['?', ':']) {
                    self.pos += 2;
                } else if self.peek() == Some('?') {
                    return Err("only (?:...) groups and a leading (?i) are supported".into());
                }
                self.depth += 1;
                let inner = self.alt()?;
                self.depth -= 1;
                if self.peek() != Some(')') {
                    return Err("unclosed (".into());
                }
                self.pos += 1;
                Ok(inner)
            }
            ')' => Err("unmatched )".into()),
            '[' => self.class(),
            ']' => Ok(Node::Char(']')),
            '.' => Ok(Node::Any),
            '^' => Ok(Node::Start),
            '$' => Ok(Node::End),
            '*' | '+' | '?' => Err(format!("nothing for {c:?} to repeat")),
            '\\' => {
                let e = self.peek().ok_or("pattern ends in a backslash")?;
                self.pos += 1;
                Ok(match escape_class(e) {
                    Some(item) => Node::Class {
                        neg: false,
                        items: vec![item],
                    },
                    None => Node::Char(escape_char(e)?),
                })
            }
            other => Ok(Node::Char(other)),
        }
    }

    fn class(&mut self) -> Result<Node, String> {
        let neg = self.peek() == Some('^');
        if neg {
            self.pos += 1;
        }
        let mut items = Vec::new();
        // A `]` in first position is a literal, the POSIX convention.
        if self.peek() == Some(']') {
            self.pos += 1;
            items.push(ClassItem::Char(']'));
        }
        loop {
            let c = self.peek().ok_or("unclosed [")?;
            self.pos += 1;
            if c == ']' {
                break;
            }
            let lo = if c == '\\' {
                let e = self.peek().ok_or("pattern ends in a backslash")?;
                self.pos += 1;
                match escape_class(e) {
                    Some(item) => {
                        items.push(item);
                        continue;
                    }
                    None => escape_char(e)?,
                }
            } else {
                c
            };
            // `a-z`, but a trailing `-` before `]` is a literal hyphen.
            if self.peek() == Some('-') && self.src.get(self.pos + 1).is_some_and(|&n| n != ']') {
                self.pos += 1;
                let hi = self.peek().ok_or("unclosed [")?;
                self.pos += 1;
                let hi = if hi == '\\' {
                    let e = self.peek().ok_or("pattern ends in a backslash")?;
                    self.pos += 1;
                    escape_char(e)?
                } else {
                    hi
                };
                if hi < lo {
                    return Err(format!("range [{lo}-{hi}] counts down"));
                }
                items.push(ClassItem::Range(lo, hi));
            } else {
                items.push(ClassItem::Char(lo));
            }
        }
        if items.is_empty() {
            return Err("empty character class".into());
        }
        Ok(Node::Class { neg, items })
    }
}

/// The shorthand classes, or `None` when the escape is a literal character.
fn escape_class(e: char) -> Option<ClassItem> {
    Some(match e {
        'd' => ClassItem::Digit(true),
        'D' => ClassItem::Digit(false),
        'w' => ClassItem::Word(true),
        'W' => ClassItem::Word(false),
        's' => ClassItem::Space(true),
        'S' => ClassItem::Space(false),
        _ => return None,
    })
}

/// The literal an escape stands for. An alphanumeric escape that is not one of
/// the supported ones is REFUSED rather than read as its bare letter: `\b` means
/// a word boundary to everyone who types it, and silently matching a literal `b`
/// would answer a different question.
fn escape_char(e: char) -> Result<char, String> {
    Ok(match e {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '0' => '\0',
        c if c.is_alphanumeric() => return Err(format!("unsupported escape \\{c}")),
        other => other,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hit(pattern: &str, text: &str) -> bool {
        Regex::compile(pattern)
            .expect("pattern compiles")
            .is_match(text)
            .0
    }

    #[test]
    fn literal_and_anchors() {
        assert!(hit("Enter the", "Enter the 128-byte quantum key"));
        assert!(!hit("^the", "Enter the key"));
        assert!(hit("^Enter", "Enter the key"));
        assert!(hit("key$", "Enter the key"));
        assert!(!hit("Enter$", "Enter the key"));
    }

    #[test]
    fn classes_repeats_and_alternation() {
        assert!(hit("%[0-9]*[sd]", "value=%s"));
        assert!(hit("a+b", "aaab"));
        assert!(hit("colou?r", "color"));
        assert!(hit("colou?r", "colour"));
        assert!(hit("cat|dog", "hotdog"));
        assert!(!hit("cat|dog", "hotpony"));
        assert!(hit("\\d{3}-\\d{4}", "call 555-1234 now"));
        assert!(!hit("\\d{3}-\\d{5}", "call 555-1234 now"));
        assert!(hit("[^a-z]+", "ABC"));
        assert!(!hit("^[^a-z]+$", "AbC"));
    }

    #[test]
    fn the_serial_format_is_findable_by_its_own_syntax() {
        // The literal an operator would paste straight out of the decompilation:
        // every metacharacter escaped.
        assert!(hit("%\\[\\^-\\]", "%[^-]-%[^-]-%s"));
        // And the shape query, which must not match an ordinary format string.
        assert!(hit("%\\[\\^.\\]-%\\[\\^.\\]-%s", "%[^-]-%[^-]-%s"));
        assert!(!hit("%\\[\\^.\\]-%\\[\\^.\\]-%s", "%s-%s-%s"));
    }

    #[test]
    fn case_insensitive_prefix() {
        assert!(hit("(?i)PASSWORD", "Enter password:"));
        assert!(!hit("PASSWORD", "Enter password:"));
    }

    #[test]
    fn a_bad_pattern_is_rejected_not_reinterpreted() {
        let bad = [
            "(unclosed",
            "[a-",
            "*leading",
            "a{3,1}",
            "trailing\\",
            "(?=x)",
            "a\\b",
            "\\x41",
        ];
        for bad in bad {
            assert!(Regex::compile(bad).is_err(), "{bad:?} must not compile");
        }
        // A `{` that is not a counter stays a literal brace.
        assert!(hit("APOCALYPSE\\{", "APOCALYPSE{THE_END_OF_CRACKMES}"));
        assert!(hit("x{not a count}", "x{not a count}"));
    }

    #[test]
    fn a_pathological_pattern_gives_up_instead_of_hanging() {
        // The classic exponential blowup: every split of the a-run is retried
        // before the missing `b` fails the match.
        let re = Regex::compile("(a+)+b").expect("compiles");
        let (hit, gave_up) = re.is_match(&"a".repeat(64));
        assert!(!hit && gave_up, "the budget must stop the search");
    }

    #[test]
    fn case_folding_reaches_classes_and_ranges() {
        assert!(hit("(?i)[a-z]+", "ABC"));
        assert!(hit("(?i)[A-Z]+", "abc"));
        assert!(!hit("[A-Z]+", "abc"));
        // The negated shorthands keep their meaning under (?i).
        assert!(hit("(?i)\\D", "x"));
        assert!(!hit("(?i)^\\d+$", "abc"));
    }

    #[test]
    fn counted_numbers_preserve_value_and_cursor() {
        for (text, start, value, end) in [
            ("", 0, None, 0),
            ("0}", 0, Some(0), 1),
            ("0007,", 0, Some(7), 4),
            ("4294967295}", 0, Some(u32::MAX), 10),
            ("4294967296x", 0, None, 10),
            ("999999999999x", 0, None, 12),
            ("α42!", 1, Some(42), 3),
            ("١2", 0, None, 0),
            ("12tail", 2, None, 2),
            ("+1", 0, None, 0),
        ] {
            let chars: Vec<_> = text.chars().collect();
            let mut parser = Parser {
                src: &chars,
                pos: start,
                depth: 0,
            };
            assert_eq!((parser.number(), parser.pos), (value, end), "{text:?}");
        }
    }

    #[test]
    fn overflowing_counts_keep_the_existing_pattern_grammar() {
        assert!(hit("^a{4294967296}$", "a{4294967296}"));
        assert!(!hit("^a{4294967296}$", "aa"));
        assert!(hit("^a{2,4294967296}$", "aaaa"));
        assert!(!hit("^a{2,4294967296}$", "a"));
        assert!(hit("^a{0,0}$", ""));
        assert!(!hit("^(?:){2}$", ""));
    }
}
