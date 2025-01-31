// RT-json is an undocumented format used by Reddit to store rich text. It was part of their plan to kill third-party apps.
// 
// {"document":[{"e":"par","c":[{"e":"text","t":"this is a paragraph"}]},{"e":"par","c":[{"e":"text","t":"this is a bold","f":[[1,10,4]]}]},{"e":"par","c":[{"e":"text","t":"this is italic","f":[[2,8,6]]}]},{"e":"par","c":[{"e":"text","t":"this is striketrhough","f":[[8,8,13]]}]},{"e":"par","c":[{"e":"text","t":"this is high","f":[[32,8,4]]}]},{"e":"h","l":1,"c":[{"e":"raw","t":"this is heading"}]},{"e":"par","c":[{"e":"text","t":"this is "},{"e":"link","t":"link","u":"https://google.com"}]},{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"this is a list"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"second item"}]}]}]},{"e":"list","o":true,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"this is a numbered list"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"second item"}]}]}]},{"e":"blockquote","c":[{"e":"par","c":[{"e":"text","t":"this is a quote"}]}]},{"e":"par","c":[{"e":"text","t":"this is code","f":[[64,0,12]]}]},{"e":"code","c":[{"e":"raw","t":"this is better code!"}]},{"e":"par","c":[{"e":"text","t":""}]},{"e":"table","h":[{"c":[{"e":"text","t":"this"}]},{"c":[{"e":"text","t":"is"}]},{"c":[]}],"c":[[{"c":[{"e":"text","t":"a"}]},{"c":[{"e":"text","t":"table"}]},{"c":[]}],[{"c":[]},{"c":[]},{"c":[]}]]},{"e":"par","c":[{"e":"spoilertext","c":[{"e":"text","t":"this is a spoiler"}]}]},{"e":"img","id":"i70gb6ewu7ge1","c":"this is a caption"},{"e":"video","id":"j3t4ym4zu7ge1","c":"this is a video caption"},{"e":"par","c":[{"e":"br"}]},{"e":"par","c":[{"e":"text","t":""}]}]}

use std::{collections::VecDeque, f32::consts::E, ops::{Add, AddAssign, Range}};

use markdown::{mdast::Node, ParseOptions};
use serde::de::value;
use serde_json::{json, Value};

const BOLD: usize = 1;
const ITALIC: usize = 2;
const STRIKETHROUGH: usize = 8;
const HIGH: usize = 32;
const CODE: usize = 64;

enum Element {
    Document(Vec<Element>),
    Paragraph(Paragraph),
    Quote(Vec<Paragraph>),
    Heading(String),
    Code(String),
    List { ordered: bool, items: Vec<Paragraph> },
    Break,
}

impl Element {
    fn into_json(self) -> Option<Value> {
        match self {
            Element::Document(elements) => Some(json! {{ 
                "document": elements.into_iter().filter_map(|e| e.into_json()).collect::<Vec<_>>()
            }}),
            Element::Paragraph(paragraph) => Some(paragraph.into_json()),
            Element::Quote(paragraphs) => Some(json! {{
                "e": "quote",
                "c": paragraphs.into_iter().map(|p| p.into_json()).collect::<Vec<_>>()
            }}),
            Element::Heading(value) => Some(json! {{
                "e": "h",
                "l": 1,
                "c": [{ "e": "raw", "t": value }]
            }}),
            Element::Code(value) => Some(json! {{
                "e": "code",
                "c": [{ "e": "raw", "t": value }]
            }}),
            Element::List { ordered, items } => Some(json! {{
                "e": "list",
                "o": ordered,
                "c": items.into_iter().map(|i| json! {{
                    "e": "li",
                    "c": [i.into_json()]
                }}).collect::<Vec<_>>()
            }}),
            Element::Break => None,
        }
    }
}

#[derive(Default)]
struct Text {
    text: String,
    format: Vec<(usize, usize, usize)>, // (format, start, length)
}

impl Text {
    fn make_bold(&mut self) {
        self.format.push((BOLD, 0, self.text.len()));
    }

    fn make_italic(&mut self) {
        self.format.push((ITALIC, 0, self.text.len()));
    }

    fn make_strikethrough(&mut self) {
        self.format.push((STRIKETHROUGH, 0, self.text.len()));
    }

    fn make_high(&mut self) {
        self.format.push((HIGH, 0, self.text.len()));
    }

    fn make_code(&mut self) {
        self.format.push((CODE, 0, self.text.len()));
    }
}

impl From<&str> for Text {
    fn from(text: &str) -> Self {
        Self {
            text: text.to_string(),
            format: Vec::new(),
        }
    }
}

impl From<String> for Text {
    fn from(text: String) -> Self {
        Self {
            text,
            format: Vec::new(),
        }
    }
}

