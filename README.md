# example-kernel-kboot-ktest-multicrate

A very simple Rust-based x86_64 kernel that uses `kboot` for build/run automation and `ktest` for testing.

kboot: https://github.com/philo-groves/kboot

ktest: https://github.com/philo-groves/ktest

## Setup

1. Clone this repository, and navigate into its directory.
2. Install the kboot runner: `cargo install kboot`
3. Start your docker service locally (e.g. Docker Desktop)

## Running the Kernel

This kernel is already configured to run in a containerized QEMU environment with the standard `run` command:

```
cargo run
```

## Testing the Kernel

This kernel is also configured for testing in a containerized QEMU environment with the standard `test` command:

You may need to install cargo-hack: `cargo install cargo-hack`

```
cargo hack test --workspace
```

The `hack` and `--workspace` options will ensure each crate is tested individually, preventing issues with unnecessary feature unification. For example, without these options, enabling the test-only allocator in one crate would enable it for all crates in the workspace; this is not desired as the allocator should only be included if needed.

## Accessing the Kernel

There are two primary interfaces:

### Web Interface

The docker container exposes a VNC "remote" connection to VGA / framebuffer, similar to the native QEMU display window.

In a browser: http://localhost:8006

### Command Line Interface

The docker container for QEMU is executed in "interactive" mode, which makes bi-directional communication with the kernel possible through the same terminal window as the `cargo run` command.