use std::{sync::{Arc, Mutex}, path::PathBuf, time::Duration};

struct  TimeSeriesPoint {
    timestamp: u64,
    value: f64,
}

struct ChunkIndex {
    start_time: u64,
    end_time: u64,
    file_offset: u64, // where we find the chunk on dixsk
}

pub struct ChunkWriter {
    buffer: Arc<Mutex<Vec<TimeSeriesPoint>>>, 
    flush_interval: Duration,                 
    chunk_dir: PathBuf,                     
}

// TODO: buffer data here.
// we'll try flushing it after t seconds

impl ChunkWriter {
    pub fn new(flush_interval: Duration, chunk_dir: PathBuf) -> Self {
        let buffer = Arc::new(Mutex::new(Vec::new()));

        // tghe background thread will go here

        ChunkWriter {
            buffer,
            flush_interval,
            chunk_dir,
        }
    }
}



pub fn tsdb() {
    
}
  