use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct Property {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub location: String,
    pub total_value: u64,
    pub total_tokens: u64,
    pub available_tokens: u64,
    pub price_per_token: u64,
    pub owner: Principal,
    pub created_at: u64,
    pub updated_at: u64,
    pub property_type: PropertyType,
    pub status: PropertyStatus,
    pub images: Vec<String>,
    pub documents: Vec<String>,
    pub rental_yield: f64,
    pub appreciation_rate: f64,
    pub property_highlights: Vec<String>,
    pub legal_structure: String,
    pub valuation_date: u64,
    pub next_dividend_date: u64,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum PropertyType {
    Residential,
    Commercial,
    Industrial,
    Land,
    Trophy,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum PropertyStatus {
    Active,
    Sold,
    Pending,
    Inactive,
    UnderMaintenance,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct TokenOrder {
    pub id: u64,
    pub property_id: u64,
    pub seller: Principal,
    pub buyer: Option<Principal>,
    pub token_amount: u64,
    pub price_per_token: u64,
    pub total_price: u64,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub created_at: u64,
    pub expires_at: u64,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    Active,
    Filled,
    Cancelled,
    Expired,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct DividendDistribution {
    pub id: u64,
    pub property_id: u64,
    pub total_amount: u64,
    pub per_token_amount: u64,
    pub distribution_date: u64,
    pub payment_status: PaymentStatus,
    pub recipients: Vec<DividendRecipient>,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct DividendRecipient {
    pub investor: Principal,
    pub token_amount: u64,
    pub dividend_amount: u64,
    pub paid: bool,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct Portfolio {
    pub owner: Principal,
    pub total_value: u64,
    pub total_tokens: u64,
    pub properties: Vec<PortfolioProperty>,
    pub total_dividends_received: u64,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct PortfolioProperty {
    pub property_id: u64,
    pub token_amount: u64,
    pub initial_investment: u64,
    pub current_value: u64,
    pub dividends_received: u64,
    pub purchase_date: u64,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub total_return: f64,
    pub annual_yield: f64,
    pub roi_percentage: f64,
    pub diversification_score: f64,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub id: u64,
    pub property_id: u64,
    pub proposer: Principal,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub voting_power_required: u64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub status: ProposalStatus,
    pub created_at: u64,
    pub voting_ends_at: u64,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum ProposalType {
    PropertyMaintenance,
    PropertySale,
    ManagementChange,
    DividendDistribution,
    Other,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct KycVerification {
    pub user: Principal,
    pub verification_level: KycLevel,
    pub documents_submitted: Vec<String>,
    pub verification_date: u64,
    pub expiry_date: u64,
    pub verified_by: String,
    pub compliance_score: u32,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum KycLevel {
    Basic,
    Standard,
    Premium,
    Institutional,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub principal: Principal,
    pub name: String,
    pub email: String,
    pub kyc_status: KycStatus,
    pub kyc_verification: Option<KycVerification>,
    pub created_at: u64,
    pub total_investments: u64,
    pub investment_limit: u64,
    pub accredited_investor: bool,
    pub jurisdiction: String,
    pub risk_profile: RiskProfile,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum KycStatus {
    Pending,
    Verified,
    Rejected,
    Expired,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum RiskProfile {
    Conservative,
    Moderate,
    Aggressive,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub property_id: u64,
    pub current_price: u64,
    pub price_change_24h: f64,
    pub trading_volume_24h: u64,
    pub market_cap: u64,
    pub liquidity_score: f64,
    pub last_updated: u64,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct CreatePropertyPayload {
    pub title: String,
    pub description: String,
    pub location: String,
    pub total_value: u64,
    pub total_tokens: u64,
    pub property_type: PropertyType,
    pub images: Vec<String>,
    pub documents: Vec<String>,
    pub rental_yield: f64,
    pub property_highlights: Vec<String>,
    pub legal_structure: String,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct CreateOrderPayload {
    pub property_id: u64,
    pub token_amount: u64,
    pub price_per_token: u64,
    pub order_type: OrderType,
    pub expires_in_hours: u64,
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct PlatformStats {
    pub total_properties: u64,
    pub total_investments: u64,
    pub total_users: u64,
    pub total_value_locked: u64,
    pub total_trading_volume: u64,
    pub active_orders: u64,
    pub total_dividends_paid: u64,
    pub platform_fee_collected: u64,
}
