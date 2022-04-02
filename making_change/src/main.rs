use std::io;

fn main() {
    println!("How many coins do you want to exchange?");
    let mut coins_to_exchange = String::new();

    io::stdin()
        .read_line(&mut coins_to_exchange)
        .expect("Fail to read line");

    let coins_to_exchange: u32 = match coins_to_exchange.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("You want to exchange {} coins", coins_to_exchange);
    let number_of_coins = change(coins_to_exchange);

    println!("You will get {} coins", number_of_coins);
}

fn change(coins_to_exchange: u32) -> u32 {
    let available_coins: [u32; 6] = [500, 100, 25, 10, 5, 1];
    let mut sum = 0;
    let mut remaining = coins_to_exchange;

    for &coin in available_coins.iter() {
        if remaining >= coin {
            sum = remaining / coin + sum;
            remaining = remaining % coin;
        }
    }

    sum
}
