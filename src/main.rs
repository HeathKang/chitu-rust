#[macro_use]
extern crate influx_db_client;
extern crate redis;
// extern crate rmp;
extern crate rmp_serialize;
extern crate rustc_serialize;


use std::collections::HashMap;
use redis::Commands;
use redis::RedisResult;
use influx_db_client::{Value, Precision, Error};
use rustc_serialize::Decodable;
use rmp_serialize::Decoder;


// inter layer between redis and influxdb
struct Transport {
    redis: redis::Client,
    redis_connection: redis::Connection,
    influxdb: influx_db_client::Client,
    // msg: Msg,
}

impl Transport {
    // fn connect_redis(&self) -> redis::Connection{
    //     self.redis.get_connection() 
    // }

    fn get_redis_data(&self) -> RedisResult<Vec<i32>>{
        self.redis_connection.blpop("data_queue", 10)
    }

    // fn unpack_redis_data(data:RedisResult<&str>) -> Msg {
    //     //
    // }

    // fn send_influxdb(&self, msg: Msg, precision: Option<Precision>, rp: Option<&str>) -> Result<(), Error> {
    //     //
    // }
}


struct Msg {
    // measurement
    meaurement: String,
    // tags
    tags: HashMap<String, Value>,
    // fields
    fields: HashMap<String, Value>,
    // timestamp
    timestamp: Option<i64>,
}

// redis_msg : HasMmap<String, ValueType>

enum ValueType {
    table_name(String),
    timestamp(i64),
    fields(HashMap<String, FieldsValueType>),
}

enum FieldsValueType {
    status(f32),
    num(i32),
    warning(String),
}

fn main() {
    let transport = Transport{
        redis: redis::Client::open("redis://127.0.0.1/").unwrap(),
        redis_connection: redis::Client::open("redis://127.0.0.1/").unwrap().get_connection().unwrap(),
        influxdb: influx_db_client::Client::default().set_authentication("root", "root"),
    };
    // get redis data value
    loop {
        let  redis_data = transport.get_redis_data().unwrap();
        // redis_data is a bunch of msgpack data
        let mut decoder = Decoder::new(redis_data);
        let data: HashMap<String, ValueType> = Decodable::decode(&mut decoder).unwarp();
        println!("{}",data);
    }
    // unpack redis data to influxdb format
    // let msg = transport.unpack_redis_data(redis_data);
    // send data to influxdb
    // let _ = transport.influx_db_client.write_point(msg, Some(Precision::Seconds), None).unwarp;

}
