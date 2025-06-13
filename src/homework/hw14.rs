use std::collections::HashMap;

fn gray(n: u8) -> Vec<String> {
    fn generate(n: u8, memo: &mut HashMap<u8, Vec<String>>) -> Vec<String> {
        if let Some(result) = memo.get(&n) {
            return result.clone();
        }

        let result = match n {
            0 => vec![String::from("")],
            1 => vec![String::from("0"), String::from("1")],
            _ => {
                let prev = generate(n - 1, memo);
                let mut with_zero = prev.iter()
                    .map(|code| format!("0{}", code))
                    .collect::<Vec<_>>();
                let mut with_one = prev.iter()
                    .rev()
                    .map(|code| format!("1{}", code))
                    .collect::<Vec<_>>();
                with_zero.append(&mut with_one);
                with_zero
            }
        };

        memo.insert(n, result.clone());
        result
    }

    let mut memo = HashMap::new();
    generate(n, &mut memo)
}

fn main() {
    let codes = gray(3);
    for code in codes {
        println!("{}", code);
    }
}
