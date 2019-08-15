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

pub fn create_export_task(log_group_name: &str,
                          from: i64,
                          to: i64,
                          destination: &str,
                          destination_prefix: &str)
                          -> Result<String> {
    let client = CloudWatchLogsClient::new(rusoto_core::Region::default());

    let mut input = CreateExportTaskRequest::default();
    input.destination        = String::from(destination);
    input.destination_prefix = Some(String::from(destination_prefix));
    input.log_group_name     = String::from(log_group_name);
    input.from               = from;
    input.to                 = to;

    let rsp = client.create_export_task(input).sync()?;
    return Ok(rsp.task_id.unwrap());
}
