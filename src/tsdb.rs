use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataPoint {
    timestamp: DateTime<Utc>,
    value: i32, // this uses integers for the data and not floats
}
#[derive(Serialize, Deserialize)]
pub struct TsDb {
    data: Vec<DataPoint>,
}

impl TsDb {
    pub fn new() -> Self {
        TsDb { data: Vec::new() }
    }

    pub fn insert(&mut self, value: i32) {
        self.data.push(DataPoint {
            timestamp: Utc::now(),
            value,
        });
    }

    pub fn query_all(&self) -> &[DataPoint] {
        &self.data
    }
}

pub fn tsdb() {
    let mut tsdb = TsDb::new();
    //prints the retrievead data to the console.
    tsdb.insert(10);
    tsdb.insert(20);
    tsdb.insert(30);

    for data_point in tsdb.query_all() {
        println!(
            "Timestamp: {}, Value: {}",
            data_point.timestamp, data_point.value
        );
    }
}
