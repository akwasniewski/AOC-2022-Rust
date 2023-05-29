//doesnt work in instant as day16A, but still reasonably within a few mins on my laptop
use std::fs::File;
use std::io::{self,prelude::*, BufReader};
use std::collections::VecDeque;
use std::collections::HashMap;
struct Node{
    connections: Vec<i32>,
    flow: i32,
    open: bool,
    shortest_paths: HashMap<i32, i32>,
    last_visited: i32,
}
fn main() -> io::Result<()>{
    let file = File::open("in.txt")?;
    let reader = BufReader::new(file);
    let mut system=HashMap::new();
    for line in reader.lines(){
        let line=line?;
        let line: Vec<&str> = line.split(" ").collect();
        let valve:&str=line[1];
        let valve=hash(valve);
        let flow:&str=line[4];
        let flow=flow.replace(";","");
        let flow:Vec<&str>=flow.split("=").collect();
        let flow=flow[1].parse::<i32>().unwrap();
        system.insert(valve, Node{connections: Vec::new(), flow: flow, open:false, last_visited:i32::MAX, shortest_paths: HashMap::new()});
        for i in 9..line.len(){
            let connection=line[i].replace(",","");
            system.get_mut(&valve).unwrap().connections.push(hash(&connection));
        }
    }
    get_shortest_paths(&mut system);
    let ans=find(&mut system, 0, 0, 0, 0, 0,false);
    println!("answer:{}",ans);
    Ok(())
}
fn bfs(system: &mut HashMap<i32, Node>,from: i32){
    let mut stq: VecDeque<(i32, i32)>=VecDeque::new();
    stq.push_back((from,0));
    while stq.is_empty()==false {
        let cur: (i32, i32) = stq.pop_front().unwrap();
        if system[&cur.0].last_visited==from{
            continue;
        }
        system.get_mut(&cur.0).unwrap().last_visited=from;
        system.get_mut(&cur.0).unwrap().shortest_paths.insert(from, cur.1);
        for i in 0..system[&cur.0].connections.len(){
            if system[&system[&cur.0].connections[i]].last_visited==from{
                continue;
            }
            stq.push_back((system[&cur.0].connections[i], cur.1+1));
        }
    }
}
fn get_shortest_paths(system: &mut HashMap<i32, Node>){
    let mut candidates: Vec<i32>=Vec::new();
    for i in &mut *system{
            candidates.push(*i.0);
    }
    for cur in candidates{

        bfs(system, cur as i32);
    }
}
fn find(system: &mut HashMap<i32, Node>, cur_player: i32, cur_elephant: i32, time: i32,flown: i32, other_start: i32, is_elephant: bool) -> i32{
    if time>26{
        return flown;
    }
    let cur: i32;
    if is_elephant==true{
        cur=cur_elephant;
    }
    else{
        cur=cur_player;
    }
    let mut max_flown=flown;
    let mut candidates: Vec<i32>=Vec::new();
    for i in &mut *system{
        if i.1.open == false && i.1.flow>0 && time+i.1.shortest_paths[&cur]+1<26{
            candidates.push(*i.0);
        }
    }
    if candidates.len()==0 && other_start<26{
        return find(system, cur_player, cur_elephant, other_start, flown, i32::MAX, !is_elephant);
    }
    for i in candidates{
        system.get_mut(&i).unwrap().open=true;
        let pos_elephant:i32;
        let pos_player:i32;
        if is_elephant==true{
            pos_elephant=i;
            pos_player=cur_player;
        }
        else{
            pos_elephant=cur_elephant;
            pos_player=i;
        }
        let cur_flown: i32;
        if other_start<=time+system[&cur].shortest_paths[&i]+1{
            cur_flown= find(system, pos_player, pos_elephant, other_start, 
                flown+(26-(time+system[&cur].shortest_paths[&i]+1))*system[&i].flow, time+system[&cur].shortest_paths[&i]+1, !is_elephant);
        }
        else{
            cur_flown= find(system, pos_player, pos_elephant, time+system[&cur].shortest_paths[&i]+1, 
                flown+(26-(time+system[&cur].shortest_paths[&i]+1))*system[&i].flow,other_start, is_elephant);
        }
        if cur_flown>max_flown{
            max_flown=cur_flown;
        }
        system.get_mut(&i).unwrap().open=false;
    }
    max_flown
}
fn hash(valve:&str)->i32{
    let valve: Vec<char>=valve.chars().collect();
    let valve: i32=(valve[0] as i32-65)*26+(valve[1] as i32-65);
    valve
}