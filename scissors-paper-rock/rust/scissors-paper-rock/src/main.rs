use rand::Rng;
use std::io::Write;

fn main() {
    print!("please input your hand (guu:1, choki:2, paa:3): ");
    std::io::stdout().flush().unwrap();
    // guu: 1, choki: 2, paa: 3
    let you: i32 = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim().parse().unwrap()
    };
    let com: i32 = rand::thread_rng().gen_range(1..=3);
    if you == com {
        println!("draw");
    } else if (you == 1 && com == 2) || (you == 2 && com == 3) || (you == 3 && com == 1) {
        println!("you win");
    } else {
        println!("you lose");
    }
}
