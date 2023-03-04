//! The standard library named [`bool::then_some`] and [`bool::then`] strangely.
//! It should have been `then` and `then_with`. I find it annoying that
//! `expr.then(|| value)` is shorter to type that the more "idiomatic"
//! `expr.then_some(value)`
//!
//! This crate provides these functions under the following names
//! [`.some`][Some::some] and [`.some_with`][Some::some_with].
//!
//! # Getting started
//!
//! First, add the crate to your Cargo manifest.
//!
//! ```sh
//! cargo add then
//! ```
//!
//! Now bring the trait into scope.
//!
//! ```
//! use then::Some;
//! ```
//!
//! The `.some` and `.some_with` methods are now available on [`bool`].
//!
//! ```
//! # use then::Some;
//! assert_eq!(false.some(0), None);
//! assert_eq!(true.some_with(Default::default), Some(0));
//! ```

/// Provides the [`some`][Some::some] and [`some_with`][Some::some_with] methods
/// on [`bool`].
pub trait Some {
    /// Returns `Some(t)` if the [`bool`] is `true`, or [`None`] otherwise.
    ///
    /// Arguments passed to `some` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use
    /// [`some_with`][Some::some_with], which is lazily evaluated.
    ///
    /// # Examples
    ///
    /// ```
    /// use then::Some;
    ///
    /// assert_eq!(false.some(0), None);
    /// assert_eq!(true.some(0), Some(0));
    /// ```
    fn some<T>(self, t: T) -> Option<T>;

    /// Returns `Some(f())` if the [`bool`] is `true`, or [`None`] otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use then::Some;
    ///
    /// assert_eq!(false.some_with(|| 0), None);
    /// assert_eq!(true.some_with(Default::default), Some(0));
    /// ```
    fn some_with<T, F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> T;
}

impl Some for bool {
    #[inline]
    fn some<T>(self, t: T) -> Option<T> {
        if self {
            Some(t)
        } else {
            None
        }
    }

    #[inline]
    fn some_with<T, F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> T,
    {
        if self {
            Some(f())
        } else {
            None
        }
    }
}
