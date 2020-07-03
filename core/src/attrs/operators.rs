use crate::{Object, Args, Result};
use crate::literals::Literal;

// in the future, we should manually expand the macros, or at least add support for 
// doc comments
macro_rules! define_operator_trait {
	($($trait:ident $method:literal)*) => {
		$(
			pub trait $trait {
				const METHOD: Literal = $method;

				fn call(this: &Object, args: Args) -> Result<Object>;
			}
		)*
	};
}

define_operator_trait! {
	Eql "=="
	Cmp "<=>"
	Not "!"
	BitAnd "&" BitAndAssign "&="
	BitOr  "|" BitOrAssign  "|="
	BitXor "^" BitXorAssign "^="
}
