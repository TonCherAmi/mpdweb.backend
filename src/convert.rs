pub trait IntoOption where Self: Sized {
    fn into_some(self) -> Option<Self> {
        Some(self)
    }
}

impl<T> IntoOption for T {
    // Default.
}

pub trait IntoResult where Self: Sized {
    fn into_ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }
}

impl<T> IntoResult for T {
    // Default.
}

pub trait IntoVec where Self: Sized {
    fn into_vec(self) -> Vec<Self> {
        vec![self]
    }
}

impl<T> IntoVec for T {
    // Default.
}

pub trait MapInto<T: Into<U>, U>: Sized + IntoIterator<Item=T> {
    fn map_into<R: FromIterator<U>>(self) -> R {
        self.into_iter()
            .map(Into::into)
            .collect()
    }
}

impl<T: Into<U>, U> MapInto<T, U> for Vec<T> {
    // Default.
}
