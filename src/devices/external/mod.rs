//! External Motors and Sensors on the Driver Expansion Board
//! 
//! # Overview
//! * [`TurningMotor`] - Interface for the Motor that controls turning
//! * [`AccelerationMotor`] - Interface for the motor that controls acceleration
//! * [`UltraSonicDistanceSensor`] - Interface for the Ultrasonic Distance Sensor
//! * [`IRSensor`] - Interface for the IR Sensors on the Autonomous Vehicle
//! * [`AnalogDeviceController`] - Controller for analog devices like the IR Sensors, but this can be adapted to support other analog devices
//! * [`ServoMotorController`] - Controller for all External Motors. Cannot be used at the same as external device controllers like [`crate::devices::internal::OnboardSensorController`]
//! 
//! # Using External and Internal Controllers at the same time
//!
//! When looking at this crate something might stick out: "[`ServoMotorController`] Cannot be used at the same as external device controllers like [`crate::devices::internal::OnboardSensorController`]"
//! This can be a cause for concern because its not impossible to want to use both an external motor and the internal Magnometer at the same time.
//! 
//! Because both of the controllers use Board.TWIM0 its impossible for both of them to exist at the same time. Though there is a workaround: We can pass around Board.TWIM0 as 
//! its needed. [`ServoMotorController::into_magnometer_accelerometer`] does this for you. 
//! 
//! Unfortunately this does create code thats more verbose but its necessary to create code that compiles
//! 
//! 

mod servomotorcontroller;
mod acceleration_motor;
mod ir_sensor;
mod turning_motor;
mod analogdevicecontroller;
mod ultrasonic_distance_sensor;

pub use servomotorcontroller::ServoMotorController;

pub use acceleration_motor::acceleration_motor::AccelerationMotor;
pub use acceleration_motor::acceleration_motor::MotorState;
pub use acceleration_motor::acceleration_motor::ServoErrors;

pub use ir_sensor::IRSensor;
pub use ultrasonic_distance_sensor::UltrasonicDistanceSensorError;
pub use ultrasonic_distance_sensor::UltraSonicDistanceSensor;

// forgot to implement this one 
pub use turning_motor::TurningMotor;

pub use analogdevicecontroller::AnalogDeviceController;


