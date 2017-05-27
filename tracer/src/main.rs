extern crate tracer;
extern crate libc;

use std::env;
//use tracer::Register;
// expected ouput

// when running the case, test shoudl print ...

fn main() {
	let mut args = Vec::new();
	for x in env::args().skip(1){
		args.push(x);
	}
	let tracee = tracer::Tracee::new(&args).unwrap();
		// the first sys call should be execvp
	// the main program shoudl get execvp CONT first, wait first
		// TODO you should catch sys call here
	let mut status = 0;
	loop {
		unsafe { libc::wait(&mut status); }
		let registers = tracee.take_regs().unwrap();
		//println!("{:?}", registers.orig_rax);
		if registers.orig_rax == 1 {
			let addr = tracee.take_regs().unwrap().rsi;
			let arg = tracee.read_string(addr);
			for c in arg{
				if c == "2^2 5^1\n" {
					tracee.reject();		
				}
			}
			
		}
		match tracee.do_continue() {
			Ok(_) => {;},
			Err(_) => break,
		};
	}
}