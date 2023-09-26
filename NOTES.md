# Project Notes

Lets record here all the relevant decisions for this project. This document is intended to change over time.

### Structure

The current project structure favours the placement of reusable elements as part of the `lib.rs` crate,
while leaving the `main.rs` just for the application code.

```bash
.
├── src
│   ├── lib.rs   ## The lib crate.
│   ├── main.rs  ## The application main crate.
│   ├── p2p      ## The P2P submodules.
│   │   ├── btc.rs
│   └── p2p.rs ## The P2P module.
├── tests
│   └── real_connection_test.rs ## The test that reaches real nodes.
```

### Async Rust program with multiple node adresses

This tool is going to interact with the network. Thats an IO-bound task in which certain concurrency/parallelism levels can improve performance, so we are going to use the well known [tokio](https://tokio.rs/) async runtime.

Since we will have the logic for executing one handshake, it should not be a problem to allow passing multiple node address and process each handshake concurrently. Tokio makes this an easy task for us.

### Bitcoin handshake

The first implementation for the `p2p-node-handshake` project will be the [Bitcoin handshake](https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch08.asciidoc#network_handshake).

We are going to use the rust [bitcoin](https://github.com/rust-bitcoin/rust-bitcoin) library, as it already provides the network messages types and serialization/deserialization capabilities out of the box.

We are already seeing a growth path for the project, since we can extend the handshakes implementations with other protocols, adding them below the `p2p` folder, and accounting for them in the `p2p` module.

### Processing messages -> TO DO write

Currently we are using a mutable growing buffer [BytesMut](https://docs.rs/bytes/latest/bytes/struct.BytesMut.html) for bringing the message bytes from the network to memory, so we can parse them accordingly. The initial buffer size its currently hardcoded to 1024 bytes, being a complete handshake around 342 bytes. It should be enough for a complete handshake without the need of growing the buffer, so no more allocations than the initial one.

After a message its successfully parsed from its binary representation, its data and the buffer part it occupies are automatically discarded.

Another alternative idea (not implemented here) would be to make use of a [circular buffer](https://en.wikipedia.org/wiki/Circular_buffer) implementation. That would avoid the costs of allocating more space as we go by reusing the already allocated but discarded one. So instead of discarding old parts of the buffer with the consequent future allocation, they would just be overwritten, using cursors to control what data is still valid or not. As commented, the current implementation is considered good enough for now, as we are pre-allocating all the needed memory beforehand.

### Error handling

Errors may have different treatment depending their nature:

- Runtime errors. This errors are going to interrupt the entire program. This are very rare and unexpected.
- Handshake errors. This are expected ones.

Here the strategy for dealing with errors its very simple, creating a `P2PError` type to which all the other errors can be converted `From`. We are only interested in getting the error messages to properly show it in the console, so no further action needed for now.

### Testing

The testing could be improved a lot.
Currently is reaching real servers in order to assess the program is working correctly, which could be an impediment for local development for many reasons. A possible solution would be to build a mock server for emulating the real ones, and also be able to test other edge case scenarios like network timeouts.

### Main references

- https://en.bitcoin.it/wiki/Protocol_documentation
- https://github.com/bitcoinbook/bitcoinbook
