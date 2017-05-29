extern crate tracer;
extern crate libc;
use std::env;
use std::io::Write;
use tracer::Tracee;

mod permission;
use permission::*;


// return true if the process is still alive
fn check_process(tracee: &tracer::Tracee) -> bool{
	let mut status = 0;
	let a;

	unsafe{
		a = libc::waitpid(tracee.take_pid(), &mut status, libc::WNOHANG)
	}

	return a==0;
}

fn main() {
	// read arguments from argvs and turn it into a vec
	let mut argvs = Vec::new();
	let argvs_raw = env::args();
	if  argvs_raw.len() < 2  {
		let _ = writeln!(&mut std::io::stderr(), "[Error] safe-box: usage error");
		return ;
	}
	for x in argvs_raw.skip(1){
		argvs.push(x);
	}

	// create a new tracee
	let mut tracee = Tracee::new(&argvs).unwrap();

	// wait for execvp and start the tracee
	let _ = tracee.wait_syscall();
	tracee.do_continue();
	
	// wait for every sys call the tracee make and then determine whether the syscall is valid 
	// TODO if the child make a fork, the box also fork a process to trace the process forked by child
	loop {
		let temp = tracee.wait_syscall().unwrap();
		// println!("caller: {:?}",tracee.is_on_entry());
		if !temp { break; }
		if !check_process(&tracee) { break; }		// break the loop when the child exit, handle the sys call
		let call_num = tracee.get_syscall().unwrap();
		// println!("{:?}, {}", temp, call_num);
		// Grouping philosophy: 
		// Some "musts" with similar function are grouped together. [ by "musts" I mean must pass(like read) or must deny(like chdir) ]
		// Other undetermined with same arguments position in registers are grouped together.
		// ( So that doing checking will be easier. )
		if !tracee.is_entry() { 
			tracee.do_continue();
			continue;
		}
		let registers = tracee.take_regs().unwrap();
		if registers.orig_rax == 59 {
			println!("{:?}", tracee.read_string(registers.rdi));
		}
		println!("{}", registers.orig_rax);
		match call_num{
			// TODO, implement the map
			// IO / Memory part
			0 | 1 | 3			=> { tracee.do_continue(); },		// read | write | close	
			2					=> { open_request(&tracee); },		// open
			4 | 5 | 6			=> { tracee.do_continue(); },		// stat | fstat | lstat
//			7					=> {;},								// poll
//			8					=> {;},								// lseek
			9 | 10 | 11			=> { tracee.do_continue(); },		// mmap | mprotect | munmap
			12					=> { tracee.do_continue(); },		// brk
			13 | 14 | 15 		=> { tracee.do_continue(); },		// sigaction | sigprocmask | sigreturn
//			16					=> {;},								// ioctl
//			17 | 18 | 19 | 20	=> {;},								// pread64 | pwrite64 | readv | writev
//			21					=> {;},								// access
//			22					=> {;},								// pipe
//			23					=> {;},								// select
//			24					=> {;},								// sys_sched_yield	
			25 | 26 | 27 | 28	=> { tracee.do_continue(); }		// mremap | msync | mincore | madvise
			// I personally couldn't really understand the following 3 syscalls.
			29 | 30 | 31		=> { tracee.do_continue(); },		// shmget | shmat | shmctl
			32 | 33				=> { tracee.do_continue(); },		// dup | dup2
			34 | 35				=> { tracee.do_continue(); },		// pause
			36 | 37 | 38		=> { tracee.do_continue(); },		// getitimer | alarm | setitimer
			39 					=> { tracee.do_continue(); },		// getpid
			40					=> { tracee.do_continue(); },		// sendfile
			// Internet part
			41					=> { tracee.do_continue(); },		// socket
			42					=> { connect_request(&tracee); },	// connect
			43 | 44 | 45		=> { tracee.do_continue(); },		// accept | sendto | recvfrom
			46 | 47				=> { tracee.do_continue(); },		// sendmsg | recvmsg
			48 					=> { tracee.do_continue(); },		// shutdown
			49 | 50				=> { tracee.do_continue(); },		// bind | listen
			51 | 52				=> { tracee.do_continue(); },		// getsockname | getpeername
			53					=> { tracee.do_continue(); },		// socketpair
			54 | 55				=> { tracee.do_continue(); },		// setsockopt | getsockopt
			56 | 57 | 58		=> { tracee.deny(); },				// fork | vfork, we don't allow it for now.
			59 					=> { execve_request(&tracee); },	// execve
			60 					=> { tracee.do_continue(); }		// exit, why bother preventing someone from suicide?
			62					=> { tracee.deny(); },				// kill, we always deny it.	
			_ => {tracee.do_continue();},
		}
		// record the syscall before continue
	}

}