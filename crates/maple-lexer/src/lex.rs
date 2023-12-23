//! Token definitions

use std::str;
use error_chain::bail;
use crate::codemap::Span;
use crate::errors::*;
use error_chain::example_generated::ResultExt;
use serde::{Deserialize, Serialize};


pub enum TokenKind {
    // Values
    Integer(usize),
    Decimal(f64),
    QuotedString(String),

    // Definitions
    Identifier(String),

    Dollar, // String interpolation

    Hash, // Companion references

    Colon, // Multi-use: namespaced symbol access (::),

    Underscore, // Multi-use: numerical spacer (1_000), unused destructure, default case

    Dot, // Multi-use: property access/function calls, decimal point

    Question, // Multi-use: nullish type, throws operator
    Exclamation, // Multi-use: boolean not, throws operator

    // Brackets
    OpenParen, // Multi use: function argument lists, keyword narrowing
    CloseParen, // Multi use: function argument lists, keyword narrowing
    OpenSquare, // Arrays: array type definition, index access
    CloseSquare, // Arrays: array type definition, index access
    OpenCurly, // Multi use: blocks, complex string interpolation
    CloseCurly, // Multi use: blocks, complex string interpolation
    OpenAngle, // Multi-use: generic brackets, less than via < and <=
    CloseAngle, // Multi-use: generic brackets, greater than via > and >=, case & lambda arrow (->)

    // Operators

    // Addition, subtraction via +, ++, += or -, --, -=
    // Minus only: Case & lambda arrow (->)
    Plus, Minus,
    Slash, // Multi-use: division, comments
    Asterisk, // Multiplication
    Pipe, // Multi-use: boolean or, bitwise or, closure shadowing
    Ampersand, // Multi-use: boolean and, bitwise and

    // Multi-use: assignment (=), equality (==), or others (see other comments that include '=')
    Equals,

    EOF, // End of file
}

impl From<String> for TokenKind {
    fn from(other: String) -> TokenKind {
        TokenKind::Identifier(other)
    }
}

impl<'a> From<&'a str> for TokenKind {
    fn from(other: &'a str) -> TokenKind {
        TokenKind::Identifier(other.to_string())
    }
}

impl From<usize> for TokenKind {
    fn from(other: usize) -> TokenKind {
        TokenKind::Integer(other)
    }
}

impl From<f64> for TokenKind {
    fn from(other: f64) -> TokenKind {
        TokenKind::Decimal(other)
    }
}

/// Consumes bytes while a predicate evaluates to true.
fn take_while<F>(data: &str, mut pred: F) -> Result<(&str, usize)>
    where F: FnMut(char) -> bool
{
    let mut current_index = 0;

    for ch in data.chars() {
        let should_continue = pred(ch);

        if !should_continue {
            break;
        }

        current_index += ch.len_utf8();
    }

    if current_index == 0 {
        Err("No Matches".into())
    } else {
        Ok((&data[..current_index], current_index))
    }
}

fn tokenize_identifier(data: &str) -> Result<(TokenKind, usize)> {
    // identifiers can't start with a number
    match data.chars().next() {
        Some(ch) if ch.is_digit(10) => return bail!("Identifiers can't start with a number"),
        None => return bail!(ErrorKind::UnexpectedEOF),
        _ => {},
    }

    let (got, bytes_read) = take_while(data, |ch| ch == '_' || ch.is_alphanumeric())?;

    // TODO: Recognise keywords using a `match` statement here.

    let tok = TokenKind::Identifier(got.to_string());
    Ok((tok, bytes_read))
}

fn tokenize_number(data: &str) -> Result<(TokenKind, usize)> {
    let mut seen_dot = false;

    let (decimal, bytes_read) = take_while(data, |c| {
        if c.is_digit(10) {
            true
        } else if c == '.' {
            if !seen_dot {
                seen_dot = true;
                true
            } else {
                false
            }
        } else {
            false
        }
    })?;

    if seen_dot {
        let n: f64 = decimal.parse()?;
        Ok((TokenKind::Decimal(n), bytes_read))
    } else {
        let n: usize = decimal.parse()?;
        Ok((TokenKind::Integer(n), bytes_read))

    }
}

fn skip_whitespace(data: &str) -> usize {
    match take_while(data, |ch| ch.is_whitespace()) {
        Ok((_, bytes_skipped)) => bytes_skipped,
        _ => 0,
    }
}

