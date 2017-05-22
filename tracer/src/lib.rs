extern crate libc;

#[derive(Debug)]
pub struct Registers {
	a: i32,
	// unimplemented
}

pub enum Request{
	TraceMe = 0,
	PEEKTEXT= 1,
	PEEKDATA= 2,
	PEEKUSER= 3,
	POKETEXT= 4,
	POKEDATA= 5,
	POKEUSER= 6,
	CONT= 7,
	KILL= 8,
	SINGLESTEP= 9,
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
	pub fn new_with_pid(pid: i32) -> Result<Tracee,()>{
		unimplemented!();
	}
	pub fn new(args: &Vec<String>) -> Result<Tracee,()>{
		unimplemented!();	
	}
	// perform the base request
	pub fn base_request(&self, option: Request, addr: &mut i32, data: i32) -> Result<i64, i64>{
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

	// indicate the current process should be traced by its parent
	pub fn trace_me(&self) -> Result<i64,i64> {
		// the pid, addr and data will be ignored
		let mut temp = 0;
		self.base_request(Request::TraceMe, &mut temp, 0)
	}
}






#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }

    fn call_trace_me(){

    }
}
