use std::thread;
use std::sync::{Mutex};
use std::cell::RefCell;

use futures::executor::block_on;
#[macro_use]
extern crate lazy_static;

use crate::model::{ChatSession, Args};

mod server;
mod handler;
mod model;
mod utils;
mod watcher;

lazy_static! {
	static ref SOCKET_VEC: Mutex<Vec<RefCell<ChatSession>>> = {
		let s = Vec::new();
		Mutex::new(s)
	};
}

fn main() {
	let args = Args::new();
	println!("{:?}", args);
	
	if args.open {
		open::that(format!("http://{}:{}", args.host, args.port)).unwrap();
	}

	let root = args.root.clone();
	
	thread::spawn(move||{
		block_on(watcher::run(root.as_str()));
	});
	
	server::run(args.host.as_str(), args.port, args.root)
}
