use rand::Rng;
use std::io;

struct Door {
    name: String,
    number: u32,
    has_prize: bool,
    is_open: bool,
    belongs_to_user: bool,
}

impl Door {
    fn open(&mut self) {
        self.is_open = true;
    }

    fn put_prize(&mut self) {
        self.has_prize = true
    }
    fn has_prize(&self) -> bool {
        self.has_prize
    }
    fn belongs_to_user(&self) -> bool {
        self.belongs_to_user
    }
    fn assign_to_user(&mut self) {
        self.belongs_to_user = true
    }
    fn release(&mut self) {
        self.belongs_to_user = false
    }
}

fn main() {
    println!("Welcome to Monty Hall game");
    let door1 = Door {
        name: String::from("#1"),
        number: 1,
        has_prize: false,
        is_open: false,
        belongs_to_user: false,
    };
    let door2 = Door {
        name: String::from("#2"),
        number: 2,
        has_prize: false,
        is_open: false,
        belongs_to_user: false,
    };
    let door3 = Door {
        name: String::from("#3"),
        number: 3,
        has_prize: false,
        is_open: false,
        belongs_to_user: false,
    };

    let mut doors: [Door; 3] = [door1, door2, door3];

    put_prize(&mut doors);

    println!("Done");
    println!("Now it's time for you to choose a dor");
    choose_door(&mut doors);

    open_random_door(&mut doors);

    choose_different_door(&mut doors);
}

fn put_prize(doors: &mut [Door; 3]) {
    println!("I am putting a prize behind a door, please wait");

    let door_with_prize: usize = rand::thread_rng().gen_range(0..3);

    doors[door_with_prize].put_prize();
}

fn choose_door(doors: &mut [Door; 3]) {
    println!("What door do you want to choose?");
    let mut user_door = String::new();

    io::stdin()
        .read_line(&mut user_door)
        .expect("Fail to read line");

    let user_door: u32 = match user_door.trim().parse() {
        Ok(num) => {
            if num > 3 || num <= 0 {
                println!("You can choose between 1, 2 or 3. I choosed 1 for you");
                1
            } else {
                num
            }
        }
        Err(_) => 0,
    };

    release_user_door(doors);
    for door in doors {
        if door.number == user_door {
            door.assign_to_user()
        }
    }
}

fn open_random_door(doors: &mut [Door; 3]) {
    for door in doors {
        if !door.belongs_to_user() && !door.has_prize() {
            println!("I opened the door: {}", door.name);
            door.open()
        }
    }
}

fn check_winner(doors: &[Door; 3]) {
    for door in doors {
        if door.has_prize() && door.belongs_to_user() {
            return println!("YOU WIN");
        }
    }
    println!("Sorry :( Try again later")
}

fn choose_different_door(doors: &mut [Door; 3]) {
    println!("Do you want to choose a different door? (y/n)");
    let mut question_answer = String::new();

    io::stdin()
        .read_line(&mut question_answer)
        .expect("Fail to read line");

    let question_answer: String = match question_answer.trim().parse() {
        Ok(answer) => {
            if answer == "y" || answer == "n" {
                answer
            } else {
                "n".to_string()
            }
        }
        Err(_) => "n".to_string(),
    };

    if question_answer == "y" {
        choose_door(doors);
        check_winner(doors)
    } else {
        check_winner(doors)
    }
}

fn release_user_door(doors: &mut [Door; 3]) {
    for door in doors {
        door.release()
    }
}
