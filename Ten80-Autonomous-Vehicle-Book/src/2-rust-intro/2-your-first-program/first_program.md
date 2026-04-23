# Your first program

## Hello, World

For our first program we're going to create a program that gets two random numbers, adds them together and then prints out the result.

Navigate to your projects `main.rs` and open it you'll be greeted with something like this:


```rust
fn main() {
    println!("Hello, world!");
}
```

## Declaring Variables

this is your simple hello world program in rust. this is also where we're going to be doing most of our programming for now. 

For now, lets scrap our boilerplate code and start writing our own. right now we dont have a way to get all our random numbers so lets just declare the variables we're going to use to store our random numbers once we write the code to get the random numbers.

The syntax for declaring variables in rust might remind you a bit of javascript. It looks like this:

`let variable_name;`

> [!IMPORTANT]
> always remember to add a "`;`" at the end of all your lines

> [!IMPORTANT]
> Variables in rust are **Immutable** by default meaning that they cannot be changed after they are initialized. If you wish to have a variable that can change after it is initialized you can do so by adding `mut` to your variable declaration. It would look like this: `let mut variable_name;`

so once we declare our two variables your program should now look like this:

we will also want to assign our variables a value so our program compiles. This can be any value you want.

```rust,no_run
fn main() {
    let random_number_one = 0;
    let random_number_two = 0;
}
```



You may notice that right next to your variables theres some text that you didnt add that says `i32`. this is your variables **Type** rust is a strongly typed language which means that for your code to compile: The compiler needs to know exactly what type everything is before it compiles everything, and it also demands that once you say a variable is one type you cant reassign it to another type

example:
```rust,compile_fail
fn main() {
    let x:usize;
    let y:i32 = 0;
    x = y
//  x has a type of usize and y has a type of i32 so we cant set x = y 
}
```
```rust
fn main() {
    let x;
    let y:i32 = 0;
    x = y;
// works because x is automatically assigned a type based on how its used.
}
```

you can run your program just to be sure it compiles but unfortunately right now it wont do anything. In the following step we will begin to add functionality to our program.

### Signed & Unsigned integers

You may be wondering what an "`i32`" or a "`usize`" is. for now all you need to know about those types is that they are both numbers. number types that start with **u** (like `u8`, `u16`,`usize`) are **unsigned integers** meaning they're integers that can only be positive. They're formatted as `u(number of bits taken up by integer)`. So for example: if you wanted to use an 8 bit integer you would use the type `u8`. Number types that start with **i** (like `i8`, `i16`,`isize`) are **signed integers** meaning they're integers that can be both positive and negative. Similarly to unsigned integers They're formatted as `i(number of bits taken up by integer)`. So for example: if you wanted to use an 8 bit integer you would use the type `i8`. Both of them have their own cases where they're useful so whenever you're assigning a number think about which one fits your use case

> [!NOTE]
> There is also another number type that starts with **f**: they are **floats**. they will not be covered in this guide but you should be aware of their existence 




## Adding External Libraries

