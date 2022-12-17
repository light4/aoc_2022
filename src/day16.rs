use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Debug,
};

#[allow(dead_code)]
static INPUT: &str = r#"
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

pub fn run() {
    // let input = include_str!("../input/day16/input");
    // let mm = init_map(input);
    // dbg!(mm);
    // dbg!(first(input, 30));
    // dbg!(second(input, 4000000));

    let mm = init_map(INPUT, 30);
    let items = mm.open_next_valve_path();
    let item = items.iter().find(|i| i.name == "DD").unwrap();
    let cloned = mm.goto_path(item.clone());
    // dbg!(&cloned);

    let new_items = cloned.open_next_valve_path();
    let new_item = new_items.iter().find(|i| i.name == "BB").unwrap();
    let new_cloned = cloned.goto_path(new_item.clone());
    // dbg!(&new_cloned);

    let new_new_items = new_cloned.open_next_valve_path();
    let new_new_item = new_new_items.iter().find(|i| i.name == "JJ").unwrap();
    let new_new_cloned = new_cloned.goto_path(new_new_item.clone());
    dbg!(&new_new_items);
    dbg!(&new_new_cloned);
    // dbg!(&mm);
    // println!("{}", mm);
    // dbg!(first(INPUT, 30));
    // dbg!(&mm.get_sorted_valves());
    // dbg!(&mm);
    // dbg!(second(INPUT, 20));
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Valve {
    name: String,
    rate: usize,
    lead_to: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct WeightedValve {
    name: String,
    weight: usize,
    round: usize,
    path: Vec<String>,
}

impl Ord for WeightedValve {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.weight).cmp(&other.weight)
    }
}

impl PartialOrd for WeightedValve {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default, Clone)]
pub struct MineMap {
    start: Option<Valve>,
    total_round: usize,
    round: usize,
    valves: HashMap<String, Valve>,
    opened: HashMap<String, (usize, usize)>,
    path: Vec<String>,
}

