mod blog {
    use std::fmt::Result;
    use std::path::Path;

    use chrono::NaiveDate;

    use crate::render::render::{Html, HtmlContent};
    use crate::{render, BlockElm};

    struct Blog {
        title: String,
        content: Vec<BlockElm>,
        created: chrono::NaiveDate,
        modified: chrono::NaiveDate,
        tags: Vec<String>,
    }

    #[test]
    fn test_blog_build() {
        let result = NaiveDate::parse_from_str("1970-01-01", "%Y-%m-%d");

        let md_path = Path::new("./stuff/first_blog.md");
        let html_path = Path::new("./test.html");

        let mut blog = Blog::from(md_path);
        // println!("{}",blog.build().render());

        blog.write(html_path);
    }

    //from
    impl Blog {
        pub fn write(&mut self, output_path: &Path) {
            let html = self.build().render();
            let html = html.replace("<>", "");
            let html = html.replace("</>", "");

            let mut file = std::fs::File::create(output_path).unwrap();
            std::io::Write::write_all(&mut file, html.as_bytes()).unwrap();
        }

        pub fn from(path: &Path) -> Self {
            let mut blog: Blog = Blog {
                title: "".to_string(),
                content: vec![],
                created: NaiveDate::parse_from_str("1970-01-01", "%Y-%m-%d").unwrap(),
                modified: NaiveDate::parse_from_str("1970-01-01", "%Y-%m-%d").unwrap(),
                tags: vec![],
            };

            let file = std::fs::read_to_string(path).unwrap();
            let lines = file.lines().map(|s| s.to_string()).collect::<Vec<String>>();
            let mut yamls: Vec<String> = vec![];
            let mut lines_not_yaml = String::new();

            let mut in_yaml = false;
            for line in lines.iter() {
                if line == "---" {
                    in_yaml = !in_yaml;
                    continue;
                } else if in_yaml == true {
                    yamls.push(line.clone());
                } else {
                    lines_not_yaml += format!("{}\n", line).as_str();
                }
            }

            for yaml in yamls {
                let mut key_value = yaml.split(":").collect::<Vec<&str>>();
                let key = key_value[0].trim();
                let value = key_value[1].trim();

                match key {
                    "title" => {
                        blog.title = value.to_string();
                    }
                    "created" => {
                        blog.created = NaiveDate::parse_from_str(value, "\"%Y-%m-%d\"").unwrap();
                    }
                    "modified" => {
                        blog.modified = NaiveDate::parse_from_str(value, "\"%Y-%m-%d\"").unwrap();
                    }
                    "tags" => {
                        blog.tags = value
                            .replace("[", "")
                            .replace("]", "")
                            .replace("\"", "")
                            .split(",")
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<String>>();
                    }
                    _ => {}
                }
            }

            blog.content = BlockElm::from(lines_not_yaml);

            blog
        }
        pub fn build(&mut self) -> Html {
            let mut html = Html::new();

            html.append_child(HtmlContent::Html(self.render_head()));

            let mut body = self.render_base();
            let mut wrapper = Html::new();
            wrapper.tag = "div".to_string();
            wrapper.set_attr("class".to_string(), "mx-40".to_string());

            let mut metadata = Html::new();

            let mut title = Html::new();

            title.tag = "h1".to_string();
            title.set_attr(
                "class".to_string(),
                "text-3xl font-bold text-center sourceCodePro mt-2".to_string(),
            );
            title.append_child(HtmlContent::Text(self.title.clone()));
            metadata.append_child(HtmlContent::Html(title));

            let mut date_container = Html::new();
            date_container.tag = "div".to_string();
            date_container.set_attr(
                "class".to_string(),
                "sourceCodePro text-center text-gray-600 font-bold".to_string(),
            );

            let mut created = Html::new();
            created.tag = "div".to_string();
            created.append_child(HtmlContent::Text(format!(
                "created: {}",
                self.created.format("%Y-%m-%d")
            )));
            date_container.append_child(HtmlContent::Html(created));

            let mut modified = Html::new();
            modified.tag = "div".to_string();
            modified.append_child(HtmlContent::Text(format!(
                "modified: {}",
                self.modified.format("%Y-%m-%d")
            )));
            date_container.append_child(HtmlContent::Html(modified));

            let mut tags = Html::new();
            tags.tag = "div".to_string();

            let mut tag_ctx = String::new();
            tag_ctx += "tags: [";

            for (i, tag) in self.tags.iter().enumerate() {
                tag_ctx += &format!(
                    "<a href=\"https://blog.bido.dev/tags/{}\">'#{}'</a>{}",
                    tag,
                    tag,
                    if i == self.tags.len() - 1 { "" } else { "," }
                );
            }

            tag_ctx += "]";
            tags.append_child(HtmlContent::Text(tag_ctx));

            date_container.append_child(HtmlContent::Html(tags));
            metadata.append_child(HtmlContent::Html(date_container));

            wrapper.append_child(HtmlContent::Html(metadata));

            wrapper.append_child(HtmlContent::Html(render::render::render(
                self.content.clone(),
            )));

            body.append_child(HtmlContent::Html(wrapper));
            body.append_child(HtmlContent::Html(self.render_footer()));
            html.append_child(HtmlContent::Html(body));

            html
        }

