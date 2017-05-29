pub trait SubString {
    fn is_substr_of(&self, &String) -> bool;
}

impl SubString for String {
	fn is_substr_of(&self, w: &String) -> bool {
		match w.find(self.as_str()) {
			Some(0) => true,
			_ => false,
		}
	}
}

#[test]
fn test_substr(){
	let str1 = "/home/peter/Desktop/".to_string();
	let str2 = "/home/peter/Desktop/a.out".to_string();
	assert_eq!(str1.is_substr_of(&str2), true);
}