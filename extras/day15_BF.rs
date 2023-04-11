//for fun tried to bruteforce
//took 3.5 hours to run
//but it worked
use std::fs::File;
use std::io::{self,prelude::*, BufReader};
use std::time::Instant;
fn main() -> io::Result<()>{
    let now = Instant::now();
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let size: i32=4000000;
    let mut pos: Vec<((i32,i32),(i32,i32))>=Vec::new();
    for line in reader.lines(){
        let line=line?;
        let line: String = line.chars().filter(|c| (c.is_digit(10) || c==&'=')).collect();
        let line:Vec<&str>=line.split("=").collect();
        let pos_sensor: (i32,i32)=(line[1].parse::<i32>().unwrap(),line[2].parse::<i32>().unwrap());
        let pos_beacon: (i32,i32)=(line[3].parse::<i32>().unwrap(),line[4].parse::<i32>().unwrap());
        pos.push((pos_sensor, pos_beacon));
    }
    let mut ans_x:i32=0;
    let mut ans_y:i32=0;
    'find:for row in 0..size{
        if row%10000==0{
            let elapsed = now.elapsed();
            println!("{} {:.2?}", row, elapsed);
        }
        let mut map: Vec<bool>=vec!(false;(size+1) as usize); //would rather use arrays but segfaults
        let mut count: i32=0;
        for cur in &pos{
            let pos_beacon=cur.1;
            let pos_sensor=cur.0;
            let distance=(pos_beacon.0-pos_sensor.0).abs()+(pos_beacon.1-pos_sensor.1).abs();
            let horizontal=distance-(pos_sensor.1-row).abs();
            if horizontal>=0{
                if pos_sensor.0<=size{
                    if map[(pos_sensor.0) as usize]==false{
                        count+=1;
                        map[(pos_sensor.0) as usize]=true;
                    }
                }
            }
            for i in 0..horizontal{
                if (pos_sensor.0+i+1)<=size {
                    map[(pos_sensor.0+i+1) as usize]=true;
                }
                if (pos_sensor.0-i-1)>=0 && (pos_sensor.0-i-1)<=size{
                    map[(pos_sensor.0-i-1) as usize ]=true;
                }
            }
        }
        for i in 0..size{
            if map[i as usize]==false{
                ans_x=i;
                ans_y=row;
                break 'find;
            }
        }
    }
    println!("{} {}", ans_x, ans_y);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}