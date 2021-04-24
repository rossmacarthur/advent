const INPUT: &str = include_str!("input/08.txt");
const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const SIZE: usize = WIDTH * HEIGHT;

pub fn default_input() -> Vec<u32> {
    INPUT
        .trim()
        .chars()
        .map(|c| c.to_digit(10))
        .map(Option::unwrap)
        .collect()
}

fn count(layer: &[u32], pixel: u32) -> usize {
    layer.iter().filter(|&&p| p == pixel).count()
}

pub fn part1(image: &[u32]) -> usize {
    image
        .chunks(SIZE)
        .map(|layer| (count(layer, 0), layer))
        .min()
        .map(|(_, layer)| count(layer, 1) * count(layer, 2))
        .unwrap()
}

pub fn part2(image: &[u32]) -> String {
    (0..SIZE)
        .map(|i| {
            (0..)
                .map(|layer| image[layer * SIZE + i])
                .find(|&layer| layer != 2)
                .unwrap()
        })
        .enumerate()
        .fold(String::new(), |mut s, (i, pixel)| {
            if i % WIDTH == 0 {
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
