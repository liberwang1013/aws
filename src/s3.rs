use rusoto_core::Region;
use rusoto_s3::*;

use futures::{Stream, Future};

use crate::error::*;
use crate::core::get_region;

pub fn list_objects(bucket: &str, prefix: Option<String>) -> Result<Vec<String>> {
    let mut objects = Vec::<String>::new();

    let mut continuation_token = None;
    let _provider = rusoto_credential::ChainProvider::new();
    let client = S3Client::new(Region::default());
    let mut list_request = ListObjectsV2Request::default();
    list_request.bucket = bucket.to_owned();
    list_request.prefix = prefix;
    loop {
        list_request.continuation_token = continuation_token;

        let output = client.list_objects_v2(list_request.clone()).sync()?;
        continuation_token = output.next_continuation_token;
        if let Some(rsp_objects) = output.contents {
            debug!("get {} objects", rsp_objects.len());
            for object in rsp_objects {
                objects.push(object.key.unwrap());
            }
        }
        if continuation_token.is_none() {
            return Ok(objects);
        }
    }
}

pub fn get_object(bucket: &str, object: &str) -> Result<Vec<u8>> {
    let client = S3Client::new(Region::default());

    let mut get_request = GetObjectRequest::default();
    get_request.bucket = bucket.to_owned();
    get_request.key = object.to_owned();

    let output = client.get_object(get_request).sync()?;

    if let Some(stream) = output.body {
        let body = stream.concat2().wait()?;
        return Ok(body);
    }

    return Err(ErrorKind::NoObjectError.into());
}
