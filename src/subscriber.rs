#[macro_use]
extern crate rosrust;
use rosrust::Ros;
use std::{thread, time};

rosmsg_include!();

fn main() {
    let mut ros = Ros::new("subscriber_rs").unwrap();
    ros.subscribe("chat", |v: msg::std_msgs::String| println!("{}", v.data)).unwrap();
    loop {
        thread::sleep(time::Duration::from_secs(100));
    }
}
