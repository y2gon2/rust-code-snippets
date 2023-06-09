use rand::Rng;

const MAP_N: usize = 25;

fn main() {
    let mut rng = rand::thread_rng();
    let mut maze: Vec<Vec<usize>> = Vec::new();

    // matrix setting
    for i in 0..MAP_N {
        maze.push(vec![0; MAP_N]);
    }

    // arounding wall
    for j in 0..MAP_N {
        maze[0][j] = 1;
        maze[MAP_N - 1][j] = 1;
        maze[j][0] = 1;
        maze[j][MAP_N - 1] = 1;
    } 

    // a wall every 2 cells ?
    for y in 2..MAP_N - 2 {
        for x in 2..MAP_N - 2 {
            if y % 2 == 1 || x % 2 == 1{ continue; }

            maze[y][x] = 1;

            let r = rng.gen_range(0..=3);

            match r {
                0 => maze[y - 1][x] = 1,
                1 => maze[y + 1][x] = 1,
                2 => maze[y][x - 1] = 1,
                3 => maze[y][x + 1] = 1,
                _ => {},
            }
        } 
    }

    let tiles = ["⬜️", "⬛️"];
    
    for y in 0..MAP_N {
        for x in  0..MAP_N {
            print!("{}", tiles[maze[y][x]]);
        }
        println!("");
    }
}