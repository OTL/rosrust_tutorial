# rosrust tutorial

Tutorial code of [rosrust](https://github.com/adnanademovic/rosrust).


## How to build

```bash
$ cargo build
```

## How to run

### run roscore

```bash
$ roscore
```

### run publisher

```bash
$ cargo run --bin publisher
```

or,

```bash
$ ./target/debug/publisher
```

### run subscriber

```bash
$ cargo run --bin subscriber
```

or

```bash
$ ./target/debug/subscriber
```

## How to use other msg

### edit build.rs

Add more message to build.rs.

```rust
#[macro_use]
extern crate rosrust;

rosmsg_main!("std_msgs/String", "more_msgs/SomeMsg");
```

then, you can use msg::more_msgs::SomeMsg in the sample code.
