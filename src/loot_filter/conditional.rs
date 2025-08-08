pub trait Conditional {
    fn only_if(self, condition: bool) -> Self;
    fn except_if(self, condition: bool) -> Self;
    fn if_not_default<N: Default + PartialEq>(self, n: &N) -> Self;
    fn is_default(&self) -> bool;
}

impl<T: Default + PartialEq> Conditional for T {
    fn only_if(self, condition: bool) -> Self {
        condition.then_some(self).unwrap_or_default()
    }

    fn except_if(self, condition: bool) -> Self {
        (!condition).then_some(self).unwrap_or_default()
    }

    fn if_not_default<N: Default + PartialEq>(self, n: &N) -> Self {
        (n != &N::default()).then_some(self).unwrap_or_default()
    }

    fn is_default(&self) -> bool {
        self == &Self::default()
    }
}
