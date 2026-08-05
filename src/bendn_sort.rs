use core::marker::Destruct;

#[allow(warnings, clippy::undocumented_unsafe_blocks, clippy::ptr_as_ptr)]
pub const fn cont2<T: [const] Clone>(
    array: &[T],
    key: &mut (impl const FnMut(&T) -> usize + [const] Destruct),
    shift: usize,
    counts: &mut [usize; 256],
) -> &'static [T] {
    if array.is_empty() {
        return &[];
    }
    let mut i = 0;
    while i < array.len() {
        counts[(key(&array[i]) >> shift) & 0xff] += 1;
        i += 1;
    }
    let mut i = 1;
    while i < 256 as usize {
        counts[i] += counts[i - 1];
        i += 1;
    }
    let outputs = unsafe {
        core::intrinsics::const_allocate(
            core::mem::size_of::<T>() * array.len(),
            core::mem::align_of::<T>(),
        )
    }
    .cast::<T>();
    let mut i = array.len() - 1;
    while {
        let j = (key(&array[i]) >> shift) & 0xff;
        counts[j] -= 1;
        unsafe { outputs.add(counts[j]).write(array[i].clone()) };
        i > 0
    } {
        i -= 1;
    }
    unsafe {
        core::slice::from_raw_parts(
            core::intrinsics::const_make_global(outputs as _) as _,
            array.len(),
        )
    }
}

#[allow(clippy::manual_bit_width)]
pub(crate) const fn radixsort<T: [const] Clone>(
    mut array: &'static [T],
    mut key: impl const FnMut(&T) -> usize + [const] Destruct,
) -> &'static [T] {
    let mut c = [0usize; 256];
    let mut i = 0;
    let bits = if array.len() < 50 {
        let mut m = 0;
        while i < array.len() {
            m = m.max(key(&array[i]));
            i += 1;
        }
        usize::BITS - m.leading_zeros()
    } else {
        usize::BITS
    };
    let mut shift = 0;
    while shift < bits {
        array = cont2(array, &mut key, shift as _, &mut c);
        shift += 8;
        c = [0; 256];
    }
    array
}
