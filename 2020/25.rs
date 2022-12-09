fn default_input() -> (u64, u64) {
    (13316116, 13651422)
}

const MOD: u64 = 20201227;
const SUBJECT: u64 = 7;

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

fn part1((card_public, door_public): (u64, u64)) -> u64 {
    let card_secret = find_secret(card_public);
    let door_secret = find_secret(door_public);
    let card_encryption_key = transform(door_public, card_secret);
    let door_encryption_key = transform(card_public, door_secret);
    assert_eq!(card_encryption_key, door_encryption_key);
    card_encryption_key
}

fn main() {
    let mut run = advent::with(default_input);
    run.part(part1);
    run.finish();
}

#[test]
fn example() {
    assert_eq!(part1((5764801, 17807724)), 14897079);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 12929);
}
