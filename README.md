# rock-paper-scissors
This smart contract implement *rock paper scissors* game on Near blockchain, which use a cryptographic primitive named [commitment](https://en.wikipedia.org/wiki/Commitment_scheme).

## Normal Process


## TODO
### Support bet on something
It can be implemented to bet on:
1. Near, the native token of Near blockchain
2. NEP-141 token(fungible token standard of Near)
3. NEP-4 token(NFT standard of Near)

A time window mechanism is needed to avoid locking bet money permenantly.

### Support one game for more players
Currently the contract only support two-players schema. In reality, rock paper scissors game can be played by more than 2 people with multiple rounds.
