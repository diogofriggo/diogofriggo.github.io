# How to initialize a vector filled with a constant (macro for beginners)

The `vec!` macro is your friend here. This is as simple as:

```rust
let vec = vec![1usize; 5];
println!("{:?}", vec); // [1, 1, 1, 1, 1]
```

Before learning this trick I would do some unnecessary heavy lifting like

```rust
let vec = (0..5).map(|_| 0).collect::<Vec<usize>>();
println!("{:?}", vec); // [1, 1, 1, 1, 1]
```

But how is it done? Let us try to reproduce it

Macro syntax is cryptic amd confusing so let's approach it step by step
The body of an empty macro looks like this

```rust
macro_rules! my_vec {
    (  ) => {

    };
}
```

That's an empty `match` arm with an empty pattern.

Let's add a pattern

```rust
macro_rules! my_vec {
    ( $constant:expr; $n:expr ) => {

    };
}
```

We now match on a pattern of two expressions separated by a semicolon.
We name them as `constant` and `n`

Let's now create a `Vec` with a capacity for `n` items

```rust
macro_rules! my_vec {
    ( $constant:expr; $n:expr ) => {
        {
            let mut temp_vec = Vec::with_capacity($n);
            temp_vec
        }
    };
}
```

Notice how we used the captured `$n` variable.

Rust is an expression based language and this holds true in macros as well.
That's why we created a `{}` block and put our code inside it. The last statement of the block is the return value so
this means we are returning `temp_vec`

This vector however does not hold any element yet. Let's add one

```rust
macro_rules! my_vec {
    ( $constant:expr; $n:expr ) => {
        {
            let mut temp_vec = Vec::with_capacity($n);
            temp_vec.push($constant)
            temp_vec
        }
    };
}
```

At this point let's check what our macro gives us

```rust
let vec = my_vec![1; 5];
println!("{:?}", vec); // [1]
```

Neat! We're getting a vec with one element in it. Let's turn that into `n` elements

```rust
macro_rules! my_vec {
    ( $constant:expr; $n:expr ) => {
        {
            let mut temp_vec = Vec::with_capacity($n);
            (0..$n).for_each(|_| temp_vec.push($constant));
            temp_vec
        }
    };
}
```

```rust
let vec = my_vec![1; 5];
println!("{:?}", vec); // [1, 1, 1, 1, 1]
```

Awesome, it works! Our macro converted the above code into this

```rust
let vec = {
    let mut temp_vec = Vec::with_capacity(5);
    (0..5).for_each(|_| temp_vec.push($constant));
    temp_vec
};
println!("{:?}", vec); // [1, 1, 1, 1, 1]
```

Rust is open-source so let's peek at how they do it and compare with our approach

```rust
macro_rules! vec {
    () => (
        $crate::__rust_force_expr!($crate::vec::Vec::new())
    );
    ($elem:expr; $n:expr) => (
        $crate::__rust_force_expr!($crate::vec::from_elem($elem, $n))
    );
    ($($x:expr),+ $(,)?) => (
        $crate::__rust_force_expr!(<[_]>::into_vec(box [$($x),+]))
    );
}
```

You can see that they've got more match arms for the other uses of the `vec!` macro, we're focusing only on the second branch.


