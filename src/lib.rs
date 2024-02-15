#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

mod parser;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let readed_file_ctx = std::fs::read_to_string("Cargo.toml").unwrap();
        let readed_file_ctx = r#"# Hello World
## 2nd
### 3rd
#### 4th
> Quote
Quote2

[Link](https://www.example.com)

**Strong**
![Image](https://www.example.com)


Plain

Plain2"#
            .to_string();

        let ast = BlockElm::from(readed_file_ctx);
        println!("{:#?}", ast);

        // let html = Ast::to_html(ast);
        // print!("{}", html);
    }

    #[test]
    fn is_work_span_field_from() {
        let text =
            "!Hello [aa](asdf) ![alt](https://img.example.com)[aa](bb)[aa](asdf)".to_string();
        let span = SpanField::from(text.to_string());
        println!("{:#?}", span);
    }

    #[test]
    fn append_child_test() {
        let mut span = SpanField {
            child: None,
            span_type: SpanType::Root,
        };
        let child = SpanField {
            child: None,
            span_type: SpanType::PlainText("strong".to_string()),
        };
        let child2 = SpanField {
            child: None,
            span_type: SpanType::Link {
                text: "text".to_string(),
                href: "href".to_string(),
            },
        };
        span.append_child(child);
        span.append_child(child2);

        let result = SpanField {
            span_type: SpanType::Root,
            child: Some(Box::new(SpanField {
                span_type: SpanType::PlainText("strong".to_string()),
                child: Some(Box::new(SpanField {
                    child: None,
                    span_type: SpanType::Link {
                        text: "text".to_string(),
                        href: "href".to_string(),
                    },
                })),
            })),
        };

        println!("{:#?}", span);

        assert_eq!(span, result);
    }
}

#[derive(Debug, PartialEq, Clone)]
struct SpanField {
    span_type: SpanType,
    child: Option<Box<SpanField>>,
}

#[derive(Debug, PartialEq)]
enum BlockElm {
    Span(SpanField),
    Heading {
        depth: u8,
        text: String,
    },

    OrderedList {
        indent_depth: u8,
        span: Box<SpanField>,
    },
    UnorderedList {
        indent_depth: u8,
        span: Box<SpanField>,
    },
    TaskList {
        indent_depth: u8,
        checked: bool,
        span: Box<SpanField>,
    },

    BlockQuote(Box<SpanField>), //TODO! nested block quote
    BlockCode {
        lang: String,
        filename: Option<String>,
        code: String,
    },

    FootnoteDefinition {
        id: String,
        span: Box<SpanField>,
    },
    Table {
        header: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    Nothing,
}

#[derive(Debug, PartialEq, Clone)]
enum SpanType {
    Root,
    Decoration {
        // ✅
        text: String,
        is_strong: bool,
        is_italic: bool,
    },
    Image {
        alt: String,
        src: String,
    }, // ✅
    Link {
        text: String,
        href: String,
    }, // ✅
    InlineCode(String),    // 🚧
    StrikeThrough(String), // ✅
    Highlight(String),     // ✅
    FootnoteReference {
        id: String,
    }, // 🚧
    InlineHTML(String),    // 🦺
    PlainText(String),     // ✅
}

use crate::parser::parser::Parsed;
use nom::character::complete::{char, multispace0, multispace1, none_of, space0};
use nom::{Err, IResult};

impl SpanField {
    pub fn from(text: String) -> SpanField {
        let mut root_span = SpanField {
            span_type: SpanType::Root,
            child: None,
        };

        /*
         !TODO ここの処理一つにまとめれれる気がする
         if let Ok() .......の繰り返しだから、関数を配列に入れて、forで回すとか
        */
        if text == "" {
        } else if let Ok((input, parsed)) = parser::parser::parse_image(&text) {
            let Parsed { left, span, right } = parsed;
            let left = SpanField::from(left);
            let right = SpanField::from(right);

            root_span.append_child(left);
            root_span.append_child(span);
            root_span.append_child(right);
        } else if let Ok((input, parsed)) = parser::parser::parse_link(&text) {
            let Parsed { left, span, right } = parsed;
            let left = SpanField::from(left);
            let right = SpanField::from(right);

            root_span.append_child(left);
            root_span.append_child(span);
            root_span.append_child(right);
        } else if let Ok((input, parsed)) = parser::parser::parse_strikethrough(&text) {
            let Parsed { left, span, right } = parsed;
            let left = SpanField::from(left);
            let right = SpanField::from(right);

            root_span.append_child(left);
            root_span.append_child(span);
            root_span.append_child(right);
        } else if let Ok((input, parsed)) = parser::parser::parse_highlight(&text) {
            let Parsed { left, span, right } = parsed;
            let left = SpanField::from(left);
            let right = SpanField::from(right);

            root_span.append_child(left);
            root_span.append_child(span);
            root_span.append_child(right);
        } else if let Ok((input, parsed)) = parser::parser::parse_strong_italic(&text) {
            let Parsed { left, span, right } = parsed;
            let left = SpanField::from(left);
            let right = SpanField::from(right);

            root_span.append_child(left);
            root_span.append_child(span);
            root_span.append_child(right);
        } else if let Ok((input, parsed)) = parser::parser::parse_strong(&text) {
            let Parsed { left, span, right } = parsed;
            let left = SpanField::from(left);
            let right = SpanField::from(right);

            root_span.append_child(left);
            root_span.append_child(span);
            root_span.append_child(right);
        } else if let Ok((input, parsed)) = parser::parser::parse_italic(&text) {
            let Parsed { left, span, right } = parsed;
            let left = SpanField::from(left);
            let right = SpanField::from(right);

            root_span.append_child(left);
            root_span.append_child(span);
            root_span.append_child(right);
        } else if let Ok((input, parsed)) = parser::parser::parse_inline_code(&text) {
            let Parsed { left, span, right } = parsed;
            let left = SpanField::from(left);
            let right = SpanField::from(right);

            root_span.append_child(left);
            root_span.append_child(span);
            root_span.append_child(right);
        } else if let Ok((input, parsed)) = parser::parser::parse_footnote_ref(&text) {
            let Parsed { left, span, right } = parsed;
            let left = SpanField::from(left);
            let right = SpanField::from(right);

            root_span.append_child(left);
            root_span.append_child(span);
            root_span.append_child(right);
        } else {
            let span = SpanField {
                span_type: SpanType::PlainText(text),
                child: None,
            };
            root_span.append_child(span);
        }

        return root_span.strip_root();
    }
    fn strip_root(&mut self) -> SpanField {
        match &mut self.child {
            Some(c) => return *self.clone().child.unwrap(),
            None => {
                return self.clone();
            }
        }
    }

