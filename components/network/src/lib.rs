// use anyhow::Result;
// use proto::Message;
// use reqwest::header;
// use reqwest::multipart::Part;

// #[tokio::main]
// async fn main() -> Result<()> {
//     let mut request = proto::frs_page::FrsPageReqIdl::default();

//     let mut common_request = proto::CommonReq::default();
//     common_request.client_type = 2;
//     common_request.client_version = "12.65.1.1".to_string();

//     let mut data = proto::frs_page::DataReq::default();
//     data.common = Some(common_request);
//     data.kw = "amd".to_string();
//     data.pn = 1;
//     data.rn = 5;
//     data.sort_type = 1;
//     data.is_good = 0;
//     request.data = Some(data);

//     let mut buf = vec![];
//     request.encode(&mut buf)?;

//     let part = Part::bytes(buf).file_name("data");
//     let file = reqwest::multipart::Form::new().part("data", part);

//     let response = reqwest::Client::new()
//         .post("https://tiebac.baidu.com/c/f/frs/page?cmd=301001&format=protobuf")
//         .header("x_bd_data_type", "protobuf")
//         .header(
//             header::USER_AGENT,
//             format!("tieba/{}", proto::TIEBA_CLIENT_VERSION),
//         )
//         .header(header::CACHE_CONTROL, "no-cache")
//         .header(header::CONNECTION, "keep-alive")
//         .multipart(file)
//         .send()
//         .await?;

//     let body = response.bytes().await?;

//     let result = proto::frs_page::FrsPageResIdl::decode(body)?;

//     // println!("Status: {}", response.status());
//     // println!("Headers:\n{:#?}", response.headers());
//     println!("Response:\n{:#?}", result);

//     Ok(())
// }

mod api_client;
mod api_error;
mod api_request;
mod common;
mod enums;
mod error;
mod requests;
mod responses;

use api_request::ApiProtobufRequest;

pub use api_client::ApiClient;
pub use error::Error;
pub use requests::GetThreadsRequest;
