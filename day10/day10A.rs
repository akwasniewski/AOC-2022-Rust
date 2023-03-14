use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut sum: i32 = 0;
    let mut reg: i32 = 1;
    let mut cycle: i32  = 0;
    let mut on_next: i32  = 0;
    for line in reader.lines()
    {
        cycle+=1;
        reg+=on_next;
        on_next=0;
        //println!("{}, {}", cycle, reg);
        if ((cycle+20)%40)==0 {sum+=reg*cycle;}
        let line = line?;
        if line=="noop" {continue;}
        //else
        let line: Vec<&str> = line.split(" ").collect();
        let num = line[1].parse::<i32>().unwrap();
        cycle+=1;
        if ((cycle+20)%40)==0 {sum+=reg*cycle;}
        //println!("{}, {}", cycle, reg);
        on_next = num;
    }
    println!("sum: {sum}");
    Ok(())
}