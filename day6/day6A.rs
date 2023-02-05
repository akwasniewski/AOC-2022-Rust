use std::fs;
fn main() -> Result<(), std::io::Error> {
    let input = fs::read_to_string("in.txt")?;
    let mut prev = vec!['ą', 'ą', 'ą', 'ą']; //just a char not in the text for reference
    let mut index = 0;
    for c in input.chars(){
        index+=1;
        prev.remove(0);
        prev.push(c);
        if check(&prev){
            break;
        }
    }
    println!("marker after: {index}");
    Ok(())
}
fn check(prev: &Vec<char>) -> bool{
    if prev[0] == 'ą'{
        return false;
    }
    for i in 0..3{
        for j in (i+1)..4{
            if prev[i]==prev[j]{ return false};
        }
    }
    true
}