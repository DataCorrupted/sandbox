extern crate libc;

#[derive(Debug)]
pub struct Registers {
	field: Type
}

pub enum Request{
	TraceMe = 0,


}

#[derive(Debug, Clone)]
pub struct Tracee {
	pid: libc::pid_t,
}

impl Tracee {
	pub fn new(pid: i32) -> Result<Tracee>{
		unimplemented!();
	}
	pub fn new(args: &Vec<String>) -> Result<Tracee>{
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
