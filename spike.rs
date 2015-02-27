fn main() {
    for n in values(true) {
        println!("{}", n);
    }
}

fn values(even: bool) -> Box<Iterator<Item=usize>> {
    Box::new(vec![3usize, 4, 2, 1].into_iter()
        .map(|n| n * 2)
        .filter(move |n| if even {
            n % 2 == 0
        } else {
            true
        }))
}
