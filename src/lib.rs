#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use core::convert::TryInto;

/// The Minter contract for the ECITY token.
#[elrond_wasm::contract]
pub trait EcityTest: elrond_wasm_modules::default_issue_callbacks::DefaultIssueCallbacksModule {
    #[init]
    fn init(&self) {
        self.preminted().set_if_empty(false);
        self.odd_episode_minted().set_if_empty(false);
        self.even_episode_minted().set_if_empty(false);
        self.router_locked().set_if_empty(false);
    }

    // Storage and views

    #[view(token)]
    #[storage_mapper("token")]
    fn token(&self) -> FungibleTokenMapper<Self::Api>;

    #[view(episodeVesting)]
    #[storage_mapper("episodeVesting")]
    fn episode_vesting(&self) -> VecMapper<BigUint<Self::Api>>; // Stores the number of token to be minted per episode for each year. The index represents the years elapsed after the token release.
                                                                // (i.e. episodeVesting[1] = the number of tokens to be minted per episode during the first year)
                                        
    #[view(evenEpisodeMinted)]
    #[storage_mapper("evenEpisodeMinted")] // Stores whether or not the current episode has already been minted, if its number is even
    fn even_episode_minted(&self) -> SingleValueMapper<Self::Api, bool>;

    #[view(oddEpisodeMinted)]
    #[storage_mapper("oddEpisodeMinted")] // Stores whether or not the current episode has already been minted, if its number is odd
    fn odd_episode_minted(&self) -> SingleValueMapper<Self::Api, bool>;

    #[view(routerContract)]
    #[storage_mapper("routerContract")]
    fn router_contract(&self) -> SingleValueMapper<ManagedAddress<Self::Api>>; // Stores the address of the router contract, which will receive the newly minted tokens to distribute them according to the WP

    #[view(routerLocked)]
    #[storage_mapper("routerLocked")] // Might be removed if we can only set the router once. Food for thought.
    fn router_locked(&self) -> SingleValueMapper<Self::Api, bool>; // Security for us to lock the router contract address forever, with no way of changing it. 

    #[view(preminted)]
    #[storage_mapper("preminted")]
    fn preminted(&self) -> SingleValueMapper<Self::Api, bool>; // Stores whether or not the premint already happened. The vesting schedule starts right after the premint.

    #[view(vestingStart)]
    #[storage_mapper("vestingStart")] // The timestamp of the premint, which also represents the start of the vesting schedule.
    fn vesting_start(&self) -> SingleValueMapper<u64>;

    #[view(episode)] // returns the current episode number
    fn episode(&self) -> u64 {
        let curr_time = self.blockchain().get_block_timestamp();
        let episode_length = 14 * 24 * 60 * 60; // The length of an episode, in seconds (2 weeks)
        let elapsed_time = curr_time - self.vesting_start().get();
        let episode_number = elapsed_time / episode_length + 1;
        return episode_number;
    }
    
    // Only owner
    
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueToken)]
    fn issue_token(
        &self,
        issue_cost: BigUint, //Should be 5000000000000000 (0.05EGLD), but kept as argument for safety
        token_name: ManagedBuffer,
        token_ticker: ManagedBuffer
    ) {
        self.token().issue_and_set_all_roles(issue_cost, token_name, token_ticker, 18, None);
    }

    #[only_owner]
    #[endpoint(premint)]
    fn premint(
        &self,
        amount: BigUint,
        to: ManagedAddress
    ) {
        require!(!self.preminted().get(), "Already preminted");

        self.preminted().set(true);
        self.vesting_start().set(self.blockchain().get_block_timestamp());
        self.token().mint_and_send(&to, amount);
    }

    #[only_owner]
    #[endpoint(episodeVestingPush)] // Allows one to add a year to the vesting schedule, during which "amount" will be minted at each episode.
    fn episode_vesting_push(
        &self,
        amount: BigUint
    ) {
        require!(!self.preminted().get(), "Vesting already started");
        self.episode_vesting().push(&amount);
    }

    #[only_owner]
    #[endpoint(setRouter)]
    fn set_router(
        &self,
        router: ManagedAddress
    ) {
        require!(!self.router_locked().get(), "Router locked");
        self.router_contract().set(router);
    }

    #[only_owner]
    #[endpoint(lockRouter)]
    fn lock_router(&self)
    {
        self.router_locked().set(true);
    }

    // Public endpoints

    #[endpoint(mint)]
    fn mint(
        &self
    ) {
        require!(self.preminted().get(), "Not preminted yet.");

        let curr_time = self.blockchain().get_block_timestamp();
        let episode_length = 14 * 24 * 60 * 60; // The length of an episode, in seconds (2 weeks)
        let elapsed_time = curr_time - self.vesting_start().get();
        let episode_number = elapsed_time / episode_length; // +1

        require!(episode_number / 26 < self.episode_vesting().len().try_into().unwrap(), "Max supply reached"); // There are 26 episodes in a year, the length of episode_vesting is the number of years of the vesting schedule

        if episode_number % 2 == 0 {
            require!(!self.even_episode_minted().get(), "Episode already minted");
            self.even_episode_minted().set(true);
            self.odd_episode_minted().set(false);
        } else {
            require!(!self.odd_episode_minted().get(), "Episode already minted");
            self.odd_episode_minted().set(true);
            self.even_episode_minted().set(false);
        }

        let to_mint = self.episode_vesting().get((episode_number / 26 + 1).try_into().unwrap());

        self.token().mint_and_send(&self.router_contract().get(), to_mint);

    }
    
}
