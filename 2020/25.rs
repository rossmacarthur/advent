const MOD: u64 = 20201227;
const SUBJECT: u64 = 7;

pub fn default_input() -> (u64, u64) {
    (13316116, 13651422)
}

fn transform(subject: u64, secret: u32) -> u64 {
    (0..secret).fold(1, |value, _| value * subject % MOD)
}

fn find_secret(public: u64) -> u32 {
    let mut value = 1;
    let mut secret = 1;
    loop {
        value = value * SUBJECT % MOD;
        if value == public {
            break secret;
        }
        secret += 1;
    }
}

pub fn part1(input: &(u64, u64)) -> u64 {
    let &(card_public, door_public) = input;
    let card_secret = find_secret(card_public);
    let door_secret = find_secret(door_public);
    let card_encryption_key = transform(door_public, card_secret);
    let door_encryption_key = transform(card_public, door_secret);
    assert_eq!(card_encryption_key, door_encryption_key);
    card_encryption_key
}

pub fn part2(_: &(u64, u64)) -> &'static str {
    "we gucci"
}

#[test]
fn example() {
    assert_eq!(part1(&(5764801, 17807724)), 14897079);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(&input), 12929);
}
