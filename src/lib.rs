use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env, AccountId, PanicOnDefault};
use near_sdk::collections::LookupMap;
use std::str::FromStr;

mod shape;
mod common;
mod commit;

use shape::Shape;
use common::{GameError, MAX_PLAYER_COUNT};
use commit::{Commit, HashIngredient};

#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct Player {
    pub account: AccountId,
    commit: Commit,
    reveal: Option<Shape>,
}

#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Game {
    players: Vec<Player>,
}

impl Game {
    pub fn get_winner(&self) -> Result<Option<Player>, GameError> {
        if self.players.len() != MAX_PLAYER_COUNT {
            return Err(GameError::NotEnoughPlayers);
        }
        if self.players.iter().find(|x| x.reveal.is_none()).is_some() {
            return Err(GameError::NotEnoughReveal);
        }
        let reveals: Vec<Shape> = self.players.iter().map(|x| x.reveal.unwrap()).collect();
        return Ok(Shape::get_result(&reveals).map(|x| self.players[x].clone()));
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct RockPaperScissors {
    games: LookupMap<u32/* game nunber */, Game>,
    game_count: u32,
}

#[near_bindgen]
impl RockPaperScissors {
    #[init]
    pub fn new() -> Self {
        assert!(env::state_read::<Self>().is_none(), "Already initialized");
        Self {
            games: LookupMap::new(b"a".to_vec()),
            game_count: 0,
        }
    }

    pub fn new_game(&mut self) -> u32/* game number */ {
        let game = Default::default();
        self.games.insert(&self.game_count, &game);

        self.game_count += 1;
        self.game_count - 1
    }

    pub fn commit(&mut self, game_num: u32, commit: Commit) -> Result<(), GameError> {
        let mut game = self.find_game(&game_num).ok_or(GameError::GameNotExist)?;

        let account = env::signer_account_id();
        if self.find_player(&game_num, &account).is_some() {
            return Err(GameError::AlreadyCommit);
        }

        if game.players.len() == MAX_PLAYER_COUNT {
            return Err(GameError::EnoughPlayers);
        }

        let player = Player {
            account: env::signer_account_id(),
            commit: commit,
            reveal: None,
        };
        game.players.push(player);
        self.games.insert(&game_num, &game);
        Ok(())
    }

    fn find_player(&self, game_num: &u32, account: &String) -> Option<Player> {
        self.games.get(game_num)?.players.into_iter().find(|item| item.account.eq(account))
    }

    fn find_game(&self, game_num: &u32) -> Option<Game> {
        self.games.get(game_num)
    }

    pub fn reveal(&mut self, game_num: u32, shape: String, factor: String) -> Result<(), GameError> {
        let mut game = self.find_game(&game_num).ok_or(GameError::GameNotExist)?;
        if game.players.len() != MAX_PLAYER_COUNT {
            return Err(GameError::NotTimeToReveal);
        }

        let account = env::signer_account_id();
        let mut player =
            game.players.iter_mut().
                find(|item| item.account.eq(&account)).
                    ok_or(GameError::PlayersNotMatch)?;

        let shape = Shape::from_str(shape.as_str())?;
        let hash_ingredient =
            HashIngredient {
                game_num,
                account,
                shape,
                factor
            };
        if hash_ingredient.match_commit(&player.commit) {
            player.reveal = Some(hash_ingredient.shape);
            self.games.insert(&game_num, &game);
            Ok(())
        } else {
            Err(GameError::RevealNotMatch)
        }
    }
    pub fn get_result(&mut self, game_num: u32) -> Result<Option<AccountId>, GameError> {
        let game = self.find_game(&game_num).ok_or(GameError::GameNotExist)?;
        let ret = game.get_winner();
        self.games.remove(&game_num);
        return ret.map(|r| r.map(|player| player.account));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::MockedBlockchain;
    use near_sdk::testing_env;
    use near_sdk::json_types::ValidAccountId;

    fn get_context(predecessor_account_id: ValidAccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn game_exist() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = RockPaperScissors::new();
        let game_num = contract.new_game();

        let game = contract.find_game(&game_num);
        assert!(game.is_some());
        assert!(game.unwrap().players.is_empty());
    }

    #[test]
    fn player_exist() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = RockPaperScissors::new();
        let game_num = contract.new_game();
        let account = env::signer_account_id();
        let factor = "d".to_string();
        let ingredient =
            HashIngredient {
                game_num,
                account: account.clone(),
                shape: Shape::Rock,
                factor,
            };

        let commit = ingredient.hash();
        assert!(contract.commit(game_num, commit.clone()).is_ok());

        let player = contract.find_player(&game_num, &account);
        assert!(player.is_some());

        let player = player.unwrap();
        assert!(player.account == account);
        assert!(player.commit == commit);
        assert!(player.reveal.is_none());
    }

    #[test]
    fn game_not_exist() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = RockPaperScissors::new();

        let ingredient =
            HashIngredient {
                game_num: 0,
                account: env::signer_account_id().clone(),
                shape: Shape::Rock,
                factor: "alice".to_string(),
            };
        
        assert_eq!(contract.commit(0, ingredient.hash().clone()), Err(GameError::GameNotExist));
    }

    fn normal_play(shape_of_account0: Shape, shape_of_account1: Shape, expected_winner: Option<ValidAccountId>) {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = RockPaperScissors::new();
        let game_num = contract.new_game();
        let ingredient =
            HashIngredient {
                game_num,
                account: env::signer_account_id().clone(),
                shape: shape_of_account0,
                factor: "alice".to_string(),
            };
        
        assert!(contract.commit(game_num, ingredient.hash().clone()).is_ok());
        println!("{} commit!", env::signer_account_id());

        let context = get_context(accounts(1));
        testing_env!(context.build());
    
        let ingredient =
            HashIngredient {
                game_num,
                account: env::signer_account_id().clone(),
                shape: shape_of_account1,
                factor: "bob".to_string(),
            };
        assert!(contract.commit(game_num, ingredient.hash()).is_ok());
        println!("{} commit!", env::signer_account_id());

        let context = get_context(accounts(0));
        testing_env!(context.build());
        println!("{}", env::signer_account_id());

        assert!(contract.reveal(game_num, shape_of_account0.to_string(), "alice".to_string()).is_ok());

        println!("{} reveal! her choice is {}", env::signer_account_id(), shape_of_account0.to_string());

        let context = get_context(accounts(1));
        testing_env!(context.build());
        println!("{}", env::signer_account_id());

        assert!(contract.reveal(game_num, shape_of_account1.to_string(), "bob".to_string()).is_ok());

        println!("{} reveal! his choice is {}", env::signer_account_id(), shape_of_account1.to_string());

        let winner = contract.get_result(game_num).unwrap();
        if let Some(winner_id) = expected_winner {
            assert_eq!(winner.unwrap(), winner_id.to_string());
        } else {
            assert!(winner.is_none());
        }
    }

    #[test]
    fn normal() {
        normal_play(Shape::Paper, Shape::Rock, Some(accounts(0)));
        normal_play(Shape::Rock, Shape::Paper, Some(accounts(1)));

        normal_play(Shape::Rock, Shape::Scissors, Some(accounts(0)));
        normal_play(Shape::Scissors, Shape::Rock, Some(accounts(1)));

        normal_play(Shape::Scissors, Shape::Paper, Some(accounts(0)));
        normal_play(Shape::Paper, Shape::Scissors, Some(accounts(1)));

        normal_play(Shape::Paper, Shape::Paper, None);
        normal_play(Shape::Rock, Shape::Rock, None);
        normal_play(Shape::Scissors, Shape::Scissors, None);
    }

    #[test]
    fn enough_players() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = RockPaperScissors::new();
        let game_num = contract.new_game();
        let ingredient =
            HashIngredient {
                game_num,
                account: env::signer_account_id().clone(),
                shape: Shape::Rock,
                factor: "alice".to_string(),
            };
        
        assert!(contract.commit(game_num, ingredient.hash().clone()).is_ok());
        println!("{} commit!", env::signer_account_id());

        let context = get_context(accounts(1));
        testing_env!(context.build());
    
        let ingredient =
            HashIngredient {
                game_num,
                account: env::signer_account_id().clone(),
                shape: Shape::Scissors,
                factor: "bob".to_string(),
            };
        assert!(contract.commit(game_num, ingredient.hash()).is_ok());
        println!("{} commit!", env::signer_account_id());

        let context = get_context(accounts(2));
        testing_env!(context.build());

        let ingredient =
            HashIngredient {
                game_num,
                account: env::signer_account_id().clone(),
                shape: Shape::Paper,
                factor: "charlie".to_string(),
            };
        assert_eq!(contract.commit(game_num, ingredient.hash()), Err(GameError::EnoughPlayers));
    }

