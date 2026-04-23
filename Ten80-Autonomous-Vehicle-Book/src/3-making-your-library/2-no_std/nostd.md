# What is a no_std

## Why do we need no_std

When you compile a program with rust it makes a couple assumptions. It assumes that whatever you're compiling for will have an operating system and it will be able to benefit from all of the benefits of having an operating system. 

The Micro:bit v2 doesnt have the liberty of having an operating system though. This is one of the cases where we're going to want to use no_std

no_std throws all of that out the window. Like the name implies it prevents you from using functions from the standard library (libstd) but rather it only lets you use functions from the core library (libcore)

There are not a whole lot of places in this guide where you would be tempted to reach for a function in the standard library but its important to go over the traits of programming in a no_std environment to make it easier to implement any new features you may desire to the library.

## Making Your Library a no_std environment 

Inside of your `lib.rs` add this line to the top of your file
```rust
#![no_std]
```