// 引用类库用于监听读取，
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
// 引用类库用于多线程处理
use std::thread;
// 引用类库用于类型转换。
use std::str;

// 程序主函数
fn main() { 
    // 定义一个请求地址和IP:端口
    let addr = "127.0.0.1:8866".to_string();
    // 创建一个Tcp监听，通过字符串切片将addr传入
    let listener = TcpListener::bind(&addr).unwrap();
    // 调用 incoming()方法接收客户端的链接信息，新信息返回一个Result枚举
    for stream in listener.incoming() {
        // 打印有新链接进入
        println!("有新客户端链接进入...");
        // 模式匹配
        match stream {
            // 当Result枚举类型匹配Ok时
            Ok(stream) => {
                // 链接成功，开启一个新线程
                thread::spawn(move|| {
                    // 将客户端处理信息解耦到handle_client函数中，并移交stream变量所有权
                    handle_client(stream);
                });
            }
            // 当Result枚举匹配错误时
            Err(e) => { 
                // 输出错误信息，并终止程序运行。
                panic!("出错了！ {:?}",e) 
            }
        }
    }

    // 关闭Tcp监听链接
    drop(listener);
}

// 线程调用的处理函数
fn handle_client(mut stream: TcpStream) {
    
    println!("客户端链接处理中...");
    // 定义一个存储用的数组
    let mut buf = [0; 512];
    // 建立一个循环，反复读取客户的输入信息
    loop {
        // 使用read方法
        let bytes_read = stream.read(&mut buf).expect("读取出现错误，中断程序运行");
        // 输出调试信息
        println!("byte size: {}", bytes_read);
        // 如果输入的字符长度是0，直接退出循环。
        if bytes_read == 0 {
            // 退出loop
            break;
        }

    // 将byte转换为str类型
    let s = match str::from_utf8(&buf[..bytes_read]) {
        // 如果转换成功返回字符串值。
        Ok(v) => v,
        // 遇到转换错误输出错误信息，终止程序运行。
        Err(e) => {
            // 输出调试信息。
            stream.write(b"Need utf-8 sequence.").unwrap();
            // 继续监听
            continue;
        },
    };
             
    // 如果输入的前3个字符串是 bye 则程序终止
    if s.len() >= 3 && s[0..3] == "bye".to_string() {
        // 输出终止前的消息
        stream.write(b"Bye bye and polkadot to the moooooon!!\n").unwrap();
        // 跳出 loop 循环
        break;
    }
        
        // 如果程序没有终止，返回输入的消息
        stream.write(&buf[..bytes_read]).unwrap();
    }
}
    


