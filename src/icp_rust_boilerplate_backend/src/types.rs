use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::VirtualMemory;
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, Storable};
use std::borrow::Cow;

pub type Memory = VirtualMemory<DefaultMemoryImpl>;
pub type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Business {
    pub id: u64,
    pub name: String,
    pub owner_principal: String,
    pub description: String,
    pub address: String,
    pub created_at: u64,
    pub updated_at: Option<u64>,
    pub products_ids: Vec<u64>,
}

impl Storable for Business {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Business {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Product {
    pub id: u64,
    pub business_id: u64,
    pub name: String,
    pub description: String,
    pub price: u64,
    pub created_at: u64,
    pub updated_at: Option<u64>,
}

impl Storable for Product {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Product {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct Order {
    pub id: u64,
    pub client_principal: String,
    pub products: Vec<Product>,
    pub total_price: u64,
    pub created_at: u64,
    pub updated_at: Option<u64>,
}

impl Storable for Order {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Order {
    const MAX_SIZE: u32 = 1024; // Adjust the maximum size as needed
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct BusinessPayload {
    pub name: String,
    pub description: String,
    pub address: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct ProductPayload {
    pub name: String,
    pub business_id: u64,
    pub description: String,
    pub price: u64,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct OrderPayload {
    pub product_ids: Vec<u64>,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum Error {
    NotFound { msg: String },
    InvalidPayload { errors: Vec<String> },
    NotBusinessOwner { msg: String },
    NotOrderClient { msg: String },
}
