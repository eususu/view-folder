use std::path::{Path};
use std::path::PathBuf;
use std::os::unix::fs;

pub struct View {
	dest_base: String,
  include_extensions: Vec<String>
}

impl View {
	pub fn new(dest_base: String) -> View {

    let v: Vec<String> = Vec::new();
		View {
			dest_base,
      include_extensions: v
		}
	}

  pub fn add_include_extension(&mut self, _ext: &str) {
    self.include_extensions.push(_ext.to_string());
    //self.include_extensions.push(String::from(_ext))
  }

	fn trace_directory(&self, _dest: & PathBuf, _dir: & PathBuf) -> std::io::Result<()> {
		let mut dest = PathBuf::from(_dest.as_path());
		let mut list = _dir.components();
		let is_file = _dir.is_file();

		let mut filename = "";
		if is_file {

      let ext = String::from(_dir.extension().unwrap().to_str().unwrap());

      if !self.include_extensions.contains(&ext) {
        //println!("NOT INCLUDE {:?}", _dir.extension());
        return Ok(())
      }

			filename = _dir.file_name().unwrap().to_str().unwrap();
		};

		// bypass root dir
		list.next();

		for p in list {
			if is_file {
				if p.as_os_str() == filename {
				dest.push(p);
					dest.set_file_name(&filename);
					break;
				}
			}
			dest.push(p);

			if !dest.exists() {
				std::fs::create_dir(&dest)?;
			}
		}

		if is_file && !dest.exists() {
			println!("LINK {:?} ============ {:?}", dest, _dir);
			let cdir = std::fs::canonicalize(&_dir)?;
			fs::symlink(&cdir, dest)?;
		}
		Ok(())
	}

	pub fn add_directory(&self, _dir: &str) -> std::io::Result<()> {
		// trace directory
		let base_path = Path::new(_dir);

		for entry in base_path.read_dir().expect("read_dir failed") {
			if let Ok(entry) = entry {
				let target = PathBuf::from(self.dest_base.to_string());

				self.trace_directory(&target, &entry.path())?;
				if entry.path().is_dir() {
					self.add_directory(entry.path().to_str().unwrap())?;
				}
			}
		}
		Ok(())
	}
}
