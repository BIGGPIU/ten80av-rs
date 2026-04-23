# Introduction

In this book you will learn how to program your own library to power your Autonomous Vehicle provided by Ten80 using Rust. This book was written to be useful for people with some non embedded rust experience as well as people who have no rust experience at all.

> [!IMPORTANT]
> This book assumes you are using the Micro:Bit V2 and the Driver Expansion Board

## What you will learn

In this book we will go over:

* The basics of Rust

* The basics of Embedded Develpoment using Rust

* How to write rust code that works in no standard library (no_std) environments

## What you will *NOT* learn

This book will not go over things like:

* The absolute basics of programming. This book assumes that you have some experience in programming in languages aside from rust.

* How to write Generic code that works on microcontrollers other than the Micro:Bit

* Multitasking & Interrupts

* Circuits and Electronics. This book will only go into enough for you to understand how to program your AV

* How to create programs to complete competition challenges

* How to use rust in non embedded environments

## Why you should use rust for your Autonomous Vehicle

* **Rust is Decentralized** - If the Micro:Bit website were to shut down tomorrow your JavaScript or MakeCode would stop working. If you were to write your entire project in rust then no matter what happens to the people who make the Micro:Bit, your code will still be able to be used.

* **Rust is Used In Real Life Embedded Programming** - Rust is becoming more and more popular among companies as the programming language of choice for embedded systems. Companies like Google, Microsoft, Samsung and Volvo are beginning to use Rust ([Source](https://onevariable.com/blog/embedded-rust-production/)). Getting rust experience early can potentially help you out in the long run.

* **Rust is fun** - This is subjective but developing for rust is a ton of fun. You get to look at all sorts of datasheets and watch a library you wrote on your computer interact with the real world.

## Other useful Resources

* [Discovery Rust book](https://docs.rust-embedded.org/discovery/microbit/)

## Reporting Problems

The source for this book is in the [ten80av_rs github repository](https://github.com/BIGGPIU/ten80av-rs). Please create a pull request if you notice a typo or something that is not gramatically correct.