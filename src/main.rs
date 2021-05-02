#![warn(dead_code)]

mod lib;

fn main() {
	println!("=> {:?}", lib::parse());
}