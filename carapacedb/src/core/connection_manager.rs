use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use super::connection::Connection;

pub struct ConnectionManager {
    connections: Mutex<HashSet<Arc<Connection>>>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Mutex::new(HashSet::new()),
        }
    }

    pub fn add_connection(&self, conn: Arc<Connection>) {
        let mut connections = self.connections.lock().unwrap();
        connections.insert(conn);
    }

    pub fn remove_connection(&self, conn: &Arc<Connection>) {
        let mut connections = self.connections.lock().unwrap();
        connections.remove(conn);
    }

    pub fn scan<F>(&self, callback: F)
    where
        F: Fn(&Arc<Connection>),
    {
        let connections = self.connections.lock().unwrap();
        for conn in connections.iter() {
            callback(conn);
        }
    }
}

impl Drop for ConnectionManager {
    fn drop(&mut self) {
        let mut connections = self.connections.lock().unwrap();
        connections.clear();
    }
}
