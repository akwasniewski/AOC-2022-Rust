use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut cur=0;
    let mut lines: [String; 3] = [String::new(), String::new(), String::new()];
    for line in reader.lines()
    {
        let line=line?;
        lines[cur] = line;
        cur+=1;
        if cur==3{
            cur=0;
            let badge = get_badge(&lines);
            sum+=priority(badge);
        }
	}
    println!("sum of the prirotities is: {sum}");
    Ok(())
}
fn get_badge(lines: &[String]) -> char {
    let mut ch: Vec<char> = Vec::new();
    for c in lines[0].chars(){
        ch.push(c);
    }
    for tb in [1,2]{
        let mut i=0;
        'gochar: while i<ch.len(){
            for c in lines[tb].chars(){
                if c==ch[i]{
                    i+=1;
                    continue 'gochar;
                }
            }
            ch.remove(i);
        }
    }
    ch[0]
}
fn priority(badge: char) ->u32{
    let mut badge = badge as u32;
    if badge<96{//uppercase
        badge-= 38;
    }
    else{ //lowercase
        badge-=96;
    }
    badge
}