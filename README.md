# rock-paper-scissors
This smart contract implement *rock paper scissors* game on Near blockchain, which use a cryptographic primitive named [commitment](https://en.wikipedia.org/wiki/Commitment_scheme).

# TODO
## Support bet on something
It can be implemented to bet on:
1. Near, the native token of Near blockchain
2. NEP-141 token(fungible token standard of Near)
3. NEP-4 token(NFT standard of Near)

After supporting bet, a time window mechanism should be there to punish players who don't reveal his or her choice. Such as proportionaly give the bet of those players to honest players.

## Support multiple one game for more players
Currently the contract only support two-players scheme. In reality, rock paper scissors game can be played by more than 2 people with multiple rounds.
