use std::time::{SystemTime, UNIX_EPOCH};
use rand::thread_rng;
use rand::RngCore;
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::collections::HashMap;




#[derive(Debug, Clone)]
pub enum CL {
    Pink,
    Purple,
    Green,
    DullGreen,
    Blue,
    DullRed,
    Red,
    Orange,
    Teal,
    DullTeal,
    Dull,
    End,
}

impl ToString for CL {
    fn to_string(&self) -> String {
        match self {
            CL::Pink => "\x1b[38;5;201m".to_string(),
            CL::Purple => "\x1b[38;5;135m".to_string(),
            CL::Green => "\x1b[38;5;46m".to_string(),
            CL::DullGreen => "\x1b[38;5;29m".to_string(),
            CL::Blue => "\x1b[38;5;27m".to_string(),
            CL::DullRed => "\x1b[38;5;124m".to_string(),
            CL::Red => "\x1b[38;5;196m".to_string(),
            CL::Orange => "\x1b[38;5;208m".to_string(),
            CL::Teal => "\x1b[38;5;14m".to_string(),
            CL::DullTeal => "\x1b[38;5;153m".to_string(),
            CL::Dull => "\x1b[38;5;8m".to_string(),
            CL::End => "\x1b[37m".to_string(),
        }
    }
}

// =-= FileHandler =-= //
pub struct FileHandler {
    file: std::fs::File,
}

impl FileHandler {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)?;
        Ok(Self { file })
    }

    pub fn write_line(&mut self, content: String) -> io::Result<()> {
        writeln!(self.file, "{}", content)
    }
}




#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub enum ExperimentID {
    SlowNoData,
    SlowWithData,
    BurstNoData,
    BurstWithData,
    ConsistentNoData,
    ConsistentWithData,
    SlowLargeData,
    BurstLargeData,
    ConsistentLargeData,
}

