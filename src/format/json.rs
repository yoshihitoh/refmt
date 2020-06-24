use serde::de;
use serde::ser;

use crate::errors;

pub use serde_json::Value as InnerValue;

pub fn serialize<V: ser::Serialize>(v: V) -> Result<String, errors::Error> {
    let json = serde_json::to_string_pretty(&v)
        .map_err(|e| errors::Error::Serialization(e.to_string()))?;
    Ok(json)
}

pub fn deserialize<V>(s: &str) -> Result<V, errors::Error>
where
    V: for<'de> de::Deserialize<'de>,
{
    let data =
        serde_json::from_str(s).map_err(|e| errors::Error::Deserialization(e.to_string()))?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Eq, PartialEq)]
    struct Author {
        pub id: u64,
        pub first_name: String,
        pub last_name: String,
    }

    #[derive(Serialize, Deserialize, Eq, PartialEq)]
    struct Book {
        pub id: u64,
        pub author: Author,
        pub title: String,
    }

    #[test]
    fn success() {
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
        }"#;

        // deserialize
        let book = deserialize::<Book>(json_text);
        assert!(book.is_ok());
        let book = book.unwrap();
        assert_eq!(book.id, 123);
        assert_eq!(book.title, "Lorem ipsum dolor sit amet");
        assert_eq!(book.author.id, 999);
        assert_eq!(book.author.first_name, "John");
        assert_eq!(book.author.last_name, "Doe");

        // serialize
        let text = serialize(&book);
        assert!(text.is_ok());
        let text = text.unwrap();
        assert_eq!(
            text,
            r#"{
  "id": 123,
  "author": {
    "id": 999,
    "first_name": "John",
    "last_name": "Doe"
  },
  "title": "Lorem ipsum dolor sit amet"
}"#
        );

        // deserialize from serialized text
        let book = deserialize::<Book>(&text);
        assert!(book.is_ok());
        let book = book.unwrap();
        assert_eq!(book.id, 123);
        assert_eq!(book.title, "Lorem ipsum dolor sit amet");
        assert_eq!(book.author.id, 999);
        assert_eq!(book.author.first_name, "John");
        assert_eq!(book.author.last_name, "Doe");
    }
}
