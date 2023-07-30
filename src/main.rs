use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("欢迎来到猜数字游戏！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("请输入一个数字：");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("输入错误");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测：{guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("密码是：{secret_number}");
                println!("你猜对了！");
                break;
            }
        }
    }
}
