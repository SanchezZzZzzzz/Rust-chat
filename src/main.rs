use std::{net::TcpStream, io::{Write, Read, stdin}, thread, time::Duration};
use std::str::from_utf8;
fn main() {
    println!("Welcome to our RUST chat application! You are welcomed to be here every time of the day :) \nApplication has been developed by student of BMSTU, Sergey Manukhin.");
    println!("Please, enter Address of the server or 'q' to quit: ");
    let mut addr = String::new();
    stdin().read_line(&mut addr).expect("Shit");
    let mut uid: u8 = 0;
    let tr_addr = addr.trim_end();
    if tr_addr != "q"{
        if let Ok(mut stream) = TcpStream::connect(tr_addr) {
            let mut new_id = [0 as u8; 1];
            match stream.read_exact(&mut new_id) {
                Ok(_) => {
                    println!("Assigned id: {}", new_id[0]);
                    uid = new_id[0];
                }
                Err(e) => {
                    print!("Failed to recieve data: {}", e);
                }
            }
            println!("Connection established. Print \"/q\" to exit");
            let stream_o = stream.try_clone();
            let mut reciever: u8 = 0;
            match stream_o {
                Ok(stream_o) =>{
                    println!("Establishing recieving and sending connections...");
                    thread::spawn(move || {
                        get_input(stream_o, &mut reciever, uid);
                        thread::sleep(Duration::from_millis(100));
                    });
                    get_output(stream, &reciever);
                        thread::sleep(Duration::from_millis(100));
                }
                Err(_) => {
                    println!("Didn't manage to clone stream...")
                }
            }    
        }
        else {
            println!("no connection");
        }
    }
    else {
        println!("quitting...");
    }
}

fn get_input(mut stream: TcpStream, reciever: &mut u8, uid: u8){
    println!("Starting input function... ");
    loop{
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error while reaidng a string");
        let trimmed_input = input.trim_end();
        if trimmed_input == "@a"{
            *reciever = 0;
            println!("Now our messages can be seen to everyone!");
        }
        else if (trimmed_input.as_bytes()[0] == "@".as_bytes()[0]) {
            let prev = *reciever;
            *reciever = trimmed_input[1..trimmed_input.len()].parse::<u8>().unwrap();
            if (uid == *reciever){
                println!("You can't send messages to yourself!");
                *reciever = prev;
            }
            else {
                println!("Now chatting with {}", reciever);
            }
            
        }
        else {
            let mut message = Vec::<u8>::new();
            message.push(*reciever);
            message.append(&mut uid.to_string().as_bytes().to_vec());
            message.append(&mut input.as_bytes().to_vec());
            stream.write(&message).unwrap();
        }
    };
}

fn get_output(mut stream: TcpStream, reciever: &u8){
    let mut recieved_data: [u8; 70] = [0 as u8; 70];
    println!("Starting output function... ");
    while match stream.read(&mut recieved_data) {
        Ok(size) => {
            let utf_data = from_utf8(&recieved_data).unwrap();
            println!("{}", utf_data);
            true
        }
        Err(_) => {
            println!("Error!");
            false
        }
    } {}
}
    // loop{
    //     match stream.read(&mut recieved_data){
    //         Ok(_) => {
    //             println!("Awaiting reply");
    //             let utf_data = from_utf8(&recieved_data).unwrap();
    //             println!("{}", utf_data);
    //             new_id = utf_data.as_bytes();
    //             println!("{}", from_utf8(&new_id).unwrap());
    //         },
    //         Err(_) => {
    //             println!("Failed to get data from the server");
    //         }
    //     } {
    //         println!{"no_data"};
    //     }
    // }