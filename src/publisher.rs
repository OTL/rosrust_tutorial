extern crate rustc_serialize;
extern crate rosrust;
mod Hello;
use rosrust::Ros;
use std::{thread, time};


fn main() {
    let mut ros = Ros::new("node_name1").unwrap();
    let mut publisher = ros.publish::<Hello::Hello>("some_topic").unwrap();
    loop {
        thread::sleep(time::Duration::from_secs(1));
        let msg = Hello::Hello { hello: "hello rosrust".to_string() };
        publisher.send(msg).unwrap();
    }
}
