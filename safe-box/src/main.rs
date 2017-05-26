use std::env;

fn main() {
	// read arguments from argvs and turn it into a vec
	let mut argvs = Vec::new();
	let mut argvs_raw = env::args();
	for x in argvs_raw{
		argvs.push(x);
	}

	// create a new tracee

	// wait for execvp and start the tracee

	// the following things is in a while loop
	// wait for every sys call the tracee make and then determine whether the syscall is valid 
	// if the child make a fork, the box also fork a process to trace the process forked by child

	// break the loop when the child exit
}