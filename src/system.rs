use rand:: Rng;
use chrono::{DateTime, Local};
use std::{
    fs::OpenOptions,
    io::Write,
    sync::{Mutex as Mx,Arc,mpsc},
    thread as Td,
    time::Duration as D,
    time::{SystemTime as St},
    net::TcpStream as TS,

};



#[derive(Debug)]
struct Sensor{
    id: u32,
    data:f64
}
fn time() ->  String {
    let now = St::now();
    let datetime: DateTime<Local> = now.into();
    datetime.format("%H:%M:%S%.3f").to_string()
}

impl Sensor{
    fn new(id:u32)->Self{
        Sensor{
            id,
            data:0.0,
        }
    }
    fn collect_data(&mut self){
        let mut rng=rand::thread_rng();
        self.data=rng.gen_range(20.0..30.0);
    }
}


pub fn collect_sensor_data(sens:u32,tx: mpsc::Sender<(u32, f64, String)>, ser_add:&str){
    let mut sensor=Sensor::new(sens);
    let mut st=TS::connect(ser_add).expect("couldnot connect to server");
    
    loop {
        sensor.collect_data();
        let timestamp=time();

        println!("time is : {}  ,Sensor {} Data: {:.2} C",timestamp,sensor.id,sensor.data);
       
        tx.send((sensor.id,sensor.data,timestamp.clone())).expect("Unable to send data");
        let msg=format!("time:{} ,sensor: {}, data: {:.2}",timestamp,sensor.id,sensor.data);
       
        st.write_all(msg.as_bytes()).expect("couldnot write to server");
        Td::sleep(D::from_secs(1));
    }


}

 pub fn log_data(sen_id:u32,data:f64,file_dir:&str,fil_mut:&Arc<Mx<()>>,t:String){
    let _lock=fil_mut.lock().unwrap();

    let mut file=OpenOptions::new()
    .create(true).append(true)
    .open(file_dir)
    .expect("Unable to open file");

    writeln!(file," time:{} ,sensor: {}, data: {:.2}",t,sen_id,data).expect("Unable to write data to file");
}
