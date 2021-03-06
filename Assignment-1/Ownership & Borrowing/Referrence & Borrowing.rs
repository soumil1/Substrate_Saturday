Reference


1
```
fn main() {
   let x = 5;
   let p = &x;

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}
// added &x to get x's address
```

2
```
fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, *y);

    println!("Success!");
}
// inserted * in y
```

3
```
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);

    println!("Success!");
}

fn borrow_object(s: &String) {}
// replaced &s
```

4
```
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}
// &mut is added to get s
```

5
```
fn main() {
    let mut s = String::from("hello, ");

    let p = &mut s;
    
    p.push_str("world");

    println!("Success!");
}
// &mut is added to get s
```

Ref

6
```
fn main() {
    let c = '中';

    let r1 = &c;
    let ref r2 = c;

    assert_eq!(*r1, *r2);
    
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success!");
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
// used ref in r2
```

Borrowing Rules

7
```
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}
// &s added in r1 and r2.
```

Mutability 

8
```
fn main() {

    let mut s = String::from("hello, ");

    borrow_object(&mut s);

    println!("Success!");
}
fn borrow_object(s: &mut String) {}
// &mut in s is used to borrow an object.
```

9
```
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&mut s);
    
    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {}
// &s replaced with &mut s.
```

NLL 

10
```
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
   // println!("{}",r1);
}
// Println commented
```

11

```
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    // let r2 = &mut s;

    r1.push_str("world");
}
// r1.push str added ()

```