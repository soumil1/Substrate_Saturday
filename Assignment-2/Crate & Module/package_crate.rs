

1 Create a package with below layout:
```
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files

# in Cargo.toml
[package]
name = "hello-package"
version = "0.1.0"
edition = "2021"

Solution: cargo new hello-package
```

2 Create a package with below layout:
```
.
├── Cargo.toml
└── src
    └── lib.rs

1 directory, 2 files

# in Cargo.toml
[package]
name = "hello-package1"
version = "0.1.0"
edition = "2021"

Solution: cargo new --lib hello-package1
```

3 /* FILL in the blank with your ANSWER */
```
// Q: Whats the difference between package 1# and 2# ?
// A: __

Solution: hello-package has a binary crate named hello-package, src/main.rs is the crate root.
hello-pacakge1 has a library crate named hello-package1, src/lib.rs is the crate root.


4 /* FILL in the blank with your ANSWER */
```
// Q: Whats the name of the library crate in package `hello-package1`?
// A: __


Solution: hello-package1
```


5 Add a library crate for hello-package and describe it's files tree below:
```

# FILL in the blanks
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── __
│   └── __

Solution: # FILL in the blanks
.
├── Cargo.lock
├── Cargo.toml
├── src
│   ├── main.rs
│   └── lib.rs

```

6  A package can contain at most one library crate, but it can contain as many binary crates as you would like by 
    placing files in src/bin directory: each file will be a separate binary crate with the same name as the file.
```

Solution: # Create a package which contains 
# 1. three binary crates: `hello-package`, `main1` and `main2`
# 2. one library crate
# describe the directory tree below
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests # directory for integrated tests files
│   └── some_integration_tests.rs
├── benches # dir for benchmark files
│   └── simple_bench.rs
└── examples # dir for example files
    └── simple_example.rs

```