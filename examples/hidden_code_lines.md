---
title: Hidden Code Lines
---

You can use the delimiter `"/// "` for lines of shell code
that will not be visible in the code snippet in the presentation,
but will still be executed.

Here is an example for compiling and running a `java` program:

```bash +exec
/// cat > HelloWorld.java << EOF
public class HelloWorld {
    public static void main(String[] args) {
        System.out.println("Hello from Java!");
    }
}
/// EOF
/// javac HelloWorld.java
/// java HelloWorld
/// rm HelloWorld.java
/// rm HelloWorld.class
```

<!-- end_slide -->

You could even omit the enclosing class and method completely:

```bash +exec
/// cat > HelloWorld.java << EOF
/// public class HelloWorld {
/// public static void main(String[] args) {
System.out.println("Hello from Java!");
/// }
/// }
/// EOF
/// javac HelloWorld.java
/// java HelloWorld
/// rm HelloWorld.java
/// rm HelloWorld.class
```

<!-- end_slide -->

An example for `Rust`:

```bash +exec
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

An example for `python`:

```bash +exec
/// python -c """
print('Hello from Python!')
/// """
```
