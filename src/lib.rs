#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate adom;

pub mod config;
pub mod controllers;
pub mod database;
pub mod errors;
pub mod migrations;
pub mod models;
