use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Food<'_>> {
    input
        .lines()
        .map(|line| {
            let [left, right] = line.split(" (contains ").next_array().unwrap();
            Food {
                ingredients: left.split_whitespace().collect(),
                allergens: right.trim_end_matches(')').split(", ").collect(),
            }
        })
        .collect()
}

fn default_input() -> Vec<Food<'static>> {
    parse_input(include_input!(2020 / 21))
}

#[derive(Debug, Clone)]
struct Food<'a> {
    ingredients: HashSet<&'a str>,
    allergens: HashSet<&'a str>,
}

/// Returns a mapping of allergen to foods that could possibly have it.
fn possible<'a>(foods: &[Food<'a>]) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut possible = HashMap::<_, HashSet<_>>::new();
    for food in foods {
        for allergen in food.allergens.clone() {
            possible
                .entry(allergen)
                .and_modify(|current| {
                    *current = current.intersection(&food.ingredients).cloned().collect()
                })
                .or_insert_with(|| food.ingredients.clone());
        }
    }
    possible
}

fn part1(foods: Vec<Food<'_>>) -> usize {
    // First calculate all the ingredients that contain at least one allergen.
    let have_allergen: HashSet<_> = possible(&foods)
        .into_iter()
        .flat_map(|(_, ingredients)| ingredients)
        .collect();

    // Now take all the ingredients and filter out the ones we just calculated.
    foods
        .iter()
        .flat_map(|food| &food.ingredients)
        .filter(|&ingredient| !have_allergen.contains(ingredient))
        .count()
}

fn part2(foods: Vec<Food<'_>>) -> String {
    // First calculate a map of allergen to possible ingredient.
    let mut maybe_dangerous = HashMap::new();
    for (allergen, ingredients) in possible(&foods) {
        for ingredient in ingredients {
            maybe_dangerous
                .entry(ingredient)
                .or_insert_with(HashSet::new)
                .insert(allergen);
        }
    }

    // Now we figure which allergen must belong to which ingredient by
    // iteratively finding an ingredient that can only have one possible
    // allergen.
    let mut dangerous = HashMap::new();
    while let Some((ingredient, allergen)) = {
        maybe_dangerous
            .iter()
            .filter(|(_, allergens)| !allergens.is_empty())
            .sorted_by_key(|(_, allergens)| allergens.len())
            .next()
            .map(|(ingredient, allergens)| {
                assert_eq!(allergens.len(), 1);
                (*ingredient, *allergens.iter().next().unwrap())
            })
    } {
        // Once we have found this allergen we remove it as a possible allergen
        // from all the other ingredients.
        for allergens in maybe_dangerous.values_mut() {
            allergens.remove(allergen);
        }
        dangerous.insert(ingredient, allergen);
    }
    dangerous
        .into_iter()
        .sorted_by_key(|(_, allergen)| *allergen)
        .map(|(ingredient, _)| ingredient)
        .join(",")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let foods = parse_input(
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
",
    );
    assert_eq!(part1(foods.clone()), 5);
    assert_eq!(part2(foods), "mxmxvkd,sqjhc,fvjkl");
}

#[test]
fn default() {
    let foods = default_input();
    assert_eq!(part1(foods.clone()), 2098);
    assert_eq!(
        part2(foods),
        "ppdplc,gkcplx,ktlh,msfmt,dqsbql,mvqkdj,ggsz,hbhsx"
    );
}
