#[macro_use] extern crate log;

#[macro_use] extern crate error_chain;
extern crate rusoto_credential;
extern crate rusoto_core;
extern crate rusoto_ec2;

pub mod ec2;
mod error;