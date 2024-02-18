mod tsdb;
use tsdb::tsdb;

fn main() {
    println!("Hello, world!");
    //print!("Hello, world!"); doesnt print anything on a new line unless you use \n.
    tsdb(); // runs the tsdb function from tsdb.rs
}
