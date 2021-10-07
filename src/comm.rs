
#[cfg(debug_assertions)]
pub const MONGO_ADDRESS: &'static str = "mongodb://192.168.0.54:27017";

#[cfg(not(debug_assertions))]
pub const MONGO_ADDRESS: &'static str = "mongodb://mongo:27017";
