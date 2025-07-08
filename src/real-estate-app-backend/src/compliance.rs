use ic_cdk_macros::*;
use crate::storage::USER_STORAGE;
use crate::types::*;
use crate::utils::{get_current_time, is_authenticated};

#[update]
pub fn submit_kyc_documents(documents: Vec<String>) -> Result<(), String> {
    let caller = is_authenticated()?;
    
    USER_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut user) = storage.get(&caller) {
            let kyc_verification = KycVerification {
                user: caller,
                verification_level: KycLevel::Basic,
                documents_submitted: documents,
                verification_date: get_current_time(),
                expiry_date: get_current_time() + (365 * 24 * 3600 * 1_000_000_000),
                verified_by: "System".to_string(),
                compliance_score: 50,
            };
            
            user.kyc_verification = Some(kyc_verification);
            user.kyc_status = KycStatus::Pending;
            storage.insert(caller, user);
            Ok(())
        } else {
            Err("User profile not found".to_string())
        }
    })
}