    fn append_child(&mut self, child: SpanField) -> SpanField {
        match &mut self.child {
            Some(c) => {
                self.child.as_mut().unwrap().append_child(child);
            }
            None => {
                /*
                SpanField::fromの特性上、root_spanを使ってしまうことは仕方ない
                けど、SpanType::RootがSpanField::fromを再帰的に呼び出す度に、
                生まれるとツリー(SpanField)が見づらくなるので、もしChildがRootだったら剥く
                ところで、荻窪に行きたい
                 */
                if child.span_type == SpanType::Root {
                    self.child = child.child;
                } else {
                    self.child = Some(Box::new(child));
                }
            }
        }
        self.clone()
    }
}

impl BlockElm {
    fn from(text: String) -> Vec<BlockElm> {
        let mut ast: Vec<BlockElm> = Vec::new();

        let lines = text.lines();

        // Block regex
        let ordered_list_regex = regex::Regex::new(r"^\d+\. ").unwrap();
        let unordered_list_regex = regex::Regex::new(r"^\* ").unwrap();
        let task_list_regex = regex::Regex::new(r"^\d+\. \[x\] ").unwrap();
        let footnote_definition_regex = regex::Regex::new(r"\[\^.*\]: ").unwrap();

        for line in lines {
            // let mut block_type: Option<BlockElm> = None;
            #[derive(Debug, PartialEq)]
            enum BlockType {
                Heading,
                OrderedList,
                UnorderedList,
                TaskList,
                BlockQuote,
                BlockCode,
                FootnoteDefinition,
                Table,
                Nothing,
            }

            let mut previous_block_type: BlockType = BlockType::Nothing;

            // classify BlockElm
            match line {
                _ if line.starts_with("#") => {
                    let depth = line.chars().take_while(|c| *c == '#').count() as u8;
                    let text = line.chars().skip_while(|c| *c == '#').collect();
                    ast.push(BlockElm::Heading { depth, text });
                    previous_block_type = BlockType::Heading;
                }
                _ if ordered_list_regex.is_match(line) => {
                    let indent_depth = line.chars().take_while(|c| *c == ' ').count() as u8;
                    let span = Box::new(SpanField::from(line.to_string()));
                    ast.push(BlockElm::OrderedList { indent_depth, span });
                    previous_block_type = BlockType::OrderedList;
                }
                _ if unordered_list_regex.is_match(line) => {
                    let indent_depth = line.chars().take_while(|c| *c == ' ').count() as u8;
                    let span = Box::new(SpanField {
                        child: None,
                        span_type: SpanType::PlainText(
                            line.chars().skip_while(|c| *c == ' ').collect(),
                        ),
                    });
                    ast.push(BlockElm::UnorderedList { indent_depth, span });
                    previous_block_type = BlockType::UnorderedList;
                }
                _ if line.starts_with("> ") => {
                    let span = Box::new(SpanField::from(line.to_string().replace("> ", "")));
                    ast.push(BlockElm::BlockQuote(span));
                    previous_block_type = BlockType::BlockQuote;
                }
                _ if line.is_empty() => {
                    ast.push(BlockElm::Nothing);
                }
                _ => {
                    // classify SpanType
                    let span = SpanField::from(line.to_string());
                    ast.push(BlockElm::Span(span));
                }
            }
        }
        ast
    }
}
