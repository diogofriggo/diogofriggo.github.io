---
layout: post
title: Macros for beginners
date: 2022-05-11 10:41:00 -0300
categories: [macros]
tags:
- rust
- macros
---

## How to initialize a vector filled with a constant

We're looking for a concise way to get a vector of `n` elements, all initialized with the same specified `constant`

{% highlight rust %}
let vec: Vec<usize> = vec![1, 1, 1, 1, 1, ..., n];
{% endhighlight %}

The `vec!` macro is our friend here. This is as simple as:

{% highlight rust %}
let vec = vec![1usize; 5];
println!("{:?}", vec); // [1, 1, 1, 1, 1]
{% endhighlight %}

Before learning this trick I would do some unnecessary heavy lifting like

{% highlight rust %}
let vec = (0..5).map(|_| 1).collect::<Vec<usize>>();
println!("{:?}", vec); // [1, 1, 1, 1, 1]
{% endhighlight %}

But how does Rust do it? Let us try to reproduce it<br/>
Macro syntax is cryptic amd confusing so let's approach it step by step<br/>
The body of an empty macro looks like this

{% highlight rust %}
macro_rules! our_vec {
    (  ) => {

    };
}
{% endhighlight %}

That's an empty `match` arm with an empty pattern.<br/>
Let's add a pattern

{% highlight rust %}
macro_rules! our_vec {
    ( $constant:expr; $n:expr ) => {

    };
}
{% endhighlight %}

We now match on a pattern of two expressions separated by a semicolon.<br/>
We name them as `constant` and `n`<br/>
Let's now create a `Vec` with a capacity for `n` items

{% highlight rust %}
macro_rules! our_vec {
    ( $constant:expr; $n:expr ) => {
        { // expression block begins here
            let mut temp_vec = Vec::with_capacity($n);
            temp_vec
        } // expression block ends here
    };
}
{% endhighlight %}

Notice how we used the captured `$n` variable.<br/>
Rust is an expression based language and this holds true in macros as well.<br/>
That's why we created a `{}` block and put our code inside it. The last statement of the block is the return value so
this means we are returning `temp_vec`<br/>
This vector however does not hold any element yet. Let's add one

{% highlight rust %}
macro_rules! our_vec {
    ( $constant:expr; $n:expr ) => {
        {
            let mut temp_vec = Vec::with_capacity($n);
            temp_vec.push($constant)
            temp_vec
        }
    };
}
{% endhighlight %}

At this point let's check what our macro gives us

{% highlight rust %}
let vec = our_vec![1; 5];
println!("{:?}", vec); // [1]
{% endhighlight %}

Neat! We're getting a `Vec` with one element in it. Let's turn that into `n` elements

{% highlight rust %}
macro_rules! our_vec {
    ( $constant:expr; $n:expr ) => {
        {
            let mut temp_vec = Vec::with_capacity($n);
            (0..$n).for_each(|_| temp_vec.push($constant));
            temp_vec
        }
    };
}
{% endhighlight %}

{% highlight rust %}
let vec = our_vec![1; 5];
println!("{:?}", vec); // [1, 1, 1, 1, 1]
{% endhighlight %}

Awesome, it works! Our macro converted the above code into this

{% highlight rust %}
let vec = {
    let mut temp_vec = Vec::with_capacity(5);
    (0..5).for_each(|_| temp_vec.push(1));
    temp_vec
};
println!("{:?}", vec); // [1, 1, 1, 1, 1]
{% endhighlight %}

Rust is open-source so let's [peek at how they do it](https://doc.rust-lang.org/src/alloc/macros.rs.html#42-52) and compare with our approach

{% highlight rust %}
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
{% endhighlight %}

We can see that they've got more match arms for the other uses of the `vec!` macro, we're focusing only on the second branch.<br/>
We can safely ignore the `__rust_force_expr` macro since it only serves the [purpose of improving error messages](https://stackoverflow.com/questions/70402502/what-exactly-does-rust-force-expr-do)<br/>
The core behaviour is within the `vec::from_elem` function

{% highlight rust %}
pub fn from_elem_in<T: Clone, A: Allocator>(elem: T, n: usize, alloc: A) -> Vec<T, A> {
    <T as SpecFromElem>::from_elem(elem, n, alloc)
}
{% endhighlight %}

Hm, the call is being delegated to `SpecFromElem::from_elem`, what does that do?

{% highlight rust %}
fn from_elem<A: Allocator>(elem: i8, n: usize, alloc: A) -> Vec<i8, A> {
    if elem == 0 {
        return Vec { buf: RawVec::with_capacity_zeroed_in(n, alloc), len: n };
    }
    unsafe {
        let mut v = Vec::with_capacity_in(n, alloc);
        ptr::write_bytes(v.as_mut_ptr(), elem as u8, n);
        v.set_len(n);
        v
    }
}
{% endhighlight %}

Finally, there it is.<br/>
If `elem` (we named it `constant`) is the constant `0`, rust takes a performant shortcut: initializes and returns `Vec` with capacity `n` filled with zeros<br/>

If `elem` is anything else Rust resorts to an unsafe block to be able to write bytes directly with our provided `constant`, then sets `vec`'s `len` and returns it<br/>

This is foundational code used very frequently by any code base, so it was to be expected that Rust would not use our loop approach. It must be fast!<br/>
Rust had to resort to pointers and the **Dark** arts of unsafe code to achieve that performance gain.<br/>

For the purposes of this post you can regard `alloc` as an internal implementation detail, a topic not frequently encountered in everyday code.<br/>
Bonus tip: just like Rust's `vec!` our `our_vec!` macro can be nested, to produce Vec of Vecs!<br/>

{% highlight rust %}
let vec_of_vecs = our_vec![our_vec![1usize; 5]; 2];
println!("{:?}", vec_of_vecs); // [[1, 1, 1, 1, 1], [1, 1, 1, 1, 1]]
{% endhighlight %}

Here's the full code we built for you to try

{% highlight rust %}
macro_rules! our_vec {
    ( $constant:expr; $n:expr ) => {
        {
            let mut temp_vec = Vec::with_capacity($n);
            (0..$n).for_each(|_| temp_vec.push($constant));
            temp_vec
        }
    };
}

fn main() {
    let vec = vec![1usize; 5];
    println!("{:?}", vec); // [1, 1, 1, 1, 1]

    let vec = (0..5).map(|_| 1).collect::<Vec<usize>>();
    println!("{:?}", vec); // [1, 1, 1, 1, 1]

    let vec: Vec<usize> = our_vec![1; 5];
    println!("{:?}", vec); // [1, 1, 1, 1, 1]

    let vec_of_vecs = our_vec![our_vec![1usize; 5]; 2];
    println!("{:?}", vec_of_vecs); // [[1, 1, 1, 1, 1], [1, 1, 1, 1, 1]]
}
{% endhighlight %}

Macros are not an easy topic in Rust, they can feel quite alien until you get the hang of it,
so I hope this has been clear enough to be useful and has encouraged you to write your own macros!