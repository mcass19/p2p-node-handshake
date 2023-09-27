# p2p-handshake 🤝

Hello 👋!

Welcome to this really simple tool for making handshakes to p2p nodes. Currently, supporting the [Bitcoin handshake](https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch08.asciidoc#network_handshake).

If you are interested in how this project was designed and further explanation, please look at the [NOTES](NOTES.md) page. Also, please see [this](node-handshake.md) page for better understanding of what this intent to resolve.

Full example usage and output:

```bash
› NODE_ADDRESSES="195.123.221.104:8333 89.177.13.137:8333 54.144.114.87:8333" USER_AGENT="/Satoshi:25.0.0/" cargo run

Connected to Bitcoin node at "54.144.114.87:8333"
Connected to Bitcoin node at "195.123.221.104:8333"
Connected to Bitcoin node at "89.177.13.137:8333"
✅ 195.123.221.104:8333: Bitcoin handshake successful!
❌ 89.177.13.137:8333: P2P Error: deadline has elapsed
✅ 54.144.114.87:8333: Bitcoin handshake successful!
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
