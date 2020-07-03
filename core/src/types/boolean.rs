use crate::{Object, Result, Args};
use crate::types::{Number, Text};
use std::fmt::{self, Debug, Display, Formatter};
use crate::attrs::{convert, operators, misc};

/// The Boolean type within Quest.
///
/// Internally, this is simply a newtype wrapping a `bool`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Boolean(bool);

impl Boolean {
	/// A constant representing the boolean value "false".
	pub const FALSE: Boolean = Boolean::new(false);

	/// A constant representing the boolean value "true".
	pub const TRUE: Boolean = Boolean::new(true);

	/// Simply create a new [`Boolean`].
	#[inline]
	pub const fn new(b: bool) -> Self {
		Boolean(b)
	}

	/// Unwraps the value.
	#[inline]
	pub const fn into_inner(self) -> bool {
		self.0
	}
}

impl PartialEq<bool> for Boolean {
	#[inline]
	fn eq(&self, rhs: &bool) -> bool {
		self.0 == *rhs
	}
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
	#[inline]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<bool> for Object {
	/// Converts this into a [`Boolean`] and then into an [`Object`]
	#[inline]
	fn from(inp: bool) -> Self {
		Boolean::new(inp).into()
	}
}

impl From<bool> for Boolean {
	#[inline]
	fn from(b: bool) -> Self {
		Boolean::new(b)
	}
}

impl From<Boolean> for bool {
	#[inline]
	fn from(b: Boolean) -> Self {
		b.into_inner()
	}
}

impl AsRef<bool> for Boolean {
	#[inline]
	fn as_ref(&self) -> &bool {
		&self.0
	}
}

impl AsMut<bool> for Boolean {
	#[inline]
	fn as_mut(&mut self) -> &mut bool {
		&mut self.0
	}
}

impl From<Boolean> for Number {
	/// Convert to a [`Number`] by mapping `true` to [`Number::ONE`] and `false` to
	/// [`Number::ZERO`]
	#[inline]
	fn from(b: Boolean) -> Self {
		match b.into_inner() {
			true => Number::ONE,
			false => Number::ZERO
		}
	}
}

impl From<Boolean> for Text {
	/// Convert to a [`Text`] by mapping `true` to `"true"` and `false` to `"false"`
	#[inline]
	fn from(b: Boolean) -> Self {
		const TRUE_TEXT: Text = Text::new_static("true");
		const FALSE_TEXT: Text = Text::new_static("false");
		match b.into_inner() {
			true => TRUE_TEXT,
			false => FALSE_TEXT
		}
	}
}

/*impl std::ops::BitAnd for Boolean {
	type Output = Self;

	#[inline]
	fn bitand(self, rhs: Self) -> Self {
		Self::from(self.0 & rhs.0)
	}
}

impl std::ops::BitAndAssign for Boolean {
	#[inline]
	fn bitand_assign(&mut self, rhs: Self) {
		self.0 &= rhs.0;
	}
}

impl std::ops::BitOr for Boolean {
	type Output = Self;

	#[inline]
	fn bitor(self, rhs: Self) -> Self {
		Self::from(self.0 | rhs.0)
	}
}

impl std::ops::BitOrAssign for Boolean {
	#[inline]
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0;
	}
}

impl std::ops::BitXor for Boolean {
	type Output = Self;

	#[inline]
	fn bitxor(self, rhs: Self) -> Self {
		Self::from(self.0 ^ rhs.0)
	}
}

impl std::ops::BitXorAssign for Boolean {
	#[inline]
	fn bitxor_assign(&mut self, rhs: Self) {
		self.0 ^= rhs.0;
	}
}

impl std::ops::Not for Boolean {
	type Output = Self;

	#[inline]
	fn not(self) -> Self {
		Self::from(!self.0)
	}
}
*//*
impl Boolean {
	/// See if a this is equal to the first argument.
	///
	/// Unlike most methods, the first argument is not implicitly converted to a  [`Boolean`] first.
	pub fn qs_eql(this: &Object, args: Args) -> Result<Boolean> {
		let rhs = args.arg(0)?;

		if this.is_identical(rhs) {
			Ok(Boolean::new(true))
		} else {
			this.try_with_ref::<Self, _, !, _>(|lhs| {
				rhs.with_ref::<Self, _, _>(|rhs| {
					Ok(Boolean::new(Some(lhs) == rhs))
				})
			})
		}
	}

	/// Compares this to the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_cmp(this: &Object, args: Args) -> Result<std::cmp::Ordering> {
		let rhs = args.arg(0)?;

		this.try_with_ref::<Self, _, _, _>(|lhs| {
			rhs.with_ref_call::<Self, _, !, _>(|rhs| {
				Ok(lhs.cmp(rhs))
			})
		})
	}

	/// Logical NOT of this.
	#[inline]
	pub fn qs_not(&self, _: Args) -> std::result::Result<Boolean, !> {
		Ok(!*self)
	}

	/// Logical AND of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitand(&self, args: Args) -> Result<Boolean> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		Ok(*self & rhs)
	}

	/// In-place logical AND of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitand_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		this.try_with_mut(|bool: &mut Self| Ok(*bool &= rhs))?;

		Ok(this.clone())
	}

	/// Logical OR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitor(&self, args: Args) -> Result<Boolean> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		Ok(*self | rhs)
	}

	/// In-place logical OR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	pub fn qs_bitor_assign(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		this.try_with_mut::<Self, _, !, _>(|bool: &mut Self| Ok(*bool |= rhs))?;

		Ok(this.clone())
	}

	pub fn qs_bitxor(&self, args: Args) -> Result<Boolean> {
		let rhs = args.arg(0)?.downcast_call::<Boolean>()?;

		Ok(*self ^ rhs)
	}


	/// The hash for this.
	#[inline]
	pub fn qs_hash(&self, _args: Args) -> Result<Object> {
		todo!("hash for Boolean")
	}
}*/

