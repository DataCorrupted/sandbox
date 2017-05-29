extern crate tracer;
use tracer::Tracee;
use std::env;

use file_conf::*;
use file_name::*;

#[derive(Debug)]
enum PosEval {
	Out,
	Danger,
	In,
}

fn check_pos(filename: &String) -> PosEval {
	match filename.find("/home") {
		None => PosEval::Out,
		Some(_) => {
			match filename.find(env::current_dir().unwrap().display().to_string().as_str()) {
				None => PosEval::Danger,
				Some(_) => PosEval::In,
			}
		}
	}
}

pub fn open_request(tracee: &Tracee, allowed_file: &FileConf) {
	let temp = "/home/peter/.rustup/asldfj".to_string();
	println!("{:?}", allowed_file.is_file_allowed(&temp));
	let registers = tracee.take_regs().unwrap();
	let mut filename = tracee.read_string(registers.rdi).unwrap();
	filename = filename.shorten();
	match check_pos(&filename) {
		PosEval::Danger => {
			match allowed_file.is_file_allowed(&filename){
				true	=> tracee.do_continue(),
				false	=> tracee.deny(),
			}
			
		},
		PosEval::In | PosEval::Out => {
			tracee.do_continue();	
		},
	};
}

pub fn execve_request(tracee: &Tracee) {
	let registers = tracee.take_regs().unwrap();
	let mut filename = tracee.read_string(registers.rdi).unwrap();
	filename = filename.shorten();
	match check_pos(&filename) {
		PosEval::Danger => {
			tracee.deny();
		},
		PosEval::In | PosEval::Out => {
			tracee.do_continue();	
		},
	};
}

pub fn connect_request(tracee: &Tracee) {
	let registers = tracee.take_regs().unwrap();
	let sockaddr = registers.rsi;
	let addrlen = registers.rdx;
	for offset in 0..2 {
		let mut data = tracee.peek_data(sockaddr + offset * 8).unwrap();
		println!("{:?}", data);
		let mask = 0xff;
		for _ in 0..8 {
			print!("{} ", (data & mask));
			data = data >> 8;
		}
		println!("");
	}
	println!("{:?} {}", tracee.peek_data(sockaddr), addrlen);
	tracee.do_continue();
}
