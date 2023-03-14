use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut items_held: [Vec<i32>; 8] =  Default::default();
    let mut inspections: [i32;8] = [0;8];
    let mut cur_monkey = 0;     
    for line in reader.lines()
    {
        let line = line?;
        let line: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        items_held[ cur_monkey] = line;
        cur_monkey+=1;
    }
    for _t in 0..20{
        for cur_monkey in 0..8{
            let monkey_len=items_held[cur_monkey].len();
            for i in 0..monkey_len{
                //println!("{}, {}, {}",cur_monkey, monkey_len-i-1, items_held[cur_monkey].len());
                let item = items_held[cur_monkey][monkey_len-i-1];
                let item = monkey_calc(cur_monkey as i32, item);
                let item = item/3;
                inspections[cur_monkey]+=1;
                let where_thrown = monkey_where(cur_monkey as i32, item);
                items_held[where_thrown as usize].push(item);
                items_held[cur_monkey].pop();
            }
        }
    }
    let mut max1=0;
    let mut max2=0;
    for i in 0..8{
        if inspections[i]>max1 {
            if max1>max2 {max2=max1;}
            max1=inspections[i];
        }
        else if inspections[i]>max2 {max2=inspections[i];}
    }
    println!("monkey business is: {}", max1*max2);
    Ok(())
}
fn monkey_calc(monkey: i32, mut val: i32) ->i32{
    match monkey{
        0=>val=val*7,
        1=>val=val*13,
        2=>val=val+1,
        3=>val=val*val,
        4=>val=val+7,
        5=>val=val+6,
        6=>val=val+4,
        7=>val=val+8,
        _=>panic!(),
    }
    val
}
fn monkey_where(monkey: i32, val: i32) -> i32{
    let where_thrown: i32;
    match monkey{
        0=>{
            if val%11==0{where_thrown = 2;}
            else{where_thrown = 6;}
        }
        1=>{
            if val%5==0{where_thrown = 7;}
            else{where_thrown = 4;}
        }
        2=>{
            if val%7==0{where_thrown = 1;}
            else{where_thrown = 7;}
        }
        3=>{
            if val%2==0{where_thrown = 0;}
            else{where_thrown = 6;}
        }
        4=>{
            if val%17==0{where_thrown = 3;}
            else{where_thrown = 5;}
        }
        5=>{
            if val%13==0{where_thrown = 3;}
            else{where_thrown = 0;}
        }
        6=>{
            if val%3==0{where_thrown = 1;}
            else{where_thrown = 2;}
        }
        7=>{
            if val%19==0{where_thrown = 4;}
            else{where_thrown = 5;}
        }
        _=>panic!(),
    }
    where_thrown
}