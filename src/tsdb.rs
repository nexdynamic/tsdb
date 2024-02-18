pub struct TsDb {
    data: Vec<i32>, // storing only the data points for simplicity
}

impl TsDb {
    pub fn new() -> Self {
        TsDb { data: Vec::new() }
    }

    // adds the data point to the tsdb
    pub fn insert(&mut self, value: i32) {
        self.data.push(value);
    }

    // should retrieve the data points.
    pub fn query_all(&self) -> &[i32] {
        &self.data
    }
}

pub fn tsdb() {
    println!("TSDB!");
    let mut tsdb = TsDb::new();

    tsdb.insert(10);
    tsdb.insert(20);
    tsdb.insert(30);

    let all_data = tsdb.query_all();
    println!("All Data Points: {:?}", all_data);
}
