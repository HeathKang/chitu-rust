#[macro_use]
extern crate influx_db_client;
extern crate redis;
use redis::Commands;
use influx_db_client::{Value, Precision};

// inter layer between redis and influxdb
struct Transport {
    redis: redis::Client,
    redis_connection: redis::Connection,
    influxdb: influx_db_client::Client,
    // msg: Msg,
}


impl Transport {
    fn connect_redis(&self) -> redis::Connection{
        // 
    }

    fn get_redis_data(&self) -> redis::RedisResult{
        //
    }

    fn unpack_redis_data(data: redis::RedisResult) -> Msg {
        //
    }

    fn send_influxdb(&self, msg: Msg, precision: Option<Precision>, rp: Option<&str>) -> Result<(), error::Error> {
        //
    }
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


fn main() {
    let transport = Transport{
        redis: redis::Client::open("redis://127.0.0.1/").unwrap(),
        redis_connection: self.get_connection().unwrap,
        influxdb: influx_db_client::Client::default().set_authentication("root", "root"),
    }
    // get redis data value
    let redis_data = transport.get_redis_data().unwrap();
    // unpack redis data to influxdb format
    let msg = transport.unpack_redis_data(redis_data);
    // send data to influxdb
    let _ = transport.influx_db_client.write_point(msg, Some(Precision::Seconds), None).unwarp;

}
