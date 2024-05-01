use core::panic;
use std::{
    any::Any, io::{Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, sync::mpsc::{channel, Receiver, Sender}, task, thread, time::{self, Duration}
};

//TODO handle clients disconnecting


// reciever is the other clients
fn handle_client(mut stream: TcpStream, sender: Sender<Vec<u8>>, reciever: Receiver<Vec<u8>>, id: u8) {
    // server <-> client
    let hello_msg = String::from(format!("Connected to server. Hello client{}\n",id));
    // TODO error checking for write w/o unwrap()
    stream.write(hello_msg.as_bytes()).unwrap(); // server -> client
                                         // sender <-> reciever : client1 <-> client2

    // recieve input
    loop {
        // over 128 bytes we don't get the message
        // might be a way to do this w/ Strings for buffer
        let mut buffer = [0; 128];
        // output = number of bytes read from buffer
        let output = stream.read(&mut buffer);
        let p = match output {
            Ok(output) => output,
            Err(_) => break,
        };

        // Make function that takes buffer checks for new line. breaks loop and then add to another buffer
        let output_to_string = String::from_utf8(buffer.to_vec()).unwrap();
        let s2 = output_to_string.trim();

        // send the message to the other client
        // each client in its own thread std::sync::mpsc
        // https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
        println!("{:?}", s2); // prints output on server side

        // send stdin from client -> pair client
        let send = sender.send(s2.into());
        // let rec = match reciever {
        //     Ok() => {
        //         reciever.
        //     },
        //     Err(_) => {

        //     }
        // };
        let rec = reciever.recv().unwrap();
        let mut r = &rec.clone();
        let rec_str = String::from_utf8(rec);
        
        let mut client_buff:[u8;10] = [0;10];
        thread::sleep(time::Duration::from_secs(4));

        for (client_byte, received_byte) in client_buff.iter_mut().zip(r.clone().into_iter()) {
            *client_byte = received_byte;
        }

        let _ = stream.write(&client_buff);

        println!("server has recieved {:?} from client {}", rec_str,id); // prints what channel recv on server side
    }
}

fn main() -> std::io::Result<()> {
    // might need to match it later
    let listener: TcpListener = TcpListener::bind("127.0.0.1:8080").unwrap();
    loop {
        let client1 = listener.accept();
        let client2 = listener.accept();

        let (c1_sender, c1_reciever) = channel();
        let (c2_sender, c2_reciever) = channel();

        // future error handling
        // let client1 = match client1{
        //     Ok(client1) => client1,
        //     Err(_) => {panic!("gjkkgjf")},
        // };
        // let client2 = match client2{
        //     Ok(client2) => client2,
        //     Err(_) => {panic!("gjkkgjf")},
        // };

        match client1 {
            Ok((stream, _socketaddr)) => {
                // accept connections and process them serially
                thread::spawn(move || {
                    handle_client(stream, c1_sender, c2_reciever, 1);
                });
            }
            // handle error listening
            Err(_) => {}
        }

        match client2 {
            Ok((stream, _socketaddr)) => {
                // accept connections and process them serially
                thread::spawn(move || {
                    handle_client(stream, c2_sender, c1_reciever, 2);
                });
            }
            // handle error listening
            Err(_) => {}
        }

        // c1_sender.send(t)

        // server task which creates channel
        // channel is between c1 c2
        // concurrent client pairs we need it in a loop
    }

    Ok(())
}
