use crate::investment::{Investment, InvestmentStatus};
use crate::storage::*;
use crate::types::*;
use crate::utils::*;
use candid::Principal;
use ic_cdk::api::time;
use ic_cdk_macros::*;

#[update]
pub fn create_token_order(payload: CreateOrderPayload) -> Result<TokenOrder, String> {
    let caller = is_authenticated()?;
    validate_kyc(caller)?;

    // Verify property exists
    let _property = PROPERTY_STORAGE.with(|storage| {
        storage.borrow().get(&payload.property_id)
            .ok_or_else(|| "Property not found".to_string())
    })?;

    // For sell orders, verify user has enough tokens
    if matches!(payload.order_type, OrderType::Sell) {
        let user_tokens = get_user_token_balance(caller, payload.property_id)?;
        if user_tokens < payload.token_amount {
            return Err("Insufficient token balance".to_string());
        }
    }

    let order_id = get_next_id();
    let current_time = time();
    let expires_at = current_time + (payload.expires_in_hours * 3600 * 1_000_000_000);

    let order = TokenOrder {
        id: order_id,
        property_id: payload.property_id,
        seller: caller,
        buyer: None,
        token_amount: payload.token_amount,
        price_per_token: payload.price_per_token,
        total_price: payload.token_amount * payload.price_per_token,
        order_type: payload.order_type,
        status: OrderStatus::Active,
        created_at: current_time,
        expires_at,
    };

    ORDER_STORAGE.with(|storage| {
        storage.borrow_mut().insert(order_id, order.clone())
    });

    Ok(order)
}

#[update]
pub fn execute_order(order_id: u64) -> Result<TokenOrder, String> {
    let caller = is_authenticated()?;
    validate_kyc(caller)?;

    let mut order = ORDER_STORAGE.with(|storage| {
        storage.borrow().get(&order_id)
            .ok_or_else(|| "Order not found".to_string())
    })?;

    if !matches!(order.status, OrderStatus::Active) {
        return Err("Order is not active".to_string());
    }

    if order.expires_at < time() {
        order.status = OrderStatus::Expired;
        ORDER_STORAGE.with(|storage| {
            storage.borrow_mut().insert(order_id, order.clone())
        });
        return Err("Order has expired".to_string());
    }

    // Execute the trade
    match order.order_type {
        OrderType::Buy => {
            // Caller is selling to the buy order
            let seller_tokens = get_user_token_balance(caller, order.property_id)?;
            if seller_tokens < order.token_amount {
                return Err("Insufficient tokens to sell".to_string());
            }
            
            // Transfer tokens from seller to buyer
            transfer_tokens(caller, order.seller, order.property_id, order.token_amount)?;
            order.buyer = Some(caller);
        }
        OrderType::Sell => {
            // Caller is buying from the sell order
            // Transfer tokens from seller to buyer
            transfer_tokens(order.seller, caller, order.property_id, order.token_amount)?;
            order.buyer = Some(caller);
        }
    }

    order.status = OrderStatus::Filled;
    ORDER_STORAGE.with(|storage| {
        storage.borrow_mut().insert(order_id, order.clone())
    });

    // Update market data
    update_market_data(order.property_id, order.price_per_token, order.token_amount)?;

    Ok(order)
}

#[query]
pub fn get_active_orders(property_id: u64) -> Vec<TokenOrder> {
    ORDER_STORAGE.with(|storage| {
        storage.borrow()
            .iter()
            .filter(|(_, order)| {
                order.property_id == property_id && 
                matches!(order.status, OrderStatus::Active) &&
                order.expires_at > time()
            })
            .map(|(_, order)| order)
            .collect()
    })
}

#[query]
pub fn get_user_orders(user: Principal) -> Vec<TokenOrder> {
    ORDER_STORAGE.with(|storage| {
        storage.borrow()
            .iter()
            .filter(|(_, order)| order.seller == user || order.buyer == Some(user))
            .map(|(_, order)| order)
            .collect()
    })
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

fn transfer_tokens(from: Principal, to: Principal, property_id: u64, amount: u64) -> Result<(), String> {
    // Create investment record for the buyer
    let investment_id = get_next_id();
    let investment = Investment {
        id: investment_id,
        investor: to,
        property_id,
        token_amount: amount,
        investment_amount: 0, // Will be calculated based on order price
        timestamp: time(),
        status: InvestmentStatus::Confirmed,
    };

    INVESTMENT_STORAGE.with(|storage| {
        storage.borrow_mut().insert(investment_id, investment)
    });

    // Update portfolios
    update_portfolio_after_trade(from, to, property_id, amount)?;

    Ok(())
}

fn update_market_data(property_id: u64, price: u64, volume: u64) -> Result<(), String> {
    let current_time = time();
    
    let market_data = MARKET_DATA_STORAGE.with(|storage| {
        storage.borrow().get(&property_id)
    }).unwrap_or_else(|| MarketData {
        property_id,
        current_price: price,
        price_change_24h: 0.0,
        trading_volume_24h: 0,
        market_cap: 0,
        liquidity_score: 0.0,
        last_updated: current_time,
    });

    let mut updated_data = market_data;
    updated_data.current_price = price;
    updated_data.trading_volume_24h += volume;
    updated_data.last_updated = current_time;

    MARKET_DATA_STORAGE.with(|storage| {
        storage.borrow_mut().insert(property_id, updated_data)
    });

    Ok(())
}

fn update_portfolio_after_trade(from: Principal, to: Principal, property_id: u64, amount: u64) -> Result<(), String> {
    // Update seller's portfolio
    PORTFOLIO_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if let Some(mut portfolio) = storage.get(&from) {
            if let Some(prop) = portfolio.properties.iter_mut().find(|p| p.property_id == property_id) {
                prop.token_amount = prop.token_amount.saturating_sub(amount);
            }
            storage.insert(from, portfolio);
        }
    });

    // Update buyer's portfolio
    PORTFOLIO_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        let mut portfolio = storage.get(&to).unwrap_or_else(|| Portfolio {
            owner: to,
            total_value: 0,
            total_tokens: 0,
            properties: Vec::new(),
            total_dividends_received: 0,
            performance_metrics: PerformanceMetrics {
                total_return: 0.0,
                annual_yield: 0.0,
                roi_percentage: 0.0,
                diversification_score: 0.0,
            },
        });

        if let Some(prop) = portfolio.properties.iter_mut().find(|p| p.property_id == property_id) {
            prop.token_amount += amount;
        } else {
            portfolio.properties.push(PortfolioProperty {
                property_id,
                token_amount: amount,
                initial_investment: 0,
                current_value: 0,
                dividends_received: 0,
                purchase_date: time(),
            });
        }

        storage.insert(to, portfolio);
    });

    Ok(())
}
