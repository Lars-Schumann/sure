use crate::base::Set;
use crate::base::Sure;
use crate::sure_eq::SureEq;
use core::marker::Destruct;

const CODOMAIN1<
    I: SureEq + Copy,
    O: SureEq + Copy,
    const INP: Set<I>,
    F: const Fn(I) -> O + Copy + Send + Sync + 'static,
>: Set<O> = {
    use crate::set::LENGTH;
    use core::mem::MaybeUninit;
    use core::mem::conjure_zst;

    const { assert!(core::mem::size_of::<F>() == 0) }

    // SAFETY: trust me bro
    let f: F = unsafe { conjure_zst() };

    let mut codomain: [MaybeUninit<O>; LENGTH::<I, INP>] = [MaybeUninit::uninit(); _];
    let codomain_len: usize = codomain.len();

    let mut i: usize = 0;

    while i < codomain_len {
        codomain[i] = MaybeUninit::new(f(INP[i]));
        i += 1;
    }

    // SAFETY: trust me bro
    &unsafe { MaybeUninit::array_assume_init(codomain) }
};

const CODOMAIN2<
    I1: SureEq + Copy,
    I2: SureEq + Copy,
    O: SureEq + Copy,
    const INP1: Set<I1>,
    const INP2: Set<I2>,
    F: const Fn(I1, I2) -> O + Copy + Send + Sync + 'static,
>: Set<O> = {
    use crate::set::CARTESIAN_LENGTH;
    use core::mem::MaybeUninit;
    use core::mem::conjure_zst;

    const { assert!(core::mem::size_of::<F>() == 0) }

    // SAFETY: trust me bro
    let f: F = unsafe { conjure_zst() };

    let inp2_len = INP2.len();

    let mut codomain: [MaybeUninit<O>; CARTESIAN_LENGTH::<I1, I2, INP1, INP2>] =
        [MaybeUninit::uninit(); _];
    let codomain_len: usize = codomain.len();

    let mut i: usize = 0;

    while i < codomain_len {
        codomain[i] = MaybeUninit::new(f(INP1[i / inp2_len], INP2[i % inp2_len]));
        i += 1;
    }

    // SAFETY: trust me bro
    &unsafe { MaybeUninit::array_assume_init(codomain) }
};

/// # Safety
///
/// This function is safe, because it relies on the purity of const functions and the fact that const functions are not allowed to detect whether they are running in const or not.
/// Any usage of unstable features to circumvent either of these are considered to be not my problem.  
pub const fn f1<
    I: SureEq + const Ord + Copy + const Destruct,
    const I_SET: Set<I>,
    O: SureEq + const Ord + Copy + const Destruct,
    F: const Fn(I) -> O + Copy + Send + Sync + 'static,
>(
    input: Sure<I, I_SET>,
    f: F,
) -> Sure<O, { CODOMAIN1::<I, O, I_SET, F> }> {
    const { assert!(core::mem::size_of::<F>() == 0) }
    // SAFETY: trust me bro
    unsafe { Sure::new_unchecked(f(input.inner())) }
}

/// # Safety
///
/// This function is safe, because it relies on the purity of const functions and the fact that const functions are not allowed to detect whether they are running in const or not.
/// Any usage of unstable features to circumvent either of these are considered to be not my problem.  
pub const fn f2<
    I1: SureEq + const Ord + Copy + const Destruct,
    I2: SureEq + const Ord + Copy + const Destruct,
    const INP1: Set<I1>,
    const INP2: Set<I2>,
    O: SureEq + const Ord + Copy + const Destruct,
    F: const Fn(I1, I2) -> O + Copy + Send + Sync + 'static,
>(
    input1: Sure<I1, INP1>,
    input2: Sure<I2, INP2>,
    f: F,
) -> Sure<O, { CODOMAIN2::<I1, I2, O, INP1, INP2, F> }> {
    const { assert!(core::mem::size_of::<F>() == 0) }
    // SAFETY: trust me bro
    unsafe { Sure::new_unchecked(f(input1.inner(), input2.inner())) }
}

/// Enables method syntax for the [`f1`] function.
pub trait FnExt1<
    I: SureEq + const Ord + Copy + const Destruct,
    const I_SET: Set<I>,
    O: SureEq + const Ord + Copy + const Destruct,
>
{
    /// Equivalent to [`f1`].
    fn f(self, input: Sure<I, I_SET>) -> Sure<O, { CODOMAIN1::<I, O, I_SET, Self> }>
    where
        Self: const Fn(I) -> O + Copy + Send + Sync + 'static;
}

impl<
    I: SureEq + const Ord + Copy + const Destruct,
    const I_SET: Set<I>,
    O: SureEq + const Ord + Copy + const Destruct,
    F: const Fn(I) -> O + Copy + Send + Sync + 'static,
> FnExt1<I, I_SET, O> for F
{
    fn f(self, input: Sure<I, I_SET>) -> Sure<O, { CODOMAIN1::<I, O, I_SET, Self> }>
    where
        Self: const Fn(I) -> O + Copy + Send + Sync + 'static,
    {
        // SAFETY: trust me bro.
        unsafe { Sure::new_unchecked(self(input.inner())) }
    }
}

/// Enables method syntax for the [`f2`] function.
pub trait FnExt2<
    I1: SureEq + const Ord + Copy + const Destruct,
    I2: SureEq + const Ord + Copy + const Destruct,
    const INP1: Set<I1>,
    const INP2: Set<I2>,
    O: SureEq + const Ord + Copy + const Destruct,
>
{
    /// Equivalent to [`f2`].
    fn f(
        self,
        input1: Sure<I1, INP1>,
        input2: Sure<I2, INP2>,
    ) -> Sure<O, { CODOMAIN2::<I1, I2, O, INP1, INP2, Self> }>
    where
        Self: const Fn(I1, I2) -> O + Copy + Send + Sync + 'static;
}

impl<
    I1: SureEq + const Ord + Copy + const Destruct,
    I2: SureEq + const Ord + Copy + const Destruct,
    const INP1: Set<I1>,
    const INP2: Set<I2>,
    O: SureEq + const Ord + Copy + const Destruct,
    F: const Fn(I1, I2) -> O + Copy + Send + Sync + 'static,
> FnExt2<I1, I2, INP1, INP2, O> for F
{
    fn f(
        self,
        input1: Sure<I1, INP1>,
        input2: Sure<I2, INP2>,
    ) -> Sure<O, { CODOMAIN2::<I1, I2, O, INP1, INP2, Self> }>
    where
        Self: const Fn(I1, I2) -> O + Copy + Send + Sync + 'static,
    {
        // SAFETY: trust me bro.
        unsafe { Sure::new_unchecked(self(input1.inner(), input2.inner())) }
    }
}
