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
//! This crate has two main modules [`devices`] and [`utils`]
//! * [`devices`] houses the internal and external sesnors and motors on (and outside) the micro:bit
//!     * [`devices::internal`] has internal sensors like: [`devices::internal::radio`], [`devices::internal::Magnometer`]
//!     * [`devices::external`] has external sensors and motors, like your [`devices::external::TurningMotor`], [`devices::external::AccelerationMotor`], [`devices::external::UltraSonicDistanceSensor`], and your [`devices::external::IRSensor`]
//! * [`utils`] has some functions and structures that exist to make it easier to develop for your microbit without having a couple datasheets opened on another monitor.
//!     * [`utils::ports`] will be your best friend
//! 
//! # Using External and Internal Controllers at the same time
//!
//! When looking at this crate something might stick out: "[`devices::internal::OnboardSensorController`] cannot be used at the same as external device controllers like [`devices::external::ServoMotorController`]"
//! This can be a cause for concern because its not impossible to want to use both an external motor and the internal Magnometer at the same time.
//! 
//! Because both of the controllers use Board.TWIM0 its impossible for both of them to exist at the same time. Though there is a workaround: We can pass around Board.TWIM0 as 
//! its needed. [`devices::internal::OnboardSensorController::into_servo_motor_controller`] does this for you. 
//! 
//! Unfortunately this does create code thats more verbose but its necessary to create code that compiles
//! 
//! Generally: instaead of getting your i2c pins and controllers as they're needed. They should be taken somewhere near the top of your main function
//! ```rust
//! #[cortext_m_rt::entry]
//! fn main_fn() {
//! let mut board = Board::take().unwrap();
//! 
//! // now we can lend out and take back our i2c pins and controllers whenever they're needed
//! let mut i2c_internal:microbit::hal::twim::Pins = board.i2c_internal.into();
//! let mut i2c_external:microbit::hal::twim::Pins = board.i2c_external.into(); 
//! let mut servo_controller:ten80av_rs::devices::external::ServoMotorController;
//! let mut onboard_sensor_controller:ten80av_rs::devices::internal::OnboardSensorController;
//! }
//! ```
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
//!             ten80av_rs::utils::serial::Serial::write(serial, "Cannot start servo motor with Motor Ports.",MessageSeverity::Error);
//!         }    
//!     }
//! }
//! ```````
//! 
//! ## Example: 90 seconds of bad apple
//! ```
//! 
//! #[cortex_m_rt::entry]
//! fn main_fn() -> {
//!     let mut board = Board::take().unwrap();
//!     let mut timer = Timer::new(board.TIMER0);
//! 
//!     let mut serial:ten80av_rs::utils::serial::UartePort<microbit::pac::UARTE0> = {        
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
//!     };
//!     
//!     let speaker_pin = board.speaker_pin.into_push_pull_output(microbit::hal::gpio::Level::Low).degrade();
//!     let speaker_driver = ten80av_rs::devices::internal::Speaker::new(speaker_pin);
//! 
//!     const NOTE_ON_MS:[u32;639] = [0, 102, 204, 306, 408, 510, 612, 714, 816, 1122, 1224, 1326, 1428, 1530, 1632, 1938, 2040, 2346, 2448, 2551, 2653, 2755, 2857, 2959, 3061, 3163, 3265, 3367, 3469, 3571, 3673, 3775, 3877, 3979, 4081, 4387, 4489, 4591, 4693, 4795, 4897, 4999, 5102, 5204, 5306, 5408, 5510, 5612, 5714, 5816, 5918, 6020, 6122, 6224, 6326, 6428, 6530, 6632, 6734, 6836, 6938, 7040, 7142, 7244, 7346, 7653, 7755, 7857, 7959, 8061, 8163, 8469, 8571, 8877, 8979, 9081, 9183, 9285, 9387, 9489, 9591, 9693, 9795, 9897, 9999, 10102, 10204, 10306, 10408, 10510, 10612, 10918, 11020, 11122, 11224, 11326, 11428, 11734, 11836, 12142, 12244, 12551, 12653, 12959, 13061, 13163, 13265, 13367, 13469, 13571, 13673, 13775, 13877, 14183, 14285, 14387, 14489, 14591, 14693, 14999, 15102, 15408, 15510, 15612, 15714, 15816, 15918, 16020, 16122, 16224, 16326, 16428, 16530, 16632, 16734, 16836, 16938, 17040, 17142, 17448, 17551, 17653, 17755, 17857, 17959, 18061, 18163, 18265, 18367, 18469, 18571, 18673, 18775, 18877, 18979, 19081, 19183, 19285, 19387, 19489, 19591, 19693, 19795, 19897, 19999, 20102, 20204, 20306, 20408, 20714, 20816, 20918, 21020, 21122, 21224, 21530, 21632, 21938, 22040, 22142, 22244, 22346, 22448, 22551, 22653, 22755, 22857, 22959, 23061, 23163, 23265, 23367, 23469, 23571, 23673, 23979, 24081, 24183, 24285, 24387, 24489, 24795, 24897, 25204, 25306, 25612, 25714, 26020, 26122, 26224, 26326, 26428, 26530, 26632, 26734, 26836, 26938, 27244, 27346, 27448, 27551, 27653, 27755, 27857, 27959, 28061, 28163, 28265, 28367, 28469, 28571, 28877, 28979, 29081, 29183, 29285, 29387, 29489, 29591, 29693, 29795, 29897, 29999, 30102, 30204, 30510, 30612, 30714, 30816, 30918, 31020, 31122, 31224, 31326, 31428, 31530, 31632, 31734, 31836, 32142, 32244, 32346, 32448, 32550, 32653, 32755, 32857, 32959, 33061, 33163, 33265, 33367, 33469, 33775, 33877, 33979, 34081, 34183, 34285, 34387, 34489, 34591, 34693, 34795, 34897, 34999, 35102, 35408, 35510, 35612, 35714, 35816, 35918, 36020, 36122, 36224, 36326, 36428, 36530, 36632, 36734, 37040, 37142, 37244, 37346, 37448, 37550, 37653, 37755, 37857, 37959, 38061, 38163, 38265, 38367, 38673, 38775, 38877, 38979, 39081, 39183, 39285, 39387, 39489, 39591, 39693, 39795, 39897, 39999, 40306, 40408, 40510, 40612, 40714, 40816, 40918, 41020, 41122, 41224, 41326, 41428, 41530, 41632, 41938, 42040, 42142, 42244, 42346, 42448, 42550, 42653, 42755, 42857, 42959, 43061, 43163, 43265, 43571, 43673, 43775, 43877, 43979, 44081, 44183, 44285, 44387, 44489, 44591, 44693, 44795, 44897, 45204, 45306, 45408, 45510, 45612, 45714, 45816, 45918, 46020, 46122, 46224, 46326, 46428, 46530, 46836, 46938, 47040, 47142, 47244, 47346, 47448, 47550, 47653, 47755, 47857, 47959, 48061, 48163, 48469, 48571, 48673, 48775, 48877, 48979, 49081, 49183, 49285, 49387, 49489, 49591, 49693, 49795, 50102, 50204, 50306, 50408, 50510, 50612, 50714, 50816, 50918, 51020, 51122, 51224, 51326, 51428, 51734, 51836, 51938, 52040, 52142, 52244, 52448, 52653, 52857, 53061, 53367, 53469, 53673, 53877, 54081, 54285, 54489, 54693, 54999, 55102, 55306, 55510, 55714, 55918, 56122, 56326, 56632, 56734, 56938, 57142, 57346, 57550, 57755, 57959, 58265, 58367, 58571, 58775, 58979, 59183, 59387, 59591, 59897, 59999, 60204, 60408, 60612, 60816, 61020, 61224, 61530, 61632, 61836, 62040, 62244, 62448, 62653, 62857, 63163, 63265, 63469, 63673, 63877, 64081, 64285, 64489, 64795, 64897, 65101, 65306, 65510, 65714, 65918, 66122, 66428, 66530, 66734, 66938, 67142, 67346, 67550, 67755, 68061, 68163, 68367, 68571, 68775, 68979, 69183, 69387, 69693, 69795, 69999, 70204, 70408, 70612, 70816, 71020, 71326, 71428, 71632, 71836, 72040, 72244, 72448, 72653, 72959, 73061, 73265, 73469, 73673, 73877, 74081, 74285, 74591, 74693, 74897, 75101, 75306, 75510, 75714, 75918, 76224, 76326, 76530, 76734, 76938, 77142, 77346, 77550, 77857, 78367, 78571, 78673, 78877, 79081, 79183, 79387, 79489, 79693, 79897, 79999, 80204, 80306, 80510, 80714, 80816, 80918, 81020, 81122, 81224, 81326, 81428, 81530, 81632, 81836, 81938, 82142, 82346, 82448, 82653, 82755, 82959, 83163, 83265, 83469, 83571, 83775, 83979, 84081, 84183, 84285, 84387, 84489, 84591, 84693, 84795, 84897, 85101, 85204, 85408, 85612, 85714, 85918, 86020, 86224, 86428, 86530, 86734, 86836, 87040, 87244, 87346, 87448, 87550, 87653, 87755, 87857, 87959, 88061, 88163, 88367, 88469, 88673, 88877, 88979, 89183, 89285, 89489, 89693, 89795, 89999]; 
//!     const NOTE_OFF_MS:[u32;640] = [102, 204, 306, 408, 510, 612, 714, 816, 918, 1224, 1326, 1428, 1530, 1632, 1734, 2040, 2142, 2448, 2551, 2653, 2755, 2857, 2959, 3061, 3163, 3265, 3367, 3469, 3571, 3673, 3775, 3877, 3979, 4081, 4183, 4489, 4591, 4693, 4795, 4897, 4999, 5102, 5204, 5306, 5408, 5510, 5612, 5714, 5816, 5918, 6020, 6122, 6224, 6326, 6428, 6530, 6632, 6734, 6836, 6938, 7040, 7142, 7244, 7346, 7448, 7755, 7857, 7959, 8061, 8163, 8265, 8571, 8673, 8979, 9081, 9183, 9285, 9387, 9489, 9591, 9693, 9795, 9897, 9999, 10102, 10204, 10306, 10408, 10510, 10612, 10714, 11020, 11122, 11224, 11326, 11428, 11530, 11836, 11938, 12244, 12346, 12653, 12755, 13061, 13163, 13265, 13367, 13469, 13571, 13673, 13775, 13877, 13979, 14285, 14387, 14489, 14591, 14693, 14795, 15102, 15204, 15510, 15612, 15714, 15816, 15918, 16020, 16122, 16224, 16326, 16428, 16530, 16632, 16734, 16836, 16938, 17040, 17142, 17244, 17551, 17653, 17755, 17857, 17959, 18061, 18163, 18265, 18367, 18469, 18571, 18673, 18775, 18877, 18979, 19081, 19183, 19285, 19387, 19489, 19591, 19693, 19795, 19897, 19999, 20102, 20204, 20306, 20408, 20510, 20816, 20918, 21020, 21122, 21224, 21326, 21632, 21734, 22040, 22142, 22244, 22346, 22448, 22551, 22653, 22755, 22857, 22959, 23061, 23163, 23265, 23367, 23469, 23571, 23673, 23775, 24081, 24183, 24285, 24387, 24489, 24591, 24897, 24999, 25306, 25408, 25714, 25816, 26122, 26224, 26326, 26428, 26530, 26632, 26734, 26836, 26938, 27040, 27346, 27448, 27551, 27653, 27755, 27857, 27959, 28061, 28163, 28265, 28367, 28469, 28571, 28673, 28979, 29081, 29183, 29285, 29387, 29489, 29591, 29693, 29795, 29897, 29999, 30102, 30204, 30306, 30612, 30714, 30816, 30918, 31020, 31122, 31224, 31326, 31428, 31530, 31632, 31734, 31836, 31938, 32244, 32346, 32448, 32550, 32653, 32755, 32857, 32959, 33061, 33163, 33265, 33367, 33469, 33571, 33877, 33979, 34081, 34183, 34285, 34387, 34489, 34591, 34693, 34795, 34897, 34999, 35102, 35204, 35510, 35612, 35714, 35816, 35918, 36020, 36122, 36224, 36326, 36428, 36530, 36632, 36734, 36836, 37142, 37244, 37346, 37448, 37550, 37653, 37755, 37857, 37959, 38061, 38163, 38265, 38367, 38469, 38775, 38877, 38979, 39081, 39183, 39285, 39387, 39489, 39591, 39693, 39795, 39897, 39999, 40102, 40408, 40510, 40612, 40714, 40816, 40918, 41020, 41122, 41224, 41326, 41428, 41530, 41632, 41734, 42040, 42142, 42244, 42346, 42448, 42550, 42653, 42755, 42857, 42959, 43061, 43163, 43265, 43367, 43673, 43775, 43877, 43979, 44081, 44183, 44285, 44387, 44489, 44591, 44693, 44795, 44897, 44999, 45306, 45408, 45510, 45612, 45714, 45816, 45918, 46020, 46122, 46224, 46326, 46428, 46530, 46632, 46938, 47040, 47142, 47244, 47346, 47448, 47550, 47653, 47755, 47857, 47959, 48061, 48163, 48265, 48571, 48673, 48775, 48877, 48979, 49081, 49183, 49285, 49387, 49489, 49591, 49693, 49795, 49897, 50204, 50306, 50408, 50510, 50612, 50714, 50816, 50918, 51020, 51122, 51224, 51326, 51428, 51530, 51836, 51938, 52040, 52142, 52244, 52448, 52653, 52857, 53061, 53163, 53469, 53673, 53877, 54081, 54285, 54489, 54693, 54795, 55102, 55306, 55510, 55714, 55918, 56122, 56326, 56428, 56734, 56938, 57142, 57346, 57550, 57755, 57959, 58061, 58367, 58571, 58775, 58979, 59183, 59387, 59591, 59693, 59999, 60204, 60408, 60612, 60816, 61020, 61224, 61326, 61632, 61836, 62040, 62244, 62448, 62653, 62857, 62959, 63265, 63469, 63673, 63877, 64081, 64285, 64489, 64591, 64897, 65101, 65306, 65510, 65714, 65918, 66122, 66224, 66530, 66734, 66938, 67142, 67346, 67550, 67755, 67857, 68163, 68367, 68571, 68775, 68979, 69183, 69387, 69489, 69795, 69999, 70204, 70408, 70612, 70816, 71020, 71122, 71428, 71632, 71836, 72040, 72244, 72448, 72653, 72755, 73061, 73265, 73469, 73673, 73877, 74081, 74285, 74387, 74693, 74897, 75101, 75306, 75510, 75714, 75918, 76020, 76326, 76530, 76734, 76938, 77142, 77346, 77550, 77653, 77959, 78469, 78673, 78775, 78979, 79183, 79285, 79489, 79591, 79795, 79999, 80101, 80306, 80408, 80612, 80816, 80918, 81020, 81122, 81224, 81224, 81326, 81428, 81530, 81632, 81734, 81938, 82040, 82244, 82448, 82550, 82755, 82857, 83061, 83265, 83367, 83571, 83673, 83877, 84081, 84183, 84285, 84387, 84489, 84591, 84693, 84795, 84897, 84999, 85204, 85306, 85510, 85714, 85816, 86020, 86122, 86326, 86530, 86632, 86836, 86938, 87142, 87346, 87448, 87550, 87653, 87755, 87857, 87959, 88061, 88163, 88265, 88469, 88571, 88775, 88979, 89081, 89285, 89387, 89591, 89795, 89897, 90101]; 
//!     const NOTE_NUM:[u8;639] = [63, 63, 65, 65, 66, 66, 68, 68, 70, 70, 75, 75, 73, 73, 70, 70, 63, 63, 70, 70, 68, 68, 66, 66, 65, 65, 63, 63, 65, 65, 66, 66, 68, 68, 70, 70, 68, 68, 66, 66, 65, 65, 63, 63, 65, 65, 66, 66, 65, 65, 63, 63, 62, 62, 65, 65, 63, 63, 65, 65, 66, 66, 68, 68, 70, 70, 75, 75, 73, 73, 70, 70, 63, 63, 70, 70, 68, 68, 66, 66, 65, 65, 63, 63, 65, 65, 66, 66, 68, 68, 70, 70, 68, 68, 66, 66, 65, 65, 66, 66, 68, 68, 70, 70, 63, 63, 65, 65, 66, 66, 68, 68, 70, 70, 75, 75, 73, 73, 70, 70, 63, 63, 70, 70, 68, 68, 66, 66, 65, 65, 63, 63, 65, 65, 66, 66, 68, 68, 70, 70, 68, 68, 66, 66, 65, 65, 63, 63, 65, 65, 66, 66, 65, 65, 63, 63, 62, 62, 65, 65, 63, 63, 65, 65, 66, 66, 68, 68, 70, 70, 75, 75, 73, 73, 70, 70, 63, 63, 70, 70, 68, 68, 66, 66, 65, 65, 63, 63, 65, 65, 66, 66, 68, 68, 70, 70, 68, 68, 66, 66, 65, 65, 66, 66, 68, 68, 70, 70, 73, 73, 75, 75, 70, 70, 68, 68, 70, 70, 68, 68, 70, 70, 73, 73, 75, 75, 70, 70, 68, 68, 70, 70, 68, 68, 70, 70, 68, 68, 66, 66, 65, 65, 61, 61, 63, 63, 61, 61, 63, 63, 65, 65, 66, 66, 68, 68, 70, 70, 63, 63, 70, 70, 73, 73, 73, 73, 75, 75, 70, 70, 68, 68, 70, 70, 68, 68, 70, 70, 73, 73, 75, 75, 70, 70, 68, 68, 70, 70, 68, 68, 70, 70, 68, 68, 66, 66, 65, 65, 61, 61, 63, 63, 61, 61, 63, 63, 65, 65, 66, 66, 68, 68, 70, 70, 63, 63, 70, 70, 73, 73, 73, 73, 75, 75, 70, 70, 68, 68, 70, 70, 68, 68, 70, 70, 73, 73, 75, 75, 70, 70, 68, 68, 70, 70, 68, 68, 70, 70, 68, 68, 66, 66, 65, 65, 61, 61, 63, 63, 61, 61, 63, 63, 65, 65, 66, 66, 68, 68, 70, 70, 63, 63, 70, 70, 73, 73, 73, 73, 75, 75, 70, 70, 68, 68, 70, 70, 68, 68, 70, 70, 73, 73, 75, 75, 70, 70, 68, 68, 70, 70, 75, 75, 77, 77, 78, 78, 77, 77, 75, 75, 73, 73, 70, 70, 68, 68, 70, 70, 68, 68, 66, 66, 65, 65, 61, 61, 63, 63, 70, 70, 73, 73, 73, 75, 70, 68, 70, 70, 68, 70, 73, 75, 70, 68, 70, 70, 68, 70, 68, 66, 65, 61, 63, 63, 61, 63, 65, 66, 68, 70, 63, 63, 70, 73, 73, 75, 70, 68, 70, 70, 68, 70, 73, 75, 70, 68, 70, 70, 68, 70, 68, 66, 65, 61, 63, 63, 61, 63, 65, 66, 68, 70, 63, 63, 70, 73, 73, 75, 70, 68, 70, 70, 68, 70, 73, 75, 70, 68, 70, 70, 68, 70, 68, 66, 65, 61, 63, 63, 61, 63, 65, 66, 68, 70, 63, 63, 70, 73, 73, 75, 70, 68, 70, 70, 68, 70, 73, 75, 70, 68, 70, 70, 75, 77, 78, 77, 75, 73, 70, 70, 68, 70, 68, 66, 65, 61, 63, 63, 63, 63, 63, 63, 61, 63, 63, 63, 63, 61, 63, 63, 63, 63, 61, 63, 63, 61, 63, 68, 68, 66, 68, 63, 63, 63, 63, 61, 63, 63, 63, 63, 61, 63, 63, 63, 63, 61, 68, 68, 66, 68, 66, 66, 65, 66, 63, 63, 63, 63, 61, 63, 63, 63, 63, 61, 63, 63, 63, 63, 61, 63, 63, 61, 63, 68, 68, 66, 68, 63, 63, 63, 63, 61, 63, 63, 63, 63, 61, 63, 63]; 
//!     
//!     let max_i = NOTE_OFF_MS[NOTE_NUM.len() - 1];
//!     let mut time_elapsed = 0;
//!     let mut ptr = 0;
//!     while max_i > time_elapsed {
//!         if time_elapsed == NOTE_ON_MS[ptr] {
//!             let duration_ms = NOTE_OFF_MS[ptr] - NOTE_ON_MS[ptr];
//!             let duration_us = duration_ms * 1000;
//!             speaker_driver.play_tune_midi_note(&mut timer, NOTE_NUM[ptr], duration_us/2);
//!             
//!             time_elapsed += duration_ms;
//!             ptr += 1;
//!         }
//!         else {
//!             timer.delay_ms(1);
//!             time_elapsed+=1;
//!         }
//!     }
//! }
//! 
//! ```


pub mod devices;
pub mod utils;