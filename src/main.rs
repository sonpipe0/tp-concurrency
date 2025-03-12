use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3030")?;
    for stream in listener.incoming() {
        handle_client(stream?).unwrap();
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buf = [0; 512];
    let bytes = stream.read(&mut buf).unwrap();
    let req = String::from_utf8_lossy(&buf[..bytes]);
    println!("{}", req); // los headers terminan con un /r/r/n/n
    let (headers, body) = req.split_at(req.rfind("\r\n\r\n").expect("Headers not found"));
    println!("headers:{},body:{}", headers, body);
    Ok(())
}
