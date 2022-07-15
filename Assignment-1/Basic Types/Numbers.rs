Integer

1
```
fn main() {
    let _x: u32 = 5;
    let mut _y: u32 = 5;

   
    let _z:i32 = 10; // Type of z ? 

    println!("Success!");
}

// Remove the line y=x and added the data type _z and changed x -> _x, y -> _y.  */  
```

2
```
fn main() {
    let _v: u16 = 38_u8 as u16;
    println!("Success!");
}

// added u16
```

3 
```
fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));
}
// Determine the type of a variable and return a string representation of the type, such as "i8", "u8", "i16", "u16", "i64", "u64", "i32", "u32".
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
```

4 
```
fn main() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 
    println!("Success!");
}
// 127 and 255 were added as the maximum i8 and u8 values.
```

5
```
fn main() {
   let v1 = 247_u8 + 8;
   let v2 = i8::checked_add(119, 8).unwrap();
   println!("{},{}",v1,v2);
}
// Because u8 and i8 have maximum values, I changed 251 u8 to 247 u8 in v1 and add(251,8) to (119,8) in v2.
```

6
```
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
    println!("Success!");
}
// changed v == 1579 to 1597 because adding the v yields 1597.
```

Floating-Point

7
```
fn main() {
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    println!("Success!");
} 
// added f64 to cmnt area to describe its type.
```

8
```
fn main() {
    assert!(0.1_f32+0.2_f32==0.3_f32);
} 
// added _f32 to line, in order to get float output
```

Range

9
```
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}
// added c as u8
```

10
```
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    println!("Success!");
}
// 5 and =5 were filled
```

Computations

11
```
fn main() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1); 
    
    assert!(3 * 50 == 150);

    assert!(9/ 3 == 3); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

output:
0011 AND 0101 is 0001
0011 OR 0101 is 0111
0011 XOR 0101 is 0110
1 << 5 is 32
0x80 >> 2 is 0x20\

// True/False added in Boolean logic and added unsigned datatype in some areas
```
