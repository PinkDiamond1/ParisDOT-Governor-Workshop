#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod governor {
    use ink_storage::Mapping;
    use scale::{Encode, Decode};
    use ink_storage::traits::*;

    pub const ONE_MINUTE: u64 = 60 * 1000;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum GovernorError {
        AmountShouldNotBeZero,
        DurationError
    }

    #[derive(Encode, Decode, SpreadLayout, PackedLayout, SpreadAllocate, Default)]
    #[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout))]
    pub struct Proposal {
        to: AccountId,
        amount: Balance,
        vote_start: Timestamp,
        vote_end: Timestamp,
        executed: bool,
    }

    #[derive(Encode, Decode, SpreadLayout, PackedLayout, SpreadAllocate, Default)]
    #[cfg_attr(feature = "std", derive(Debug, PartialEq, Eq, scale_info::TypeInfo, StorageLayout))]
    pub struct ProposalVote {
        against_votes: u32,
        for_votes: u32,
    }

    pub type ProposalId = u32;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Governor {
        proposal_votes: Mapping<ProposalId, ProposalVote>,
        proposals: Mapping<ProposalId, Proposal>,
        votes: Mapping<(ProposalId, AccountId), bool>,
        next_proposal_id: u32,
    }

    impl Governor {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|_| {})
        }

        #[ink(message)]
        pub fn propose(&mut self, to: AccountId, amount: Balance, duration: u64) -> Result<(), GovernorError> {
            if amount == 0 {
                return Err(GovernorError::AmountShouldNotBeZero)
            }
            if duration == 0 || duration > 60 * ONE_MINUTE {
                return Err(GovernorError::DurationError)
            }

            let now = self.env().block_timestamp();
            let proposal = Proposal {
                to,
                amount,
                vote_start: now,
                vote_end: now + duration * ONE_MINUTE,
                executed: false
            };

            let id = self.next_proposal_id();
            self.proposals.insert(id, &proposal);

            Ok(())
        }

        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Proposal {
            self.proposals.get(&proposal_id).unwrap_or_default()
        }

        fn next_proposal_id(&mut self) -> ProposalId {
            let id = self.next_proposal_id;
            self.next_proposal_id += 1;
            id
        }
    }
}