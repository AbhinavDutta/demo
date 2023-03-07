use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;



pub fn reading_json(fname:&str) -> Vec<(String, f64, i64)> {
    // File hosts must exist in current path before this produces output
    let mut parseddata  : Vec<(String,f64,i64)> = Vec::with_capacity(5000); //symbol,price,timestamp
    
    if let Ok(lines) = read_lines(fname) {
        // Consumes the iterator, returns an (Optional) String
        
        for line in lines {
            if let Ok(ip) = line {
                let words: Vec<&str> = ip.split(",").collect();
                let mut v:Vec<&str> = Vec::with_capacity(10); //symbol,timestamp,bid,ask
                for (i,word) in words.iter().enumerate() {
                    let sp: Vec<&str> = word.split(":").collect();
                    if sp.len()>1 && (i== 2 || i == 3 || i == 5 || i==7) {
                        v.push(sp[1]);
                    }
                }

                let bid_num: f64 = remove_first_last(v[1]).parse().unwrap();
                let ask_num: f64 = remove_first_last(v[2]).parse().unwrap();
                let mut price_num: f64 = (bid_num+ask_num)*1.0f64/2.0;
                price_num = (price_num * 10000000000.0).round() / 10000000000.0;
                let timestamp: i64 = v[3].to_string().parse().unwrap();
                parseddata.push((v[0].to_string(),price_num,timestamp));

             

            }
        }
    }
    parseddata

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
fn remove_first_last(inp:&str) -> String {
    let mut s = inp.to_string();
    s.pop();      // remove last
    if s.len() > 0 {
        s.remove(0);  // remove first
    }
    s
}


pub fn break_by_symbols(data:&Vec<(String, f64, i64)>) -> HashMap<String, Vec<(f64,i64)>> {
    let mut table:HashMap<String, Vec<(f64,i64)>> = HashMap::new();

    for (key,val,timestamp) in data {
    table.entry(key.to_string())
    .or_insert_with(Vec::new)
    .push((*val,*timestamp));
    }
    table

}

pub fn write_json(fname:&str, data:&Vec<(String, f64, i64)>, open:&mut HashMap<String,VecDeque<f64>>, low:&mut HashMap<String,VecDeque<f64>>, high:&mut HashMap<String,VecDeque<f64>>, close:&mut HashMap<String,VecDeque<f64>> )  {
    let mut file = match File::create(&fname) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let mut output: String = String::new();
    for (key,_val,timestamp) in data {
        let open_price = format!("{}",open.get(key).expect("error1").front().expect("error2"));
        open.get_mut(key).expect("error").pop_front();
        
        let close_price = format!("{}",close.get(key).expect("error").front().expect("error"));
        close.get_mut(key).expect("error").pop_front();
        
        let high_price = format!("{}",high.get(key).expect("error").front().expect("error"));
        high.get_mut(key).expect("error").pop_front();

        let low_price = format!("{}",low.get(key).expect("error").front().expect("error"));
        low.get_mut(key).expect("error").pop_front();
        
        let result = format!("{{\"symbol\":{},\"timestamp\":{},\"open\":\"{}\",\"high\":\"{}\",\"low\":\"{}\",\"close\":\"{}\"}}\n",key, timestamp, open_price, high_price, low_price, close_price);
    
        output.push_str(&result);
        
    }

    match file.write_all(output.as_bytes()){
        Err(why) => panic!("couldn't write {}", why),
        Ok(_) => println!("Task done!"),
    }

}
