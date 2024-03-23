use std::time::{SystemTime, UNIX_EPOCH};
use rand::thread_rng;
use rand::RngCore;
use serde::{Serialize, Deserialize};
use std::fs::OpenOptions;
use std::io::{self, Write};
use tokio::io::BufReader;


// =-= Text Coloring =-= //
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


// =-= ExperimentID =-= //

#[derive(Debug, Clone, Serialize, Deserialize)]
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


// =-= WSData =-= //

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WSData {
    pub experiment_id: ExperimentID,
    pub timestamp: u128,
    pub data: Vec<u8>,
}


// =-= Main =-= //

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
                .expect("[!][Server] Failed to build tokio runtime");
            let local = tokio::task::LocalSet::new();
            local.block_on(&rt, async {
                use tokio::io::AsyncWriteExt;

                // =---------------------------------------------------------= //
                // =-= TcpListener =-= //


                let server_url = "127.0.0.1";
                let server_port = 7200;

                let listener = tokio::net::TcpListener::bind(format!("{}:{}", server_url, server_port)).await.expect("[!][Server] Failed to bind to ip:port");
                println!("{}[+][Server] Listening on: {}{}", CL::DullTeal.to_string(), server_port, CL::End.to_string());

                // only accepting one connection so no need to loop
                let (mut stream, _) = listener.accept().await.expect("[!][Server] Failed to accept connection");


                // =---------------------------------------------------------= //
                // =-= Buffers =-= //

                let mut rng: rand::rngs::ThreadRng = thread_rng(); // Random number generator
                
                let mut small_buffer: [u8; 32] = [0u8; 32]; // 32 bytes (w/ Data)
                rng.fill_bytes(&mut small_buffer); // Fill the buffer with random bytes

                let mut large_buffer: [u8; 256] = [0u8; 256]; // 256 bytes (Large Data)
                rng.fill_bytes(&mut large_buffer); // Fill the buffer with random bytes


                // =---------------------------------------------------------= //
                // Let's begin!

                let slow_sample_size: i32 = 2000;
                let burst_sample_size: i32 = 15000;
                let consistent_sample_size: i32 = 6000;
                

                // =-= Slow Traffic =-= //

                println!("{}[-][Server] Sending SlowNoData{}", CL::Dull.to_string(), CL::End.to_string());
                // Slow traffic, w/ no data
                for _ in 0..slow_sample_size {
                    let send_data = WSData {
                        experiment_id: ExperimentID::SlowNoData,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
                        data: vec![],
                    };

                    let mut bytes = serde_json::to_vec(&send_data).expect("[!][Server] Error serializing data");
                    bytes.push(0);

                    stream.write_all(&bytes).await.expect("[!][Server] Failed to write to stream");
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }

                println!("{}[-][Server] Sending SlowWithData{}", CL::Dull.to_string(), CL::End.to_string());
                // Slow traffic, w/ data
                for _ in 0..slow_sample_size {
                    let send_data = WSData {
                        experiment_id: ExperimentID::SlowWithData,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
                        data: small_buffer.to_vec(),
                    };

                    let mut bytes = serde_json::to_vec(&send_data).expect("[!][Server] Error serializing data");
                    bytes.push(0);

                    stream.write_all(&bytes).await.expect("[!][Server] Failed to write to stream");
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }

                println!("{}[-][Server] Sending SlowLargeData{}", CL::Dull.to_string(), CL::End.to_string());
                // Slow traffic, w/ large data
                for _ in 0..slow_sample_size {
                    let send_data = WSData {
                        experiment_id: ExperimentID::SlowLargeData,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
                        data: large_buffer.to_vec(),
                    };

                    let mut bytes = serde_json::to_vec(&send_data).expect("[!][Server] Error serializing data");
                    bytes.push(0);

                    stream.write_all(&bytes).await.expect("[!][Server] Failed to write to stream");
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                }


                // =-= Burst Traffic =-= //

                println!("{}[-][Server] Sending BurstNoData{}", CL::Dull.to_string(), CL::End.to_string());
                // Burst traffic, w/ no data
                for _ in 0..burst_sample_size {
                    let send_data = WSData {
                        experiment_id: ExperimentID::BurstNoData,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
                        data: vec![],
                    };

                    let mut bytes = serde_json::to_vec(&send_data).expect("[!][Server] Error serializing data");
                    bytes.push(0);

                    stream.write_all(&bytes).await.expect("[!][Server] Failed to write to stream");
                    tokio::time::sleep(tokio::time::Duration::from_micros(50)).await;
                }

                println!("{}[-][Server] Sending BurstWithData{}", CL::Dull.to_string(), CL::End.to_string());
                // Burst traffic, w/ data
                for _ in 0..burst_sample_size {
                    let send_data = WSData {
                        experiment_id: ExperimentID::BurstWithData,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
                        data: small_buffer.to_vec(),
                    };

                    let mut bytes = serde_json::to_vec(&send_data).expect("[!][Server] Error serializing data");
                    bytes.push(0);

                    stream.write_all(&bytes).await.expect("[!][Server] Failed to write to stream");
                    tokio::time::sleep(tokio::time::Duration::from_micros(50)).await;
                }

                println!("{}[-][Server] Sending BurstLargeData{}", CL::Dull.to_string(), CL::End.to_string());
                // Burst traffic, w/ large data
                for _ in 0..burst_sample_size {
                    let send_data = WSData {
                        experiment_id: ExperimentID::BurstLargeData,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
                        data: large_buffer.to_vec(),
                    };

                    let mut bytes = serde_json::to_vec(&send_data).expect("[!][Server] Error serializing data");
                    bytes.push(0);

                    stream.write_all(&bytes).await.expect("[!][Server] Failed to write to stream");
                    tokio::time::sleep(tokio::time::Duration::from_micros(50)).await;
                }


                // =-= Consistent Traffic =-= //

                println!("{}[-][Server] Sending ConsistentNoData{}", CL::Dull.to_string(), CL::End.to_string());
                // Consistent traffic, w/ no data
                for _ in 0..consistent_sample_size {
                    let send_data = WSData {
                        experiment_id: ExperimentID::ConsistentNoData,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
                        data: vec![],
                    };

                    let mut bytes = serde_json::to_vec(&send_data).expect("[!][Server] Error serializing data");
                    bytes.push(0);

                    stream.write_all(&bytes).await.expect("[!][Server] Failed to write to stream");
                    tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
                }

                println!("{}[-][Server] Sending ConsistentWithData{}", CL::Dull.to_string(), CL::End.to_string());
                // Consistent traffic, w/ data
                for _ in 0..consistent_sample_size {
                    let send_data = WSData {
                        experiment_id: ExperimentID::ConsistentWithData,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
                        data: small_buffer.to_vec(),
                    };

                    let mut bytes = serde_json::to_vec(&send_data).expect("[!][Server] Error serializing data");
                    bytes.push(0);

                    stream.write_all(&bytes).await.expect("[!][Server] Failed to write to stream");
                    tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
                }

                

                println!("{}[-][Server] Sending ConsistentLargeData{}", CL::Dull.to_string(), CL::End.to_string());
                // Consistent traffic, w/ large data
                for _ in 0..consistent_sample_size {
                    let send_data = WSData {
                        experiment_id: ExperimentID::ConsistentLargeData,
                        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
                        data: large_buffer.to_vec(),
                    };

                    let mut bytes = serde_json::to_vec(&send_data).expect("[!][Server] Error serializing data");
                    bytes.push(0);

                    stream.write_all(&bytes).await.expect("[!][Server] Failed to write to stream");
                    tokio::time::sleep(tokio::time::Duration::from_millis(15)).await;
                }

                

                println!("{}[+][Server] All Experiments Complete{}", CL::Green.to_string(), CL::End.to_string());
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
                .expect("[!][Client] Failed to build tokio runtime");
            let local = tokio::task::LocalSet::new();
            local.block_on(&rt, async {
                use tokio::io::AsyncBufReadExt;
                
                let mut slow_no_data_file = FileHandler::new(&format!("latency/SlowNoData.txt")).unwrap();
                let mut slow_with_data_file = FileHandler::new(&format!("latency/SlowWithData.txt")).unwrap();
                let mut burst_no_data_file = FileHandler::new(&format!("latency/BurstNoData.txt")).unwrap();
                let mut burst_with_data_file = FileHandler::new(&format!("latency/BurstWithData.txt")).unwrap();
                let mut consistent_no_data_file = FileHandler::new(&format!("latency/ConsistentNoData.txt")).unwrap();
                let mut consistent_with_data_file = FileHandler::new(&format!("latency/ConsistentWithData.txt")).unwrap();
                let mut slow_large_data_file = FileHandler::new(&format!("latency/SlowLargeData.txt")).unwrap();
                let mut burst_large_data_file = FileHandler::new(&format!("latency/BurstLargeData.txt")).unwrap();
                let mut consistent_large_data_file = FileHandler::new(&format!("latency/ConsistentLargeData.txt")).unwrap();

                let server_url = "127.0.0.1";
                let server_port = 7200;

                // sleep for 2 seconds incase server is not ready
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

                println!("{}[-][Client] Connecting to Server{}", CL::Dull.to_string(), CL::End.to_string());
                let stream = tokio::net::TcpStream::connect(format!("{}:{}", server_url, server_port)).await.expect("[!][Client] Failed to connect to the server");
                let mut stream = BufReader::new(stream);
                println!("{}[+][Client] Connected & ready to receive data{}", CL::DullTeal.to_string(), CL::End.to_string());

                let mut payload = Vec::new();
                loop {
                    payload.clear();
                    stream.read_until(0, &mut payload).await.expect("[!][Client] Failed to read data"); // read until the appended null byte

                    if payload.is_empty() {
                        break;
                    }

                    // Remove the null byte from the end of the payload
                    payload.pop();

                    // Deserialize and process the payload
                    let ws_data = serde_json::from_slice::<WSData>(&payload).expect("[!][Client] Failed to deserialize data");
                    let latency = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() - ws_data.timestamp;
                    //println!("Latency: {}ns", latency);
                    //println!("Data: {:?}", ws_data);

                    match ws_data.experiment_id {
                        ExperimentID::SlowNoData => slow_no_data_file.write_line(latency.to_string()).expect("[!][Client] Failed to write to file"),
                        ExperimentID::SlowWithData => slow_with_data_file.write_line(latency.to_string()).expect("[!][Client] Failed to write to file"),
                        ExperimentID::BurstNoData => burst_no_data_file.write_line(latency.to_string()).expect("[!][Client] Failed to write to file"),
                        ExperimentID::BurstWithData => burst_with_data_file.write_line(latency.to_string()).expect("[!][Client] Failed to write to file"),
                        ExperimentID::ConsistentNoData => consistent_no_data_file.write_line(latency.to_string()).expect("[!][Client] Failed to write to file"),
                        ExperimentID::ConsistentWithData => consistent_with_data_file.write_line(latency.to_string()).expect("[!][Client] Failed to write to file"),
                        ExperimentID::SlowLargeData => slow_large_data_file.write_line(latency.to_string()).expect("[!][Client] Failed to write to file"),
                        ExperimentID::BurstLargeData => burst_large_data_file.write_line(latency.to_string()).expect("[!][Client] Failed to write to file"),
                        ExperimentID::ConsistentLargeData => consistent_large_data_file.write_line(latency.to_string()).expect("[!][Client] Failed to write to file"),
                    }

                }
            });
        }
    });


    server_thread.join().unwrap();
    client_thread.join().unwrap();

}
