use rand::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(1), 2);
    }
}

pub fn add_one(a: i32) -> i32 {
    a + 1
}

pub fn add_rand(a: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let y: i32 = rng.gen();
    a + y
}
