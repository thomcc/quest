use crate::{Object, Args, Result};
// use crate::error::TypeError;
// use crate::types::{Text, Boolean, Number, List};
use crate::literals::Literal;

macro_rules! define_conversion_trait {
	($($trait:ident $method:literal/* $type:ty*/),*) => {
		$(
			pub trait $trait {
				const METHOD: Literal = $method;

				// #[inline]
				fn call(this: &Object, args: Args) -> Result<Object>;
					// let obj = Self::convert(this, args)?;
					// if obj.is_a::<$type>() {
					// 	Ok(obj)
					// } else {
					// 	Err(TypeError::ConversionReturnedBadType {
					// 		func: Self::METHOD,
					// 		expected: std::any::type_name::<$type>(),
					// 		got: obj.typename()
					// 	}.into())
					// }
				// }

				// fn convert(this: &Object, args: Args) -> Result<$type>;
			}
		)*
	};
}

define_conversion_trait! {
	Inspect "__inspect__" /*Text*/,
	AtText "@text" /*Text*/,
	AtBoolean "@bool" /*Boolean*/,
	AtNumber "@num" /*Number*/,
	AtList "@list" /*List*/
}