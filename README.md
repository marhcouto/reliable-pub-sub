# SDLE First Assignment

SDLE First Assignment of group T06G14.

The project is structured in three sub-projects. A library called meic_mq. A client that has some test scenarios and the broker.

## Dependencies [Rust](https://www.rust-lang.org/)
We recommend installing Rust by following the official guide that can be found [here](https://www.rust-lang.org/learn/get-started). Addicionally our project depends on ZeroMQ because our library is actually a binding to the C implementation and on pkg-config. We only tested our program under Ubuntu and Ubuntu under WSL2.

### How to install ZeroMQ

**ZeroMQ** can be installed with the following command:

```
apt install libzmq3-dev
```

### How to install pkg-config
**pkg-config** can be installed with the following command:

```
apt install pkg-config
```

## How to compile and run our program
Our program uses the [Cargo](https://doc.rust-lang.org/cargo/) as package manager and build system

### Running broker
The following commands assume the user is inside the broker folder.

The broker can be compiled with the following command:

```
cargo build
```

The broker can be ran with the following command (that also compiles the program):

```
cargo run
```

### Running client
The following commands assume the user is inside the client folder.

The client can be compiled with the following command:

```
cargo build
```

The client has two test scenarios. One called slow subscriber that tries to emulate the scenario where a subscriber is behind other subscriber and the broker needs to wait for the slowest one to be able to delete messages. The second one is called late subscriber and tries to replicate the scenario where a subscriber subscribes a topic when a topic already has messages.

### Running the slow subscriber scenario

```
cargo run -- scenario slow_sub
```

### Running the late subscriber

```
cargo run -- scenario late_sub
```

Group members:

1. Marcelo Couto up201906086@up.pt
2. Francisco Oliveira up201907361@up.pt
3. Miguel Amorim up201907756@up.pt
4. André Santos up201907879@up.pt
