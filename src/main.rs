#[cfg(not(tarpaulin_include))]
fn main() {
    println!("Hello, world!");
}

fn this_function_is_not_tested(input: u32) {
    if input == 2 {
        println!("It's a two!");
    } else {
        println!("It's a {}", input);
    }
}

#[cfg(test)]
mod tests {
    use crate::this_function_is_not_tested;

    #[test]
    fn check_function() {
        this_function_is_not_tested(123);
        this_function_is_not_tested(2);
    }
}