    #[test]
    fn not_time_to_reveal() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = RockPaperScissors::new();
        let game_num = contract.new_game();
        let ingredient =
            HashIngredient {
                game_num,
                account: env::signer_account_id().clone(),
                shape: Shape::Rock,
                factor: "alice".to_string(),
            };
        
        assert!(contract.commit(game_num, ingredient.hash().clone()).is_ok());
        println!("{} commit!", env::signer_account_id());

        let context = get_context(accounts(0));
        testing_env!(context.build());
        println!("{}", env::signer_account_id());

        assert_eq!(contract.reveal(game_num, Shape::Rock.to_string(), "alice".to_string()), Err(GameError::NotTimeToReveal));

        println!("{} reveal! her choice is {}", env::signer_account_id(), Shape::Rock.to_string());
    }

    #[test]
    fn player_not_match() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = RockPaperScissors::new();
        let game_num = contract.new_game();
        let ingredient =
            HashIngredient {
                game_num,
                account: env::signer_account_id().clone(),
                shape: Shape::Rock,
                factor: "alice".to_string(),
            };
        
        assert!(contract.commit(game_num, ingredient.hash().clone()).is_ok());
        println!("{} commit!", env::signer_account_id());

        let context = get_context(accounts(1));
        testing_env!(context.build());
    
        let ingredient =
            HashIngredient {
                game_num,
                account: env::signer_account_id().clone(),
                shape: Shape::Scissors,
                factor: "bob".to_string(),
            };
        assert!(contract.commit(game_num, ingredient.hash()).is_ok());
        println!("{} commit!", env::signer_account_id());

        let context = get_context(accounts(2));
        testing_env!(context.build());
        println!("{}", env::signer_account_id());

        assert_eq!(
            contract.reveal(game_num, Shape::Rock.to_string(), "alice".to_string()),
            Err(GameError::PlayersNotMatch));
    }

    #[test]
    fn reveal_not_match() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

        let mut contract = RockPaperScissors::new();
        let game_num = contract.new_game();
        let ingredient =
            HashIngredient {
                game_num,
                account: env::signer_account_id().clone(),
                shape: Shape::Rock,
                factor: "alice".to_string(),
            };
        
        assert!(contract.commit(game_num, ingredient.hash().clone()).is_ok());
        println!("{} commit!", env::signer_account_id());

        let context = get_context(accounts(1));
        testing_env!(context.build());
    
        let ingredient =
            HashIngredient {
                game_num,
                account: env::signer_account_id().clone(),
                shape: Shape::Scissors,
                factor: "bob".to_string(),
            };
        assert!(contract.commit(game_num, ingredient.hash()).is_ok());
        println!("{} commit!", env::signer_account_id());

        let context = get_context(accounts(0));
        testing_env!(context.build());
        println!("{}", env::signer_account_id());

        assert_eq!(
            contract.reveal(game_num, Shape::Rock.to_string(), "wrong".to_string()),
            Err(GameError::RevealNotMatch));
    }
}
