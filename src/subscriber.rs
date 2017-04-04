extern crate rustc_serialize;
extern crate rosrust;
mod Hello;
use rosrust::Ros;
use std::{thread, time};

fn main() {
    let mut ros = Ros::new("node_name2").unwrap();
    ros.subscribe("some_topic", |v: Hello::Hello| println!("{}", v.hello)).unwrap();
    loop {
        thread::sleep(time::Duration::from_secs(100));
    }
}
