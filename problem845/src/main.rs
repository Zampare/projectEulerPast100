use std::collections::HashMap;
use std::time::Instant;
use cached::proc_macro::cached;
fn main(){
    let now = Instant::now();
       println!("{:?} : {:?}", 10_i128.pow(16), prime_sum_sub_digits(10_i128.pow(16)));
    println!("Elapsed time: {:.2?}", now.elapsed());
}

//sets up a range for a binary search with reasonable
//upper and lower bound. then uses the distribution function
//to evaluate if it needs to search higher or lower
fn prime_sum_sub_digits(target:i128)-> i128{

    let mut upperbound:i128 = (target as f64* (target.checked_ilog10().unwrap() as f64 * 2 as f64).log2()) as i128;
    let mut lowerbound:i128 = (target as f64* (target.checked_ilog10().unwrap() as f64).log2()) as i128;
    let mut guess:i128 = (upperbound + lowerbound) / 2;
    let mut map: HashMap<i32, bool> = HashMap::new();
    for i in 1..((upperbound.checked_ilog10().unwrap_or(0) + 10)*10){
        if is_prime(i as i32){
            map.insert(i as i32, true);
        }else{
            map.insert(i as i32, false);
        }
    }
    let mut solved:bool = false;
    while !solved {
        solved = true;

        guess = (upperbound + lowerbound) / 2;
        let digits:Vec<i32> = guess.to_string()
                                 .chars()
                                 .map(|x:char| x.to_string().parse::<i32>().unwrap())
                                 .collect();
        let current_result = distribution(&digits, &map);
        if current_result > target {
            upperbound = guess;
            solved = false;
        }
        if current_result < target {
            lowerbound = guess;
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
            guess -=1;
            solved = false;
        }
    }
    guess +=1;
    return guess;
}


//sums all combinations of the sums of the digits of number n by only
//looking at the leading digit followed by 0s then treating the next digit
//as if its only followed by 0s then repeating down till the last digit
fn distribution(digits: &Vec<i32>, map: &HashMap<i32, bool>)->i128{
    let mut sum:i128 = 0;
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
fn num_ways_to_sum_dice(leading_die_sides:i32, ten_sided_dice:i32, sum_target:i32)->i128{
    let mut sum_of_sum_target:i128 = 0;
    if sum_target > 0&& sum_target <=leading_die_sides && ten_sided_dice == 0 {
        return 1;
    }
    if ten_sided_dice == 0 {
        return 0;
    }
    if leading_die_sides == sum_target {
        sum_of_sum_target += 1;
    }
    for i in 0..leading_die_sides{
        if sum_target>=i {
            sum_of_sum_target += ways_to_sum_n_tensided_die_to_s(ten_sided_dice, sum_target-i);
        }
    }
    return sum_of_sum_target;

}
//cache size should be about upperbound length^2 * 9
#[cached(size = 10000)]
fn ways_to_sum_n_tensided_die_to_s(n:i32, s:i32)->i128{
    let mut sum:i128 = 0;
    let score = s+n;
    for j in 0..=((score - n)/10){
        sum += (-1 as i128).pow(j as u32)
                  *binom(n, j as i32)
                  *binom(score - (10*j as i32) - 1, n - 1);
    }
    return sum;
}

fn binom(n:i32, k:i32) -> i128{
    let mut res:i128 = 1;
    for i in 1..k+1{
        res = res * (n as i128 - (k as i128 - i as i128))/(i as i128);
    }
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