use std::collections::HashMap;
use std::time::Instant;
use cached::proc_macro::cached;
use num_bigint::{ToBigInt, BigInt};
use num_traits::{Zero, One};
use num::pow;
fn main(){
    let now = Instant::now();
       println!("{:?} : {:?}", 10_i128.pow(25),
                prime_sum_sub_digits(BigInt::parse_bytes(b"10", 10).unwrap().pow( 25)));
    println!("Elapsed time: {:.2?}", now.elapsed());
}

//sets up a range for a binary search with reasonable
//upper and lower bound. then uses the distribution function
//to evaluate if it needs to search higher or lower
fn prime_sum_sub_digits(target:BigInt)-> BigInt{
    let mut upperbound:BigInt = Zero::zero();
    upperbound += target.clone() * 10 as u32;

    let mut lowerbound:BigInt = Zero::zero();
    lowerbound += target.clone();
    let mut guess:BigInt = (upperbound.clone() + lowerbound.clone()) / (2 as u32);
    let mut map: HashMap<i32, bool> = HashMap::new();
    for i in 1..((upperbound.clone().to_str_radix(10).len() + 10)*10){
        if is_prime(i as i32){
            map.insert(i as i32, true);
        }else{
            map.insert(i as i32, false);
        }
    }
    let mut solved:bool = false;
    while !solved {
        solved = true;

        guess = (upperbound.clone() + lowerbound.clone()) / 2 as u32;
        let digits:Vec<i32> = guess.to_string()
                                 .chars()
                                 .map(|x:char| x.to_string().parse::<i32>().unwrap())
                                 .collect();
        for e in digits.iter(){
            print!("{e}");
        }
        let current_result = distribution(&digits, &map);
        if current_result > target {
            upperbound = guess.clone();
            solved = false;
        }
        if current_result < target {
            lowerbound = guess.clone();
            solved = false;
        }
        println!("lowerbound:{lowerbound}, upperbound:{upperbound}, guess{guess}, current_result:{current_result}");

    }
    solved = false;
    while !solved{
        solved = true;
        let digits:Vec<i32> = guess.to_string()
                                 .chars()
                                 .map(|x:char| x.to_string().parse::<i32>().unwrap())
                                 .collect();
        let current_result = distribution(&digits, &map);
        println!("guess:{guess}, current_result{current_result}, target{target}");

        if current_result == target {
            guess -= 1 as i32;
            solved = false;
        }
    }
    guess += 1 as i32;
    return guess;
}


//sums all combinations of the sums of the digits of number n by only
//looking at the leading digit followed by 0s then treating the next digit
//as if its only followed by 0s then repeating down till the last digit
fn distribution(digits: &Vec<i32>, map: &HashMap<i32, bool>)->BigInt{
    let mut sum:BigInt = Zero::zero();
    for leading_digit in 0..digits.len(){

            let mut sum_of_leading_digits:usize = 0;
            for i in 0..leading_digit{
                sum_of_leading_digits+= *digits.get(i).unwrap() as usize;
            }
            for i in 1..((digits.len()-leading_digit)*9+*digits.get(leading_digit).unwrap() as usize){
                if *map.get(&((i + sum_of_leading_digits)as i32)).unwrap_or(&false){
                    sum += num_ways_to_sum_dice(
                        *digits.get(leading_digit).unwrap(),
                        (digits.len()-leading_digit-1) as i32,
                        (i) as i32);
                }

            }
    }
    return sum;
}
//first entry is sides of the leading dice which will be 1-9
//second is number of 10 sided dice which can roll 0-9
//last entry is the sum target
fn num_ways_to_sum_dice(leading_die_sides:i32, ten_sided_dice:i32, sum_target:i32)->BigInt{
    let mut sum_of_sum_target:BigInt = Zero::zero();
    if sum_target > 0&& sum_target <=leading_die_sides && ten_sided_dice == 0 {
        return One::one();
    }
    if ten_sided_dice == 0 {
        return Zero::zero();
    }
    if leading_die_sides == sum_target {
        sum_of_sum_target += 1 as i32;
    }
    for i in 0..leading_die_sides{
        if sum_target>=i {
            sum_of_sum_target += ways_to_sum_n_tensided_die_to_s(ten_sided_dice, sum_target-i);
        }
    }
    return sum_of_sum_target;

}
//cache size should be about upperbound length^2 * 9
#[cached(size = 6000)]
fn ways_to_sum_n_tensided_die_to_s(n:i32, s:i32)->BigInt{
    let mut sum:BigInt = Zero::zero();
    let score = s+n;
    for j in 0..=((score - n)/10){
        let current_result = pow(-1, j as usize)
                  *binom(n, j as i32)
                  *binom(score - (10*j as i32) - 1, n - 1);
        println!("current_result:{current_result}");
        sum += current_result;
    }
    println!("sum{sum}");
    return sum;
}

fn binom(n:i32, k:i32) -> BigInt{
    let mut res:BigInt = One::one();
    for i in 1..k+1{
        res = res * ((n as i128 - (k as i128 - i as i128))/(i as i128));
    }
    println!("res:{res}");
    return res;
}

fn is_prime(num:i32) -> bool{
    if num <= 1 {
        return false;
    }
    for i in 2..=((num as f32).sqrt()) as i32{
        if num % i as i32 == 0{
            return false;
        }
    }
    return true;
}
