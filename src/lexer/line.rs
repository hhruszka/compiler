use std::fmt;
use std::ops::RangeBounds;

pub struct Line {
    data: String,
    #[warn(dead_code)]
    start: usize,
    #[warn(dead_code)]
    current: usize,
}

impl Line {
    pub fn new(data: String) -> Self {
        Self {
            data,
            start: 0,
            current: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn advance(&mut self, n: usize) {
        if n >= self.data.len() {
            self.data = String::new();
        } else {
            self.data = self.data[n..].to_string();
        }
    }

    #[warn(dead_code)]
    fn peek(&self) -> Option<char> {
        if self.current == self.data.len() - 1 {
            None
        } else {
            self.data.chars().nth(self.current)
        }
    }

    #[warn(dead_code)]
    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.data.len() {
            None
        } else {
            self.data.chars().nth(self.current + 1)
        }
    }

    pub fn skip_whitespace(&mut self) {
        self.data = self.data.trim_start().to_string();
    }

    #[warn(dead_code)]
    fn get_token(&mut self) -> String {
        let str = self.data[self.start..self.current].to_string();
        // self.start = self.current;
        // self.current += 1;
        str.clone()
    }

    #[warn(dead_code)]
    fn as_str(&self) -> &str {
        self.data.as_str()
    }

    #[warn(dead_code)]
    fn to_string(&self) -> String {
        self.data.clone()
    }

    #[warn(dead_code)]
    fn slice_to_string(&self, range: impl RangeBounds<usize>) -> String {
        use std::ops::Bound;

        let len = self.data.len();
        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Included(&n) => n + 1,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => len,
        };
        self.data[start..end].to_string()
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}
