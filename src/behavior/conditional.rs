/// [`Conditional`] represents a collection of functions to do things conditionally.
pub trait Conditional {
    /// [`Conditional::only_if`] will return [`self`] if the [`bool`] condition is `true`,
    /// and [`Default`] otherwise.
    /// # Example
    /// [`Conditional::only_if`] can be used to return [`self`] only if a condition is `true`:
    /// ```rust
    /// use filter_maker::behavior::conditional::Conditional;
    ///
    /// fn say_short_hello(greeting: &str) -> String {
    ///     greeting.to_string().only_if(greeting.len() < 10)
    /// }
    /// ```
    fn only_if(self, condition: bool) -> Self;

    /// [`Conditional::if_not_default`] will return [`self`] if it is not [`Default`],
    /// and [`Default`] otherwise.
    /// # Example
    /// [`Conditional::if_not_default`] can be used to return [`self`] only if [`self`] is not
    /// [`Default`]:
    /// ```rust
    /// use filter_maker::behavior::conditional::Conditional;
    ///
    /// fn pass_along_valid_greeting(mut name: &str) -> String {
    ///     format!("Hello {name}!").if_not_default(&mut name)
    /// }
    /// ```
    fn if_not_default<N: Default + PartialEq>(self, n: &N) -> Self;

    /// [`Conditional::is_default`] will return `true` if [`self`] is [`Default`],
    /// and `false` otherwise.
    /// # Example
    /// [`Conditional::is_default`] can be used to check if something is [`Default`]:
    /// ```rust
    /// use filter_maker::behavior::conditional::Conditional;
    ///
    /// fn check_if_greeting_is_empty(greeting: &str) -> bool {
    ///     greeting.is_default()
    /// }
    /// ```
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
