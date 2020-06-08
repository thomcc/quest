#![deny(warnings)]

use quest::{Object, Binding};
use quest_parser::{Result as ParseResult, Stream, Expression};
use std::convert::TryFrom;

// TODO: repl
mod repl;

use std::env;

fn main() {
	let filename = env::args().nth(1).unwrap_or_else(|| "code/test.qs".to_string());
	let mut stream = Stream::try_from(<_ as AsRef<std::path::Path>>::as_ref(&filename))
		.expect("couldn't open file")
		.collect::<ParseResult<Vec<_>>>()
		.unwrap()
		.into_iter();

	let expression = Expression::try_from_iter(&mut stream).unwrap();
	let mut args: Vec<Object> = std::env::args()
		.skip(1)
		.map(Object::from)
		.collect::<Vec<Object>>();
	args.insert(0, Object::default());
	let result = Binding::new_stackframe(args.into(), |_| expression.execute());
	if cfg!(debug) {
		println!("{:?}", result);
	} else {
		result.unwrap();
	}
}