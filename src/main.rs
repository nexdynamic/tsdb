mod saving;
mod tsdb;
//use tsdb::tsdb;

fn main() {
    //tsdb(); // disabled for to test new file saving systme.
    if let Err(e) = saving::saving() {
        println!("An error occurred: {}", e);
    }
}