        fn render_base(&self) -> Html {
            let mut body = Html::new();
            body.tag = "body".to_string();

            let mut blog_link_icon = Html::new();
            blog_link_icon.tag = "a".to_string();
            blog_link_icon.set_attr("href".to_string(), "https://blog.bido.dev".to_string());
            blog_link_icon.set_attr(
                "class".to_string(),
                "text-4xl font-bold m-6 sourceCodePro inline-block text-black".to_string(),
            );
            blog_link_icon.append_child(HtmlContent::Text("blog.bido.dev;".to_string()));
            body.append_child(HtmlContent::Html(blog_link_icon));

            body
        }

        fn render_footer(&self) -> Html {
            let mut footer = Html::new();
            footer.tag = "footer".to_string();
            let mut footer_content = Html::new();
            footer_content.tag = "p".to_string();
            footer_content.set_attr("class".to_string(), "flex space-x-8".to_string());
            let mut rights_reserved = Html::new();
            rights_reserved.tag = "span".to_string();
            rights_reserved.set_attr("class".to_string(), "italic".to_string());
            rights_reserved.append_child(HtmlContent::Text("Copyright© ".to_string()));

            let mut link = Html::new();
            link.tag = "a".to_string();
            link.set_attr("href".to_string(), "https://bido.dev/".to_string());
            link.append_child(HtmlContent::Text("bido".to_string()));
            rights_reserved.append_child(HtmlContent::Html(link));
            rights_reserved.append_child(HtmlContent::Text(
                " 2023.\nAll rights reserved.\n See ".to_string(),
            ));

            let mut site_policy_link = Html::new();
            site_policy_link.tag = "a".to_string();
            site_policy_link.set_attr(
                "href".to_string(),
                "https://blog.bido.dev/policies/site".to_string(),
            );
            site_policy_link.append_child(HtmlContent::Text("site policy".to_string()));
            rights_reserved.append_child(HtmlContent::Html(site_policy_link));
            footer_content.append_child(HtmlContent::Html(rights_reserved));

            let mut humans_txt_link = Html::new();
            humans_txt_link.tag = "a".to_string();
            humans_txt_link.set_attr(
                "href".to_string(),
                "https://blog.bido.dev/humans.txt".to_string(),
            );
            humans_txt_link.append_child(HtmlContent::Text("humans.txt".to_string()));
            footer_content.append_child(HtmlContent::Html(humans_txt_link));

            footer.append_child(HtmlContent::Html(footer_content));

            footer
        }

        fn render_head(&self) -> Html {
            let mut head = Html::new();
            head.tag = "head".to_string();

            head.append_child(HtmlContent::Text("<meta charset=\"UTF-8\">".to_string()));
            head.append_child(HtmlContent::Text(
                "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"
                    .to_string(),
            ));

            let mut title = Html::new();
            title.tag = "title".to_string();
            title.append_child(HtmlContent::Text(self.title.clone()));
            head.append_child(HtmlContent::Html(title));

            //head.append_child(HtmlContent::Text(
            //    "<script src=\"https://cdn.tailwindcss.com\"></script>".to_string(),
            //));

            head.append_child(HtmlContent::Text(
                "<link rel=\"stylesheet\" href=\"./output.css\">".to_string(),
            ));
            head.append_child(HtmlContent::Text(
                "<link rel=\"preconnect\" href=\"https://fonts.googleapis.com\">".to_string(),
            ));
            head.append_child(HtmlContent::Text(
                "<link rel=\"preconnect\" href=\"https://fonts.gstatic.com\" crossorigin>"
                    .to_string(),
            ));
            head.append_child(HtmlContent::Text("<link rel=\"stylesheet\" href=\"https://fonts.googleapis.com/css2?family=Source+Code+Pro:ital,wght@0,200..900;1,200..900&family=Ubuntu+Mono&display=swap\">".to_string()));

            head
        }
    }
}
