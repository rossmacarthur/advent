use std::collections::{HashMap, HashSet};

use itertools::Itertools;

const INPUT: &str = include_str!("input/day21.txt");

#[derive(Debug)]
pub struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

fn parse_input(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split(" (contains ").next_tuple().unwrap();
            Food {
                ingredients: left.split_whitespace().collect(),
                allergens: right.trim_end_matches(')').split(", ").collect(),
            }
        })
        .collect()
}

pub fn default_input() -> Vec<Food<'static>> {
    parse_input(INPUT)
}

fn possible<'a>(foods: &[Food<'a>]) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut possible = HashMap::<_, HashSet<_>>::new();
    for food in foods {
        for allergen in food.allergens.iter().cloned() {
            possible
                .entry(allergen.clone())
                .and_modify(|current| {
                    *current = current.intersection(&food.ingredients).cloned().collect()
                })
                .or_insert_with(|| food.ingredients.clone());
        }
    }
    possible
}

pub fn part1(foods: &[Food]) -> usize {
    let possible = possible(foods);
    let maybe_allergens: HashSet<_> = possible
        .into_iter()
        .flat_map(|(_, ingredients)| ingredients)
        .collect();
    foods
        .iter()
        .flat_map(|food| food.ingredients.iter())
        .filter(|&ingredient| !maybe_allergens.contains(ingredient))
        .count()
}

pub fn part2(foods: &[Food]) -> String {
    let mut maybe_dangerous = HashMap::new();
    for (allergen, ingredients) in possible(foods).into_iter() {
        for ingredient in ingredients {
            maybe_dangerous
                .entry(ingredient)
                .or_insert_with(HashSet::new)
                .insert(allergen.clone());
        }
    }
    let mut dangerous = HashMap::new();
    while let Some((ingredient, allergen)) = {
        maybe_dangerous
            .iter()
            .filter(|(_, allergens)| !allergens.is_empty())
            .sorted_by_key(|(_, allergens)| allergens.len())
            .next()
            .map(|(ingredient, allergens)| {
                assert_eq!(allergens.len(), 1);
                (ingredient.clone(), allergens.iter().next().unwrap().clone())
            })
    } {
        for allergens in maybe_dangerous.values_mut() {
            allergens.remove(&allergen);
        }
        dangerous.insert(ingredient, allergen);
    }
    dangerous
        .into_iter()
        .sorted_by_key(|(_, allergen)| allergen.clone())
        .map(|(ingredient, _)| ingredient)
        .join(",")
}

#[test]
fn ex1() {
    let foods = parse_input(
        r#"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
"#,
    );
    assert_eq!(part1(&foods), 5);
    assert_eq!(part2(&foods), "mxmxvkd,sqjhc,fvjkl");
}

#[test]
fn default() {
    let foods = default_input();
    assert_eq!(part1(&foods), 2098);
    assert_eq!(
        part2(&foods),
        "ppdplc,gkcplx,ktlh,msfmt,dqsbql,mvqkdj,ggsz,hbhsx"
    );
}
