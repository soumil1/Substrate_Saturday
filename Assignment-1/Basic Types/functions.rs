Functions

1
```
fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);
    assert_eq!(s, 3);
    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32  {
    x + y
}
// added i32 in x and  in function
```

2
```
fn main() {
    print();
}

// replace i32 with another type
fn print() -> () {
    println!("hello,world");
}
// Change it to () because i32 is not the return type.
```

3
```
fn main() {
    never_return();
}

fn never_return() -> ! {
    // implement this function, don't modify fn signatures
    panic!("Exactly! Not return anything")
}
// Add panic statement and remove unnecessary println
```

Diverging functions

4
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    unimplemented!()
}
// unimplemented function added 
```

5
```
fn main() {
    let b = false;

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}
// add false to b and change let v to _v
```


