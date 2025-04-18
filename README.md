# README

1. Make sure the rust compiler and package manager is installed by running `cargo -V`; you should see a version number.
2. If it isn't installed, install it. On Linux, run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.

3. On one terminal, run the server using `./server <hostname> <port number>`; try using "localhost" and "12345" for the args.
4. On another, run the client using `./server <hostname> <port number>`; make sure the arguments match between calls.
5. Both programs should now be communicating with each other.
