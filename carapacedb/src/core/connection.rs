use std::sync::{Weak};

use super::database::DuckDB;


pub struct Connection {
    db: Weak<DuckDB>,
    context: Box<ClientContext>,
    warning_cb: Option<Box<dyn Fn(&str)>>,
}
