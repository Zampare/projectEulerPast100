fn main(){
    for i in 0..=9{
        for j in 0..=9{
            for k in 0..=9{
                for n in 0..=9{

                }
            }
        }
    }
    for i in 0..=36{
        println!("{:?} = {:?}",i,array[i]);
    }

}

fn is_prime(num:i32) -> bool{
    if(num == 1){
        return false;
    }
    for i in 2..=((num as f32).sqrt()) as i32{
        if num % i as i32 == 0{
            return false;
        }
    }
    return true;
}
