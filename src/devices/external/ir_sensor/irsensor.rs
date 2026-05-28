use microbit::hal::saadc::Channel;

use crate::utils::{IRSensorMessage, serial::Serial};

/// Controller for External IR Sensors
pub struct IRSensor<T>
where T: microbit::hal::saadc::Channel
{
    /// port that recieves output from sensor (the same as output_port_writer)
    channel:T,
    offset:i16,
}




impl<T:Channel> IRSensor<T> {



    /// Make a new IR Sensor 
    /// 
    /// # Example:
    /// ```rust
    /// let mut right_ir_sensor = IRSensor::new(board.edge.e01.into_floating_input(), 17500, &mut serial);
    /// let mut left_ir_sensor = IRSensor::new(board.edge.e00.into_floating_input(), 17500, &mut serial);
    /// ```
    pub fn new(
        output_port:T,
        offset:i16,
        serial: &mut crate::utils::serial::UartePort<microbit::pac::UARTE0>
    ) -> Self {
        Serial::write(serial, "Starting IR Sensor", crate::utils::serial::MessageSeverity::INFORMATIVE);

        return Self {
            channel:output_port,
            offset
        }
    }

    // Make a new IR Sensor
    pub fn new_nolog(
        output_port:T,
        offset:i16,
    ) -> Self {
        
        return Self {
            channel:output_port,
            offset
        }
    }




    /// Measure the raw value of the IR Sensor minus the offset
    pub fn measure(&mut self, controller: &mut crate::devices::external::analogdevicecontroller::AnalogDeviceController) -> i16{
        controller.controller.read_channel(&mut self.channel).unwrap() - self.offset
    }

    /// Measure the raw value of the IR sensor
    pub fn measure_no_offset(&mut self, controller: &mut crate::devices::external::analogdevicecontroller::AnalogDeviceController) -> i16{
        controller.controller.read_channel(&mut self.channel).unwrap()
    }

    
}