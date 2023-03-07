use std::collections::HashMap;
use std::collections::VecDeque;
use ohlc_lib::utils::reading_json;
use ohlc_lib::utils::write_json;
use ohlc_lib::utils::break_by_symbols;
use ohlc_lib::ohlc::wrapper_ohlc;
use ohlc_lib::comparators::comp_highprice;
use ohlc_lib::comparators::comp_lowprice;
use ohlc_lib::comparators::comp_openprice;
use ohlc_lib::comparators::comp_closeprice;



fn main() {
    
    let  original_data:Vec<(String, f64, i64)> = reading_json("data/dataset-b.txt");  //source of input
    let  dataframe:HashMap<String, Vec<(f64,i64)>>  = break_by_symbols(&original_data);
    let  w = 5i64;  //time window in minutes
    
    let mut high:HashMap<String, VecDeque<f64>> = wrapper_ohlc(&dataframe, w,&comp_highprice); 
    let mut low:HashMap<String, VecDeque<f64>> =  wrapper_ohlc(&dataframe, w,&comp_lowprice);
    let mut open:HashMap<String, VecDeque<f64>> =  wrapper_ohlc(&dataframe, w,&comp_openprice);
    let mut close:HashMap<String, VecDeque<f64>> = wrapper_ohlc(&dataframe, w,&comp_closeprice);
     
    write_json("data/ohlc-5m-b.txt",&original_data,&mut open,&mut low,&mut high,&mut close); //destination of output
    
}

