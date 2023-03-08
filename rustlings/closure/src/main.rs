#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn give_away(&self, user_preference: Option<ShirtColor>) -> ShirtColor {

        // The unwrap_or_else method on Option<T> is defined by the standard library. 
        // It takes one argument: a closure without any arguments that returns a value T 
        // (the same type stored in the Some variant of the Option<T>, in this case ShirtColor). 
        // If the Option<T> is the Some variant, unwrap_or_else returns the value from within the Some. 
        // If the Option<T> is the None variant, unwrap_or_else calls the closure and returns the value returned by the closure.
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let give_away1 = store.give_away(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, give_away1
    );

    let user_pref2 = None;
    let give_away2 = store.give_away(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, give_away2
    );
}
