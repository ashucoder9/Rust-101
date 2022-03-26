use std::net::TcpListener;

fn main() {
    // Binding localhost port  
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for request in listener.incoming() {
        let req = request.unwrap();

        // Prints this message everytime a connection request is made
        println!("A connection request was made");
    }
}
