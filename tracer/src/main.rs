extern crate tracer;

// expected ouput

// when running the case, test shoudl print ...

fn main() {
	let mut args = Vec::new();
    	args.push("ls".to_string());
    	args.push("-la".to_string());
    	let tracee = tracer::Tracee::new(&args).unwrap();

    	// TODO you should catch sys call here

    	// 
    	match tracee.do_continue() {
    		Ok(_) => println!("Ok"),
    		Err(_) => println!("Err"),
    	};
}