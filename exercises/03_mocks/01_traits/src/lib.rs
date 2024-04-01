trait Logger {
    fn log(&self, msg: &str);
}
pub fn square<L>(x: i32, logger: L) -> i32
where
    L: Logger,
{
    let y = x * x;
    logger.log(&format!("{}^2 == {}", x, y));
    y
}

pub struct PrintlnLogger;

impl Logger for PrintlnLogger {
    fn log(&self, msg: &str) {
        println!("{}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::square;
    use crate::Logger;
    use googletest::assert_that;
    use googletest::matchers::eq;

    struct TestLogger;

    impl Logger for TestLogger {
        fn log(&self, msg: &str) {}
    }

    #[test]
    fn square_works() {
        assert_eq!(square(2, TestLogger {}), 4);
    }
}
