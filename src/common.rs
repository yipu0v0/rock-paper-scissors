#[derive(Debug, PartialEq)]
pub enum GameError {
    EnoughPlayers,
    NotEnoughPlayers,
    GameNotExist,
    AlreadyCommit,
    IllegalShape,
    PlayersNotMatch,
    RevealNotMatch,
    NotTimeToReveal,
    NotEnoughReveal,
}

pub const MAX_PLAYER_COUNT: usize = 2;