# cli-failure

Provides `Failure(String)` implementing `std::error::Error`. Includes convenience macros making it perfect for usage with `wrap-match` in CLIs.

**This crate should not be used in libraries.** Instead, use something like [`thiserror`](https://docs.rs/thiserror). For libraries, it is much better to have specific errors so library users can
handle them better.

This crate is not yet on crates.io because I haven't published any crates that use it to crates.io yet. If you use this crate and need to publish your crate to crates.io, create an issue or contact me
via other means.

## Example

```rs
// wrap-match is not required, but it is highly recommended
fn example() -> Result<(), Box<dyn Error>> {
    let result = "bad";
    // With convenience macros
    return failure!("something {result} happened");
    return failure_raw!("something {result} happened").err_boxed();
    return Err(failure_raw!("something {result} happened").boxed());
    // Manually
    return Failure(format!("something {result} happened")).err_boxed();
    return Err(Failure(format!("something {result} happened")).boxed());
    Ok(())
}
```
