extern crate serde;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ClientRequest {
    pub time: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    pub channel: String,

    pub event: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
}

impl ClientRequest {
    pub fn new() -> ClientRequest {
        ClientRequest {
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            id: Some(0),
            channel: String::new(),
            event: String::new(),
            payload: Some(Vec::new()),
            auth: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Auth {
    pub method: String,

    #[serde(rename = "KEY")]
    pub key: String,

    #[serde(rename = "SIGN")]
    pub sign: String,
}

impl Auth {
    pub fn new() -> Auth {
        Auth {
            method: String::new(),
            key: String::new(),
            sign: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerResponse {
    pub time: u64,

    pub channel: String,

    pub event: String,

    #[serde(default)]
    pub error: Option<ResponseError>,

    #[serde(default)]
    pub result: Option<HashMap<String, Value>>,
}

impl ServerResponse {
    pub fn to_number(&mut self) {
        match &mut self.result {
            None => {
                return;
            }
            Some(hash_map) => {
                for (_, value) in hash_map.iter_mut() {
                    value.to_float64();
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Value {
    Float(f64),

    ValueString(String),

    Array(Vec<[String; 2]>),

    Arrayf64(Vec<[f64; 2]>),

    Bool(bool),
}

impl Value {

    //将需要的string类型结果转换为float64类型；
    fn to_float64(&mut self) {
        match self {
            Self::ValueString(s) => match s.parse::<f64>() {
                Ok(n) => {
                    *self = Self::Float(n);
                }
                Err(_) => {
                    return;
                }
            },

            Self::Array(array) => {
                let mut vec = Vec::new();
                for n in array {
                    let mut num_array: [f64; 2] = [0.0, 0.0];
                    for i in 0..2 {
                        let result = n[i].parse::<f64>().unwrap();
                        num_array[i] = result;
                    }
                    vec.push(num_array);
                }
                *self = Self::Arrayf64(vec);
            }
            _ => {}
        }
    }
}

impl ServerResponse {
    pub fn new() -> ServerResponse {
        ServerResponse {
            time: 0,
            channel: String::new(),
            event: String::new(),
            error: None,
            result: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseError {
    pub code: i8,

    pub message: String,
}
