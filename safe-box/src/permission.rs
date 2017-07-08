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

pub fn unlink_request(tracee: &mut Tracee, allowed_file: &FileConf) {
	let filename = tracee.take_filename().unwrap().shorten();
	tracee.add_file(filename.clone());
	tracee.do_continue(); return;
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
	let ip_str = tracee.take_ip().unwrap();
	tracee.add_ip(ip_str.clone());
	match allowed_ip.is_ip_allowed(&ip_str){
		true => tracee.do_continue(),
		false => tracee.deny(),
	};
}
