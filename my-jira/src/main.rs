mod models;
mod db;

use db::JSONFileDatabase;
use db::Database;

fn main() {
    let json_file = JSONFileDatabase {
        file_path: "data/db.json".to_owned()
    };

    let deserialized = json_file.read_db().unwrap();
    println!("{deserialized:#?}");
}
