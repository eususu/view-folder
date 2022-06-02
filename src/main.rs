struct View<'a> {
	dest_base: &'a str
}

impl <'a>View {
	pub fn new(dest_base: &str) -> Self {
		Self {
			dest_base,
		}
	}
}

fn main() {
	let output:&str = "output/test.md";
	let target1:&str ="/Users/lovian/my/gift-account/test.md";

	println!("Hello, world!");

	let v = View::new(output);


}
