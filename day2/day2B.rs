use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut score: i32 = 0;
    for line in reader.lines()
    {
        let line=line?;
        let args: Vec<&str>= line.split(" ").collect();
        score+=get_score(args);
	}
    println!("score is {score}");
    Ok(())
}
fn get_score(args: Vec<&str>) ->i32{
    let mut score = 0;
    let codes = (get_code(args[0]), get_code(args[1]));
    score+=codes.1*3;
    if codes.1==0{
        //lost
        let mut val=codes.0;
        if val==0{
            val=3;
        } 
        score+=val;
    }
    else if codes.1==1{
        //drawn
        score+=codes.0+1;
    }
    else{
        //won
        let mut val=codes.0+2;
        if val==4{
            val=1;
        } 
        score+=val;
    }
    score
}
fn get_code(letter: &str) -> i32{
    if letter=="A" || letter=="X"{
        0//rock
    }
    else if letter=="B" || letter=="Y"{
        1//paper
    }
    else{
        2//scissors
    }
}