impl MineMap {
    #[inline]
    pub fn no_rate_valves(&self) -> Vec<String> {
        self.valves
            .iter()
            .filter_map(|(k, v)| {
                if v.rate == 0 {
                    Some(k.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn open_next_valve_path(&self) -> BinaryHeap<WeightedValve> {
        // 没有可以再开的阀门了
        if self.opened.len() + self.no_rate_valves().len() == self.valves.len() {
            return BinaryHeap::new();
        }

        let left_to_open: HashSet<String> = self
            .valves
            .iter()
            .filter_map(|(k, v)| {
                if v.rate != 0 && !self.opened.contains_key(k) {
                    Some(k.to_string())
                } else {
                    None
                }
            })
            .collect();
        let mut result = BinaryHeap::default();
        let start = self.start.clone().unwrap();
        let mut calculated = HashSet::new();
        let mut came_from = HashMap::new();
        let mut mytest_valves = HashSet::new();
        mytest_valves.insert(start.clone());
        let mut used_round = 0;
        while result.len() != left_to_open.len() {
            let mut next_to_calc = HashSet::new();
            for curr in mytest_valves {
                used_round += 1;
                if calculated.contains(&curr) {
                    continue;
                }
                calculated.insert(curr.clone());
                next_to_calc = self.get_lead_to_valves(&curr, false);
                'inner: for v in &next_to_calc {
                    if !came_from.contains_key(&v.name) {
                        came_from.insert(v.name.to_string(), curr.name.to_string());
                    }
                    if v.rate == 0 || self.opened.contains_key(&v.name) {
                        continue 'inner;
                    }
                    let item = WeightedValve {
                        name: v.name.to_string(),
                        weight: self.new_get_weight(used_round + 1, v),
                        round: used_round + 1,
                        path: vec![],
                    };
                    result.push(item);
                }
            }
            mytest_valves = next_to_calc;
        }
        result
    }

    fn __get_path(
        &self,
        curr: &str,
        start: &str,
        came_from: &HashMap<String, String>,
    ) -> Vec<String> {
        let mut result = vec![];
        let mut name = curr.to_string();
        loop {
            if let Some(p) = came_from.get(&name) {
                result.push(p.to_string());
                if p == &start {
                    break;
                }
                name = p.to_string();
            } else {
                break;
            }
        }
        result
    }

    pub fn get_lead_to_valves(&self, v: &Valve, filter: bool) -> HashSet<Valve> {
        let mut result = HashSet::new();
        for n in &v.lead_to {
            let new_valve = self.valves.get(n).unwrap().to_owned();
            if filter {
                if new_valve.rate != 0 && !self.opened.contains_key(&new_valve.name) {
                    result.insert(new_valve);
                }
            } else {
                result.insert(new_valve);
            }
        }
        result
    }

    pub fn new_get_weight(&self, round: usize, v: &Valve) -> usize {
        if round >= self.total_round {
            return 0;
        }
        (self.total_round - round) * v.rate
    }

    pub fn get_weight(
        &self,
        round: usize,
        valve: &Valve,
        depth: usize,
        computed: &mut HashSet<String>,
    ) -> usize {
        if round >= self.total_round || computed.contains(&valve.name) {
            return 0;
        }

        let mut need_round = 1;
        let rate = if self.opened.contains_key(&valve.name) {
            0
        } else {
            valve.rate
        };
        if rate != 0 {
            need_round += 1;
        }
        computed.insert(valve.name.clone());
        let lead_to_weight: usize = valve
            .lead_to
            .iter()
            .map(|i| {
                self.get_weight(
                    round + need_round,
                    self.valves.get(i).unwrap(),
                    depth + 1,
                    computed,
                )
            })
            .sum();
        if self.total_round <= round + need_round {
            lead_to_weight
        } else {
            rate * (self.total_round - round - need_round) + lead_to_weight
        }
    }

    pub fn goto_path(&self, item: WeightedValve) -> Self {
        let mut result = self.clone();
        let real_next = result.valves.get(&item.name).unwrap();
        result
            .opened
            .insert(item.name.clone(), (real_next.rate, self.round + item.round));
        result.path.push(item.name.to_string());
        result.start = Some(real_next.clone());
        result.round += item.round;
        result
    }

    fn calc_result(&self) -> usize {
        let mut result = 0;
        for (_, (rate, start_at)) in self.opened.iter() {
            if self.total_round > *start_at {
                result += (self.total_round - start_at) * rate;
            }
        }
        result
    }
}

#[allow(dead_code)]
fn init_map(input: &str, mins: usize) -> MineMap {
    let mut mm = MineMap::default();
    mm.total_round = 30;
    for s in input.lines().map(|i| i.trim()).filter(|i| !i.is_empty()) {
        let splited: Vec<_> = s.split(';').collect();
        let name = splited[0].split(' ').nth(1).unwrap();
        let rate = splited[0].split('=').last().unwrap().parse().unwrap();
        let pattern = if splited[1].contains("valves") {
            "valves "
        } else {
            "valve "
        };
        let lead_to = splited[1]
            .split(pattern)
            .last()
            .unwrap()
            .split(',')
            .map(|i| i.trim())
            .filter(|i| !i.is_empty())
            .map(|i| i.to_string())
            .collect();
        let valve = Valve {
            name: name.to_string(),
            rate,
            lead_to,
        };
        if mm.start.is_none() {
            mm.start = Some(valve.clone());
        }
        mm.valves.insert(name.to_string(), valve);
    }
    mm
}

fn first(input: &str, mins: usize) -> usize {
    let mm = init_map(input, mins);
    let need_opened_len = mm.valves.iter().filter(|(k, v)| v.rate > 0).count();
    dbg!(need_opened_len);

    let mut possible_result = vec![];
    let mut all_maps = vec![mm.clone()];
    loop {
        if &all_maps.len() <= &0 {
            break;
        }
        let mut new_maps = vec![];
        'inner: for map in &all_maps {
            let mut items = map.open_next_valve_path();
            if items.is_empty() {
                break 'inner;
            }
            for _ in 0..=need_opened_len {
                if let Some(item) = items.pop() {
                    let cloned = map.goto_path(item);
                    if cloned.round >= cloned.total_round || cloned.opened.len() >= need_opened_len
                    {
                        let r = cloned.calc_result();
                        if r > 1000 {
                            possible_result.push(cloned.calc_result());
                        }
                    } else {
                        new_maps.push(cloned);
                    }
                }
            }
        }
        all_maps = new_maps;
    }
    possible_result.sort();
    *possible_result.last().unwrap()
}

fn second(input: &str, dist: usize) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(first(INPUT, 30), 1651);
    }
}
