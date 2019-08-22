use rusoto_logs::*;
use crate::error::Result;
use futures::prelude::*;

pub enum ExportTaskStatus {
    COMPLETED,
    PENDING,
}

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

    let input = CreateExportTaskRequest{
        destination: String::from(destination),
        destination_prefix: Some(String::from(destination_prefix)),
        log_group_name: String::from(log_group_name),
        from: from,
        to: to,
        ..Default::default()
    };

    let rsp = client.create_export_task(input).sync()?;
    return Ok(rsp.task_id.unwrap());
}

pub fn describe_export_task(id: &str) -> Result<Option<String>> {

    let client = CloudWatchLogsClient::new(rusoto_core::Region::default());

    let input = DescribeExportTasksRequest{
        task_id: Some(String::from(id)),
        ..Default::default()
    };

    let rsp = client.describe_export_tasks(input).sync()?;

    match rsp.export_tasks {
        Some(tasks) => {
            if tasks.len() != 0 {
                let task = tasks.first().unwrap();
                return Ok(task.to_owned().status.unwrap().code);
            }
            else {
                return Ok(None);
            }
        }

        None => {
            return Ok(None);
        }
    }

    return Ok(None);
}