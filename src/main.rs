use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub ts: u64,  // timestamp in ms
    pub val: f64, // value
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chunk {
    pub points: Vec<Point>,
}

pub struct DB {
    buffer: Vec<Point>,
    path: String,
}

impl DB {
    pub fn new(path: &str) -> Self {
        std::fs::create_dir_all(path).ok();
        DB {
            buffer: Vec::new(),
            path: path.to_string(),
        }
    }

    pub fn insert(&mut self, ts: u64, val: f64) {
        self.buffer.push(Point { ts, val });
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        if self.buffer.is_empty() {
            return Ok(());
        }

        // Sort by timestamp
        self.buffer.sort_by_key(|p| p.ts);

        // Serialize to bytes
        let chunk = Chunk {
            points: self.buffer.clone(),
        };
        let bytes = bincode::serialize(&chunk).unwrap();

        // Write to file
        let filename = format!("{}/chunk_{}.bin", self.path, self.buffer[0].ts);
        let mut file = File::create(&filename)?;
        file.write_all(&bytes)?;

        println!("Flushed {} points to {}", self.buffer.len(), filename);
        self.buffer.clear();

        Ok(())
    }

    pub fn query(&self, start: u64, end: u64) -> std::io::Result<Vec<Point>> {
        let mut results = Vec::new();

        // List all chunk files
        for entry in std::fs::read_dir(&self.path)? {
            let entry = entry?;
            let path = entry.path();

            if !path.to_string_lossy().ends_with(".bin") {
                continue;
            }

            // Read and deserialize chunk
            let mut file = File::open(&path)?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;

            let chunk: Chunk = bincode::deserialize(&bytes).unwrap();

            // Filter points in time range
            for point in chunk.points {
                if point.ts >= start && point.ts <= end {
                    results.push(point);
                }
            }
        }

        results.sort_by_key(|p| p.ts);
        Ok(results)
    }
}

fn main() -> std::io::Result<()> {
    let mut db = DB::new("mydb");

    // Insert data
    println!("Inserting data...");
    for i in 0..100 {
        db.insert(i * 1000, (i as f64) * 1.5);
    }
    println!("Buffer: {} points", db.buffer.len());

    // Flush to disk
    println!("\nFlushing...");
    db.flush()?;

    // Query back
    println!("\nQuerying data from 10s to 50s...");
    let results = db.query(10_000, 50_000)?;
    println!("Found {} points:", results.len());
    for p in &results[..std::cmp::min(5, results.len())] {
        println!("  ts={}, val={}", p.ts, p.val);
    }

    Ok(())
}
