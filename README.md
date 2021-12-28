# SailDB test

This is a quick example of SailDB

```bash
cargo run --bin=server -- --bind="tcp@127.0.0.1:8080"
```
```bash
cargo run --bin=client -- --addr="tcp@127.0.0.1:8080" insert --key="hello" --value="world!"
cargo run --bin=client -- --addr="tcp@127.0.0.1:8080" get --key="hello"
```