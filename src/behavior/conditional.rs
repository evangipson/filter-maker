/// [`Conditional`] represents a collection of functions to do things conditionally.
pub trait Conditional {
    /// [`Conditional::only_if`] will return [`self`] if the [`bool`] condition is `true`, and [`Default::default`] otherwise.
    fn only_if(self, condition: bool) -> Self;

    /// [`Conditional::if_not_default`] will return [`self`] if it is not [`Default::default`], and [`Default::default`] otherwise.
    fn if_not_default<N: Default + PartialEq>(self, n: &N) -> Self;

    /// [`Conditional::is_default`] will return `true` if [`self`] is [`Default::default`], and `false` otherwise.
    fn is_default(&self) -> bool;
}

/// Implement [`Conditional`] for anything that implements [`Default`] and [`PartialEq`].
impl<T: Default + PartialEq> Conditional for T {
    fn only_if(self, condition: bool) -> Self {
        if condition { self } else { Default::default() }
    }

    fn if_not_default<N: Default + PartialEq>(self, n: &N) -> Self {
        if n != &N::default() {
            self
        } else {
            Default::default()
        }
    }

    fn is_default(&self) -> bool {
        self == &Self::default()
    }
}