impl ToString for ExperimentID {
    fn to_string(&self) -> String {
        match self {
            ExperimentID::SlowNoData => "SlowNoData".to_string(),
            ExperimentID::SlowWithData => "SlowWithData".to_string(),
            ExperimentID::BurstNoData => "BurstNoData".to_string(),
            ExperimentID::BurstWithData => "BurstWithData".to_string(),
            ExperimentID::ConsistentNoData => "ConsistentNoData".to_string(),
            ExperimentID::ConsistentWithData => "ConsistentWithData".to_string(),
            ExperimentID::SlowLargeData => "SlowLargeData".to_string(),
            ExperimentID::BurstLargeData => "BurstLargeData".to_string(),
            ExperimentID::ConsistentLargeData => "ConsistentLargeData".to_string(),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSData {
    pub experiment_id: ExperimentID,
    pub timestamp: u128,
    pub data: Vec<u8>,
}



fn main() {
    // Client and Server
    // Client will be generic std::net::TcpStream
    // Server will be generic std::net::TcpListener

    // Client will connect to server and then listen to incoming messages
    // Server will listen to incoming connections and then send messages to connected clients in a varying manner
    
    // want to test bursts of traffic / consistent traffic / slow traffic
    // want to test different message sizes
    
    // each object will have a timstamp generated on the server side, and client will use that to determine latency


    // =-= Server =-= //
    // want core affinity on one thread, using tokio runtime for localset


    let server_thread = std::thread::spawn(move || {
        let res = core_affinity::set_for_current(core_affinity::CoreId { id: 0 });
        if res {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .worker_threads(1)
                .build()
                .expect("build runtime");
            let local = tokio::task::LocalSet::new();
            local.block_on(&rt, async {
                let mut handles = Vec::new();

                let handle_1 = tokio::task::spawn_local(async move {
                    use tokio::io::AsyncWriteExt;

                    let server_url = "127.0.0.1";
                    let server_port = 7200;

                    let listener = tokio::net::TcpListener::bind(format!("{}:{}", server_url, server_port)).await.expect("bind");
                    println!("[+] Server listening on: {}", server_port);
                    loop {
                        let (mut stream, _) = listener.accept().await.expect("accept");

                        // =-= Experiments =-= //
                        // 1. Slow traffic, w/ no data
                        // 2. Slow traffic, w/ data
                        // 3. Burst traffic, w/ no data
                        // 4. Burst traffic, w/ data
                        // 5. Consistent traffic, w/ no data
                        // 6. Consistent traffic, w/ data
                        // 7. Slow traffic, w/ large data
                        // 8. Burst traffic, w/ large data
                        // 9. Consistent traffic, w/ large data
                        // =---------------------------------------------------------= //
                        // =-= Latencies =-= //
                        // 50us = burst
                        // 15ms = consistent
                        // 50ms = slow
                        // =-= Message Sizes =-= //
                        // No Data: 0KB
                        // w/ Data: 32bytes
                        // Large Data: 256bytes
                        // =-= Message Format =-= //
                        // ExperimentID, Timestamp, Data
                        // =---------------------------------------------------------= //
                        // =-= Buffers =-= //

                        let mut rng = thread_rng();
                        let mut small_buffer = [0u8; 32]; // Allocate a 1KB buffer
                        rng.fill_bytes(&mut small_buffer); // Fill the buffer with random bytes


                        let mut rng = thread_rng();
                        let mut medium_buffer = [0u8; 128]; // Allocate a 10KB buffer
                        rng.fill_bytes(&mut medium_buffer); // Fill the buffer with random bytes


                        let mut rng = thread_rng();
                        let mut large_buffer = [0u8; 256]; // Allocate a 100KB buffer
                        rng.fill_bytes(&mut large_buffer); // Fill the buffer with random bytes

                        // =---------------------------------------------------------= //
                        // There is a lot of inherent fixed latency from serializing and sending the data,
                        // the main measurement tho is the relative change which this setup should be able to capture fairly well

                        

                        println!("[+] Sending SlowNoData");
                        // 1.Slow traffic, w/ no data
                        for _ in 0..2000 {
                            let mut send_data = WSData {
                                experiment_id: ExperimentID::SlowNoData,
                                timestamp: 0,
                                data: [].to_vec(),
                            };
                            send_data.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

                            // json serialize the data
                            let data = serde_json::to_string(&send_data).expect("serialize");
                            let data = format!("{}\0", data);
                            let bytes = data.as_bytes();

                            stream.write_all(bytes).await.expect("write");
                            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                        }

                        println!("[+] Sending SlowWithData");
                        // 2.Slow traffic, w/ data
                        for _ in 0..2000 {
                            let mut send_data = WSData {
                                experiment_id: ExperimentID::SlowWithData,
                                timestamp: 0,
                                data: small_buffer.to_vec(),
                            };
                            send_data.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

                            // json serialize the data
                            let data = serde_json::to_string(&send_data).expect("serialize");
                            let data = format!("{}\0", data);
                            let bytes = data.as_bytes();

                            stream.write_all(bytes).await.expect("write");
                            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                        }

                        println!("[+] Sending BurstNoData");
                        // 3.Burst traffic, w/ no data
                        for _ in 0..15000 {
                            let mut send_data = WSData {
                                experiment_id: ExperimentID::BurstNoData,
                                timestamp: 0,
                                data: [].to_vec(),
                            };
                            send_data.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

                            // json serialize the data
                            let data = serde_json::to_string(&send_data).expect("serialize");
                            let data = format!("{}\0", data);
                            let bytes = data.as_bytes();

                            stream.write_all(bytes).await.expect("write");
                            tokio::time::sleep(tokio::time::Duration::from_micros(50)).await;
                        }

                        println!("[+] Sending BurstWithData");
                        // 4.Burst traffic, w/ data
                        for _ in 0..15000 {
                            let mut send_data = WSData {
                                experiment_id: ExperimentID::BurstWithData,
                                timestamp: 0,
                                data: small_buffer.to_vec(),
                            };
                            send_data.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

                            // json serialize the data
                            let data = serde_json::to_string(&send_data).expect("serialize");
                            let data = format!("{}\0", data);
                            let bytes = data.as_bytes();

                            stream.write_all(bytes).await.expect("write");
                            tokio::time::sleep(tokio::time::Duration::from_micros(50)).await;
                        }

                        println!("[+] Sending ConsistentNoData");
                        // 5.Consistent traffic, w/ no data
                        for _ in 0..6000 {
                            let mut send_data = WSData {
                                experiment_id: ExperimentID::ConsistentNoData,
                                timestamp: 0,
                                data: [].to_vec(),
                            };
                            send_data.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

                            // json serialize the data
                            let data = serde_json::to_string(&send_data).expect("serialize");
                            let data = format!("{}\0", data);
                            let bytes = data.as_bytes();

                            stream.write_all(bytes).await.expect("write");
                            tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
                        }

                        println!("[+] Sending ConsistentWithData");
                        // 6.Consistent traffic, w/ data
                        for _ in 0..6000 {
                            let mut send_data = WSData {
                                experiment_id: ExperimentID::ConsistentWithData,
                                timestamp: 0,
                                data: small_buffer.to_vec(),
                            };
                            send_data.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

                            // json serialize the data
                            let data = serde_json::to_string(&send_data).expect("serialize");
                            let data = format!("{}\0", data);
                            let bytes = data.as_bytes();

                            stream.write_all(bytes).await.expect("write");
                            tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
                        }

                        println!("[+] Sending SlowLargeData");
                        // 7.Slow traffic, w/ large data
                        for _ in 0..2000 {
                            let mut send_data = WSData {
                                experiment_id: ExperimentID::SlowLargeData,
                                timestamp: 0,
                                data: large_buffer.to_vec(),
                            };
                            send_data.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

                            // json serialize the data
                            let data = serde_json::to_string(&send_data).expect("serialize");
                            let data = format!("{}\0", data);
                            let bytes = data.as_bytes();

                            stream.write_all(bytes).await.expect("write");
                            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                        }

                        println!("[+] Sending ConsistentLargeData");
                        // 8.Consistent traffic, w/ large data
                        for _ in 0..6000 {
                            let mut send_data = WSData {
                                experiment_id: ExperimentID::ConsistentLargeData,
                                timestamp: 0,
                                data: large_buffer.to_vec(),
                            };
                            send_data.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

                            // json serialize the data
                            let data = serde_json::to_string(&send_data).expect("serialize");
                            let data = format!("{}\0", data);
                            let bytes = data.as_bytes();

                            stream.write_all(bytes).await.expect("write");
                            tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
                        }

                        println!("[+] Sending BurstLargeData");
                        // 9.Burst traffic, w/ small data
                        for _ in 0..15000 {
                            let mut send_data = WSData {
                                experiment_id: ExperimentID::BurstLargeData,
                                timestamp: 0,
                                data: large_buffer.to_vec(),
                            };
                            send_data.timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();

                            // json serialize the data
                            let data = serde_json::to_string(&send_data).expect("serialize");
                            let data = format!("{}\0", data);
                            let bytes = data.as_bytes();

                            stream.write_all(bytes).await.expect("write");
                            tokio::time::sleep(tokio::time::Duration::from_micros(50)).await;
                        }

                        println!("[+] All Experiments Complete");
                    }
                });
                handles.push(handle_1);

                for handle in handles {
                    handle.await.expect("join handle");
                }
            });
        }
    });


    // =-= Client =-= //
    // want core affinity on one thread, using tokio runtime for localset


    let client_thread = std::thread::spawn(move || {
        let res = core_affinity::set_for_current(core_affinity::CoreId { id: 1 });
        if res {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .worker_threads(1)
                .build()
                .expect("build runtime");
            let local = tokio::task::LocalSet::new();
            local.block_on(&rt, async {
                let mut handles = Vec::new();
                let handle_1 = tokio::task::spawn_local(async move {
                    use tokio::io::AsyncReadExt;


                    let mut files: HashMap<ExperimentID, FileHandler> = HashMap::new();
                    files.insert(ExperimentID::SlowNoData, FileHandler::new(&format!("latency/SlowNoData.txt")).unwrap());
                    files.insert(ExperimentID::SlowWithData, FileHandler::new(&format!("latency/SlowWithData.txt")).unwrap());
                    files.insert(ExperimentID::BurstNoData, FileHandler::new(&format!("latency/BurstNoData.txt")).unwrap());
                    files.insert(ExperimentID::BurstWithData, FileHandler::new(&format!("latency/BurstWithData.txt")).unwrap());
                    files.insert(ExperimentID::ConsistentNoData, FileHandler::new(&format!("latency/ConsistentNoData.txt")).unwrap());
                    files.insert(ExperimentID::ConsistentWithData, FileHandler::new(&format!("latency/ConsistentWithData.txt")).unwrap());
                    files.insert(ExperimentID::SlowLargeData, FileHandler::new(&format!("latency/SlowLargeData.txt")).unwrap());
                    files.insert(ExperimentID::BurstLargeData, FileHandler::new(&format!("latency/BurstLargeData.txt")).unwrap());
                    files.insert(ExperimentID::ConsistentLargeData, FileHandler::new(&format!("latency/ConsistentLargeData.txt")).unwrap());
                    



                    let server_url = "127.0.0.1";
                    let server_port = 7200;

                    // sleep for 5 seconds
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                    println!("[Tokio] Connecting to Server");
                    let mut stream = tokio::net::TcpStream::connect(format!("{}:{}", server_url, server_port)).await.expect("connect");
                    

                    // *definitely* better way to handle the payload and reading the incoming bytes but this is just for testing
                    let mut payload = Vec::new();
                    loop {
                        let mut buffer = [0; 8192];
                        let bytes_read = stream.read(&mut buffer).await.unwrap();
                        if bytes_read == 0 {
                            break;
                        }

                        for i in 0..bytes_read {
                            let byte = buffer[i];
                            if byte == 0 {
                                let stringed_payload = String::from_utf8(payload.clone()).expect("parse");
                                let ws_data = serde_json::from_str::<WSData>(&stringed_payload).expect("deserialize");
                                let latency = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() - ws_data.timestamp;
                                //println!("Latency: {}ns", latency);
                                //println!("Data: {:?}", ws_data);
                                files.get_mut(&ws_data.experiment_id).unwrap().write_line(format!("{}", latency)).unwrap();
                                payload.clear();
                            } else {
                                payload.push(byte);
                            }
                        }

                    }
                });
                handles.push(handle_1);
                for handle in handles {
                    handle.await.expect("join handle");
                }
            });
        }
    });


    server_thread.join().unwrap();
    client_thread.join().unwrap();

}