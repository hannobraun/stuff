use std::iter;

pub trait IteratorExt<T, E>: Iterator<Item = Result<T, E>> {
    fn switch<U>(
        self,
        mut f: impl FnMut(T) -> Result<U, E>,
    ) -> iter::Map<Self, impl FnMut(Result<T, E>) -> Result<U, E>>
    where
        Self: Sized,
    {
        self.map(move |res| match res {
            Ok(a) => f(a),
            Err(err) => Err(err),
        })
    }
}

impl<I, T, E> IteratorExt<T, E> for I where I: Iterator<Item = Result<T, E>> {}

pub fn track<A, B, E>(
    mut f: impl FnMut(A) -> B,
) -> impl FnMut(Result<A, E>) -> Result<B, E> {
    move |res| match res {
        Ok(a) => Ok(f(a)),
        Err(err) => Err(err),
    }
}
