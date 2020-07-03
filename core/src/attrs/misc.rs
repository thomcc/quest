use crate::{Object, Args, Result};
// use crate::error::TypeError;
// use crate::types::{Text, Boolean, Number, List};
use crate::literals::Literal;

pub trait Hash {
	const METHOD: Literal = "hash";

	fn call(this: &Object, args: Args) -> Result<Object>;
}