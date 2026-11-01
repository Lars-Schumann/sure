/// Wraps a pure function.
#[derive(Clone, Copy)]
pub struct Pure<F> {
    inner: F,
}

impl<F: Copy + Send + Sync + 'static> Pure<F> {
    /// # Safety
    ///
    /// `f` must be pure (for some definition of yet to be defined purity).
    pub const unsafe fn new(f: F) -> Self {
        const { assert!(core::mem::size_of::<F>() == 0) }
        Self { inner: f }
    }

    /// Access the stored pure function.
    pub const fn inner(self) -> F {
        self.inner
    }
}

/// Allows method syntax instead of [`Pure::new`]
pub trait AssertPure: Sized {
    /// # Safety
    ///
    /// This inherits the safety requirements from [`Pure::new`].
    unsafe fn assert_pure(self) -> Pure<Self>;
}

impl<F: Copy + Send + Sync + 'static> AssertPure for F {
    unsafe fn assert_pure(self) -> Pure<Self> {
        // SAFETY: The safety requirements are passed to the caller.
        unsafe { Pure::new(self) }
    }
}
