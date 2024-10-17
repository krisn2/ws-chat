extern crate ws;

use std::rc::Rc;
use std::cell::RefCell;
use ws::{listen, Handler, CloseCode, Message, Result, Sender};


struct ChatServer {
    out: Sender,  //  the server's sender
    peers: Rc<RefCell<Vec<Sender>>>  //  a list of all connected clients (peers)
}

impl Handler for ChatServer {
    // Handle incoming connections

    fn on_message(&mut self, msg: Message) -> Result<()>{

        println!("Received message: {}", msg);

        // Broadcast the message to all connected clients (peers)
        for peer in self.peers.borrow().iter() {
            if *peer != self.out {
                peer.send(msg.clone())?;
            }
        }
        Ok(())
    }

  fn on_close(&mut self, _code: CloseCode, _reason: &str) {
      println!("peer disconnected");

      // Remove the client from the list of peers

      let mut peers = self.peers.borrow_mut();
      if let Some(pos) = peers.iter().position(|p| *p == self.out) {
        peers.remove(pos);
      }
  }

}

fn main () {

    // Create an empty vectors of peers, shared across all connnections 
    let peers = Rc::new(RefCell::new(Vec::new()));

    // Listen for incoming connections on the specified address
    listen("127.0.0.1:8080", |out| {
        println!("new client connected");

        // Add the new client to the list of peers
        let peers = peers.clone();
        peers.borrow_mut().push(out.clone());
        

        ChatServer {
            out: out,
            peers: peers
        }
    }).unwrap();
    
}