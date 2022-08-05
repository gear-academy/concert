#![no_std]

use codec::{Decode, Encode};
use gear_lib::multitoken::io::*;
use gstd::{prelude::*, ActorId};
use scale_info::TypeInfo;

// Concert related stuff
#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ConcertAction {
    Create {
        creator: ActorId,
        name: String,
        description: String,
        number_of_tickets: u128,
        date: u128,
    },
    Hold {},
    BuyTickets {
        amount: u128,
        metadata: Vec<Option<TokenMetadata>>,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ConcertEvent {
    Creation {
        creator: ActorId,
        concert_id: u128,
        number_of_tickets: u128,
        date: u128,
    },
    Hold {
        concert_id: u128,
    },
    Purchase {
        concert_id: u128,
        amount: u128,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ConcertStateQuery {
    CurrentConcert,
    Buyers,
    UserTickets { user: ActorId },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ConcertStateReply {
    CurrentConcert {
        name: String,
        description: String,
        date: u128,
        number_of_tickets: u128,
        tickets_left: u128,
    },
    Buyers {
        accounts: BTreeSet<ActorId>,
    },
    UserTickets {
        tickets: Vec<Option<TokenMetadata>>,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub struct InitConcert {
    pub owner_id: ActorId,
    pub mtk_contract: ActorId,
}
