mod queue;

use std::collections::{HashMap, HashSet};

use crate::regex_engine::automata::{dfa::queue::MarkedQueue, nfa::NFA};

pub struct DFA {
    /// Start state
    pub q0: usize,
    /// Accept states
    pub qf: HashSet<usize>,
    /// Transition function: (state, symbol) -> state
    pub d: HashMap<(usize, char), usize>,
}

impl DFA {
    pub fn new(q0: usize, qf: HashSet<usize>) -> Self {
        DFA {
            q0,
            qf,
            d: HashMap::new(),
        }
    }

    pub fn simulate(&self, input: Vec<char>) -> bool {
        let mut current_state = self.q0;

        for c in input {
            if let Some(&next_state) = self.d.get(&(current_state, c)) {
                current_state = next_state;
            } else {
                return false; // No valid transition
            }
        }

        self.qf.contains(&current_state) // Check if the final state is an accept state
    }
}

fn to_str(s: &HashSet<usize>) -> String {
    let mut v: Vec<usize> = s.iter().cloned().collect();
    v.sort_unstable();
    let repr: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    repr.join(" ")
}

fn to_set(s: &String) -> HashSet<usize> {
    let nums: HashSet<usize> = s.split(' ').map(|x| x.parse::<usize>().unwrap()).collect();
    nums
}

impl From<NFA> for DFA {
    fn from(nfa: NFA) -> Self {
        let q0: usize = 0;
        let mut qf: HashSet<usize> = HashSet::new();
        let mut d: HashMap<(usize, char), usize> = HashMap::new();

        let mut queue = MarkedQueue::new();

        let e0 = nfa.e_closure(&HashSet::from([nfa.q0]));
        queue.add_unmarked(to_str(&e0));
        while let Some(t) = queue.pop_unmarked() {
            let t_set = to_set(&t);
            for c in 0..=255u8 {
                let a = char::from(c);
                let u_set = nfa.e_closure(&nfa.move_to(&t_set, &a.into()));
                if u_set.is_empty() {
                    continue;
                }
                let u = to_str(&u_set);
                if !queue.contains(&u) {
                    queue.add_unmarked(u.clone());
                    if u_set.contains(&nfa.qf) {
                        qf.insert(queue[&u]);
                    }
                }
                d.insert((queue[&t], a), queue[&u]);
            }
        }

        DFA { q0, qf, d }
    }
}
