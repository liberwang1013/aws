error_chain! {
    foreign_links {
        ListObjectsV2Error(rusoto_core::RusotoError<rusoto_s3::ListObjectsV2Error>);
        GetObjectError(rusoto_core::RusotoError<rusoto_s3::GetObjectError>);

        DescribeTagsError(rusoto_core::RusotoError<rusoto_ec2::DescribeTagsError>);

        DescribeLogGroupsError(rusoto_core::RusotoError<rusoto_logs::DescribeLogGroupsError>);
        CreateExportTaskError(rusoto_core::RusotoError<rusoto_logs::CreateExportTaskError>);
        DescribeExportTasksError(rusoto_core::RusotoError<rusoto_logs::DescribeExportTasksError>);
        IoError(std::io::Error);
    }

    errors {
        AwsError {
            display("Aws Error")
        }

        NoObjectError {
            description("no object error")
            display("S3 Error, no object")
        }
    }
}
