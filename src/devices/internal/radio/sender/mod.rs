// vv no mixing and matching
#[cfg(not(feature = "calcru-serial-standard"))]
pub mod radio;
#[cfg(feature = "calcru-serial-standard")]
pub mod calvert_cruisers_radio;