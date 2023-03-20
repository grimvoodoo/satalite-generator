use rand::{Rng, rngs};
use rand::distributions::{Distribution, Standard};
use rand_derive2::*;
use parse_display::Display;

#[derive(Debug, RandGen, Display)]
enum Names {
    Horizon,
    Event,
    Enterprise,
    Voyager,
}

#[derive(Debug, RandGen, Display)]
enum Enemies {
    Goblins,
    Robots,
    Daemons,
    Ghosts,
    None,
}

#[derive(Debug)]
struct Satalite {
    name: Names,
    oxygen: bool,
    heat: bool,
    power: bool,
    enemies: Enemies,
}

fn main() {
    let mut satalite_list = Vec::new();
    for x in 0..5 {
        let satalite = Satalite {
            name: rand::random(),
            oxygen: coin_flip(),
            heat: coin_flip(),
            power: coin_flip(),
            enemies: rand::random(),

        };
        satalite_list.push(satalite)

    }
    println!("Hello, world!");
    println!("{:?}", satalite_list)
}


fn coin_flip() -> bool {
    let coin;
    let mut rng = rand::thread_rng();
    if rng.gen_range(0..2) == 1 {
        coin = true;
    } else {
        coin = false;
    }
    coin
}