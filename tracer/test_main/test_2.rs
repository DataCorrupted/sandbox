extern crate tracer;
extern crate libc;

use std::env;
use std::{thread, time};
/* 	input: a pid as argv
 	expected ouput:
 		the process stop for 2s and continue after detach */

// when running the case, test shoudl print ...

// test attach and detach
fn main() {
	let mut argvs = env::args();
	let _ = argvs.next();
	let pid_str = argvs.next().unwrap();
	let pid:i32 = pid_str.parse().unwrap();

	let tracee = tracer::Tracee::from(pid).unwrap();

	// the first sys call should be execvp
	// the main program shoudl get execvp CONT first, wait first

	// TODO you should catch sys call here
	let time = time::Duration::from_millis(2000);
	thread::sleep(time);

	tracee.detach();
}
