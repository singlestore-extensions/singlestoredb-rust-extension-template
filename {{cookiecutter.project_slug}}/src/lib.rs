wit_bindgen_rust::export!("{{cookiecutter.project_slug}}.wit");

struct {{cookiecutter.project_pascal_case}};

impl {{cookiecutter.project_slug}}::{{cookiecutter.project_pascal_case}} for {{cookiecutter.project_pascal_case}} {
    fn foo(s: String) -> String {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::{{cookiecutter.project_slug}}::{{cookiecutter.project_pascal_case}} as _;
    use super::*;

    #[test]
    fn test_passes_through_string() {
        assert_eq!({{cookiecutter.project_pascal_case}}::foo("asdf".to_string()), "asdf".to_string());
    }
}