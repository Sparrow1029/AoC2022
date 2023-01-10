use nom::{bytes::complete::tag, character::complete as cc, sequence::tuple, Finish, IResult};
use std::collections::HashMap;

const COST_MASK: u16 = 0b0000_0000_1111_1111;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
/// Robot types
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Robot {
    fn iter<'a>() -> std::slice::Iter<'a, Robot> {
        [Robot::Geode, Robot::Obsidian, Robot::Clay, Robot::Ore].iter()
    }
}

struct Factory {
    clay: u16,
    ore: u16,
    obsidian: u16,
    geodes: u16,
    robot_costs: HashMap<Robot, u16>,
    robots: HashMap<Robot, u16>,
}

impl Factory {
    fn new(robot_costs: HashMap<Robot, u16>) -> Self {
        Self {
            clay: 0,
            ore: 0,
            obsidian: 0,
            geodes: 0,
            robot_costs,
            robots: HashMap::from_iter(vec![
                (Robot::Ore, 1),
                (Robot::Clay, 0),
                (Robot::Obsidian, 0),
                (Robot::Geode, 0),
            ]),
        }
    }

    fn get_costs(num: u16) -> (u16, u16) {
        let ore_cost = num & COST_MASK;
        let other_cost = (num >> 8) & COST_MASK;
        (ore_cost, other_cost)
    }

    fn build_robot(&mut self) -> Option<Robot> {
        for robot_type in Robot::iter() {
            let (ore, other) = Factory::get_costs(*(self.robot_costs.get(&robot_type).unwrap()));
            match robot_type {
                Robot::Geode => {
                    println!(
                        "{robot_type:?} -> Need ({ore}, {other}). Have ({}, {})",
                        self.ore, self.obsidian
                    );
                    if self.ore > 0
                        && self.obsidian > 0
                        && self.ore >= ore
                        && self.obsidian >= other
                    {
                        self.ore -= ore;
                        self.obsidian -= other;
                        return Some(Robot::Geode);
                    }
                }
                Robot::Obsidian => {
                    println!(
                        "{robot_type:?} -> Need ({ore}, {other}). Have ({}, {})",
                        self.ore, self.clay
                    );
                    if self.robots[&Robot::Obsidian] > 2 {
                        if !(self.robots[&Robot::Geode] > 0) {
                            continue;
                        }
                    }
                    if self.ore > 0 && self.clay > 0 && self.ore >= ore && self.clay >= other {
                        self.ore -= ore;
                        self.clay -= other;
                        return Some(Robot::Obsidian);
                    }
                }
                Robot::Clay => {
                    println!(
                        "{robot_type:?} -> Need ({ore}, {other}). Have ({}, 0)",
                        self.ore
                    );
                    if self.robots[&Robot::Clay] > 2 {
                        if !(self.robots[&Robot::Geode] > 2) && !(self.robots[&Robot::Obsidian] > 1)
                        {
                            continue;
                        }
                    }
                    if self.ore >= ore && ore > 0 {
                        self.ore -= ore;
                        return Some(Robot::Clay);
                    }
                }
                Robot::Ore => {
                    println!(
                        "{robot_type:?} -> Need ({ore}, {other}). Have ({}, 0)",
                        self.ore
                    );
                    if (self.robots[&Robot::Clay] > 3)
                        && (self.robots[&Robot::Obsidian] > 2)
                        && (self.robots[&Robot::Geode] > 1)
                    {
                        if self.ore >= ore && ore > 0 {
                            self.ore -= ore;
                            return Some(Robot::Ore);
                        }
                    }
                }
            }
        }
        None
    }
    fn cycle(&mut self, minutes: usize) {
        for i in 0..minutes {
            println!("\nMinute {i}");
            let build_robot = self.build_robot();
            for (robot, count) in self.robots.iter() {
                if *count > 0 {
                    match robot {
                        Robot::Ore => self.ore += count,
                        Robot::Clay => self.clay += count,
                        Robot::Obsidian => self.obsidian += count,
                        Robot::Geode => self.geodes += count,
                    }
                }
            }
            if let Some(robot) = build_robot {
                self.robots
                    .entry(robot)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }
}

impl std::fmt::Debug for Factory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Factory:\n  clay:     {}\n  ore:      {}\n  obsidian: {}\n  geodes:   {}\n  costs:\n",
            self.clay, self.ore, self.obsidian, self.geodes
        )?;
        for (k, v) in self.robot_costs.iter() {
            writeln!(f, "    {k:?} -> {:?}", Factory::get_costs(*v))?;
        }
        writeln!(f, "  robots: {:?}", self.robots)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Factory>> {
    let mut factories = vec![];
    for line in input.lines() {
        let mut robot_costs = HashMap::new();
        let (_, (_, _, _, ore, _, clay, _, obsidian1, _, obsidian2, _, geode1, _, geode2, _)) =
            tuple((
                tag("Blueprint "),
                cc::u16,
                tag(": Each ore robot costs "),
                cc::u16,
                tag(" ore. Each clay robot costs "),
                cc::u16,
                tag(" ore. Each obsidian robot costs "),
                cc::u16,
                tag(" ore and "),
                cc::u16,
                tag(" clay. Each geode robot costs "),
                cc::u16,
                tag(" ore and "),
                cc::u16,
                tag(" obsidian."),
            ))(line)?;
        robot_costs.insert(Robot::Ore, ore);
        robot_costs.insert(Robot::Clay, clay);
        let obsidian = (obsidian2 << 8) | obsidian1;
        robot_costs.insert(Robot::Obsidian, obsidian);
        let geode = (geode2 << 8) | geode1;
        robot_costs.insert(Robot::Geode, geode);
        factories.push(Factory::new(robot_costs));
    }
    Ok((input, factories))
}

pub fn run() {
    println!("{SAMPLE_INPUT}");
    let mut factories = parse(SAMPLE_INPUT).finish().unwrap().1;
    factories[0].cycle(24);
    println!("{:?}", factories[0])
}

const SAMPLE_INPUT: &str = "\
Blueprint 1: \
 Each ore robot costs 4 ore. \
 Each clay robot costs 2 ore. \
 Each obsidian robot costs 3 ore and 14 clay. \
 Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: \
 Each ore robot costs 2 ore. \
 Each clay robot costs 3 ore. \
 Each obsidian robot costs 3 ore and 8 clay. \
 Each geode robot costs 3 ore and 12 obsidian.";
