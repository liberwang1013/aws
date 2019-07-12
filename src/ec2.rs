use rusoto_core::Region;
use rusoto_ec2::{DescribeTagsRequest, Ec2, Ec2Client, Filter};
use std::collections::HashMap;

use crate::error::*;

pub fn get_ec2_tags(instance_id: &str) -> Result<HashMap<String, String>> {

    let client = Ec2Client::new(Region::ApEast1);

    let mut next_token = None;

    let mut request = DescribeTagsRequest::default();
    let mut filter = Filter::default();
    filter.name = Some("resource-id".to_string());
    filter.values = Some(vec![String::from(instance_id)]);
    request.filters = Some(vec![filter]);

    let mut ec2_tags = HashMap::<String, String>::new();
    loop {
        request.next_token = next_token;
        let rsp = client.describe_tags(request.clone()).sync()?;
        let tags = rsp.tags.unwrap();
        for tag in tags {
            let key = tag.key.unwrap();
            let value = tag.value.unwrap();
            ec2_tags.insert(key, value);
        }
        next_token = rsp.next_token;
        if next_token.is_none() {
            return Ok(ec2_tags)
        }
    }
}
