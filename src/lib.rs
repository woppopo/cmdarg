#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
	Val(String),
	Flag(String),
	Param(String, String),
}

pub fn parse() -> Result<Vec<Arg>, String> {
	parse_iter(std::env::args())
}

pub fn parse_iter(iter: impl DoubleEndedIterator<Item = String>) -> Result<Vec<Arg>, String> {
	let mut parsed = Vec::new(); // Uses this as a stack

	for v in iter.rev() {
		let arg = if v.starts_with("--") {
			if let Some(Arg::Val(val)) = parsed.pop() {
				Arg::Param(v, val)
			} else {
				return Err(format!("no value passed for {}", v))
			}
		} else if v.starts_with("-") {
			Arg::Flag(v)
		} else {
			Arg::Val(v)
		};

		parsed.push(arg);
	}

	parsed.reverse(); // This is no longer a stack

	/*
	Remove hyphens:
		Val(val) => Val(val)
		Flag("-flag") => Flag("flag")
		Param("--param", val) => Param("param", val)
	*/
	for v in parsed.iter_mut() {
			match v {
					Arg::Val(_) => {}
					Arg::Flag(flag) => { flag.drain(0..1); }
					Arg::Param(param, _) => { param.drain(0..2); }
			}
	}

	Ok(parsed)
}

#[test]
fn test() {
	let test = vec!["test", "-test", "--test", "test", "test", "--test"];
	let test: Vec<String> = test.into_iter().map(ToString::to_string).collect();
	assert!(parse_iter(test.into_iter()).is_err());

	let test = vec!["test", "-test", "--test", "test", "test"];
	let test: Vec<String> = test.into_iter().map(ToString::to_string).collect();
		let test = parse_iter(test.into_iter()).unwrap();
		let ans = vec![Arg::Val("test".to_string()), Arg::Flag("test".to_string()), Arg::Param("test".to_string(), "test".to_string())];
	assert!(test.into_iter().zip(ans.into_iter()).all(|(a, b)| a == b));
}