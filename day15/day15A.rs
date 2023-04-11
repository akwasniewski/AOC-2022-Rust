use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    use std::time::Instant;
    let now = Instant::now();
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let row: i32=2000000;
    let size: i32=10000000;
    let mut map: Vec<bool>=vec!(false;size as usize); //would rather use arrays but segfaults

    //true cant be
    let mut to_check: Vec<i32>=Vec::new(); //x values of beacons on row to check after
    for line in reader.lines(){
        let line=line?;
        let line: String = line.chars().filter(|c| (c.is_digit(10) || c==&'=')).collect();
        let line:Vec<&str>=line.split("=").collect();
        let pos_sensor: (i32,i32)=(line[1].parse::<i32>().unwrap(),line[2].parse::<i32>().unwrap());
        let pos_beacon: (i32,i32)=(line[3].parse::<i32>().unwrap(),line[4].parse::<i32>().unwrap());
        if pos_beacon.1==row{
            to_check.push(pos_beacon.0);
        }
        let distance=(pos_beacon.0-pos_sensor.0).abs()+(pos_beacon.1-pos_sensor.1).abs();
        let horizontal=distance-(pos_sensor.1-row).abs();
        if horizontal>=0{
            map[(pos_sensor.0+size/2) as usize]=true;
        }
        for i in 0..horizontal{
            map[(pos_sensor.0+i+1+size/2) as usize]=true;
            map[(pos_sensor.0-i-1+size/2) as usize ]=true;
        }
    }
    for i in to_check{
        map[(i+size/2) as usize]=false;
    }
    let mut count:i32=0;
    for i in map{
        if i==true{
            count+=1;
        }
    }
    println!("count: {}",count);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}