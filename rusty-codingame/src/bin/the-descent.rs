use std::io;

fn readline() -> i32 {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line.trim().parse::<i32>().unwrap()
}

fn pick_tallest(mountains: Vec<i32>) -> usize {
    let mut tallest: Option<(usize, i32)> = None;
    for (i, height) in mountains.iter().enumerate() {
        match tallest {
            None => {
                tallest = Some((i, *height))
            },
            Some((_, tallest_height)) => {
                if *height > tallest_height {
                    tallest = Some((i, *height))
                }
            }
        }
    };
    tallest.unwrap().0
}

fn main() {
    loop {
        let mut mountains: Vec<i32> = vec![];
        for _ in 0..8 as usize {
            let mountain: i32 = readline();
            mountains.push(mountain);
        }

        let tallest = pick_tallest(mountains);
        println!("{}", tallest); // The index of the mountain to fire on.
    }
}
