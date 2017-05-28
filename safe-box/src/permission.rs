extern crate tracer;
use tracer::Tracee;

#[derive(Debug)]
enum PosEval {
	Out,
	Danger,
	In,
}

fn shorten(filename: String) -> String {
	filename
}

fn check(filename: &String) -> PosEval {
	PosEval::Out
}

pub fn open_request(tracee: &Tracee) {
	let registers = tracee.take_regs().unwrap();
	let mode = registers.rdx;
	let flags = registers.rsi;
	let mut filename = tracee.read_string(registers.rdi).unwrap();
	filename = shorten(filename);
	match check(&filename) {
		PosEval::Out => {},
		PosEval::Danger => {},
		PosEval::In => {},
	};
	tracee.do_continue();
}

//open("/lib/x86_64-linux-gnu/libc.so.6", O_RDONLY|O_CLOEXEC) = 3
//open("/lib/x86_64-linux-gnu/libpthread.so.0", O_RDONLY|O_CLOEXEC) = 3
//open("/etc/ld.so.cache", O_RDONLY|O_CLOEXEC) = 3
