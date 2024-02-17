---
title: Switching to Rust as a Python developer
author: luciano
date: 2024-02-18 12:30:00 +0200
categories: [Coding , Blog]
tags: [Rust , Python]
toc: true
image:
    path: /assets/img/python-ferries.jpeg
    alt: I did not make this - Rust's mascot is a crab
---


Learning Rust for the past months has been one best learning experiences I have had since learning Python. If you have to pick a new language to learn in 2024, I would definitely recommend Rust! 

Here are the top features that I particularly enjoy as a Python developer! :)

## Why Rust?
You may want to know why I wanted to learn Rust out of the many other languages out there. First, I wanted to pick a lower-level language as I think it would nicely complete with my current knowledge of Python. By learning a lower-level language I can familiarize myself more with hardware resources and architectures as well as learning how to optimize for performance. As a Python developer, I kept looking more and more at rust-dependent code such as [Polars](https://pola.rs/), [Pydantic](https://docs.pydantic.dev/latest/) and more. 

## Static typing 
Although I really like the dynamic nature of Python, I have been starting to appreciate the comfort of having typehints in the code I write. Typehints improve code readability and blends with most code editors and **lsps**. 

Writing typehints in my Python code has actually saved me time! Every time I am using the wrong parameter for a function or when looking for a bug, I can always keep an eye for the warnings that wouldn't normally be there if I hadn't put typehints in the first place! 

All of this makes me code with more ease. Moving to a statistically typed language is exciting and brings me even more comfort because I know there is little room for silly errors. Which brings me to the next amazing feature Rust has.

## The compiler
I love how the compiler is the best code companion in Rust. After compilation, I am  comfortable that there are not going to be any major issues since most of them were caught by the compiler. Rust makes sure that  your script is safe to run. And if any errors are raise during compilation, these are presented in a very clear way and it even provide helpful solutions.
As I learn my way through Rust, I often learn things just by compiling my scripts ! 

## Portability
Python is a great language which offers a lot of flexibility on what to develop and how. But sharing your Python app is sometimes difficult. You always need to consider the environment in which Python needs to be interpreted and run. Moving to a compiled language really made me discover the advantage of having a single binary file that can be executed in different machines with very few difficulties. 

## Cargo
I tried various library mangers in Python. Although I mainly use [anaconda](https://anaconda.com) for my day-to-day development which suits most of my cases, I sometimes will  use [poetry](https://python-poetry.org) and I even start experimenting with [rye](https://rye-up.com). 
But still, to this day, managing environments and packages in Python can be very difficult, especially because there could be many different ways to pack a library and each library will bring its own sometimes messy requirements. Some popular libraries are very easy to install in any Os but others can be very cumbersome! Cargo brings serenity to all of this. You can easily add libraries to your Rust project, compile it and run it without having to do any manual work. Learning Rust has been particularly pleasant for me especially because of [Cargo](https://doc.rust-lang.org/stable/cargo/).

## Safety
More specifically Null safety and error handling. Yes - not necessarily memory safety since with Python I never really had to deal with memory leaks by leaving the garbage collector do its job. In Rust, there two main **Enums** that makes handling null values and errors very trivial. These are `Option` and `Result` respectively.

In Python, you can convey that a function might return a value or `None` with the `Option` typehint or, more recently, with the notation `| None`. But this still leave room for the usuals `ValueError` or `AttributeError` and leave the optional freedom to programer to handle null cases. 

Consider the following example in Python:

```python
import random

def foo() -> str | None:
    if random.randint(0,10) == 1:
        return "Hello"

def bar(s:str) -> str:
    return s.upper()

random_val = foo()
assert bar(random_val) in ("HELLO","") 
```

Now,this is a very basic examples! There are chances for this program to run successfully but let's see why it may not. 

The function `bar` is expecting a value of type `str`, but since the function `foo` may return a string or `None`, line `8` will raise an `AttributeError` since `None` does not have the method `.upper`. The rest of the code really won't be run until `foo` returns `"Hello"`. 

We can make this code *safer* with one extra line of code:

```python
import random

def foo() -> str | None:
    if random.randint(0,10) == 1:
        return "Hello"

def bar(s:str | None) -> str:
    if isinstance(s,str):
        return s.upper()
    return ""

random_val = foo()
assert bar(random_val) in ("HELLO","") 
```

I love the approach that Rusts takes to handle this case using `Option`. 
Here is how the previous example would look like written in Rust:

*Unfortunately*, we need to add a package to address the random behavior of the `foo` function as Rust does not have it to the standard library. 

Here I am using Cargo to add the package *rand*.

```bash
cargo add rand
```


```rust
use rand::{thread_rng, Rng};

fn foo() -> Option<String> {
    let mut rng = thread_rng();

    if rng.gen_range(0..10) == 1 {
        Some(String::from("Hello"))
    } else {
        None
    }
}

fn bar(s: Option<String>) -> String {
    s.unwrap_or(String::from("")).to_uppercase()
}

fn main() {
    let random_val = foo();
    let res = vec!["HELLO".to_string(), "".to_string()];
    assert!(res.contains(&bar(random_val)))
}
```
I did my best to replicate the example in Rust, I am sure a more seasoned Rust dev can do a better job.
With this code, `foo` uses the `Option` and it's variant `Some` which literally means that there is a value, and `None` -  no value.

This makes so that any function that uses the output of `foo` would need to handle each variant of the Enum `Option`. The same would be true for the `Result` enum which has `Ok` and `Err` as a variant. 

This add the great advantage to explicitly force the user to handle cases in which a function or method may return no value or raise an error. This functionality is what I always miss when I go back to Python.


## Final thoughts - Is it actually that good?

I spent quite some words describing how *good* Rust is and how *bad* Python can get. But there is a whole new post that can be made on how good Python is compared to Rust, in terms of its rapid prototyping, learning curves, huge community, and much more! I won't stop using Python as I think it is a fundamental language for data analytics but I am ready to start experimenting with Rust for tools that would require more control and performance.

With this post, I  wanted to highlight my very first reactions in learning a lower level language coming from Python. I am sure some might agree with the points I have highlighted, but I also know that some may not and that's why I would love to see others's points of view and reactions.