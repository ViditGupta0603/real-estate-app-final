use crate::investment::Investment;
use crate::types::*;
use candid::{Decode, Encode, Principal};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, Storable};
use ic_stable_structures::storable::Bound;
use std::borrow::Cow;
use std::cell::RefCell;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdStore = StableBTreeMap<u64, u64, Memory>;
type PropertyStore = StableBTreeMap<u64, Property, Memory>;
type InvestmentStore = StableBTreeMap<u64, Investment, Memory>;
type UserStore = StableBTreeMap<Principal, UserProfile, Memory>;
type OrderStore = StableBTreeMap<u64, TokenOrder, Memory>;
type DividendStore = StableBTreeMap<u64, DividendDistribution, Memory>;
type PortfolioStore = StableBTreeMap<Principal, Portfolio, Memory>;
type ProposalStore = StableBTreeMap<u64, GovernanceProposal, Memory>;
type MarketDataStore = StableBTreeMap<u64, MarketData, Memory>;

// Implement Storable for all types
impl Storable for Property {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for Investment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for UserProfile {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for TokenOrder {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for DividendDistribution {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for Portfolio {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for GovernanceProposal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for MarketData {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdStore> = RefCell::new(
        IdStore::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))))
    );

    pub static PROPERTY_STORAGE: RefCell<PropertyStore> = RefCell::new(
        PropertyStore::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    pub static INVESTMENT_STORAGE: RefCell<InvestmentStore> = RefCell::new(
        InvestmentStore::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))))
    );

    pub static USER_STORAGE: RefCell<UserStore> = RefCell::new(
        UserStore::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    pub static ORDER_STORAGE: RefCell<OrderStore> = RefCell::new(
        OrderStore::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    pub static DIVIDEND_STORAGE: RefCell<DividendStore> = RefCell::new(
        DividendStore::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );

    pub static PORTFOLIO_STORAGE: RefCell<PortfolioStore> = RefCell::new(
        PortfolioStore::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))))
    );

    pub static PROPOSAL_STORAGE: RefCell<ProposalStore> = RefCell::new(
        ProposalStore::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7))))
    );

    pub static MARKET_DATA_STORAGE: RefCell<MarketDataStore> = RefCell::new(
        MarketDataStore::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(8))))
    );
}

pub fn get_next_id() -> u64 {
    ID_COUNTER.with(|counter| {
        let current_value = counter.borrow().get(&0).unwrap_or(0);
        let next_value = current_value + 1;
        counter.borrow_mut().insert(0, next_value);
        next_value
    })
}
