# Programming the acceleration motor

## Creating your motor controller 
We have one last intermediate step before we can actually program our acceleration motor we have to program a controller for it. We do this because we have to configure the chip that controls the motors. Create a file called `servo_motor_controller.rs` in your `devices` folder and paste this code:

```rust,no_run
use microbit::hal::Twim;
use microbit::hal::twim::Frequency;
use pwm_pca9685::{Address, Pca9685};

pub struct ServoMotorController {
    pub(crate) controller: Pca9685<Twim<microbit::pac::TWIM0>>
}

impl ServoMotorController {

}
```

the code should appear gray at this point. hit `CTRL + .` then `Enter` this will add the file to your `mod.rs`

What this does is create a structure that contains the Object that lets us communicate with the PCA9685 chip. `Twim<microbit::pac::TWIM0>` is the I2C interface on the micro:bit. This guide will not explain how I2C works but you can read more about it [here](https://learn.sparkfun.com/tutorials/i2c/all).

Now that we have the structure created we have to create a way to use it. 
add this function inside of `impl ServoMotorController {}`

```rust,no_run
pub fn new(
    // this allows us to interact with our I2C interface
    board_twim0:microbit::pac::TWIM0,
    // our actual i2c pins. 
    board_external_i2c_pins:microbit::hal::twim::Pins
) -> ServoMotorController 
{
    
    let address = Address::default();

    // set up our I2C interface so it can be used
    let i2c_twim = microbit::hal::Twim::new(
        board_twim0,
        board_external_i2c_pins.into(),
        Frequency::K250
    );     

    // try to create our controller
    let mut pwm: Pca9685<Twim<microbit::pac::TWIM0>> = match pwm_pca9685::Pca9685::new(i2c_twim, address) {
        // if it works then use it 
        Ok(x) => x,
        // otherwise, quit the program
        Err(_) => {
                panic!();
            },
    };

    // set the prescale to a value the motors like
    pwm.set_prescale(100).unwrap();

    // enable the chip
    pwm.enable().unwrap();

    return ServoMotorController {
        controller: pwm
    }
}
```

with this function created we now have a way to interact with our controller inside of other functions.

## Why are we making our own interface? The object is right there

You may be wondering why did we go out of way to create our own struct just to hold this one object. Its apart of a process I like to describe as **Stupid Proofing**. Your first instinct may be to give the person using your library as much power as possible. It can meet more peoples needs that way right? Thats where you're wrong, not fundamentally wrong but wrong in this case. You are creating a library for one chip, one motor interface and likely one kind of motor. If you give people the power to configure the chip themselves they might do it wrong. When you're writing a library like this one **you dont want to give people the liberty to be wrong**. For example: Later on in this guide you will end up setting up a serial interface for your micro:bit. To set this up you create a `UartePort` struct that looks like this 
```rust,no_copy,no_run
UartePort::new(Uarte::new(
    # board_uarte,
    # uart_pin.into(),
    # uarte::Parity::EXCLUDED,
    --snip--
    uarte::Baudrate::BAUD115200
))
```
If the developers of the Uarte library were to not engage in the practice of stupid proofing they would just let you set the bauderate using a `u32` like this:
```rust,no_copy,no_run
UartePort::new(Uarte::new(
    # board_uarte,
    # uart_pin.into(),
    # uarte::Parity::EXCLUDED,
    --snip--
    # // the typo is there to prove my point
    baudrate: 115300
))
```
It should be obvious how this can go wrong. One typo can cause someone to be stuck in hours of bug fixing and stack overflow questions just because the developers behind Uarte gave you the liberty to be wrong. When you're developing libraries always give yourself time to ask yourself the question "how much nuance is there in the problem im solving?" If there is no nuance then dont even ask the user how they want to configure the object. If there is some nuance then consider using an `enum`. If the problem is really nuanced and each person has different needs then this is the time to allow the user to fully customize how they want to configure the object.

## Creating your Acceleration Motor Interface

Create a file named `acceleration_motor_controller.rs`. just like before press `CTRL + .` then `Enter` to add this file to your mod.rs. Then add this code to your `acceleration_motor_controller.rs` file
```rust,no_run
use pwm_pca9685::Channel;

// an enum to say what we're doing with the motor right now
pub enum MotorState {
    Coast,
    Reverse,
    Forward,
    Brake
}

// function to make your life easier
pub enum MicrobitDriverMotorPorts {
    M1,
    M2,
    M3,
    M4,
}

impl MicrobitDriverMotorPorts {
    // function to make using this library easier
    pub fn motor_get(self) -> (Channel,Channel) {
        match self {
                        // the channels associated with port MX
            Self::M1 =>   (Channel::C7,Channel::C6),
            Self::M2 =>   (Channel::C5,Channel::C4),
            Self::M3 =>   (Channel::C3,Channel::C2),
            Self::M4 =>   (Channel::C1,Channel::C0)
        }
    }
}

pub struct AccelerationMotor {
    forward_port:Channel,
    reverse_port:Channel,
    speed:u16,
    state:MotorState
}


impl AccelerationMotor {
    pub fn new(forward_channel:Channel, reverse_channel:Channel) -> Self{
        return Self {
            forward_port:forward_channel,
            reverse_port:reverse_channel,
            speed:0_u16,
            state:MotorState::Brake
        }
    }
}
```

now that we have a way to initialize our acceleration controller, now we have to give it the ability to actually control the motors on the Autonomous Vehicle.

Before we write the functions though we should learn about how the state of the two ports affects how the motor moves. The behavior can be described with this table:

|                  | Forward Port ON | Forward Port OFF |
|------------------|-----------------|------------------|
| Reverse Port ON  | Brake           | Reverse          |
| Reverse Port OFF | Forward         | Coast            |


now that we know this information we can start writing functions that let us interact with the motor

```rust,no_run
impl AccelerationMotor {
// we are separating the change_state and change speed
// functions to make this library easier to use.
// it would get annoying if we had to put in the same speed/state 
// we had before if we only wanted to change our state or speed
pub fn change_state(&mut self, servo_motor_controller: &mut crate::devices::servo_motor_controller::ServoMotorController,new_state:MotorState) {
    self.state = new_state;
    self.write_to_motor_chip(servo_motor_controller);
}

pub fn change_speed(&mut self, servo_motor_controller: &mut crate::devices::servo_motor_controller::ServoMotorController,new_speed:u16) {
        self.speed = new_speed;
        self.write_to_motor_chip(servo_motor_controller);
    }

// convienience function
pub fn brake(&mut self, servo_motor_controller: &mut crate::devices::servo_motor_controller::ServoMotorController) {
    self.state = MotorState::Brake;
    self.write_to_motor_chip(servo_motor_controller);
}

// convienience function
pub fn forward(&mut self, servo_motor_controller: &mut crate::devices::servo_motor_controller::ServoMotorController) {
    self.state = MotorState::Forward;
    self.write_to_motor_chip(servo_motor_controller);
}

fn write_to_motor_chip(&self, servo_motor_controller: &mut crate::devices::servo_motor_controller::ServoMotorController) {
        

    match self.state {
        MotorState::Coast => {
            servo_motor_controller.controller.set_channel_full_off(self.forward_port).unwrap();
            servo_motor_controller.controller.set_channel_full_off(self.reverse_port).unwrap();
            
        },
        MotorState::Reverse => {
            servo_motor_controller.controller.set_channel_full_off(self.forward_port).unwrap();
            // set the reverse port on for self.speed / 4095 cycles
            servo_motor_controller.controller.set_channel_on_off(self.reverse_port, 0, self.speed).unwrap();
        },
        MotorState::Forward => {
            // set the reverse port on for self.speed / 4095 cycles
            servo_motor_controller.controller.set_channel_on_off(self.forward_port, 0,self.speed).unwrap();
            servo_motor_controller.controller.set_channel_full_off(self.reverse_port).unwrap();
        },
        MotorState::Brake => {
            servo_motor_controller.controller.set_channel_on_off(self.forward_port, 0,self.speed).unwrap();
            servo_motor_controller.controller.set_channel_on_off(self.reverse_port, 0,self.speed).unwrap();
            
        },
    }
}
}
```

The final thing we have to do before we're done is update our `mod.rs` file. Add the following lines to your `devices/mod.rs` file

```rust
pub use servo_motor_controller::ServoMotorController;
pub use acceleration_motor_controller::*;
```

Your `devices/mod.rs` file should look like this:
```rust
mod servo_motor_controller;
mod acceleration_motor_controller;


pub use servo_motor_controller::ServoMotorController;
pub use acceleration_motor_controller::*;
```

once you have that done then you have completely finished writing an interface to your Acceleration Motor. A user of your library would use it like this 

```rust
#![no_std]
#![no_main]

use ten80_av_library;
use microbit::{self as _};
use microbit::board::Board;
use ten80_av_library::devices::MicrobitDriverMotorPorts;

#[cortex_m_rt::entry]
fn main_fn() -> ! {
    let mut board = Board::take().unwrap();

    let i2c_external:microbit::hal::twim::Pins = board.i2c_external.into();

    let mut servo_controller = ten80_av_library::devices::ServoMotorController::new(
        board.TWIM0,
        i2c_external,
    );

    let (forward_port,reverse_port) = MicrobitDriverMotorPorts::M1.motor_get();

    let mut acceleration_motor = ten80_av_library::devices::AccelerationMotor::new(
        forward_port,
        reverse_port
    );

    acceleration_motor.change_speed(&mut servo_controller, 4095);
    acceleration_motor.change_state(&mut servo_controller, ten80_av_library::devices::MotorState::Brake);

    loop {

    }
}
```


**Remember to save all of your `mod.rs` files.** 