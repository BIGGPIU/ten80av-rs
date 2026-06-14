#[cfg(feature = "calcru-serial-standard")]
use crate::graphing::{IR_SENSOR_IDENTIFIER, MAGNOMETER_MESSAGE_IDENTIFIER, RADIO_MESSAGE_IDENTIFIER, ULTRASONIC_DISTANCE_SENSOR_IDENTIFIER};

pub trait SerialMessage {
    /// check if the identifier for a message checks out 
    fn identifier_matches(identifier:u8) -> bool;

    /// Serializes a message into the struct. useful for having your own custom serial messages.
    fn serialize_message(full_buffer:[u8;20]) -> Self;
}
#[cfg(feature = "calcru-serial-standard")]
pub struct UltraSonicDistanceSensorMessage {
    pub value:u32,
}

#[cfg(feature = "calcru-serial-standard")]
impl SerialMessage for UltraSonicDistanceSensorMessage {
    fn identifier_matches(identifier:u8) -> bool {
        identifier == ULTRASONIC_DISTANCE_SENSOR_IDENTIFIER
    }

    fn serialize_message(full_buffer:[u8;20]) -> Self {
        return Self {
            value: u32::from_le_bytes(
                [
                    full_buffer[7],
                    full_buffer[8],
                    full_buffer[9],
                    full_buffer[10]
                ]
            ),
        }
    }
}

pub struct IRSensorMessage {
    pub left_ir_value:i16,
    pub right_ir_value:i16,
}

#[cfg(feature = "calcru-serial-standard")]
impl SerialMessage for IRSensorMessage {
    fn identifier_matches(identifier:u8) -> bool {
        identifier == IR_SENSOR_IDENTIFIER
    }

    fn serialize_message(full_buffer:[u8;20]) -> Self {
        return Self {
            left_ir_value: i16::from_le_bytes([
                full_buffer[7],
                full_buffer[8]
            ]),
            right_ir_value: i16::from_le_bytes([
                full_buffer[9],
                full_buffer[10]
            ]),
        }
    }
}

#[cfg(feature = "calcru-serial-standard")]
pub struct RadioMessage {
    pub message:[u8;11]
}

#[cfg(feature = "calcru-serial-standard")]
impl SerialMessage for RadioMessage {
    fn identifier_matches(identifier:u8) -> bool {
        identifier == RADIO_MESSAGE_IDENTIFIER
    }

    fn serialize_message(full_buffer:[u8;20]) -> Self {
        let mut m = [0_u8;11];
        m.clone_from_slice(&full_buffer[6..17]);
        return Self {
            message: m,
        };
    }
}


#[cfg(feature = "calcru-serial-standard")]
pub struct MagnometerMessage {
    pub x_value:i16,
    pub y_value:i16,
    pub z_value:i16,
}

#[cfg(feature = "calcru-serial-standard")]
impl SerialMessage for MagnometerMessage {
    fn identifier_matches(identifier:u8) -> bool {
        MAGNOMETER_MESSAGE_IDENTIFIER == identifier
    }

    fn serialize_message(full_buffer:[u8;20]) -> Self {
        return Self {
            x_value: i16::from_le_bytes(
                [
                    full_buffer[7],
                    full_buffer[8],
                ]
            ),
            y_value: i16::from_le_bytes(
                [
                    full_buffer[9],
                    full_buffer[10],
                ]
            ),
            z_value: i16::from_le_bytes(
                [
                    full_buffer[11],
                    full_buffer[12],
                ]
            ),
        }
    }
}


