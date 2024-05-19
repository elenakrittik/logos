/// Test against issue #265 on GitHub and duplicates.
use logos_derive::Logos;
use tests::assert_lex;

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
#[logos(skip " ")]
pub enum Token {
    #[regex(r"[a-zA-Z][a-zA-Z0-9]*")] // p = ...
    Word,
    #[regex("not in", priority = 12)] // p = 12
    NotIn,
    #[regex("not", priority = 13)] // p = 6
    Not,
}

#[test]
fn test_not_info() {
    assert_lex(
        "not info",
        &[
            (Ok(Token::Not), "not", 0..3),
            (Ok(Token::Word), "info", 4..8),
        ],
    );
}
