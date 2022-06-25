use std::path::Path;
use std::path::PathBuf;
use std::os::unix::fs;


struct View {
	dest_base: String
}

impl View {
	pub fn new(dest_base: String) -> Self {
		Self {
			dest_base,
		}
	}

	pub fn add_directory(&self, _dir: &str) -> std::io::Result<()> {
		// trace directory
		Ok(())
	}

	pub fn add_target(&self, _target: String) -> std::io::Result<()> {

		println!("#destination path: {}", self.dest_base);

		let path = Path::new(&_target);

		// extract filename
		let f = path.file_stem().unwrap().to_str().unwrap();
		let ext = path.extension().unwrap().to_str().unwrap();

		println!("#file: {}.{}", f, ext);

		// make link
		let mut p = PathBuf::new();
		let mut file = PathBuf::new();
		p.push(&self.dest_base);
		file.set_file_name(format!("{}.{}", f, ext));
		p.push(file.as_path());

		println!("#TRY path = {}", p.to_str().unwrap());
		match fs::symlink(path, p) {
			Ok(_b) => Ok(()),
			Err(_error) => Ok(())
		}
	}
}

fn main() -> std::io::Result<()> {
	let output = String::from("output");
	let target1 = String::from("../example/test1.md");
	let target2 = String::from("../example/test2.md");




	let v = View::new(output);
	v.add_directory("example");

	v.add_target(target1)?;
	println!("-------------------");
	v.add_target(target2)?;
	

	//let _ = fs::symlink(target1, output);
	Ok(())
}
