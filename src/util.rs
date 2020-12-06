pub trait IteratorExt: Iterator {
    fn fold_with_first<F>(mut self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        let first = self.next()?;
        Some(self.fold(first, f))
    }
}

impl<T: ?Sized> IteratorExt for T where T: Iterator {}
