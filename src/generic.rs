use alloc::vec::Vec;
use core::mem;

#[no_mangle]
pub fn test_u32(num: u32) -> usize {
    let mut v: Vec<u32> = Vec::new();
    v.push(2);
    v.push(3);
    if num % 3 == 8 {
        for num in 0..num {
            v.push(num);
        }
    }

    v.shrink_to_fit();
    let cap = v.capacity();
    mem::forget(v);
    cap
}
#[no_mangle]
pub fn test_u64(num: u64) -> bool {
    let mut v: Vec<u64> = Vec::new();
    v.push(54309);
    v.push(4323);

    for n in v {
        if n % &3 == num {
            return true;
        }
    }
    false
}

#[no_mangle]
pub fn test_char(c: char) -> char {
    let mut v: Vec<char> = vec!['h', 'e', 'l', 'l', 'o', ' '];
    let rem = c as usize % 6;
    v.insert(rem, c);
    let rem = c as usize % 5;
    v[rem]
}

#[no_mangle]
pub fn test_f64(num: f64) -> f64 {
    let mut v: Vec<f64> = vec![0.9, 4324.2, 20.1, 4324.0, 432.0, 432.432, 234.43];
    let rem = num as usize % 6;
    v.insert(rem, num);

    let output = v.into_iter().reduce(|p, n| n * n + p).unwrap_or(0.0);

    output
}
