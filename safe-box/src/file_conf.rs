use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use file_name::FileName;

pub struct FileConf{
	premitted_files: Vec<String>
}

impl FileConf {
	pub fn new() -> FileConf{
		let mut conf = FileConf {premitted_files: Vec::new(),};

		// read the file line by line
		// open the file
		let f_res = File::open("file_permission.conf");
		match f_res {
			Ok(fd) => {
				// create reader
				let reader = BufReader::new(&fd);
				// read the file 
				for x in reader.lines(){
					let l = x.unwrap();
					// if the line is not a commment
					if (l.len() > 0) && (l.as_bytes()[0] != '#' as u8){
						conf.premitted_files.push(String::from(l));
					}
				};
				conf
			},
			_ => conf,
		}

	}

	pub fn is_file_allowed(&self, que: &String) -> bool{
		for x in self.get_files(){
			if que.is_inside(&x) {
				return true;
			}
		}
		return false;
	}

	fn get_files(&self) -> Vec<String>{
		self.premitted_files.clone()
	}
}
