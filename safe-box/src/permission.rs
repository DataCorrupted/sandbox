extern crate tracer;
use tracer::Tracee;
use std::env;

use file_conf::*;
use ip_conf::*;
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

pub fn open_request(tracee: &mut Tracee, allowed_file: &FileConf) {
	let registers = tracee.take_regs().unwrap();
	let mut filename = tracee.read_string(registers.rdi).unwrap();
	filename = filename.shorten();
	tracee.add_file(filename.clone());

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

pub fn connect_request(tracee: &mut Tracee, allowed_ip: &IpConf) {
	let registers = tracee.take_regs().unwrap();
	let sockaddr = registers.rsi;
	let addrlen = registers.rdx;

	if addrlen != 16 {
		tracee.do_continue();
		return;
	}

	let mut data = tracee.peek_data(sockaddr).unwrap();
	let mask = 0xff00000000;
	let mut ip_u8: Vec<u8> = Vec::new();
	for _ in 4..8 {
		ip_u8.push(((data & mask) >> 32) as u8);
		data = data >> 8;
	}
	let mut ip_str = String::new();
	for c in ip_u8{
		ip_str.push_str(c.to_string().as_str());
		ip_str.push('.');	
	}
	let _ = ip_str.pop();				// pop the last '.' out
	tracee.add_ip(ip_str.clone());
	match allowed_ip.is_ip_allowed(&ip_str){
		true => tracee.do_continue(),
		false => tracee.deny(),
	};
}
