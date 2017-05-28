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
		println!("{:?}", registers.orig_rax);
		if registers.orig_rax == 59 {
			println!("{:?}", tracee.read_string(registers.rdi));
		}
		tracee.do_continue();
	}
}