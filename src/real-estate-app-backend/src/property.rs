use ic_cdk_macros::*;
use crate::storage::{get_next_id, PROPERTY_STORAGE};
use crate::types::*;
use crate::utils::{get_current_time, is_authenticated, validate_kyc};

#[update]
pub fn create_property(payload: CreatePropertyPayload) -> Result<Property, String> {
    let caller = is_authenticated()?;
    validate_kyc(caller)?;

    let property_id = get_next_id();
    let current_time = get_current_time();
    let price_per_token = payload.total_value / payload.total_tokens;
    
    let property = Property {
        id: property_id,
        title: payload.title,
        description: payload.description,
        location: payload.location,
        total_value: payload.total_value,
        total_tokens: payload.total_tokens,
        available_tokens: payload.total_tokens,
        price_per_token,
        owner: caller,
        created_at: current_time,
        updated_at: current_time,
        property_type: payload.property_type,
        status: PropertyStatus::Active,
        images: payload.images,
        documents: payload.documents,
        rental_yield: payload.rental_yield,
        appreciation_rate: 0.0,
        property_highlights: payload.property_highlights,
        legal_structure: payload.legal_structure,
        valuation_date: current_time,
        next_dividend_date: current_time + (90 * 24 * 3600 * 1_000_000_000),
    };

    PROPERTY_STORAGE.with(|storage| {
        storage.borrow_mut().insert(property_id, property.clone())
    });

    Ok(property)
}

#[query]
pub fn get_all_properties() -> Vec<Property> {
    PROPERTY_STORAGE.with(|storage| {
        storage.borrow().iter().map(|(_, property)| property).collect()
    })
}
