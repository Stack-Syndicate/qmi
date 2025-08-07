# QMI - Query, Mutation, Interaction

[![Crates.io](https://img.shields.io/crates/v/qmi.svg)](https://crates.io/crates/qmi)
[![Docs.rs](https://docs.rs/qmi/badge.svg)](https://docs.rs/qmi)
[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=red)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/license/mit/)

This project is in the very early stages of development and is nowhere near ready for use.

## Description (Plannned)

QMI is a Rust ECS framework designed around *multiple isolated worlds* running in *parallel threads*, coordinated through an event-driven architecture and an efficient async I/O runtime.

### What QMI Does Differently

- **Event-Driven Worlds**: Instead of polling, systems react to events making execution efficient and naturally concurrent. This allows for optimization to be performed on how events are batched and sorted.
- **Parallelism by Design**: Each world runs on its own OS thread, maximizing CPU usage without locking or contention.
- **Integrated Async I/O**: A dedicated Tokio runtime handles async tasks like networking or file I/O, enabling worlds to offload and react to async operations cleanly.
- **Clear Interaction Model**: Worlds communicate through immutable events on a centralized event bus, enabling safe, auditable, and composable workflows.

### Who Should Use QMI
Anyone looking for a new way to think about ECS while maintaining high-performance.

### Roadmap

Coming soon!

## License

This work is distributed under the MIT License. Dependencies either direct or indirect may have different licenses, so all of them have been reproduced in the THIRD-PARTY-LICENSES.md file at the root of this repository.