impl AddAssign for Text {
    fn add_assign(&mut self, other: Self) {
        for (format, start, length) in other.format {
            self.format.push((format, start + self.text.len(), length));
        }
        self.text.push_str(&other.text);
    }
}

enum ParagraphElement {
    Text(Text),
    Link{ label: Text, url: String },
}

#[derive(Default)]
struct Paragraph {
    elements: Vec<ParagraphElement>,
}

impl Paragraph {
    fn from_link(label: String, url: String) -> Self {
        Self {
            elements: vec![ParagraphElement::Link{ label: Text::from(label), url }],
        }
    }

    fn into_strikethrough(mut self) -> Self {
        self.elements.iter_mut().for_each(|element| {
            if let ParagraphElement::Text(text) = element {
                text.make_strikethrough();
            }
        });
        self
    }

    fn into_bold(mut self) -> Self {
        self.elements.iter_mut().for_each(|element| {
            if let ParagraphElement::Text(text) = element {
                text.make_bold();
            }
        });
        self
    }

    fn into_italic(mut self) -> Self {
        self.elements.iter_mut().for_each(|element| {
            if let ParagraphElement::Text(text) = element {
                text.make_italic();
            }
        });
        self
    }

    fn into_high(mut self) -> Self {
        self.elements.iter_mut().for_each(|element| {
            if let ParagraphElement::Text(text) = element {
                text.make_high();
            }
        });
        self
    }

    fn into_code(mut self) -> Self {
        self.elements.iter_mut().for_each(|element| {
            if let ParagraphElement::Text(text) = element {
                text.make_code();
            }
        });
        self
    }

    fn into_text(self) -> Text {
        let mut text = Text::default();
        for element in self.elements {
            match element {
                ParagraphElement::Text(t) => text += t,
                ParagraphElement::Link{ label, .. } => text += label,
            }
        }
        text
    }

    fn into_raw(self) -> String {
        self.into_text().text
    }
    
    fn then_break(self) -> Element {
        Element::Document(vec![Element::Paragraph(self), Element::Break])
    }

    fn into_json(self) -> Value {
        json! {{
            "e": "par",
            "c": self.elements.into_iter().map(|e| match e {
                ParagraphElement::Text(text) if text.format.is_empty() => json! {{
                    "e": "text",
                    "t": text.text
                }},
                ParagraphElement::Text(text) => json! {{
                    "e": "text",
                    "t": text.text,
                    "f": text.format,
                }},
                ParagraphElement::Link{ label, url } => json! {{
                    "e": "link",
                    "t": label.text,
                    "u": url
                }},
            }).collect::<Vec<_>>()
        }}
    }
}

impl AddAssign for Paragraph {
    fn add_assign(&mut self, mut other: Self) {
        if other.elements.is_empty() {
            return;
        }

        // Merge potentially adjacent text elements
        if let Some(ParagraphElement::Text(last_text)) = self.elements.last_mut() {
            let first = other.elements.remove(0);
            match first {
                ParagraphElement::Text(first_text) => {
                    *last_text += first_text;
                }
                _ => self.elements.push(first),
            }
        }

        self.elements.extend(other.elements);
    }
}

impl<T> From<T> for Paragraph where T: Into<Text> {
    fn from(text: T) -> Self {
        Self {
            elements: vec![ParagraphElement::Text(text.into())],
        }
    }
}

fn merge_children(children: Vec<Node>) -> Element {
    // Merge all adjacent paragraphs and flatten the documents
    let mut elements = Vec::new();
    for child in children.into_iter().map(inner) {
        match child {
            Element::Paragraph(paragraph) => {
                if let Some(Element::Paragraph(last_paragraph)) = elements.last_mut() {
                    *last_paragraph += paragraph;
                } else {
                    elements.push(Element::Paragraph(paragraph));
                }
            }
            Element::Document(children) => elements.extend(children),
            _ => elements.push(child),
        }
    }

    // If there is only one element, return it
    if elements.len() == 1 {
        return elements.remove(0);
    }

    Element::Document(elements)
}

fn children_into_paragraph(children: Vec<Node>) -> Paragraph {
    let mut paragraph = Paragraph::default();
    for child in children.into_iter().map(inner) {
        match child {
            Element::Paragraph(child_paragraph) => {
                paragraph += child_paragraph;
            }
            Element::Break => (),
            Element::Document(children) => {
                for child in children {
                    match child {
                        Element::Paragraph(child_paragraph) => {
                            paragraph += child_paragraph;
                        }
                        Element::Break => (),
                        _ => paragraph += Paragraph::from("�"),
                    }
                }
            }
            _ => paragraph += Paragraph::from("�"),
        }
    };

    paragraph
}

