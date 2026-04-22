# Installing Rust 

The most common way to install rust is to use the Rustup command line tool. Rustup is a tool primarily used for managing rust versions and associated tools. Before continuing on, be sure that you're connected to the internet.

If any of this is unclear or you encounter any issues, refer to the installation section of the Rust Programming Language Book which you can read [here](https://doc.rust-lang.org/book/ch01-01-installation.html)

# For Windows 

1. Go to the rust Installation page [https://rust-lang.org/tools/install/](https://rust-lang.org/tools/install/)
2. Depending on the Architecture of your computer, click on one of the "Download Rustup-init.exe" Buttons
3. Run the exe you downloaded.
4. At some point in time you'll be asked to install Visual Studio. You are required to do this for rust to work. 
5. to be sure that you have rust installed correctly, open your terminal and enter this command:
    ```cmd
    $ rustc --version
    ```

# For MacOS

```rust
// refer to rust download page
// https://rust-lang.org/tools/install/
//
// or the chapter in the book about installation 
// https://doc.rust-lang.org/book/ch01-01-installation.html
todo!(":3");
```

# For Linux 

```rust
// refer to rust download page
// https://rust-lang.org/tools/install/
//
// or the chapter in the book about installation 
// https://doc.rust-lang.org/book/ch01-01-installation.html
todo!(":3");
```

# Setting Up Your Environment

> [!IMPORTANT]
> This part of the guide assumes you already have VS Code installed. If you do not have VS Code installed you can follow step one of [this guide](https://code.visualstudio.com/docs/setup/setup-overview)


## Installing Rust analyzer

Rust analyzer is a extremely useful tool for checking your rust code for bugs. In this step we're going to install it 

1. Open VSCode and open the Extensions View (```Ctrl+Shift+X```)
2. click on the search bar and type (or copy) this into the search bar 
```
rust-lang.rust-analyzer
```
3. Install the First extension to show up 

## Creating your first project folder with Cargo

before we start, you should create a folder for all of your rust projects then open that folder in your terminal.

Once you have your project folder open run the following commands

```cmd
$ cargo new ./my-first-rust-project --name my-first-rust-project
$ cd my-first-rust-project
$ code .
```

## Your project structure

When VScode opens you should see your project folder. The project folder will contain a couple files which will be explained in this section


```
my-first-rust-project
├── src
│   └── main.rs
├── target
│   ├── debug
│   ├── flycheck0
│   ├── .rustc_info_json
│   └── CACHEDIR.TAG
├── Cargo.lock
└── Cargo.toml
```

There are a couple files in your project but not a bunch of them are actually important. The following files are actually useful

*  `Cargo.toml` - This file has configuration information about your project
* `src` - this is the folder where all your code goes
    * `main.rs` - this is the main file that calls any code you write. This is the main file we're going to be working on 
* `target` - this folder has information that helps rust build your code and this folder also houses the distribution files that get built after you run `cargo build`

