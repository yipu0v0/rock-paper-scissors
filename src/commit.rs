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
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, MockedBlockchain};
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
    fn hash_works() {
        let context = get_context(accounts(0));
        testing_env!(context.build());

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
