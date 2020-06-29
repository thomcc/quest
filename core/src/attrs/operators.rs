use std::ops::*;
use crate::{Args, Object, Result};
use crate::obj::Key;
use crate::types::Convertible;

quest_method_trait! {
	QsPos "+@" qs_pos;
	QsNeg "-@" qs_neg;
	QsAdd "+" qs_add;
	QsSub "-" qs_sub;
	QsMul "*" qs_mul;
	QsDiv "/" qs_div;
	QsMod "%" qs_mod;
	QsPow "**" qs_pow;
	QsCmp "<=>" qs_cmp;

	QsCall "()" qs_call;

	QsNot "!" qs_not;
	QsEql "==" qs_eql;
	QsNeq "!=" qs_neq;
	QsLth "<" qs_lth;
	QsGth ">" qs_gth;
	QsLeq "<=" qs_leq;
	QsGeq ">=" qs_geq;
	QsOr "||" qs_or;
	QsAnd "&&" qs_and;

	QsBitNot "~" qs_bitnot;
	QsBitAnd "&" qs_bitand;
	QsBitOr "|" qs_bitor;
	QsBitXor "^" qs_bitxor;
	QsShl "<<" qs_shl;
	QsShr ">>" qs_shr;
}

macro_rules! define_quest_method_assign {
	($($trait:ident $key:literal $fn:ident);* $(;)?) => {
		$(
			#[doc = "A trait representing the `"]
			#[doc = $key]
			#[doc = "` method within Quest"]
			pub trait $trait {
				/// The key that will be used when calling this attribute.
				const KEY: Key = Key::Literal($key);

				#[doc = "Perform the `"]
				#[doc = $key]
				#[doc = "` operation."]
				fn $fn(this: &Object, args: Args) -> Result<Object>;
			}
		)*
	}
}

define_quest_method_assign! {
	QsAssign "=" qs_assign;
	QsAddAssign "+=" qs_add_assign;
	QsSubAssign "-=" qs_sub_assign;
	QsMulAssign "*=" qs_mul_assign;
	QsDivAssign "/=" qs_div_assign;
	QsModAssign "%=" qs_mod_assign;
	QsPowAssign "**=" qs_pow_assign;
	QsBitAndAssign "&=" qs_bitand_assign;
	QsBitOrAssign "|=" qs_bitor_assign;
	QsBitXorAssign "^=" qs_bitxor_assign;
	QsShlAssign "<<=" qs_shl_assign;
	QsShrAssign ">>=" qs_shr_assign;
}

macro_rules! impl_quest_unary_operator {
	(
		$T:ident $rhs:ident
		$($qs_trait:ident $trait:path, $key:literal
		$qs_fn:ident $fn:ident $output:ty, $expr:expr);* $(;)?
	) => {
		$(
			impl<$T: $trait + Convertible + Clone> $qs_trait for $T {
				type Output = $output;

				#[inline]
				fn $qs_fn(&self, _: Args) -> Result<Self::Output> {
					Ok(self.clone().$fn())
				}
			}
		)*
	};
}

impl_quest_unary_operator!{ T rhs
	QsNeg Neg, "-@" qs_neg neg <Self as Neg>::Output, rhs;
	QsBitNot Not, "~" qs_bitnot not <Self as Not>::Output, rhs;
}

macro_rules! impl_quest_binary_operator {
	(
		$T:ident $rhs:ident
		$($qs_trait:ident $trait:path, $key:literal
		$qs_fn:ident $fn:ident $output:ty, $expr:expr);* $(;)?
	) => {
		$(
			impl<$T: $trait + Convertible + Clone> $qs_trait for $T {
				type Output = $output;

				#[doc = "Perform the `"]
				#[doc = $key]
				#[doc = "` operation with this and the first argument\n\n"]
				#[doc = "The first argument is converted to a `Self` if it isn't already."]
				#[inline]
				fn $qs_fn(&self, args: Args) -> Result<Self::Output> {
					let $rhs = args.arg(0)?.downcast_call::<Self>()?;

					Ok(self.clone().$fn($expr))
				}
			}
		)*
	};
}

