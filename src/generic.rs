use alloc::string::String;
use alloc::vec::{Vec};

#[no_mangle]
pub fn test_u32(num: u32) -> String {
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
    let s: String = v
        .into_iter()
        .map(|n| format!("{n}"))
        .collect();
    format!("cap: {cap}, {s}")
}
#[no_mangle]
pub fn test_u64(num: u64) -> String {
    let mut v: Vec<u64> = Vec::new();
    v.push(54309);
    v.push(4323);
    v.push(num);

    
    let s = v
        .into_iter()
        .map(|n| format!("{n}"))
        .collect();
    s
}

#[no_mangle]
pub fn test_char(c: char) -> String {
    let mut v: Vec<char> = vec![
        'h',
        'e',
        'l',
        'l',
        'o',
        ' ',
    ];
    let rem = c as usize % 6;
    v.insert(rem, c);
    let rem = c as usize % 5;
    
    let s = v
        .into_iter()
        .map(|n| format!("{n} {rem}"))
        .collect();
    s
}

#[no_mangle]
pub fn test_f64(num: f64) -> String {
    let mut v: Vec<f64> = vec![
        0.9,
        4324.2,
        20.1,
        4324.0,
        432.0,
        432.432,
        234.43,
    ];
    let rem = num as usize % 6;
    v.insert(rem, num);

    let s = v
        .into_iter()
        .map(|n| format!("{n}"))
        .collect();
    s
}
