use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct HtmlParser;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
    #[error("Mismatched tag: expected {expected}, found {found}")]
    MismatchedTag { expected: String, found: String },
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Element {
    pub tag_name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Node>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Element(Element),
    Text(String),
}

pub fn parse_html(input: &str) -> Result<Vec<Node>, ParseError> {
    let pairs = HtmlParser::parse(Rule::html, input)?;
    let mut nodes = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::html => {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::elements => {
                            nodes.extend(process_elements(inner_pair));
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(nodes)
}

fn process_elements(pair: Pair<Rule>) -> Vec<Node> {
    let mut nodes = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::element => {
                nodes.push(Node::Element(process_element(inner_pair)));
            }
            Rule::self_closed_tag => {
                nodes.push(Node::Element(process_self_closed_tag(inner_pair)));
            }
            _ => {}
        }
    }

    nodes
}

fn process_element(pair: Pair<Rule>) -> Element {
    let mut tag_name = String::new();
    let mut attributes = Vec::new();
    let mut children = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::opening_tag => {
                let (name, attrs) = process_opening_tag(inner_pair);
                tag_name = name;
                attributes = attrs;
            }
            Rule::content => {
                for content_pair in inner_pair.into_inner() {
                    match content_pair.as_rule() {
                        Rule::element => {
                            children.push(Node::Element(process_element(content_pair)));
                        }
                        Rule::self_closed_tag => {
                            children.push(Node::Element(process_self_closed_tag(content_pair)));
                        }
                        Rule::text => {
                            children.push(Node::Text(content_pair.as_str().to_string()));
                        }
                        _ => {}
                    }
                }
            }
            Rule::closing_tag => {
                // Handle closing tag if needed
            }
            _ => {}
        }
    }

    Element {
        tag_name,
        attributes,
        children,
    }
}

fn process_self_closed_tag(pair: Pair<Rule>) -> Element {
    let mut tag_name = String::new();
    let mut attributes = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::tag_name => {
                tag_name = inner_pair.as_str().to_string();
            }
            Rule::attribute_list => {
                attributes = process_attributes(inner_pair);
            }
            _ => {}
        }
    }

    Element {
        tag_name,
        attributes,
        children: Vec::new(),
    }
}

fn process_opening_tag(pair: Pair<Rule>) -> (String, Vec<(String, String)>) {
    let mut tag_name = String::new();
    let mut attributes = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::tag_name => {
                tag_name = inner_pair.as_str().to_string();
            }
            Rule::attribute_list => {
                attributes = process_attributes(inner_pair);
            }
            _ => {}
        }
    }

    (tag_name, attributes)
}

fn process_attributes(pair: Pair<Rule>) -> Vec<(String, String)> {
    let mut attributes = Vec::new();

    for attr in pair.into_inner() {
        if attr.as_rule() == Rule::attribute {
            let mut key = String::new();
            let mut value = String::new();

            for inner in attr.into_inner() {
                match inner.as_rule() {
                    Rule::identifier => {
                        key = inner.as_str().to_string();
                    }
                    Rule::quoted_string => {
                        // Remove the surrounding quotes
                        value = inner.as_str()[1..inner.as_str().len() - 1].to_string();
                    }
                    _ => {}
                }
            }

            attributes.push((key, value));
        }
    }

    attributes
}
