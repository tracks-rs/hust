use HustToken::*;
use crate::lexer::lexer;
use crate::lexer::HustToken;

pub fn parse(content: &str) -> String {
    let tokens = lexer(content);
    let mut output_buffer = String::from("let mut output_buffer = String::new();\n");
    let mut previous_token;
    let mut this_token = None;

    for token in &tokens {
        previous_token = this_token;
        this_token = Some(token);
        match token {
            Html(html_content) => {
                let escaped_html = escape_html(&html_content);
                output_buffer.push_str(&format!("output_buffer.push_str({:?});\n", escaped_html));
            },
            CodeStartSilent | CodeStartString | CodeComment => {
                // No explicit action required for these tokens
                // They just signify the type of code block following them
            },
            Code(rust_code) => {
                // For CodeStartString, the Rust code is to be treated as a string expression
                // For CodeStartSilent, the Rust code is to be executed but not appended to the buffer
                if let Some(prev_token) = previous_token { //tokens.iter().rev().next() {
                    match prev_token {
                        CodeStartString => {
                            output_buffer.push_str(&format!("output_buffer.push_str({});\n", rust_code));
                        },
                        CodeStartSilent => {
                            output_buffer.push_str(&format!("{}\n", rust_code));
                        },
                        CodeComment => {
                            // Do nothing
                        },
                        _ => { // Should not happen.
                            output_buffer.push_str(&format!("{}\n", rust_code));
                        }
                    }
                }
            },
            EOF => {
                output_buffer.push_str("output_buffer");
            }
        }
    }
    
    match syn::parse_str::<crate::TokenStream>(&output_buffer) {
        Ok(_parsed_code) => {
        }
        Err(err) => {
            println!("HUST Parsing Error: {} on code rust_code: {}", err, output_buffer);
        }
    }
    output_buffer
}

fn escape_html(html: &str) -> String {
    html.replace("\"", "\"")
}

