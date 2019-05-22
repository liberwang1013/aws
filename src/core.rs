use rusoto_core::Region;

use std::env;
use std::net::Shutdown::Read;

const AWS_CUSTOM_REGION: &str = "AWS_CUSTOM_REGION";
const AWS_CUSTOM_ENDPOINT: &str = "AWS_CUSTOM_ENDPOINT";

pub fn get_region() -> Region {
    let region = env::var(AWS_CUSTOM_REGION);
    let endpoint = env::var(AWS_CUSTOM_ENDPOINT);
    if endpoint.is_ok() {
        return Region::Custom {
            name: region.unwrap_or("us-east-1".to_string()),
            endpoint: endpoint.unwrap()
        };
    }
    Region::default()
}