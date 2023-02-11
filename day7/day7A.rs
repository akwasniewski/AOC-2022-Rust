use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut dirs: Vec<u32> = Vec::new();        
    let mut sum=0;
    for line in reader.lines(){
        let line = line?;
        let args: Vec<&str> = line.split(" ").collect();
        if line.chars().next() == Some('$'){ //command
            if args[1] == "cd"{
                if args[2]==".."{
                    let last=dirs.pop();
                    if let Some(last) = last{
                        if last<=100000{
                            sum+=last;
                        }
                    }
                }
                else{
                    dirs.push(0);
                }
            }
        }
        else{
            let num=args[0].parse::<u32>();
            let num = match num{
                Ok(num)=>num,
                Err(_)=>{continue;}
            };
            for i in 0 .. dirs.len(){
                dirs[i]+=num;
            }
        }
    }
    println!("sum is: {sum}");
    Ok(())
}