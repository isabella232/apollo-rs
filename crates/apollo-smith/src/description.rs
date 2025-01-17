use arbitrary::{Arbitrary, Result, Unstructured};

use crate::DocumentBuilder;

const CHARSET: &[u8] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_\n\r\t/$#!.-+='";

/// The `__Description` type represents a description
///
/// *Description*:
///     "string"
///
/// Detailed documentation can be found in [GraphQL spec](https://spec.graphql.org/October2021/#sec-Descriptions).
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Arbitrary)]
pub struct Description(StringValue);

impl From<Description> for String {
    fn from(desc: Description) -> Self {
        desc.0.into()
    }
}

/// The `__StringValue` type represents a sequence of characters
///
/// *StringValue*:
///     "string" | """string"""
///
/// Detailed documentation can be found in [GraphQL spec](https://spec.graphql.org/October2021/#sec-Descriptions).
///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringValue {
    /// Represents a string value between """
    Block(String),
    /// Represents a one line string value between "
    Line(String),
}

impl From<StringValue> for String {
    fn from(str_value: StringValue) -> Self {
        match str_value {
            StringValue::Block(str_val) => format!(r#""""{str_val}""""#),
            StringValue::Line(str_val) => format!(r#"{str_val}""#),
        }
    }
}

impl Arbitrary<'_> for StringValue {
    fn arbitrary(u: &mut arbitrary::Unstructured<'_>) -> Result<Self> {
        let arbitrary_str = limited_string_desc(u, 100)?;
        let variant_idx = u.int_in_range(0..=1usize)?;
        let str_value = match variant_idx {
            0 => Self::Block(arbitrary_str),
            1 => Self::Line(arbitrary_str),
            _ => unreachable!(),
        };

        Ok(str_value)
    }
}

impl<'a> DocumentBuilder<'a> {
    /// Create an arbitrary `Description`
    pub fn description(&mut self) -> Result<Description> {
        self.u.arbitrary()
    }
}

fn limited_string_desc(u: &mut Unstructured<'_>, max_size: usize) -> Result<String> {
    let size = u.int_in_range(0..=max_size)?;

    let gen_str = String::from_utf8(
        (0..size)
            .map(|_curr_idx| {
                let idx = u.arbitrary::<usize>()?;

                let idx = idx % CHARSET.len();

                Ok(CHARSET[idx])
            })
            .collect::<Result<Vec<u8>>>()?,
    )
    .unwrap();

    Ok(gen_str)
}
