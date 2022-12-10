/*
* Julian Columbres, Ryan Hedgecock
* TCP Server implemented in Rust. Serves helloworld.html at port 6789 of localhost.
*/

use std::net::{TcpListener, TcpStream};
use std::io::{self, Read, Write};
use std::fs::File;

const ADDRESS: &str = "0.0.0.0:6789"; // listen on port 6789

fn main() -> io::Result<()> {
    
    loop {    

        // create TcpListener --> abstracts socket server and listens, binded to an address 
        let listener: TcpListener = TcpListener::bind(&ADDRESS)?;

        // connect to random address, extract the local address from stream
        let stream = TcpStream::connect("google.com:443")?;
        let local_addr = stream.local_addr()?;
        let local_ip = local_addr.ip();
    
        // Print the local IP address and port
        println!("Listening on local address: {}, port 6789", local_ip);
        
        // listener.accept() returns a Result -> either an error or success: in which case is a Tcp stream
        match listener.accept() {
            
            Err(error) => {
                panic!("Error when establishing connection: {:?}", error);
            }
            // successfully established connection - now processs incoming stream
            Ok((mut tcp_stream, client_addr)) => { 

                println!("Connection established! Client address: {:?}", client_addr);

                // read stream bytes into buffer
                let mut client_buffer: [u8; 1024] = [0; 1024];
                tcp_stream.read(&mut client_buffer)?;
    
                // expect() is another way of handling Result, - provides an error response
                // stream bytes into string 
                let request_string: &str = std::str::from_utf8(&client_buffer).expect("Expecting valid utf8");

                // Expected string is in format: GET /helloworld.html ... 

                // get 2nd word in the request, i.e "/helloworld.html" then remove the preceding slash
                let mut file_name: &str = request_string.split(' ').nth(1).unwrap();
                file_name = &file_name[1..file_name.len()];

                // ignore additional requests serving the website icon
                if file_name.starts_with("favico.co") {
                    drop(listener);
                    continue;
                }

                println!("Serving file {:?} if it exists", &file_name);
    
                /*  open the file: 
                *       if result is error - 
                *                return 404 html file, and 404 error header
                *       if result is success - 
                */                return available html file, and 200 OK header
                let (mut file, response_header) = match File::open(&file_name) {
                    Err(_error) => {
                        let not_found_file: File = File::open("404.html")?;
                        let error_header: &str =  "HTTP/1.0 404 NOT FOUND";
                        (not_found_file, error_header)
                    }
                    Ok(file) => {
                        let target_file: File = file;
                        let ok_header: &str = "HTTP/1.0 200 OK";
                        (target_file, ok_header)
                    } 
                };
    
                // stream the file contents to string 'contents'
                let mut contents: String = String::new();
                file.read_to_string(&mut contents)?;

                // format response with header status code and contents
                let response: String = format!("{}\r\n\r\n{}", response_header, contents);
                // send response back through tcp server
                tcp_stream.write_all(response.as_bytes())?;
                tcp_stream.flush()?;
                drop(listener);
            }
        }
    }
}

