extern crate serde_json;

use std::sync::mpsc::SyncSender;
use std::time::Duration;

use websocket::client::ClientBuilder;
use websocket::OwnedMessage;

use super::json_struct::*;

pub fn send_websocket_response(request: &ClientRequest, sender: SyncSender<ServerResponse>) {
    let mut client = ClientBuilder::new("wss://api.gateio.ws/ws/v4/")
        .unwrap()
        .connect_secure(None)
        .unwrap();

    //set read and write TcpStream timeout to stop block indefinitely
    client
        .stream_ref()
        .get_ref()
        .set_read_timeout(Some(Duration::from_millis(10000)))
        .unwrap();
    client
        .stream_ref()
        .get_ref()
        .set_write_timeout(Some(Duration::from_millis(10000)))
        .unwrap();

    let string_request = serde_json::to_string(request).unwrap();
    let message_request = &OwnedMessage::Text(string_request);
    let mut result = ServerResponse::new();
    client.send_message(message_request).unwrap();

    loop {
        
        let message = match client.recv_message() {
            Ok(data) => data,
            //在实际使用过程中，由于网络或者其他不明的服务器原因，会导致client阻塞或者报错，
            //此处的目的是当出现会导致panic的报错的时候，重新构建链接；
            Err(e) => {
                println!(
                    "Error happend in send_websocket_response function\nerror: {:?}",
                    e
                );
                client.shutdown().unwrap();
                client = ClientBuilder::new("wss://api.gateio.ws/ws/v4/")
                    .unwrap()
                    .connect_secure(None)
                    .unwrap();
                client
                    .stream_ref()
                    .get_ref()
                    .set_read_timeout(Some(Duration::from_millis(10000)))
                    .unwrap();
                client
                    .stream_ref()
                    .get_ref()
                    .set_write_timeout(Some(Duration::from_millis(10000)))
                    .unwrap();
                client.send_message(message_request).unwrap();
                continue;
            }
        };

        //根据服务端返回的信息类型处理数据；
        match message {
            OwnedMessage::Close(_) => {
                client.shutdown().unwrap();
                client = ClientBuilder::new("wss://api.gateio.ws/ws/v4/")
                    .unwrap()
                    .connect_secure(None)
                    .unwrap();
                client
                    .stream_ref()
                    .get_ref()
                    .set_read_timeout(Some(Duration::from_millis(10000)))
                    .unwrap();
                client
                    .stream_ref()
                    .get_ref()
                    .set_write_timeout(Some(Duration::from_millis(10000)))
                    .unwrap();
                client.send_message(message_request).unwrap();
            }
            OwnedMessage::Ping(data) => {
                client.send_message(&OwnedMessage::Pong(data)).unwrap();
            }

            OwnedMessage::Text(data) => {
                //String &String 
                result = serde_json::from_str(&data.as_str()).unwrap();
                //result.to_number();
                sender.send(result).unwrap();
            }

            OwnedMessage::Binary(data) => {
                result  = serde_json::from_slice(&data).unwrap();
                //result.to_number();
                sender.send(result).unwrap();
            }
            OwnedMessage::Pong(_) => {}
        }
    }
}
