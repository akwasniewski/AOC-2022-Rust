use std::fs::File;
use std::io::{self,prelude::*, BufReader};
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    lines.push("[[2]]".to_string());
    lines.push("[[6]]".to_string());
    for line in reader.lines()
    {
        let line=line?;
        if line.is_empty() {continue;}
        lines.push(line);
    }
    bubble_sort(&mut lines); //quickest to implement
    let mut divider_pos = (1,1);
    for e in 0..lines.len(){
        if lines[e] == "[[2]]"{
            divider_pos.0 += e;
        }
        if lines[e] == "[[6]]"{
            divider_pos.1 += e;
        }
    }
    println!("divider pos: {} {}", divider_pos.0, divider_pos.1);
    println!("decorder key: {}", divider_pos.0 * divider_pos.1);
    Ok(())
}
fn bubble_sort(arr: &mut Vec<String>){
    let mut swaps=1;
    let len=arr.len();
    while swaps!=0 {
        swaps=0;
        let mut cur_element=0;
        while cur_element<len-1{
            if !compare_lines(&arr[cur_element], &arr[cur_element+1]){
                arr.swap(cur_element, cur_element+1);
                swaps+=1;
            }
            cur_element+=1;
        }
    }
}
fn compare_lines(line0: &str, line1: &str)->bool{
    let lines: (Vec<char>, Vec<char>) = (line0.chars().collect(), line1.chars().collect());
    if compare_sets(lines, &mut(0,0)) ==1 {return true;}
    return false;
}
fn compare_sets(mut lines: (Vec<char>, Vec<char>), cur_char: &mut (usize, usize)) -> i32{
    while cur_char.0<lines.0.len() && cur_char.1<lines.1.len() && lines.0[cur_char.0] !=']' && lines.1[cur_char.1] !=']'{
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
        if cur_char.0>=lines.0.len() {return -1;}
        if cur_char.1>=lines.1.len() {return -1;}
        return 0;
    }
    if lines.0[cur_char.0] ==']' { return 1;}

    return -1;
}