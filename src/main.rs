use std::path::Path;
use clap::Parser;

mod view;

#[derive(Parser)]
struct Cli {
	output: String,
	dir: String
}

fn main() -> std::io::Result<()> {
	let args = Cli::parse();
	let output = args.output;
	let p = Path::new(output.as_str());

	if !p.exists() {
		std::fs::create_dir(p).expect("failed to create dest");
	}

	println!("============================");
	println!(" view-folder");
	println!("============================");
	println!("Destination directory: {:?}", output);
	println!(" - Source directory: {:?}", args.dir);


	let mut v = view::View::new(output);
	v.add_include_extension(String::from("md").as_str());
	//v.add_include_extension(String::from("png").as_str());
	println!("-------------------");
	v.add_directory(&args.dir)?;

	let stat = v.get_stat().unwrap();
	println!("DIR:{}\nLINKS:{}", stat.dirs, stat.links);

	Ok(())
}
