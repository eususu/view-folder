use std::ffi::OsString;
use std::path::PathBuf;
use symlink::symlink_auto;

#[derive(Copy, Clone)]
pub struct Statistics {
  pub links: u64,
  pub dirs: u64,
}

pub struct View {
	dest_base: PathBuf,
  include_extensions: Vec<OsString>,
  symbolic: bool,
  verbose: bool,
  links: u64,
  dirs: u64
}

impl View {
	pub fn new(dest_base: PathBuf) -> View {
		View {
			dest_base,
      include_extensions: Vec::new(),
      symbolic: true,
      verbose: false,
      links:0, dirs:0
		}
	}

  pub fn set_symbolic(&mut self, _symbolic: bool) {
    self.symbolic = _symbolic
  }

  pub fn set_verbose(&mut self, _verbose:bool) {
    self.verbose = _verbose;
  }

  pub fn add_include_extension(&mut self, _ext: &str) {
    self.include_extensions.push(OsString::from(_ext));
  }

  fn link(&mut self, _target: &PathBuf, _source: & PathBuf) -> std::io::Result<()>  {
    let ext: OsString = if _target.extension().is_none() {
      OsString::from("")
      } else {
        _target.extension().unwrap().to_os_string()
      };
    if !self.include_extensions.contains(&ext) {
      if self.verbose {
        println!("NOT INCLUDE file {:?}", _target);
      }
      return Ok(())
    }
    self.links += 1;

    if _target.exists() {
      if self.verbose {
        println!("REMOVE FILE becauseof relinking ({:?})", _target);
      }
      // re link
      std::fs::remove_file(_target)?;
    }

    if self.symbolic {
      symlink_auto(_source, _target)
    } else {
      std::fs::hard_link(_source, _target)
    }
  }

  fn recurse(&mut self, _target: &PathBuf, _source: & PathBuf) -> std::io::Result<()> {
    // if source is file
    // if source is directory
    for entry in _source.read_dir().expect("read dir fail") {
      if let Ok(entry) = entry {

        let mut new_target = PathBuf::from(_target);
        new_target.push(entry.file_name());
        if entry.path().is_dir() {
          self.recurse(&new_target, &entry.path())?;
        } else {
          if !_target.exists() {
            self.dirs += 1;
            std::fs::create_dir_all(&_target)?;
          }

          self.link(&new_target, &entry.path())?;
        }

      }
    }
    Ok(())
  }

	pub fn add_directory(&mut self, _dir: &PathBuf) -> std::io::Result<()> {
    let b = PathBuf::from(_dir);
    let p = PathBuf::from(&self.dest_base);

    self.recurse(&p, &b)
	}

  pub fn get_stat(&self) -> std::io::Result<Statistics> {
    Ok(Statistics{
      dirs: self.dirs,
      links: self.links
    })
  }
}
