#[macro_use] extern crate log;

#[macro_use] extern crate error_chain;
extern crate rusoto_credential;
extern crate rusoto_core;
extern crate rusoto_ec2;
extern crate rusoto_s3;

extern crate futures;

pub mod ec2;
pub mod s3;
pub mod error;