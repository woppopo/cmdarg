#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Val(String),
    Flag(String),
    Param(String, String),
}

pub fn parse() -> Result<Vec<Arg>, String> {
    parse_iter(std::env::args())
}

pub fn parse_iter(
    iter: impl DoubleEndedIterator<Item = impl AsRef<str>>,
) -> Result<Vec<Arg>, String> {
    let mut parsed = Vec::new();

    for ref v in iter.rev() {
        let v = v.as_ref();
        let arg = if v.starts_with("--") {
            if let Some(Arg::Val(val)) = parsed.pop() {
                Arg::Param(v[2..].to_string(), val)
            } else {
                return Err(format!("no value passed for {}", v));
            }
        } else if v.starts_with("-") {
            Arg::Flag(v[1..].to_string())
        } else {
            Arg::Val(v.to_string())
        };

        parsed.push(arg);
    }

    parsed.reverse();
    Ok(parsed)
}

#[test]
fn test() {
    let test = vec!["test", "-test", "--test", "test", "test", "--test"];
    assert!(parse_iter(test.into_iter()).is_err());

    let test = vec!["test", "-test", "--test", "test", "test"];
    let test = parse_iter(test.into_iter()).unwrap();
    let ans = vec![
        Arg::Val("test".to_string()),
        Arg::Flag("test".to_string()),
        Arg::Param("test".to_string(), "test".to_string()),
    ];
    assert!(test.into_iter().zip(ans.into_iter()).all(|(a, b)| a == b));
}
