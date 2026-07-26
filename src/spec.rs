use crate::const_helpers::not;
use core::any::TypeId;
use core::intrinsics::transmute_unchecked;
use core::marker::Destruct;

const fn type_eq<T: 'static, U: 'static>() -> bool {
    TypeId::of::<T>() == TypeId::of::<U>()
}

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
    F: [const] FnOnce(ConcreteInput) -> ConcreteOutput + [const] Destruct,
>(
    generic_input: GenericInput,
    f: F,
) -> Result<GenericOutput, GenericInput> {
    if not(type_eq::<GenericInput, ConcreteInput>()) || not(type_eq::<GenericOutput, ConcreteOutput>()) {
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
pub(crate) fn try_fn_mut<Gen: 'static, Con: 'static, F: FnMut(&mut Con)>(
    g: Gen,
    mut f: F,
) -> Result<Gen, Gen> {
    let mut concrete: Con = try_transmute::<Gen, Con>(g)?;
    f(&mut concrete);

    Ok(try_transmute::<Con, Gen>(concrete).ok().unwrap())
}
