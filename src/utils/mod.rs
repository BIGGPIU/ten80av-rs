//! Utilities that help with programming the vehicle
//! 
//! # Overview 
//! * [`serial`] - Functions that make it easier to initalize your Serial monitor and standardize messages sent to it.
//! * [`ports`] - Functions that make it easier to use the pins on the Driver Expansion Board without looking at the micro:bit Datasheet.
//! * [`display`] - A collection of Numbers, Letters and Symbols for display on the 5x5 display matrix
//! 
//! # Examples:
//! ## Example: Use port M1 to accelerate
//! ```rust
//! use microbit::{self as _};
//! use microbit::{
//!     board::Board,
//!     display::blocking::Display
//! };
//! 
//! 
//! #[cortex_m_rs::entry]
//! fn main_fn() -> ! {
//! 
//! let mut board = Board::take().unwrap();
//! let mut timer = Timer::new(board.TIMER0);
//! let i2c_external:microbit::hal::twim::Pins = board.i2c_external.into();
//! 
//! let mut serial:ten80av_rs::utils::serial::UartePort<microbit::pac::UARTE0> = {
//!        
//!     let uart_pins = board.uart;
//!
//!     let serial = uarte::Uarte::new(
//!         board.UARTE0, 
//!         uart_pins.into(), 
//!         uarte::Parity::EXCLUDED, 
//!         uarte::Baudrate::BAUD115200
//!     );
//!    
//!     ten80av_rs::utils::serial::UartePort::new(serial)
//! };
//! 
//! ten80av_rs::utils::serial::Serial::clear_terminal(&mut serial);
//! 
//! let mut servo_controller = ten80av_rs::devices::external::ServoMotorController::new(board.TWIM0, i2c_external, &mut serial);
//! 
//! let mut acceleration_motor = ten80av_rs::devices::external::AccelerationMotor::new(ten80av_rs::utils::ports::MicrotbitDriverPorts::M1).unwrap();
//! 
//! acceleration_motor.forward(&mut servo_controller);
//! 
//! timer.delay_ms(500);
//! 
//! acceleration_motor.brake(&mut servo_controller);
//! 
//! loop {
//! 
//!     
//! }
//! }
//! ```



pub mod serial;
pub mod ports;
pub mod display;