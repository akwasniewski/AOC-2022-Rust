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
            if compare_sets(lines.clone(), &mut(0, 0))==1{
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
fn compare_sets(mut lines: (Vec<char>, Vec<char>), cur_char: &mut (usize, usize)) -> i32{
    while cur_char.0<lines.0.len() && cur_char.0<lines.0.len() && lines.0[cur_char.0] !=']' && lines.1[cur_char.1] !=']'{
        if lines.0[cur_char.0] == ',' {cur_char.0+=1;}
        if lines.1[cur_char.1] == ',' {cur_char.1+=1;}
        if lines.0[cur_char.0] == '[' && lines.1[cur_char.1] == '['{
            cur_char.0+=1;
            cur_char.1+=1;
            let res = compare_sets(lines.clone(),  cur_char);
            if res != 0 {return res;}
        }
        else if lines.0[cur_char.0] == '['{
            cur_char.0+=1;
            let mut find_char=cur_char.1;
            while lines.1[find_char].is_ascii_digit(){
                find_char+=1;
            }
            lines.1.insert(find_char, ']');
            let res = compare_sets(lines.clone(),  cur_char);
            if res != 0 {return res;}
        }
        else if lines.1[cur_char.1] == '['{
            cur_char.1+=1;
            let mut find_char=cur_char.0;
            while lines.0[find_char].is_ascii_digit(){
                find_char+=1;
            }
            lines.0.insert(find_char, ']');
            let res = compare_sets(lines.clone(),  cur_char);
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
            if nums.1 < nums.0 {
                return -1;}
            else if nums.0 < nums.1 {
                return 1;}
        }
    }
    if cur_char.0>=lines.0.len() {return 1;}
    if cur_char.1>=lines.1.len() {return -1;}
    if lines.0[cur_char.0] ==']' && lines.1[cur_char.1] ==']'{
        cur_char.0+=1;
        cur_char.1+=1;
        return 0;
    }
    if lines.0[cur_char.0] ==']' { return 1;}

    return -1;
}
