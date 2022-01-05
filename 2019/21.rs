mod intcode;

use intcode::{parse_program, Computer};

fn default_input() -> Vec<i64> {
    parse_program(include_str!("input/21.txt"))
}

fn run_script(input: Vec<i64>, script: &str) -> Option<i64> {
    let mut c = Computer::new(input);

    assert_eq!(c.read_line()?, "Input instructions:");
    c.write_line(script);

    assert_eq!(c.read_line()?, "");
    assert!(matches!(&*c.read_line()?, "Running..." | "Walking..."));
    assert_eq!(c.read_line()?, "");

    match c.next()? {
        v @ 0..=127 => {
            // If the computer returns an ASCII value then we assume that the
            // script failed and output all the images.
            eprint!("{}", v as u8 as char);
            while let Some(line) = c.read_line().as_deref() {
                eprintln!("{}", line)
            }
            panic!("script failed:\n{}", script)
        }
        v => Some(v),
    }
}

fn part1(input: Vec<i64>) -> i64 {
    // Always jumping too early or too late increases our chances of getting
    // stuck. For example
    //
    //     @ABCD
    //     #####.##.##
    //
    // In the above we wouldn't want to jump to D even though we can. The
    // optimal point to avoid the above would be to jump when B or C is a hole.
    // Jumping at this point would avoid a variety of bad scenarios:
    //
    //     @ABCD
    //     ###.##.####
    //
    //     @ABCD
    //     ##.##.#####
    //
    //     @ABCD
    //     ###.#.#####
    //
    //     @ABCD
    //     ##..#######
    //
    // In boolean logic this would be:
    //
    //     (!B || !C) && D
    //
    // We must also jump when A is a hole. So lets add it:
    //
    //     (!A || !B || !C) && D
    //
    // We can factorize out the negation to get the following:
    //
    //     !(A && B && C) && D
    //
    // This we can easily convert to springscript manually.
    let script = "\
OR  A T
AND B T
AND C T
NOT T J
AND D J
WALK";
    run_script(input, script).unwrap()
}

fn part2(input: Vec<i64>) -> i64 {
    // We now have the ability to look further ahead! Now we need to avoid cases
    // like the following:
    //
    //     @ABCDEFGHI
    //     ###.#.#...#####
    //
    //     @ABCDEFGHI
    //     ###.##...#..###
    //
    // Our current script is too eager, it is jumping when C is a hole which is
    // too soon for these cases. So lets make sure that if we did jump then
    // there is another move for us to make. Either we should be able to move
    // forward after jumping or we should be able jump again. We should check
    // that one of E or H are not holes.
    //
    //     !(A && B && C) && D && (E | H)
    //
    // This we can easily convert to springscript manually.
    let script = "\
OR  A T
AND B T
AND C T
NOT T J
AND D J
OR  E T
OR  H T
AND T J
RUN";
    run_script(input, script).unwrap()
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 19356081);
    assert_eq!(part2(input), 1141901823);
}
