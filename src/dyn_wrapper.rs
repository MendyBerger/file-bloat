use alloc::boxed::Box;
use alloc::vec::Vec;
use core::mem;
use core::{any::Any, marker::PhantomData};

#[repr(transparent)]
struct DynWrapper<T>
where
    T: Any,
{
    val: Box<dyn Any>,
    _a: PhantomData<T>,
}

impl<T> DynWrapper<T>
where
    T: Any,
{
    #[inline(always)]
    pub fn new(val: T) -> Self {
        Self {
            val: Box::new(val),
            _a: PhantomData,
        }
    }
}

impl<T> AsRef<T> for DynWrapper<T>
where
    T: Any,
{
    #[inline(always)]
    fn as_ref(&self) -> &T {
        unsafe { self.val.downcast_ref_unchecked::<T>() }
    }
}

#[no_mangle]
pub fn test_u32(num: u32) -> usize {
    let mut v: Vec<DynWrapper<u32>> = Vec::new();
    v.push(DynWrapper::new(2));
    v.push(DynWrapper::new(3));
    if num % 3 == 8 {
        for num in 0..num {
            v.push(DynWrapper::new(num));
        }
    }

    v.shrink_to_fit();
    let cap = v.capacity();
    mem::forget(v);
    cap
}
#[no_mangle]
pub fn test_u64(num: u64) -> bool {
    let mut v: Vec<DynWrapper<u64>> = Vec::new();
    v.push(DynWrapper::new(54309));
    v.push(DynWrapper::new(4323));

    for n in v {
        if n.as_ref() % &3 == num {
            return true;
        }
    }
    false
}

#[no_mangle]
pub fn test_char(c: char) -> char {
    let mut v: Vec<DynWrapper<char>> = vec![
        DynWrapper::new('h'),
        DynWrapper::new('e'),
        DynWrapper::new('l'),
        DynWrapper::new('l'),
        DynWrapper::new('o'),
        DynWrapper::new(' '),
    ];
    let rem = c as usize % 6;
    v.insert(rem, DynWrapper::new(c));
    let rem = c as usize % 5;
    *v[rem].as_ref()
}

#[no_mangle]
pub fn test_f64(num: f64) -> f64 {
    let mut v: Vec<DynWrapper<f64>> = vec![
        DynWrapper::new(0.9),
        DynWrapper::new(4324.2),
        DynWrapper::new(20.1),
        DynWrapper::new(4324.0),
        DynWrapper::new(432.0),
        DynWrapper::new(432.432),
        DynWrapper::new(234.43),
    ];
    let rem = num as usize % 6;
    v.insert(rem, DynWrapper::new(num));

    let output = v
        .into_iter()
        .reduce(|p, n| DynWrapper::new(*n.as_ref() * *n.as_ref() + *p.as_ref()))
        .unwrap_or(DynWrapper::new(0.0));

    *output.as_ref()
}
