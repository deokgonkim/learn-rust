use std::vec::Vec;
use std::io;

fn sort_vector(input: &Vec<i32>) -> Vec<i32> {
    let mut tobe_sorted: Vec<i32> = Vec::new();
    for i in input.iter() {
        tobe_sorted.push(*i);
    }
    for i in 0..tobe_sorted.len() {
        for j in i..tobe_sorted.len() {
            if tobe_sorted[i] > tobe_sorted[j] {
                let temp = tobe_sorted[i];
                tobe_sorted[i] = tobe_sorted[j];
                tobe_sorted[j] = temp;
            }
        }
    }
    return tobe_sorted;
}


fn fn_avg(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut count = 0;
    for i in vec.iter() {
        sum += i;
        count += 1;
    }
    sum / count
}

fn fn_median(vec: &Vec<i32>) -> i32 {
    let count = vec.len();
    let middle = count / 2;
    let sorted = sort_vector(&vec);
    println!("sorted: {:?}", sorted);
    return sorted[middle];
}

fn main() {
    let mut user_input = String::new();
    let mut vec: Vec<i32> = Vec::new();

    loop {
        println!("Please enter a number: ");
        user_input.clear();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        match user_input.trim() {
            "exit" => break,
            _ => (),
        }

        let number: i32 = user_input.trim().parse().expect("Please type a number");
        vec.push(number);
    }

    println!("Completed user input count: {}", vec.len());

    let median = fn_median(&vec);

    println!("The median of the numbers: {}", median);
}

