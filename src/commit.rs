use super::shape::Shape;
use near_sdk::env;

pub type Commit = Vec<u8>;

#[derive(Clone)]
pub struct HashIngredient {
    pub game_num: u32,
    pub account: String,
    pub shape: Shape,
    pub factor: String,
}

impl HashIngredient {
    pub fn hash(&self) -> Vec<u8> {
        let mut hash_input = self.game_num.to_string();
        hash_input += &self.account;
        hash_input += &self.shape.to_string();
        hash_input += &self.factor.to_string();

        let hash_input = hash_input.as_bytes();
        env::keccak256(hash_input)
    }
    pub fn match_commit(&self, commit: &Commit) -> bool {
        self.hash() == *commit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn hash_works() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let ingredient =
            HashIngredient {
                game_num: 0,
                account: "alice.testnet".to_string(),
                shape: Shape::Paper,
                factor: "factor".to_string(),
            };

        let commit = ingredient.hash();

        assert_eq!(ingredient.match_commit(&commit), true);

        let wrong_game =
            HashIngredient {
                game_num: 1,
                ..ingredient.clone()
            };
        assert_eq!(wrong_game.match_commit(&commit), false);
        assert_ne!(wrong_game.hash(), commit);

        let wrong_factor =
            HashIngredient {
                factor: "xx".to_string(),
                ..ingredient.clone()
            };
        assert_eq!(wrong_factor.match_commit(&commit), false);
        assert_ne!(wrong_factor.hash(), commit);

        let wrong_account =
            HashIngredient{
                account: "bob.testnet".to_string(),
                ..ingredient.clone()
            };
        assert_eq!(wrong_account.match_commit(&commit), false);
        assert_ne!(wrong_account.hash(), commit);

        let wrong_shape =
            HashIngredient{
                shape: Shape::Rock,
                ..ingredient.clone()
            };
        assert_eq!(wrong_shape.match_commit(&commit), false);
        assert_ne!(wrong_shape.hash(), commit);
    }
}
