//---------------------------------------------------------------------------------------------------- Use
use serde::{Serialize, Deserialize};
use std::borrow::Cow;
use crate::version::Version;
use crate::id::Id;
use serde_json::value::Value;

//---------------------------------------------------------------------------------------------------- Request
/// JSON-RPC 2.0 Request object
#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct Request<'a> {
	/// JSON-RPC 2.0
    pub jsonrpc: Version,

	#[serde(borrow)]
    /// A String containing the name of the method to be invoked
    pub method: Cow<'a, str>,

	#[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A Structured value that holds the parameter values to be used during the invocation of the method
    pub params: Option<Cow<'a, Value>>,

	#[serde(borrow)]
    #[serde(skip_serializing_if = "Option::is_none")]
	/// An identifier established by the Client that MUST contain a String, Number, or NULL value if included.
	///
	/// If it is not included it is assumed to be a notification.
    pub id: Option<Id<'a>>,
}

impl<'a> Request<'a> {
	#[inline]
	/// Create a new [`Self`].
	pub fn new(
		method: Cow<'a, str>,
		params: Option<Cow<'a, Value>>,
		id: Option<Id<'a>>,
	) -> Self {
		Self {
			jsonrpc: Version,
			method,
			params,
			id,
		}
	}

	#[inline]
	/// Returns whether request is notification.
	pub const fn is_notification(&self) -> bool {
		self.id.is_none()
	}

	#[inline]
	/// Convert `Self<'a>` to `Self<'static>`
	pub fn into_owned(self) -> Request<'static> {
		Request {
			jsonrpc: self.jsonrpc,
			method: Cow::Owned(self.method.into_owned()),
			params: self.params.map(|p| Cow::Owned(p.into_owned())),
			id: self.id.map(|id| id.into_owned()),
		}
	}
}

//---------------------------------------------------------------------------------------------------- Trait impl
impl<'a> std::fmt::Display for Request<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match serde_json::to_string_pretty(self) {
			Ok(json) => write!(f, "{json}"),
			Err(_)   => Err(std::fmt::Error),
		}
	}
}

impl<'a> PartialEq for Request<'_> {
	fn eq(&self, other: &Self) -> bool {
		let this_v = self.params.as_ref().map(|r| r);
		let other_v = other.params.as_ref().map(|r| r);
		self.method == other.method && this_v == other_v
	}
}

//---------------------------------------------------------------------------------------------------- TESTS
mod test {
	use super::*;
	use crate::id::Id;

	#[test]
	fn serde() {
		let method = String::from("a_method");
		let params = serde_json::json!("[0, 1, 2]");
		let id     = Id::Num(123);

		let r = Request::new(
			Cow::Borrowed(&method),
			Some(Cow::Borrowed(&params)),
			Some(id.clone()),
		);

		assert!(!r.is_notification());

		let s: String = serde_json::to_string(&r).unwrap();
		let d: Request = serde_json::from_str(&s).unwrap();

		assert_eq!(d.method, method);
		assert_eq!(d.params.unwrap().as_ref(), &params);
		assert_eq!(d.id.unwrap(), id);
	}
}
