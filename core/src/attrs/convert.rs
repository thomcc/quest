use crate::{Object, Args, Result};
use crate::types::{Text, Boolean, Number, List};
use crate::literals::Literal;

macro_rules! define_at_trait {
	($($trait:ident $method:literal $type:ty),*) => {
		$(
			pub trait $trait {
				const METHOD: Literal = $method;

				fn call(this: &Object, args: Args) -> Result<$type>;
			}
		)*
	};
}

define_at_trait! {
	Inspect "__inspect__" Text,
	AtText "@text" Text,
	AtBoolean "@bool" Boolean,
	AtNumber "@num" Number,
	AtList "@list" List
}