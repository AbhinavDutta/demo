pub fn comp_highprice((_a_val,_a_time):&(f64,i64), (_b_val,_b_time):&(f64,i64)) -> f64 {
    _a_val-_b_val
}
pub fn comp_lowprice((_a_val,_a_time):&(f64,i64), (_b_val,_b_time):&(f64,i64)) -> f64 {
    _b_val-_a_val
}
pub fn comp_openprice((_a_val,_a_time):&(f64,i64), (_b_val,_b_time):&(f64,i64)) -> f64 {
    (_b_time-_a_time) as f64
}
pub fn comp_closeprice((_a_val,_a_time):&(f64,i64), (_b_val,_b_time):&(f64,i64)) -> f64 {
    (_a_time-_b_time) as f64
}
