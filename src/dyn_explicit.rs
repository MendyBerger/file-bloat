use alloc::boxed::Box;
use alloc::vec::Vec;
use core::any::Any;
use core::mem;

#[no_mangle]
pub fn test_u32(num: u32) -> usize {
    let mut v: Vec<Box<dyn Any>> = Vec::new();
    v.push(Box::new(2));
    v.push(Box::new(3));
    if num % 3 == 8 {
        for num in 0..num {
            v.push(Box::new(num));
        }
    }

    v.shrink_to_fit();
    let cap = v.capacity();
    mem::forget(v);
    cap
}
#[no_mangle]
pub fn test_u64(num: u64) -> bool {
    let mut v: Vec<Box<dyn Any>> = Vec::new();
    v.push(Box::new(54309));
    v.push(Box::new(4323));

    for n in v {
        let n = unsafe { n.downcast_ref_unchecked::<u64>() };
        if n % &3 == num {
            return true;
        }
    }
    false
}

#[no_mangle]
pub fn test_char(c: char) -> char {
    let mut v: Vec<Box<dyn Any>> = vec![
        Box::new('h'),
        Box::new('e'),
        Box::new('l'),
        Box::new('l'),
        Box::new('o'),
        Box::new(' '),
    ];
    let rem = c as usize % 6;
    v.insert(rem, Box::new(c));
    let rem = c as usize % 5;
    unsafe { *v[rem].downcast_ref_unchecked::<char>() }
}

#[no_mangle]
pub fn test_f64(num: f64) -> f64 {
    let mut v: Vec<Box<dyn Any>> = vec![
        Box::new(0.9),
        Box::new(4324.2),
        Box::new(20.1),
        Box::new(4324.0),
        Box::new(432.0),
        Box::new(432.432),
        Box::new(234.43),
    ];
    let rem = num as usize % 6;
    v.insert(rem, Box::new(num));

    let output = v
        .into_iter()
        .reduce(|p, n| {
            let p = unsafe { p.downcast_unchecked::<f64>() };
            let n = unsafe { n.downcast_unchecked::<f64>() };
            Box::new(*n * *n + *p) as Box<dyn Any>
        })
        .unwrap_or(Box::new(0.0) as Box<dyn Any>);

    let output = unsafe { output.downcast_unchecked::<f64>() };
    *output
}
