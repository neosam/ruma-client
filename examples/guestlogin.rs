extern crate ruma_client;

use ruma_client::Client;

fn main() {
    let mut client = Client::new("https://matrix.org/")
        .expect("Could not connect to server");
    client.guest_session()
        .expect("Cound not register as guest");
    println!("Client: {:?}", client);
}