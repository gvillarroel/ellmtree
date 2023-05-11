mod tools;

use config::Config;
use crate::tools::Profile;



fn main() {
    let config = Config::builder()
    .add_source(config::File::with_name("test"))
    .build()
    .expect("file does not exist")
    .try_deserialize::<Profile>().expect("Parsing failed");
    
    dbg!(config);
}
