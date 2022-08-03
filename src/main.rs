use std::path::PathBuf;
use clap::Parser;
mod view;

#[derive(Parser)]
#[clap(name = "view-folder")]
#[clap(author = "belovian@gmail.com")]
#[clap(version, about, long_about = None)]
struct Cli {
	/// specify output directory, if not exist, make it automatically.
	#[clap(short, long, value_parser, value_name = "output directory")]
	output: PathBuf,

	/// specify a file extension for include
	#[clap(short, long, required = true, multiple_values = true, value_parser, value_name = "file extension")]
	extension: Vec<String>,

	/// specify input file or directory.
	#[clap(value_parser, multiple_values = true, value_name = "input file or directory")]
	input: Vec<PathBuf>,

	/// use hard link instead of symbolic
	#[clap(long, action= clap::ArgAction::Count )]
	hard_link: u8,

	/// Turn debugging information on
	#[clap(short, long, action = clap::ArgAction::Count)]
	debug: u8
}

fn main() -> std::io::Result<()> {
	let args = Cli::parse();
	let output = args.output;
	let exts = args.extension;

	let symbolic = if args.hard_link == 0 { true } else { false };

	for input_path in args.input.iter() {
		if !input_path.exists() {
			println!("ionput path({:?}) does not exist", input_path);
			std::process::exit(2);
		}
	}

	if !output.as_path().exists() {
		std::fs::create_dir(output.as_path()).expect("failed to create dest");
	}

	if args.debug != 0 {
		println!(">> Destination directory: {:?}", output);
		println!(">> Source directory: {:?}", args.input);
	}

	let mut v = view::View::new(output);

	v.set_symbolic(symbolic);

	if args.debug != 0 {
		v.set_verbose(true);
	}

	// add include extensions
	for ext in exts.iter() {
		println!(">> ext:{:?}", ext);

		v.add_include_extension(ext);
	}

	// add include directory
	for input_path in args.input.iter() {
		println!("add dir: {:?}", input_path);
		v.add_directory(input_path).unwrap_or_else(|error| {
			println!(">> Error: {}", error);
			std::process::exit(2);
		});
	}

	if args.debug != 0 {
		let stat = v.get_stat().unwrap();
		println!(">> DIR:{}", stat.dirs);
		println!(">> LINKS:{}", stat.links);
	}

	Ok(())
}
