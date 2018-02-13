use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs::File;

// SAMPLE GET ON DEFAULT LISENER:
//
// METHOD REQUESTED_URI HTTP-VERSION \r\n
// HEADERS ...
//

// GET / HTTP/1.1
// Host: 127.0.0.1:8088
// User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:58.0) Gecko/20100101 Firefox/58.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
// Accept-Language: en-US,en;q=0.5
// Accept-Encoding: gzip, deflate
// Connection: keep-alive
// Upgrade-Insecure-Requests: 1

// SAMPLE RESPONSE FROM DEFAULT LISTENER:
//
// HTTP-VERSION STATUS-CODE REASON-PHRASE \r\n
// HEADERS ...
// Message
//

fn process_stream(mut stream : TcpStream) {
    // TODO: This should be dynamic, requests can be theoretically any size
    // Although it is likely a good idea to have some level of control to limit
    // request size
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let mut file = File::open("index.html").unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n {}", content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    // Listen on localhost:8088 for incoming TCP connections
    // TODO: Better error handling here
    let listener = TcpListener::bind("127.0.0.1:8088").unwrap();

    // Continue listening on 8088 and process the stream obtained from each
    for stream in listener.incoming() {
        // We're now iterating through each connection made to the web server
        // A connection is a complete request/reply, the stream here in the input
        // of the user's request
        let stream = stream.unwrap();

        process_stream(stream);
    }
}
