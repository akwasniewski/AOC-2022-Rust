use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for _ in 0 ..1000{
        let mut row: Vec<bool> = Vec::new();
        row.resize(1000,false);
        visited.push(row);
    }
    let mut pos: [(i32, i32);10] = [(500, 500);10];
    for line in reader.lines(){
        let line=line?;
        let line: Vec<&str> = line.split(" ").collect();
        let num = line[1].parse::<i32>().unwrap();
        for _i in 0..num{
            println!("next");
            move_head(&mut pos[0], line[0]);
            for j in 0..9{
                move_tail(pos[j], &mut pos[j+1]);
            }
            println!("{:?}", pos[9]);

            visited[pos[9].0 as usize][pos[9].1 as usize] = true;
        }
    }
    let mut pos_visited: i32=0;
    for i in 0 ..1000{
        for j in 0..1000{
            if visited[i][j] == true{
                pos_visited+=1;
            }
        }
    }
    println!("tail visited {pos_visited} positions");
    Ok(())
}
fn move_head(mut pos: &mut(i32, i32), dir: &str)
{
    if dir=="U"{
        pos.1+=1;
    }
    else if dir=="D"{
        pos.1-=1;
    }
    else if dir=="R"{
        pos.0+=1;
    }
    else if dir=="L"{
        pos.0-=1;
    }
}
fn move_tail(pos_head: (i32, i32), mut pos_tail:  &mut(i32, i32))
{
    let dist_x = pos_head.0 - pos_tail.0;
    let dist_y = pos_head.1 - pos_tail.1;
    if dist_x.abs() >1 && dist_y.abs() > 1{
        let mut change = -1;
        if dist_x < 0{ change=1};
        pos_tail.0 += dist_x + change;
        change = -1;
        if dist_y < 0{ change=1};
        pos_tail.1 += dist_y + change;
        return;
    }
    if dist_x.abs() > 1{
        let mut change = -1;
        if dist_x < 0{ change=1};
        pos_tail.0 += dist_x + change;
        if dist_y !=0 {
            pos_tail.1 += dist_y;
        }
    }
    if dist_y.abs() > 1{
        let mut change = -1;
        if dist_y < 0{ change=1};
        pos_tail.1 += dist_y + change;
        if dist_x !=0 {
            pos_tail.0 += dist_x;
        }
    }
}