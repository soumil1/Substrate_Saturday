Binding and mutability


1
```
fn main() {
    let x: i32 = 5; 
    let y: i32; 

    assert_eq!(x, 5);
    println!("Success!");
}

// x is initialised
```

2
```
fn main() {
    let mut x =  1;
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}

// x is used twice, hence add mut
```

Scope

3
```
fn main() {
    let x: i32 = 10;
    
    {
        let y: i32 = 5;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {}", x); 
}

// Remove y from the last println
```

4
```
fn main() {
    let x = define_x();
    println!("{}, world", x); 
}

fn define_x() -> String{
    let x = "hello".to_string();
    x
}

// add to_string() and declare x in main fn
```

Shadowing

5
```
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x =  42;
    println!("{}", x); // Prints "42".
}

// modified the value in (x,_)
```

6
```
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x; 
    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

// remove unnecessary fields
```

7
```
fn main() {
    let x = 1;
    println!("{}",x)
}

// use variable in conjunction with println
```

Destructuring

8
```
fn main() {
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("Success!");
}

// mut in x is added
```

Destructuring assignments

9
```
fn main() {
    let (x, y);
    (x,..) = (3, 4);
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);
    println!("Success!");
} 

// add [3,2]

```