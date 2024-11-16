use anyhow::Result;
use html_parser_tarasenko::{parse_html, Node};

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
    let expected = vec![Node::Element(html_parser_tarasenko::Element {
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
    let expected = vec![Node::Element(html_parser_tarasenko::Element {
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
    let expected = vec![Node::Element(html_parser_tarasenko::Element {
        tag_name: "div".to_string(),
        attributes: vec![],
        children: vec![Node::Element(html_parser_tarasenko::Element {
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
    let expected = vec![Node::Element(html_parser_tarasenko::Element {
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

#[test]
fn test_empty_element() -> Result<()> {
    let input = "<div></div>";
    let expected = vec![Node::Element(html_parser_tarasenko::Element {
        tag_name: "div".to_string(),
        attributes: vec![],
        children: vec![],
    })];
    let result = parse_html(input)?;
    assert_eq!(result, expected);
    Ok(())
}

#[test]
fn test_invalid_html() {
    let input = "<div><p>Unclosed tag";
    let result = parse_html(input);
    assert!(result.is_err());
}

#[test]
fn test_html() -> Result<()> {
    assert!(
        html_parser_tarasenko::parse_html("<html><body><p>Hello World!</p></body></html>").is_ok()
    );
    assert!(html_parser_tarasenko::parse_html("<div><img src=\"image.jpg\" /></div>").is_ok());
    assert!(
        html_parser_tarasenko::parse_html("<html><body><p>Hello World!</body></html>").is_err()
    );
    assert!(html_parser_tarasenko::parse_html("<html><body><p>Hello World!").is_err());
    Ok(())
}

#[test]
fn test_elements() -> Result<()> {
    assert!(
        html_parser_tarasenko::parse("elements", "<p>Paragraph</p><img src=\"img.jpg\" />").is_ok()
    );
    assert!(html_parser_tarasenko::parse("elements", "<div></div><span></span>").is_ok());
    Ok(())
}

#[test]
fn test_element() -> Result<()> {
    assert!(html_parser_tarasenko::parse("element", "<p>Text</p>").is_ok());
    assert!(html_parser_tarasenko::parse("element", "<div><span>Nested</span></div>").is_ok());
    assert!(html_parser_tarasenko::parse("element", "<p>Text").is_err());
    assert!(html_parser_tarasenko::parse("element", "<div><span>Nested</div>").is_err());
    Ok(())
}

#[test]
fn test_opening_tag() -> Result<()> {
    assert!(html_parser_tarasenko::parse("opening_tag", "<div>").is_ok());
    assert!(html_parser_tarasenko::parse("opening_tag", "<img src=\"image.jpg\">").is_ok());
    assert!(html_parser_tarasenko::parse("opening_tag", "<div").is_err());
    assert!(html_parser_tarasenko::parse("opening_tag", "<123>").is_err());
    Ok(())
}

#[test]
fn test_closing_tag() -> Result<()> {
    assert!(html_parser_tarasenko::parse("closing_tag", "</div>").is_ok());
    assert!(html_parser_tarasenko::parse("closing_tag", "</span>").is_ok());
    assert!(html_parser_tarasenko::parse("closing_tag", "</div").is_err());
    assert!(html_parser_tarasenko::parse("closing_tag", "<div>").is_err());
    Ok(())
}

#[test]
fn test_self_closed_tag_second_test() -> Result<()> {
    assert!(html_parser_tarasenko::parse("self_closed_tag", "<img src=\"image.jpg\" />").is_ok());
    assert!(html_parser_tarasenko::parse("self_closed_tag", "<br />").is_ok());
    assert!(html_parser_tarasenko::parse("self_closed_tag", "<img src=\"image.jpg\">").is_err());
    assert!(html_parser_tarasenko::parse("self_closed_tag", "<img src=\"image.jpg\" /").is_err());
    Ok(())
}

#[test]
fn test_tag_name() -> Result<()> {
    assert!(html_parser_tarasenko::parse("tag_name", "div").is_ok());
    assert!(html_parser_tarasenko::parse("tag_name", "img1").is_ok());
    assert!(html_parser_tarasenko::parse("tag_name", "123").is_err());
    Ok(())
}

#[test]
fn test_attribute_list() -> Result<()> {
    assert!(html_parser_tarasenko::parse(
        "attribute_list",
        "src=\"image.jpg\" alt=\"description\""
    )
    .is_ok());
    assert!(html_parser_tarasenko::parse("attribute_list", "class=\"container\"").is_ok());
    Ok(())
}

#[test]
fn test_attribute() -> Result<()> {
    assert!(html_parser_tarasenko::parse("attribute", "src=\"image.jpg\"").is_ok());
    assert!(html_parser_tarasenko::parse("attribute", "alt=\"description\"").is_ok());
    assert!(html_parser_tarasenko::parse("attribute", "src=image.jpg").is_err());
    assert!(html_parser_tarasenko::parse("attribute", "alt='description'").is_err());
    Ok(())
}

#[test]
fn test_identifier() -> Result<()> {
    assert!(html_parser_tarasenko::parse("identifier", "src").is_ok());
    assert!(html_parser_tarasenko::parse("identifier", "class").is_ok());
    assert!(html_parser_tarasenko::parse("identifier", "123class").is_err());
    Ok(())
}

#[test]
fn test_quoted_string() -> Result<()> {
    assert!(html_parser_tarasenko::parse("quoted_string", "\"image.jpg\"").is_ok());
    assert!(html_parser_tarasenko::parse("quoted_string", "\"description\"").is_ok());
    assert!(html_parser_tarasenko::parse("quoted_string", "image.jpg").is_err());
    assert!(html_parser_tarasenko::parse("quoted_string", "\"unclosed string").is_err());
    Ok(())
}

#[test]
fn test_content() -> Result<()> {
    assert!(html_parser_tarasenko::parse("content", "Text").is_ok());
    assert!(html_parser_tarasenko::parse("content", "<p>Paragraph</p>Nested text").is_ok());
    Ok(())
}

#[test]
fn test_text() -> Result<()> {
    assert!(html_parser_tarasenko::parse("text", "Hello World!").is_ok());
    assert!(html_parser_tarasenko::parse("text", "Plain text with spaces").is_ok());
    assert!(html_parser_tarasenko::parse("text", "<text>").is_err());
    Ok(())
}

#[test]
fn test_whitespace() -> Result<()> {
    assert!(html_parser_tarasenko::parse("WHITESPACE", " ").is_ok());
    assert!(html_parser_tarasenko::parse("WHITESPACE", "\t\n\r").is_ok());
    assert!(html_parser_tarasenko::parse("WHITESPACE", "a").is_err());
    Ok(())
}
