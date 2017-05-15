#[macro_use]
extern crate rosrust;
extern crate env_logger;

use rosrust::Ros;
use std::{thread, time};

rosmsg_include!();

fn main() {
    env_logger::init().unwrap();
    let mut ros = Ros::new("publisher_rs").unwrap();
    let mut publisher = ros.publish("chat").unwrap();
    loop {
        thread::sleep(time::Duration::from_secs(1));
        let mut msg = msg::std_msgs::String::new();
        msg.data = "hello rosrust".to_string();
        publisher.send(msg).unwrap();
    }
}
