# cli-failure

Provides `Failure(String)` implementing `std::error::Error`. Includes convenience macros making it perfect for usage with `wrap-match` in CLIs.

**This crate should not be used in libraries.** Instead, use something like [`thiserror`](https://docs.rs/thiserror). For libraries, it is much better to have specific errors so library users can
handle them better.

[Documentation](https://docs.rs/cli-failure) | [crates.io](https://crates.io/crates/cli-failure)

## Example

```rs
// wrap-match is not required, but it is highly recommended
fn example() -> Result<(), Box<dyn Error>> {
    let result = "bad";
    // With convenience macros

    // These two lines are the same
    bail!("something {result} happened");
    return failure!("something {result} happened");

    return failure_raw!("something {result} happened").err_boxed();
    return Err(failure_raw!("something {result} happened").boxed());
    // Manually
    return Failure(format!("something {result} happened")).err_boxed();
    return Err(Failure(format!("something {result} happened")).boxed());
    Ok(())
}
```
