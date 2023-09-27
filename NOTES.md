# Project Notes

Lets record here all the relevant decisions and information for this project.

## Rust program with concurrent node processing

The project is going to interact with the network, in which certain concurrency/parallelism levels can improve performance and stability, so we are going to use the well known [tokio](https://tokio.rs/) async runtime.

Once we have the logic for executing one handshake, Tokio makes an easy task to allow multiple node addresses and process each handshake in parallel. Currently, each node will have a hardcoded timeout to not wait forever for a node to respond and block the entire program.

## The Bitcoin P2P protocol

The first P2P protocol implementation for the project will be the [Bitcoin one](https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch08.asciidoc#network_handshake).

For that, is going to use the rust [bitcoin](https://github.com/rust-bitcoin/rust-bitcoin) library, as it already provides the network messages types and serialization/deserialization capabilities out of the box, so we don't need to put extra effort on a lower level implementation for now.

## Reading messages

For bringing the message bytes from the network to memory and parse them accordingly, it uses a mutable growing buffer through [BytesMut](https://docs.rs/bytes/latest/bytes/struct.BytesMut.html) for this first iteration.

The initial buffer size is hardcoded to 512 bytes, which should be really enough for a complete handshake without the need of growing the buffer and allocate more memory (as we are pre-allocating more than needed beforehand).

## Error handling

Errors should have different treatment depending of their nature:

- Runtime errors -> are going to interrupt the entire program. Unexpected.
- P2P errors -> the ones we expect to happen. Should be treated accordingly.

The strategy for dealing with errors is simple. Just create a `P2PError` type to which all the other errors can be converted from. Note that we are only interested in getting the error messages to potentially show it in the console, so no further action needed for now here.

## Testing

Currently is reaching real servers in order to assess the program is working correctly. This is not great, because for example could be an impediment for local development for so many reasons. A possible solution would be to build a mock server for emulate the nodes, and also be able to test other edge case scenarios, like network timeouts.

## Formatting

The formatting is done by [rustfmt](https://github.com/rust-lang/rustfmt), so:

```bash
cargo fmt
```

## Structure

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

## Future work

Just some ideas of what could be done as next steps for the project:

- We can easily see a growth path for the project by extending the handshake implementations with other protocols. We should add them below the `p2p` folder, and account for them in the `p2p` module.
- Testing could be improved a lot. At least, create a mock server to emulate the node responses.
- More async work. We could implement triggering processes for reading and writing the socket, and create some channels to be able to interact between them.
- A CLI tool to make the interaction with the user easier would be great
- There is definitely room for improvement on how we are presenting the results to the user
- Make connection timeout configurable as well
- Do not use the bitcoin library and implement a low level TCP implementation
- Explore more ways to read the data other than `BytesMut` (to also make it more memory efficient)

## Main references

- https://en.bitcoin.it/wiki/Protocol_documentation
- https://github.com/bitcoinbook/bitcoinbook
