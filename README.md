# p2p-node-handshake 🤝

Hey 👋!

Welcome to this really simple project for making handshakes to p2p nodes. Currently, supporting the [Bitcoin P2P protocol](https://github.com/bitcoinbook/bitcoinbook/blob/develop/ch08.asciidoc#network_handshake).

If you are interested in how this project was designed and further explanations, please look at the [NOTES](NOTES.md) page. And, if you want to better understand what this intent to resolve, please see the [instructions](instructions.md) page.

Example output:

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

A [rust installation](https://rustup.rs/) is needed.

### From cargo

As an example, you can download the [Bitcoin core](https://bitcoincore.org/en/download/) node and run it on your machine. It's worth noting that you can change the running port of this particular node if that's clearer (update the `bitcoin.conf` with `port=YOUR_CUSTOM_PORT`).

```bash
cargo run
```

You can also change the adress(es) and user agent via the env vars `NODE_ADDRESSES` and `USER_AGENT`. Same as explained below.

### From tests

Other option, is to pick a node from the [list of available nodes](https://bitnodes.io/), and run:

```bash
NODE_ADDRESSES="<ip_address:port> <ip_address:port>" USER_AGENT=<user_agent> cargo test
```

## License

Unlicensed. See [LICENSE](LICENSE).
