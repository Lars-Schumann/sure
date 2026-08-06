use core::any::TypeId;
use core::intrinsics::transmute_unchecked;
use core::marker::Destruct;

#[expect(unused)]
const fn type_eq<T: 'static, U: 'static>() -> bool {
    TypeId::of::<T>() == TypeId::of::<U>()
}

const fn type_ne<T: 'static, U: 'static>() -> bool {
    TypeId::of::<T>() != TypeId::of::<U>()
}

#[expect(unused)]
fn try_transmute<Src: 'static, Dst: 'static>(src: Src) -> Result<Dst, Src> {
    match type_eq::<Src, Dst>() {
        true => Ok(
            // SAFETY: the types are equal
            unsafe { transmute_unchecked::<Src, Dst>(src) },
        ),
        false => Err(src),
    }
}

#[rustfmt::skip]
pub(crate) const fn try_fn_once<
    GenericInput: 'static,
    GenericOutput: 'static,
    ConcreteInput: 'static,
    ConcreteOutput: 'static,
>(
    generic_input: GenericInput,
    f: impl [const] FnOnce(ConcreteInput) -> ConcreteOutput + [const] Destruct,
) -> Result<GenericOutput, GenericInput> {
    if const { type_ne::<GenericInput, ConcreteInput>() || type_ne::<GenericOutput, ConcreteOutput>() } {
        return Err(generic_input);
    }

    // SAFETY: these types are equal
    let concrete_input: ConcreteInput = unsafe { transmute_unchecked::<GenericInput, ConcreteInput>(generic_input) };

    let concrete_output: ConcreteOutput = f(concrete_input);

    // SAFETY: these types are equal
    let generic_output: GenericOutput = unsafe { transmute_unchecked::<ConcreteOutput, GenericOutput>(concrete_output) };

    Ok(generic_output)
}

#[expect(unused)]
#[rustfmt::skip]
pub(crate) fn try_fn_mut<
    GenericInput: 'static,
    ConcreteInput: 'static,
>(
    generic_input: GenericInput,
    mut f: impl FnMut(&mut ConcreteInput),
) -> Result<GenericInput, GenericInput> {
    if const { type_ne::<GenericInput, ConcreteInput>() } {
        return Err(generic_input);
    }

    // SAFETY: these types are equal
    let mut concrete_input: ConcreteInput = unsafe { transmute_unchecked::<GenericInput, ConcreteInput>(generic_input) };
    
    f(&mut concrete_input);

    // SAFETY: these types are equal
    let generic_input: GenericInput = unsafe { transmute_unchecked::<ConcreteInput, GenericInput>(concrete_input) };
    
    Ok(generic_input)
}
