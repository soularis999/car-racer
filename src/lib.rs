pub mod init;
pub mod state;
pub mod game;


const PLAYER_NAME: &str = "player";
const SCORE_BOARD: &str = "score";
const LIFE_BOARD: &str = "life";
const RESULT: &str = "result";
const OBSTACLE_NAME: &str = "obstacle_";
const MAX_Y: i32 = 480;
const MAX_X: i32 = 320;
const INIT_POS: (f32, f32) = (0.0, -1.0 * (MAX_Y as f32) + 80.0);

