#[macro_use]
extern crate influx_db_client;
extern crate redis;
extern crate rmp;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate time;
extern crate log;
extern crate rmp_serde as rmps;

use std::collections::HashMap;
use redis::Commands;
use redis::RedisResult;
use influx_db_client::{Client, Point, Points, Value, Precision, Error};
// use rmp::decode::read_str;
#[macro_use]
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use rmps::{decode};
// use rmp::decode::read_map;



// inter layer between redis and influxdb
struct Transport {
    redis: redis::Client,
    redis_connection: redis::Connection,
    influxdb: influx_db_client::Client,
    // msg: Msg,
}
#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct redis_data {
    timestamp: i64,
    table_name: String,
    fields: Vec<u8>,

}

// #[derive(Debug, PartialEq, Deserialize, Serialize)]
// struct Msg {
//     // measurement
//     meaurement: String,
//     // tags
//     tags: HashMap<String, Value>,
//     // fields
//     fields: HashMap<String, Value>,

// }

// // redis_msg : HasMmap<String, ValueType>
#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum ValueType {
    table_name(String),
    timestamp(i64),
    fields(HashMap<String, FieldsValueType>),
    heartbeat(bool),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum FieldsValueType {
    status(f32),
    num(i32),
    warning(String),
    tags(HashMap<String, String>),
}


impl Transport {
    // fn connect_redis(&self) -> redis::Connection{
    //     self.redis.get_connection() 
    // }

    fn get_redis_data(&self) -> RedisResult<HashMap<String, Vec<u8>>>{
        self.redis_connection.blpop("data_queue", 10)
    }

    // fn unpack_redis_data(data:RedisResult<&str>) -> Msg {
    //     //
    // }

    // fn send_influxdb(&self, msg: Msg, precision: Option<Precision>, rp: Option<&str>) -> Result<(), Error> {
    //     //
    // }
}







// #[derive(Debug)]

fn timestamp() -> i64 {
    let time = time::get_time();
    time.sec
}

fn main() {
    let transport = Transport{
        redis: redis::Client::open("redis://127.0.0.1/").unwrap(),
        redis_connection: redis::Client::open("redis://127.0.0.1/").unwrap().get_connection().unwrap(),
        influxdb: influx_db_client::Client::default().set_authentication("root", "root"),
    };
    // get redis data value
    // loop {
    
    fetch_redis_data(&transport);


    
    // let timestamp = timestamp();
    // let mut point = Point::new("test")
    //         .add_tag("eqpt_no", Value::String(String::from("PEC0-1900")))
    //         .add_field("data", Value::String(String::from("test data")))
    //         .add_timestamp(timestamp)
    //         .to_owned();
    
    // send_data_to_influxdb(&transport, point);
        
        
    // }
    // unpack redis data to influxdb format
    // let msg = transport.unpack_redis_data(redis_data);
    // send data to influxdb
    // let _ = transport.influx_db_client.write_point(msg, Some(Precision::Seconds), None).unwarp;

}

fn fetch_redis_data(transport:&Transport) {
    let mut redis_data = transport.get_redis_data().unwrap();
    // redis_data is a bunch of msgpack data
    println!("{:?}", redis_data);
    let mut out = [0u8;30];
    // println!("{:?}", redis_data.as_bytes());
    let key = "data_queue";
    let mut data = redis_data.get(key).unwrap();
    println!("{:?}", data);

    // let data = [0xaa, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65];
    // let read_str = read_str(&mut &data[..], &mut out);
    // println!("{}", read_str.unwrap())
    let mut de = Deserializer::new(&data[..]);
    let data_1:HashMap<String, String> = Deserialize::deserialize(&mut de).unwrap();
    println!("{:?}", data_1);

}

fn send_data_to_influxdb(transport:&Transport, point:Point) {
    let _ = transport.influxdb.write_point(point, Some(Precision::Seconds), None).unwrap();

}