Before continuing with the guide I want you to test something. Go into your program and type `random` and look at all the things vscode shows you. It may show you the variables you declared earlier as well as `hash::RandomState` (which is not relevant to what we are doing right now.) This might make you wonder "Where is the function for generating a random number?" there actually *isnt* a function for generating random numbers in rust's standard library which means we have to source our random functions from somewhere else. In this cause we're going to use the [rand library](https://crates.io/crates/rand). If we want to use the rand library though we must add it. 

Firstly, open your project folder inside of your terminal again

```
$ cd ./path/to/my-first-rust-project
```

now that we're in our project folder in our terminal we're going to use a useful tool that comes with our installation of rust: **Cargo**.

Cargo is the main tool in a rust users toolkit. It handles everything from running your program to even adding external libraries like what we're going to do now.

To add libraries to your project you can run the following command in your terminal 

```
$ cargo add <library_name>
```

so in this case we are going to run

```
$ cargo add rand
```

once you run that command cargo will install the rand library and add it to your project. Its such a convienient tool.


## Linking External Rust Files

now that we installed our library its time to use it. To make sure that our project is as clear as possible we're going to make another module for all of our random generation. 

Make a file in the same folder as `main.rs` called `random_numbers.rs` then open `random_numbers.rs` in vscode

You may notice that all of the code in vscode is grayed out. this is to show that the file is not linked to our main file. This begs the question: "How do you link the main file to random_numbers.rs?" 

There are two ways to do this: you can either use `CTRL + .` to open a suggestion menu and click `insert 'mod random_numbers;'` this suggestion adds `mod random_numbers` to `main.rs` which links the file to the project (**MAKE SURE TO SAVE**). The other way to do this is just by doing it manually. Navigate to your `main.rs` and simply insert the line `mod random_numbers;` manually.

for future reference, mod stands for module and it links one rust file to another rust file. the syntax for linking files is like this:

```
<visibility> mod <other_file_name>
```

## Using External Libraries

Now that we have our `random_numbers` file linked to our project lets start adding some code to our random_numbers file. first lets make an object called `RandomNumber` then give it a block that will soon hold all of our functions

```rust,no_run
pub struct RandomNumber {

}


impl RandomNumber {

}
```

`structs` in rust are roughly equivalent to objects or classes in other languages. they are formatted like so:
```rust,no_run
<visibility> struct <name> {
    <object variable name>:<type>,
    <object variable name>:<type>
}
```

structs in rust are also able to functions associated with them inside of `impl` blocks. `impl` blocks are formatted like this:
```rust,no_run
impl <struct_name> {

    <visibility> fn <name>(arg1:<arg_type>, arg2:<arg_type>) -> <return_type> {

    }

    <visibility> fn <name>{
        
    }
}
```

Before we start adding functionality to our object lets import the functions we're going to need from the rand library. change your `random_numbers.rs` so it looks something like this:
```rust,no_run
use rand::{Rng, rng};
use rand::prelude::ThreadRng;


pub struct RandomNumber {

}




impl RandomNumber {

}
```

the `use` keyword is used to use functions and objects from external libraries. "External libraries" doesnt always have to mean its a library you downloaded from the internet. An external library can be a file you created. to use our `random_numbers.rs` file as an example: if we wanted to use some of the code in this file inside of `main.rs` we could add the object to main.rs by adding `use crate::random_numbers::RandomNumber;` below the `mod random_numbers;`

now that we have our object and impl blocks and all the things from `rand` we're going to use lets finally start writing our functions

## Your first function

We're going to add three functions to our `RandomNumber` struct. 
```no_copy,rust
get_random_number()
add_random_number()
update_random_number()
```

first, lets make our get_random_number function. inside of our impl block we're going to add the outline for our get_random_number function

```rust
use rand::{Rng, rng};
use rand::prelude::ThreadRng;


pub struct RandomNumber {

}




impl RandomNumber {

    pub fn get_random_number() -> u32 {
        // get our rng generator
        let mut rng: ThreadRng = rng();
        
        // get a random 32 bit unsigned integer
        rng.next_u32()
    }

}
```

next lets add our `add_random_number` function

```rust 
use rand::{Rng, rng};
use rand::prelude::ThreadRng;


pub struct RandomNumber {

}




impl RandomNumber {

    pub fn get_random_number() -> u32 {
        let mut rng: ThreadRng = rng();
        
        rng.next_u32()
    }


    pub fn add_random_number(num_1:u32, num_2:u32) -> u32 {
        return num_1 + num_2;
    }
}
```
finally, we're going to add our `update_random_number` function
```rust
use rand::{Rng, rng};
use rand::prelude::ThreadRng;


pub struct RandomNumber {

}




impl RandomNumber {

    pub fn get_random_number() -> u32 {
        let mut rng: ThreadRng = rng();
        
        rng.next_u32()
    }


    pub fn add_random_number(num_1:u32, num_2:u32) -> u32 {
        return num_1 + num_2;
    }

    pub fn update_random_number(mut num_1:u32) {
        let mut rng:ThreadRng = rng();

        num_1 = rng.next_u32();
    }

}

```


### Returns In Rust

You may notice something when looking at the functions: `add_random_number` has ```return num_1 + num_2;``` while `get_random_number` doesnt have a semicolon at the end of the last line.

> [!WARNING]
> This section will mention concepts that are not discussed in this guide

rust gives developers two options for returning values which are useful in in different cases. Lets say for example we had a match statement (roughly equivalent to a switch statement in java) we could technically do it like this 
```rust
fn main() {
    let x:Option<u32> = Some(0);

    let y:u32;

    match x {
        Some(a) => {
            y = a;
        }
        None => {
            panic!("Couldnt find value");
        }
    }
    println!("{y:?}");
}
```  
but this leads to more code being written for something that could do the same thing:
```rust
fn main() {
    let x:Option<u32> = Some(0);

   

    let y = match x {
        Some(a) => {
            a
        }
        None => {
            panic!("Couldnt find value");
        }
    };
   println!("{y:?}");
}
```
this is one of the times where its useful to do a return without a return statement. If we were to use a return statement here, our program wouldnt compile.
```rust,no_compile
fn main() {
    let x:Option<u32> = Some(0);

   

    let y = match x {
        Some(a) => {
            // returning a doesnt return it as the result 
            // of the block which would cause y to equal 0
            // it returns it as the result of the function
            // which would assume that the function is "fn main() -> u32" 
            return a
        }
        None => {
            panic!("Couldnt find value");
        }
    };
   println!("{y:?}");
}
```
returns can be useful though if you want to return out of a function in a `if` or `match` statement
```rust,editable
# fn main() {let x = anon_function(); println!("{x:?}")}
pub fn anon_function() -> Result<String,u32> {
    let mut x:Option<u32> = Some(0);
    // x:Option<u32> = None;
    // ^ uncomment me 

    let y = match x {
        Some(a) => {
            a
        }
        None => {
            // because we want to stop executing
            // the function here, using a return 
            // is useful
            return Err(0)
        }
    };
   let z = format!("{y:?}");

    println!("{z:?}");

    
    Ok(String::from(z)) // interchangable with "return Ok(String::from(z));"
}
```
deciding which to use is all up to context and your own coding style. 
## Bringing it all together

Now, lets bring it all together. In your `main.rs` file lets add the functionality we created in `random_numbers.rs`

```rust
# pub struct RandomNumber { }
# impl RandomNumber { pub fn get_random_number() -> u32 { 123 }
# pub fn add_random_number(num_1:u32, num_2:u32) -> u32 { return num_1 + num_2; }
# pub fn update_random_number(mut num_1:u32) { num_1 = 144 }
# }

fn main() {
    let random_number_one = RandomNumber::get_random_number();
    let random_number_two = RandomNumber::get_random_number();

    RandomNumber::update_random_number(random_number_one);
    
    let result = RandomNumber::add_random_number(random_number_one, random_number_two);
    
    println!("your result is: {result:?}");
}
```