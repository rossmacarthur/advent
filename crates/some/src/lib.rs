//! I just hate the naming of the standard library `bool::then` and
//! `bool::then_some` methods. I would have preferred to make it `then` and
//! `then_with`. It is so annoying that `expr.then(|| value)` is shorter to type
//! that the more idiomatic `expr.then_some(value)`

pub trait Some {
    fn some<T>(self, t: T) -> Option<T>;

    fn some_with<T, F>(self, f: F) -> Option<T>
    where
        F: FnOnce() -> T;
}

impl Some for bool {
    fn some<T>(self, t: T) -> Option<T> {
        if self {
            Some(t)
        } else {
            None
        }
    }

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
