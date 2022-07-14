Char 

1
```
use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!");
} 
// Changed c1 and c2 values to 4
```

2
```
fn main() {
    let c1 = '中';
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}
// changed the "中" -> '中'
```
Bool

3
```
fn main() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("hello, world");
    }
} 
// Changed the t = true to make the "if" condition work
```

4
```
fn main() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);
}
// Replaced the && to ||\
```
Unit type

5
```
fn main() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit())
}

fn implicitly_ret_unit() {
    println!("I will returen a ()")
}

// don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will returen a ()")
}

fn implicitly_ret_unit() {
    println!("I will returen a ()")
}

// don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will returen a ()")
}
// changed the first two lines to begin with v0
```

6    What's the size of the unit type?
```
use std::mem::size_of_val;

fn main() {
    let unit: () = ();
    // unit type does't occupy any memeory space
    assert!(size_of_val(&unit) == 0);
}
// the value reset to 0
```