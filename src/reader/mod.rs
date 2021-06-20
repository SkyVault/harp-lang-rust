pub mod ast;
pub mod reader;

use reader::{Lexer, Loc, Tok};

#[test]
fn reader_skip_whitespace_test() {
  let mut lexer = Lexer::new("   \n \t \r X");
  lexer.skip_whitespace();
  let chr = lexer.current_char_or('\0');
  assert_eq!(chr, 'X');
  assert_eq!('\0'.is_numeric(), false);
}

#[test]
fn reader_number_literal_test() {
  let mut lexer = Lexer::new("3.14159 -32.1 .41 -.123 #f");

  assert_eq!(Tok::Number(3.14159, Loc::blank()), lexer.next_token());
  assert_eq!(Tok::Number(-32.1, Loc::blank()), lexer.next_token());
  assert_eq!(Tok::Number(0.41, Loc::blank()), lexer.next_token());
  assert_eq!(Tok::Number(-0.123, Loc::blank()), lexer.next_token());
  assert_ne!(Tok::Number(123.0, Loc::blank()), lexer.next_token());
}

#[test]
fn reader_boolean_literal_test() {
  let mut lexer = Lexer::new(" #f #t");

  assert_eq!(Tok::Bool(false, Loc::blank()), lexer.next_token());
  assert_eq!(Tok::Bool(true, Loc::blank()), lexer.next_token());
}

#[test]
fn reader_comment_test() {
  let mut lexer = Lexer::new(
    "
    ;; This is a comment
    32 ; this is an inline comment 
    123
  ",
  );

  assert_eq!(Tok::Number(32.0, Loc::blank()), lexer.next_token());
  assert_eq!(Tok::Number(123.0, Loc::blank()), lexer.next_token());
}

#[test]
fn reader_single_character_test() {
  let mut lexer = Lexer::new(" () {} []' ");

  assert_eq!(Tok::OpenParen(Loc::blank()), lexer.next_token());
  assert_eq!(Tok::CloseParen(Loc::blank()), lexer.next_token());

  assert_eq!(Tok::OpenBrace(Loc::blank()), lexer.next_token());
  assert_eq!(Tok::CloseBrace(Loc::blank()), lexer.next_token());

  assert_eq!(Tok::OpenBracket(Loc::blank()), lexer.next_token());
  assert_eq!(Tok::CloseBracket(Loc::blank()), lexer.next_token());

  assert_eq!(Tok::Quote(Loc::blank()), lexer.next_token());
}

#[test]
fn reader_string_literal_test() {
  let mut lexer = Lexer::new(" \"Hello, World\" ");

  assert_eq!(
    Tok::Str("Hello, World".to_string(), Loc::blank()),
    lexer.next_token()
  );
}
