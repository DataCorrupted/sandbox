extern crate libc;

pub enum Request{
	TraceMe = 0,
	

}

// perform the base request
pub fn base_request(option: Request, pid: i32, addr: &mut i8, data: i32) -> Result<i64, i64>{
	let res;
	unsafe{
		res = libc::ptrace(option as u32, pid, addr, data);
	}

	// error handling, TODO peek user need special care
	match res {
		-1 => Err(-1),
		_  => Ok(res),
	}
}

// indicate the current process should be traced by its parent
pub fn ptrace_traceme() -> Result<i64,i64> {
	// the pid, addr and data will be ignored
	let mut temp = 0;
	base_request(Request::PtraceTraceme, 0, &mut temp, 0)
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }

    fn call_trace_me(){

    }
}
