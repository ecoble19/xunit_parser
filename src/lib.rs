
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;
extern crate chrono;

use std::path::PathBuf;
use serde_xml_rs::from_reader;
use std::io::{BufReader, Read};
use std::fs::File;
use chrono::{DateTime, Utc, NaiveTime};

#[derive(Debug, Deserialize)]
pub enum Outcome {
    Passed, Failed
}

#[derive(Debug, Deserialize)]
pub struct TestRun {
    pub id: String,

    #[serde(rename="Times")]
    pub times: Times,

    #[serde(rename="Results")]
    pub results: Result,
}


#[derive(Debug, Deserialize)]
pub struct Times {
    creation: DateTime<Utc>,
    start: DateTime<Utc>,
    finish: DateTime<Utc>,


}

#[derive(Debug, Deserialize)]
pub struct Result {
    #[serde(rename="UnitTestResult")]
    test_results: Vec<UnitTestResult>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UnitTestResult {
    execution_id: String,
    test_id: String,
    test_name: String,
    computer_name: String,
    duration: NaiveTime,
    start_time: DateTime<Utc>,
    end_time: DateTime<Utc>,
    test_type: String,
    outcome: Outcome,
    test_list_id: String,
    relative_results_directory: String,

}



pub fn read(p: PathBuf) -> TestRun {
    let file = File::open(p).unwrap();
    let mut file = BufReader::new(file);
    let mut buf = vec![0;3];
    file.read(&mut buf).unwrap();
    if (buf[0] == 0xef) && (buf[1] == 0xbb) && (buf[2] == 0xbf) {
        // If this is true there is a bom
        // for now we will assume everything has a bom and just continue forward
    }

    from_reader::<_, TestRun>(file).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::read;
    use std::path::PathBuf;
    use std::env;

    #[test]
    fn it_works() {
        let x = read(PathBuf::from("./files/output.xml"));
        println!("{:#?}", x);
    }
}
