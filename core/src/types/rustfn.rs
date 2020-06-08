mod args;
mod binding;

pub use self::binding::Binding;
pub use self::args::Args;

use crate::{Object, types};
use std::fmt::{self, Debug, Formatter};

type FnType = fn(Args) -> crate::Result<Object>;

#[derive(Clone, Copy)]
pub struct RustFn(&'static str, FnType);

impl Debug for RustFn {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		f.debug_tuple("RustFn")
			.field(&self.0)
			// .field(&(self.1 as usize as *const ()))
			.finish()
	}
}

impl Eq for RustFn {}
impl PartialEq for RustFn {
	fn eq(&self, rhs: &RustFn) -> bool {
		let eql = (self.1 as usize) == (rhs.1 as usize);
		assert_eq!(eql, self.0 == rhs.0);
		eql
	}
}


impl RustFn {
	pub fn new(name: &'static str, func: FnType) -> Self {
		RustFn(name, func)
	}

	pub fn call(&self, args: Args) -> crate::Result<Object> {
		(self.1)(args)
	}
}


impl AsRef<FnType> for RustFn {
	fn as_ref(&self) -> &FnType {
		&self.1
	}
}

impl From<RustFn> for types::Text {
	fn from(rustfn: RustFn) -> Self {
		types::Text::new_static(rustfn.0)
	}
}

mod impls {
	use super::*;
	use crate::{Object, Result, Args};

	pub fn call(args: Args) -> Result<Object> {
		let this = args.this()?.try_downcast_ref::<RustFn>()?;
		this.call(args.args(..)?)
	}

	pub fn at_text(args: Args) -> Result<Object> {
		let this = *args.this()?.try_downcast_ref::<RustFn>()?;
		Ok(types::Text::from(this).into())
	}
}

impl_object_type!{
for RustFn [(parents super::Function)]:
	"@text" => impls::at_text,
	"()" => impls::call,
}