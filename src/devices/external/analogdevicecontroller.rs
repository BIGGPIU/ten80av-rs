use microbit::hal::Saadc;
use crate::utils::serial::Serial;




/// Controller for external Analog Devices
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

    pub fn new_nolog(
        saadc:Saadc,
    ) -> AnalogDeviceController {
     
        return Self {
            controller: saadc,
        }
    }
}