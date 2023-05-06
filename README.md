# Validation Oracle Smart Contract

[![stability-badge][stability-alpha]][stability-info]

A smart contract which acts as a marketplace for validation to be requested from third-party reviewers.

[stability-alpha]: https://img.shields.io/badge/stability-alpha-f4d03f.svg?style=for-the-badge
[stability-info]: https://github.com/mkenney/software-guides/blob/master/STABILITY-BADGES.md#alpha

## Usage

_TODO: Add details on the data model, the assumptions it makes, and how the contract should be used (at a high level only, leaving specifics to the Rust docs)_

## Development

### Toolchain

This project has been tested and confirmed to work with

- [Rust](https://www.rust-lang.org/tools/install) 1.66.1
- [Provenance](https://github.com/provenance-io/provenance/blob/main/docs/Building.md) 1.13.1
  - [Go](https://go.dev/dl/) 1.19.3

### Running a Local Demo

1. Run `make` in the root of this repository to generate an optimized contract WASM file at `artifacts/validation_oracle_smart_contract.wasm`
2. if you have not already done so, clone [Provenance](https://github.com/provenance-io/provenance/) and install its [prerequisites](https://github.com/provenance-io/provenance/blob/main/docs/Building.md)
3. In the cloned `provenance` directory, run `make clean build localnet-start`
4. Run the commands in `scripts/setup_vo_contract.sh` from within the Provenance directory (you may need to tune some values in the script first)
5. Run the commands in any other file in `scripts` from within the Provenance directory

## Credits

Much of the code in this repository was copied or adapted from code by @hyperschwartz, primarily within the [asset classification smart contract](https://github.com/FigureTechnologies/asset-classification-smart-contract).
