use serde::{Serialize,Deserialize};

use std::fmt;

// newtypes

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub struct OrderId(pub u64);


#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash,Serialize,Deserialize)]
pub struct UserId(pub u64);


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Symbol(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Price(pub i64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Qty(pub u64);


// enums
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Limit,
    Market,
}


// error type
#[derive(Debug, Serialize, Deserialize)]
pub enum AppError {
    InvalidOrder,
    InsufficientFunds,
    OrderNotFound,
    InternalError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::InvalidOrder => write!(f, "Invalid order"),
            AppError::InsufficientFunds => write!(f, "Insufficient funds"),
            AppError::OrderNotFound => write!(f, "Order not found"),
            AppError::InternalError => write!(f, "Internal server error"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_id_equality() {
        assert_eq!(OrderId(1), OrderId(1));
    }

    #[test]
    fn different_types_not_equal() {
        let _order = OrderId(1);
        let _user = UserId(1);

        // This should NEVER compile if uncommented:
        // assert_eq!(_order, _user);
    }
}