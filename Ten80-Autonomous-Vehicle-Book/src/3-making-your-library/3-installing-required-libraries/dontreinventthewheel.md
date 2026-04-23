# Installing the Required Libraries

## Installing the libraries from the command line

There are a couple libraries that are integral to making your library work and they will be used in the other parts of the chapter. To make everything flow better we're just going to install all of the libraries we're going to need right now. Open your library folder in your terminal and run the following commands:

```no_copy
$ cargo add embedded-hal
$ cargo add embedded-io
$ cargo add lsm303agr
$ cargo add microbit-v2
$ cargo add pwm-pca9685
```

## Installing the libraries by editing your `Cargo.toml`

Open your `Cargo.toml` file in your code editor and copy the following lines under "\[dependencies\]" 

```toml
embedded-hal = "1.0.0"
embedded-io = "0.7.1"
lsm303agr = "1.1.0"
microbit-v2 = "0.16.0"
pwm-pca9685 = "1.0.0"
```

when you're done your `Cargo.toml` should look like this:
```toml
[package]
name = "ten80-av-library"
version = "0.1.0"
edition = "2024"

[dependencies]
embedded-hal = "1.0.0"
embedded-io = "0.7.1"
lsm303agr = "1.1.0"
microbit-v2 = "0.16.0"
pwm-pca9685 = "1.0.0"
```