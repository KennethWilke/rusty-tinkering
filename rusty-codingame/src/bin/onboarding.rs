use std::io;

struct Enemy {
    name: String,
    distance: u32
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn read_distance() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_enemy() -> Enemy {
    let name = read_line();
    let distance = read_distance();
    
    Enemy {
        name,
        distance
    }
}

fn main() {
    loop {
        let enemy1 = read_enemy();
        let enemy2 = read_enemy();
        if enemy1.distance < enemy2.distance {
            println!("{}", enemy1.name);
        } else {
            println!("{}", enemy2.name);
        }
    }
}