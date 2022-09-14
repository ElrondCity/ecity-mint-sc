# Elrond City $ECITY minter SC

## Setup

These endpoint must be called just after deploying the contract in order:

- **episodeVestingPush**(amount: `BigUint`)  
  Adds a year to the vesting schedule, with _amount_ tokens minted every two weeks.
- **setRouter**(router: `ManagedAddress`)  
  Sets the address of the _router contract_, which the newly minted tokens will be sent to.
- **issueToken**(price: `BigUint`, token_name: `ManagedBuffer`, token_ticker: `ManagedBuffer`)  
  Creates the token on the blockchain, without minting any.

## Endpoints

### Public

- **mint**()
  Mints the correct amount of tokens according to the vesting schedule, and sends it to the router address. Can only be called once per episode.

### Private

- **episodeVestingPush**(amount: `BigUint`)  
- **setRouter**(router: `ManagedAddress`)  
- **issueToken**(price: `BigUint`, token_name: `ManagedBuffer`, token_ticker: `ManagedBuffer`)  
- **premint**(amount: `BigUint`, to: `ManagedAddress`)
  Premints the given quantity of the token and sends it to the given address. This also starts the vesting schedule. The endpoint can only be called once.
- **lockRouter**()
  Locks the current router address so that it cannot be changed anymore. Can only be used once and is irreversible. This is a safety measure for real trustlessness from the community.

## owner_interactions.sh

This script is meant to help you deploy, upgrade and use the contract. To use it, modify the OWNER_ADDRESS and PRIVATE_KEY fields with your erd address and the path to your pem file.

Assuming you are using Linux, type *source owner_interactions.sh* to be able to use its functions in your terminal.

Typing **deploy** will deploy the contract (by default on the devnet, change the PROXY field if needed). Copy its address and insert it in the ADDRESS field to be able to use the other functions.

Once the contract has been deployed, you can call its endpoints by typing their name, followed by potential arguments, in your terminal.

For example: `vestingPush 1000`

**Note:** For your convenience, numeric arguments are automatically multiplied by 10¹⁸, so 1 would represent 1eGld/token.
