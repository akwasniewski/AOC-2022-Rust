use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut dirs: Vec<i32> = Vec::new();  
    let mut closed: Vec<i32> = Vec::new();  
    for line in reader.lines(){
        let line = line?;
        let args: Vec<&str> = line.split(" ").collect();
        if line.chars().next() == Some('$'){ //command
            if args[1] == "cd"{
                if args[2]==".."{
                    let close=dirs.pop();
                    if let Some(close) = close{
                        closed.push(close);
                    }
                }
                else{
                    dirs.push(0);
                }
            }
        }
        else{
            let num=args[0].parse::<i32>();
            let num = match num{
                Ok(num)=>num,
                Err(_)=>{continue;}
            };
            for i in 0 .. dirs.len(){
                dirs[i]+=num;
            }
        }
    }
    while dirs.len()!=0{
        let close = dirs.pop();
        if let Some(close) = close{
            closed.push(close);
        }
    }
    let fs_size_option=closed.last();
    let fs_size:i32 = match fs_size_option{
        Some(num)=>*num,
        None=>{panic!("error getting root file size!");},
    };
    let needed = -(40000000-fs_size);
    if needed<=0{
        println!("no files need to be deleted");
    }
    let mut smallest: i32 = fs_size;
    for dir in closed{
        if dir>=needed && dir<smallest{
            smallest=dir;
        }
    }
    println!("smallest sufficient file: {smallest}");
    Ok(())
}