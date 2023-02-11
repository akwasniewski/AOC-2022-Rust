use std::fs::File;
use std::io::{self,prelude::*, BufReader};
struct Tree{
    vis: bool,
    height: i32,
}
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut trees: Vec<Vec<Tree>> = Vec::new();        
    for line in reader.lines(){
        let line = line?;
        let line: Vec<char>= line.chars().collect();
        let mut row: Vec<Tree> = Vec::new();
        for c in line{
            let c = c.to_digit(10);
            if let Some(num) = c{
                let cur_tree = Tree{
                    vis: false,
                    height: num as i32,
                };
                row.push(cur_tree);
            }
        }
        trees.push(row);
    }
    let mut sum=0; //summing visible trees 
    let row_last=trees[0].len()-1;
    for i in 0 .. trees.len(){
        let mut cur_highest_left: i32=-1;
        let mut cur_highest_right: i32=-1;
        for j in 0 .. trees[0].len(){
            if trees[i][j].height> cur_highest_left{
                if trees[i][j].vis==false{
                    sum+=1;
                }
                cur_highest_left=trees[i][j].height;
                trees[i][j].vis=true;
            }
            let j = row_last-j;
            if trees[i][j].height> cur_highest_right{
                if trees[i][j].vis==false{
                    sum+=1;
                }
                cur_highest_right=trees[i][j].height;
                trees[i][j].vis=true;
            }
        }
    }
    let column_last=trees.len()-1;
    for j in 0 .. trees[0].len(){
        let mut cur_highest_top: i32=-1;
        let mut cur_highest_bottom: i32=-1;
        for i in 0 .. trees.len(){
            if trees[i][j].height> cur_highest_top{
                if trees[i][j].vis==false{
                    sum+=1;
                }
                cur_highest_top=trees[i][j].height;
                trees[i][j].vis=true;
            }
            let i = column_last-i;
            if trees[i][j].height> cur_highest_bottom{
                if trees[i][j].vis==false{
                    sum+=1;
                }
                cur_highest_bottom=trees[i][j].height;
                trees[i][j].vis=true;
            }
        }
    }
    println!("sum: {sum}");
    Ok(())
}