use std::{
    io::{self, Read, Write},
    net::TcpStream, thread, time,
};

fn connect(mut stream: TcpStream){
    // stdin on client side loop
    loop {

        let mut buffer = [0; 128];
        let w = stream.read(&mut buffer).unwrap();
        let from_server = String::from_utf8(buffer.to_vec()).unwrap();
        let fs = from_server.lines().next().unwrap();
        println!("{:?}", fs);

        // check if fs is PGN 
        // PGN -> board

        let stdin = io::stdin();
        let mut sbuffer = String::new();
        let _ = stdin.read_line(&mut sbuffer);
        let _ = stream.write(&mut sbuffer.as_bytes());
    }


}

fn main() -> std::io::Result<()> {
    // create timeout possibly MAYBEEEE
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    
    thread::spawn(move || {
        connect(stream);
    });

    loop {

    }
    
    Ok(())
}

// fn connect() {}
