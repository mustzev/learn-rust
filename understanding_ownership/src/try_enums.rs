enum IPAddressKind {
    v4,
    v6,
}

struct IPAddress {
    kind: IPAddressKind,
    address: String,
}

fn route(ip_kind: IPAddressKind) {
    let home = IPAddress {
        kind: IPAddressKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IPAddress {
        kind: IPAddressKind::v6,
        address: String::from("::1"),
    };

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
