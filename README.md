Install and update Rust

https://www.rust-lang.org/learn/get-started

Rocket needs the nightly Rust compiler for now, follow these instructions to enable it

https://rocket.rs/v0.4/guide/getting-started/

Mostly it is just

    rustup default nightly
   
After that it should just require you to run this from the command line.

The first build may take a minute.

    cargo update
        Updating crates.io index
        Updating libc v0.2.71 -> v0.2.72
    cargo run
       Compiling libc v0.2.72
       Compiling getrandom v0.1.14
       Compiling time v0.1.43
       Compiling num_cpus v1.13.0
       ...
 