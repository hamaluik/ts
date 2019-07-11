#[macro_use]
extern crate clap;

use std::fs;
use std::path;
use std::collections::HashMap;
use std::io;

mod cli;

type Record = (String, String, f64);

fn write_records<T: io::Write>(writer: T, records: &[Record]) -> Result<(), Box<std::error::Error>> {
    let mut wtr = csv::Writer::from_writer(writer);
    //wtr.write_record(&["date", "description", "hours"])?;
    for record in records {
        wtr.write_record(&[&record.0, &record.1, &format!("{}", record.2)])?;
    }
    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<std::error::Error>> {
    let matches = cli::build_cli().get_matches();

    let csv_filename = matches.value_of("csv-file").ok_or("missing csv file?")?;
    let csv_path = path::PathBuf::from(csv_filename);
    let csv_path = fs::canonicalize(csv_path).map_err(|_| "failed to canonicalize csv file path!")?;
    if !csv_path.exists() {
        return Err(Box::from(format!("{} doesn't exist!", csv_path.display())));
    }
    if !csv_path.is_file() {
        return Err(Box::from(format!("{} isn't a file!", csv_path.display())));
    }

    let out_path = matches.value_of("OUTPUT");

    let file = fs::File::open(csv_path)?;

    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(file);
    let mut records: HashMap<(String, String), f64> = HashMap::new();
    for result in rdr.records() {
        let record = result?;

        let date = String::from(record.get(7).ok_or("can't parse date")?);
        let description: String = format!(
            "{}: {}",
            record.get(3).ok_or("can't parse project")?,
            record.get(5).ok_or("can't parse description")?
        );
        let hours: f64 = record.get(12).ok_or("can't parse hours")?.parse::<f64>()?;

        records.entry((date, description))
            .and_modify(|v| *v += hours)
            .or_insert(hours);
    }
    let mut records = records.iter().map(|(key, hours)| (key.0.clone(), key.1.clone(), *hours)).collect::<Vec<Record>>();
    records.sort_by(|a, b| a.0.cmp(&b.0));

    match out_path {
        Some(p) => {
            let file_out = fs::File::create(p)?;
            write_records(file_out, &records)?;
            println!("Time sheet saved to {}!", p);
        },
        None => write_records(io::stdout(), &records)?
    }

    Ok(())
}
