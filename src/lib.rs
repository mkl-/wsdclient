#![feature(custom_attribute)]
#![feature(plugin)]

extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate reqwest;

pub mod options;
pub mod client;
pub mod config;

pub fn get_help() {
    println!("Hello from wsdclient");
}