fn children_into_paragraphs(children: Vec<Node>) -> Vec<Paragraph> {
    let mut paragraphs = Vec::new();

    let mut paragraph = Paragraph::default();
    for child in children.into_iter().map(inner) {
        match child {
            Element::Paragraph(child_paragraph) => {
                paragraph += child_paragraph;
            }
            Element::Break => {
                paragraphs.push(paragraph);
                paragraph = Paragraph::default();
            }
            Element::Document(children) => {
                for child in children {
                    match child {
                        Element::Paragraph(child_paragraph) => {
                            paragraph += child_paragraph;
                        }
                        Element::Break => {
                            paragraphs.push(paragraph);
                            paragraph = Paragraph::default();
                        }
                        _ => paragraph += Paragraph::from("�"),
                    }
                }
            }
            _ => paragraph += Paragraph::from("�"),
        }
    };

    if !paragraph.elements.is_empty() {
        paragraphs.push(paragraph);
    }

    paragraphs
}

fn inner(node: Node) -> Element {
    match node {
        Node::Root(root) => merge_children(root.children),
        Node::Blockquote(blockquote) => Element::Quote(children_into_paragraphs(blockquote.children)),
        Node::List(list) => Element::List{
            ordered: list.ordered,
            items: list.children.into_iter().map(|i| children_into_paragraph(vec![i])).collect()
        },
        Node::Break(_) => Element::Break,
        Node::Delete(delete) => Element::Paragraph(children_into_paragraph(delete.children).into_strikethrough()),
        Node::Emphasis(emphasis) => Element::Paragraph(children_into_paragraph(emphasis.children).into_italic()),
        Node::Strong(strong) => Element::Paragraph(children_into_paragraph(strong.children).into_bold()),
        Node::InlineCode(inline_code) => Element::Paragraph(Paragraph::from(inline_code.value).into_code()),
        Node::InlineMath(inline_math) => Element::Paragraph(Paragraph::from(inline_math.value).into_code()),
        Node::Link(link) => Element::Paragraph(Paragraph::from_link(children_into_paragraph(link.children).into_raw(), link.url)),
        Node::Text(text) => Paragraph::from(text.value).then_break(),
        Node::Heading(heading) => Element::Heading(children_into_paragraph(heading.children).into_raw()),
        Node::ListItem(list_item) => Element::Paragraph(children_into_paragraph(list_item.children)),
        Node::Paragraph(paragraph) => children_into_paragraph(paragraph.children).then_break(),
        Node::ThematicBreak(_) => Element::Break,
        Node::MdxjsEsm(mdxjs_esm) => Element::Code(mdxjs_esm.value),
        Node::Toml(toml) => Element::Code(toml.value),
        Node::Yaml(yaml) => Element::Code(yaml.value),
        Node::Html(html) => Element::Code(html.value),
        Node::Code(code) => Element::Code(code.value),
        Node::FootnoteDefinition(_) => Paragraph::from("[unsupported footnote]").then_break(),
        Node::MdxJsxFlowElement(_) => Paragraph::from("[unsupported markdown JSX]").then_break(),
        Node::Math(_) => Paragraph::from("[unsupported math]").then_break(),
        Node::MdxFlowExpression(_) => Paragraph::from("[unsupported MDX]").then_break(),
        Node::Table(_) => Paragraph::from("[unsupported table]").then_break(), // TODO
        Node::LinkReference(_) => Paragraph::from("[unsupported link reference]").then_break(), // TODO
        Node::Image(_) => Paragraph::from("[unsupported image]").then_break(), // TODO
        Node::ImageReference(_) => Paragraph::from("[unsupported image reference]").then_break(), // TODO
        Node::MdxJsxTextElement(_) => Paragraph::from("[unsupported JSX text]").then_break(),
        Node::MdxTextExpression(_) => Paragraph::from("[unsupported MDX]").then_break(),
        Node::FootnoteReference(_) => Paragraph::from("[unsupported footnote reference]").then_break(),
        Node::TableRow(_) => Element::Paragraph(Paragraph::default()),
        Node::TableCell(_) => Element::Paragraph(Paragraph::default()),
        Node::Definition(_) => Element::Paragraph(Paragraph::default()), // TODO
    }
}

fn markdown_to_rtjson(markdown: &str) -> String {
    let ast = match markdown::to_mdast(markdown, &ParseOptions::default()) {
        Ok(ast) => ast,
        Err(e) => return format!("Invalid markdown: {e}"),
    };

    println!("{:?}", ast);
    let document = inner(ast);

    serde_json::to_string(&document.into_json()).unwrap()
}

#[cfg(test)]
#[test]
fn test_rtjson() {
    let markdown = r#"
this is a paragraph

this is a **bold**

this is *italic*

this is ~~strikethrough~~

# this is heading

this is [link](https://google.com)

- this is a list
- second item

1. this is a numbered list
2. second item

> this is a quote

`this is code`

```
this is better code!
```
    "#;

    let rtjson = markdown_to_rtjson(markdown);
    println!("{rtjson}");
}
