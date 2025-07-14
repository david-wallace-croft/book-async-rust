# Book Async Rust

[![MIT licensed][mit-badge]][mit-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/david-wallace-croft/book-async-rust/blob/main/LICENSE.txt

- Code adapted from the book "Async Rust" by Flitton & Morton
  - https://www.oreilly.com/library/view/async-rust/9781098149086
  - https://github.com/maxwellflitton/async-rust-oreilly
  - https://www.oreilly.com/catalog/errata.csp?isbn=9781098149093

## Usage

- rustup default nightly
- cargo run --bin ch01-fibonacci
- cargo run --bin ch01-http-one
- cargo run --bin ch01-http-four
- cargo run --bin ch01-http-four-join
- cargo run --bin ch01-connection
- cargo run --bin ch01-server
- scripts/ch01-prep.sh
- scripts/ch01-run.sh
- scripts/ch01-time.sh
- cargo run --bin ch01-server-four
- cargo run --bin ch01-fibonacci-one
- cargo run --bin ch01-fibonacci-four
- cargo run --bin ch01-condvar
- touch data.txt
- cargo run --bin ch01-file
- cargo run --bin ch01-http-async
- cargo run --bin ch02-tasks
- cargo run --bin ch02-futures
- cargo run --bin ch02-pinning
- cargo run --bin ch02-context
- cargo run --bin ch02-waking
- cargo run --bin ch02-sharing
- cargo run --bin ch02-high-level
- cargo run --bin ch03-building
- cargo run --bin ch03-increasing
- cargo run --bin ch03-passing
- cargo run --bin ch03-task
- cargo run --bin ch03-refactoring
- cargo run --bin ch03-creating
- cargo run --bin ch03-configuring
- cargo run --bin ch03-running
- cargo run --bin ch04-p073-integrating
- cargo run --bin ch04-p074-building
- cargo run --bin ch04-p077-implementing
- cargo run --bin ch04-p079-implementing
- cargo run --bin ch04-p081-connecting
- cargo run --bin ch04-p082-introducing
- cargo run --bin ch05-p091-why
- cargo run --bin ch05-p095-implementing
- cargo run --bin ch05-p096-stacking
- cargo run --bin ch05-p098-calling
- cargo run --bin ch05-p100-mimicking
- cargo run --bin ch05-p104-controlling
- cargo test --bin ch05-p109-testing
- cargo test --bin ch06-p116-defining
- cargo run --bin ch06-p116-defining
- cargo run --bin ch06-p124-getting
- cargo run --bin ch06-p128-building
- cargo run --bin ch07-p137-building
- cargo run --bin ch07-p142-processing
- cargo run --bin ch07-p147-getting
- cargo run --bin ch07-p149-graceful
- cargo run --bin ch08-p155-building
- cargo run --bin ch08-p157-working --release
- cargo run --bin ch08-p160-implementing
- cargo run --bin ch08-p165-implementing
- echo "{}" > data.json
- cargo run --bin ch08-p170-creating
- cargo run --bin ch09-p179-building
- cargo run --bin ch09-p185-waterfall
- cargo run --bin ch09-p186-decorator
- cargo run --bin ch09-p186-decorator --features "logging_decorator"
- cargo run --bin ch09-p188-future
- cargo run --bin ch09-p190-state
- cargo run --bin ch09-p193-retry
- cargo run --bin ch09-p194-circuit
- cargo test --test ch10-p197-setting
- cargo test --test ch10-p200-building
- cargo run --bin ch10-p205-running
- In separate terminals
  - cargo run --bin ch10-p211-accepting
  - cargo run --bin ch10-p215-building
- cargo test --lib -- ch11_p219_performing --show-output
- cargo test --lib -- ch11_p222_mocking --show-output
- cargo test --lib -- ch11_p225_testing
- cargo test --lib -- ch11_p227_testing
- cargo test --lib -- ch11_p229_testing --show-output
- cargo test --lib -- ch11_p231_testing
- cargo test --lib -- ch11_p233_fine_grained --show-output
- rustup default stable

## History

- 2025-05-04: Initial release
