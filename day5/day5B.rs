use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result <()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut arr: Vec<Vec<char>> = Vec::new();
    arr.resize(9, Vec::new());
    let mut loaded = false;
    for line in reader.lines()
    {
        let line=line?;
        if line.is_empty(){
            continue;
        }
        if !loaded{
            let len = line.len()-1;
            let line: Vec<char> = line.chars().collect();
            if line[1].is_digit(10){
                loaded=true;
                for i in 0..(arr.len()){
                    arr[i].reverse();
                }
                continue;
            }
            for i in 0..len{
                if line[i].is_alphabetic(){
                    let stack_num = (i-1)/4;
                    arr[stack_num].push(line[i]);
                }
            } 
        }
        else{
            let line: Vec<&str> = line.split(" ").collect();
            let line = get_nums(line);
            let len=arr[(line[1]-1) as usize].len() as i32;
            let len = (len - line[0])as usize;
            let dropped: Vec<char> = arr[(line[1]-1) as usize].drain(len..).collect();
            for d in dropped{
                arr[(line[2]-1) as usize].push(d);
            }
        }

	}
    let mut last_crates = String::new();
    for v in arr{
        last_crates.push(*v.last().unwrap_or(&'-'));
    }
    println!("Top crate labels:");
    println!("{}",last_crates);
    Ok(())
}
fn get_nums(line: Vec<&str>) ->Vec<i32>{
    let mut prod: Vec<i32> = Vec::new();
    for w in line{
        let w = w.parse::<i32>();
        let num = match w{
            Ok(num) => num,
            Err(_) => {continue},
        };
        prod.push(num);
    }
    prod
}