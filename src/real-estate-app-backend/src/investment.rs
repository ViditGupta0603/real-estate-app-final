use candid::{CandidType, Principal};
use serde::{Serialize, Deserialize};
use ic_cdk_macros::*;
use crate::storage::{get_next_id, INVESTMENT_STORAGE, PROPERTY_STORAGE};
use crate::types::PropertyStatus;
use crate::utils::{get_current_time, is_authenticated, validate_kyc};

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct Investment {
    pub id: u64,
    pub investor: Principal,
    pub property_id: u64,
    pub token_amount: u64,
    pub investment_amount: u64,
    pub timestamp: u64,
    pub status: InvestmentStatus,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum InvestmentStatus {
    Pending,
    Confirmed,
    Cancelled,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct InvestmentPayload {
    pub property_id: u64,
    pub token_amount: u64,
}

#[update]
pub fn invest_in_property(payload: InvestmentPayload) -> Result<Investment, String> {
    let caller = is_authenticated()?;
    validate_kyc(caller)?;

    let property = PROPERTY_STORAGE.with(|storage| {
        storage.borrow().get(&payload.property_id)
            .ok_or_else(|| "Property not found".to_string())
    })?;

    if !matches!(property.status, PropertyStatus::Active) {
        return Err("Property is not available for investment".to_string());
    }

    if payload.token_amount > property.available_tokens {
        return Err("Insufficient tokens available".to_string());
    }

    let investment_amount = payload.token_amount * property.price_per_token;
    let investment_id = get_next_id();

    let investment = Investment {
        id: investment_id,
        investor: caller,
        property_id: payload.property_id,
        token_amount: payload.token_amount,
        investment_amount,
        timestamp: get_current_time(),
        status: InvestmentStatus::Confirmed,
    };

    INVESTMENT_STORAGE.with(|storage| {
        storage.borrow_mut().insert(investment_id, investment.clone())
    });

    Ok(investment)
}

#[query]
pub fn get_investments_by_user(user: Principal) -> Vec<Investment> {
    INVESTMENT_STORAGE.with(|storage| {
        storage.borrow()
            .iter()
            .filter(|(_, investment)| investment.investor == user)
            .map(|(_, investment)| investment)
            .collect()
    })
}
