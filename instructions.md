# P2P node handshake

Pick a publicly available P2P node (e.g. a blockchain one) implementation - which itself doesn't need to be written in Rust - and write a [network handshake](https://en.wikipedia.org/wiki/Handshaking) for it in Rust, and instructions on how to test it.

## Requirements

- Both the target node and the handshake code should compile at least on Linux.
- The solution has to perform a full **protocol-level** (post-TCP/etc.) handshake with the target node.
- The provided **instructions** should include information on how to verify that the handshake has concluded.
- The solution can not depend on the code of the target node (but it can share some of its dependencies).
- The submitted code can not reuse entire preexisting handshake implementations like `libp2p_noise/XX`.
