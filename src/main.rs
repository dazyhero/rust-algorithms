use rand;

fn main() {
    let rand_array = gen_rand_array();
    let output = bubble_sort(rand_array);

    println!("{:?}", output);
}

fn bubble_sort(mut input: Vec<i64>) -> Vec<i64> {
    for _ in 0..input.len() {
        for j in 0..input.len() - 1 {
            if input[j] > input[j + 1] {
                input.swap(j, j + 1);
            }
        }
    }

    input
}

fn gen_rand_array() -> Vec<i64> {
    let mut random_array: Vec<i64> = Vec::with_capacity(40);
    for _ in 0..random_array.capacity() {
        random_array.push(rand::random());
    }

    random_array
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sorts_array() {
        let initial_array = [7, 2, 1, 4].to_vec();

        let sorted = bubble_sort(initial_array);

        let to_equal = [1, 2, 4, 7].to_vec();

        assert_eq!(to_equal, sorted);
    }
}
