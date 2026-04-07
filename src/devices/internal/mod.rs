//! Internal Components on the Micro:bit v2
//! 
//! # Overview
//! * [Magnometer] - Interface for the on board Magnometer.
//! * [Speaker] - Interface for the onboard speaker, this can used to support external buzzers as well.
//! * [OnboardSensorController] - Controller for all the onboard sensors. Cannot be used with extenral device controllers like [`crate::devices::external::ServoMotorController`].
//! 
//! # Using External and Internal Controllers at the same time
//!
//! When looking at this crate something might stick out: "[`OnboardSensorController`] cannot be used at the same as external device controllers like [`crate::devices::external::ServoMotorController`]"
//! This can be a cause for concern because its not impossible to want to use both an external motor and the internal Magnometer at the same time.
//! 
//! Because both of the controllers use Board.TWIM0 its impossible for both of them to exist at the same time. Though there is a workaround: We can pass around Board.TWIM0 as 
//! its needed. [`OnboardSensorController::into_servo_motor_controller`] does this for you. 
//! 
//! Unfortunately this does create code thats more verbose but its necessary to create code that compiles
//! 
//! 
//! # todo
//! * Include Magnometer Direction calculations.  


mod onboardsensorcontroller;
mod speaker;
mod magnometer;
pub mod radio;



pub use onboardsensorcontroller::OnboardSensorController;
pub use magnometer::Magnometer;
pub use speaker::Speaker;
