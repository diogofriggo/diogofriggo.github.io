# How to initialize a vector filled with a constant

The `vec!` macro is your friend here. This is as simple as:

```rust
let vec = vec![1usize; 5];
```

Before learning this trick I would do some unnecessary heavy lifting like

```rust
let vec = (0..5).map(|_| 0).collect::<Vec<usize>>();
```