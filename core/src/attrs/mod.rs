pub mod literals;

macro_rules! quest_method_trait {
	($($trait:ident $key:literal $fn:ident);* $(;)?) => {
		$(
			#[doc = "A trait representing the `"]
			#[doc = $key]
			#[doc = "` method within Quest"]
			pub trait $trait {
				/// The result of the operation.
				type Output;

				/// The key that will be used when calling this attribute.
				const KEY: crate::obj::Key = crate::obj::Key::Literal($key);

				#[doc = "Perform the `"]
				#[doc = $key]
				#[doc = "` operation."]
				fn $fn(&self, args: crate::Args) -> crate::Result<Self::Output>;
			}
		)*
	}
}

pub mod operators;
pub mod conversion;
pub use self::operators::*;
pub use self::conversion::*;