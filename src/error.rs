
error_chain! {
    foreign_links {
        ListObjectsV2Error(rusoto_core::RusotoError<rusoto_s3::ListObjectsV2Error>);
        GetObjectError(rusoto_core::RusotoError<rusoto_s3::GetObjectError>);

        DescribeTagsError(rusoto_core::RusotoError<rusoto_ec2::DescribeTagsError>);
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