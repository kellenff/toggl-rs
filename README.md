# toggl-rs

[![Build Status](https://travis-ci.com/rakenodiax/toggl-rs.svg?branch=master)](https://travis-ci.com/rakenodiax/toggl-rs)

This is a typed interface to [version 8 of the Toggl API](https://github.com/toggl/toggl_api_docs/blob/master/toggl_api.md). This is considered very much a work in progress, PRs and feedback encouraged! The library can be found in the toggl_rs folder as we are using cargo workspaces.

## Getting Started

### CLI
There is a simple CLI included in the toggl_cli workspace. For usage of the CLI please see its help menu (cargo run -- --help).

### Prerequisites

This library targets the latest version of `rust`, though previous versions may build as well.

## Contributing

Please read [./CONTRIBUTING.md](CONTRIBUTING.md) for details on code of conduct and how to contribute

## Versioning

This project uses [Rust's semantic versioning](https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md).

## Authors

- **Kellen Frodelius-Fujimoto** - *Initial work* - [@rakenodiax](https://github.com/rakenodiax/)
- **Narfinger** - *Implement CLI and entry endpiont* - [@Narfinger](https://github.com/Narfinger)

## License

This project is licensed under either Apache 2.0 or MIT, at your option

## Acknowledgments

Thank you to [PurpleBooth](https://github.com/PurpleBooth/) for the README template.