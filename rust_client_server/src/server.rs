use serde::{Serialize, Deseralize};

// // not going to use channels because they are not meant for networking like I once thought they were 
// // DOH!!

// Channels are great for managing communication between threads, especially when you want to send tasks, events, or data between different threads in the same process.
// In contrast, when working with multiple processes or networked applications (like your chess game scenario), sockets and serialization are the better fit for communication.

//view channels as streams
// also channels allow for data to be sent asynchrnously betwqeen threads 


/* READ THIS
https://www.reddit.com/r/rust/comments/17za1x5/i_love_the_idea_of_using_channels_instead_of/

https://github.com/NimonSour/interthread

*/

struct Client {
    username: &str
}

struct Data {
    player_turn: Client,
    players: Client![]
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
}