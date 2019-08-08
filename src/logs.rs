use rusoto_logs::*;
use crate::error::Result;

pub fn describe_log_groups(prefix: &str) -> Result<Vec<String>> {
    let client = CloudWatchLogsClient::new(rusoto_core::Region::default());
    let mut input = DescribeLogGroupsRequest::default();
    input.log_group_name_prefix = Some(String::from(prefix));
    input.next_token = None;
    let mut groups = Vec::<String>::new();
    loop {
        let rsp = client.describe_log_groups(input.clone()).sync()?;
        for item in rsp.log_groups.unwrap() {
            groups.push(item.log_group_name.unwrap());
        }
        if rsp.next_token.is_none() {
            return Ok(groups);
        }

        input.next_token = Some(rsp.next_token.unwrap());
    }
    return Ok(groups);
}
