use ic_cdk_macros::*;
use candid::Principal;
use crate::storage::USER_STORAGE;
use crate::types::*;
use crate::utils::{get_current_time, is_authenticated};

#[update]
pub fn create_user_profile(name: String, email: String) -> Result<UserProfile, String> {
    let caller = is_authenticated()?;

    if USER_STORAGE.with(|storage| storage.borrow().contains_key(&caller)) {
        return Err("User profile already exists".to_string());
    }

    let user_profile = UserProfile {
        principal: caller,
        name,
        email,
        kyc_status: KycStatus::Pending,
        kyc_verification: None,
        created_at: get_current_time(),
        total_investments: 0,
        investment_limit: 100_000,
        accredited_investor: false,
        jurisdiction: "Unknown".to_string(),
        risk_profile: RiskProfile::Conservative,
    };

    USER_STORAGE.with(|storage| {
        storage.borrow_mut().insert(caller, user_profile.clone())
    });

    Ok(user_profile)
}

#[query]
pub fn get_user_profile(user: Principal) -> Result<UserProfile, String> {
    USER_STORAGE.with(|storage| {
        storage.borrow().get(&user)
            .ok_or_else(|| "User profile not found".to_string())
    })
}
