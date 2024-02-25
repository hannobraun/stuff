pub fn switch<T, B, E>(
    mut f: impl FnMut(T) -> Result<B, E>,
) -> impl FnMut(Result<T, E>) -> Result<B, E> {
    move |res| match res {
        Ok(a) => f(a),
        Err(err) => Err(err),
    }
}

pub fn track<A, B, E>(
    mut f: impl FnMut(A) -> B,
) -> impl FnMut(Result<A, E>) -> Result<B, E> {
    move |res| match res {
        Ok(a) => Ok(f(a)),
        Err(err) => Err(err),
    }
}
