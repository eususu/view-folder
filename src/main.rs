mod view;

fn main() -> std::io::Result<()> {
	let output = String::from("output");

	let mut v = view::View::new(output);
	v.add_include_extension(String::from("md").as_str());
	println!("-------------------");
	let res = v.add_directory("example");
	res
}
