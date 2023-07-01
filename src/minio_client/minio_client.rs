use crate::utils::config::APPCONFIG;
use awsregion::Region;
use s3::creds::Credentials;
use s3::Bucket;

pub async fn create_minio() -> Bucket {
    let conf = &APPCONFIG.minio;
    let access_key = conf.clone().access_key;
    let secret_key = conf.clone().secret_key;
    let bucket_name = conf.clone().bucket_name;
    let region = conf.clone().region;
    let endpoint = conf.clone().endpoint;
    let credentials =
        Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap();
    let name = &bucket_name;
    let bucket = Bucket::new(name, Region::Custom { region, endpoint }, credentials)
        .unwrap()
        .with_path_style();
    bucket
}
