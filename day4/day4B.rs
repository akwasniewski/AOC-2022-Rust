use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines()
    {
        let line=line?;
        let ranges: Vec<&str> = line.split(",").collect();
        let ranges_left: Vec<&str> = ranges[0].split("-").collect();
        let ranges_right: Vec<&str> = ranges[1].split("-").collect();
        let ranges_left: (u32, u32) = (ranges_left[0].parse().unwrap(), ranges_left[1].parse().unwrap());
        let ranges_right: (u32, u32) = (ranges_right[0].parse().unwrap(), ranges_right[1].parse().unwrap());
        if ranges_left.1>=ranges_right.0 && ranges_left.0<=ranges_right.1{
            sum+=1;
        }
	}
    println!("total is: {sum}");
    Ok(())
}
