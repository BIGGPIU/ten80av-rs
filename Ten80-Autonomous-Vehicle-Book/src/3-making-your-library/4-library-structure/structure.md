# Creating Your Library Strucutre

## Moving your devices from the root of the folder

To make your Library easier to navigate for other users or future you we're going to split the library into multiple different parts. inside of your `src` file create a folder called `devices` then add a file called `mod.rs` to that folder.

This is the folder where we're going to use to interact with the external and internal devices on the micro:bit. To make use of this folder though, we have to link it to `lib.rs` open `lib.rs` and insert the following line:

```rust
pub mod devices;
```

When you're done with this step your folder should look something like this 
```
ten80-av-library
├── src
│   ├── lib.rs
│   └── devices
│       └── mod.rs
├── target
│   ├── debug
│   ├── flycheck0
│   ├── .rustc_info_json
│   └── CACHEDIR.TAG
├── Cargo.lock
└── Cargo.toml

```


## Optional: Separating your internal and external devices
This is marked as optional because technically you dont *need* to separate your external and internal sensors but it does make your library slightly more clear. 

Inside of your `devices` folder create two folders. `external` and `internal`. Create a `mod.rs` file in both of these folders. Once you are done with that open the `mod.rs` file inside of `devices` and add the following lines:

```rust 
pub mod external;
pub mod internal;
```

If you decide to do this then your folder should look something like this 
```
ten80-av-library
├── src
│   ├── lib.rs
│   └── devices
│       ├── mod.rs
│       ├── external
│       │   └── mod.rs
│       └── internal
│           └── mod.rs
├── target
│   ├── debug
│   ├── flycheck0
│   ├── .rustc_info_json
│   └── CACHEDIR.TAG
├── Cargo.lock
└── Cargo.toml
```

> [!WARNING]
> This guide will not explicitly tell you if each file should be placed in `external` or `internal` I advise you use common sense to tell where each component should be placed