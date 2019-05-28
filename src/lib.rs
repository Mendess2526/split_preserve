//! This module provides an iterator over strings that splits on whitespace
//! but doesn't throw the whitespace away, like the version in
//! [std](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace)
//! does.

//! An iterator over the whitespace and non-whitespace sub-strings of a string, separated by any
//! amount of whitespace.
pub struct SplitPreserveWS<'a> {
    string: Option<Token<'a>>,
}

/// The token returned by the `SplitPreserveWS` iterator. It can be either
/// `Whitespace` or `Other`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token<'a> {
    Whitespace(&'a str),
    Other(&'a str),
}

impl<'a> SplitPreserveWS<'a> {
    /// Splits a string slice by whitespace.
    ///
    /// The iterator returned will return string slices that are sub-slices of the original string
    /// slice, annotated as `Whitespace` or `Other` using the `Token` enum.
    ///
    /// 'Whitespace' is defined according to the terms of the Unicode Derived Core Property
    /// `White_Space`.
    ///
    /// ```rust
    /// use split_preserve::{SplitPreserveWS, Token};
    ///
    /// assert_eq!(SplitPreserveWS::new("aa  ").next(), Some(Token::Other("aa")))
    /// ```
    pub fn new(string: &'a str) -> Self {
        if string.is_empty() {
            Self { string: None }
        } else if string.starts_with(char::is_whitespace) {
            Self {
                string: Some(Token::Whitespace(string)),
            }
        } else {
            Self {
                string: Some(Token::Other(string)),
            }
        }
    }

    /// Maps over the `Token::Other` elements of the iterator.
    ///
    /// This will allocate a new string for each of the tokens in the iterator
    ///
    /// ```rust
    /// use split_preserve::{SplitPreserveWS, Token};
    ///
    /// assert_eq!(
    ///     SplitPreserveWS::new("Line\twith\nweird whitespace")
    ///         .map_words(|f| f.chars().rev().collect::<String>())
    ///         .collect::<String>(),
    ///     "eniL\thtiw\ndriew ecapsetihw"
    /// )
    /// ```
    pub fn map_words<S>(self, mut f: S) -> std::iter::Map<Self, impl FnMut(Token<'a>) -> String>
    where
        S: FnMut(&str) -> String,
    {
        self.map(move |t: Token<'a>| match t {
            Token::Other(s) => f(s),
            Token::Whitespace(s) => s.to_string(),
        })
    }

    /// Maps over the `Token::Whitespace` elements of the iterator.
    ///
    /// This will allocate a new string for each of the tokens in the iterator
    ///
    /// ```rust
    /// use split_preserve::{SplitPreserveWS, Token};
    ///
    /// assert_eq!(
    ///     SplitPreserveWS::new("Line\twith\nweird whitespace")
    ///         .map_whitespace(|_| String::from(" "))
    ///         .collect::<String>(),
    ///     "Line with weird whitespace"
    /// )
    /// ```
    pub fn map_whitespace<S>(
        self,
        mut f: S,
    ) -> std::iter::Map<Self, impl FnMut(Token<'a>) -> String>
    where
        S: FnMut(&str) -> String,
    {
        self.map(move |t: Token<'a>| match t {
            Token::Other(s) => s.to_string(),
            Token::Whitespace(s) => f(s),
        })
    }
}

impl<'a> Iterator for SplitPreserveWS<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.string.take().map(|t| match t {
            Token::Whitespace(s) => {
                let (token, rest) = match s.find(|c: char| !c.is_whitespace()) {
                    Some(i) => {
                        let (a, b) = s.split_at(i);
                        (a, Some(Token::Other(b)))
                    }
                    None => (s, None),
                };
                self.string = rest;
                Token::Whitespace(token)
            }
            Token::Other(s) => {
                let (token, rest) = match s.find(char::is_whitespace) {
                    Some(i) => {
                        let (a, b) = s.split_at(i);
                        (a, Some(Token::Whitespace(b)))
                    }
                    None => (s, None),
                };
                self.string = rest;
                Token::Other(token)
            }
        })
    }
}
