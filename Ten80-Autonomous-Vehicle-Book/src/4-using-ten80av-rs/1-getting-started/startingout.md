# Getting Started with ten80av-rs 

This part of the guide assumes that you have some experience in rust. If you are completely unfamiliar with rust then please refer to chapter 2 of this book.

## Creating your rust project

Before we can get started writing any code we have to create the project (or crate) that will hold said code. to do this open your favorite terminal in the folder you wish to craete your project in and enter the following commands

```not_rust
$ cargo new ten80-av
$ cd ten80-av
```

after you run those commands, if you are using vs code then type
```not_rust
$ code . 
```

to open your project in vs code


## installing dependencies 

### Using the command line

with your terminal open in the folder that has your rust project run the following command to install the ten80av_rs library and another library that you will need to properly use ten80av_rs 
```not_rust 
$ cargo add ten80av-rs
$ cargo add microbit-v2
$ cargo add cortex-m-rt
```

### Using `Cargo.toml`

Open your `Cargo.toml` inside of a text editor and copy the following lines under "[dependencies]"

```toml
ten80av-rs = "1.0.0"
microbit-v2 = "0.16.0"
cortex-m-rt = "0.7.5"
```