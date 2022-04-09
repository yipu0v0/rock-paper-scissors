#[derive(Debug)]
pub enum GameError {
    EnoughPlayers,
    NotEnoughPlayers,
    GameNotExist,
    AlreadyCommit,
    IllegalShape,
    PlayersNotMatch,
    RevealNotMatch,
    NotTimeToReveal,
}

pub const MAX_PLAYER_COUNT: usize = 2;