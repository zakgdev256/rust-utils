pub trait IterChain<T> {
    type Output;

    fn chain<F>(self, f: F) -> Self::Output
    where F: FnMut(T) -> Self;
}

impl<T> IterChain<T> for Option<T> {
    type Output = ();

    fn chain<F>(self, mut f: F) -> Self::Output
    where F: FnMut(T) -> Self
    {
        let mut opt = self;
        while let Some(val) = opt {
            opt = f(val)
        }
    }
}

impl<T, E> IterChain<T> for Result<T, E> {
    type Output = E;

    fn chain<F>(self, mut f: F) -> Self::Output
    where F: FnMut(T) -> Self
    {
        let mut res = self;
        loop {
            match res {
                Ok(val) => res = f(val),
                Err(err) => break err
            }
        }
    }
}