extern crate libc;
use std::ffi::*;
use libc::*;
use std::default::Default;
use std::ptr;

#[derive(Debug, Default)]
pub struct Registers {
	rax: i64,
	rbx: i64,
	rcx: i64,
	rdx: i64,
	rsi: i64,
	rdi: i64,
	rbp: i64,
	rsp: i64,
	r8: i64,
	r9: i64,
	r10: i64,
	r11: i64,
	r12: i64,
	r13: i64,
	r14: i64,
	r15: i64,
}

#[derive(Debug)]
pub enum Request{
	TRACEME = 0,
	PEEKTEXT= 1,
	PEEKDATA= 2,
	PEEKUSER= 3,
	POKETEXT= 4,
	POKEDATA= 5,
	POKEUSER= 6,
	CONT= 7,
	KILL= 8,
	SINGLESTEP= 9,
	GETREGS= 12,
	SETREGS= 13,
	ATTACH= 16,
	DETACH= 17,
	SYSCALL= 24,
	SETOPTIONS= 0x4200,
	GETEVENTMSG= 0x4201,
	GETSIGINFO= 0x4202,
	SETSIGINFO= 0x4203,
	GETREGSET= 0x4204,
	SETREGSET= 0x4205,
	SEIZE= 0x4206,
	INTERRUPT= 0x4207,
	LISTEN= 0x4208,
	PEEKSIGINFO= 0x4209,

}

#[derive(Debug, Clone)]
pub struct Tracee {
	pid: libc::pid_t,
}


impl Tracee {
	// new a tracee with a pid, and attach the caller to the tracee\
	// return a result tracee(
	pub fn new(args: &Vec<String>) -> Result<Tracee,&'static str>{
		// new a child process
		let child = unsafe{ libc::fork() };
		let tracee = Tracee{ pid: child };
		println!("Parent check");
		if tracee.pid == 0 {				// child
			println!("check point_1");
			let cmd = args[0].clone();
			// Convert cmd(args[0]) and args into C style,
			let c_prog = CString::new(cmd.as_bytes()).unwrap();
			let c_args_temp: Vec<_> = args.iter()
						.map(|x| CString::new(x.as_bytes())
							.unwrap()).collect();
			let mut c_args: Vec<_> = c_args_temp.iter()
						.map(|x| x.as_ptr()).collect();
			c_args.push(std::ptr::null());
			unsafe{ 
				println!("Are you here");
				let _ = tracee.trace_me().unwrap();
				execvp(c_prog.as_ptr(), c_args.as_ptr()) ;
				panic!("{:?}","tracer: child failed to run execvp" );
			};
		} else {
			// wait for execvp and do_continue
			let mut status = 0;
			unsafe{ wait(&mut status); }
			match tracee.do_continue(){
				Ok(_) => {
					return Ok(tracee);
				}
				_ => {
					// if the base request failed to send cont, output failed
					return Err("tracer: failed to run child process");
				}
			}
					
		}
	}
	pub fn from(pid: i32) -> Result<Tracee, String>{
		let tracee = Tracee{ pid : pid };
		match tracee.attach(){
			Ok(_) => Ok(tracee),
			Err(_) => Err("Failed to attach.".to_string()), 
		}
	}
			// indicate the current process should be traced by its parent
	pub fn trace_me(&self) -> Result<i64,i64> {
		// the pid, addr and data will be ignored
		self.base_request(Request::TRACEME, ptr::null_mut(), ptr::null_mut())
	}
	pub fn attach(&self) -> Result<(),()>{
		unimplemented!();
	}
	pub fn do_continue(&self) -> Result<i64, i64>{
		self.base_request(Request::CONT, ptr::null_mut(), ptr::null_mut())
	}

	pub fn take_regs(&self) -> Result<Registers, &'static str >{
		let mut buf: Registers = Default::default();
		let buf_ref: *mut Registers = &mut buf;
		match self.base_request(Request::GETREGSET, ptr::null_mut(), buf_ref as *mut libc::c_void) {
			Ok(_) => Ok(buf),
			Err(_) => Err("Error"),
		}
	}
	pub fn read_string(&self, addr: u64) -> Result<>
	// perform the base request
	pub fn base_request(&self, 
						option: Request, 
						addr: *mut libc::c_void, 
						data: *mut libc::c_void) 
		-> Result<i64, i64>{
		let res;
		unsafe{
			res = libc::ptrace(option as u32, self.pid, addr, data);
		}
				// error handling, TODO peek user need special care
		match res {
			-1 => Err(-1),
			_  => Ok(res),
		}
	}

}




#[cfg(test)]
mod tests {
	use Tracee;
    #[test]
    fn it_works() {
    }

    #[test]
    fn call_trace_me(){
    	let mut args = Vec::new();
    	args.push("ls".to_string());
    	args.push("-la".to_string());
    	let tracee = Tracee::new(&args).unwrap();
    	match tracee.do_continue() {
    		Ok(_) => println!("Ok"),
    		Err(_) => println!("Err"),
    	};
    } 
}