use std::fs::File;
use std::io::{self,prelude::*, BufReader};
struct Tree{
    vis: bool,
    height: i32,
    score: i32,
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
                    score: 1,
                };
                row.push(cur_tree);
            }
        }
        trees.push(row);
    }
    let row_last=trees[0].len()-1;
    for i in 0 .. trees.len(){
        let mut cur_higher_right = [row_last as i32; 10];
        let mut cur_higher_left = [0; 10];
        for j in 0 .. trees[0].len(){
            trees[i][j].score *=  j as i32 - cur_higher_left[trees[i][j].height as usize];
            for k in 0 .. (trees[i][j].height+1)
            {
                cur_higher_left[k as usize] = j as i32;
            }
            let j=row_last-j;
            trees[i][j].score *=  cur_higher_right[trees[i][j].height as usize] - j as i32;
            for k in 0 .. (trees[i][j].height+1)
            {
                cur_higher_right[k as usize] = j as i32;
            }
        }
    }
    let col_last=trees.len()-1;
    for j in 0 .. trees[0].len(){
        let mut cur_higher_bottom= [col_last as i32; 10];
        let mut cur_higher_top = [0; 10];
        for i in 0 .. trees.len(){
            trees[i][j].score *=  i as i32 - cur_higher_top[trees[i][j].height as usize];
            for k in 0 .. (trees[i][j].height+1)
            {
                cur_higher_top[k as usize] = i as i32;
            }
            let i=col_last-i;
            trees[i][j].score *=  cur_higher_bottom[trees[i][j].height as usize] - i as i32;
            for k in 0 .. (trees[i][j].height+1)
            {
                cur_higher_bottom[k as usize] = i as i32;
            }
        }
    }
    
    let mut max_score = 0;
    for i in 0 .. trees.len(){
        for j in 0 .. trees[0].len(){
            if trees[i][j].score>max_score{
                println!("{i} {j}");
                max_score=trees[i][j].score;
            }
        }
    }

    println!("max score: {max_score}");
    Ok(())
}