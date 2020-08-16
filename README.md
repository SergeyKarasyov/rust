# rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
# github for the code 
https://github.com/PacktPublishing/Rust-Quick-Start-Guide

# main rust tools
* rustup - updater for toolchain
```
rustup update
rustup toolchain install beta
```
* cargo - backend for rust compiler, can create a boilerplate project
```
cargo new foo
cargo new --lib bar
```


# Rust project metadata
## cargo.toml 
* includes  program's name, version, authors, dependencies. 
* Written in Tom's Obvious, Minimal Language (TOML). 
* If dependency is available on https://crates.io you can link it, how to search on crates?
```
cargo search serde
...
[dependencies]
serde = "1.0.70"
```
* If dependency is somewhere in git repo
```
[dependencies]
thing = { git = "https://github.com/example/thing" }
```
* If dependency is a local lib
```
[dependencies]
example = { path = "/path/to/example" }
```
# compilation
```
cd foo # where is Cargo.toml
...
cargo build
//or
cargo build --release
```
* recompilation
```
cargo run
```



# Modules
## modules as a separate file
```
#filename is module_b.rs or module_b/mod.rs
pub mod module_b;
```

## modules as a part of file
```
pub mod module_a {
    pub fn a_thing() {
         println!("This is a thing");
    }

    pub fn a_second_thing() {
         a_thing();
         println!("This is another thing");
    }
}
```