use std::fs::File;
use std::io::{self, Read};
use std::collections::HashSet;


fn check(data: &[u8], index: usize) -> bool {
    let tmp = data[index];
    (tmp & 1) == 0
}

fn main() {
    
    let booms: HashSet<usize> = vec![
        2, 11, 57, 114, 142, 180, 187, 218, 323, 384, 386, 432, 455, 463, 515, 546, 553, 635, 638, 641, 
        654, 709, 792, 893, 903, 943, 1021, 1049, 1056, 1088, 1135, 1334, 1348, 1386, 1488, 1518, 1589, 
        1963, 2019, 2269, 2272, 2417, 2432, 2539, 2704, 2997, 3011, 3286, 3393, 3461, 3588, 3696, 3714, 
        3753, 3766, 3910, 3939, 3943, 3948, 3964, 3981, 4018, 4033, 4088
    ].into_iter().collect();


    println!("Welcome to the game!");

    let maze = vec![
        vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
        vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'],
        vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'],
        vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'],
        vec!['#', 'W', 'E', 'L', 'C', 'O', 'M', 'E', '!', '#'],
        vec!['#', ' ', ' ', ' ', ' ', '.', '.', '.', ' ', '#'],
        vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', '.', ' ', '#'],
        vec!['#', ' ', ' ', ' ', '.', '.', ' ', '.', ' ', '#'],
        vec!['#', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '#'],
        vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
    ];

    for row in maze {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
    loop{
        println!("Enter the path to win the game");
        println!("w: up, s: down, a: left, d: right");
        println!("Example: wwsdd");
        println!("Your path: ");   

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        input = input.trim().to_string();


        for c in input.chars() {
            if !matches!(c, 'w' | 's' | 'a' | 'd') {
                println!("Invalid path");
                return;
            }
        }

        let mut file = File::open("map").expect("Cannot open file");
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("Failed to read file");

        let (mut x, mut y):(usize, usize) = (0, 0);

        for c in input.chars() {
            let mut dx: isize = 0;
            let mut dy: isize = 0;

            if c == 'w' {
                dx = -1;
            } else if c == 's' {
                dx = 1;
            } else if c == 'a' {
                dy = -1;
            } else if c == 'd' {
                dy = 1;
            } else {
                dy = 0;
                dx = 0;
            }

            let new_x = (x as isize + dx) as isize;
            let new_y = (y as isize + dy) as isize;

            if  new_x >= 0 && new_x < 64 && new_y >= 0 && new_y < 64{
                let tmp = (new_x * 64 + new_y) as usize;
                if check(&data, tmp) {
                    x = new_x as usize;
                    y = new_y as usize;
                }
            }
            
            if booms.contains(&(x * 64 + y)) {
                println!("Boom! You died!");
                return;
            }
        }

        if x * 64 + y == 62 * 64 + 63 {
            println!("You won!");
            println!("The flag is: BKISC{{This_is_a_fake_flag}}");
            break;
        } else {
            println!("Try again!");
        }
    }
}
