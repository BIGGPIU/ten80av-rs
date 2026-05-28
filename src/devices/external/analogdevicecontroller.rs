use microbit::{hal::{Saadc, Timer}, pac::TIMER0};
use crate::{devices::external::{IRSensor, UltraSonicDistanceSensor, UltrasonicDistanceSensorError}, utils::{IRSensorMessage, UltraSonicDistanceSensorMessage, serial::Serial}};




/// Controller for external Analog Devices
pub struct AnalogDeviceController
{
    pub controller:Saadc
}

impl AnalogDeviceController {



    /// Create a new Analog Device Controller.
    /// 
    /// This requires the on board SAADC peripheral which can be gotten with 
    /// ```rust
    /// let saadc_config = SaadcConfig::default();
    /// let mut saadc = Saadc::new(board.ADC, saadc_config);
    /// ```
    pub fn new(
        saadc:Saadc,
        serial: &mut crate::utils::serial::UartePort<microbit::pac::UARTE0>,
    ) -> AnalogDeviceController {
        Serial::write(serial, "Starting Analog Device Controller", crate::utils::serial::MessageSeverity::INFORMATIVE);

        return Self {
            controller: saadc,
        }
    }


    /// Create a new Analog Device Controller.
    /// 
    /// This requires the on board SAADC peripheral which can be gotten with 
    /// ```rust
    /// let saadc_config = SaadcConfig::default();
    /// let mut saadc = Saadc::new(board.ADC, saadc_config);
    /// ```
    pub fn new_nolog(
        saadc:Saadc,
    ) -> AnalogDeviceController {
     
        return Self {
            controller: saadc,
        }
    }


    /// Creates a message with the IR Sensors Current value (including the offset). this message can then be used to be sent over the Radio or to a computer through the
    /// serial (UART)
    pub fn create_ir_sensor_message
    <
        Port1: microbit::hal::saadc::Channel,
        Port2:microbit::hal::saadc::Channel
    >
    (&mut self, left_ir_sensor:&mut IRSensor<Port1>, right_ir_sensor:&mut IRSensor<Port2>) -> IRSensorMessage {
        return IRSensorMessage::new_with_values(
        left_ir_sensor.measure(self), 
        right_ir_sensor.measure(self)    
        )
    }


    /// Creates a message with the UDS Current value (not in centimeters). this message can then be used to be sent over the Radio or to a computer through the
    /// serial (UART)
    pub fn create_ultrasonic_distance_sensor_message(uds:&mut UltraSonicDistanceSensor,timer:&mut Timer<TIMER0>) -> Result<UltraSonicDistanceSensorMessage,UltrasonicDistanceSensorError> {
        uds.create_ultrasonic_distance_sensor_message(timer)
    }
    
}


