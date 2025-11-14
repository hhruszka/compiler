#![allow(dead_code)]
use std::fmt;
use std::ops::RangeBounds;

pub struct Line {
    data: String,
    pos: usize,
}

impl Line {
    pub fn new(data: String) -> Self {
        Self { data, pos: 0 }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn advance(&mut self, n: usize) {
        if self.pos + n >= self.data.len() {
            self.data = String::new();
        } else {
            self.data = self.data[self.pos + n..].to_string();
        }
    }

    pub fn skip_whitespace(&mut self) {
        self.data = self.data.trim_start().to_string();
    }

    pub fn remaining(&self) -> String {
        return self.data[self.pos..].to_string();
    }

    fn to_string(&self) -> String {
        self.data.to_string()
    }

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
