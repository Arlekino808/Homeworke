use std::collections::HashSet;

fn main() {
    let mut count = 0;

    
    let digits = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut perms = permutations(digits);

    while let Some(perm) = perms.next() {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        
        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = muxa * a;

        if slon < 1000 || slon > 9999 {
            continue;
        }

        let slon_digits: Vec<u32> = slon
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();

        
        let slon_check = [s, l, o, n];
        let slon_set: HashSet<_> = slon_digits.iter().cloned().collect();
        if slon_digits.iter().all(|&d| slon_check.contains(&(d as usize))) &&
           slon_set.len() == 4
        {
            count += 1;
            println!("{:04}", muxa);
            println!("x   {}", a);
            println!("-----");
            println!("{:04}", slon);
            println!();
        }
    }

    println!("Total solutions found: {}", count);
}

fn permutations(mut nums: Vec<u32>) -> impl Iterator<Item = Vec<u32>> {
    let len = nums.len();
    let mut result = Vec::new();
    

    fn permute(nums: &mut Vec<u32>, left: usize, right: usize, result: &mut Vec<Vec<u32>>) {
        if left == right {
            result.push(nums.clone());
            return;
        }

        for i in left..=right {
            nums.swap(left, i);
            permute(nums, left + 1, right, result);
            nums.swap(left, i);
        }
    }

    permute(&mut nums, 0, len - 1, &mut result);
    result.into_iter()
}
