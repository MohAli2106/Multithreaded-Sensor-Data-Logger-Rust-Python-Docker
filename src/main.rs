use rand:: Rng;
use chrono::{DateTime, Local};
use std::{
    fs::OpenOptions,
    io::Write,
    sync::{Mutex,Arc,mpsc},
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


fn collect_sensor_data(sens:u32,tx: mpsc::Sender<(u32, f64, String)>){
    let mut sensor=Sensor::new(sens);
    loop {
        sensor.collect_data();
        let timestamp=time();
        println!("time is : {}  ,Sensor {} Data: {:.2} C",timestamp,sensor.id,sensor.data);
        tx.send((sensor.id,sensor.data,timestamp)).expect("Unable to send data");
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

    let (tx, rx) = mpsc::channel();

    let file_dir="records.txt";
    let file_mux=Arc::new(Mutex::new(()));
    let mx_clone1=tx.clone();
    let mx_clone2=tx.clone();

    let h1=thread::spawn(move||{
        collect_sensor_data(1,  mx_clone1);
    });
    let h2=thread::spawn(move||{
        collect_sensor_data(2, mx_clone2);
    });

    for received in rx {
        let (sen_id, data, t) = received;
        log_data(sen_id, data, file_dir, &file_mux, t);
    }
    h1.join().expect("Thread 1 panicked");
    h2.join().expect("Thread 2 panicked");
}
