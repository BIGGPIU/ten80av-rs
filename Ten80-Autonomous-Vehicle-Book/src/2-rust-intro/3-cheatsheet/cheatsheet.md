# Rust Cheetsheet

>[!NOTE]
> if a code box starts with $ it is meant to be ran in your command line

## Variables

### Initialize a immutable variable

```
let <variable_name>:<type>;
```
or
```
let <variable_name>:<type is implied from object> = <object>;
```
### Initialize a mutable variable
```
let <variable_name>:<type>;
```
or
```
let <variable_name>:<type is inferred from object> = <object>;
```
## File management
### Create a new rust project
```
$ cargo new
```
### Install a new library from crates.io
``` 
$ cargo add <crate_name>
```
### Link a new file to your main file
```
mod <file_name>;
```
### use a function, struct, etc from an external file
this works for extenral files you created as well as libraries
```
use <crate name>::<crate module>
```
or, to import everything
```
use <crate name>::*
```
## Objects
### Create a struct that is available in other files
```
pub struct <Name> {
    <visibility> <variable name>:<type>,
    <visibility> <variable name>:<type>,
}
```

example:
``` 
pub struct name {
    pub var_1:u32,
    var_2:isize
}
```
### Create a struct that is only available in one file
```
struct <Name> {
    <visibility> <variable name>:<type>,
    <visibility> <variable name>:<type>,
}
```

example:
``` 
struct name {
    pub var_1:u32,
    var_2:isize
}
```
### Associate a group of Functions with an object
```
impl <struct name> {

}
```
### Create a function that can be called from external files
```
impl <struct name> {
    pub fn <function_name>(arg_1:<type>,arg_2:<type>) -> <return _type> {

        return <return_type> 
        OR
        <variable with return type>
    } 
}
```

example:
```rust,no_copy,no_run
impl SampleStruct {
    pub fn function(arg1:u32,arg2:u32) -> u32 {
        return arg1 + arg2
    }
    // OR
    pub fn function2(arg1:u32,arg2:u32) -> u32 {
        arg1 + arg2
    }
}
```
### Create a function that cannot be called from extenral files
```
impl <struct name> {
    fn <function_name>(arg_1:<type>,arg_2:<type>) -> <return_type> {

        return <return_type> 
        OR
        <variable with return type>
    } 
}
```

example:
```rust,no_copy,no_run
impl SampleStruct {
    fn function(arg1:u32,arg2:u32) -> u32 {
        return arg1 + arg2
    }
    // OR
    fn function2(arg1:u32,arg2:u32) -> u32 {
        arg1 + arg2
    }
}
```

## Loops 
### For loops
```rust 
fn main() {
    let x:[u8;4] = [4,5,2,3];

    for i in x {
        println!("{x:?}")
    }
    // prints
    // > 4
    // > 5 
    // > 2 
    // > 3 
}
```
### While loops 
```rust
fn main() {
    let mut x = 0;

    while x != 5 {
        println!("{x:?}");
        x+=1;
    }
    // print
    // > 0  
    // > 1 
    // > 2 
    // > 3 
    // > 4 
}
```
### While true loops 
```rust
fn main() {
    let mut x = 0;
    loop {
        if x == 5 {
            break;
        }

        println!("{x:?}");
        x+=1;
    }

    // prints
    // > 0 
    // > 1
    // > 2 
    // > 3
    // > 4
}
```


## Conditionals 
### If Else statements
```rust
# #[derive(PartialEq)]
pub enum SuperBool {
    TRUE,
    FALSE,
    UNSURE,
}
fn main() {
    let x = SuperBool::TRUE;

    if x == SuperBool::TRUE {
        println!("Superbool is true");
    }
    else if x == SuperBool::FALSE {
        println!("Superbool is false");
    }
    else {
        println!("Superbool is... unsure");
    }
} 
```
### Match statements 
```rust
# #[derive(PartialEq)]
pub enum SuperBool {
    TRUE,
    FALSE,
    UNSURE,
}
fn main() {
    let x = SuperBool::TRUE;

    match x {
        SuperBool::TRUE => {
            println!("Superbool is true");
        },
        SuperBool::FALSE => {
            println!("Superbool is false");
        },
        SuperBool::UNSURE => {
            println!("Superbool is... unsure");
        }

    }
} 
```