fn skip_comments(src: &str) -> usize {
    let pairs = [("//", "\n"), ("/*", "*/")];

    for &(pattern, matcher) in &pairs {
        if src.starts_with(pattern) {
            let leftovers = skip_until(src, matcher);
            return src.len() - leftovers.len();
        }
    }

    0
}

fn skip_until<'a>(mut src: &'a str, pattern: &str) -> &'a str {
    while !src.is_empty() && !src.starts_with(pattern) {
        let next_char_size = src.chars().next().expect("The string isn't empty").len_utf8();
        src = &src[next_char_size..];
    }

    &src[pattern.len()..]
}

fn skip(src: &str) -> usize {
    let mut remaining = src;

    loop {
        let ws = skip_whitespace(remaining);
        remaining = &remaining[ws..];
        let comments = skip_comments(remaining);
        remaining = &remaining[comments..];

        if ws + comments == 0 {
            return src.len() - remaining.len();
        }
    }
}

/// Try to lex a single token from the input stream.
pub fn tokenize_single_token(data: &str) -> Result<(TokenKind, usize)> {
    let next = match data.chars().next() {
        Some(c) => c,
        None => bail!(ErrorKind::UnexpectedEOF),
    };

    let (tok, length) = match next {
        '.' => (TokenKind::Dot, 1),
        '=' => (TokenKind::Equals, 1),
        '+' => (TokenKind::Plus, 1),
        '-' => (TokenKind::Minus, 1),
        '*' => (TokenKind::Asterisk, 1),
        '/' => (TokenKind::Slash, 1),
        '|' => (TokenKind::Pipe, 1),
        '_' => (TokenKind::Underscore, 1),
        '$' => (TokenKind::Dollar, 1),
        ':' => (TokenKind::Colon, 1),
        '#' => (TokenKind::Hash, 1),
        '&' => (TokenKind::Ampersand, 1),
        '(' => (TokenKind::OpenParen, 1),
        ')' => (TokenKind::CloseParen, 1),
        '[' => (TokenKind::OpenSquare, 1),
        ']' => (TokenKind::CloseSquare, 1),
        '{' => (TokenKind::OpenCurly, 1),
        '}' => (TokenKind::CloseCurly, 1),
        '<' => (TokenKind::OpenAngle, 1),
        '>' => (TokenKind::CloseAngle, 1),
        '!' => (TokenKind::Exclamation, 1),
        '?' => (TokenKind::Question, 1),
        '0' ... '9' => tokenize_number(data).chain_err(|| "Couldn't tokenize a number")?,
        c @ '_' | c if c.is_alphabetic() => tokenize_identifier(data)
            .chain_err(|| "Couldn't tokenize an identifier")?,
        other => bail!(ErrorKind::UnknownCharacter(other)),
    };

    Ok((tok, length))
}

struct Tokenizer<'a> {
    current_index: usize,
    remaining_text: &'a str,
}

impl<'a> Tokenizer<'a> {
    fn new(src: &str) -> Tokenizer {
        Tokenizer {
            current_index: 0,
            remaining_text: src,
        }
    }

    fn next_token(&mut self) -> Result<Option<(TokenKind, usize, usize)>> {
        self.skip_whitespace();

        if self.remaining_text.is_empty() {
            Ok(None)
        } else {
            let start = self.current_index;
            let tok = self._next_token()
                .chain_err(|| ErrorKind::MessageWithLocation(self.current_index,
                                                             "Couldn't read the next token"))?;
            let end = self.current_index;
            Ok(Some((tok, start, end)))
        }
    }

    fn skip_whitespace(&mut self) {
        let skipped = skip(self.remaining_text);
        self.chomp(skipped);
    }

    fn _next_token(&mut self) -> Result<TokenKind> {
        let (tok, bytes_read) = tokenize_single_token(self.remaining_text)?;
        self.chomp(bytes_read);

        Ok(tok)
    }

    fn chomp(&mut self, num_bytes: usize) {
        self.remaining_text = &self.remaining_text[num_bytes..];
        self.current_index += num_bytes;
    }
}

