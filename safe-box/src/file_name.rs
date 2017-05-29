pub trait FileName {
    fn is_inside(&self, &String) -> bool;
    fn shorten(&self) -> String;
}

impl FileName for String {
	// This checks if self is a preceeding substring of w.
	fn is_inside(&self, w: &String) -> bool {
		match self.find(w.as_str()) {
			Some(0) => true,
			_ => false,
		}
	}
	// shorten() will shorten a filename, leave ./ and ../ out.
	fn shorten(&self) -> String{
		let path_vec: Vec<String> = self.clone().split('/').map(|x| x.to_string()).collect();
		let mut new_path: Vec<String> = Vec::new();
		// We need to skip the first one rep since is "" (because of split)
		for rep in path_vec.into_iter().skip(1) {
			if rep == "..".to_string(){
				let _ = new_path.pop();
			} else if rep != ".".to_string() {
				new_path.push(rep);
			}
		}
		let mut filename = String::new();
		for rep in new_path {
			filename = filename + "/" + rep.as_str();
		}
		filename
	}
}

#[test]
fn test_substr(){
	let str1 = "/home/peter/Desktop/OS/sandbox/safe-box/target/debug/tls/x86_64/libpthread.so.0".to_string();
	let str2 = "/home/peter/Desktop/OS/sandbox/".to_string();
	assert_eq!(str1.is_inside(&str2), true);
}

#[test]
fn test_shorten() {
	let string = "/../../.././from/a/.././asdf/../a".to_string().shorten();
	assert_eq!(string, "/from/a".to_string());
}
