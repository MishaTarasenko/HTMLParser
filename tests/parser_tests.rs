use anyhow::Result;
use html_parser::{parse_html, Node};

#[test]
fn test_empty_html() -> Result<()> {
    let input = "";
    let result = parse_html(input)?;
    assert!(result.is_empty());
    Ok(())
}

#[test]
fn test_simple_element() -> Result<()> {
    let input = "<div></div>";
    let expected = vec![Node::Element(html_parser::Element {
        tag_name: "div".to_string(),
        attributes: vec![],
        children: vec![],
    })];
    let result = parse_html(input)?;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_self_closed_tag() -> Result<()> {
    let input = "<img src=\"image.png\" />";
    let expected = vec![Node::Element(html_parser::Element {
        tag_name: "img".to_string(),
        attributes: vec![("src".to_string(), "image.png".to_string())],
        children: vec![],
    })];
    let result = parse_html(input)?;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_nested_elements() -> Result<()> {
    let input = "<div><span>Text</span></div>";
    let expected = vec![Node::Element(html_parser::Element {
        tag_name: "div".to_string(),
        attributes: vec![],
        children: vec![Node::Element(html_parser::Element {
            tag_name: "span".to_string(),
            attributes: vec![],
            children: vec![Node::Text("Text".to_string())],
        })],
    })];
    let result = parse_html(input)?;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_attributes() -> Result<()> {
    let input = "<a href=\"https://example.com\" title=\"Example\">Link</a>";
    let expected = vec![Node::Element(html_parser::Element {
        tag_name: "a".to_string(),
        attributes: vec![
            ("href".to_string(), "https://example.com".to_string()),
            ("title".to_string(), "Example".to_string()),
        ],
        children: vec![Node::Text("Link".to_string())],
    })];
    let result = parse_html(input)?;
    assert_eq!(result, expected);
    Ok(())
}
