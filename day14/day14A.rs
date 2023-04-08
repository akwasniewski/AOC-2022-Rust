use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut map:[[bool;200];600] = [[false;200];600];
    for line in reader.lines(){
        let line=line?;
        let line: Vec<&str> = line.split("->").collect();
        let mut first: bool=true;
        let mut last_cmd: Vec<usize>=Vec::new();
        //get rock formation
        for cmd in line{
            let cmd: Vec<usize> = cmd.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect();
            if first==true{
                last_cmd=cmd;
                first=false;
                continue;
            }
            if last_cmd[0] == cmd[0]{
                let sm: usize;
                let bg: usize;
                if last_cmd[1]<cmd[1]{
                    sm=last_cmd[1];
                    bg=cmd[1];
                }
                else{
                    bg=last_cmd[1];
                    sm=cmd[1];
                }
                for i in sm..bg{
                    map[cmd[0]][i]=true;
                }
                map[cmd[0]][bg]=true;
            }
            else{
                let sm: usize;
                let bg: usize;
                if last_cmd[0]<cmd[0]{
                    sm=last_cmd[0];
                    bg=cmd[0];
                }
                else{
                    bg=last_cmd[0];
                    sm=cmd[0];
                }
                for i in sm..bg{
                    map[i][cmd[1]]=true;
                }
                map[bg][cmd[1]]=true;
            }
            last_cmd=cmd;
        }
    }
    let mut sand_placed: i32=0;
    while place_sand(&(500,0), &mut map)==true{
        sand_placed+=1;
    }
    println!("sand places: {}", sand_placed);
    Ok(())
}
//recursively place sand (kinda like DFS)
fn place_sand(pos: &(usize,usize), map: &mut [[bool;200];600]) -> bool{
    if map[pos.0][pos.1+1]==false{
        return place_sand(&(pos.0, pos.1+1), map);
    }
    if map[pos.0-1][pos.1+1]==false{
        return place_sand(&(pos.0-1, pos.1+1), map);
    }
    if map[pos.0+1][pos.1+1]==false{
        return place_sand(&(pos.0+1, pos.1+1), map);
    }
    map[pos.0][pos.1]=true;
    true
}