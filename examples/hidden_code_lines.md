---
title: Hidden Code Lines
---

Compile and execute Rust code using hidden code lines:

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

# Line Numbering

This is how the line numbers render without any hidden code lines:

```rust +line_numbers
let foo = 2;
let bar = 2;
let sum = foo + bar;
println!("The sum is: {}", sum);
```

Observe that the line numbering in the following snippet matches the above. The following snippet has a hidden enclosed main function:

```rust +line_numbers
/// fn main() {
let foo = 2;
let bar = 2;
let sum = foo + bar;
println!("The sum is: {}", sum);
/// }
```

<!-- end_slide -->

# Selective Highlighting

The following code snippet has a hidden enclosing main function. The lines `{1,3,4,7,9-11}` should be highlighted, which match the visible code line numbers:

```rust {1,3,4,7,9-11} +line_numbers
/// fn main() {
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
/// }
```

<!-- end_slide -->

# Dynamic Highlighting

Dynamic highlighting also respects the hidden/visible code lines. Here we highlight each line on each consectuive slide, in turn:

```rust {1|2|3|4}
/// fn main() {
println!("Hello world");
let mut q = 42;
q = q * 1337;
/// hidden
500;
/// }
```
