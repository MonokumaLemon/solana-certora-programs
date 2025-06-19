# SolanaExamples

This repository contains Solana smart contract programs that can be formally verified using the Certora Prover. It is an extended version of the official [certora/solanaExamples](https://github.com/Certora/solanaExamples), with additional programs, simplified inline comments, and the removal of `.just` files for personal preference. Special thanks to the Certora team for providing the tools, examples, and inspiration.

The motivation behind this project is to explore the application of [SCAR (Smart Contract Abstract Representation framework)](https://gitlab.kit.edu/jonas.schiffl/Scar) to Solana programs.

<<<<<<< HEAD
---Add commentMore actions

=======
>>>>>>> 26faf08 (Correct wording and formatting in README)
**Below is the introduction from the official Certora Solana Examples repository:**

## Prerequisites

See the [Certora Solana Prover documentation](https://docs.certora.com/en/latest/docs/solana/index.html) 
for instruction about how to install the prerequisites.

Furthermore, you will need to install [just](https://just.systems/man/en/).

## Structure and Usage

Each example has a `certora` subdirectory which contains files to perform the formal verification.
Inside of `certora/conf` there are several `.conf` files which are the config files for `certoraSolanaProver` and 
run the rules in the example.

For instance, to run the verification on the `first_example` run the following:
```bash
cd cvlr_by_example/first_example/certora/conf
certoraSolanaProver Default.conf
```
This will build the code and run the verification.