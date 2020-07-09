extern crate proc_macro;

#[cfg(not(feature = "prusti"))]
mod private {
    use proc_macro_hack::proc_macro_hack;

    /// A macro for writing a precondition on a function.
    pub use prusti_contracts_impl::requires;

    /// A macro for writing a postcondition on a function.
    pub use prusti_contracts_impl::ensures;

    /// A macro for writing a pledge on a function.
    pub use prusti_contracts_impl::after_expiry;

    /// A macro for writing a conditional pledge on a function.
    pub use prusti_contracts_impl::after_expiry_if;

    /// A macro for writing a loop invariant.
    #[proc_macro_hack]
    pub use prusti_contracts_impl::invariant;
}

#[cfg(feature = "prusti")]
mod private {
    use proc_macro_hack::proc_macro_hack;

    /// A macro for writing a precondition on a function.
    pub use prusti_contracts_internal::requires;

    /// A macro for writing a postcondition on a function.
    pub use prusti_contracts_internal::ensures;

    /// A macro for writing a pledge on a function.
    pub use prusti_contracts_internal::after_expiry;

    /// A macro for writing a conditional pledge on a function.
    pub use prusti_contracts_internal::after_expiry_if;

    /// A macro for writing a loop invariant.
    #[proc_macro_hack]
    pub use prusti_contracts_internal::invariant;
}

pub use private::*;
