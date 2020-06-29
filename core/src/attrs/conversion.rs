use crate::{Args, obj::Key};
use std::convert::TryInto;
use crate::types::*;

/// A trait representing cloning of something
pub trait QsClone {
	/// The key that will be used when calling this attribute.
	const KEY: Key = Key::Literal("clone");

	/// Clone this object
	fn qs_clone(&self, _: Args) -> Result<Self, !> where Self: Sized;
}

impl<T: Clone> QsClone for T {
	#[inline]
	fn qs_clone(&self, _: Args) -> Result<Self, !> {
		Ok(self.clone())
	}
}

macro_rules! quest_conversion_trait {
	($($trait:ident $key:literal $fn:ident $intolit:literal $into:ty);* $(;)?) => {
		$(
			#[doc = "A trait representing a conversion to [`"]
			#[doc = $intolit]
			#[doc = "`] (via the `"]
			#[doc = $key]
			#[doc = "` method)"]
			pub trait $trait {
				/// What could go wrong?
				type Error;

				/// The key that will be used when calling this attribute.
				const KEY: Key = Key::Literal($key);

				#[doc = "Convert to a [`"]
				#[doc = $intolit]
				#[doc = "`]."]
				fn $fn(&self, _: Args) -> Result<$into, Self::Error>;
			}

			impl<T: Clone + TryInto<$into>> $trait for T {
				type Error = <T as TryInto<$into>>::Error;

				#[doc = "Convert to a [`"]
				#[doc = $intolit]
				#[doc = "`].\n\n"]
				#[doc = "This is simply a wrapper around the [`TryInto`] trait"]
				#[inline]
				fn $fn(&self, _: Args) -> Result<$into, Self::Error> {
					self.clone().try_into()
				}
			}
		)*
	}
}

quest_conversion_trait! {
	QsAtText "@text" qs_at_text "Text" Text;
	QsAtBool "@bool" qs_at_bool "Boolean" Boolean;
	QsAtNum  "@num"  qs_at_num  "Number" Number;
	QsAtList "@list" qs_at_list "List" List;
}
