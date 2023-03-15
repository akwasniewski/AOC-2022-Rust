//dfs
use std::fs::File;
use std::collections::LinkedList;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<i32>> =  Vec::new();
    let mut starting_pos: (i32, i32) = (0, 0);
    let mut ending_pos: (i32, i32) = (0, 0);
    let mut queue: LinkedList<(i32, i32, i32)>= LinkedList::new();
    let mut line_num = 0;
    const ROW_COUNT: i32 = 41;
    const COLUMN_COUNT: i32=144;
    for line in reader.lines()
    {
        let mut row: Vec<i32> = Vec::new();
        let mut char_num = 0;
        for c in line?.chars(){
            if c=='S'{
                starting_pos=(char_num, line_num);
                row.push(0);
            }
            else if c=='E'{
                ending_pos=(char_num, line_num);
                row.push(25)
            }
            else{
                row.push(c as i32 - 97);
            }
            char_num+=1;
        }
        map.push(row);
        line_num += 1;
    }
    //bfs
    queue.push_back((starting_pos.0, starting_pos.1, 0));
    let mut res= 0;
    while queue.len()!=0 {
        let cur=queue.pop_front().unwrap();
        if (cur.0, cur.1) == ending_pos{
            res=cur.2;
            break;
        }
        let val = map[cur.1 as usize][cur.0 as usize];
        if val>30 {continue;}
        map[cur.1 as usize][cur.0 as usize] +=100;
        if cur.1 + 1<ROW_COUNT{
            let val_neighbour = map[(cur.1 + 1) as usize][cur.0 as usize];
            if val_neighbour<= val + 1{
                queue.push_back((cur.0, cur.1 + 1, cur.2+1));
            }
        }
        if cur.1 - 1>=0{
            let val_neighbour = map[(cur.1 - 1) as usize][cur.0 as usize];
            if val_neighbour<= val + 1{
                queue.push_back((cur.0, cur.1 - 1, cur.2+1));
        }
        }
        if cur.0 + 1<COLUMN_COUNT{
            let val_neighbour = map[cur.1 as usize][(cur.0+1) as usize];
            if val_neighbour<= val + 1{
                queue.push_back((cur.0+1, cur.1, cur.2+1));
            }
        }  
        if cur.0 - 1>=0{
            let val_neighbour = map[cur.1 as usize][(cur.0 - 1) as usize];
            if val_neighbour<= val + 1{
                queue.push_back((cur.0-1, cur.1, cur.2+1));
            }
        }
    }
    println!("shortest path len is:{}", res);
    Ok(())
}
