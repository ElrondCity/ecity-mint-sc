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
