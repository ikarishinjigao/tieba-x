use network::{ApiClient, GetThreadsRequest};

#[tokio::test]
async fn test_get_threads_request() {
    let client = ApiClient::new();

    let reuqest = GetThreadsRequest {
        forum_name: "amd".to_owned(),
        ..Default::default()
    };
    let response = client.request(reuqest).await.unwrap();

    assert_eq!(4420, response.forum_id);
    assert_eq!("amd", response.forum_name);
}
