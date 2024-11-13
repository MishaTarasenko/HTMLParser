use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

/// The `HtmlParser` struct uses the `grammar.pest` file to define the parsing rules.
/// It implements the `Parser` trait from the `pest` library.
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct HtmlParser;

/// The `ParseError` enum represents the possible errors that can occur during parsing.
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),

    #[error("Mismatched tag: expected {expected}, found {found}")]
    MismatchedTag { expected: String, found: String },

    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}

/// The `Element` struct represents an HTML element with its tag name, attributes, and child nodes.
#[derive(Debug, PartialEq, Eq)]
pub struct Element {
    pub tag_name: String,

    pub attributes: Vec<(String, String)>,

    pub children: Vec<Node>,
}

/// The `Node` enum represents either an `Element` or a text node within the HTML structure.
#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    Element(Element),

    Text(String),
}

/// Parses the given HTML input string and returns a vector of `Node` objects or a `ParseError`.
///
/// # Arguments
///
/// * `input` - A string slice that holds the HTML content to be parsed.
///
/// # Returns
///
/// * `Ok(Vec<Node>)` if parsing is successful.
/// * `Err(ParseError)` if an error occurs during parsing.
///
/// # Grammar Rules
///
/// This function starts parsing from the `html` rule defined in `grammar.pest`.
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

/// Processes the `elements` rule and returns a vector of `Node` objects.
///
/// # Arguments
///
/// * `pair` - A `Pair` representing the `elements` rule.
///
/// # Returns
///
/// * `Vec<Node>` containing the parsed child nodes.
///
/// # Grammar Rules
///
/// This function processes the `element` and `self_closed_tag` rules within `elements`.
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

/// Processes the `element` rule and returns an `Element` struct.
///
/// # Arguments
///
/// * `pair` - A `Pair` representing the `element` rule.
///
/// # Returns
///
/// * `Element` containing the tag name, attributes, and child nodes.
///
/// # Grammar Rules
///
/// This function handles the `opening_tag`, `content`, and `closing_tag` rules within an `element`.
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

            Rule::closing_tag => {}
            _ => {}
        }
    }

    Element {
        tag_name,
        attributes,
        children,
    }
}

/// Processes the `self_closed_tag` rule and returns an `Element` struct.
///
/// # Arguments
///
/// * `pair` - A `Pair` representing the `self_closed_tag` rule.
///
/// # Returns
///
/// * `Element` containing the tag name and attributes, with no children.
///
/// # Grammar Rules
///
/// This function handles the `tag_name` and `attribute_list` rules within a `self_closed_tag`.
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

/// Processes the `opening_tag` rule and returns the tag name and attributes.
///
/// # Arguments
///
/// * `pair` - A `Pair` representing the `opening_tag` rule.
///
/// # Returns
///
/// * A tuple containing the tag name (`String`) and a vector of attributes (`Vec<(String, String)>`).
///
/// # Grammar Rules
///
/// This function handles the `tag_name` and `attribute_list` rules within an `opening_tag`.
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

/// Processes the `attribute_list` rule and returns a vector of attribute key-value pairs.
///
/// # Arguments
///
/// * `pair` - A `Pair` representing the `attribute_list` rule.
///
/// # Returns
///
/// * `Vec<(String, String)>` containing the attribute key-value pairs.
///
/// # Grammar Rules
///
/// This function handles multiple `attribute` rules within an `attribute_list`.
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
