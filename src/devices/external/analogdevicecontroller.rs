use microbit::hal::Saadc;
use crate::utils::serial::Serial;




pub struct AnalogDeviceController
{
    pub controller:Saadc
}

impl AnalogDeviceController {



    pub fn new(
        saadc:Saadc,
        serial: &mut crate::utils::serial::UartePort<microbit::pac::UARTE0>,
    ) -> AnalogDeviceController {
        Serial::write(serial, "Starting Analog Device Controller", crate::utils::serial::MessageSeverity::INFORMATIVE);

        return Self {
            controller: saadc,
        }
    }
}