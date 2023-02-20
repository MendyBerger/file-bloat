use core::fmt::Display;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;


#[no_mangle]
pub fn test_u32(num: u32) -> String {
    let mut v: Vec<Box<dyn Display>> = Vec::new();
    v.push(Box::new(2));
    v.push(Box::new(3));
    if num % 3 == 8 {
        for num in 0..num {
            v.push(Box::new(num));
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
    let mut v: Vec<Box<dyn Display>> = Vec::new();
    v.push(Box::new(54309));
    v.push(Box::new(4323));
    v.push(Box::new(num));

    
    let s = v
        .into_iter()
        .map(|n| format!("{n}"))
        .collect();
    s
}

#[no_mangle]
pub fn test_char(c: char) -> String {
    let mut v: Vec<Box<dyn Display>> = vec![
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
    
    let s = v
        .into_iter()
        .map(|n| format!("{n} {rem}"))
        .collect();
    s
}

#[no_mangle]
pub fn test_f64(num: f64) -> String {
    let mut v: Vec<Box<dyn Display>> = vec![
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

    let s = v
        .into_iter()
        .map(|n| format!("{n}"))
        .collect();
    s
}
