//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.9
//
// Note that this code has serious security risks!  You should not run it 
// on any system with access to sensitive files.
// 
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#[feature(globs)];
use std::io::*;
use std::io::net::ip::{SocketAddr};
use std::{str,os};
use std::str;
use std::io::File;

static IP: &'static str = "127.0.0.1";
static PORT:        int = 4414;
static mut COUNT: int =0;

fn main() {
    let addr = from_str::<SocketAddr>(format!("{:s}:{:d}", IP, PORT)).unwrap();
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen();
    
    println(format!("Listening on [{:s}] ...", addr.to_str()));
    
    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        do spawn {
            let mut stream = stream;
            
            match stream {
                Some(ref mut s) => {
                             match s.peer_name() {
                                Some(pn) => {println(format!("Received connection from: [{:s}]", pn.to_str()));},
                                None => ()
                             }
                           },
                None => ()
            }
            
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf).to_owned();
	    if(request_str.contains("GET")){
		let tempstr: ~[&str]= request_str.split(' ').collect();
		if(tempstr[1].len() >2){
			let file = Path::new(tempstr[1].slice_from(1));
			println!("{}",tempstr[1].slice_from(1));
			if tempstr[1].ends_with(".html") {
                    match result(|| File::open(&file)) {
                        Ok(mut f) => {
			    println!("FILE OK");
                            let response: ~[u8] = f.read_to_end();
                            stream.write(response);
                        } ,
                        Err(e) => {
                            if e.kind == PermissionDenied {
                                stream.write("404".as_bytes());
                            } 
			    else if e.kind == FileNotFound {
                                stream.write("404".as_bytes());
                            } 
			    else {
                                stream.write("io error".as_bytes());
                            }
                        }
                    }
                } 
		}
	    }
            println(format!("Received request :\n{:s}", request_str));
	    unsafe{
	       COUNT +=1;
	    
            let response: ~str = 
                ~"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>Hello, Rust!</title>
                 <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 </style></head>
                 <body>
                 <h1>Greetings, Krusty!</h1>
                 <h2> Visitors:" + COUNT.to_str() + "</h2></body></html>\r\n";
            stream.write(response.as_bytes());
            println!("Connection terminates.");
            }
        }
    }
}

//fn loadFile(path: &str)-> &str{
	 

//}
