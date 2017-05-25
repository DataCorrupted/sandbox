extern crate tracer;
extern crate libc;

// expected ouput

// when running the case, test shoudl print ...

fn main() {
	let mut args = Vec::new();
		args.push("ls".to_string());
		args.push("-la".to_string());
		let tracee = tracer::Tracee::new(&args).unwrap();

		// the first sys call should be execvp
		// the main program shoudl get execvp CONT first, wait first

		// TODO you should catch sys call here
		let mut status = 0;
		loop {
			unsafe { libc::wait(&mut status); }
			//let registers = tracee.take_regs().unwrap();
			//println!("{}", registers.rax);
			match tracee.do_continue() {
				Ok(_) => {;},
				Err(_) => break,
			};
		}
}