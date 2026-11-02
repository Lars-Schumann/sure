use crate::Set;
use crate::Sure;
use crate::sure_eq::SureEq;

use core::marker::Destruct;

/// Wraps a pure function.
#[derive(Clone, Copy)]
pub struct Pure<F> {
    inner: F,
}

impl<F: Copy + Send + Sync + 'static> Pure<F> {
    /// # Safety
    ///
    /// `f` must be pure (for some definition of yet to be defined purity).
    pub const unsafe fn new_unchecked(f: F) -> Self {
        const { assert!(core::mem::size_of::<F>() == 0) }
        Self { inner: f }
    }

    /// Access the stored pure function.
    pub const fn inner(self) -> F {
        self.inner
    }

    /// TODO(doc)
    pub const fn call1<
        A: SureEq + const Ord + Copy + const Destruct,
        const A_SET: Set<A>,
        OUTPUT: SureEq + const Ord + Copy + const Destruct,
    >(
        self,
        a: Sure<A, A_SET>,
    ) -> Sure<OUTPUT, { CODOMAIN1::<A, A_SET, OUTPUT, F> }>
    where
        F: const Fn(A) -> OUTPUT + Copy + Send + Sync + 'static,
    {
        // SAFETY: TODO
        unsafe { Sure::new_unchecked(self.inner()(a.inner())) }
    }

    /// TODO(doc)
    pub const fn call2<
        A: SureEq + const Ord + Copy + const Destruct,
        B: SureEq + const Ord + Copy + const Destruct,
        const A_SET: Set<A>,
        const B_SET: Set<B>,
        OUTPUT: SureEq + const Ord + Copy + const Destruct,
    >(
        self,
        a: Sure<A, A_SET>,
        b: Sure<B, B_SET>,
    ) -> Sure<OUTPUT, { CODOMAIN2::<A, B, A_SET, B_SET, OUTPUT, F> }>
    where
        F: const Fn(A, B) -> OUTPUT + Copy + Send + Sync + 'static,
    {
        // SAFETY: TODO
        unsafe { Sure::new_unchecked(self.inner()(a.inner(), b.inner())) }
    }
}

/// Allows method syntax instead of [`Pure::new`]
pub trait AssumePure: Sized {
    /// # Safety
    ///
    /// This inherits the safety requirements from [`Pure::new`].
    unsafe fn assume_pure(self) -> Pure<Self>;
}

impl<F: Copy + Send + Sync + 'static> AssumePure for F {
    unsafe fn assume_pure(self) -> Pure<Self> {
        // SAFETY: The safety requirements are passed to the caller.
        unsafe { Pure::new_unchecked(self) }
    }
}

const CODOMAIN1<
    A: SureEq + Copy,
    const A_SET: Set<A>,
    OUTPUT: SureEq + Copy,
    F: const Fn(A) -> OUTPUT + Copy + Send + Sync + 'static,
>: Set<OUTPUT> = {
    use crate::set::LENGTH;
    use core::mem::MaybeUninit;
    use core::mem::conjure_zst;

    const { assert!(core::mem::size_of::<F>() == 0) }

    // SAFETY: trust me bro
    let f: F = unsafe { conjure_zst() };

    let mut codomain: [MaybeUninit<OUTPUT>; LENGTH::<A, A_SET>] = [MaybeUninit::uninit(); _];
    let codomain_len: usize = codomain.len();

    let mut i: usize = 0;

    while i < codomain_len {
        codomain[i] = MaybeUninit::new(f(A_SET[i]));
        i += 1;
    }

    // SAFETY: trust me bro
    &unsafe { MaybeUninit::array_assume_init(codomain) }
};

const CODOMAIN2<
    A: SureEq + Copy,
    B: SureEq + Copy,
    const A_SET: Set<A>,
    const B_SET: Set<B>,
    OUTPUT: SureEq + Copy,
    F: const Fn(A, B) -> OUTPUT + Copy + Send + Sync + 'static,
>: Set<OUTPUT> = {
    use crate::set::CARTESIAN_LENGTH;
    use core::mem::MaybeUninit;
    use core::mem::conjure_zst;

    const { assert!(core::mem::size_of::<F>() == 0) }

    // SAFETY: trust me bro
    let f: F = unsafe { conjure_zst() };

    let b_len = B_SET.len();

    let mut codomain: [MaybeUninit<OUTPUT>; CARTESIAN_LENGTH::<A, B, A_SET, B_SET>] =
        [MaybeUninit::uninit(); _];
    let codomain_len: usize = codomain.len();

    let mut i: usize = 0;

    while i < codomain_len {
        codomain[i] = MaybeUninit::new(f(A_SET[i / b_len], B_SET[i % b_len]));
        i += 1;
    }

    // SAFETY: trust me bro
    &unsafe { MaybeUninit::array_assume_init(codomain) }
};
