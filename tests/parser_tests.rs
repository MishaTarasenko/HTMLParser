use pest::Parser;
use HTMLParser::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valid_attribute_with_simple_identifier_and_string() {
        let input = r#"width="100%""#;
        let result = Grammar::parse(Rule::attribute, input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_attribute_with_unquoted_string_should_fail() {
        let input = r#"width=100%"#; // No quotes around 100%
        let result = Grammar::parse(Rule::attribute, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_attribute_with_underscore_identifier_should_fail() {
        let input = r#"data_attribute="some value""#;
        let result = Grammar::parse(Rule::attribute, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_attribute_with_invalid_identifier_starting_with_digit_should_fail() {
        let input = r#"123name="value""#; // Identifier starting with a digit
        let result = Grammar::parse(Rule::attribute, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_attribute_with_unclosed_string_should_fail() {
        let input = r#"width="100%"#; // Missing closing quote
        let result = Grammar::parse(Rule::attribute, input);
        assert!(result.is_err());
    }

    #[test]
    fn test_attribute_with_empty_quoted_string() {
        let input = r#"placeholder="""#;
        let result = Grammar::parse(Rule::attribute, input);
        assert!(result.is_ok());
    }
}
