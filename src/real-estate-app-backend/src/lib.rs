use candid::Principal;
use ic_cdk_macros::*;

mod types;
mod storage;
mod utils;
mod property;
mod investment;
mod user;
mod marketplace;
mod governance;
mod analytics;
mod compliance;
use types::*;

// Explicitly re-export all module functions
pub use property::*;
pub use investment::*;
pub use user::*;
pub use marketplace::*;
pub use governance::*;
pub use analytics::*;
pub use compliance::*;

// Manual function exports to ensure visibility
#[update]
pub fn create_property_wrapper(payload: CreatePropertyPayload) -> Result<Property, String> {
    property::create_property(payload)
}

#[query]
pub fn get_all_properties_wrapper() -> Vec<Property> {
    property::get_all_properties()
}

#[update]
pub fn invest_in_property_wrapper(payload: InvestmentPayload) -> Result<Investment, String> {
    investment::invest_in_property(payload)
}

#[query]
pub fn get_investments_by_user_wrapper(user: Principal) -> Vec<Investment> {
    investment::get_investments_by_user(user)
}

#[update]
pub fn create_user_profile_wrapper(name: String, email: String) -> Result<UserProfile, String> {
    user::create_user_profile(name, email)
}

#[query]
pub fn get_user_profile_wrapper(user: Principal) -> Result<UserProfile, String> {
    user::get_user_profile(user)
}

#[update]
pub fn create_token_order_wrapper(payload: CreateOrderPayload) -> Result<TokenOrder, String> {
    marketplace::create_token_order(payload)
}

#[update]
pub fn execute_order_wrapper(order_id: u64) -> Result<TokenOrder, String> {
    marketplace::execute_order(order_id)
}

#[query]
pub fn get_active_orders_wrapper(property_id: u64) -> Vec<TokenOrder> {
    marketplace::get_active_orders(property_id)
}

#[query]
pub fn get_user_orders_wrapper(user: Principal) -> Vec<TokenOrder> {
    marketplace::get_user_orders(user)
}

#[update]
pub fn create_proposal_wrapper(
    property_id: u64,
    title: String,
    description: String,
    proposal_type: ProposalType,
    voting_duration_hours: u64,
) -> Result<GovernanceProposal, String> {
    governance::create_proposal(property_id, title, description, proposal_type, voting_duration_hours)
}

#[update]
pub fn vote_on_proposal_wrapper(proposal_id: u64, vote_for: bool) -> Result<(), String> {
    governance::vote_on_proposal(proposal_id, vote_for)
}

#[query]
pub fn get_property_proposals_wrapper(property_id: u64) -> Vec<GovernanceProposal> {
    governance::get_property_proposals(property_id)
}

#[update]
pub fn submit_kyc_documents_wrapper(documents: Vec<String>) -> Result<(), String> {
    compliance::submit_kyc_documents(documents)
}

#[query]
pub fn get_enhanced_platform_stats() -> PlatformStats {
    analytics::get_platform_analytics()
}

// Generate complete Candid interface
ic_cdk::export_candid!();
