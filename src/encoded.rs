use crate::Encoded;

impl<T, S> Encoded<T, S> {
    /// Create a new `Encoded` from a value.
    #[inline]
    pub fn new(value: T) -> Self {
        Self::from(value)
    }
}

impl<T, S> From<T> for Encoded<T, S> {
    #[inline]
    fn from(value: T) -> Self {
        Self {
            value,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, S> std::ops::Deref for Encoded<T, S> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T, S> std::ops::DerefMut for Encoded<T, S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
