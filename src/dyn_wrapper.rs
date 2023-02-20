use core::fmt::Display;
use core::{marker::PhantomData};
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[repr(transparent)]
struct DynWrapper<T>
where
    T: Display + 'static,
{
    val: Box<dyn Display>,
    _a: PhantomData<T>,
}

impl<T> DynWrapper<T>
where
    T: Display + 'static,
{
    #[inline(always)]
    pub fn new(val: T) -> Self {
        Self {
            val: Box::new(val),
            _a: PhantomData,
        }
    }
}

impl<T: Display + 'static> Display for DynWrapper<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.val.fmt(f)
    }
}

#[no_mangle]
pub fn test_u32(num: u32) -> String {
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
    let s: String = v
        .into_iter()
        .map(|n| format!("{n}"))
        .collect();
    format!("cap: {cap}, {s}")
}
#[no_mangle]
pub fn test_u64(num: u64) -> String {
    let mut v: Vec<DynWrapper<u64>> = Vec::new();
    v.push(DynWrapper::new(54309));
    v.push(DynWrapper::new(4323));
    v.push(DynWrapper::new(num));

    
    let s = v
        .into_iter()
        .map(|n| format!("{n}"))
        .collect();
    s
}

#[no_mangle]
pub fn test_char(c: char) -> String {
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
    
    let s = v
        .into_iter()
        .map(|n| format!("{n} {rem}"))
        .collect();
    s
}

#[no_mangle]
pub fn test_f64(num: f64) -> String {
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

    let s = v
        .into_iter()
        .map(|n| format!("{n}"))
        .collect();
    s
}
