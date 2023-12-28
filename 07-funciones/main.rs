fn main() {
    let cat_breeds = vec!["Siamese", "Persian", "Maine Coon", "Bengal", "Sphynx"];
    cats(&cat_breeds);

    let nums = vec![1, 2, 1];
    println!("Suma de los números: {}", num_sum(&nums));
}

fn num_sum(nums: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for &num in nums {
        sum += num;
    }
    sum
}

fn cats(cb: &Vec<&str>) {
    for &breed in cb {
        if breed == "Sphynx" {
            println!("{} is my favorite cat! 🐈", breed);
            continue;
        }
        println!("{}", breed);
    }
}
