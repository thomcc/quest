use std::{fmt::Debug, any::Any};
use crate::Object;

#[macro_use]
pub mod macros;

mod convert;

pub trait ObjectType : Debug + Any + Send + Sync + Clone {
	fn mapping() -> Object;

	#[inline]
	fn init() {
		#[allow(unused)]
		Self::mapping();
	}

	#[inline]
	fn new_object(self) -> Object where Self: Sized {
		Object::new_with_parent(self, vec![Self::mapping()])
	}

	// #[cfg(test)]
	// todo: remove this
	fn _wait_for_setup_to_finish() {}
}

pub mod pristine;
pub mod kernel;
pub mod basic;

pub mod function;
pub mod comparable;
pub mod bound_function;
pub mod rustfn;
// pub mod block;
pub mod scope;

pub mod null;
pub mod boolean;
pub mod number;
pub mod text;

pub mod list;

pub use convert::Convertible;
pub use comparable::Comparable;
pub use pristine::Pristine;
pub use kernel::Kernel;
pub use basic::Basic;
pub use bound_function::BoundFunction;
pub use function::Function;
pub use rustfn::{RustFn, ArgsOld};
// pub use block::Block;
pub use scope::Scope;
pub use null::Null;
pub use boolean::Boolean;
pub use number::Number;
pub use text::Text;
pub use list::List;