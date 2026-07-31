use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// Struct for store.toml

#[derive(Debug, Serialize)]
pub struct Store {
    active: String,
    vaults: HashMap<String, String>,
}

// pub fn create_store() {}
