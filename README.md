# Rust Tic Tac Toe

## Install Rust
I used the 1.0.0-alpha version. Here's how I did it:
  1. Download the 1.0 binaries [here][rust]
  2. Extract the .tar
  3. `cd` into the new directory and run `sh install.sh`

By installing this way, you also get Cargo, the Rust build tool.

## Get the project
	git clone https://github.com/davetttt/ttt_rust.git
	cd ttt_rust

## Run the tests
	cargo test
* One of the tests plays two minimax players against one another and may take a while (30 seconds) to run.
* `cargo test` deletes the executable if one exists (so you'll have to `cargo build` again). 
	
## Compile and play
	cargo build
	./target/ttt_rust

[rust]: http://www.rust-lang.org/install.html