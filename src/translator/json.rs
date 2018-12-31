use serde::Deserialize;

use super::translator::{parse_as, to_translate_error, Format, TranslateError, Translator};

pub struct JsonTranslator {}

impl Default for JsonTranslator {
    fn default() -> Self {
        JsonTranslator {}
    }
}

impl Translator for JsonTranslator {
    fn translate(&self, s: &str, fmt: Format) -> Result<String, TranslateError> {
        let value = parse_as::<serde_json::Value>(s, fmt)?;
        serde_json::to_string_pretty(&value).map_err(to_translate_error)
    }
}

pub fn parse_json<'de, V: Deserialize<'de>>(s: &'de str) -> Result<V, serde_json::Error> {
    serde_json::from_str(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_derive::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Author {
        pub id: u64,
        pub first_name: String,
        pub last_name: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Book {
        pub id: u64,
        pub author: Author,
        pub title: String,
    }

    #[test]
    fn test_translator() {
        let translator = JsonTranslator::default();

        // normal case
        // NOTE: indent has meaning on yaml format, so do not indent the below text.
        let yaml_text = r#"---
id: 123
title: Lorem ipsum dolor sit amet
author:
  id: 999
  first_name: John
  last_name: Doe
"#;
        let r = translator.translate(yaml_text, Format::Yaml);
        assert!(r.is_ok());
        assert_eq!(
            r.ok().unwrap(),
            r#"{
  "id": 123,
  "title": "Lorem ipsum dolor sit amet",
  "author": {
    "id": 999,
    "first_name": "John",
    "last_name": "Doe"
  }
}"#
        );

        // error case: syntax error (wrong indents)
        let yaml_text = r#"
        ---
        id: 123
        title: Lorem ipsum dolor sit amet
        author:
          id: 999
          first_name: John
          last_name: Doe
        "#;
        let r = translator.translate(yaml_text, Format::Yaml);
        assert!(r.is_err());
    }

    #[test]
    fn test_parse_json() {
        // normal case
        let json_text = r#"
        {
            "id": 123,
            "title": "Lorem ipsum dolor sit amet",
            "author": {
                "id": 999,
                "first_name": "John",
                "last_name": "Doe"
            }
        }
        "#;

        let book = parse_json::<Book>(json_text);
        assert!(book.is_ok());
        let book = book.unwrap();
        assert_eq!(book.id, 123);
        assert_eq!(book.title, "Lorem ipsum dolor sit amet");
        assert_eq!(book.author.id, 999);
        assert_eq!(book.author.first_name, "John");
        assert_eq!(book.author.last_name, "Doe");

        // error case: no quote on each field names
        let json_text = r#"
        {
            id: 123,
            title: "Lorem ipsum dolor sit amet",
            author: {
                id: 999,
                first_name: "John",
                last_name: "Doe"
            }
        }
        "#;
        let book = parse_json::<Book>(json_text);
        assert!(book.is_err());
        assert!(book.err().unwrap().is_syntax());
    }
}
