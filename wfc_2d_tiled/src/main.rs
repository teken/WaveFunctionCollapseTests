use rand::{self, Rng};
use std::collections::{HashMap, HashSet, VecDeque};

// 0:║ 1:═ 2:╠ 3:╣ 4:╦ 5:╩ 6:╬ 7:╚ 8:╝ 9:╔ 10:╗ 11:

fn main() {
    println!("WFC 2D");

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
                weight: 1.0,
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
                weight: 1.0,
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
                weight: 1.0,
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
                weight: 1.0,
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
                weight: 1.0,
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
                weight: 1.0,
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
                weight: 1.0,
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
                weight: 1.0,
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
                weight: 1.0,
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
                weight: 1.0,
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
                weight: 1.0,
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
                weight: 0.6,
            },
        ),
    ]);

    const GRID_SIZE: usize = 16;
    const PROCESSING_TILE_SIZE: usize = 4;
    const PROCESSING_TILE_OVERLAP: usize = 1;
    const TILE_COUNT: usize = 12;

    if GRID_SIZE % PROCESSING_TILE_SIZE != 0 {
        panic!("GRID_SIZE must be divisible by PROCESSING_TILE_SIZE");
    }
    if PROCESSING_TILE_SIZE < PROCESSING_TILE_OVERLAP * 2 {
        panic!("PROCESSING_TILE_SIZE must be greater than PROCESSING_TILE_OVERLAP multiplied by 2");
    }

    const EMPTY: Vec<u32> = vec![];
    let mut grid = [EMPTY; GRID_SIZE * GRID_SIZE];
    let mut processing_grid = [EMPTY; PROCESSING_TILE_SIZE * PROCESSING_TILE_SIZE];
    let mut processing_grid_x = 0;
    let mut processing_grid_y = 0;

    for i in 0..(GRID_SIZE * GRID_SIZE) {
        grid[i] = (0..TILE_COUNT as u32).collect();
    }

    let processing_grid_clone = processing_grid.clone();
    let mut processing_grid_loop_clone;
    let mut processing_grid_loop_clone_use = 0;
    let mut rng = rand::thread_rng();

    while processing_grid_x <= GRID_SIZE - PROCESSING_TILE_SIZE {
        while processing_grid_y <= GRID_SIZE - PROCESSING_TILE_SIZE {
            for i in 0..(PROCESSING_TILE_SIZE * PROCESSING_TILE_SIZE) {
                let x = i % PROCESSING_TILE_SIZE;
                let y = i / PROCESSING_TILE_SIZE;
                let grid_pos = (processing_grid_x + x) + ((processing_grid_y + y) * GRID_SIZE);
                processing_grid[i] = grid[grid_pos].clone();
            }
            'sub: while processing_grid.iter().any(|x| x.len() > 1) {
                processing_grid_loop_clone = processing_grid.clone();
                if processing_grid_loop_clone_use > 100 {
                    processing_grid = processing_grid_clone.clone();
                    processing_grid_loop_clone_use = 0;
                    continue;
                }
                let mut stack: VecDeque<usize> = VecDeque::new();

                {
                    let mut i = -1;
                    let possible_cells = processing_grid
                        .iter()
                        .map(|x| {
                            i += 1;
                            (i, x)
                        })
                        .filter(|x| x.1.len() > 1)
                        .collect::<Vec<_>>();
                    let rand_pos =
                        possible_cells[rng.gen_range(0..possible_cells.len())].0 as usize;

                    if processing_grid[rand_pos].len() == 1 {
                        continue;
                    }
                    let rand_tile = rng.gen_range(0..processing_grid[rand_pos].len());
                    processing_grid[rand_pos] = vec![processing_grid[rand_pos][rand_tile]];
                    // up
                    if rand_pos > PROCESSING_TILE_SIZE {
                        stack.push_back(rand_pos - PROCESSING_TILE_SIZE);
                    }
                    // down
                    if rand_pos < PROCESSING_TILE_SIZE * (PROCESSING_TILE_SIZE - 1) {
                        stack.push_back(rand_pos + PROCESSING_TILE_SIZE);
                    }
                    // left
                    if rand_pos > 0 {
                        stack.push_back(rand_pos - 1);
                    }
                    // right
                    if rand_pos < (PROCESSING_TILE_SIZE * PROCESSING_TILE_SIZE) - 1 {
                        stack.push_back(rand_pos + 1);
                    }
                }

                while !stack.is_empty() {
                    let stack_item = stack.pop_front().unwrap();
                    let stack_options_original = processing_grid[stack_item].clone();
                    let mut stack_options = processing_grid[stack_item].clone();

                    // up
                    if stack_item > PROCESSING_TILE_SIZE {
                        let up_cell_options =
                            processing_grid[stack_item - PROCESSING_TILE_SIZE].clone();
                        let possible_options = up_cell_options
                            .iter()
                            .flat_map(|x| tiles.get(x).unwrap().bottom_connections.clone())
                            .collect::<HashSet<_>>();
                        stack_options.retain(|x| possible_options.contains(x));
                    }

                    // down
                    if stack_item < PROCESSING_TILE_SIZE * (PROCESSING_TILE_SIZE - 1) {
                        let down_cell_options =
                            processing_grid[stack_item + PROCESSING_TILE_SIZE].clone();
                        let possible_options = down_cell_options
                            .iter()
                            .flat_map(|x| tiles.get(x).unwrap().top_connections.clone())
                            .collect::<HashSet<_>>();
                        stack_options.retain(|x| possible_options.contains(x));
                    }
                    // left
                    if stack_item > 0 {
                        let left_cell_options = processing_grid[stack_item - 1].clone();
                        let possible_options = left_cell_options
                            .iter()
                            .flat_map(|x| tiles.get(x).unwrap().right_connections.clone())
                            .collect::<HashSet<_>>();
                        stack_options.retain(|x| possible_options.contains(x));
                    }
                    // right
                    if stack_item < (PROCESSING_TILE_SIZE * PROCESSING_TILE_SIZE) - 1 {
                        let right_cell_options = processing_grid[stack_item + 1].clone();
                        let possible_options = right_cell_options
                            .iter()
                            .flat_map(|x| tiles.get(x).unwrap().left_connections.clone())
                            .collect::<HashSet<_>>();
                        stack_options.retain(|x| possible_options.contains(x));
                    }

                    if stack_options.is_empty() {
                        processing_grid = processing_grid_loop_clone;
                        processing_grid_loop_clone_use += 1;
                        continue 'sub;
                    }

                    if stack_options == stack_options_original {
                        continue;
                    }

                    processing_grid[stack_item] = stack_options;

                    // up
                    if stack_item > PROCESSING_TILE_SIZE {
                        stack.push_back(stack_item - PROCESSING_TILE_SIZE);
                    }
                    // down
                    if stack_item < PROCESSING_TILE_SIZE * (PROCESSING_TILE_SIZE - 1) {
                        stack.push_back(stack_item + PROCESSING_TILE_SIZE);
                    }
                    // left
                    if stack_item > 0 {
                        stack.push_back(stack_item - 1);
                    }
                    // right
                    if stack_item < (PROCESSING_TILE_SIZE * PROCESSING_TILE_SIZE) - 1 {
                        stack.push_back(stack_item + 1);
                    }
                }
            }

            for i in 0..(PROCESSING_TILE_SIZE * PROCESSING_TILE_SIZE) {
                let x = i % PROCESSING_TILE_SIZE;
                let y = i / PROCESSING_TILE_SIZE;
                let grid_pos = (processing_grid_x + x) + ((processing_grid_y + y) * GRID_SIZE);
                grid[grid_pos] = processing_grid[i].clone();
            }
            processing_grid_y += PROCESSING_TILE_SIZE - (PROCESSING_TILE_OVERLAP * 2);
            processing_grid_loop_clone_use = 0;
        }
        processing_grid_y = 0;
        processing_grid_x += PROCESSING_TILE_SIZE - (PROCESSING_TILE_OVERLAP * 2);
    }

    let mut row_count = 1;
    let mut output = "\n".to_string();

    for cell in grid {
        let tile_char = if !cell.is_empty() {
            let t_i = (*cell.first().unwrap()) as usize;
            tiles.get(&(t_i as u32)).unwrap().character
        } else {
            '#'
        };
        output += tile_char.to_string().as_str();
        if row_count == GRID_SIZE {
            output += "\n";
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
