Ownership


1
```
fn main() {
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);
}
// The clone() method is used
```

2
// Don't modify code in main!
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// Only modify the code below!
fn take_ownership(s: String) {
    println!("{}", s);
} 

Solution 
```
fn main() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

fn take_ownership(s: String)-> String {
    println!("{}", s);
    s
}
// fn take_ownership, add a string method
```

3
```
fn main() {
    let s = give_ownership();
    println!("{}", s);
}

// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("hello, world"); 
    s
}

```

4
```
fn main() {
    let s = String::from("hello, world");

    print_str(s.clone());

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}",s)
}
// The clone() method is used
```

5
```
fn main() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}
// To obtain output, changed the y=x line
```

Mutability

6
```
fn main() {
    let s = String::from("hello, ");
    
    let mut s1 = s;

    s1.push_str("world");

    println!("Success!");
}
// added mut in s1 = s
```

7
fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(3);   
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}
// Box::new added mut y (3)
```

Partial Move

8
```
fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0;

   println!("{:?}", t.1);
}
// inserted t.1 into print to print the second half of t
```

9
```
fn main() {
   let t = (String::from("hello"), String::from("world"));

    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); 
}
// Let (s1, s2) = t.clone was added ()

```