impl_quest_binary_operator! { T rhs
	QsAdd Add<T>, "+" qs_add add <Self as Add>::Output, rhs;
	QsSub Sub<T>, "-" qs_sub sub <Self as Sub>::Output, rhs;
	QsMul Mul<T>, "*" qs_mul mul <Self as Mul>::Output, rhs;
	QsDiv Div<T>, "/" qs_div div <Self as Div>::Output, rhs;
	QsMod Rem<T>, "%" qs_mod rem <Self as Rem>::Output, rhs;

	QsLth PartialOrd, "<" qs_lth lt bool, &rhs;
	QsGth PartialOrd, ">" qs_gth gt bool, &rhs;
	QsLeq PartialOrd, "<=" qs_leq le bool, &rhs;
	QsGeq PartialOrd, ">=" qs_geq ge  bool, &rhs;
	QsCmp Ord, "<=>" qs_cmp cmp std::cmp::Ordering, &rhs;

	QsBitAnd BitAnd, "&" qs_bitand bitand <Self as BitAnd>::Output, rhs;
	QsBitOr BitOr, "|" qs_bitor bitor <Self as BitOr>::Output, rhs;
	QsBitXor BitXor, "^" qs_bitxor bitxor <Self as BitXor>::Output, rhs;
	QsShl Shl, "<<" qs_shl shl <Self as Shl>::Output, rhs;
	QsShr Shr, ">>" qs_shr shr <Self as Shr>::Output, rhs;
}

impl<T: PartialEq + Convertible + Clone> QsEql for T {
	type Output = bool;

	/// Perform the `==` operation with this and the first argument
	///
	/// Unlike most operators, this **doesn't** convert the first argument to `Self`
	#[inline]
	fn qs_eql(&self, args: Args) -> Result<bool> {
		match args.arg(0)?.downcast_ref::<Self>() {
			Some(val) if *self == *val => Ok(true),
			_ => Ok(false)
		}
	}
}

// QsNeq PartialEq, "!=" qs_neq ne bool, &rhs;


macro_rules! impl_quest_binary_assign_operator {
	($($qs_trait:ident $trait:ident $key:literal $qs_fn:ident $fn:ident);* $(;)?) => {
		$(
			impl<T: $trait<T> + Convertible + Clone> $qs_trait for T {
				#[doc = "Perform the `"]
				#[doc = $key]
				#[doc = "` operation with this and the first argument\n\n"]
				#[doc = "The first argument is converted to a `Self` if it isn't already."]
				#[inline]
				fn $qs_fn(this: &Object, args: Args) -> Result<Object> {
					let rhs = args.arg(0)?.downcast_call::<Self>()?;

					this.try_downcast_mut::<Self>()?.$fn(rhs);

					Ok(this.clone())
				}
			}
		)*
	};
}

impl_quest_binary_assign_operator! {
	QsAddAssign AddAssign "+=" qs_add_assign add_assign;
	QsSubAssign SubAssign "-=" qs_sub_assign sub_assign;
	QsMulAssign MulAssign "*=" qs_mul_assign mul_assign;
	QsDivAssign DivAssign "/=" qs_div_assign div_assign;
	QsModAssign RemAssign "%=" qs_mod_assign rem_assign;
	QsBitAndAssign BitAndAssign "&=" qs_bitand_assign bitand_assign;
	QsBitOrAssign BitOrAssign "|=" qs_bitor_assign bitor_assign;
	QsBitXorAssign BitXorAssign "^=" qs_bitxor_assign bitxor_assign;
	QsShlAssign ShlAssign "<<=" qs_shl_assign shl_assign;
	QsShrAssign ShrAssign ">>=" qs_shr_assign shr_assign;
}





