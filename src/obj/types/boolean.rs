use crate::obj::{self, Object, types};
use std::sync::{Arc, RwLock};
use std::fmt::{self, Debug, Display, Formatter};
use std::ops;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Boolean(bool);

impl Boolean {
	#[inline]
	pub const fn new(t: bool) -> Self {
		Boolean(t)
	}

	pub const FALSE: Boolean = Boolean::new(false);
	pub const TRUE: Boolean = Boolean::new(true);
}

impl Debug for Boolean {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		if f.alternate() {
			write!(f, "Boolean({:?})", self.0)
		} else {
			Display::fmt(self, f)
		}
	}
}

impl Display for Boolean {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<bool> for Object {
	fn from(inp: bool) -> Self {
		Boolean::new(inp).into()
	}
}

impl From<bool> for Boolean {
	fn from(b: bool) -> Self {
		Boolean::new(b)
	}
}

impl From<Boolean> for bool {
	fn from(b: Boolean) -> Self {
		b.0
	}
}

impl AsRef<bool> for Boolean {
	fn as_ref(&self) -> &bool {
		&self.0
	}
}

impl ops::Not for Boolean {
	type Output = Self;
	fn not(self) -> Self::Output {
		Boolean::new(!self.0)
	}
}

impl ops::BitAnd for Boolean {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self::Output {
		Boolean::new(self.0 & rhs.0)
	}
}

impl ops::BitOr for Boolean {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self::Output {
		Boolean::new(self.0 | rhs.0)
	}
}

impl ops::BitXor for Boolean {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self::Output {
		Boolean::new(self.0 ^ rhs.0)
	}
}

impl From<Boolean> for types::Number {
	fn from(b: Boolean) -> Self {
		const TRUE_NUMBER: types::Number = types::Number::ONE;
		const FALSE_NUMBER: types::Number = types::Number::ZERO;
		if b.0 { TRUE_NUMBER } else { FALSE_NUMBER }
	}
}

impl From<Boolean> for types::Text {
	fn from(b: Boolean) -> Self {
		const TRUE_TEXT: types::Text = types::Text::new_static("true");
		const FALSE_TEXT: types::Text = types::Text::new_static("false");
		if b.0 { TRUE_TEXT } else { FALSE_TEXT }
	}
}

mod impls {
	use super::*;
	use std::ops::{Deref, BitAnd, BitOr, BitXor, Not};
	use crate::obj::{Object, Result, Args, types, literals};

	pub fn at_num(args: Args) -> Result<Object> {
		let this = args.this_downcast::<Boolean>()?;
		Ok(types::Number::from(this).into())
	}

	pub fn at_text(args: Args) -> Result<Object> {
		let this = args.this_downcast::<Boolean>()?;
		Ok(types::Text::from(this).into())
	}

	pub fn at_bool(args: Args) -> Result<Object> {
		let this = args.this()?;
		debug_assert!(this.is_a::<Boolean>(), "bad `this` given");
		// TODO: forwarding args, make sure `self` is updated.
		this.call_attr(&literals::CLONE, args.args(..)?)
	}

	pub fn clone(args: Args) -> Result<Object> {
		let this = args.this_downcast::<Boolean>()?;
		Ok(this.into())
	}

	pub fn eql(args: Args) -> Result<Object> {
		let this = args.this_downcast::<Boolean>()?;
		Ok(args.arg_downcast::<Boolean>(0)
				.map(|rhs| (this == rhs).into())
				.unwrap_or(Boolean::FALSE)
				.into())
	}

	pub fn not(args: Args) -> Result<Object> {
		let this = args.this_downcast::<Boolean>()?;
		Ok((!this).into())
	}

	pub fn bitand(args: Args) -> Result<Object> {
		let this = args.this_downcast::<Boolean>()?;
		let rhs = args.arg_call_into::<Boolean>(0)?;
		Ok((this & rhs).into())
	}

	pub fn bitor(args: Args) -> Result<Object> {
		let this = args.this_downcast::<Boolean>()?;
		let rhs = args.arg_call_into::<Boolean>(0)?;
		Ok((this | rhs).into())
	}

	pub fn bitxor(args: Args) -> Result<Object> {
		let this = args.this_downcast::<Boolean>()?;
		let rhs = args.arg_call_into::<Boolean>(0)?;
		Ok((this ^ rhs).into())
	}

