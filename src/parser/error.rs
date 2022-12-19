impl<T> Result<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(value) => value,
            Self::Err(error) => panic!("{}", error)
        }
    }
}