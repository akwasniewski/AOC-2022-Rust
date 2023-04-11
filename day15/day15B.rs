
use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut sensors: Vec<((i32,i32),i32)>=Vec::new();
    for line in reader.lines(){
        let line=line?;
        let line: String = line.chars().filter(|c| (c.is_digit(10) || c==&'=')).collect();
        let line:Vec<&str>=line.split("=").collect();
        let pos_sensor: (i32,i32)=(line[1].parse::<i32>().unwrap(),line[2].parse::<i32>().unwrap());
        let pos_beacon: (i32,i32)=(line[3].parse::<i32>().unwrap(),line[4].parse::<i32>().unwrap());
        let distance=(pos_beacon.0-pos_sensor.0).abs()+(pos_beacon.1-pos_sensor.1).abs();
        sensors.push((pos_sensor, distance));
    }
    let mut pairs:Vec<(usize, usize)>=Vec::new();
    for i in 0..sensors.len(){
        for j in 0..sensors.len(){
            if i==j{
                 continue;
            }
            if get_distance(sensors[i].0, sensors[j].0)==sensors[i].1+sensors[j].1+2{
                let pair: (usize, usize);
                if sensors[i].0.0<sensors[j].0.0{
                    pair=(i,j);
                }
                else{
                    pair=(j,i);
                }
                if !pairs.contains(&pair){
                    pairs.push(pair);
                }
            }
        }
    }
    //if there werent two pairs would have to find a point for each
    // and then check if it is not within range for any other
    println!("pairs: {}",pairs.len());
    //two pairs so perfect
    let mut y:i64=0;
    let mut x:i64=0;
    for pair in &mut pairs{
        if sensors[pair.0].0.1<sensors[pair.1].0.1{
            //first is bottom left
            let val=sensors[pair.0].0.1+sensors[pair.0].1+1+sensors[pair.0].0.0;
            x+=val as i64;
            y+=val as i64;
        }
        else{
            //first is top right
            let val=-sensors[pair.0].1-1-sensors[pair.0].0.0+sensors[pair.0].0.1;
            x-=val as i64;
            y+=val as i64;
        }
    }
    let ans: i64;
    x/=2;
    y/=2;
    ans=x*4000000+y;
    println!("x: {} y: {}", x, y);
    println!("ans: {}", ans);
    Ok(())
}
fn get_distance(pos1: (i32, i32), pos2: (i32, i32))->i32{
    (pos1.0-pos2.0).abs()+(pos1.1-pos2.1).abs()
}