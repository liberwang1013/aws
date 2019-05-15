use rusoto_core::Region;
use rusoto_ec2::{DescribeTagsRequest, Ec2, Ec2Client, Filter};
use std::collections::HashMap;
use std::fmt::Error;

pub fn get_ec2_tags(instance_id: &str) -> Result<HashMap<String, String>, Error> {

    let client = Ec2Client::new(Region::default());

    let mut next_token = None;

    let mut ec2_tags = HashMap::<String, String>::new();
    loop {
        let mut request = DescribeTagsRequest::default();
        let mut filter = Filter::default();
        filter.name = Some("resource-id".to_string());
        filter.values = Some(vec![String::from(instance_id)]);
        request.filters = Some(vec![filter]);
        request.next_token = next_token.clone();
        let result = client.describe_tags(request).sync();
        match result {
            Ok(rsp) => {
                next_token = rsp.next_token;
                let tags = rsp.tags.unwrap();
                for tag in tags {
                    let key = tag.key.unwrap();
                    let value = tag.value.unwrap();
                    ec2_tags.insert(key, value);

                }
            }

            Err(err) => {
                warn!("error: {}", err);
            }
        }

        if next_token.is_none() {
            return Ok(ec2_tags);
        }
    }
}