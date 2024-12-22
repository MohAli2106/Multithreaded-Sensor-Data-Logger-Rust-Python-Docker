use rand:: Rng;
use chrono::{DateTime, Local};
use std::{
    fs::OpenOptions,
    io::Write,
    sync::{Mutex,Arc},
    thread,
    time::Duration,
    time::{SystemTime, UNIX_EPOCH},
};
#[derive(Debug)]
struct Sensor{
    id: u32,
    data:f64
}
fn time() ->  String {
    let now = SystemTime::now();
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


fn collect_sensor_data(sens:u32,file_dir:&str,fil_mut:&Arc<Mutex<()>>){
    let mut sensor=Sensor::new(sens);
    loop {
        sensor.collect_data();
        let timestamp=time();
        println!("time is : {}  ,Sensor {} Data: {:.2} C",timestamp,sensor.id,sensor.data);
        log_data(sensor.id, sensor.data, file_dir, fil_mut,timestamp);
        thread::sleep(Duration::from_secs(1));
    }


}

fn log_data(sen_id:u32,data:f64,file_dir:&str,fil_mut:&Arc<Mutex<()>>,t:String){
    let _lock=fil_mut.lock().unwrap();

    let mut file=OpenOptions::new()
    .create(true).append(true)
    .open(file_dir)
    .expect("Unable to open file");

    writeln!(file," time:{} ,sensor: {}, data: {:.2}",t,sen_id,data).expect("Unable to write data to file");
}

fn main() {

    let file_dir="records.txt";
    let file_mux=Arc::new(Mutex::new(()));
    let mx_clone1=Arc::clone(&file_mux);
    let mx_clone2=Arc::clone(&file_mux);

    let h1=thread::spawn(move||{
        collect_sensor_data(1, file_dir, &mx_clone1);
    });
    let h2=thread::spawn(move||{
        collect_sensor_data(2, file_dir, &mx_clone2);
    });

    
    h1.join().expect("Thread 1 panicked");
    h2.join().expect("Thread 2 panicked");
}
