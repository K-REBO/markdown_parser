#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

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

[Link](https://www.example .com)

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
        let text = "ab[link](asdf)asd![this image](https://google.com)asdf".to_string();
        let span = SpanField::from(text.to_string());
        println!("{:#?}", span);
    }
}

#[derive(Debug, PartialEq, Clone)]
struct SpanField {
    child: Option<Box<SpanField>>,
    span_type: SpanType,
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
    Strong(String),
    Italic(String),
    Image { alt: String, src: String },
    Link { text: String, href: String },
    InlineCode(String),
    StrikeThrough(String),
    Highlight(String),
    FootnoteReference { id: String },
    InlineHTML(String),
    PlainText(String),
}

mod parser {

    #[derive(Debug, PartialEq)]
    struct Parsed<'a> {
        left: &'a str,
        span: &'a str,
        right: &'a str,
    }

    #[test]
    fn strip_strong_test() {
        let text = ["**strong**", "***strong***"];
        let expected_result = [
            Parsed {
                left: "",
                span: "strong",
                right: "",
            },
            Parsed {
                left: "",
                span: "*strong*",
                right: "",
            },
        ];

        for (i, t) in text.iter().enumerate() {
            // let (_, result) = strip_strong(t).unwrap();
            // assert_eq!(result, expected_result[i]);
        }
    }
}

use nom::character::complete::{char, multispace0, multispace1, none_of, space0};
use nom::{Err, IResult};

impl SpanField {
    pub fn from(text: String) -> SpanField {
        let mut left_text: String;
        let mut span_text: String;
        let mut right_text: String;

        //detect strong

        todo!()
    }

    fn append_child(&mut self, child: SpanField) -> SpanField {
        self.child = Some(Box::new(child));

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
                        span_type: SpanType::Strong(
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
                _ => {
                    // classify SpanType
                    let mut text = line.chars();
                }
            }
        }
        ast
    }
}
