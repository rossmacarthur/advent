fn parse_input(input: &str, width: usize, height: usize) -> Image {
    let data = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10))
        .map(Option::unwrap)
        .collect();
    Image {
        width,
        height,
        data,
    }
}

fn default_input() -> Image {
    parse_input(include_str!("input/08.txt"), 25, 6)
}

struct Image {
    width: usize,
    height: usize,
    data: Vec<u32>,
}

impl Image {
    const fn size(&self) -> usize {
        self.width * self.height
    }
}

fn count(layer: &[u32], pixel: u32) -> usize {
    layer.iter().filter(|&&p| p == pixel).count()
}

fn part1(img: Image) -> usize {
    img.data
        .chunks(img.size())
        .map(|layer| (count(layer, 0), layer))
        .min()
        .map(|(_, layer)| count(layer, 1) * count(layer, 2))
        .unwrap()
}

fn part2(img: Image) -> String {
    (0..img.size())
        .map(|i| {
            (0..)
                .map(|layer| img.data[layer * img.size() + i])
                .find(|&layer| layer != 2)
                .unwrap()
        })
        .enumerate()
        .fold(String::new(), |mut s, (i, pixel)| {
            if i % img.width == 0 {
                s.push('\n');
            }
            s.push_str(match pixel {
                0 => "  ",
                1 => "██",
                _ => panic!("unrecognized pixel value"),
            });
            s
        })
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input("123456789012", 3, 2);
    assert_eq!(part1(input), 1);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1677);
}
