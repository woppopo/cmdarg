# cmdarg

```
cargo run -- val -flag --param paramval
=> Ok([Val("target/debug/cmdarg"), Val("val"), Flag("flag"), Param("param", "paramval")])
```

## Usage

```
[dependencies]
cmdarg = { git = "https://github.com/woppopo/cmdarg" }
```

```rust
extern use cmdarg;

fn main() {
	let args = cmdarg::parse().unwrap();
}
```