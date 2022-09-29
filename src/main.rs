use std::fs::File;
use chrono::Datelike;
use std::io::prelude::*;

fn main() {


    let mut ms_ledgerx = File::open("files/ms_ledgerx.txt").expect("File not found");
    let mut ms = String::new();
    ms_ledgerx.read_to_string(&mut ms).expect("Error while reading file");

    let mut pg_ledgerx = File::open("files/pg_ledgerx.txt").expect("File not found");
    let mut pg = String::new();
    pg_ledgerx.read_to_string(&mut pg).expect("Error while reading file");

    let mut delta_ledgerx = File::open("files/delta_ledgerx.txt").expect("File not found");
    let mut data3 = String::new();
    delta_ledgerx.read_to_string(&mut data3).expect("Error while reading file");
    changes(&mut pg, &mut ms);
}



fn changes(pg: &str, ms: &str){
    let year = chrono::Utc::now().date().year();
    let month = chrono::Utc::now().date().month();
    let to_match = contatinate_strings(year, month).to_owned();
    for l in ms.lines().rev(){
        let indexes: Vec<&str>  = l.split(',').collect();
        let ms_bookdate = &indexes[24];
        match ms_bookdate.contains(&to_match){
            true => {
                for line in pg.lines(){
                    let a: Vec<&str> = line.split(',').collect();
                    data_clean(&indexes[6..=9], &a[6..=9]);
                }
            },
            false => continue,
            }
        }
    }



fn contatinate_strings(v1: i32 , v2: u32 ) -> String {
    let mut year = v1.to_string();
    let month = v2.to_string();

   
    year.push_str("-0");
    year.push_str(&month);
    let concated_string = year.replace('"', "'");
    concated_string
}



fn data_clean(data1: &[&str], data2:&[&str]) -> bool{
    match data1 == data2{
        true => {
            true
        }
        false => {
            false
        }
    }
}










