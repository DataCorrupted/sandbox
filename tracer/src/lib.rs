extern crate libc;
use std::ffi::*;
use libc::*;
use std::default::Default;
use std::ptr;
	
#[derive(Debug, Default, Clone)]
pub struct Registers {
	pub r15: u64,
	pub r14: u64,
	pub r13: u64,
	pub r12: u64,
	pub rbp: u64,
	pub rbx: u64,
	pub r11: u64,
	pub r10: u64,
	pub r9: u64,
	pub r8: u64,
	pub rax: u64,
	pub rcx: u64,
	pub rdx: u64,
	pub rsi: u64,
	pub rdi: u64,
	pub orig_rax: u64,
	pub rip: u64,
	pub cs: u64,
	pub eflags: u64,
	pub rsp: u64,
	pub ss: u64,
	pub fs_base: u64,
	pub gs_base: u64,
	pub ds: u64,
	pub es: u64,
	pub fs: u64,
	pub gs: u64,
}

#[derive(Debug)]
pub enum Register{
	R15 = 0,
	R14 = 1,
	R13 = 2,
	R12 = 3,
	RBP = 4,
	RBX = 5,
	R11 = 6,
	R10 = 7,
	R9 = 8,
	R8 = 9,
	RAX = 10,
	RCX = 11,
	RDX = 12,
	RSI = 13,
	RDI = 14,
	ORIGRAX = 15,
	RIP = 16,
	CS = 17,
	EFLAGS = 18,
	RSP = 19,
	SS = 20,
	FSBASE = 21,
	GSBASE = 22,
	DS = 23,
	ES = 24,
	FS = 25,
	GS = 26,
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

	// create a tracee with a pid, and attach the caller to the tracee
	// return a result tracee on succeeding in creating a process, return Err on failure
	// Note: when the function return the caller should wait for an execvp signal first
	pub fn new(args: &Vec<String>) -> Result<Tracee,&'static str>{
		// fork a child process
		let child = unsafe{ libc::fork() };
		let tracee = Tracee{ pid: child };
		// if succes, child process run trace_me
		match tracee.pid{
			-1 => {							// failed to fork
				Err("tracer: failed to fork a child process")
			},
			0 => {							// child
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
					let _ = tracee.trace_me().unwrap();						// run trace_me
					execvp(c_prog.as_ptr(), c_args.as_ptr()) ;				// run execvp
					panic!("{:?}","tracer: child failed to run execvp" );
				};
			},
			_ =>{
				// parent return the tracee
				Ok(tracee)
			},
		}
	}

	// take process's pid.
	// we did so instead of pub pid just to prevent the user from messing pid around.
	// such deed bans user from changing pid.
	pub fn take_pid(&self) -> pid_t {
		self.pid
	}

	// yet another way to tracee a process with given pid.
	// It is not recommended.
	pub fn from(pid: i32) -> Result<Tracee, String>{
		let tracee = Tracee{ pid : pid };
		match tracee.attach(){
			Ok(_) => Ok(tracee),
			Err(_) => Err("tracer: failed to attach.".to_string()), 
		}
	}
	
	// tell the parent that the current process should be traced.
	// Only used by child.
	pub fn trace_me(&self) -> Result<i64,i64> {
		// the pid, addr and data will be ignored
		self.base_request(Request::TRACEME, 
							ptr::null_mut(), ptr::null_mut())
	}

	// the caller should use waitpid to wait for syscall from traccee
	pub fn attach(&self) -> Result<i64,String>{
		let res = self.base_request(Request::ATTACH,ptr::null_mut(), ptr::null_mut());
		match res{
			Ok(some) => return Ok(some),
			_ => return Err("tracer: failed to attach tracee".to_string()),
		}
	}

	// detach the process and send PTRACE_CONT
	pub fn detach(&self) -> Result<i64,i64>{
		self.base_request(Request::DETACH,ptr::null_mut(), ptr::null_mut())
	}

	// continue execution and stop the tracee on the entry of the next syscall
	pub fn do_continue(&self) -> Result<i64, i64>{
		let temp = 0;
		self.base_request(Request::SYSCALL, 
							ptr::null_mut(), temp as *mut libc::c_void)
	}

	// Take a certain registers from register file. 
	// The register is specified by it number, which is defined by Register.
	pub fn take_reg(&self, reg: u64) -> Result<u64, &'static str> {
		let addr = 8 * reg;
		match self.base_request(Request::PEEKUSER,
								addr as *mut libc::c_void,
								ptr::null_mut()) {
			Ok(memory) => Ok(memory as u64),
			Err(_) => Err("Failed to take one register."),
		}
	}

	// Take the whole register file and return a Registers type.
	pub fn take_regs(&self) -> Result<Registers, &'static str >{
		let mut registers: Registers = Default::default();
		let registers_ref: *mut Registers = &mut registers;
		match self.base_request(Request::GETREGS, 
								ptr::null_mut(), 
								registers_ref as *mut libc::c_void) {
			Ok(_) => Ok(registers.clone()),
			Err(_) => Err("Failed to take registers."),
		}		
	}

	// Given child's address, this will take that data out. 
	// It's not often to directly use it, 
	// since signle data are more likely to be in the register file.
	pub fn peek_data(&self, addr: u64) -> Result<u64, &'static str>{
		match self.base_request(Request::PEEKDATA, 
								addr as *mut libc::c_void, ptr::null_mut()) {
			Ok(data) => Ok(data as u64),
			Err(_) => Err("Failed to peek data."),
		}
	}

	// Given start position, read the whole string.
	// By default it will read a string as long as 256 byte.
	pub fn read_string(&self, mut addr: u64) -> Result<String, &'static str>{
		// by default 256 bytes can be read . 
		// we want to use default argument, but it's still not possible in rust 1.0.
		let mut string: String = String::with_capacity(256);
		'outter: loop {
			let data;
			// peek that data out first.
			match self.peek_data(addr) {
				Ok(d) => data = d,
				Err(e) => return Err(e),
			};
			// and do the parsing by 0xff mask (one byte).
			let mut mask = 0xff;
			for byte in 0..8 {
				let temp: u8 = ((data & mask) >> 8 * byte) as u8;
				// the mask will also shift.
				mask = mask << 8;
				if temp == 0 {
					break 'outter;
				}
				string.push(temp as char);
			}
			// next block in memory.
			addr += 8;
			if string.len() == 256 {
				return Err("Too long a string.")
			}
		}
		Ok(string)
	}
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