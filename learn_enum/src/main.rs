
fn main() {
    //1.类似于c语言的方式定义
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let i1 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let i2 = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    //2.rust语言提倡的方式定义
    enum IpAddr2 {
        V4(String),
        V6(String),
    }
    let i1 = IpAddr2::V4(String::from("127.0.0.1"));
    let i2 = IpAddr2::V6(String::from("::1"));

    //3.可以是不同类型
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let i1 = IpAddr3::V4(127, 0, 0, 1);
    let i2 = IpAddr3::V6(String::from("::1"));

    //4.经典用法
    enum Message {
        Quit,
        Move{x: i32, y: i32},
        Write(String),
        Change(i32, i32, i32),
    }

}
