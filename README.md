
# Success
- `cargo test --package spacetime_bug --bin spacetime_bug -- tests::test_success --exact --show-output`
shows that deserialization works properly when `big_number` is set to 2922044998640 and `spacetime-sdk` is a dependency in `Cargo.toml`
# Failure
- `cargo test --package spacetime_bug --bin spacetime_bug -- tests::test_failure --exact --show-output`
shows that deserialization does not work properly when `big_number` is set to 2.92204499864e12 and `spacetime-sdk` is a dependency in `Cargo.toml`
# Success
- `cargo test --package spacetime_bug --bin spacetime_bug -- tests::test_failure --exact --show-output`
works properly when `spacetime-sdk` is not a dependency in `Cargo.toml`