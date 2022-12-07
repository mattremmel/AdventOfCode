pub trait Challenge {
    type Ret;

    fn part1<T>(input: T) -> Self::Ret
    where
        T: AsRef<str>;

    fn part2<T>(input: T) -> Self::Ret
    where
        T: AsRef<str>;
}