impl convert::Inspect for Boolean {
	/// Inspect the boolean.
	#[inline]
	fn call(this: &Object, _: Args) -> Result<Object> {
		this.try_with_ref::<Self, _, !, _>(|this| Ok(format!("{:?}", this).into()))
	}
}

impl convert::AtBoolean for Boolean {
	/// Convert this into a [`Boolean`].
	///
	/// This is simply a wrapper around [`Boolean::clone`](#method.clone).
	#[inline]
	fn call(this: &Object, _: Args) -> Result<Object> {
		this.try_with_ref::<Self, _, !, _>(|this| Ok(this.clone().into()))
	}
}

impl convert::AtNumber for Boolean {
	/// Convert this into a [`Number`].
	///
	/// This is simply a wrapper around [`Number::from(Boolean)`](Number#impl-From<Boolean>).
	#[inline]
	fn call(this: &Object, _: Args) -> Result<Object> {
		this.try_with_ref::<Self, _, !, _>(|this| Ok(Number::from(*this).into()))
	}
}

impl convert::AtText for Boolean {
	/// Convert this into a [`Text`].
	///
	/// This is simply a wrapper around [`Text::from(Boolean)`](Number#impl-From<Boolean>).
	#[inline]
	fn call(this: &Object, _: Args) -> Result<Object> {
		this.try_with_ref::<Self, _, !, _>(|bool| Ok(Text::from(*bool).into()))
	}
}

impl operators::Not for Boolean {
	/// Logical NOT of this.
	#[inline]
	fn call(this: &Object, _: Args) -> Result<Object> {
		this.try_with_ref::<Self, _, !, _>(|this| Ok((!this.0).into()))
	}
}

impl operators::Eql for Boolean {
	/// See if a this is equal to the first argument.
	///
	/// Unlike most methods, the first argument is not implicitly converted to a  [`Boolean`] first.
	fn call(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;

		if this.is_identical(rhs) {
			Ok(true.into())
		} else {
			this.try_with_ref::<Self, _, !, _>(|lhs| {
				rhs.with_ref::<Self, _, _>(|rhs| {
					Ok((Some(lhs) == rhs).into())
				})
			})
		}
	}
}

impl operators::Cmp for Boolean {
	/// Compares this to the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	fn call(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;

		this.try_with_ref::<Self, _, _, _>(|lhs| {
			rhs.with_ref_call::<Self, _, !, _>(|rhs| {
				Ok(lhs.cmp(rhs).into())
			})
		})
	}
}

