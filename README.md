<!-- Using h2 instead of h1 because npm doesn't support align=center on h1 tags -->
<h1 align="center">
  <a href="#readme" title="Leology README.md"><img alt="Leology" src="README/leologo.png" alt="Leology" width="160"/></a>


Leology

</h1>

<h3 align="center">
  A tool for testing programs for fast Aleo development.
</h3>


<p align="center">
  <a href="#features">Features</a> •
  <a href="#getting-started">Getting Started</a> •
  <a href="#future">Future Work</a> •
  <a href="#related">Related</a>
</p>

---

## Features

Leology is an Aleo testing framework that makes developing Leo programs faster, easier, and safer. It includes the most commonly used RPC functions and can be run deterministically to make development a breeze.

- Easy setup of Local Testnet
- Generate Accounts for testing
- Programmatic use in Rust to run tests
- Testing of both private and public values
- Local deployment of programs
- Aleo RPC support 

## Getting Started

### Requirements

- Tmux
- Rust
- Cargo

### Command line use

Clone this Repository

```console
$ git clone git@github.com:ottodevs/leology.git
```

Navigate to the directory

```console
$ cd leology
```

To spin up the local testnet

```console
$ cargo run start
```

To stop the local testnet

```console
$ cargo run stop
```

### To test the example Leo program

Navigate to the directory

```console
$ cd examples/token
```

Run the testing command

```console
$ cargo test
```

## Future

- Run minimalistic version of snarkos for efficiency
- Full integration of RPC
- Integration with Leo

## Related

- [Aleo GitHub](https://github.com/aleoHQ/)

<br/>

---

