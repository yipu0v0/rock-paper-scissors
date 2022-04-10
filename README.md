# rock-paper-scissors
This smart contract implement *rock paper scissors* game on Near blockchain, which use a cryptographic primitive named [commitment](https://en.wikipedia.org/wiki/Commitment_scheme).

## Normal Process
Supposed that alice and bob want to play a game:
1. Someone create a new game, get game id(gid);
2. Alice use her private factor(such as hash of signature(gid, shape)) to pass her commitment to contract, which bind her choice to this commitment without showing it.
3. Bob do the same.
4. Now that both alice and bob cannot change his or her choice. Alice can reveal her choice(Shape);
5. Bob can also reveal his choice(He may choose not to reveal, so some punishment is needed to implemented later).
6. Now that Alice and Bob both reveal their choices, someone(maybe the winner) can run the get_result method if contract to show the winner!

## Test
`cargo test -- --nocapture`

## TODO
### Limit storage
1. Limit total game count
2. Limit the number of games a player can play at the same time

### Bet on something
It can be implemented to bet on:
1. Near, the native token of Near blockchain
2. NEP-141 token(fungible token standard of Near)
3. NEP-4 token(NFT standard of Near)

#### Time window
A time window mechanism is needed to avoid locking bet money permenantly.

### One game for more players
Currently the contract only support two-players schema. In reality, rock paper scissors game can be played by more than two people with multiple rounds.
