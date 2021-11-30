extern crate serde_json;
extern crate websocket;

mod functions;
use functions::*;

mod json_struct;
use json_struct::*;

use std::sync::mpsc::{SyncSender,Receiver,sync_channel};
use std::thread;


fn main() {
    let (sender,receiver) :(SyncSender<ServerResponse>,Receiver<ServerResponse>) = sync_channel(100);
    let mut request: ClientRequest = ClientRequest::new();
    request.channel = "spot.order_book".to_string();
    request.event = "subscribe".to_string();
    request.payload = Some(vec![
        "FIL_USDT".to_string(),
        "5".to_string(),
        "100ms".to_string(),
    ]);
    
    let mut thread_handles = vec![];
    let thread1 = thread::spawn(move ||{
        send_websocket_response(&request,sender);
    });
    thread_handles.push(thread1);

    let thread2 = thread::spawn(move ||{
        for mut response in receiver{
            response.to_number();
            if response.event == "update".to_string(){
                let result = response.result.unwrap();
                let asks = result.get(&"asks".to_string()).unwrap().as_arrayf64().unwrap();
                let bids = result.get(&"bids".to_string()).unwrap().as_arrayf64().unwrap();
                println!("asks are: {:?}",asks);
                println!("bids are: {:?}",bids);
            }else{
                println!("{:?}",response);
            }
        }
    });
    thread_handles.push(thread2);
    
    for handle in thread_handles{
        handle.join().unwrap();
    }

}
