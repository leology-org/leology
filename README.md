<h1 align="center">
  <img src="img/leologo.png" alt="Leology Logo" width="200"/>
  <br/>
  Leology
</h1>

<h4 align="center">Accelerating Aleo Development with Next-Gen Testing Tools</h4>

<p align="center">
  <a href="#about">About</a> •
  <a href="#features">Features</a> •
  <a href="#getting-started">Getting Started</a> •
  <a href="#contributing">Contributing</a> •
  <a href="#future-directions">Future Directions</a> •
  <a href="#contact">Contact</a>
</p>

---

## About

Leology is the pioneering testing framework designed exclusively for the Aleo blockchain, aimed at enhancing the speed, ease, and security of Leo program development. Our mission is to streamline the Aleo development process, making it more accessible to both seasoned developers and newcomers alike.

## Features

Leology brings to the table an array of powerful features tailored for Aleo, including:

- **Local Testnet Setup**: Simulate Aleo network environments with ease.
- **Account Generation**: Create test accounts for comprehensive testing scenarios.
- **Rust Integration**: Utilize our Rust-based tools for seamless test execution.
- **Value Testing**: Rigorously test both private and public values.
- **Local Program Deployment**: Deploy and test your programs in a local environment.
- **Aleo RPC Support**: Leverage Aleo's Remote Procedure Call capabilities for enhanced testing.

## Getting Started

Jump into Leology with these simple steps:

### Prerequisites

Ensure you have the following installed:

- Rust and Cargo
- [snarkOS](https://github.com/AleoHQ/snarkOS)
- [snarkVM](https://github.com/AleoHQ/snarkVM)

### Installation

Clone the repository and navigate to the directory:

```bash
git clone https://github.com/leology-org/leology.git
cd leology
```

````

### Usage

Start the local testnet:

```bash
cargo run start
```

To stop the local testnet press keys `control+c`

### Running Examples

While running the local testnet, open a new terminal.

Test the example Leo program:

```bash
cd examples/token
cargo test
```

## Limitations

The testing for private to private txs is not yet fully implemented.

## Contributing

We welcome contributions of all forms, from code improvements to documentation enhancements. If you're interested in contributing, please see our [Contributing Guidelines](CONTRIBUTING.md) for more information on how to get started. Your insights and expertise can help shape the future of Leology and Aleo development!

## Future Directions

- **Efficiency Improvements**: Implement a minimalistic version of snarkOS for faster testing.
- **Full RPC Integration**: Enhance our framework with comprehensive RPC capabilities.
- **Leo Integration**: Strengthen the synergy between Leology and the Leo programming language.

## Contact

Join our vibrant community, share your feedback, and discuss new ideas:

- [Discord](#) - For real-time discussions.
- [GitHub Issues](https://github.com/leology-org/leology/issues) - For feature requests and bug reports.
- [Email Us](mailto:contact@leology.org) - For direct inquiries.

Let's build the future of Aleo development together!
````
