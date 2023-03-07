use std::collections::VecDeque;
use std::collections::HashMap;

pub fn wrapper_ohlc(table: &HashMap<String, Vec<(f64,i64)>>, k: i64, comp: &dyn Fn(&(f64,i64),&(f64,i64)) -> f64 ) -> HashMap<String, VecDeque<f64>> {
    let mut windowed_table:HashMap<String,VecDeque<f64>> = HashMap::new();
    
    for (key, vector_price) in table {
        let windowed_price = windowed_ohlc(vector_price,k,comp);
        let windowed_price_deque: VecDeque<f64> = VecDeque::from(windowed_price);
        windowed_table.insert(key.to_string(),windowed_price_deque);
    }
    windowed_table
}

fn windowed_ohlc(nums: &Vec<(f64,i64)>, k: i64, comp: &dyn Fn(&(f64,i64),&(f64,i64)) -> f64) -> Vec<f64> {
    let mut price: Vec<f64> = Vec::with_capacity(nums.len());
    let mut queue: VecDeque<usize> = VecDeque::with_capacity(nums.len());

    for idx in 0..nums.len() {
		// if the stored idx at the front (corresponding to the largest number) is not in the current window, 
		// then we have to remove it from the queue
        while let Some(&pos) = queue.front() {
            if  nums[idx].1- nums[pos as usize].1 <= k*60*1000 {
                break;
            }
            queue.pop_front(); 
        }
        // the window's optimum is at the front of the queue, this is done to preserve monotonicity  
        while let Some(&pos) = queue.back() { 
            let old = &nums[pos as usize]; 
            if comp(old , &nums[idx]) > 0f64 {
                break;                      
            } 
            queue.pop_back();           
        }
        queue.push_back(idx);
        if let Some(&max) = queue.front() {
            price.push(nums[max].0);
        } 
    }
    price
}
