pub trait IteratorExt: Iterator {
    fn try_reduce<T, F>(self, f: F) -> Option<T>
    where
        Self: Sized + Iterator<Item = Option<T>>,
        F: FnMut(T, T) -> T;
}

impl<U: Iterator> IteratorExt for U {
    #[inline]
    #[allow(clippy::while_let_on_iterator)]
    fn try_reduce<T, F>(mut self, mut f: F) -> Option<T>
    where
        Self: Sized + Iterator<Item = Option<T>>,
        F: FnMut(T, T) -> T,
    {
        let mut accum = self.next()??;
        while let Some(Some(x)) = self.next() {
            accum = f(accum, x);
        }
        Some(accum)
    }
}
