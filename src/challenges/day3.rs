use crate::challenges::utils::read_file;

pub struct Path {
    right: usize,
    down: usize,
}

pub fn nb_trees_in(data: &[String], p: &Path) -> usize {
    data.iter()
        .enumerate()
        .step_by(p.down)
        .map(|(n, l)| l.chars().nth(p.right * n % l.len()).unwrap())
        .filter(|&c| c == '#')
        .count()
}

pub fn part1() {
    let data: Vec<String> = read_file("./resources/3_1.txt").unwrap();

    println!("{:?}", nb_trees_in(&data, &Path { right: 3, down: 1 }));
}

pub fn part2() {
    let data: Vec<String> = read_file("./resources/3_1.txt").unwrap();

    let paths = vec![
        Path { right: 1, down: 1 },
        Path { right: 3, down: 1 },
        Path { right: 5, down: 1 },
        Path { right: 7, down: 1 },
        Path { right: 1, down: 2 },
    ];

    println!(
        "{}",
        paths
            .iter()
            .map(|p| nb_trees_in(&data, p))
            .product::<usize>()
    );
}
