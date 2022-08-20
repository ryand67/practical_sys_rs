use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_literal_test() {
        let s = "<h1>Hello world</h1>";
        assert_eq!(ContentType::Literal(s.to_string()), get_content_type(s));
    }

    #[test]
    fn check_template_var_test() {
        let content = ExpressionData {
            head: Some("Hi ".to_string()),
            variable: "name".to_string(),
            tail: Some(", welcome".to_string()),
        };
        assert_eq!(
            ContentType::TemplateVariable(content),
            get_content_type("Hi {{name}}, welcome")
        )
    }

    #[test]
    fn check_for_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::ForTag),
            get_content_type("{% for name in names %}, welcome")
        )
    }

    #[test]
    fn check_if_tag_test() {
        assert_eq!(
            ContentType::Tag(TagType::IfTag),
            get_content_type("{% if name == 'Bob' %}")
        );
    }

    #[test]
    fn check_get_expression_data_test() {
        let expression_data = ExpressionData {
            head: Some("Hi".to_string()),
            variable: "name".to_string(),
            tail: Some(" , welcome".to_string()),
        };
        assert_eq!(expression_data, get_expression_data("Hi {{name}}, welcome"));
    }

    #[test]
    fn check_get_index_for_symbol_test() {
        assert_eq!((true, 3), get_index_for_symbol("Hi {name}, welcome", '{'));
    }
}

#[derive(Debug, PartialEq)]
pub enum ContentType {
    Literal(String),
    TemplateVariable(ExpressionData),
    Tag(TagType),
    Unrecognized,
}

#[derive(Debug, PartialEq)]
pub enum TagType {
    ForTag,
    IfTag,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionData {
    pub head: Option<String>,
    pub variable: String,
    pub tail: Option<String>,
}

pub fn get_content_type(input_line: &str) -> ContentType {
    let is_tag_expression = check_matching_pair(&input_line, "{%", "%}");
    let is_for_tag = (check_symbol_string(&input_line, "for"))
        && check_symbol_string(&input_line, "in")
        || check_symbol_string(&input_line, "endfor");
    let is_if_tag =
        check_symbol_string(&input_line, "if") || check_symbol_string(&input_line, "endif");
    let is_template_variable = check_matching_pair(&input_line, "{{", "}}");
    let return_val;
    if is_tag_expression && is_for_tag {
        return_val = ContentType::Tag(TagType::ForTag);
    } else if is_tag_expression && is_if_tag {
        return_val = ContentType::Tag(TagType::IfTag);
    } else if is_template_variable {
        let content = get_expression_data(&input_line);
        return_val = ContentType::TemplateVariable(content);
    } else if !is_tag_expression && !is_template_variable {
        return_val = ContentType::Literal(input_line.to_string());
    } else {
        return_val = ContentType::Unrecognized;
    }
    return_val
}

pub fn check_symbol_string(input: &str, symbol: &str) -> bool {
    input.contains(symbol)
}

pub fn check_matching_pair(input: &str, symbol1: &str, symbol2: &str) -> bool {
    input.contains(symbol1) && input.contains(symbol2)
}

pub fn get_expression_data(input_line: &str) -> ExpressionData {
    let (_h, i) = get_index_for_symbol(input_line, '{');
    let head = input_line[0..i].to_string();
    let (_j, k) = get_index_for_symbol(input_line, '}');
    let variable = input_line[i + 1 + 1..k].to_string();
    let tail = input_line[k + 1 + 1..].to_string();
    ExpressionData {
        head: Some(head),
        variable,
        tail: Some(tail),
    }
}

pub fn get_index_for_symbol(input: &str, symbol: char) -> (bool, usize) {
    let mut characters = input.char_indices();
    let mut does_exist = false;
    let mut index = 0;

    while let Some((c, d)) = characters.next() {
        if d == symbol {
            does_exist = true;
            index = c;
            break;
        }
    }

    (does_exist, index)
}

pub fn generate_html_template_var(
    content: ExpressionData,
    context: HashMap<String, String>,
) -> String {
    let mut html = String::new();

    if let Some(h) = content.head {
        html.push_str(&h);
    }
    if let Some(val) = context.get(&content.variable) {
        html.push_str(&val);
    }
    if let Some(t) = content.tail {
        html.push_str(&t)
    }

    html
}
