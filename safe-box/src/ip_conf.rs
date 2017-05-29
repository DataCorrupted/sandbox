use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::Ordering;

/* domain permission need support from DNS analyse, it is on the TODO list */

pub struct IpConf{
	permitted_ips: Vec<String>,
	// permitted_domains: Vec<String>,
}

impl IpConf {
	pub fn new() -> IpConf{
		let mut conf = IpConf {permitted_ips: Vec::new()};

		// read the file line by line
		
		// open the file
		let f_res = File::open("ip_permission.conf");
		match f_res {
			Ok(fd) => {
				// create reader
				let reader = BufReader::new(&fd);
				// read the file 
				for x in reader.lines(){
					let l = x.unwrap();
					// if the line is not a commment
					if (l.len() > 0) && (l.as_bytes()[0] != '#' as u8){
						conf.permitted_ips.push(String::from(l));
					}
				};
				conf
			},
			_ => conf,
		}
	}

	// if the ip is allowed, return true
	pub fn is_ip_allowed(&self, ip: &Vec<u8>) -> bool{
		let mut target_ip = String::new();
		for sub_ip in ip{
			target_ip.push_str(sub_ip.to_string().as_str());
			target_ip.push('.');	
		}
		let _ = target_ip.pop();				// pop the last '.' out

		// check whether the ip is allowed or not
		for x in self.permitted_ips.clone(){
			if target_ip.partial_cmp(&x) == Some(Ordering::Equal) {
				return true;
			}
		}
		return false;
	}
}
