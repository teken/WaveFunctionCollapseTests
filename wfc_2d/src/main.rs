use rand::{self, Rng};
use std::collections::{HashMap, HashSet, VecDeque};

// 0:║ 1:═ 2:╠ 3:╣ 4:╦ 5:╩ 6:╬ 7:╚ 8:╝ 9:╔ 10:╗ 11:

fn main() {
    println!("WFC 2D");

    println!("{}", wfc());
    println!("{}", wfc());
    println!("{}", wfc());
}

fn wfc() -> String {
    let top_con = vec![0, 2, 3, 4, 6, 9, 10];
    let top_bla = vec![1, 5, 7, 8, 11];

    let bottom_con = vec![0, 2, 3, 5, 6, 7, 8];
    let bottom_bla = vec![1, 4, 9, 10, 11];

    let left_con = vec![1, 2, 4, 5, 6, 7, 9];
    let left_bla = vec![0, 3, 8, 10, 11];

    let right_con = vec![1, 3, 4, 5, 6, 8, 10];
    let right_bla = vec![0, 2, 7, 9, 11];

    let tiles = HashMap::from([
        (
            0,
            Tile {
                character: '║',
                top_connections: top_con.clone(),
                bottom_connections: bottom_con.clone(),
                left_connections: left_bla.clone(),
                right_connections: right_bla.clone(),
                ..Default::default()
            },
        ),
        (
            1,
            Tile {
                character: '═',
                top_connections: top_bla.clone(),
                bottom_connections: bottom_bla.clone(),
                left_connections: left_con.clone(),
                right_connections: right_con.clone(),
                ..Default::default()
            },
        ),
        (
            2,
            Tile {
                character: '╠',
                top_connections: top_con.clone(),
                bottom_connections: bottom_con.clone(),
                left_connections: left_bla.clone(),
                right_connections: right_con.clone(),
                ..Default::default()
            },
        ),
        (
            3,
            Tile {
                character: '╣',
                top_connections: top_con.clone(),
                bottom_connections: bottom_con.clone(),
                left_connections: left_con.clone(),
                right_connections: right_bla.clone(),
                ..Default::default()
            },
        ),
        (
            4,
            Tile {
                character: '╦',
                top_connections: top_bla.clone(),
                bottom_connections: bottom_con.clone(),
                left_connections: left_con.clone(),
                right_connections: right_con.clone(),
                ..Default::default()
            },
        ),
        (
            5,
            Tile {
                character: '╩',
                top_connections: top_con.clone(),
                bottom_connections: bottom_bla.clone(),
                left_connections: left_con.clone(),
                right_connections: right_con.clone(),
                ..Default::default()
            },
        ),
        (
            6,
            Tile {
                character: '╬',
                top_connections: top_con.clone(),
                bottom_connections: bottom_con.clone(),
                left_connections: left_con.clone(),
                right_connections: right_con.clone(),
                ..Default::default()
            },
        ),
        (
            7,
            Tile {
                character: '╚',
                top_connections: top_con.clone(),
                bottom_connections: bottom_bla.clone(),
                left_connections: left_bla.clone(),
                right_connections: right_con.clone(),
                ..Default::default()
            },
        ),
        (
            8,
            Tile {
                character: '╝',
                top_connections: top_con.clone(),
                bottom_connections: bottom_bla.clone(),
                left_connections: left_con.clone(),
                right_connections: right_bla.clone(),
                ..Default::default()
            },
        ),
        (
            9,
            Tile {
                character: '╔',
                top_connections: top_bla.clone(),
                bottom_connections: bottom_con.clone(),
                left_connections: left_bla.clone(),
                right_connections: right_con.clone(),
                ..Default::default()
            },
        ),
        (
            10,
            Tile {
                character: '╗',
                top_connections: top_bla.clone(),
                bottom_connections: bottom_con.clone(),
                left_connections: left_con.clone(),
                right_connections: right_bla.clone(),
                ..Default::default()
            },
        ),
        (
            11,
            Tile {
                character: ' ',
                top_connections: top_bla.clone(),
                bottom_connections: bottom_bla.clone(),
                left_connections: left_bla.clone(),
                right_connections: right_bla.clone(),
                ..Default::default()
            },
        ),
    ]);

    // let top_con = vec![0, 2];
    // let top_bla = vec![1];

    // let bottom_con = vec![0, 2];
    // let bottom_bla = vec![1];

    // let left_con = vec![1, 2];
    // let left_bla = vec![0, 2];

    // let right_con = vec![1, 2];
    // let right_bla = vec![0, 2];

    // let tiles: HashMap<u32, Tile> = HashMap::from([
    //     (
    //         0,
    //         Tile {
    //             character: '║',
    //             top_connections: top_con.clone(),
    //             bottom_connections: bottom_con.clone(),
    //             left_connections: left_bla.clone(),
    //             right_connections: right_bla.clone(),
    //             weight: 0.5,
    //         },
    //     ),
    //     (
    //         1,
    //         Tile {
    //             character: '═',
    //             top_connections: top_bla.clone(),
    //             bottom_connections: bottom_bla.clone(),
    //             left_connections: left_con.clone(),
    //             right_connections: right_con.clone(),
    //             weight: 0.5,
    //         },
    //     ),
    //     (
    //         2,
    //         Tile {
    //             character: '╬',
    //             top_connections: top_con.clone(),
    //             bottom_connections: bottom_con.clone(),
    //             left_connections: left_con.clone(),
    //             right_connections: right_con.clone(),
    //             weight: 0.5,
    //         },
    //     ),
    // ]);

    const GRID_SIZE: usize = 9;
    const TILE_COUNT: usize = 12;

    // fresh code

    let mut output = "".to_string();

    // gen grid
    // clone grid
    // get random position
    // set to random tile
    // add tiles surroundit it to the stack for propergation processing
    // for each stack item, look at the 4 cells around it limit its options based on thoes for cells
    // if stack item changed add the 4 cells around it to the stack

    const EMPTY: Vec<u32> = vec![];
    let mut grid: [Vec<u32>; GRID_SIZE * GRID_SIZE] = [EMPTY; GRID_SIZE * GRID_SIZE];

    for i in 0..(GRID_SIZE * GRID_SIZE) {
        grid[i] = (0..TILE_COUNT as u32).collect();
    }

    let mut grid_loop_clone;
    let mut rng = rand::thread_rng();

    'main: while grid.iter().any(|x| x.len() > 1) {
        grid_loop_clone = grid.clone();
        let mut stack: VecDeque<usize> = VecDeque::new();

        {
            let rand_pos = rng.gen_range(0..GRID_SIZE * GRID_SIZE);
            if grid[rand_pos].len() == 1 {
                continue;
            }
            let rand_tile = rng.gen_range(0..grid[rand_pos].len());
            grid[rand_pos] = vec![grid[rand_pos][rand_tile]];
            // up
            if rand_pos > GRID_SIZE {
                stack.push_back(rand_pos - GRID_SIZE);
            }
            // down
            if rand_pos < GRID_SIZE * (GRID_SIZE - 1) {
                stack.push_back(rand_pos + GRID_SIZE);
            }
            // left
            if rand_pos > 0 {
                stack.push_back(rand_pos - 1);
            }
            // right
            if rand_pos < (GRID_SIZE * GRID_SIZE) - 1 {
                stack.push_back(rand_pos + 1);
            }
        }

        while stack.len() > 0 {
            let stack_item = stack.pop_front().unwrap();
            let stack_options_original = grid[stack_item].clone();
            let mut stack_options = grid[stack_item].clone();

            // up
            if stack_item > GRID_SIZE {
                let up_cell_options = grid[stack_item - GRID_SIZE].clone();
                let possible_options = up_cell_options
                    .iter()
                    .flat_map(|x| tiles.get(x).unwrap().bottom_connections.clone())
                    .collect::<HashSet<_>>();
                stack_options.retain(|x| possible_options.contains(x));
            }

            // down
            if stack_item < GRID_SIZE * (GRID_SIZE - 1) {
                let down_cell_options = grid[stack_item + GRID_SIZE].clone();
                let possible_options = down_cell_options
                    .iter()
                    .flat_map(|x| tiles.get(x).unwrap().top_connections.clone())
                    .collect::<HashSet<_>>();
                stack_options.retain(|x| possible_options.contains(x));
            }
            // left
            if stack_item > 0 {
                let left_cell_options = grid[stack_item - 1].clone();
                let possible_options = left_cell_options
                    .iter()
                    .flat_map(|x| tiles.get(x).unwrap().right_connections.clone())
                    .collect::<HashSet<_>>();
                stack_options.retain(|x| possible_options.contains(x));
            }
            // right
            if stack_item < (GRID_SIZE * GRID_SIZE) - 1 {
                let right_cell_options = grid[stack_item + 1].clone();
                let possible_options = right_cell_options
                    .iter()
                    .flat_map(|x| tiles.get(x).unwrap().left_connections.clone())
                    .collect::<HashSet<_>>();
                stack_options.retain(|x| possible_options.contains(x));
            }

            if stack_options.len() == 0 {
                grid = grid_loop_clone;
                break 'main;
            }

            if stack_options == stack_options_original {
                continue;
            }

            grid[stack_item] = stack_options;

            // up
            if stack_item > GRID_SIZE {
                stack.push_back(stack_item - GRID_SIZE);
            }
            // down
            if stack_item < GRID_SIZE * (GRID_SIZE - 1) {
                stack.push_back(stack_item + GRID_SIZE);
            }
            // left
            if stack_item > 0 {
                stack.push_back(stack_item - 1);
            }
            // right
            if stack_item < (GRID_SIZE * GRID_SIZE) - 1 {
                stack.push_back(stack_item + 1);
            }
        }
    }

    let mut row_count = 1;
    for cell in grid {
        let tile_char = if cell.len() > 0 {
            let t_i = (*cell.first().unwrap()) as usize;
            tiles.get(&(t_i as u32)).unwrap().character
        } else {
            '#'
        };
        output = output + tile_char.to_string().as_str();
        if row_count == GRID_SIZE {
            output = output + "\n";
            row_count = 1;
        } else {
            row_count += 1;
        }
    }

    output
}

#[derive(Default)]
struct Tile {
    character: char,
    top_connections: Vec<u32>,
    bottom_connections: Vec<u32>,
    left_connections: Vec<u32>,
    right_connections: Vec<u32>,
    weight: f32,
}