impl operators::BitAnd for Boolean {
	/// Logical AND of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	fn call(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;
		this.try_with_ref::<Self, _, _, _>(|lhs| {
			rhs.with_ref_call::<Self, _, !, _>(|rhs| {
				Ok((lhs.0 & rhs.0).into())
			})
		})
	}
}

impl operators::BitAndAssign for Boolean {
	/// In-place logical AND of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	fn call(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;
		// `true & true = true, false & false = false`
		if !this.is_identical(rhs) {
			this.try_with_mut::<Self, _, _, _>(|lhs| {
				rhs.with_ref_call::<Self, _, !, _>(|rhs| {
					Ok(lhs.0 &= rhs.0)
				})
			})?;
		}
		Ok(this.clone())
	}
}

impl operators::BitOr for Boolean {
	/// Logical OR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	fn call(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;
		this.try_with_ref::<Self, _, _, _>(|lhs| {
			rhs.with_ref_call::<Self, _, !, _>(|rhs| {
				Ok((lhs.0 | rhs.0).into())
			})
		})
	}
}

impl operators::BitOrAssign for Boolean {
	/// In-place logical OR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	fn call(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;
		// `true | true = true, false | false = false`
		if !this.is_identical(rhs) {
			this.try_with_mut::<Self, _, _, _>(|lhs| {
				rhs.with_ref_call::<Self, _, !, _>(|rhs| {
					Ok(lhs.0 |= rhs.0)
				})
			})?;
		}
		Ok(this.clone())
	}
}

impl operators::BitXor for Boolean {
	/// Logical XOR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	fn call(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;
		this.try_with_ref::<Self, _, _, _>(|lhs| {
			rhs.with_ref_call::<Self, _, !, _>(|rhs| {
				Ok((lhs.0 ^ rhs.0).into())
			})
		})
	}
}

impl operators::BitXorAssign for Boolean {
	/// In-place logical XOR of this and the first argument.
	///
	/// The first argument is converted to a [`Boolean`] if it isn't already.
	fn call(this: &Object, args: Args) -> Result<Object> {
		let rhs = args.arg(0)?;
		// `true ^ true = false, false ^ false = false`
		if !this.is_identical(rhs) {
			this.try_with_mut::<Self, _, _, _>(|lhs| {
				rhs.with_ref_call::<Self, _, !, _>(|rhs| {
					Ok(lhs.0 |= rhs.0)
				})
			})?;
		} else {
			this.try_with_mut::<Self, _, !, _>(|lhs| Ok(lhs.0 = false))?;
		}

		Ok(this.clone())
	}
}

impl misc::Hash for Boolean {
	/// The hash for this.
	fn call(this: &Object, args: Args) -> Result<Object> {
		unimplemented!()
	}
}

