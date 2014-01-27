use std::rand::random;
use std::os;
use std::io::File;
use std::str;

use std::io::buffered::BufferedReader;

fn main() {
    let args: ~[~str] = os::args();
    if args.len() != 3 {
        println!("Usage: {:s} <inputfile>", args[0]); 
    } else {
        let file1 = args[1].clone();
	let file2 = args[2].clone();
        //let path = Path::new(fname.clone());
        //let msg_file = File::open(&path);

        match File::open(&Path::new(file1)) {
	        Some(file) => {
	            let mut reader = BufferedReader::new(file);
		    let s: ~[u8] = reader.read_to_end();
		    match File::open(&Path::new(file2)) {
	        	Some(file) => {
	            	   let mut reader = BufferedReader::new(file); 
		    	   let d: ~[u8] = reader.read_to_end();
			   let p = str::from_utf8_owned(xor(s,d));
	   		   println!("{:?}",p);
	        	   }
	        	None =>{
	           	   println!("Opening message.txt failed!");
	       	    	}
		    }
	        }	
        	None =>{
           		println!("Opening message.txt failed!");
        	}
    	}
    }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}


