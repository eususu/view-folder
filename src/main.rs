struct View {
	dest_base: String
}

impl View {
	pub fn new(dest_base: String) -> Self {
		Self {
			dest_base,
		}
	}

	pub fn add_target(&self, _target: String) {

		println!("{}", self.dest_base);
	}
}

fn main() {
	let output = String::from("output/test.md");
	let target1 = String::from("/Users/lovian/my/gift-account/test.md");

	println!("Hello, world!");

	let v = View::new(output);

	v.add_target(target1);


}
