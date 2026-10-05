use std::mem::{self, MaybeUninit};

struct Guard<'data, T, const N: usize> {
    data: &'data mut [MaybeUninit<T>; N],
    initialised: usize,
}

impl<T, const N: usize> Drop for Guard<'_, T, N> {
    fn drop(&mut self) {
        for i in 0..self.initialised {
            // SAFETY: elemnts from 0 up to self.intialised were initialised.
            unsafe {
                self.data[i].assume_init_drop();
            }
        }
    }
}

pub fn try_array_from_fn<F, T, E, const N: usize>(mut f: F) -> Result<[T; N], E>
where
    F: FnMut(usize) -> Result<T, E>,
{
    let mut data: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut guard = Guard {
        data: &mut data,
        initialised: 0,
    };

    for i in 0..N {
        let elem = f(i)?;
        guard.data[i].write(elem);
        guard.initialised += 1;
    }

    mem::forget(guard);

    unsafe { Ok(mem::transmute_copy(&data)) }
}