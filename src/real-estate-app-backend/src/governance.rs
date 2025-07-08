use crate::storage::*;
use crate::types::*;
use crate::utils::*;
use candid::Principal;
use ic_cdk::api::time;
use ic_cdk_macros::*;

#[update]
pub fn create_proposal(
    property_id: u64,
    title: String,
    description: String,
    proposal_type: ProposalType,
    voting_duration_hours: u64,
) -> Result<GovernanceProposal, String> {
    let caller = is_authenticated()?;
    validate_kyc(caller)?;

    // Verify caller has tokens in the property
    let user_tokens = get_user_token_balance(caller, property_id)?;
    if user_tokens == 0 {
        return Err("Must own tokens to create proposals".to_string());
    }

    let proposal_id = get_next_id();
    let current_time = time();
    let voting_ends_at = current_time + (voting_duration_hours * 3600 * 1_000_000_000);

    let proposal = GovernanceProposal {
        id: proposal_id,
        property_id,
        proposer: caller,
        title,
        description,
        proposal_type,
        voting_power_required: calculate_required_voting_power(property_id)?,
        votes_for: 0,
        votes_against: 0,
        status: ProposalStatus::Active,
        created_at: current_time,
        voting_ends_at,
    };

    PROPOSAL_STORAGE.with(|storage| {
        storage.borrow_mut().insert(proposal_id, proposal.clone())
    });

    Ok(proposal)
}

#[update]
pub fn vote_on_proposal(proposal_id: u64, vote_for: bool) -> Result<(), String> {
    let caller = is_authenticated()?;
    validate_kyc(caller)?;

    let mut proposal = PROPOSAL_STORAGE.with(|storage| {
        storage.borrow().get(&proposal_id)
            .ok_or_else(|| "Proposal not found".to_string())
    })?;

    if !matches!(proposal.status, ProposalStatus::Active) {
        return Err("Proposal is not active".to_string());
    }

    if proposal.voting_ends_at < time() {
        proposal.status = ProposalStatus::Rejected;
        PROPOSAL_STORAGE.with(|storage| {
            storage.borrow_mut().insert(proposal_id, proposal)
        });
        return Err("Voting period has ended".to_string());
    }

    let voting_power = get_user_token_balance(caller, proposal.property_id)?;
    if voting_power == 0 {
        return Err("No voting power for this property".to_string());
    }

    if vote_for {
        proposal.votes_for += voting_power;
    } else {
        proposal.votes_against += voting_power;
    }

    // Check if proposal passes
    if proposal.votes_for >= proposal.voting_power_required {
        proposal.status = ProposalStatus::Passed;
    }

    PROPOSAL_STORAGE.with(|storage| {
        storage.borrow_mut().insert(proposal_id, proposal)
    });

    Ok(())
}

#[query]
pub fn get_property_proposals(property_id: u64) -> Vec<GovernanceProposal> {
    PROPOSAL_STORAGE.with(|storage| {
        storage.borrow()
            .iter()
            .filter(|(_, proposal)| proposal.property_id == property_id)
            .map(|(_, proposal)| proposal)
            .collect()
    })
}

fn calculate_required_voting_power(property_id: u64) -> Result<u64, String> {
    let property = PROPERTY_STORAGE.with(|storage| {
        storage.borrow().get(&property_id)
            .ok_or_else(|| "Property not found".to_string())
    })?;

    // Require majority (51%) of total tokens
    Ok((property.total_tokens * 51) / 100)
}

fn get_user_token_balance(user: Principal, property_id: u64) -> Result<u64, String> {
    INVESTMENT_STORAGE.with(|storage| {
        let balance = storage.borrow()
            .iter()
            .filter(|(_, investment)| investment.investor == user && investment.property_id == property_id)
            .map(|(_, investment)| investment.token_amount)
            .sum();
        Ok(balance)
    })
}
