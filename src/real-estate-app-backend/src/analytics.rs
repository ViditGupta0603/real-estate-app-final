use ic_cdk_macros::*;
use crate::storage::*;
use crate::types::PlatformStats;

#[query]
pub fn get_platform_analytics() -> PlatformStats {
    let total_properties = PROPERTY_STORAGE.with(|storage| storage.borrow().len());
    let total_investments = INVESTMENT_STORAGE.with(|storage| storage.borrow().len());
    let total_users = USER_STORAGE.with(|storage| storage.borrow().len());
    
    let total_value_locked = PROPERTY_STORAGE.with(|storage| {
        storage.borrow()
            .iter()
            .map(|(_, property)| property.total_value)
            .sum::<u64>()
    });

    PlatformStats {
        total_properties,
        total_investments,
        total_users,
        total_value_locked,
        total_trading_volume: 0,
        active_orders: 0,
        total_dividends_paid: 0,
        platform_fee_collected: 0,
    }
}
