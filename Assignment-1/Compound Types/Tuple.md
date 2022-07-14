


1
```
fn main() {
    let _t0: (u8,i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
    println!("Success!");
}
// Types have been added to round out the code.

```

2
```
fn main() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");
    println!("Success!");
}
// t.1 is changed to t.2.
```

3
```
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}
// 13 was removed because a tuple can only hold 12 values.
```

4
```
fn main() {
    let tup = (1, 6.4, "hello");

    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

     println!("Success!");
}
// Added (x, z, y) to complete tupe order
```

5
```
fn main() {
    let (x, y, z);

    // fill the blank
    (y, z, x) = (1, 2, 3);
    
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}
// (y, z, x) were added to complete the tupe order.
```

6
```
fn main() {
    let (x, y) = sum_multiply((2, 3));
 
    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
 }
 
 fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
     (nums.0 + nums.1, nums.0 * nums.1)
 }
// (2) and (3) were added to complete the sentence (x,y)

```
