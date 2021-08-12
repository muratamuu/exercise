use rand::Rng;
use std::io::Write;

#[derive(Debug, PartialEq)]
enum Hand {
    Scissors,
    Paper,
    Rock,
}

trait ToHand {
    fn to_hand(self) -> Option<Hand>;
}

impl ToHand for i32 {
    fn to_hand(self) -> Option<Hand> {
        match self {
            1 => Some(Hand::Scissors),
            2 => Some(Hand::Paper),
            3 => Some(Hand::Rock),
            _ => None,
        }
    }
}

fn main() {
    loop {
        print!("please input your hand (scissors:1, paper:2, rock:3) or Quit:q -> ");
        std::io::stdout().flush().unwrap();
        let input: String = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            s
        };
        if input.chars().next() == Some('q') {
            break;
        }
        let you = input.trim().parse::<i32>().unwrap().to_hand().unwrap();
        let com = rand::thread_rng().gen_range(1..=3).to_hand().unwrap();
        if you == com {
            println!("draw");
        } else if (you == Hand::Scissors && com == Hand::Paper) ||
                  (you == Hand::Paper && com == Hand::Rock) ||
                  (you == Hand::Rock && com == Hand::Scissors) {
            println!("you win");
        } else {
            println!("you lose");
        }
    }
}
