use std::net::{TcpListener, TcpStream};
use std::io::{Read};

use std::io::{Write, Result};


fn main() {

    if let Err(err) = client_two() {
        eprintln!("Client error: {:?}", err);
    }

    if let Err(err) = server_two() {
        eprintln!("Server error: {:?}", err);
    }
}



fn server_two() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081")?;
    println!("Server listening on 127.0.0.1:8081");

    for stream in listener.incoming() {
        let mut stream = stream.expect("Failed to accept incoming connection");
        println!("Client connected");

        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

        println!("Closing connection");
    }

    Ok(())
}


fn client_two() ->  Result<()>{
    let mut stream = TcpStream::connect("127.0.0.1:8082")?;
    println!("Connected to server");

    let message = "\nNode 2 to Node3\n";
    stream.write_all(message.as_bytes())?;
    println!("Sent: {}", message);

    println!("Closing connection");

  Ok(())

}



//NEW CODE FOR FILE START


// use std::net::{TcpListener, TcpStream};
// use std::io::{Read, Write, Result};
// use std::fs::File;
// use std::io::{BufReader, BufWriter};

// fn main() {
//     if let Err(err) = client_one() {
//         eprintln!("Client error: {:?}", err);
//     }

//     if let Err(err) = server_one() {
//         eprintln!("Server error: {:?}", err);
//     }
// }

// fn server_one() -> Result<()> {
//     let listener = TcpListener::bind("127.0.0.1:8080")?;
//     println!("Server listening on 127.0.0.1:8080");

//     for stream in listener.incoming() {
//         let mut stream = stream.expect("Failed to accept incoming connection");
//         println!("Client connected");

//         let mut buffer: [u8; 1024] = [0; 1024];
//         let bytes_read = stream.read(&mut buffer)?;
//         println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

//         // Open the output file for writing
//         let mut file = File::create("received_file.txt")?;
//         file.write_all(&buffer[..bytes_read])?;

//         println!("File received and saved");

//         println!("Closing connection");
//     }

//     Ok(())
// }

// fn client_one() -> Result<()> {
//     let mut stream = TcpStream::connect("127.0.0.1:8080")?;
//     println!("Connected to server");

//     let file = File::open("./large_file.txt")?;
//     let mut reader = BufReader::new(file);
//     let mut buffer = [0; 1024];

//     loop {
//         let bytes_read = reader.read(&mut buffer)?;
//         if bytes_read == 0 {
//             break;
//         }
//         stream.write_all(&buffer[..bytes_read])?;
//     }

//     println!("File sent");

//     println!("Closing connection");

//     Ok(())
// }


//NEW CODE FOR FILE END