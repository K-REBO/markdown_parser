pub mod render {
    use std::{fmt::format, string};

    use chrono::RoundingError;

    use crate::{BlockElm, SpanType};

    #[test]
    fn test_render() {
        let text = "# Hello";
        let asts = BlockElm::from(text.to_string());
        println!("{:?}", asts);

        let html = render(asts);
        println!("{}", html.render());
    }

    #[derive(Debug)]
    pub enum HtmlContent {
        Text(String),
        Html(Html),
        None,
    }

    #[derive(Debug)]
    pub struct Html {
        pub tag: String,
        pub child: Vec<HtmlContent>,
        pub attrs: Vec<(String, String)>,
    }

    fn render_span(spanfield: crate::SpanField) -> Html {
        let mut Html = Html::new();

        match spanfield.span_type {
            SpanType::Root => {}
            crate::SpanType::Decoration {
                text,
                is_strong,
                is_italic,
            } => {
                if is_strong {
                    Html.tag = "span".to_string();
                    Html.set_attr("class".to_string(), "bold".to_string());
                }
                if is_italic {
                    Html.tag = "span".to_string();
                    Html.set_attr("class".to_string(), "italic".to_string());
                }
                Html.append_child(HtmlContent::Text(text));
            }
            crate::SpanType::Image { alt, src } => {
                let mut img = Html::new();
                img.tag = "img".to_string();
                img.set_attr("src".to_string(), src);
                img.set_attr("alt".to_string(), alt);
                Html.append_child(HtmlContent::Html(img));
            }
            crate::SpanType::Link { text, href } => {
                let mut link = Html::new();
                link.tag = "a".to_string();
                link.set_attr("href".to_string(), href);
                link.append_child(HtmlContent::Text(text));
                Html.append_child(HtmlContent::Html(link));
            }
            crate::SpanType::InlineCode(ctx) => {
                let mut code = Html::new();
                code.tag = "code".to_string();
                code.append_child(HtmlContent::Text(ctx));
                Html.append_child(HtmlContent::Html(code));
            }
            crate::SpanType::StrikeThrough(ctx) => {
                let mut strike = Html::new();
                strike.tag = "span".to_string();
                strike.set_attr("class".to_string(), "strike-through".to_string());
                strike.append_child(HtmlContent::Text(ctx));
                Html.append_child(HtmlContent::Html(strike));
            }
            crate::SpanType::Highlight(ctx) => {
                let mut highlight = Html::new();
                highlight.tag = "span".to_string();
                highlight.set_attr("style".to_string(), "color:#FF6542".to_string());
                highlight.append_child(HtmlContent::Text(ctx));
                Html.append_child(HtmlContent::Html(highlight));
            }
            crate::SpanType::FootnoteReference { id } => todo!(),
            crate::SpanType::InlineHTML(ctx) => {
                let mut inline_html = Html::new();
                inline_html.tag = "span".to_string();
                inline_html.append_child(HtmlContent::Text(ctx));
                Html.append_child(HtmlContent::Html(inline_html));
            }
            crate::SpanType::PlainText(text) => {
                let mut plain_text = Html::new();
                plain_text.tag = "p".to_string();
                plain_text.set_attr("class".to_string(), "main-txt".to_string());
                plain_text.append_child(HtmlContent::Text(text));
                Html.append_child(HtmlContent::Html(plain_text));
            }
            _ => {}
        }
        if let Some(child) = spanfield.child {
            let child_html = render_span(*child);
            Html.append_child(HtmlContent::Html(child_html));
        }

        Html
    }

    pub fn render(asts: Vec<BlockElm>) -> Html {
        let mut html = Html::new();
        for ast in asts {
            match ast {
                BlockElm::Span(span_field) => {
                    let span_html = render_span(span_field);
                    html.append_child(HtmlContent::Html(span_html));
                }
                BlockElm::Heading { depth, text } => {
                    let mut heading = Html::new();
                    heading.tag = format!("h2");

                    heading.append_child(HtmlContent::Text(format!(
                        "{}{}",
                        "#".repeat(depth as usize),
                        text
                    )));
                    heading.set_attr("class".to_string(), "text-2xl border-spacing-2".to_string());

                    // println!("{:#?}", &heading);

                    html.append_child(HtmlContent::Html(heading));
                }
                BlockElm::OrderedList { indent_depth, span } => {
                    let mut ol = Html::new();
                    ol.tag = "ol".to_string();
                    ol.append_child(HtmlContent::Html(render_span(*span)));
                    html.append_child(HtmlContent::Html(ol));

                }
                BlockElm::UnorderedList { indent_depth, span } => {
                    let mut ul = Html::new();
                    ul.tag = "ul".to_string();
                    ul.append_child(HtmlContent::Html(render_span(*span)));
                    html.append_child(HtmlContent::Html(ul));
                }
                BlockElm::TaskList {
                    indent_depth,
                    checked,
                    span,
                } => {
                    let mut task_list = Html::new();
                    task_list.tag = "ul".to_string();
                    task_list.append_child(HtmlContent::Html(render_span(*span)));
                    html.append_child(HtmlContent::Html(task_list));

                }
                BlockElm::BlockQuote(span) => {
                    let mut block_quote = Html::new();
                    block_quote.tag = "div".to_string();
                    block_quote.set_attr("class".to_string(), "block-quote".to_string());
                    block_quote.append_child(HtmlContent::Html(render_span(*span)));
                    html.append_child(HtmlContent::Html(block_quote));
                }
                BlockElm::BlockCode {
                    lang,
                    filename,
                    code,
                } => {
                    let mut code_bg = Html::new();
                    code_bg.tag = "div".to_string();
                    code_bg.set_attr("class".to_string(), "code-bg".to_string());
                    let mut copy_button = Html::new();
                    copy_button.tag = "button".to_string();
                    copy_button.set_attr("class".to_string(), "code-copy_button".to_string());
                    copy_button.append_child(HtmlContent::Text("copy".to_string()));
                    code_bg.append_child(HtmlContent::Html(copy_button));

                    let mut code_title = Html::new();
                    code_title.tag = "div".to_string();
                    code_title.set_attr("class".to_string(), "code-title".to_string());
                    code_title.append_child(HtmlContent::Text(format!("{}", lang)));

                    if let Some(filename) = filename {
                        let mut file_name_link = Html::new();
                        file_name_link.tag = "a".to_string();
                        file_name_link.set_attr("href".to_string(), format!("#{}", filename));
                        code_title.append_child(HtmlContent::Text(format!("{}", filename)));
                    }
                    code_bg.append_child(HtmlContent::Html(code_title));

                    let mut code_ctx = Html::new();
                    code_ctx.tag = "pre".to_string();
                    code_ctx.set_attr("class".to_string(), "pt-2".to_string());
                    code_ctx.append_child(HtmlContent::Text(code));

                    code_bg.append_child(HtmlContent::Html(code_ctx));
                    html.append_child(HtmlContent::Html(code_bg));
                }
                BlockElm::FootnoteDefinition { id, span } => {}
                BlockElm::LineBreak => {
                    let mut line_break = Html::new();
                    html.append_child(HtmlContent::Text("<br/>".to_string()));
                }
                _ => {}
            }
        }

        html
    }

    impl Html {
        pub fn new() -> Html {
            Html {
                child: vec![],
                tag: "".to_string(),
                attrs: vec![],
            }
        }
        pub fn set_attr(&mut self, key: String, value: String) {
            self.attrs.retain(|(k, _)| *k != key);
            self.attrs.push((key, value));
        }
        pub fn add_attr(&mut self, key: String, value: String) {
            let mut updated_attrs = vec![];
            for (k, v) in &self.attrs {
                if *k == key {
                    updated_attrs.push((k.clone(), format!("{} {}", v, value)));
                } else {
                    updated_attrs.push((k.clone(), v.clone()));
                }
            }
            self.attrs = updated_attrs;
        }

        pub fn append_child(&mut self, child: HtmlContent) {
            self.child.push(child);
        }
        pub fn render(&self) -> String {
            let mut html = String::new();
            html.push_str(&format!("<{}", self.tag));
            for (k, v) in &self.attrs {
                html.push_str(&format!(" {}=\"{}\"", k, v));
            }
            html.push_str(">");
            for child in &self.child {
                match child {
                    HtmlContent::Text(text) => html.push_str(text),
                    HtmlContent::Html(child_html) => html.push_str(&child_html.render()),
                    HtmlContent::None => {}
                }
            }
            html.push_str(&format!("</{}>", self.tag));
            html
        }
    }
}
