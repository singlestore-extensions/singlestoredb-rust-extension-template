use chrono::NaiveDateTime;

wit_bindgen_rust::export!("{{cookiecutter.project_slug}}.wit");

struct {{cookiecutter.project_slug.capitalize()}};

impl {{cookiecutter.project_slug}}::{{cookiecutter.project_slug.capitalize()}} for {{cookiecutter.project_slug.capitalize()}} {
    fn foo(s: String) -> String {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::{{cookiecutter.project_slug}}::{{cookiecutter.project_slug.capitalize()}} as _;
    use super::*;

    #[test]
    fn test_passes_through_string() {
        assert_eq!({{cookiecutter.project_slug.capitalize()}}::foo("asdf".to_string()), "asdf".to_string());
    }
}