use chrono::NaiveDateTime;

wit_bindgen_rust::export!("{{extension_name}}.wit");

struct {{extension_name_uppercase}};

impl {{extension_name}}::{{extension_name_uppercase}} for {{extension_name_uppercase}} {
    fn foo(s: String) -> String {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::{{extension_name}}::{{extension_name_uppercase}} as _;
    use super::*;

    #[test]
    fn test_passes_through_string() {
        assert_eq!({{extension_name_uppercase}}::foo("asdf".to_string()), "asdf".to_string());
    }
}