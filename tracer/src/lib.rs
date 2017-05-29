extern crate libc;
use libc::*;

use std::ffi::*;
use std::default::Default;
use std::ptr;
use std::process::exit;

mod consts;
use consts::*;

mod registers;
use registers::Registers;

#[derive(Debug, Clone)]
pub struct Tracee {
	pid: libc::pid_t,
	allow_all: bool,
	entry_flag : bool,				// indicate whether it is on entry or exit from a syscall
	last_syscall: u64,
	ip_connected: Vec<String>,
	file_opened: Vec<String>,
}

impl Tracee {

	// create a tracee with a pid, and attach the caller to the tracee
	// return a result tracee on succeeding in creating a process, return Err on failure
	// Note: when the function return the caller should wait for an execvp signal first
	pub fn new(args: &Vec<String>, all: bool) -> Result<Tracee,&'static str>{
		// fork a child process
		let child = unsafe{ libc::fork() };
		let tracee = Tracee{ pid: child, allow_all: all,
							entry_flag: false, last_syscall: 1023,
							ip_connected: Vec::new(), file_opened: Vec::new() };
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
	pub fn from(pid: i32, all: bool) -> Result<Tracee, String>{
		let tracee = Tracee{ pid : pid, allow_all: all,
							entry_flag: false, last_syscall: 1023,
							ip_connected: Vec::new(), file_opened: Vec::new() };
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
	pub fn do_continue(&self){
		let temp = 0;
		self.base_request(Request::SYSCALL, 
							ptr::null_mut(), temp as *mut libc::c_void) 
		.unwrap();
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

	pub fn set_reg(&self, reg: u64, val: u64) {
		let addr = 8 * reg;
		let _ = self.base_request(Request::POKEUSER,
								addr as *mut libc::c_void,
								val as *mut libc::c_void);
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

	// If we find out that things are going south,
	// we deny that syscall.
	// one proposal is to kill it immediately, now we use SIGTERM,
	// another is to deny that syscall, but we had a hard time doing that.
	pub fn deny(&self) {
/*		let addr = 0;
		let data = 0;
		self.base_request(Request::CONT, 
							addr as *mut libc::c_void,
							data as *mut libc::c_void);*/
		let registers = self.take_regs().unwrap();
		unsafe{ kill(self.pid, libc::SIGKILL); }
		println!("your programme just get killed because we \
			denied your syscall. (syscall number: {}) \n\
			We denied it because we think this syscall to be dangerous, \n\
			if you think otherwise, consider add something to your config file.", registers.orig_rax);
		
		match registers.orig_rax{
			42 => {
				let ip = &self.ip_connected;
				println!("hint: you were trying to connect to disallowed ip: {}", ip[ip.len()-1]);
			},
			2  => {
				let file = &self.file_opened;
				println!("hint: you were trying to open file: {}", file[file.len()-1]);
			},
			_  => println!("hint: Did you use fork() or kill()? We don't allow it for now."),
		}
		exit(0);
	}

	// or we can manually kill it anytime we want.
	pub fn kill(&self) {
		unsafe{ kill(self.pid, libc::SIGKILL); }
		exit(0);
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

	// get the syscall number of tracee when it stop
	pub fn get_syscall(&self) -> Result<u64,String>{
		let regs = self.take_regs();			// take the registers
		match regs {
			// orig_rax store the syscall
			Ok(temp) => Ok(temp.orig_rax),
			_ => Err("Failed to get syscall".to_string()),
		}
	}

	// return true if the tracee is on the entry of a syscall
	pub fn is_entry(&self) -> bool{
		self.entry_flag
	}

	// return true if catch a signal or false if the tracee exit
	pub fn wait_syscall(&mut self) -> Result<bool,String>{
		let mut status = 0;
		// wait for the child syscall
		unsafe{
			libc::waitpid(self.take_pid(),&mut status,0);
		}
		// check whether the waitpid return because of exit
		if unsafe{ libc::WIFEXITED(status)} {
			return Ok(false);
		}
		// if the tracee made a syscall
		else{
			// Update the syscall exit/entry flag 
			match self.update_entry_flag(){
				false => return Err("Failed to update the entry flag".to_string()),
				true => {},
			}
			return Ok(true);
		}
	}

	// update the entry flag and last syscall
	// return true on success or false on failure
	// This function should be called only 
	fn update_entry_flag(&mut self) -> bool{
		let call_num;
		match self.get_syscall(){
			Ok(temp) => call_num = temp,
			Err(_) => return false,
		}

		let last_call = self.last_syscall;
		// println!("call_num {:?}", call_num);
		// println!("last_call {:?}", last_call);

		// update the entry flag
		if call_num == 59{			//execvp
			if self.last_syscall != 59{self.entry_flag = true;}		// if it is on entry
			else{self.entry_flag = false;}
		}
		else if last_call == call_num {
			self.entry_flag = !self.entry_flag;
		}
		else {
			// println!("{:?}", "I'm here_2" );
			self.entry_flag = true;
		}

		// update the last_syscall
		// println!("entry: {:?}", self.entry_flag);
		// println!("" );
		self.last_syscall = call_num;
		return true;
	}
	pub fn print_ip_connected(&self) {
		for ip in &self.ip_connected {
			println!("{}", ip);
		}
	}
	pub fn print_file_opened(&self) {
		for file in &self.file_opened {
			println!("{}", file);
		}
	}

	pub fn add_ip(&mut self, ip: String) {
		self.ip_connected.push(ip);
	}
	pub fn add_file(&mut self, file: String) {
		self.file_opened.push(file);
	}
}