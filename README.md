# Merkle Gift List

This is week 2's final project for [Alchemy University Ethereum Bootcamp](https://university.alchemy.com/home).

## Requirements

To run both the `server` and the `client`, [rust](https://doc.rust-lang.org/book/ch01-01-installation.html) must be installed on your machine.

## Project Overview

The project consists in simple a server, with a memory size of 32 bytes (of course this is an imaginary constraint) that needs to verify that an element belongs to a gift list using a merklee tree, and a client that supplies the needed values for the verification.

### Client

The client app is a simple cli program that requires a `.json` file storing a list of strings to be used to build the merkle tree.

```cmd
Usage: client [OPTIONS] <COMMAND>

Commands:
  get-root  Builds a merkle root from <FILE>
  validate  Validates that <NAME> belongs to the merkle tree by querying <HOST>

Options:
  -f, --file <FILE>  File that stores the list of allowed names to receive a gift [default: nice-list.json]
```

### Server

The server has a single endpoint `/gift`, which expects a `name` and a merkle `proof` to validate that the given `name` belongs to the allow list.

The server needs the `MERKLE_ROOT` env variable to start, otherwise it will panic. If a `PORT` is not specified the project will use the default value `3000`.

### Examples

#### Build a merkle root

```cmd
cargo run --bin client -- get-root
```

#### Validate that a name is in the list

```cmd
cargo run --bin client -- validate --name "Traci McDermott"
```

#### Run the server

```cmd
MERKLE_ROOT=a166212461750725be5a1baf054338a810fc9a3500f979f6da55054dd8e893fe cargo run --bin server
```
