use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut reg: i32 = 1;
    let mut cycle: i32  = 0;
    let mut on_next: i32  = 0;
    let mut cur_line = String::new();
    for line in reader.lines()
    {
        cycle+=1;
        reg+=on_next;
        on_next=0;
        cur_line = draw(cur_line, reg, cycle);
        //println!("{}, {}", cycle, reg);
        let line = line?;
        if line=="noop" {continue;}
        //else
        let line: Vec<&str> = line.split(" ").collect();
        let num = line[1].parse::<i32>().unwrap();
        cycle+=1;
        cur_line = draw(cur_line, reg, cycle);
        //println!("{}, {}", cycle, reg);
        on_next = num;
    }
    Ok(())
}
fn draw(mut cur_line: String, reg: i32, cycle: i32) -> String{
    let pos = cycle%40 - 1;
    if i32::abs(pos-reg) <= 1 {
        cur_line += "#";
    }
    else {cur_line += ".";}
    if cycle%40 == 0{
        println!("{cur_line}");
        cur_line = String::new();
    }
    cur_line
}