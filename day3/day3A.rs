use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    'lineReading: for line in reader.lines()
    {
        let line=line?;
        let len = line.len();
        let (first, second) = line.split_at(len/2);
        for c in first.chars(){
            for h in second.chars(){
                if c==h{
                    sum+=priority(c);
                    continue 'lineReading;
                }

            }
        }
	}
    println!("sum of the prirotities is: {sum}");
    Ok(())
}
fn priority(ch: char) ->u32{
    let mut ch = ch as u32;
    if ch<96{//uppercase
        ch-= 38;
    }
    else{ //lowercase
        ch-=96;
    }
    ch
}
