use std::{io::{BufRead as Br,BufReader as Brr, Write},net::{TcpListener as TL,TcpStream as TS},thread,sync::{Mutex as Mx,Arc},};
use std::io;
pub struct Tcpserv{
    address: String,
    data_sto:Arc<Mx<String>>,
}
impl Tcpserv{
    pub fn new(adrs: &str)->Self{
        Tcpserv{
            address:adrs.to_string(),
            data_sto:Arc::new(Mx::new(String::new())),

    }
}
fn handle_serv(mut stream:TS,data_sto:Arc<Mx<String>>){
    let mut reader=Brr::new(&stream);
    let mut req=String::new();

    if reader.read_line(&mut req).is_ok(){
if req.starts_with("GET / "){
    let res_body={
        let data=data_sto.lock().unwrap();
        format!("Sensor Data: \n {}",data)
    };

    let response =format!( "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
    res_body.len(),
    res_body);
    stream.write_all(response.as_bytes()).expect("couldnot write to stream");

}

}

    
}


pub fn server_on(&self)->io::Result<()>{
    let les=TL::bind(&self.address).expect("couldnot bind to address");
   
    println!("the server is ready on {}",self.address);
   

    for i in les.incoming(){
        match i{
            Ok(i)=>{
                let d_sto=self.data_sto.clone();
                thread::spawn(move || Tcpserv::handle_serv(i,d_sto));
                println!("new connection established");
            }
            Err(e)=>eprintln!("couldnot connect: {}",e),
        }


    }
    Ok(())
}

    pub fn update_data(&self,new_data:String){
    let mut data=self.data_sto.lock().unwrap();
        *data=new_data;



}

}