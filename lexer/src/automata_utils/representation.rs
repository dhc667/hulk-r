use std::collections::HashSet;

pub fn to_str(s: &HashSet<usize>) -> String {
    let mut v: Vec<usize> = s.iter().cloned().collect();
    v.sort_unstable();
    let repr: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    repr.join(" ")
}

pub fn to_set(s: &String) -> HashSet<usize> {
    let nums: HashSet<usize> = s.split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
    nums
}
