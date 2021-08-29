use crate::Args;
use csv::StringRecordsIter;
use std::collections::HashMap;

pub fn run(args: &Args) {
    let mut reader = csv::ReaderBuilder::new() //Builds a CSV reader with various configuration knobs.
        .has_headers(true) //Whether to treat the first row as a special header row.
        .from_path(&args.filename)
        .expect("Unable to read file"); //Build a CSV parser from this configuration that reads data from the given file path.
                                        // let count: &usize = &reader.records().count();
                                        // println!("The total number of records in the file is : {}", count);
                                        // return;
    let headers = reader.headers().expect("Unable to read headers"); //Returns a reference to the first row read by this parser (The headers).
    let header_positions = header_positions(&headers); // returns a map with every header as key and its position as value

    if args.query.clone().unwrap() == "count" {
        display_count(
            reader.records(),
            &args.group_by.clone().unwrap(),
            &header_positions,
        );
    } else if args.query.clone().unwrap() == "avg" || args.query.clone().unwrap() == "sum" {
        display_sum_avg(
            reader.records(),
            &args.group_by.clone().unwrap(),
            &args.query.clone().unwrap(),
            &args.select.clone().unwrap(),
            &header_positions,
        )
    } else {
        panic!("The query you entered is not valid!");
    }
}

fn header_positions(headers: &csv::StringRecord) -> HashMap<String, usize> {
    let mut header_positions: HashMap<String, usize> = HashMap::new();
    for (index, header) in headers.iter().enumerate() {
        header_positions.insert(String::from(header), index);
    }
    header_positions
}
fn display_count<R: std::io::Read>(
    records: StringRecordsIter<R>,
    group_by: &str,
    header_positions: &HashMap<String, usize>,
) {
    let mut count = HashMap::new(); // the count of each subgroup
    let records: Vec<Result<csv::StringRecord, csv::Error>> = records.collect();
    let mut rec: &csv::StringRecord;
    let mut name: &str;
    let header_index = header_positions[group_by];
    for (index, _) in records.iter().enumerate() {
        rec = &records[index].as_ref().expect("unable to read record");
        name = &rec[header_index];
        *count.entry(name.clone()).or_insert(0) += 1;
    }
    println!("{0: <12}   {1: <12}\n", group_by, "count");
    for rec in count {
        println!("{0: <12} | {1: <12}", rec.0, rec.1);
    }
}

fn display_sum_avg<R: std::io::Read>(
    records: StringRecordsIter<R>,
    group_by: &str,
    query: &str,
    select: &str,
    header_positions: &HashMap<String, usize>,
) {
    let mut result: HashMap<&str, f64> = HashMap::new();
    let mut count: HashMap<&str, f64> = HashMap::new(); // the count of each subgroup
    let groupby_header_index = header_positions[group_by];
    let select_header_index = header_positions[select];
    let records: Vec<Result<csv::StringRecord, csv::Error>> = records.collect();
    let mut rec: &csv::StringRecord;
    let mut name: &str;
    let mut field: &str;
    for (index, _) in records.iter().enumerate() {
        rec = records[index].as_ref().expect("unable to read record");
        name = &rec[groupby_header_index];
        field = &rec[select_header_index];

        *result.entry(&name).or_insert(0.0) += &field
            .parse::<f64>()
            .expect("Cannot perform the aggregate function on none numeric values!");
        *count.entry(&name).or_insert(0.0) += 1.0;
    }
    let first_header = String::from(group_by);
    let mut second_header = String::from(query);
    second_header.push_str(" ");
    second_header.push_str(select);
    println!("{0: <12}   {1: <12}\n", first_header, second_header);
    for rec in result {
        if query == "sum" {
            println!("{0: <12} | {1: <12}", rec.0, rec.1);
        } else {
            println!(
                "{0: <12} | {1: <12}",
                rec.0,
                rec.1 as f64 / *count.get(&rec.0).unwrap() as f64
            );
        }
    }
}
