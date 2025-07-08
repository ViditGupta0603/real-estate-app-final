use candid::Principal;
use crate::storage::USER_STORAGE;
use crate::types::KycStatus;
use ic_cdk::api::time;

pub fn is_authenticated() -> Result<Principal, String> {
    let caller = ic_cdk::caller();
    if caller == Principal::anonymous() {
        Err("Authentication required".to_string())
    } else {
        Ok(caller)
    }
}

pub fn validate_kyc(user_principal: Principal) -> Result<(), String> {
    USER_STORAGE.with(|storage| {
        match storage.borrow().get(&user_principal) {
            Some(user) => match user.kyc_status {
                KycStatus::Verified => Ok(()),
                KycStatus::Pending => Err("KYC verification pending".to_string()),
                KycStatus::Rejected => Err("KYC verification rejected".to_string()),
                KycStatus::Expired => Err("KYC verification expired".to_string()),
            },
            None => Err("User profile not found".to_string()),
        }
    })
}

pub fn get_current_time() -> u64 {
    time()
}
