use std::collections::BTreeMap;
// use nom::combinator::map;
fn prime_factors(n: i64) -> String 
{
    let mut given_num = n;
    let mut fact: i64 = 2;
    let mut valid_prime_facts: Vec<(i64, u32)> = Vec::new();
    let mut sorted_btree = BTreeMap::new();
    
    while fact * fact <= given_num 
    {
        while given_num % fact == 0 
        {
            *sorted_btree.entry(fact).or_insert(0) += 1;
            given_num /= fact;
        }
        fact += 1;
    }
    
    if given_num > 1 
    {
        *sorted_btree.entry(given_num).or_insert(0) += 1;
    }
    
    //println!("{:?}", sorted_btree);
    let final_string: String = sorted_btree.iter()
                               .map( |(&base_v, &pow_v)| 
                               {
                                   if pow_v > 1
                                   {
                                       format!("({}**{})", base_v, pow_v)
                                   }
                                   else
                                   {
                                       format!("({})", base_v)
                                   }
                               })
                               .collect();
                    
    
    
    final_string
}

// fn is_prime_num(val: i64) -> bool
// {
//     if val == 1 || val == 2 { return true; }
    
//     let mut factr: i64 = 2;
    
//     while factr < val
//     {
//         if val % factr == 0 { return false; }
//         factr += 1;
//     }
//     return true;
// }

// fn prime_power(val: i64) -> (Option<i64>, Option<u32>)
// {
//     let mut base_val: i64 = 2;
//     let mut pow_val : u32 = 1;
    
//     while base_val <= val
//     {
//         if is_prime_num(base_val)
//         {
//             pow_val = 1;
//             while base_val.pow(pow_val) <= val
//             {
//                 if base_val.pow(pow_val) == val
//                 {
//                     return ( Some(base_val), Some(pow_val) );
//                 }
//                 pow_val += 1;
//             }
//         }
//         base_val += 1;
//     }
    
//     return (None, None);
// }

fn main ()
{
    println!("{:?}", prime_factors(123863*7537) );
    //println!("{:?}", is_prime_num(7919) );
    //  println!("{:?}", prime_power(625) );
}
