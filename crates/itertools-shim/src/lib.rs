use std::fmt::Write;

use itertools::{
    ExactlyOneError, Intersperse, Itertools, MultiProduct, Permutations, Positions, Powerset,
};

pub trait ItertoolsShim: Iterator {
    fn join(&mut self, sep: &str) -> String
    where
        Self::Item: std::fmt::Display,
    {
        match self.next() {
            None => String::new(),
            Some(first_elt) => {
                // estimate lower bound of capacity needed
                let (lower, _) = self.size_hint();
                let mut result = String::with_capacity(sep.len() * lower);
                write!(&mut result, "{}", first_elt).unwrap();
                self.for_each(|elt| {
                    result.push_str(sep);
                    write!(&mut result, "{}", elt).unwrap();
                });
                result
            }
        }
    }

    fn permutations(self, k: usize) -> Permutations<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Itertools::permutations(self, k)
    }

    fn multi_cartesian_product(self) -> MultiProduct<<Self::Item as IntoIterator>::IntoIter>
    where
        Self: Sized,
        Self::Item: IntoIterator,
        <Self::Item as IntoIterator>::IntoIter: Clone,
        <Self::Item as IntoIterator>::Item: Clone,
    {
        Itertools::multi_cartesian_product(self)
    }

    fn exactly_one(self) -> Result<Self::Item, ExactlyOneError<Self>>
    where
        Self: Sized,
    {
        Itertools::exactly_one(self)
    }

    fn powerset(self) -> Powerset<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Itertools::powerset(self)
    }

    fn positions<P>(self, predicate: P) -> Positions<Self, P>
    where
        Self: Sized,
        P: FnMut(Self::Item) -> bool,
    {
        Itertools::positions(self, predicate)
    }

    fn intersperse(self, element: Self::Item) -> Intersperse<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Itertools::intersperse(self, element)
    }
}

impl<I: ?Sized> ItertoolsShim for I where I: Iterator {}
