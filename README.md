# README

I tried simplifying the guide by just using the rust compiler `rustc`, but I was having trouble linking an external crate for random number generation. I ended up just using `cargo` like I did while developing, since I wanted to at least get it working without submitting it in late because I couldn't get the linking right.

1. Make sure the rust compiler and package manager is installed by running `cargo -V`; you should see a version number.
2. If it isn't installed, install it. On Linux, run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
3. Either extract the folder I uploaded into the assignment page or use `git clone git@github.com:internetwiz4rd/3530-networking-assignment.git`, then `cd` into the folder
4. To build the binaries, run `cargo build -r --bins`
5. Open another terminal tab
6. On one terminal, run the server using `./target/release/server <hostname> <port number>`; try using "localhost" and "12345" for the args.
7. On another, run the client using `./target/release/client <hostname> <port number>`; make sure the arguments match between calls.
8. Both programs should now be communicating with each other.
