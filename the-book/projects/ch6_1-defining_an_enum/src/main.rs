fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Enum variant
    enum IpAddrV1 {
        V4(String),
        V6(String),
    }
    let home = IpAddrV1::V4(String::from("127.0.0.1"));
    let loopback = IpAddrV1::V6(String::from("::1"));

    enum IpAddrV2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddrV2::V4(127, 0, 0, 1);
    let loopback = IpAddrV2::V6(String::from("::1"));

    // We can define methods on enums as structs
    impl IpAddrV2 {
        fn Ping(&self) {
            // Do something
        }
    }
    home.Ping();

    // The `Option` Enum and Null values
    //
    // NOTE: Rust doesn't have NULL
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}
