use std::collections::HashMap;

/// A type alias for a `HashMap` where the keys are `String`
/// and the values are of a generic type `T`.
///
/// This type can be used as a convenient shorthand for
/// defining string-keyed hash maps with customizable value types.
///
/// # Examples
///
/// ```
/// use hermes::concepts::Dictionary;
///
/// let mut dict: Dictionary<i32> = Dictionary::new();
/// dict.insert("key1".to_string(), 10);
/// dict.insert("key2".to_string(), 20);
///
/// assert_eq!(dict["key1"], 10);
/// assert_eq!(dict["key2"], 20);
/// ```
pub type Dictionary<T> = HashMap<String, T>;

#[cfg(doc)]
use crate::http::{Message, Request, Response};

/// A type alias for a `Vec` where the elements are boxed values of type `T`.
///
/// This type can be used as a shorthand for creating a vector that stores heap-allocated elements,
/// which may be useful for avoiding copying large structures or **for polymorphism when using trait objects**.
///
/// # Examples
///
/// ```
/// use hermes::concepts::BoxVec;
/// use hermes::http::{MessageTrait};
///
/// let mut vec: BoxVec<dyn MessageTrait> = Vec::new();
/// ```
/// In this example, the vector `vec` accepts [Request], [Response] or [Message] objects.
pub type BoxVec<T> = Vec<Box<T>>;

/// Concatenates a prefix and a suffix if both are non-empty,
/// otherwise returns an empty string.
///
/// # Arguments
///
/// * `prefix` - The start of the concatenated string.
/// * `suffix` - The end of the concatenated string.
///
/// # Returns
///
/// A `String` which contains the concatenated result if both arguments are non-empty,
/// or an empty string otherwise.
///
/// # Examples
///
/// ```
/// use hermes::concepts::both_or_none;
///
/// let empty0 = both_or_none("prefix", "");
/// let empty1 = both_or_none("", "suffix");
/// let appended     = both_or_none("prefix", "suffix");
///
/// assert_eq!(empty0, "");
/// assert_eq!(empty1, "");
/// assert_eq!(appended, "prefixsuffix");
/// ```
pub fn both_or_none(prefix: &str, suffix: &str) -> String {
    if !(prefix.is_empty() || suffix.is_empty()) {
        format!("{}{}", prefix, suffix)
    } else {
        "".to_string()
    }
}

use nom::IResult;

pub trait Parsable {
    /// Parses a string input and construct this type.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice that serves as the input to the parser.
    ///
    /// # Returns
    ///
    /// Returns an [IResult] which is a result type provided by the `nom` library. It represents either:
    /// - `Ok((&str, Self))`: A tuple containing the remainder of the input and the parsed object if the parsing succeeds.
    /// - `Err(nom::Err)`: An error if the parsing fails.
    ///
    /// The `Self` here refers to the type implementing this trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use nom::character::complete::digit1;
    /// use nom::IResult;
    /// use hermes::concepts::Parsable;
    ///
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct Number(u32);
    ///
    /// impl Parsable for Number {
    ///     fn parse(input: &str) -> IResult<&str, Self> {
    ///         let (input, number) = digit1(input)?;
    ///         Ok((input, Number(number.parse::<u32>().unwrap())))
    ///     }
    /// }
    ///
    /// let input = "42 is the response";
    /// let (input, number) = Number::parse(input).unwrap();
    ///
    /// assert_eq!(input, " is the response");
    /// assert_eq!(number, Number(42));
    /// ```
    fn parse(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}

pub fn identifier(input: &str) -> IResult<&str, &str> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, alphanumeric1};
    use nom::combinator::recognize;
    use nom::multi::many0_count;
    use nom::sequence::pair;
    use nom::Parser;

    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .parse(input)
}

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Dictionary(Dictionary<Value>),
}
