mod challenges;
fn main() {
    let data = challenges::utils::read_file("./resources/1_1.txt").unwrap();

    // challenges::day1::part1();
    println!("{:?}", challenges::day1::part2_1(&data));
    println!("{:?}", challenges::day1::part2_2(&data));
    println!("{:?}", challenges::day1::part2_3(&data));
}
