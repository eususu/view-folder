use std::path::PathBuf;
use clap::Parser;

mod view;

#[derive(Parser)]
#[clap(name = "view-folder")]
#[clap(author = "belovian@gmail.com")]
#[clap(version, about, long_about = None)]
struct Cli {
	/// specify output directory, if not exist, make it automatically.
	#[clap(value_parser, value_name = "output directory")]
	output: PathBuf,

	/// specify input file or directory.
	#[clap(value_parser, value_name = "input file or directory")]
	input: PathBuf,

	/// Turn debugging information on
	#[clap(short, long, action = clap::ArgAction::Count)]
	debug: u8
}

fn main() -> std::io::Result<()> {
	let args = Cli::parse();
	let output = args.output;
	let source = args.input;

  if !source.exists() {
    println!("input path({:?}) is not exists.", source.as_path());
		std::process::exit(1);
  }

	if !output.as_path().exists() {
		std::fs::create_dir(output.as_path()).expect("failed to create dest");
	}

	if args.debug != 0 {
		println!(">> Destination directory: {:?}", output);
		println!(">> Source directory: {:?}", source);
	}

	let mut v = view::View::new(output);

	if args.debug != 0 {
		v.set_verbose(true);
	}

	// add include extensions
	v.add_include_extension("md");

	// add include directory
	v.add_directory(source).unwrap_or_else(|error| {
		println!(">> Error: {}", error);
		std::process::exit(2);
	});

	if args.debug != 0 {
		let stat = v.get_stat().unwrap();
		println!(">> DIR:{}", stat.dirs);
		println!(">> LINKS:{}", stat.links);
	}

	Ok(())
}