	pub fn cmp(args: Args) -> Result<Object> {
		todo!("cmp for Boolean")
	}

	pub fn hash(args: Args) -> Result<Object> {
		todo!("hash for Boolean")
	}
}

impl_object_type!{
for Boolean [(parent super::Basic) (convert "@bool")]:
	literals::AT_NUM  => impls::at_num,
	literals::AT_TEXT => impls::at_text,
	literals::AT_BOOL => impls::at_bool,
	literals::CLONE   => impls::clone,
	literals::EQL     => impls::eql,
	literals::NOT     => impls::not,
	literals::BAND    => impls::bitand,
	literals::BOR     => impls::bitor,
	literals::BXOR    => impls::bitxor,
	literals::CMP     => impls::cmp,
	literals::HASH    => impls::hash,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn at_num() {
		assert_call_eq!(for Boolean;
			types::Number::ONE, at_num(Boolean::TRUE) -> Number,
			types::Number::ZERO, at_num(Boolean::FALSE) -> Number
		);
	}

	#[test]
	fn at_text() {
		assert_call_eq!(for Boolean;
			Text::from("true"), at_text(Boolean::TRUE) -> Text,
			Text::from("false"), at_text(Boolean::FALSE) -> Text
		);
	}

	#[test]
	fn at_bool() {
		assert_call_eq!(for Boolean;
			Boolean::TRUE, at_bool(Boolean::TRUE) -> Boolean,
			Boolean::FALSE, at_bool(Boolean::FALSE) -> Boolean
		);
	}

	#[test]
	fn clone() {
		assert_call_eq!(for Boolean;
			Boolean::TRUE, at_bool(Boolean::TRUE) -> Boolean,
			Boolean::FALSE, at_bool(Boolean::FALSE) -> Boolean
		);
	}

	#[test]
	fn eql() {
		assert_call_eq!(for Boolean;
			Boolean::TRUE, eql(Boolean::TRUE, Boolean::TRUE) -> Boolean,
			Boolean::FALSE, eql(Boolean::TRUE, Boolean::FALSE) -> Boolean,
			Boolean::FALSE, eql(Boolean::FALSE, Boolean::TRUE) -> Boolean,
			Boolean::TRUE, eql(Boolean::FALSE, Boolean::FALSE) -> Boolean
		);
	}

	#[test]
	fn not() {
		assert_call_eq!(for Boolean;
			Boolean::FALSE, not(Boolean::TRUE) -> Boolean,
			Boolean::TRUE, not(Boolean::FALSE) -> Boolean
		);
	}

	#[test]
	fn bitand() {
		assert_call_eq!(for Boolean;
			Boolean::TRUE, bitand(Boolean::TRUE, Boolean::TRUE) -> Boolean,
			Boolean::FALSE, bitand(Boolean::TRUE, Boolean::FALSE) -> Boolean,
			Boolean::FALSE, bitand(Boolean::FALSE, Boolean::TRUE) -> Boolean,
			Boolean::FALSE, bitand(Boolean::FALSE, Boolean::FALSE) -> Boolean
		);
	}

	#[test]
	fn bitor() {
		assert_call_eq!(for Boolean;
			Boolean::TRUE, bitor(Boolean::TRUE, Boolean::TRUE) -> Boolean,
			Boolean::TRUE, bitor(Boolean::TRUE, Boolean::FALSE) -> Boolean,
			Boolean::TRUE, bitor(Boolean::FALSE, Boolean::TRUE) -> Boolean,
			Boolean::FALSE, bitor(Boolean::FALSE, Boolean::FALSE) -> Boolean
		);
	}

	#[test]
	fn bitxor() {
		assert_call_eq!(for Boolean;
			Boolean::FALSE, bitxor(Boolean::TRUE, Boolean::TRUE) -> Boolean,
			Boolean::TRUE, bitxor(Boolean::TRUE, Boolean::FALSE) -> Boolean,
			Boolean::TRUE, bitxor(Boolean::FALSE, Boolean::TRUE) -> Boolean,
			Boolean::FALSE, bitxor(Boolean::FALSE, Boolean::FALSE) -> Boolean
		);
	}

	#[test]
	#[ignore]
	fn cmp() { todo!(); }

	#[test]
	#[ignore]
	fn hash() { todo!(); }
}