use rand;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shall_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn shall_add_one() {
        assert_eq!(3, add_one(2));
    }
}
