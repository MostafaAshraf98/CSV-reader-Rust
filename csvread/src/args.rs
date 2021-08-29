// this file is for storing args

pub struct Args {
    pub filename: String,         // the CSV file to read (the path)
    pub group_by: Option<String>, // list of columns to display
    pub query: Option<String>,    // query to filter the data
    pub select: Option<String>,   // query to filter the data
}

impl Args {
    pub fn new() -> Args {
        Args {
            filename: String::new(),
            group_by: None,
            query: None,
            select: None,
        }
    }
}
