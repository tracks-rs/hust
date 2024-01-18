enum SpecialChar {
    LessThan,
    PercentageSign,
    Equal,
    Hashtag,
    GreaterThan,
    OpenBrace,
    CloseBrace
}

trait TokenAction {
    fn from_char(ch: char) -> Option<Self> where Self: Sized;
    fn is_special(ch: char) -> bool;
}

impl TokenAction for SpecialChar {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '{' => Some(SpecialChar::OpenBrace),
            '}' => Some(SpecialChar::CloseBrace),
            '%' => Some(SpecialChar::PercentageSign),
            '=' => Some(SpecialChar::Equal),
            '#' => Some(SpecialChar::Hashtag),
            '<' => Some(SpecialChar::LessThan),
            '>' => Some(SpecialChar::GreaterThan),
            _ => None,
        }
    }
    fn is_special(ch: char) -> bool {
        Self::from_char(ch).is_some()
    }
}

#[derive(Debug, PartialEq)]
pub enum HustToken {
    Html(String),   // Html content
    CodeStartSilent,// <%(no equals)
    CodeStartString,// <%=
    CodeComment,    // <%#
    // CodeEnd,        // %>
    Code(String),   // Rust code
    EOF,
}

use SpecialChar::*;
use HustToken::*;

pub fn lexer(content: &str) -> Vec<HustToken> {
    let mut tokens = Vec::new();
    let mut chars = content.chars().peekable();

    while let Some(ch) = chars.next() {
        if is_special_char(ch) {
            match from_char(ch) {
                Some(LessThan) => {
                    if let Some(&next_ch) = chars.peek() {
                        match next_ch {
                            '%' => {
                                chars.next(); // Skip '%'
                                match chars.peek() {
                                    Some(&'=') => {
                                        chars.next();
                                        tokens.push(CodeStartString);
                                    },
                                    Some(&'#') => {
                                        chars.next();
                                        tokens.push(CodeComment);
                                    },
                                    _ => tokens.push(CodeStartSilent),
                                }
                                tokens.push(collect_code(&mut chars));
                            },
                            _ => tokens.push(collect_html(&mut chars, ch)),
                        }
                    }
                },
                _ => tokens.push(collect_html(&mut chars, ch)),
            }
        } else {
            tokens.push(collect_html(&mut chars, ch));
        }
    }

    tokens.push(EOF);
    tokens
}

fn is_special_char(ch: char) -> bool {
    SpecialChar::is_special(ch)
}

fn from_char(ch: char) -> Option<SpecialChar> {
    SpecialChar::from_char(ch)
}

fn collect_code(chars: &mut std::iter::Peekable<std::str::Chars>) -> HustToken {
    let mut code = String::new();
    while let Some(ch) = chars.next() {
        if ch == '%' && chars.peek() == Some(&'>') {
            chars.next(); // Skip '>'
            break;
        }
        code.push(ch);
    }
    Code(code)
}

fn collect_html(chars: &mut std::iter::Peekable<std::str::Chars>, first_char: char) -> HustToken {
    let mut html = String::new();
    html.push(first_char);
    while let Some(&ch) = chars.peek() {
        if is_special_char(ch) && ch == '<' {
            let next_char = chars.clone().nth(1); // Cloning the iterator and getting the next character
            if next_char == Some('%') {
                break;
            }
        }
        html.push(chars.next().unwrap());
    }
    Html(html)
}

// TESTING FOR HUST LEXER

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_only() {
        let content = "<div>Hello, world!</div>";
        let tokens = lexer(content);
        assert_eq!(tokens, vec![Html(String::from("<div>Hello, world!</div>")), EOF]);
    }

    #[test]
    fn test_code_block_silent() {
        let content = "<% let x = 10; %>";
        let tokens = lexer(content);
        assert_eq!(tokens, vec![CodeStartSilent, Code(String::from(" let x = 10; ")), EOF]);
    }

    #[test]
    fn test_code_block_string() {
        let content = "<%= \"Hello, world!\" %>";
        let tokens = lexer(content);
        assert_eq!(tokens, vec![CodeStartString, Code(String::from(" \"Hello, world!\" ")), EOF]);
    }

    #[test]
    fn test_code_block_comment() {
        let content = "<%# This is a comment %>";
        let tokens = lexer(content);
        assert_eq!(tokens, vec![CodeComment, Code(String::from(" This is a comment ")), EOF]);
    }

    #[test]
    fn test_mixed_content() {
        let content = "<div><%= \"Hello\" %></div>";
        let tokens = lexer(content);
        assert_eq!(
            tokens,
            vec![
                Html(String::from("<div>")),
                CodeStartString,
                Code(String::from(" \"Hello\" ")),
                Html(String::from("</div>")),
                EOF
            ]
        );
    }

// Add more tests to cover different scenarios and edge cases
}
