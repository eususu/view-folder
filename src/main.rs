use std::path::{Path, Component};
use std::path::PathBuf;
use std::os::unix::fs;


struct View {
	dest_base: String,
}

impl View {
	pub fn new(dest_base: String) -> Self {
		Self {
			dest_base
		}
	}

	fn trace_directory(&self, _dest: & mut PathBuf, _dir: & PathBuf) -> std::io::Result<()> {

		println!("BEGINO TRACE");

		let mut dest = PathBuf::from(_dest.as_path());


		let mut list = _dir.components();
		let is_dir = _dir.is_dir();
		let is_file = _dir.is_file();

		let mut filename = "";
		if is_file {
			filename = _dir.file_name().unwrap().to_str().unwrap();
		};

		// bypass root dir
		list.next();

		for p in list {

			if is_file {
					println!("FILE FILE  {:?} == {:?}", dest, filename);
				if p.as_os_str() == filename {
				dest.push(p);
					dest.set_file_name(&filename);
					println!("FILENAME");
					break;
				}
			}
			dest.push(p);

			if !dest.exists() {
				println!("MKDIR {:?}", dest);
				std::fs::create_dir(&dest);
			}
		}
		println!("MIDDLE {:?}", dest);

		if is_file {
			println!("LINK {:?} ============ {:?}", dest, _dir);
			let cdir = std::fs::canonicalize(&_dir)?;
			fs::symlink(&cdir, dest);
		}





		Ok(())
	}

	pub fn add_directory(&self, _dir: &str) -> std::io::Result<()> {
		// trace directory
		let base_path = Path::new(_dir);

		//println!(">>>>>>>>>> {:?}", base_path.to_str());

		for entry in base_path.read_dir().expect("read_dir failed") {
			if let Ok(entry) = entry {
				//println!(">> entry={:?}", entry.path());

				let mut target = PathBuf::from(self.dest_base.to_string());

				self.trace_directory(& mut target, &entry.path());
				if entry.path().is_dir() {
					self.add_directory(entry.path().to_str().unwrap());
				}
			}
		}
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

	let v = View::new(output);
	println!("-------------------");
	v.add_directory("example");


	Ok(())
}
