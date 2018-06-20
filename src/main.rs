#[macro_use]
extern crate influx_db_client;
extern crate redis;


fn main() {
    println!("Hello, world!");
    fetch_an_integer();
}

fn fetch_an_integer() -> redis::RedisResult<isize>{
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let con = try!(client.get_connection());

    let _ :() = try!(con.set("my_key", 42));

    con.get("my_key")

}
