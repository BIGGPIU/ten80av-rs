# Creating your library folder

## Creating your Library folder with Cargo

Before we start, you should have a folder for all of your rust projects then open that folder in your terminal.

Once you have your project folder open run the following commands

```cmd
$ cargo new --lib ten80-av-library
$ cd ten80-av-library
$ code . 
```

## Your project structure 

When VSCode opens you should be able to see your library folder. this folder looks relatively similar to what you would get if you ran cargo new without the `--lib` argument. there are a few key differences though

```
ten80-av-library
├── src
│   └── lib.rs
├── target
│   ├── debug
│   ├── flycheck0
│   ├── .rustc_info_json
│   └── CACHEDIR.TAG
├── Cargo.lock
└── Cargo.toml
```

The main difference is **lib.rs** this file is kind of like `main.rs` but instead of it holding all of your code it instead holds the modules for your library as well as some introductory documentation for your library.

**Before moving on to the next part of this chapter: please go into lib.rs and delete all of the placeholder code that is in the file.**