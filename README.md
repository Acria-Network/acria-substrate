![Header](img/New%20Project.png)

# Acria Substrate

Acria Substrate Blockchain implemented in Substrate/Rust

![GitHub](https://img.shields.io/github/license/Acria-Network/acria-substrate)
![GitHub](https://img.shields.io/travis/Acria-Network/acria-substrate/master.svg)
![GitHub last commit](https://img.shields.io/github/last-commit/Acria-Network/acria-substrate)
![GitHub](https://img.shields.io/badge/OS-Linux%2FMacOS%2FWindows-brightgreen)

One of the most pressing issues when developing smart contracts is the lack of real-world data. But due to technical limitations, such as the consensus protocol, no blockchain has been able to solve this major limitation. The Acria Network solves exactly this problem with the help of so-called Oracle Nodes that don't require a middleman. In addition to this, it offers cross-chain support to supply various blockchains with real-world data.

[https://acria.network/](https://https://acria.network/)

# Acria Node

Welcome to the Acria Substrate Node documentation. 

One of the most pressing issues when developing smart contracts is the lack of real-world data. But due to technical limitations, such as the consensus protocol, no blockchain has been able to solve this major limitation. The Acria Network solves exactly this problem with the help of so-called Oracle Nodes that don't require a middleman. In addition to this, it offers cross-chain support to supply various blockchains with real-world data.  
(https://acria.network)[https://acria.network]

Please have a look at the [doc](./doc) directory for more information regarding
* [How to do the Rust setup](./doc/rust-setup.md)
* [How to create accounts](./doc/create-accounts.md)
* [How to do balance transfers](./doc/balance-transfers.md)
* [More details about running a node](./doc/run-node.md)
## Run the chain as standalone node

Use Rust's native `cargo` command to build and launch the Acria node:

```sh
cargo run --release -- --dev --tmp
```
### Build the chain

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### User Interface

For debugging and testing the node, you can use this (web user interface)[https://ipfs.io/ipns/dotapps.io/].  
You should select the connection to your node on top left, for example if your node is installed in the same machine you are connecting from,
it will be `ws://127.0.0.1`   or `ws://localhost`.


