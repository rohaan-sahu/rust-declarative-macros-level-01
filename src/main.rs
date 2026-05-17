mod generate_function;
//use crate::generate_function;

/*--Ghode in the Stable macro--*/
generate_function!(foo,bar,baz);

/*--Macros--*/
macro_rules! say_hello {
	() => {
		println!("hello");
	};
}

macro_rules! say {
	($msg:expr) => {
		println!("{}",$msg);
	};
}

macro_rules! add_and_print {
	($a:expr, $b: expr) => {
		println!("{} + {} = {}",$a,$b,$a + $b);
	};
}

macro_rules! print_all {
	($($item:expr),*) => {
		$(
			println!("{0}",$item);
		)*
	};
}

macro_rules! describe {
	($val1:expr, $val2:expr) => {
		println!("got two things: {} and {}",$val1,$val2);
	};
	($($valx:expr),*)=> {
		println!("greedy many");
	};
	($val:expr) => {
		println!("got one thing: {}",$val);
	};
	() => {
		println!("nothing was passed");
	};
}

macro_rules! sum {
	($x:expr) => {
		$x
	};

	($x:expr, $($rest:expr),+) => {
		$x + sum!($($rest),+)
	};
}

macro_rules! make_struct {
	($name:ident, $field:ident, $type:ty) => {
		struct $name {
			$field: $type,
		}

		impl $name {
			fn new(val: $type)-> $name {
				$name {$field: val}
			}

			fn get(&self) ->&$type {
				&self.$field
			}
		}
	};
}

/*--Structs--*/
make_struct!(Point,x,f64);
make_struct!(Label, text, String);

/*--Main function--*/

fn main() {
	println!("\n100xdevs notes. Ghode Khol assignment",);
	foo();
	baz();
	bar();

	println!("\nSimplest possible Macro");
	say_hello!();

	println!("\nSingle Argument");
	say!("Hello world");
	say!(1+1);
	say!("the answer is");

	println!("\nMultiple Arguments");
	add_and_print!(2,3);
	add_and_print!(10,20);
	// add_and_print!(2,3,4);  // pattern matching error

	println!("\nRepetition with '$(),*'");
	print_all!(1,2,3);
	print_all!("hello","world");
	print_all!(42);

	println!("\nMultiple Match Arm - greedy arm ON");
	describe!();
	describe!(1);
	describe!(2,3);

	println!("\nRecursive Macro");
	println!("{}",sum!(1,2,3,4));

	println!("\nGenerating Struct & Impls");
	let p = Point::new(3.14);
	println!("Point created with Macro: {}",p.get());

	let l = Label::new(String::from("hello"));
	println!("Label created by Macro: {}",l.get());
}
