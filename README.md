# Validation Oracle Smart Contract
[![stability-badge][stability-alpha]][stability-info]

A smart contract which acts as a marketplace for validation to be requested from third-party reviewers.

[stability-alpha]: https://img.shields.io/badge/stability-alpha-f4d03f.svg?style=for-the-badge
[stability-info]: https://github.com/mkenney/software-guides/blob/master/STABILITY-BADGES.md#alpha
## Development
### Running a Local Demo
- Run `make` in the root of this repository to generate an optimized contract WASM file at `artifacts/validation_oracle_smart_contract.wasm`
- if you have not already done so, clone [Provenance](https://github.com/provenance-io/provenance/) and install its [prerequisites](https://github.com/provenance-io/provenance/blob/main/docs/Building.md)
- In the cloned `provenance` directory, run `make clean build localnet-start`
- Run the commands in `setup_vo_contract.sh` from within the Provenance directory
## Credits
Much of the code in this repository was copied or adapted from code by @hyperschwartz, primarily within the [asset classification smart contract](https://github.com/FigureTechnologies/asset-classification-smart-contract).
