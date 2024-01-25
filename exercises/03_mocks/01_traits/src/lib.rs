pub fn square<T: Logger>(x: i32, logger: T) -> i32 {
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

pub trait Logger {
    fn log(&self, msg: &str);
}

pub struct TestLogger;

impl Logger for TestLogger {
    fn log(&self, _msg: &str) {}
}

#[cfg(test)]
mod tests {
    use super::{square, TestLogger};
    use googletest::assert_that;
    use googletest::matchers::eq;

    #[test]
    fn square_works() {
        assert_eq!(square(2, TestLogger), 4);
    }
}
