#[macro_export]
macro_rules! generate_function {
	($($func:ident),*) => {
		$(
			fn $func() {
				println!("Hello from {}",stringify!($func));
			}

		)*
	};
}

/*
// EXAMPLE
// lib.rs or main.rs
use crate::generate_function()

generate_function!(foo, bar, baz);

fn main() {
	foo!();
	baz!();
	bar!();
}
*/