/// Turn a string of valid Delphi code into a list of tokens, including the
/// location of that token's start and end point in the original source code.
///
/// Note the token indices represent the half-open interval `[start, end)`,
/// equivalent to `start .. end` in Rust.
pub fn tokenize(src: &str) -> Result<Vec<(TokenKind, usize, usize)>> {
    let mut tokenizer = Tokenizer::new(src);
    let mut tokens = Vec::new();

    while let Some(tok) = tokenizer.next_token()? {
        tokens.push(tok);
    }

    Ok(tokens)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Token {
    /// The token's location relative to the rest of the files being
    /// processed.
    pub span: Span,
    /// What kind of token is this?
    pub kind: TokenKind,
}

impl Token {
    /// Create a new token out of a `Span` and something which can be turned
    /// into a `TokenKind`.
    pub fn new<K: Into<TokenKind>>(span: Span, kind: K) -> Token {
        let kind = kind.into();
        Token { span, kind }
    }
}

impl<T> From<T> for Token
    where T: Into<TokenKind> {
    fn from(other: T) -> Token {
        Token::new(Span::dummy(), other)
    }
}


#[cfg(test)]
mod test {
    use crate::errors::ErrorKind;
    use crate::lex::{skip_whitespace, tokenize, tokenize_identifier, TokenKind};
    macro_rules! lexer_test {
        (FAIL: $name:ident, $func:ident, $src:expr) => {
            #[cfg(test)]
            #[test]
            fn $name() {
                let src: &str = $src;
                let func = $func;

                let got = func(src);
                assert!(got.is_err(), "{:?} should be an error", got);
            }
        };
        ($name:ident, $func:ident, $src:expr => $should_be:expr) => {
            #[cfg(test)]
            #[test]
            fn $name() {
                let src: &str = $src;
                let should_be = TokenKind::from($should_be);
                let func = $func;

                let (got, _bytes_read) = func(src).unwrap();
                assert_eq!(got, should_be, "Input was {:?}", src);
            }
        };
    }

    lexer_test!(tokenize_a_single_letter, tokenize_identifier, "F" => "F");
    lexer_test!(tokenize_an_identifer, tokenize_identifier, "Foo" => "Foo");
    lexer_test!(tokenize_ident_containing_an_underscore, tokenize_identifier, "Foo_bar" => "Foo_bar");
    lexer_test!(FAIL: tokenize_ident_cant_start_with_number, tokenize_identifier, "7Foo_bar");
    lexer_test!(FAIL: tokenize_ident_cant_start_with_dot, tokenize_identifier, ".Foo_bar");

    #[test]
    fn skip_past_several_whitespace_chars() {
        let src = " \t\n\r123";
        let should_be = 4;

        let num_skipped = skip_whitespace(src);
        assert_eq!(num_skipped, should_be);
    }

    #[test]
    fn skipping_whitespace_when_first_is_a_letter_returns_zero() {
        let src = "Hello World";
        let should_be = 0;

        let num_skipped = skip_whitespace(src);
        assert_eq!(num_skipped, should_be);
    }

    macro_rules! comment_test {
        ($name:ident, $src:expr => $should_be:expr) => {
            #[cfg(test)]
            #[test]
            fn $name() {
                let got = skip_comments($src);
                assert_eq!(got, $should_be);
            }
        }
    }

    comment_test!(slash_slash_skips_to_end_of_line, "// foo bar { baz }\n 1234" => 19);
    comment_test!(comment_skip_curly_braces, "{ baz \n 1234} hello wor\nld" => 13);
    comment_test!(comment_skip_round_brackets, "(* Hello World *) asd" => 17);
    comment_test!(comment_skip_ignores_alphanumeric, "123 hello world" => 0);
    comment_test!(comment_skip_ignores_whitespace, "   (* *) 123 hello world" => 0);

    #[cfg(test)]
    #[test]
    fn tokenize_a_basic_expression() {
        let src = "foo = 1 + 2.34";
        let should_be = vec![
            (TokenKind::from("foo"), 0, 3),
            (TokenKind::Equals, 4, 5),
            (TokenKind::from(1), 6, 7),
            (TokenKind::Plus, 8, 9),
            (TokenKind::from(2.34), 10, 14),
        ];

        let got = tokenize(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[cfg(test)]
    #[test]
    fn tokenizer_detects_invalid_stuff() {
        let src = "foo bar `%^&\\";
        let index_of_backtick = 8;

        let err = tokenize(src).unwrap_err();
        match err.kind() {
            &ErrorKind::MessageWithLocation(loc, _) => assert_eq!(loc, index_of_backtick),
            other => panic!("Unexpected error: {}", other),
        }
    }
}