impl_object_type!{
for Boolean {
	#[inline]
	fn new_object(self) -> Object where Self: Sized {
		use lazy_static::lazy_static;
		use crate::types::ObjectType;

		lazy_static! {
			static ref TRUE: Object = Object::new_with_parent(Boolean::TRUE, vec![Boolean::mapping()]);
			static ref FALSE: Object = Object::new_with_parent(Boolean::FALSE, vec![Boolean::mapping()]);
		}

		if self.into_inner() { 
			TRUE.deep_clone()
		} else {
			FALSE.deep_clone()
		}
	}
}

[(parents super::Basic) (convert "@bool")]:
	"__inspect__" => function <Boolean as self::convert::Inspect>::call,
	"@text"       => function <Boolean as self::convert::AtText>::call,
	"@num"        => function <Boolean as self::convert::AtNumber>::call,
	"@bool"       => function <Boolean as self::convert::AtBoolean>::call,
	"=="          => function <Boolean as operators::Eql>::call,
	"!"           => function <Boolean as operators::Not>::call,
	"&"           => function <Boolean as operators::BitAnd>::call,
	"&="          => function <Boolean as operators::BitAndAssign>::call,
	"|"           => function <Boolean as operators::BitOr>::call,
	"|="          => function <Boolean as operators::BitOrAssign>::call,
	"^"           => function <Boolean as operators::BitXor>::call,
	"^="          => function <Boolean as operators::BitXorAssign>::call,
	"<=>"         => function <Boolean as operators::Cmp>::call,
	"hash"        => function <Boolean as misc::Hash>::call,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::attrs::{convert::*, operators::*};
	use crate::types::ObjectType;

	#[test]
	fn at_num() {
		Boolean::_wait_for_setup_to_finish();
		assert_obj_eq!(<Boolean as AtNumber>::call(&true.into(), args!()).unwrap(), Number::ONE);
		assert_obj_eq!(<Boolean as AtNumber>::call(&false.into(), args!()).unwrap(), Number::ZERO);
	}

	#[test]
	fn at_text() {
		Boolean::_wait_for_setup_to_finish();
		assert_obj_eq!(<Boolean as AtText>::call(&true.into(), args!()).unwrap(), Text::from("true"));
		assert_obj_eq!(<Boolean as AtText>::call(&false.into(), args!()).unwrap(), Text::from("false"));
	}

	#[test]
	fn at_bool() {
		Boolean::_wait_for_setup_to_finish();
		assert_obj_eq!(<Boolean as AtBoolean>::call(&true.into(), args!()).unwrap(), Boolean::TRUE);
		assert_obj_eq!(<Boolean as AtBoolean>::call(&false.into(), args!()).unwrap(), Boolean::FALSE);
	}

	#[test]
	fn eql() {
		Boolean::_wait_for_setup_to_finish();
		assert_obj_eq!(<Boolean as Eql>::call(&true.into(), args!(true)).unwrap(), true);
		assert_obj_eq!(<Boolean as Eql>::call(&true.into(), args!(false)).unwrap(), false);
		assert_obj_eq!(<Boolean as Eql>::call(&false.into(), args!(true)).unwrap(), false);
		assert_obj_eq!(<Boolean as Eql>::call(&false.into(), args!(false)).unwrap(), true);
	}

	#[test]
	fn not() {
		Boolean::_wait_for_setup_to_finish();
		assert_obj_eq!(<Boolean as Not>::call(&true.into(), args!()).unwrap(), Boolean::FALSE);
		assert_obj_eq!(<Boolean as Not>::call(&false.into(), args!()).unwrap(), Boolean::TRUE);
	}

	#[test]
	fn bitand() {
		Boolean::_wait_for_setup_to_finish();
		assert_obj_eq!(<Boolean as BitAnd>::call(&true.into(), args!(true)).unwrap(), Boolean::TRUE);
		assert_obj_eq!(<Boolean as BitAnd>::call(&true.into(), args!(false)).unwrap(), Boolean::FALSE);
		assert_obj_eq!(<Boolean as BitAnd>::call(&false.into(), args!(true)).unwrap(), Boolean::FALSE);
		assert_obj_eq!(<Boolean as BitAnd>::call(&false.into(), args!(false)).unwrap(), Boolean::FALSE);
	}

	#[test]
	#[ignore]
	fn bitand_assign() { todo!() }

	#[test]
	fn bitor() {
		Boolean::_wait_for_setup_to_finish();
		assert_obj_eq!(<Boolean as BitOr>::call(&true.into(), args!(true)).unwrap(), Boolean::TRUE);
		assert_obj_eq!(<Boolean as BitOr>::call(&true.into(), args!(false)).unwrap(), Boolean::TRUE);
		assert_obj_eq!(<Boolean as BitOr>::call(&false.into(), args!(true)).unwrap(), Boolean::TRUE);
		assert_obj_eq!(<Boolean as BitOr>::call(&false.into(), args!(false)).unwrap(), Boolean::FALSE);
	}

	#[test]
	#[ignore]
	fn bitor_assign() { todo!() }

	#[test]
	fn bitxor() {
		Boolean::_wait_for_setup_to_finish();
		assert_obj_eq!(<Boolean as BitXor>::call(&true.into(), args!(true)).unwrap(), Boolean::FALSE);
		assert_obj_eq!(<Boolean as BitXor>::call(&true.into(), args!(false)).unwrap(), Boolean::TRUE);
		assert_obj_eq!(<Boolean as BitXor>::call(&false.into(), args!(true)).unwrap(), Boolean::TRUE);
		assert_obj_eq!(<Boolean as BitXor>::call(&false.into(), args!(false)).unwrap(), Boolean::FALSE);
	}

	#[test]
	#[ignore]
	fn bitxor_assign() { todo!() }


	#[test]
	#[ignore]
	fn cmp() { todo!(); }

	#[test]
	#[ignore]
	fn hash() { todo!(); }
}