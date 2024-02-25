pub fn switch<T, U, E>(
    mut f: impl FnMut(T) -> Result<U, E>,
) -> impl FnMut(Result<T, E>) -> Result<U, E> {
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
