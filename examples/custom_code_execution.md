---
title: Custom Code Execution
evaluators:
  - id: python
    args:
      - python
  - id: node
    args:
      - node
  - id: cargo
    args:
      - cargo-script
      - script
---

The evaluator defaults to shell if no evaluator ID is specified.

```bash +exec
for i in $(seq 1 5)
do
    echo "hi $i"
    sleep 0.5
done
```

<!-- end_slide -->

```python +exec:python
print(5)
```

<!-- end_slide -->

```javascript +exec:node
console.log("hi from node");
```

<!-- end_slide -->

```rust +exec:cargo
fn main() {
  println!("hello world!");
}
```
