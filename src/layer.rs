use std::fs::File;

pub struct Layer {
    pub data: Vec<Vec<i32>>
}

impl Layer {
    pub fn load(path: &str) -> Self {
        let file = File::open(path).expect(format!("File {} does not exist", path).as_str());
        let mut data = Vec::new();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);
        for result in rdr.records() {
            let record = result.unwrap();
            data.push(record
                .iter()
                .map(|r| r.parse::<i32>().expect("Record is not an int"))
                .collect::<Vec<_>>());
        }

        Self {
            data
        }
    }
}
