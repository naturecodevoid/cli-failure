/*!
# cli-failure

Provides `Failure(String)` implementing `std::error::Error`. Includes convenience macros making it perfect for usage with `wrap-match` in CLIs.

**This crate should not be used in libraries.** Instead, use something like [`thiserror`](https://docs.rs/thiserror). For libraries, it is much better to have specific errors so library users can
handle them better.

[Documentation](https://docs.rs/cli-failure) | [crates.io](https://crates.io/crates/cli-failure)

## Example

```
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
 */

use std::error::Error;
use std::fmt::{Debug, Display};

#[macro_export]
/// Returns a [`Failure`] after calling [`Failure::err_boxed()`]. Any arguments to this macro will be passed to `format_args!`, allowing formatting specifiers to be used.
///
/// See [module level documentation](crate) for more docs
macro_rules! bail {
    ($($args:tt)*) => {
        return $crate::failure!($($args)*)
    }
}

#[macro_export]
/// Constructs a [`Failure`] and calls [`Failure::err_boxed()`]. Any arguments to this macro will be passed to `format_args!`, allowing formatting specifiers to be used.
///
/// See [module level documentation](crate) for more docs
macro_rules! failure {
    ($($args:tt)*) => {
        $crate::Failure(::std::fmt::format(::std::format_args!($($args)*))).err_boxed()
    }
}

#[macro_export]
/// Constructs a [`Failure`]. Any arguments to this macro will be passed to `format_args!`, allowing formatting specifiers to be used.
///
/// See [module level documentation](crate) for more docs
macro_rules! failure_raw {
    ($($args:tt)*) => {
        $crate::Failure(::std::fmt::format(::std::format_args!($($args)*)))
    }
}

/// It is recommend to use [`failure!`] or [`failure_raw!`] to construct a `Failure` as this saves typing `"...".to_owned()` or `format!("...")`.
///
/// See [module level documentation](crate) for more docs
pub struct Failure(pub String);

impl Failure {
    #[inline]
    /// Transforms the `Failure` into a `Box<dyn Error>`
    pub fn boxed(self) -> Box<dyn Error> {
        Box::new(self) as Box<dyn Error>
    }

    #[inline]
    /// Transforms the `Failure` into a `Result::Err(E)` when `E` implements `From<Box<dyn Error>>`
    pub fn err_boxed<T, E: From<Box<dyn Error>>>(self) -> Result<T, E> {
        Err(self.boxed().into())
    }

    #[inline]
    /// Transforms the `Failure` into a `Result::Err(Failure)`
    pub fn err<T>(self) -> Result<T, Self> {
        Err(self)
    }
}

impl Error for Failure {}

impl Display for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Debug for Failure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

#[cfg(test)]
#[test]
#[allow(
    unreachable_code,
    unused_variables // since result is used in unreachable code the compiler says it's unused
)]
fn tests() {
    assert_eq!(Failure("a".into()).to_string(), "a".to_owned());
    fn failure() -> Result<(), Failure> {
        let result = "bad";
        return Failure("test".to_owned()).err();
        return Err(Failure("test".to_owned()));
        return failure_raw!("{result}").err();
        return Err(failure_raw!("{result}"));
        Ok(())
    }
    failure().unwrap_err();

    fn box_dyn_error() -> Result<(), Box<dyn Error>> {
        let result = "bad";
        return Failure("test".to_owned()).err_boxed();
        return Err(Failure("test".to_owned()).boxed());
        bail!("{result}");
        return failure!("{result}");
        return failure_raw!("{}", result).err_boxed();
        return Err(failure_raw!("{result}").boxed());
        Ok(())
    }
    box_dyn_error().unwrap_err();
}
