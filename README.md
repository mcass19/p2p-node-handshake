# p2p-handshake 🤝

Hello 👋!

Welcome to this really simple tool for making handshakes to p2p nodes. Currently, supporting the [Bitcoin handshake](https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch08.asciidoc#network_handshake).

If you are interested in how this project was designed and further explanation, please look at the [NOTES](NOTES.md) page. Also, please see [this](node-handshake.md) page for better understanding of what this intent to resolve.

Full example usage and output:

```bash
cargo run

Connected to Bitcoin node at <ip_address:port>
Received Version message: VersionMessage { version: 70016, services: ServiceFlags(1101), timestamp: 1695733006, receiver: Address {services: ServiceFlags(NONE), address: 64.246.65.129, port: 53729}, sender: Address {services: ServiceFlags(NETWORK|BLOOM|WITNESS|COMPACT_FILTERS|NETWORK_LIMITED), address: 0.0.0.0, port: 0}, nonce: 17875138075366979079, user_agent: "/Satoshi:25.0.0/", start_height: 809430, relay: true }
Received Verack message
Bitcoin handshake successful!
```

## How to run

Currently, a [rust installation](https://rustup.rs/) is needed.

### From cargo

For example, download the [Bitcoin core](https://bitcoincore.org/en/download/) node and run it on your machine. You can change the running port of this node if you want (updating the `bitcoin.conf` with `port=8332`, for example).

```bash
cargo run
```

You can also change the adressess and user agent via environment variables, equal as below.

### From tests

Pick a node from the [list of nodes](https://bitnodes.io/). After that, just run:

```bash
NODE_ADDRESSES="<ip_address:port> <ip_address:port>" USER_AGENT=<user_agent> cargo test
```
