
use clap::Parser;

mod server;
mod handler;
mod model;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(short, long, value_parser, default_value = "127.0.0.1")]
	host: String,
	#[clap(short, long, value_parser, default_value_t = 3030)]
	port: u16,
	#[clap(short, long, value_parser, default_value = ".")]
	root: String,
	#[clap(short, long, action)]
	open: bool,
}

fn main() {
	let args = Args::parse();
	println!("{:?}", args);
	

	if args.open {
		open::that(format!("http://{}:{}", args.host, args.port)).unwrap();
	}

	server::run(args.host.as_str(), args.port)
	
}
