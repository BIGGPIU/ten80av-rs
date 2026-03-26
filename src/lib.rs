#![no_std]

//! A simple library for programming Ten80 Autonomous Vehicles
//! 
//! ## THIS ONLY SUPPORTS THE MICRO:BIT V2
//! 
//! 
//! 
//! # Examples
//! 
//! ## Make your AV go forwards 
//! ```rust
//!     #![no_std]
//!     #![no_main]
//!     
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
//! ```

pub mod devices;
pub mod utils;
