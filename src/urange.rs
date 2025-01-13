use crate::Encode;

pub struct URange<const N: usize>(usize);

impl<const N: usize> From<URange<N>> for usize {
    fn from(value: URange<N>) -> Self {
        value.0
    }
}

impl<const N: usize> TryFrom<usize> for URange<N> {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value < N {
            Ok(URange(value))
        } else {
            Err(())
        }
    }
}

pub struct URangeContext<const N: usize> {
    values: [<bool as Encode>::Context; N],
    bits: [<bool as Encode>::Context; N],
}
