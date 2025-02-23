use wasm_bindgen::prelude::*;
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen(module="/www/utils/rnd.js")]
extern {
    fn rnd(max: usize) -> usize;
}


#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    Won,
    Lost,
    Played
}

#[derive(Clone, Copy, PartialEq)]
pub struct SnakeCell(usize);

struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {

    fn new(spawn_index: usize, size: usize) -> Self {
        let mut body = vec!();

        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }
        Self {
            // body: vec!(SnakeCell(spawn_index)),
            body,
            direction: Direction::Left
        }
    }
}

#[wasm_bindgen]
pub struct  World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: Option<usize>,
    status: Option<GameStatus>,
    points: usize
}


#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_idx: usize) -> Self{

        let snake = Snake::new(snake_idx, 3);
        let size = width * width;

        // Moved to a separate function on it's own
        // let mut reward_cell;
        // loop {
        //     reward_cell = rnd(size);
        //     if !snake.body.contains(&SnakeCell(reward_cell)){
        //         break;
        //     }
        // }

        // let reward_cell = rnd(size);
        Self {
             reward_cell: World::generate_reward_cell(size, &snake.body),
             width,
             size,
             snake,
             next_cell: None,
             status: None,
             points: 0
        }
    }

    fn generate_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize>{
        let mut reward_cell;

        loop {
            reward_cell = rnd(max);
            if !snake_body.contains(&SnakeCell(reward_cell)){
                break;
            }
        }
        Some(reward_cell)

    }
 
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn points(&self) -> usize{
        self.points
    }

    pub fn reward_cell(&self) -> Option<usize>{
        self.reward_cell
    }

    pub fn game_status(&self) -> Option<GameStatus>{
        self.status
    }

    pub fn game_status_text(&self) -> String{
        match self.status {
            Some(GameStatus::Won) => String::from("You have won"),
            Some(GameStatus::Played) => String::from("Playing.."),
            Some(GameStatus::Lost) => String::from("You have lost"),
            None => String::from("No status"),
        }
    }

    pub fn snake_head_idx(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn start_game(&mut self){
        self.status = Some(GameStatus::Played)
    }

    pub fn change_snake_direction(&mut self, direction: Direction){

        let next_cell = self.generate_next_snake_cell(&direction);

        if self.snake.body[1].0 == next_cell.0{
            return;
        }

        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn snake_length(&self) -> usize{
        self.snake.body.len()
    }

    // *const is a raw pointer
    // borrowing rules does not apply to it
    pub fn snake_cells(&self) -> *const SnakeCell{
        self.snake.body.as_ptr()
    }

    // pub fn snake_cells(&self) -> Vec<SnakeCell>{
    //     self.snake.body
    // }

    pub fn update(&mut self) {
        let snake_idx = self.snake_head_idx();
        // self.snake.body[0].0 = (snake_idx + 1) % self.size;

        // let row = snake_idx / self.width;
        // let col = snake_idx % self.width;
        
        // This is refactor of the above immediate two lines
        // let (row, col) = (snake_idx / self.width, snake_idx % self.width);
        let (row, col) = self.index_to_cell(snake_idx);

        let (row, col ) = match self.snake.direction {
            Direction::Right => {
                 (row, (col + 1) % self.width)
            }
            Direction::Left => {
                 (row,  (col - 1) % self.width)
            }
            Direction::Up => {
                ((row -1) % self.width, col)
            }
            Direction::Down => {
                ((row + 1) % self.width, col)
            }
        };

        // Refactoring
        // self.snake.body[0].0 = self.cell_to_index(row, col);
        let next_idx =  self.cell_to_index(row, col);
        self.set_snake_head(next_idx);

        // The below lines were refoctored to the above lines
        // if self.snake.direction == Direction::Right{
        //     let next_col = (col + 1) % self.width;
        //     self.snake.body[0].0 = (row * self.width) + next_col;
        // }

        // if self.snake.direction == Direction::Left{
        //     let next_col = (col - 1) % self.width;
        //     self.snake.body[0].0 = (row * self.width) + next_col;
        // }

        // if self.snake.direction == Direction::Up{
        //     let next_row = (row - 1) % self.width;
        //     self.snake.body[0].0 = (next_row * self.width) + col;
        // }

        // if self.snake.direction == Direction::Down{
        //     let next_row = (row + 1) % self.width;
        //     self.snake.body[0].0 = (next_row * self.width) + col;
        // }


    }

    pub fn step(&mut self){

        match self.status {
            Some(GameStatus::Played) => {
                let temp = self.snake.body.clone();
                match self.next_cell {
                    Some(cell) =>{
                        self.snake.body[0] = cell;
                        self.next_cell = None;
                    } 
        
                    None => {
                        self.snake.body[0] = self.generate_next_snake_cell(&self.snake.direction);
                    }
                }

                // let len = self.snake_length();

                for i in 1..self.snake_length(){
                    self.snake.body[i] = SnakeCell(temp[i - 1].0);
                }

                if self.snake.body[1..self.snake_length()].contains(&self.snake.body[0]){
                    self.status = Some(GameStatus::Lost);
                }

                if self.reward_cell == Some(self.snake_head_idx()){
                    if self.snake_length() < self.size {
                        self.points += 1;
                        self.reward_cell = World::generate_reward_cell(self.size, &self.snake.body);
                    }
                    else {
                        self.reward_cell = None;
                        self.status = Some(GameStatus::Won);
                    }
                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                }

            }
            _ => {}
        }

        // EVERYTHING MOVED TO MATCH

        // let temp = self.snake.body.clone();
        // match self.next_cell {
        //     Some(cell) =>{
        //         self.snake.body[0] = cell;
        //         self.next_cell = None;
        //     } 

        //     None => {
        //         self.snake.body[0] = self.generate_next_snake_cell(&self.snake.direction);
        //     }
        // }

        // This was refactored to include match and with next_cell attribute of World staruct
        // Since next cell is now stored there.
        // let next_cell = self.generate_next_snake_cell(&self.snake.direction);
        // self.snake.body[0] = next_cell;

        // let len = self.snake_length();

        // for i in 1..len{
        //     self.snake.body[i] = SnakeCell(temp[i - 1].0);
        // }

        // if self.reward_cell == self.snake_head_idx(){
        //     if self.snake_length() < self.size {
        //         self.reward_cell = World::generate_reward_cell(self.size, &self.snake.body);
        //     }
        //     else {
        //         self.reward_cell = 1000;
        //     }
        //     self.snake.body.push(SnakeCell(self.snake.body[1].0));
            
        // }

    }

    fn generate_next_snake_cell(&self, direction: &Direction) -> SnakeCell{
        let snake_idx = self.snake_head_idx();
        // let (row, col) = self.index_to_cell(snake_idx);
        let row = snake_idx / self.width;

        return match direction {
            Direction::Right => {
                //  (row, (col + 1) % self.width)
                SnakeCell((row * self.width) + (snake_idx + 1) % self.width)
            }
            Direction::Left => {
                //  (row,  (col - 1) % self.width)
                SnakeCell((row * self.width) + (snake_idx - 1) % self.width)
            }
            Direction::Up => {
                // ((row -1) % self.width, col)
                SnakeCell((snake_idx - self.width) % self.size)
            }
            Direction::Down => {
                // ((row + 1) % self.width, col)
                SnakeCell((snake_idx + self.width) % self.size)
            }
        };

    }

    fn set_snake_head(&mut self, idx: usize) {
        self.snake.body[0].0 = idx
    }

    fn index_to_cell(&self, idx: usize) -> (usize, usize) {
        (idx / self.width, idx % self.width)
    }

    fn cell_to_index(&self, row: usize, col: usize) -> usize{
        (row * self.width) + col
    }
}


// wasm-pack build --target web