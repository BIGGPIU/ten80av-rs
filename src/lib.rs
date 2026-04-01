#![no_std]

//! A modular library for programming Ten80 Autonomous Vehicles. This library provides a 
//! a simple interface for all the components that are apart of the Vehicle. 
//! 
//! 
//! The [Discovery Rust Book](https://docs.rust-embedded.org/discovery/microbit/) is an amazing resource that assisted the most in the development 
//! of this library 
//! 
//! **Currently, only the Microbit:V2 is supported**
//! 
//! # Overview
//! This crate has two main modules ```devices``` and ```utils```
//! * [`devices`] houses the internal and external sesnors and motors on (and outside) the micro:bit
//!     * [`devices::internal`] internal sensors like: [`devices::internal::RecieverRadio`], [`devices::internal::Magnometer`]
//!     * [`devices::external`] external sensors and motors, like your [`devices::external::TurningMotor`], [`devices::external::AccelerationMotor`], [`devices::external::UltraSonicDistanceSensor`], and your [`devices::external::IRSensor`]
//! * [`utils`] has some functions and structures that exist to make it easier to develop for your microbit without having a couple datasheets opened on another monitor.
//!     * [`utils::ports`] will be your best friend
//! 
//! 
//! # Examples
//! 
//! ## Example: Make your AV go forwards 
//! ```rust
//!     #![no_std]
//!     #![no_main]
//! 
//!     use ten80av_rs;
//!     use microbit::hal::saadc::SaadcConfig;
//!     use microbit::hal::{ Clocks, Saadc, Timer, uarte};
//!
//!     use microbit::{self as _};
//!     use microbit::{
//!         board::Board,
//!         display::blocking::Display
//!     };
//!
//! 
//!    #[cortex_m_rt::entry]
//!    fn main_fn_board_implementation_new() -> ! {
//!        // get the stuff we need from the board
//!        let mut board = Board::take().unwrap();
//!        let mut display = Display::new(board.display_pins);
//!        let mut timer = Timer::new(board.TIMER0);
//!
//!        // set up the ADC
//!        let saadc_config = SaadcConfig::default();
//!        let mut saadc = Saadc::new(board.ADC, saadc_config);
//!
//!        // get the i2c pins
//!        let i2c_internal:microbit::hal::twim::Pins = board.i2c_internal.into();
//!        let i2c_external:microbit::hal::twim::Pins = board.i2c_external.into();
//!
//!        // set up the serial writer
//!        let mut serial:ten80av_rs::utils::serial::UartePort<microbit::pac::UARTE0> = {
//!            
//!            let uart_pins = board.uart;
//!
//!            let serial = uarte::Uarte::new(
//!                board.UARTE0, 
//!                uart_pins.into(), 
//!                uarte::Parity::EXCLUDED, 
//!                uarte::Baudrate::BAUD115200
//!            );
//!            
//!            ten80av_rs::utils::serial::UartePort::new(serial)
//!        };
//!
//!        // clear the serial monitor
//!        ten80av_rs::utils::serial::Serial::clear_terminal(&mut serial);
//!
//!        // declare our controllers before initializing them with data
//!        let mut servo_controller:ten80av_rs::devices::external::ServoMotorController;
//!        
//!        servo_controller = ten80av_rs::devices::external::ServoMotorController::new(
//!            board.TWIM0, 
//!            i2c_external, 
//!            &mut serial
//!        );
//!
//!        // set up our acceleration motor
//!        let mut acceleration_motor = ten80av_rs::devices::external::AccelerationMotor::new(
//!            ten80av_rs::utils::ports::MicrotbitDriverPorts::M1
//!        ).unwrap();
//!
//!        acceleration_motor.change_speed(&mut servo_controller, 4095);
//!        acceleration_motor.change_state(&mut servo_controller, ten80av_rs::devices::external::MotorState::Forward);
//!
//!        loop {
//!            // show something on the led matrix
//!            ten80av_rs::utils::display::DisplayFuncs::display_message(
//!                "I use rust btw", 
//!                &mut display, 
//!                &mut timer, 
//!                10_000
//!            );
//!
//!        }
//!    }
//! ```````
//! 
//! ## Example: Write a status message to the serial monitor 
//! 
//! ```rust
//! use ten80av_rs;
//! #![no_std]
//! #![no_main]
//! 
//! use ten80av_rs;
//! use ten80av_rs::utils::serial::*;
//! 
//! use microbit::hal::saadc::SaadcConfig;
//! use microbit::hal::{ Clocks, Saadc, Timer, uarte};
//!
//! use microbit::{self as _};
//! use microbit::{
//!     board::Board,
//!     display::blocking::Display
//! };
//! 
//! #[cortex_m_rt::entry]
//! fn main_fn() -> {
//!     let mut board = Board::take().unwrap();
//!     let i2c_external:microbit::hal::twim::Pins = board.i2c_external.into();
//! 
//!     // set up the serial writer
//!     let mut serial:ten80av_rs::utils::serial::UartePort<microbit::pac::UARTE0> = {
//!         
//!         let uart_pins = board.uart;
//! 
//!         let serial = uarte::Uarte::new(
//!             board.UARTE0, 
//!             uart_pins.into(), 
//!             uarte::Parity::EXCLUDED, 
//!             uarte::Baudrate::BAUD115200
//!         );
//!         
//!         ten80av_rs::utils::serial::UartePort::new(serial)
//!     };
//! 
//!     ten80av_rs::utils::serial::Serial::clear_terminal(&mut serial);
//!     
//!     let mut servo_controller = ten80av_rs::devices::external::ServoMotorController::new(board.TWIM0, i2c_external, &mut serial);
//!     
//!     let mut turning_motor = match ten80av_rs::devices::external::TurningMotor::new(ten80av_rs::utils::ports::MicrotbitDriverPorts::M1) {
//!         Ok(x) => x,
//!         Err(_) => {
//!             ten80av_rs::utils::serial::Serial::write(serial, "Cannot servo motor with Motor Ports.",MessageSeverity::Error);
//!         }    
//!     }
//! }
//! ```````
//! 


pub mod devices;
pub mod utils;