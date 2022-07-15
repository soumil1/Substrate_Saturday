


1
```
fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world";
    println!("Success!");
}
// Added and placed in s1
```

2
```
fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will passed: each of the two UTF-8 chars '中' and '国'  occupies 3 bytes, 2 * 3 = 6
    assert!(std::mem::size_of_val(&slice) == 16);
    println!("Success!");
}
// size of val is 16
```

3
```
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
    println!("Success!");
}
// &[i32] = &arr[1..4]; new slice
```

String slices

4
```
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);
    println!("Success!");
}
// Added index value ..2 in slice2
```

5
```
fn main() {
    let s = "你好，世界";
    let slice = &s[0..3];

    assert!(slice == "你");
    println!("Success!");
}
// In slice, index value..3 was replaced.
```

6
```
fn main() {
    let mut s = String::from("hello world");

    // here, &s is `&String` type, but `first_word` need a `&str` type.
    // it works because `&String` can be implicitly converted to `&str, If you want know more ,this is called `Deref` 
    let word = first_word(&s);

    println!("the first word is: {}", word);

    s.clear();
}

fn first_word(s: &str) -> &str {
    &s[..1]
}
// after printing the word, s.clear() is called

```