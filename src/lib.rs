#![feature(custom_attribute)]
#![feature(plugin)]

extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

pub mod types;
pub mod client;
pub mod config;
