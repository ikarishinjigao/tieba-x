use network::{api_client::ApiClient, requests::GetThreadsRequest};

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
