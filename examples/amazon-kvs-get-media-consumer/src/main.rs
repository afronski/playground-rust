use std::fs::File;
use std::io::Write;

use bytes::Bytes;
use chrono::offset::Utc;

use rusoto_core::Region;
use rusoto_kinesisvideo::{KinesisVideo, KinesisVideoClient, ListStreamsInput, GetDataEndpointInput};
use rusoto_kinesis_video_media::{KinesisVideoMedia, KinesisVideoMediaClient, GetMediaInput, StartSelector};

#[tokio::main]
async fn main() {
    let kinesis_video_client = KinesisVideoClient::new(Region::default());
    let mut next_token: Option<String> = None;

    println!("Available streams:");

    let mut arn: Option<String> = None;

    loop {
        let list_streams_input = ListStreamsInput {
            max_results: Some(25),
            next_token,
            stream_name_condition: None,
        };

        match kinesis_video_client.list_streams(list_streams_input).await {
            Ok(output) => {
                match output.stream_info_list {
                    Some(stream_infos) => {
                        for stream_info in stream_infos {
                            match stream_info.stream_arn {
                                Some(stream_arn) => {
                                    if stream_arn.contains("webcam-test") {
                                        arn = Some(stream_arn.clone())
                                    }

                                    println!("{}", stream_arn);
                                }
                                None => (),
                            }
                        }
                    },
                    None => println!("No streams!"),
                }

                if let None = output.next_token {
                    break;
                }

                next_token = output.next_token;
            },
            Err(error) => {
                println!("ListStreams Error: {:?}", error);
                break;
            }
        }
    }

    match arn {
        None => println!("Missing an expected 'webcam-test'. Stopping..."),
        Some(ref arn_value) => {
            println!("GetMedia from {}", arn_value);

            let get_data_endpoint_input = GetDataEndpointInput {
                api_name: String::from("GET_MEDIA"),
                stream_arn: arn.clone(),
                ..Default::default()
            };

            match kinesis_video_client.get_data_endpoint(get_data_endpoint_input).await {
                Ok(output) => {
                    println!("DATA_ENDPOINT: {:?}", output.data_endpoint);

                    let region = Region::Custom {
                        name: String::from("eu-west-1"),
                        endpoint: output.data_endpoint.unwrap_or_else(|| String::from(""))
                    };

                    let kinesis_video_media_client = KinesisVideoMediaClient::new(region);

                    loop {
                        let get_media_input = GetMediaInput {
                            start_selector: StartSelector {
                                start_selector_type: String::from("NOW"),
                                ..Default::default()
                            },
                            stream_arn: arn.clone(),
                            ..Default::default()
                        };

                        match kinesis_video_media_client.get_media(get_media_input).await {
                            Ok(output) => {
                                let time = Utc::now();
                                let filename = format!("result-{}.mkv", time.format("%s").to_string());

                                let mut file = File::create(filename.clone()).expect("Could not create file!");

                                file
                                    .write_all(&output.payload.unwrap_or_else(|| Bytes::from("")))
                                    .expect("Payload should be writable!");

                                println!("FILE {} SAVED", filename);
                            }
                            Err(error) => {
                                println!("GetMedia Error: {:?}", error);
                                break;
                            }
                        }
                    }
                },

                Err(error) => {
                    println!("GetDataEndpoint Error: {:?}", error);
                }
            }
        }
    }
}
