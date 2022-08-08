use fitparser;
use fitparser::de::{from_reader_with_options, DecodeOption};
use fitparser::FitDataRecord;
use md5;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Debug)]
pub struct ParsedFitActivity<'a> {
    session_records: Vec<&'a FitDataRecord>,
    laps_records: Vec<&'a FitDataRecord>,
    device_info_records: Vec<&'a FitDataRecord>,
    activity_records: Vec<&'a FitDataRecord>,
    records: Vec<&'a FitDataRecord>,
    md5: String,
}

fn get_records<'a>(
    all_records: &'a Vec<FitDataRecord>,
    kind: fitparser::profile::MesgNum,
) -> Vec<&'a fitparser::FitDataRecord> {
    let records = all_records
        .iter()
        .filter(|record| record.kind() == kind)
        .collect::<Vec<_>>();
    records
}

pub fn parse_one_file2(file_path: &str) -> Result<String, Box<dyn Error>> {
    let opts = [
        DecodeOption::SkipHeaderCrcValidation,
        DecodeOption::SkipDataCrcValidation,
    ]
    .iter()
    .map(|o| *o)
    .collect();
    let mut fp = File::open(file_path)?;
    let mut buffer: Vec<u8> = Vec::new();
    let _ = fp.read_to_end(&mut buffer);
    let md5 = md5::compute(buffer);
    let all_records = from_reader_with_options(&mut fp, &opts)?;
    parse_all_records(format!("{:x}", md5), &all_records)
}

fn parse_all_records<'a>(
    md5: String,
    all_records: &'a Vec<FitDataRecord>,
) -> Result<String, Box<dyn Error>> {
    let session_records = get_records(&all_records, fitparser::profile::MesgNum::Session);
    let lap_records = get_records(&all_records, fitparser::profile::MesgNum::Lap);
    let device_info_records = get_records(&all_records, fitparser::profile::MesgNum::DeviceInfo);
    let activity_records = get_records(&all_records, fitparser::profile::MesgNum::Activity);
    let records = get_records(&all_records, fitparser::profile::MesgNum::Record);

    let parsed_activity = ParsedFitActivity {
        session_records: session_records,
        laps_records: lap_records,
        device_info_records: device_info_records,
        activity_records: activity_records,
        records: records,
        md5: md5,
    };

    // just serialize all records
    let serialized = serde_json::to_string(&parsed_activity).unwrap();

    Ok(serialized)
}
