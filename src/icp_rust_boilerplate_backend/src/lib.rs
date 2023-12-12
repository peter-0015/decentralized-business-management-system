#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Business {
    id: u64,
    name: String,
    description: String,
    address: String,
    created_at: u64,
    updated_at: Option<u64>,
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
struct Product {
    id: u64,
    name: String,
    description: String,
    price: u64,
    created_at: u64,
    updated_at: Option<u64>,
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
struct Order {
    id: u64,
    products: Vec<Product>,
    total_price: u64,
    created_at: u64,
    updated_at: Option<u64>,
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
struct BusinessPayload {
    name: String,
    description: String,
    address: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct ProductPayload {
    name: String,
    description: String,
    price: u64,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct OrderPayload {
    product_ids: Vec<u64>,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static BUSINESS_STORAGE: RefCell<StableBTreeMap<u64, Business, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static PRODUCT_STORAGE: RefCell<StableBTreeMap<u64, Product, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static ORDER_STORAGE: RefCell<StableBTreeMap<u64, Order, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
}

#[ic_cdk::query]
fn get_business(id: u64) -> Result<Business, Error> {
    match _get_business(&id) {
        Some(business) => Ok(business),
        None => Err(Error::NotFound {
            msg: format!("a business with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn add_business(business: BusinessPayload) -> Option<Business> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let business = Business {
        id,
        name: business.name,
        description: business.description,
        address: business.address,
        created_at: time(),
        updated_at: None,
    };
    do_insert_business(&business);
    Some(business)
}

#[ic_cdk::update]
fn update_business(id: u64, payload: BusinessPayload) -> Result<Business, Error> {
    match BUSINESS_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut business) => {
            business.name = payload.name;
            business.description = payload.description;
            business.address = payload.address;
            business.updated_at = Some(time());
            do_insert_business(&business);
            Ok(business)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a business with id={}. business not found",
                id
            ),
        }),
    }
}

fn do_insert_business(business: &Business) {
    BUSINESS_STORAGE
        .with(|service| service.borrow_mut().insert(business.id, business.clone()));
}

#[ic_cdk::update]
fn delete_business(id: u64) -> Result<Business, Error> {
    match BUSINESS_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(business) => Ok(business),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a business with id={}. business not found.",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn get_product(id: u64) -> Result<Product, Error> {
    match _get_product(&id) {
        Some(product) => Ok(product),
        None => Err(Error::NotFound {
            msg: format!("a product with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn add_product(product: ProductPayload) -> Option<Product> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let product = Product {
        id,
        name: product.name,
        description: product.description,
        price: product.price,
        created_at: time(),
        updated_at: None,
    };
    do_insert_product(&product);
    Some(product)
}

#[ic_cdk::update]
fn update_product(id: u64, payload: ProductPayload) -> Result<Product, Error> {
    match PRODUCT_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut product) => {
            product.name = payload.name;
            product.description = payload.description;
            product.price = payload.price;
            product.updated_at = Some(time());
            do_insert_product(&product);
            Ok(product)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a product with id={}. product not found",
                id
            ),
        }),
    }
}

fn do_insert_product(product: &Product) {
    PRODUCT_STORAGE
        .with(|service| service.borrow_mut().insert(product.id, product.clone()));
}

#[ic_cdk::update]
fn delete_product(id: u64) -> Result<Product, Error> {
    match PRODUCT_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(product) => Ok(product),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a product with id={}. product not found.",
                id
            ),
        }),
    }
}

#[ic_cdk::query]
fn get_order(id: u64) -> Result<Order, Error> {
    match _get_order(&id) {
        Some(order) => Ok(order),
        None => Err(Error::NotFound {
            msg: format!("an order with id={} not found", id),
        }),
    }
}

#[ic_cdk::update]
fn create_order(order_payload: OrderPayload) -> Result<Order, Error> {
    let order_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let mut order_products = Vec::new();
    let mut total_price = 0;

    for product_id in order_payload.product_ids.iter() {
        match _get_product(product_id) {
            Some(product) => {
                order_products.push(product.clone());
                total_price += product.price;
            }
            None => {
                return Err(Error::NotFound {
                    msg: format!("product with id={} not found", product_id),
                });
            }
        }
    }

    let order = Order {
        id: order_id,
        products: order_products,
        total_price,
        created_at: time(),
        updated_at: None,
    };

    do_insert_order(&order);
    Ok(order)
}

#[ic_cdk::update]
fn update_order(id: u64, order_payload: OrderPayload) -> Result<Order, Error> {
    match ORDER_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut order) => {
            let mut new_products = Vec::new();
            let mut new_total_price = 0;

            for product_id in order_payload.product_ids.iter() {
                match _get_product(product_id) {
                    Some(product) => {
                        new_products.push(product.clone());
                        new_total_price += product.price;
                    }
                    None => {
                        return Err(Error::NotFound {
                            msg: format!("product with id={} not found", product_id),
                        });
                    }
                }
            }

            order.products = new_products;
            order.total_price = new_total_price;
            order.updated_at = Some(time());
            do_insert_order(&order);
            Ok(order)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update an order with id={}. order not found", id),
        }),
    }
}

#[ic_cdk::update]
fn delete_order(id: u64) -> Result<Order, Error> {
    match ORDER_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(order) => Ok(order),
        None => Err(Error::NotFound {
            msg: format!("couldn't delete an order with id={}. order not found", id),
        }),
    }
}

fn do_insert_order(order: &Order) {
    ORDER_STORAGE
        .with(|service| service.borrow_mut().insert(order.id, order.clone()));
}

fn _get_business(id: &u64) -> Option<Business> {
    BUSINESS_STORAGE.with(|service| service.borrow().get(id))
}

fn _get_product(id: &u64) -> Option<Product> {
    PRODUCT_STORAGE.with(|service| service.borrow().get(id))
}

fn _get_order(id: &u64) -> Option<Order> {
    ORDER_STORAGE.with(|service| service.borrow().get(id))
}

ic_cdk::export_candid!();
