pub(crate) const IR_SENSOR_IDENTIFIER:u8 = 0;
pub(crate) const ULTRASONIC_DISTANCE_SENSOR_IDENTIFIER:u8 = 1;
pub(crate) const RADIO_MESSAGE_IDENTIFIER:u8 = 2;
pub(crate) const MAGNOMETER_MESSAGE_IDENTIFIER:u8 = 3;

/// trait for creating your own microbit messages
pub trait MicrobitMessageFormat {
    /// byte to identify what kind of message is being sent
    /// do not use identifiers between 0 and 10
    fn get_identification(&self) -> u8;

    /// create a slice to send over to the device your microbit is plugged into
    fn create_message_slice(&self) -> [u8;16];

}


pub struct UltraSonicDistanceSensorMessage {
    identifier:u8,
    pub value:u32
}

impl UltraSonicDistanceSensorMessage {
    pub fn new() -> Self {
        return Self {
            identifier: ULTRASONIC_DISTANCE_SENSOR_IDENTIFIER,
            value: 0
        }
    }

    pub fn new_with_values(val:u32) -> Self {
        return Self {
            identifier: ULTRASONIC_DISTANCE_SENSOR_IDENTIFIER,
            value: val,
        }
    }
}

impl MicrobitMessageFormat for UltraSonicDistanceSensorMessage {
    fn get_identification(&self) -> u8 {
        self.identifier
    }

    fn create_message_slice(&self) -> [u8;16] {
        [
        0x41,
        0x56,
        self.identifier,
        self.value.to_le_bytes()[0],
        self.value.to_le_bytes()[1],
        self.value.to_le_bytes()[2],
        self.value.to_le_bytes()[3],
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x00,
        0x56,
        0x41,    
        ]
    }
}

pub struct IRSensorMessage {
    identifier:u8,
    pub left_ir_value:i16,
    pub right_ir_value:i16,
}

impl IRSensorMessage {
    pub fn new() -> Self {
        return Self {
            identifier: IR_SENSOR_IDENTIFIER,
            left_ir_value: 0,
            right_ir_value: 0,
        }
    }

    pub fn new_with_values(left_ir_value:i16,right_ir_value:i16) -> Self {
        return Self {
            identifier: IR_SENSOR_IDENTIFIER,
            left_ir_value,
            right_ir_value,
        }
    }
}

impl MicrobitMessageFormat for IRSensorMessage {
    fn get_identification(&self) -> u8 {
        self.identifier
    }
    
    /// make sure that 16726 or 22081 is never sent
    fn create_message_slice(&self) -> [u8;16] {
        [
            0x41,
            0x56,
            self.identifier,
            self.left_ir_value.to_le_bytes()[0],
            self.left_ir_value.to_le_bytes()[1],
            self.right_ir_value.to_le_bytes()[0],
            self.right_ir_value.to_le_bytes()[1],
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x56,
            0x41,    
        ]
    }
}


pub struct RadioMessage {
    identifier:u8,
    
    // there are 5 bytes out of the 16 bytes used for identification and starting/stopping. technically this should allow messages to be of infinite length
    // but im not letting you do that 
    pub message:[u8;11]
}

impl RadioMessage {
    pub fn new() -> Self {
        return Self {
            identifier: RADIO_MESSAGE_IDENTIFIER,
            message: [0_u8;11],
        }
    }
}


impl MicrobitMessageFormat for RadioMessage {
    fn get_identification(&self) -> u8 {
        return self.identifier
    }

    fn create_message_slice(&self) -> [u8;16] {
        [
            0x41,
            0x56,
            self.identifier,
            self.message[0],
            self.message[1],
            self.message[2],
            self.message[3],
            self.message[4],
            self.message[5],
            self.message[6],
            self.message[7],
            self.message[8],
            self.message[9],
            self.message[10],
            0x56,
            0x41,    
        ]
    }
}


pub struct MagnometerMessage {
    identifier:u8,
    pub x_value:i16,
    pub y_value:i16,
    pub z_value:i16
}

impl MagnometerMessage {
    pub fn new() -> Self {
        Self {
            identifier: MAGNOMETER_MESSAGE_IDENTIFIER,
            x_value: 0,
            y_value: 0,
            z_value: 0,
        }
    }
}

impl MicrobitMessageFormat for MagnometerMessage {
    fn get_identification(&self) -> u8 {
        self.identifier
    }

    fn create_message_slice(&self) -> [u8;16] {
        [
            0x41,
            0x56,
            self.identifier,
            self.x_value.to_le_bytes()[0],
            self.x_value.to_le_bytes()[1],
            self.y_value.to_le_bytes()[0],
            self.y_value.to_le_bytes()[1],
            self.z_value.to_le_bytes()[0],
            self.z_value.to_le_bytes()[1],
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x56,
            0x41,    
        ]
    }
}


// pub enum MicrobitMessage<T:MicrobitMessageFormat>
// {
//     IRSensorMessage(IRSensorMessageStruct),
//     UltraSonicDistanceSensorMessage(IRSensorMessageStruct),
//     RadioMessage(IRSensorMessageStruct),
//     MagnometerMessage(IRSensorMessageStruct),
//     Custom(T)
// }