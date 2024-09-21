use network::{
    api_client::ApiClient,
    requests::{GetThreadsPersonalizedRequest, GetThreadsRequest},
};

#[tokio::test]
async fn test_get_threads_request() {
    let client = ApiClient::new();

    let request = GetThreadsRequest {
        forum_name: "amd".to_owned(),
        ..Default::default()
    };
    let response = client.get_threads(request).await.unwrap();

    assert_eq!(4420, response.forum_id);
    assert_eq!("amd", response.forum_name);
}

#[tokio::test]
async fn test_get_threads_personalized_request() {
    let client = ApiClient::new();

    let request = GetThreadsPersonalizedRequest::default();
    let _ = client.get_threads_personalized(request).await.unwrap();
}
