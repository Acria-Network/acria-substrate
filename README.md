
# Acria Node

Welcome to the Acria Substrate Node documentation. 
Please have a look at the [doc](./doc) directory for more information regarding
* [How to do the Rust setup](./doc/rust-setup.md)
* [How to create accounts](./doc/create-accounts.md)
* [How to do balance transfers](./doc/balance-transfers.md)
* [More details about running a node](./doc/run-node.md)
## Run the chain as standalone node

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp
```
### Build the chain

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

