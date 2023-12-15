use advent::prelude::*;

fn parse_input(input: &str) -> VecDeque<Vec<Vector3>> {
    input
        .split("\n\n")
        .map(|scan| {
            scan.lines()
                .skip(1)
                .filter_map(|line| {
                    line.split(',')
                        .map(str::parse)
                        .map(Result::unwrap)
                        .next_array::<3>()
                        .ok()
                })
                .map(Vector3::from)
                .collect()
        })
        .collect()
}

fn default_input() -> VecDeque<Vec<Vector3>> {
    parse_input(include_input!(2021 / 19))
}

fn roll(bs: Vec<Vector3>) -> Vec<Vector3> {
    bs.into_iter().map(|b| vector![b.x, b.z, -b.y]).collect()
}

fn turn(bs: Vec<Vector3>) -> Vec<Vector3> {
    bs.into_iter().map(|b| vector![-b.y, b.x, b.z]).collect()
}

fn is_match(beacons: &HashSet<Vector3>, scan: &[Vector3]) -> Option<(Vector3, Vec<Vector3>)> {
    beacons
        .iter()
        .cartesian_product(scan)
        .map(|(b1, b2)| b1 - b2)
        .find_map(|d| {
            let shifted = scan.iter().map(|b| b + d);
            let matching = shifted.clone().filter(|b| beacons.contains(b)).count();
            (matching >= 12).then(|| (d, shifted.collect()))
        })
}

fn find_match(beacons: &mut HashSet<Vector3>, mut scan: Vec<Vector3>) -> Option<Vector3> {
    for _ in 0..2 {
        for _ in 0..3 {
            scan = roll(scan);
            if let Some((d, bs)) = is_match(beacons, &scan) {
                beacons.extend(bs);
                return Some(d);
            }
            for _ in 0..3 {
                scan = turn(scan);
                if let Some((d, bs)) = is_match(beacons, &scan) {
                    beacons.extend(bs);
                    return Some(d);
                }
            }
        }
        scan = roll(turn(roll(scan)));
    }
    None
}

fn solve(mut scans: VecDeque<Vec<Vector3>>) -> (HashSet<Vector3>, Vec<Vector3>) {
    let mut beacons: HashSet<_> = scans.pop_front().unwrap().into_iter().collect();
    let mut scanners = vec![vector![0, 0, 0]];
    while let Some(scan) = scans.pop_front() {
        match find_match(&mut beacons, scan.clone()) {
            Some(scanner) => scanners.push(scanner),
            None => scans.push_back(scan),
        }
    }
    (beacons, scanners)
}

fn part1(scans: VecDeque<Vec<Vector3>>) -> usize {
    let (beacons, _) = solve(scans);
    beacons.len()
}

fn part2(scans: VecDeque<Vec<Vector3>>) -> i64 {
    let (_, scanners) = solve(scans);
    scanners
        .into_iter()
        .array_combinations()
        .map(|[b1, b2]| (b1 - b2).l1_norm())
        .max()
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14",
    );
    assert_eq!(part1(input.clone()), 79);
    assert_eq!(part2(input), 3621);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 405);
    assert_eq!(part2(input), 12306);
}
