# Single-Node Local Development

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

This command will start the single-node development chain with persistent state:

```bash
./target/release/acria-node --dev
```

Purge the development chain's state:

```bash
./target/release/acria-node purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/acria-node -lruntime=debug --dev
```

## Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to
[our Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).
This tutorial will guide you through running two nodes and constructing your own chain spec.

## Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/acria-node --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/acria-node --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/acria-node purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```