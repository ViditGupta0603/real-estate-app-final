use candid::Principal;

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

use ic_cdk_macros::*;
use types::*;

// Re-export key functions
pub use property::*;
pub use investment::*;
pub use user::*;
pub use analytics::*;
pub use compliance::*;

#[query]
pub fn get_enhanced_platform_stats() -> PlatformStats {
    get_platform_analytics()
}

// Generate the Candid interface at compile time
ic_cdk::export_candid!();
