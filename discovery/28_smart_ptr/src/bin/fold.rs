fn main() {
    let numbers = vec![1, 2, 3, 4];
    let numbers_2 = vec![1, 2, 3, 4];
    let sum = numbers.into_iter().reduce(|acc, item| acc + item).unwrap();
    let sum_2 = numbers_2.into_iter().fold(0, |acc, item| {
        return acc + item;
    });
    println!("{}", sum);
    println!("{}", sum_2);
}
