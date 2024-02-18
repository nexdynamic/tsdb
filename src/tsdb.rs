pub struct TsDb {
    data: Vec<f64>, // storing only the data points for simplicity
}

impl TsDb {
    pub fn new() -> Self {
        TsDb { data: Vec::new() }
    }

    // adds the data point to the tsdb
    pub fn insert(&mut self, value: f64) {
        self.data.push(value);
    }

    // should retrieve the data points.
    pub fn query_all(&self) -> &[f64] {
        &self.data
    }
}

pub fn tsdb() {
    println!("TSDB!");
    let mut tsdb = TsDb::new();

    // Example usage
    tsdb.insert(10.0);
    tsdb.insert(20.0);
    tsdb.insert(30.0);

    let all_data = tsdb.query_all();
    println!("All Data Points: {:?}", all_data);
}
