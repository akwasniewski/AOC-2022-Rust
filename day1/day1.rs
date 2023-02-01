use std::fs::File;
use std::io::{self,prelude::*, BufReader};
struct Elf{
    calories: u32,
    which_elf: u32,
}
impl Elf {
    fn empty() ->Self {
        Self {
            calories: 0,
            which_elf: 0,
        }
    }
}
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut cur_elf=Elf{
    calories: 0,
    which_elf: 1,
    };
    let mut best_elfs: [Elf;3] = [Elf::empty(), Elf::empty(), Elf::empty()];
    for line in reader.lines()
    {
        let line = line?;
        if line.trim().is_empty(){
            end_elf(&mut cur_elf, &mut best_elfs);
            continue;
        }
        let new_calories: u32 = line.parse().unwrap();
        cur_elf.calories+=new_calories;
	}
    end_elf(&mut cur_elf, &mut best_elfs);
    println!("best elves are:");
    let mut sum: u32=0;	  
    for elf in best_elfs{
        sum+=elf.calories;
        if elf.which_elf!=0 {
        println!("elf {} with {} calories",elf.which_elf, elf.calories);
        }
    }      
    println!("in total top three elves have {} calories!",sum);
    Ok(())
}
fn end_elf(cur_elf: &mut Elf, best_elfs: &mut [Elf])
{
    //frankly not ideal but will come back to it when I have better tools
    let mut compare:Elf = Elf{calories: cur_elf.calories, which_elf: cur_elf.which_elf};
    for elf in best_elfs{
        if elf.calories<compare.calories{
            let temp: Elf = Elf{calories: compare.calories, which_elf: compare.which_elf};
            compare = Elf{calories: elf.calories, which_elf: elf.which_elf};
            elf.calories = temp.calories;
            elf.which_elf = temp.which_elf;
        }
    }
    cur_elf.calories = 0;
    cur_elf.which_elf+=1;
}
