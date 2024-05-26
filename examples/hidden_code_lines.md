---
title: Hidden Code Lines
---

```rust +exec
/// cat > hello.rs << EOF
fn main() {
    println!("Hello from Rust!");
}
/// EOF
/// rustc hello.rs
/// ./hello
/// rm hello.rs
/// rm hello
```

<!-- end_slide -->

```rust +line_numbers
let foo = 2;
let bar = 2;
let sum = foo + bar;
println!("The sum is: {}", sum);
```

```rust +line_numbers
/// fn main() {
let foo = 2;
let bar = 2;
let sum = foo + bar;
println!("The sum is: {}", sum);
/// }
```

<!-- end_slide -->

```rust {1,3,4,7,9-11} +line_numbers
   fn potato() -> u32 {

    println!("Hello world");
    let mut q = 42;
    q = q * 1337;
    q = q * 1337;
    q = q * 1337;
    q = q * 1337;
    q = q * 1337;
    q = q * 1337;
    q = q * 1337;
    q = q * 1337;
   }
```

```rust {1,3,4,7,9-11} +line_numbers
///    fn potato() -> u32 {
println!("Hello world");
let mut q = 42;
q = q * 1337;
q = q * 1337;
q = q * 1337;
q = q * 1337;
q = q * 1337;
q = q * 1337;
q = q * 1337;
q = q * 1337;
q = q * 1337;
///    }
```

<!-- end_slide -->

```rust {1|2|3|4}
///   fn potato() -> u32 {
println!("Hello world");
let mut q = 42;
q = q * 1337;
500
///   }
```
