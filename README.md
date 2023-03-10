# pretty-flexible-env-logger

[![Crates.io][crates-badge]][crates-url]
[![Docs][docs-badge]][docs-url]
[![CC0-1.0][license-badge]][crates-url]
[![GitHub CI][ci-badge]][ci-url]

A simple logger built on top of [pretty_env_logger](https://github.com/seanmonstar/pretty-env-logger). An upstream's [design decision](https://github.com/seanmonstar/pretty-env-logger) was not to allow run-time configuration, but I have found that for CLI tools it actually makes sense to control logging in runtime based on command-line params, rather than requiring users to set `RUST_LOG` directly.

To keep things simple, `init_with` tries to look up passed string as an environment variable, and if that fails it considers the value to be inlined configuration, same as you would set `RUST_LOG` with.

```rust
let args: Vec<String> = env::args().collect();
let default = "RUST_LOG".to_string();
let level = args.get(1).unwrap_or(&default);
if let Err(e) = pretty_flexible_env_logger::try_init_with(level) {
    eprintln!("Some custom msg {}", e);
    panic!("error!") // or whatever
}

info!("info");
warn!("warn");
error!("error");
debug!("debug");
```

## License

- Creative Commons 0  ([LICENSE](./LICENSE) or https://creativecommons.org/publicdomain/zero/1.0/)

## Related Work

This crate is a little contribution built on the shoulders of giants:
- [pretty_env_logger](https://github.com/seanmonstar/pretty-env-logger)
- [env_logger](https://github.com/rust-cli/env_logger/)

[crates-badge]: https://img.shields.io/crates/v/pretty_flexible_env_logger.svg
[crates-url]: https://crates.io/crates/pretty_flexible_env_logger
[docs-badge]: https://docs.rs/pretty_flexible_env_logger/badge.svg
[docs-url]: https://docs.rs/pretty_flexible_env_logger
[license-badge]: https://img.shields.io/crates/l/pretty_flexible_env_logger.svg
[ci-badge]: https://github.com/unjello/pretty_flexible_env_logger/actions/workflows/ci.yaml/badge.svg
[ci-url]: https://github.com/unjello/pretty_flexible_env_logger/actions/workflows/ci.yaml
