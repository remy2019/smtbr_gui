use crate::types::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn deserialize(s: &str) -> Result<Vec<Problem>, Box<dyn std::error::Error>> {
    Ok(serde_yaml::from_str::<Vec<Problem>>(s)?)
}

fn deserialized_black(s: &str) -> Result<Vec<Vec<(u32, bool)>>, Box<dyn std::error::Error>> {
    let raw = serde_yaml::from_str::<Vec<(u32, u32, bool)>>(&s)?;
    let mut result: Vec<Vec<(u32, bool)>> = (1..=12).map(|_| vec![]).collect();
    for (chap, num, tf) in raw {
        result[chap as usize - 1].push((num, tf));
    }
    Ok(result)
}

pub fn get_blacklist() -> Result<Vec<Vec<(u32, bool)>>, Box<dyn std::error::Error>> {
    let exe = std::env::current_exe()?;
    let dir = exe.parent().expect("Executable must be in some directory");
    let yml = dir.join("blacklist.yml");
    let data_string = &std::fs::read_to_string(yml)?;
    deserialized_black(data_string)
}
