use crate::devices::internal::onboardsensorcontroller::OnboardSensorController;

pub struct Magnometer {

}


impl Magnometer {



    pub fn read_magnometer(controller:&mut  OnboardSensorController) -> Option<(i16, i16, i16)> {
        match controller.controller.magnetic_field() {
            Ok(x) => {
                return Some(x.xyz_unscaled());
            },
            Err(_) => {
                return None
            },
        }
    }
}