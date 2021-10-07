
use mongodb::{Collection, Database};

#[derive(Clone, Debug)]
pub struct AppContext {
    pub db: Database,
}
