# Project Notes

Lets record here all the relevant decisions and information for this project.

### Structure

```bash
.
├── src
│   ├── lib.rs   ## The lib crate.
│   ├── main.rs  ## The application main crate.
│   ├── p2p      ## The P2P submodules.
│   │   ├── btc.rs ## The Bitcoin implementation.
│   └── p2p.rs ## The P2P module.
├── tests
│   └── real_connection_test.rs ## The test that reaches real nodes.
```

### Rust program with concurrent node handshakes

This tool is going to interact with the network, in which certain concurrency/parallelism levels can improve performance and stability, so we are going to use the well known [tokio](https://tokio.rs/) async runtime.

Once we have the logic for executing one handshake, Tokio makes an easy task to allow passing multiple node address and process each handshake in parallel. Currently, each node will have a hardcoded timeout to not wait forever for the node and block the program.

### The Bitcoin one

The first implementation for the project will be the [Bitcoin handshake](https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch08.asciidoc#network_handshake).

We are going to use the rust [bitcoin](https://github.com/rust-bitcoin/rust-bitcoin) library, as it already provides the network messages types and serialization/deserialization capabilities out of the box. So we don't need to put extra focus on a lower level implementation, which makes sense for the scope of the project for now.

### Processing messages

We are using a mutable growing buffer [BytesMut](https://docs.rs/bytes/latest/bytes/struct.BytesMut.html) for bringing the message bytes from network to memory and parse them accordingly.

The initial buffer size is hardcoded to 1024 bytes, which should be really enough for a complete handshake without the need of growing the buffer and allocate more memorym, as we are pre-allocating all the needed memory beforehand.

After a message its successfully parsed from its binary representation, its data and the buffer part it occupies are automatically discarded.

### Error handling

Errors may have different treatment depending their nature:

- Runtime errors. This errors are going to interrupt the entire program. This are very rare and unexpected.
- Handshake errors. This are expected ones.

Here the strategy for dealing with errors its very simple, creating a `P2PError` type to which all the other errors can be converted `From`. We are only interested in getting the error messages to properly show it in the console, so no further action needed for now.

### Testing

The testing could be improved a lot.
Currently is reaching real servers in order to assess the program is working correctly, which could be an impediment for local development for many reasons. A possible solution would be to build a mock server for emulating the real ones, and also be able to test other edge case scenarios like network timeouts.

### Formatting

`cargo fmt`

### Future work

- More than just bitcoin
  We are already seeing a growth path for the project, since we can extend the handshakes implementations with other protocols, adding them below the `p2p` folder, and accounting for them in the `p2p` module.
- Better present of the info in the console
- Testing
- Make a CLI
- Trigger processes for reading/writing from the socket, and create channels
- Timeout

### Main references

- https://en.bitcoin.it/wiki/Protocol_documentation
- https://github.com/bitcoinbook/bitcoinbook
