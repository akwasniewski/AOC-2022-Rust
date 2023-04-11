//solves the problem and generates output
// similar to the one on AOC site containing the problem (used for debugging)
use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut lines: (Vec<char>, Vec<char>) = (Vec::new(), Vec::new());
    let mut read = 0;
    let mut pair_num = 1;
    let mut sum = 0;
    for line in reader.lines()
    {
        let line=line?;
        if read==0{
            lines.0=line.chars().collect();
        }
        if read==1{
            lines.1=line.chars().collect();
        }
        if read==2{
            println!("pair {pair_num}------");
            if compare_sets(lines.clone(), &mut(0, 0), "".to_string())==1{
                sum+=pair_num;
            }
            else {
            }
        }
        read+=1;
        if read == 3{
            read=0;
            pair_num+=1;
        }
    }
    println!("sum: {sum}");
    Ok(())
}
fn compare_sets(mut lines: (Vec<char>, Vec<char>), cur_char: &mut (usize, usize), tabs: String) -> i32{
    while lines.0[cur_char.0] !=']' && lines.1[cur_char.1] !=']'{
        if lines.0[cur_char.0] == ',' {cur_char.0+=1;}
        if lines.1[cur_char.1] == ',' {cur_char.1+=1;}
        if lines.0[cur_char.0] == '[' && lines.1[cur_char.1] == '['{
            cur_char.0+=1;
            cur_char.1+=1;
            let mut new_tabs = tabs.clone();
            new_tabs.push('\t');
            let old_cur= cur_char.0;
            let mut set0 = "[".to_string();
            let mut left=1;
            let mut right=0;
            while left>right {
                if lines.0[cur_char.0] == '[' {left+=1;}
                if lines.0[cur_char.0] == ']' {right+=1;}
                set0.push(lines.0[cur_char.0]);
                cur_char.0+=1;
            }
            cur_char.0=old_cur;
            let old_cur= cur_char.1;
            let mut set1 = "[".to_string();
            let mut left=1;
            let mut right=0;
            while left>right {
                if lines.1[cur_char.1] == '[' {left+=1;}
                if lines.1[cur_char.1] == ']' {right+=1;}
                set1.push(lines.1[cur_char.1]);
                cur_char.1+=1;
            }
            cur_char.1=old_cur;
            println!("{tabs}comparing {} with {}", set0, set1);
            let res = compare_sets(lines.clone(),  cur_char,new_tabs);
            if res != 0 {return res;}
        }
        else if lines.0[cur_char.0] == '['{
            cur_char.0+=1;
            let mut find_char=cur_char.1;
            while lines.1[find_char].is_ascii_digit(){
                find_char+=1;
            }
            lines.1.insert(find_char, ']');
            let mut new_tabs = tabs.clone();
            new_tabs.push('\t');
            let old_cur= cur_char.0;
            let mut set0 = "[".to_string();
            let mut left=1;
            let mut right=0;
            while left>right {
                if lines.0[cur_char.0] == '[' {left+=1;}
                if lines.0[cur_char.0] == ']' {right+=1;}
                set0.push(lines.0[cur_char.0]);
                cur_char.0+=1;
            }
            cur_char.0=old_cur;
            let old_cur= cur_char.1;
            let mut set1 = "[".to_string();
            let mut left=1;
            let mut right=0;
            while left>right {
                if lines.1[cur_char.1] == '[' {left+=1;}
                if lines.1[cur_char.1] == ']' {right+=1;}
                set1.push(lines.1[cur_char.1]);
                cur_char.1+=1;
            }
            cur_char.1=old_cur;
            println!("{tabs}comparing {} with {}", set0, set1);
            let res = compare_sets(lines.clone(),  cur_char,new_tabs);
            if res != 0 {return res;}
        }
        else if lines.1[cur_char.1] == '['{
            cur_char.1+=1;
            let mut find_char=cur_char.0;
            while lines.0[find_char].is_ascii_digit(){
                find_char+=1;
            }
            lines.0.insert(find_char, ']');
            let mut new_tabs = tabs.clone();
            new_tabs.push('\t');
            let old_cur= cur_char.0;
            let mut set0 = "[".to_string();
            let mut left=1;
            let mut right=0;
            while left>right {
                if lines.0[cur_char.0] == '[' {left+=1;}
                if lines.0[cur_char.0] == ']' {right+=1;}
                set0.push(lines.0[cur_char.0]);
                cur_char.0+=1;
            }
            cur_char.0=old_cur;
            let old_cur= cur_char.1;
            let mut set1 = "[".to_string();
            let mut left=1;
            let mut right=0;
            while left>right {
                if lines.1[cur_char.1] == '[' {left+=1;}
                if lines.1[cur_char.1] == ']' {right+=1;}
                set1.push(lines.1[cur_char.1]);
                cur_char.1+=1;
            }
            cur_char.1=old_cur;
            println!("{tabs}comparing {} with {}", set0, set1);
            let res = compare_sets(lines.clone(),  cur_char,new_tabs);
            if res != 0 {return res;}
        }
        //else must be nunmeric
        else{
            let mut nums: (String , String) = ("".to_string(), "".to_string());
            while lines.0[cur_char.0].is_ascii_digit(){
                nums.0.push(lines.0[cur_char.0]);
                cur_char.0+=1;
            }
            while lines.1[cur_char.1].is_ascii_digit(){
                nums.1.push(lines.1[cur_char.1]);
                cur_char.1+=1;
            }
            let nums: (i32, i32) = (nums.0.parse::<i32>().unwrap(), nums.1.parse::<i32>().unwrap());
            println!("{tabs}compare {} with {}", nums.0, nums.1);
            if nums.1 < nums.0 {
                println!("{tabs}right smaller");
                return -1;}
            else if nums.0 < nums.1 {
                println!("{tabs}left smaller");
                return 1;}
        }
    }
    if lines.0[cur_char.0] ==']' && lines.1[cur_char.1] ==']'{
        cur_char.0+=1;
        cur_char.1+=1;
        return 0;
    }
    if lines.0[cur_char.0] ==']' { 
        println!("{tabs}left run out");
        return 1;}
    println!("{tabs}right run out");

    return -1;
}
