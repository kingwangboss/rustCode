use std::{sync::mpsc, thread, time::Duration};

// 1.Rust中实现消息传递并发的主要工具是通道。通道由两部分组成，一个是发送端，一个是接收端，
// 发送端用来发送消息，接收端用来接收消息。发送者或者接收者任一被丢弃时就可以认为通道被关闭了。
// 2.通道介绍
// a.通过mpsc::channel,创建通道，mpsc是多个生产者，单个消费者；
// b.通过spmc::channel,创建通道，spmc是一个生产者，多个消费者；
fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
        
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("A"),
            String::from("B"),
            String::from("B"),
            String::from("D"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
        
    });

    
    for recv in rx {
        println!("receiver: {}", recv);
    }
}
