mod challenges;
fn main() {
    let data = challenges::utils::read_file("./resources/1_1.txt").unwrap();

    // challenges::day1::part1();
    println!("{:?}", challenges::day1::part1_1(&data));
    println!("{:?}", challenges::day1::part1_2(&data));
    println!("{:?}", challenges::day1::part1_3(&data));
    println!("{:?}", challenges::day1::part1_4(&data));
    println!("{:?}", challenges::day1::part1_4smart(&data));
    println!("{:?}", challenges::day1::part1_5(&data));
}
