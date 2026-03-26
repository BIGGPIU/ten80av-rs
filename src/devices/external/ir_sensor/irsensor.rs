use microbit::hal::saadc::Channel;

use crate::utils::serial::Serial;

pub struct IRSensor<T>
where T: microbit::hal::saadc::Channel
{
    /// port that recieves output from sensor (the same as output_port_writer)
    channel:T,
    offset:i16,
}



impl<T:Channel> IRSensor<T> {



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




    pub fn measure(&mut self, controller: &mut crate::devices::external::analogdevicecontroller::AnalogDeviceController) -> i16{
        controller.controller.read_channel(&mut self.channel).unwrap() - self.offset
    }
}