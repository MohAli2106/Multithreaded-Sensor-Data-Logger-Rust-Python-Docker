use std::{
    sync::{Mutex as Mx,Arc,mpsc},
    thread as Td};
mod system;
use system::{collect_sensor_data, log_data};

mod tcp_server;
use tcp_server::Tcpserv;

fn main() {

    let adress: String = "127.0.0.1:7878".to_string();
   
    let add1=adress.clone();
    let  add2=adress.clone();

    let server=Tcpserv::new(&adress);
    let handle=Td::spawn(move||{    if let Err(e)=server.server_on(){
        eprintln!("couldnot start server: {}",e);
    }});

    let (tx, rx) = mpsc::channel();

   // let file_dir="records.txt";
//    let file_mux=Arc::new(Mx::new(()));
    let mx_clone1=tx.clone();
    let mx_clone2=tx.clone();

    let h1=Td::spawn(move||{
        collect_sensor_data(1,  mx_clone1,&add1);
    });
    let h2=Td::spawn(move||{
        collect_sensor_data(2, mx_clone2,&add2);
    });




   
    h1.join().expect("Thread 1 panicked");
    h2.join().expect("Thread 2 panicked");
    handle.join().expect("Server thread panicked");
}
