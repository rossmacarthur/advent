use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2024 / 09)
}

#[derive(Debug, Clone, Copy)]
struct Free {
    addr: u32,
    size: u32,
}
#[derive(Debug, Clone, Copy)]
struct File {
    id: u32,
    addr: u32,
    size: u32,
}

fn blocks(disk: &str, part2: bool) -> (Vec<Free>, Vec<File>) {
    let mut disk = disk.bytes().map(|b| (b - b'0') as u32);
    let mut id = 0;
    let mut addr = 0;
    let mut frees = Vec::new();
    let mut files = Vec::new();
    while let Some(size) = disk.next() {
        if part2 {
            files.push(File { id, addr, size });
        } else {
            for offset in 0..size {
                let addr = addr + offset;
                files.push(File { id, addr, size: 1 });
            }
        }
        id += 1;
        addr += size;
        if let Some(size) = disk.next() {
            frees.push(Free { addr, size });
            addr += size;
        }
    }
    (frees, files)
}

fn solve(disk: &str, part2: bool) -> u64 {
    let mut cs: u64 = 0;

    let (mut frees, mut files) = blocks(disk, part2);

    for file in files.iter_mut().rev() {
        if let Some(f) = frees
            .iter()
            .position(|free| free.size >= file.size && free.addr < file.addr)
        {
            let free = &mut frees[f];
            cs += checksum(file.id, free.addr, file.size);
            free.addr += file.size;
            free.size -= file.size;
            file.size -= file.size;
            if free.size == 0 {
                frees.remove(f);
            }
        }
    }

    for file in files {
        cs += checksum(file.id, file.addr, file.size);
    }

    cs
}

fn checksum(id: u32, addr: u32, size: u32) -> u64 {
    let addrs = (2 * addr + size - 1) * size / 2;
    (id * addrs) as u64
}

fn part1(disk: &str) -> u64 {
    solve(disk, false)
}

fn part2(disk: &str) -> u64 {
    solve(disk, true)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = "2333133121414131402";
    assert_eq!(part1(input), 1928);
    assert_eq!(part2(input), 2858);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 6332189866718);
    assert_eq!(part2(input), 6353648390778);
}
