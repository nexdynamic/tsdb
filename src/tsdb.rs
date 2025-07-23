use std::{path::PathBuf, ptr::null};

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
    buffer: Vec<TimeSeriesPoint>,
    max_points: usize,
    chunk_dir: PathBuf
}

// TODO: buffer data here.
// we'll try flushing it after t seconds


pub fn tsdb() {
    
}
  