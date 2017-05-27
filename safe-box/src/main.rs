extern crate tracer;
extern crate libc;
use std::env;

// retrun when one of the child called syscall
fn safe_wait(){
	let mut status = 0;
	unsafe {
		libc::wait(&mut status);
	}
}

fn main() {
	// read arguments from argvs and turn it into a vec
	let mut argvs = Vec::new();
	let argvs_raw = env::args();
	for x in argvs_raw{
		argvs.push(x);
	}

	// create a new tracee
	let tracee = tracer::Tracee::new(&argvs).unwrap();

	// wait for execvp and start the tracee
	
	// test only
	loop {
		safe_wait();
		tracee.do_continue();
	}


	// the following things is in a while loop
	// wait for every sys call the tracee make and then determine whether the syscall is valid 
	// if the child make a fork, the box also fork a process to trace the process forked by child

	// break the loop when the child exit

}