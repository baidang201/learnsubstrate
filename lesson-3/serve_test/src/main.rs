use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) { //匹配读取情况
        Ok(size) => {
            // echo everything!
            println!("Request: {}", String::from_utf8_lossy(&data[..]));//打印传过来的字符串
            stream.write(&data[0..size]).unwrap();//回写到客户端
            true
        },
        Err(_) => {//处理异常
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();//关闭连接
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();//本地监听
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {//监听来的内容
        match stream {
            Ok(stream) => {//如果是字节流
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream) //处理字节流
                });
            }
            Err(e) => {//异常
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}