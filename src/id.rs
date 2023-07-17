//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize, Deserialize, Deserializer, Serializer};
use serde::de::{Error, Visitor};
use crate::error::{ErrorCode, ErrorObject};
use std::borrow::Cow;
use std::fmt;

//---------------------------------------------------------------------------------------------------- Id
#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Serialize,Deserialize)]
#[serde(untagged)]
/// [Request/Response object ID](https://www.jsonrpc.org/specification)
pub enum Id<'a> {
    /// `null`
	Null,

    /// Number ID
    Num(u64),

	#[serde(borrow)]
    /// String ID
    Str(Cow<'a, str>),
}

impl<'a> Id<'_> {
	#[inline]
	/// Return inner [`u64`] if [`Id`] is a number
	pub fn as_u64(&self) -> Option<u64> {
		match self {
			Self::Num(n) => Some(*n),
			_ => None,
		}
	}

	#[inline]
	/// Return inner [`str`] if [`Id`] is a string
	pub fn as_str(&self) -> Option<&str> {
		match self {
			Self::Str(s) => Some(s.as_ref()),
			_ => None,
		}
	}

	#[inline]
	pub fn is_null(&self) -> bool {
		*self == Self::Null
	}

	#[inline]
	/// Convert `Id<'a>` to `Id<'static>`
	pub fn into_owned(self) -> Id<'static> {
		match self {
			Id::Null   => Id::Null,
			Id::Num(u) => Id::Num(u),
			Id::Str(s) => Id::Str(Cow::Owned(s.into_owned())),
		}
	}

	#[inline]
	/// Extract the underlying number from the [`Id`].
	pub fn try_parse_num(&self) -> Option<u64> {
		match self {
			Id::Null => None,
			Id::Num(num) => Some(*num),
			Id::Str(s) => s.parse().ok(),
		}
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
mod test {
	use super::*;

	#[test]
	fn null() {
		let id = Id::Null;
		assert!(id.is_null());
	}

	#[test]
	fn parse() {
		let id = Id::Str(format!("{}", u64::MIN).into());
		assert_eq!(id.try_parse_num().unwrap(), u64::MIN);

		let id = Id::Str(format!("{}", u64::MAX).into());
		assert_eq!(id.try_parse_num().unwrap(), u64::MAX);

		let id = Id::Str(format!("{}a", u64::MAX).into());
		assert!(id.try_parse_num().is_none());

		let id = Id::Num(u64::MIN);
		assert_eq!(id.try_parse_num().unwrap(), u64::MIN);

		let id = Id::Num(u64::MAX);
		assert_eq!(id.try_parse_num().unwrap(), u64::MAX);

		let id = Id::Null;
		assert!(id.try_parse_num().is_none());
	}

	#[test]
	fn __as_u64() {
		let id = Id::Num(u64::MIN);
		assert_eq!(id.as_u64().unwrap(), u64::MIN);

		let id = Id::Num(u64::MAX);
		assert_eq!(id.as_u64().unwrap(), u64::MAX);

		let id = Id::Null;
		assert!(id.as_u64().is_none());
		let id = Id::Str("".into());
		assert!(id.as_u64().is_none());
	}

	#[test]
	fn __as_str() {
		let id = Id::Str("str".into());
		assert_eq!(id.as_str().unwrap(), "str");

		let id = Id::Null;
		assert!(id.as_str().is_none());
		let id = Id::Num(0);
		assert!(id.as_str().is_none());
	}
}
