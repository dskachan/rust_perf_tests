/*!
 * \file        main.rs
 * \author      Dmitry Kachan
 * \date        19.10.16
 * \brief       file contains performance tests for evaluating of default time costs for getting
 *              timestamps, and evaluating of the values of mean sleep error for rust.
 */


extern crate time;

use std::fs::File;
use std::path::Path;
use std::io::prelude::*; // for write_fmt
use std::io::Result;


fn main() {
    let amount: usize = 1000000;
    let res = test_time_cost(amount);
    match res{
        Ok(()) =>{ },
        Err(e)=>println!("oops, trouble! : {:?}",e),
    }
    let dur_1_us: std::time::Duration = std::time::Duration::new(0,1000);
    let dur_5_us: std::time::Duration = std::time::Duration::new(0,5000);
    let dur_10_us: std::time::Duration = std::time::Duration::new(0,10000);
    let dur_100_us: std::time::Duration = std::time::Duration::new(0,100000);
    let dur_1_ms: std::time::Duration = std::time::Duration::new(0,1000000);
    let dur_10_ms: std::time::Duration = std::time::Duration::new(0,10000000);
    let dur_vect = vec![dur_1_us,dur_5_us,dur_10_us,dur_100_us,dur_1_ms,dur_10_ms];
    for dur in dur_vect{
        let res = test_sleep_cost(amount,dur);
        match res{
            Ok(()) =>{ },
            Err(e)=>println!("oops, trouble! : {:?}",e),
        }
    }

}
fn test_sleep_cost(amount: usize,dur: std::time::Duration)->Result<()>{
    let mut v:Vec<u64> = Vec::new();
    v.reserve(amount);
    let start = time::precise_time_ns();
    let mut local_start: u64;
    let mut local_end: u64;
    for i in 1..amount {
        local_start = time::precise_time_ns();
        std::thread::sleep(dur);
        local_end = time::precise_time_ns();
        v.push(local_end-local_start);
    }
    let end = time::precise_time_ns();
    let duration = end - start;
    println!("test sleep");
    println!("test duration: {} ms",duration/(1000000 as u64));
    println!("duration per measurement: {} ns",(duration/(amount as u64)));
    let str = format!("rust_test_sleep_{}_us.log",(dur.subsec_nanos()/(1000 as u32)));

    save_vect_to_file(v,Path::new(&str))
}

fn test_time_cost(amount :usize)->Result<()>{
    let mut v: Vec<u64> = Vec::new();
    v.reserve(amount);
    let start = time::precise_time_ns();
    for i in 1..amount {
        v.push(time::precise_time_ns());
    }
    let end = time::precise_time_ns();
    let duration = end - start;
    println!("test time measurement");
    println!("test duration: {}",duration);
    println!("duration per measurement: {}",duration/(amount as u64));
    save_vect_to_file(v,Path::new("rust_test_time_cost.log"))
}

fn save_vect_to_file(vect: Vec<u64>, p: &Path) -> Result<()> {
    let mut f = try!(File::create(p));
    for elm in vect{
        try!(f.write_fmt(format_args!("{}\n",elm)));
    };
    Ok(())
}
