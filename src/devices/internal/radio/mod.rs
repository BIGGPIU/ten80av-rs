
pub mod reciever;
pub mod sender;

pub use reciever::radio::Radio as RecieverRadio;
pub use sender::radio::Radio as SenderRadio;
pub use reciever::radio::RadioError as RecieverError;
pub use sender::radio::Radio as SenderError;