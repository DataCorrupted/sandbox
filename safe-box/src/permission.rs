extern crate tracer;
use tracer::Tracee;
use std::env;

#[derive(Debug)]
enum PosEval {
	Out,
	Danger,
	In,
}

fn shorten(filename: String) -> String {
	let path_vec: Vec<String> = filename.clone().split('/').map(|x| x.to_string()).collect();
	let mut new_path: Vec<String> = Vec::new();
	for rep in path_vec {
		if rep == "..".to_string(){
			let _ = new_path.pop();
		} else if rep != ".".to_string() {
			new_path.push(rep);
		}
	}
	let mut filename = String::new();
	for rep in new_path {
		filename = filename + "/" + rep.as_str();
	}
	filename

}

fn check(filename: &String) -> PosEval {
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

pub fn open_request(tracee: &Tracee) {
	let registers = tracee.take_regs().unwrap();
	let mode = registers.rdx;
	let flags = registers.rsi;
	let mut filename = tracee.read_string(registers.rdi).unwrap();
	filename = shorten(filename);
	match check(&filename) {
		PosEval::Danger => {
			tracee.deny();
		},
		PosEval::In | PosEval::Out => {
			tracee.do_continue();	
		},
	};
	
}

#[test]
fn test_shorten() {
	let string = "/../../.././from/a/.././asdf/../a".to_string();
	assert_eq!(shorten(string), "/from/a".to_string());
}
