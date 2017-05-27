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

// return true if the process is still alive
fn check_process(tracee: &tracer::Tracee) -> bool{
	let mut status = 0;
	let a;

	unsafe{
		a = libc::waitpid(tracee.take_pid(), &mut status, libc::WNOHANG);
	}

	if a==0{return true;}
	else{return false;}
}

fn main() {
	// read arguments from argvs and turn it into a vec
	let mut argvs = Vec::new();
	let mut argvs_raw = env::args();
	if  argvs_raw.len() < 2  {
		println!("{:}", "[Error] safe-box: usage error");
		return ;
	}
	for x in argvs_raw.skip(1){
		argvs.push(x);
	}

	// create a new tracee
	let tracee = tracer::Tracee::new(&argvs).unwrap();

	// wait for execvp and start the tracee
	// safe_wait();
	// tracee.do_continue();
	
	// test only
	let mut i = 0;
	loop {
		safe_wait();
		if !check_process(&tracee) { break; }
		tracee.do_continue();
	}


	// the following things is in a while loop
	// wait for every sys call the tracee make and then determine whether the syscall is valid 
	// if the child make a fork, the box also fork a process to trace the process forked by child

	// break the loop when the child exit

}