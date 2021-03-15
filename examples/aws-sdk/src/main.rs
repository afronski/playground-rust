use rusoto_core::Region;
use rusoto_kinesisvideo::{KinesisVideo, KinesisVideoClient, ListStreamsInput};

#[tokio::main]
async fn main() {
    let client = KinesisVideoClient::new(Region::default());
    let mut token: String = String::new();

    println!("Available streams:");

    loop {
        let list_streams_input: ListStreamsInput = ListStreamsInput {
            max_results: Some(25),
            next_token: match &mut token {
                str if str.is_empty() => None,
                rest => Some(rest.to_string()),
            },
            stream_name_condition: None,
        };

        match client.list_streams(list_streams_input).await {
            Ok(output) => {
                match output.stream_info_list {
                    Some(stream_infos) => {
                        for stream_info in stream_infos {
                            match stream_info.stream_arn {
                                Some(stream_arn) => println!("{}", stream_arn),
                                None => (),
                            }
                        }
                    }
                    None => println!("No streams!"),
                }

                if let None = output.next_token {
                    break;
                }

                token = output.next_token.unwrap();
            },
            Err(error) => {
                println!("Error: {:?}", error);
                break;
            }
        }
    }
}
