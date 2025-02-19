use tokio::{io, io::AsyncBufReadExt, select};

use gossipd::gossipd::{Gossipd, GossipdOptions};

#[tokio::main]
async fn main() {
    // Create options for gossip.
    let mut gossip_options = GossipdOptions::default();
    if let Some(addr) = std::env::args().nth(1) {
        gossip_options.add_peer(addr)
    }

    let mut gossip = Gossipd::new(gossip_options);
    let tx = gossip.create_sender();

    // Set handler for received message.
    gossip.with_handler(|peer_id, message| {
        println!("{peer_id}: {}", String::from_utf8_lossy(&message.data))
    });

    tokio::spawn(async move { gossip.start().await });

    // Worker. Use tx to send data.
    let mut stdin = io::BufReader::new(io::stdin()).lines();
    println!("Enter your message.");
    loop {
        select! {
            Ok(Some(line)) = stdin.next_line() => {
                let _ = tx.send(line).await;
            }
        }
    }
}
