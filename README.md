# crap

```
cargo run -- val -flag --param paramval
=> Ok([Val("target/debug/crap"), Val("val"), Flag("flag"), Param("param", "paramval")])
```

## Usage

```
[dependencies]
crap = { git = "https://github.com/woppopo/crap" }
```

```rust
extern use crap;

fn main() {
	let args = crap::parse().unwrap();
}
```