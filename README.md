# Archway Templates

Archway Smart Contract template projects used by the [Archway Developer CLI](https://github.com/archway-network/archway-cli).

## List of templates

- [Default](./default): a blank slate project
- [Increment](./increment): a simple smart contract that increments and resets a counter

## Creating a new project from a template

### Recommended way

Follow the instructions in the [Archway Setup tutorial](https://docs.archway.io/docs/create/getting-started/setup).

### Using cargo-generate

Assuming you have a recent version of rust and cargo (v1.51.0+) installed (via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

Install [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) and cargo-run-script.
If you didn't install them already, run the following commands:

```sh
cargo install cargo-generate --features vendored-openssl
cargo install cargo-run-script
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:

```sh
cargo generate --git archway-network/archway-templates.git --name PROJECT_NAME default
```

You will now have a new folder called `PROJECT_NAME` (I hope you changed that to something else)
containing a simple working contract and build system that you can customize.
