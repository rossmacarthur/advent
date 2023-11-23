use advent::prelude::*;

fn parse_attack_type(s: &str) -> AttackType {
    match s {
        "bludgeoning" => AttackType::Bludgeoning,
        "slashing" => AttackType::Slashing,
        "cold" => AttackType::Cold,
        "fire" => AttackType::Fire,
        "radiation" => AttackType::Radiation,
        _ => panic!("unknown damage type `{}`", s),
    }
}

fn parse_group(id: Id, s: &str) -> Group {
    let re = regex!(concat!(
        r"(?P<units>\d+) units each with (?P<hp>\d+) hit points(?: \((?P<modifiers>.*)\))?",
        r" with an attack that does (?P<attack>\d+) (?P<attack_type>\w+) damage at initiative (?P<initiative>\d+)"
    ));
    let caps = re.captures(s).unwrap();
    let units = caps["units"].parse().unwrap();
    let hp = caps["hp"].parse().unwrap();
    let attack_value = caps["attack"].parse().unwrap();
    let attack_type = parse_attack_type(&caps["attack_type"]);
    let attack = (attack_value, attack_type);
    let initiative = caps["initiative"].parse().unwrap();
    let mut damages = [1; 5];
    if let Some(modifiers) = caps.name("modifiers") {
        for modifier in modifiers.as_str().split("; ") {
            let (kind, list) = modifier.split_once(" to ").unwrap();
            let m = match kind {
                "immune" => 0,
                "weak" => 2,
                _ => panic!("unknown modifier `{}`", kind),
            };
            for attack_type in list.split(", ") {
                damages[parse_attack_type(attack_type) as usize] = m;
            }
        }
    }
    Group {
        id,
        units,
        hp,
        damages,
        attack,
        initiative,
    }
}

fn parse_army(faction: Faction, army: &str) -> impl Iterator<Item = Group> + '_ {
    army.lines().skip(1).enumerate().map(move |(i, s)| {
        let id = Id {
            faction,
            index: i as u8,
        };
        parse_group(id, s)
    })
}

fn parse_input(input: &str) -> Vec<Group> {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut groups = Vec::new();
    groups.extend(parse_army(Faction::ImmuneSystem, first));
    groups.extend(parse_army(Faction::Infection, second));
    groups
}

fn default_input() -> Vec<Group> {
    parse_input(include_str!("input/24.txt"))
}

#[derive(Debug, Clone)]
struct Group {
    id: Id,
    units: i64,
    hp: i64,
    damages: [i64; 5], // Multipliers for Bludgeoning, Slashing, Cold, Fire, Radiation
    attack: (i64, AttackType),
    initiative: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Id {
    faction: Faction,
    index: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Faction {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AttackType {
    Bludgeoning = 0,
    Slashing = 1,
    Cold = 2,
    Fire = 3,
    Radiation = 4,
}

impl Group {
    fn faction(&self) -> Faction {
        self.id.faction
    }

    fn effective_power(&self) -> i64 {
        self.units * self.attack.0
    }

    fn attack_type(&self) -> AttackType {
        self.attack.1
    }
}

// Simulate a battle between the groups, returning the winning faction and the
// number of units.
fn simulate(mut groups: Vec<Group>) -> Option<(Faction, i64)> {
    // Reverse sort the groups by initiative
    let by_initiative: Vec<_> = groups
        .iter()
        .sorted_by_key(|group| Reverse(group.initiative))
        .map(|group| group.id)
        .collect();

    loop {
        // # Target selection

        // Reverse sort the groups by effective power and then initiative
        groups.sort_by_cached_key(|group| Reverse((group.effective_power(), group.initiative)));

        let attacks = {
            let mut attacks = HashMap::with_capacity(groups.len());
            let mut already_attacked = HashSet::with_capacity(groups.len());
            for attacker in &groups {
                let defender_id = groups
                    .iter()
                    .filter(|defender| {
                        defender.faction() != attacker.faction()
                            && !already_attacked.contains(&defender.id)
                    })
                    .filter_map(|defender| {
                        let damage = attacker.effective_power()
                            * defender.damages[attacker.attack_type() as usize];
                        (damage != 0).some((damage, defender))
                    })
                    .max_by_key(|(damage, defender)| {
                        (*damage, defender.effective_power(), defender.initiative)
                    })
                    .map(|(_, defender)| defender.id);
                if let Some(defender_id) = defender_id {
                    attacks.insert(attacker.id, defender_id);
                    already_attacked.insert(defender_id);
                }
            }
            attacks
        };

        // # Attacking

        let stalemate = {
            // Build a simple map of group id to the index for groups, this helps
            // us avoid multiple mutable references when modifying the units.
            let indexes: HashMap<_, _> = groups
                .iter()
                .enumerate()
                .map(|(i, group)| (group.id, i))
                .collect();

            let mut stalemate = true;
            for attacker_id in &by_initiative {
                let Some(defender_id) = attacks.get(attacker_id) else {
                    continue;
                };
                let attacker = &groups[indexes[attacker_id]];
                let defender = &groups[indexes[defender_id]];
                let units_lost = attacker.effective_power()
                    * defender.damages[attacker.attack_type() as usize]
                    / defender.hp;
                if units_lost > 0 {
                    groups[indexes[defender_id]].units -= units_lost;
                    stalemate = false;
                }
            }
            stalemate
        };

        // # Remove dead groups

        groups.retain(|group| group.units > 0);

        // # Check if the battle is over

        if stalemate {
            return None;
        }

        // Calculate the number of remaining units for each faction
        let units = groups
            .iter()
            .fold((0, 0), |(f1, f2), group| match group.faction() {
                Faction::ImmuneSystem => (f1 + group.units, f2),
                Faction::Infection => (f1, group.units + f2),
            });

        match units {
            (0, units) => return Some((Faction::Infection, units)),
            (units, 0) => return Some((Faction::ImmuneSystem, units)),
            _ => continue,
        }
    }
}

fn simulate_with_boost(mut groups: Vec<Group>, boost: i64) -> Option<(Faction, i64)> {
    // Boost the immune system's attack
    for group in &mut groups {
        if group.faction() == Faction::ImmuneSystem {
            group.attack.0 += boost;
        }
    }
    simulate(groups)
}

fn part1(groups: Vec<Group>) -> i64 {
    let (_, units) = simulate(groups).unwrap();
    units
}

fn part2(groups: Vec<Group>) -> i64 {
    let mut lo = 0;
    let mut hi = 1;
    loop {
        hi *= 2;
        match simulate_with_boost(groups.clone(), hi) {
            Some((Faction::ImmuneSystem, _)) => break,
            _ => continue,
        }
    }
    loop {
        let boost = (lo + hi) / 2;
        match simulate_with_boost(groups.clone(), boost) {
            Some((Faction::ImmuneSystem, units)) => {
                hi = boost;
                if lo >= hi {
                    return units;
                }
            }
            _ => lo = boost + 1,
        }
    }
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.part(part2);
    run.finish();
}

#[test]
fn example() {
    let input = parse_input("Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4");
    assert_eq!(part1(input.clone()), 5216);
    assert_eq!(part2(input), 51);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 15392);
    assert_eq!(part2(input), 1092);
}
