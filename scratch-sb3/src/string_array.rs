use std::fmt::{Debug, Display};

use serde::de::Error;
use serde::{Deserialize, Serialize};

use crate::Id;

pub trait StringArrayElement: Display {
    fn from_str(s: &str) -> Result<Self, &'static str>
    where
        Self: Sized;
}

impl StringArrayElement for String {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        Ok(s.to_string())
    }
}

impl StringArrayElement for Id {
    fn from_str(s: &str) -> Result<Self, &'static str> {
        s.to_string().try_into()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct StringArray<T: StringArrayElement>(Vec<T>);

impl<T: StringArrayElement> StringArray<T> {
    pub fn new() -> StringArray<T> {
        Self(Vec::new())
    }

    pub fn push(&mut self, element: T) {
        self.0.push(element);
    }
}

impl<T: StringArrayElement> From<Vec<T>> for StringArray<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T: StringArrayElement> Serialize for StringArray<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut string_builder = String::from('[');
        for (i, element) in self.0.iter().enumerate() {
            string_builder.push_str(&format!("\"{}\"", escape(element.to_string())));
            if i != self.0.len() - 1 {
                string_builder.push(',');
            }
        }
        string_builder.push(']');

        serializer.serialize_str(&string_builder)
    }
}

impl<'de, T: StringArrayElement> Deserialize<'de> for StringArray<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        let Some(s) = s.strip_prefix("[") else {
            return Err(D::Error::custom("Must start with `[`"));
        };

        let Some(s) = s.strip_suffix("]") else {
            return Err(D::Error::custom("Must end with `]`"));
        };

        let s = deescape(s);

        let elements = s.split("\",");
        let number = elements.clone().count();

        let mut stringarray = StringArray::new();
        for (i, mut element) in elements.enumerate() {
            if i == number - 1 {
                let Some(stripped_element) = element.strip_suffix("\"") else {
                    return Err(D::Error::custom("Element must end with `\\\"`"));
                };
                element = stripped_element;
            }

            if let Some(element) = element.strip_prefix("\"") {
                let t = match T::from_str(element) {
                    Ok(t) => t,
                    Err(err) => return Err(D::Error::custom(err)),
                };
                stringarray.push(t);
            } else {
                return Err(D::Error::custom("Element must start with `\\\"`"));
            }
        }

        Ok(stringarray)
    }
}

fn escape(s: String) -> String {
    s.replace('\\', r#"\\\\"#).replace('"', r#"\\\""#)
}

fn deescape(s: &str) -> String {
    s.replace(r#"\\"#, "\\").replace(r#"\""#, "\"")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_stringarray_one() {
        let stringarray: StringArray<String> = serde_json::from_str(r#""[\"one\"]""#).unwrap();

        let mut expected = StringArray::new();
        expected.push("one".to_string());

        assert_eq!(stringarray, expected);
    }

    #[test]
    fn deserialize_stringarray_two() {
        let stringarray: StringArray<String> =
            serde_json::from_str(r#""[\"one\",\"two\"]""#).unwrap();

        let mut expected = StringArray::new();
        expected.push("one".to_string());
        expected.push("two".to_string());

        assert_eq!(stringarray, expected);
    }

    #[test]
    fn deserialize_stringarray_escape() {
        let stringarray: StringArray<String> =
            serde_json::from_str(r#""[\"\\\\\\\"one\\\"\"]""#).unwrap();

        let mut expected = StringArray::new();
        expected.push(r#"\"one""#.to_string());

        assert_eq!(stringarray, expected);
    }

    #[test]
    fn serialize_stringarray_one() {
        let mut stringarray = StringArray::new();
        stringarray.push("hallo".to_string());

        let serialized = serde_json::to_value(stringarray).unwrap();
        assert_eq!(serialized.to_string(), String::from(r#""[\"hallo\"]""#));
    }

    #[test]
    fn serialize_stringarray_two() {
        let mut stringarray = StringArray::new();
        stringarray.push("hallo".to_string());
        stringarray.push("bye".to_string());

        let serialized = serde_json::to_value(stringarray).unwrap();
        assert_eq!(
            serialized.to_string(),
            String::from(r#""[\"hallo\",\"bye\"]""#)
        );
    }
}
