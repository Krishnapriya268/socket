use std::net::{TcpStream};
use std::io::{self,Read, Write};
use std::str::from_utf8;
use std::thread;
use std::collections::VecDeque;
use std::time::{Duration,Instant};
use std::fs::File;

const ONE_MINUTE: u64=60;

fn handle_read(mut stream: &TcpStream)->String
{
    let mut buffer = Vec::new();
    for _ in 0..128 {
         buffer.push(0); 
    }
    let read_bytes = stream.read(&mut buffer).unwrap();
    let text = from_utf8(&buffer[0..read_bytes]).unwrap();
    println!("Received message: {}", text);
    return String::from(text)
}

fn handle_write(mut stream: &TcpStream)->String
{
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line"); // replace  the 
    let a1= a.trim().as_bytes();
    stream.write(&a1).unwrap();
    return String::from(a.trim())
}

fn main() {
    let q = VecDeque::new();
    let mut read_q=q.clone();
    let mut write_q=q.clone();
    let mut current_time=Instant::now();
    match TcpStream::connect("localhost:3333") {
        
       
        Ok(stream) => {
             let read_stream=stream.try_clone().expect("Failed to clone stream");
             let write_stream=stream.try_clone().expect("Failed to clone stream");
            println!("Successfully connected to server in port 3333");
            
            let handle1=thread::spawn(move ||  {
                loop{
                let message1=handle_write(&write_stream);
                thread::sleep(Duration::from_secs(1));

                let captured_data="Me : ".to_owned()+ &message1;
                    write_q.push_back(captured_data);
                    write_data_to_file(&write_q);
                    //println!("added-{:?}",current_time);
                    
                    if current_time.elapsed().as_secs() > ONE_MINUTE {
                       // println!("delete-{:?}",current_time);
                        write_q.pop_front();
                        current_time=Instant::now();
                    }
                    thread::sleep(Duration::from_secs(1));
                }
            
    
            });
            
            let handle2=thread::spawn(move ||  {
               loop{
                let message2=handle_read(&read_stream);

                let captured_data="You : ".to_owned()+ &message2;
                    read_q.push_back(captured_data);
                    write_data_to_file(&read_q);
                    //println!("added");
                    
                    if current_time.elapsed().as_secs() > ONE_MINUTE {
                        //println!("delete");
                        read_q.pop_front();
                        current_time=Instant::now();
                    }
                thread::sleep(Duration::from_secs(1));
               }
            });
           
            handle1.join().unwrap();
            handle2.join().unwrap();
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn write_data_to_file<T:std::fmt::Debug>(data_buffer:&VecDeque<T>) ->io::Result<()> 
{
    let mut file =File::create("data11.txt")?;
    for line in data_buffer.iter() {
        writeln!(file,"{:?}",line)?;
    }
    Ok(())
}
