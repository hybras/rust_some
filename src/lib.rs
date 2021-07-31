trait Some where Self: Sized {
    fn some(self) -> Option<Self>  {
        Some(self)
    }
}

impl<T> Some for T {}


#[cfg(test)]
mod tests {
    use crate::Some;

    #[test]
    fn it_works() {
        assert_eq!(4.some(), Some(4));
    }
}
