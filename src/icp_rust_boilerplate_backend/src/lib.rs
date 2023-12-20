#[macro_use]
extern crate serde;
use ic_cdk::api::time;
use ic_cdk::caller;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;


mod types;
use types::*;
mod helpers;
use helpers::*;

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

// Function that users to fetch a business
#[ic_cdk::query]
fn get_business(id: u64) -> Result<Business, Error> {
    match _get_business(&id) {
        Some(business) => Ok(business),
        None => Err(Error::NotFound {
            msg: format!("a business with id={} not found", id),
        }),
    }
}
// Function that allows users to create a business
#[ic_cdk::update]
fn add_business(business: BusinessPayload) -> Result<Business, Error> {
    validate_business_payload(&business)?;

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let business = Business {
        id,
        name: business.name,
        owner_principal: caller().to_string(),
        description: business.description,
        address: business.address,
        created_at: time(),
        updated_at: None,
        products_ids: Vec::new()
    };
    do_insert_business(&business);
    Ok(business)
}
// Function that allows a business owner to update his business
#[ic_cdk::update]
fn update_business(id: u64, payload: BusinessPayload) -> Result<Business, Error> {
    match BUSINESS_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut business) => {
            validate_business_payload(&payload)?;
            is_caller_business_owner(&business)?;
            
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

// Function that allows a business owner to delete his business
#[ic_cdk::update]
fn delete_business(id: u64) -> Result<Business, Error> {
    let mut business = get_business(id)?;
    is_caller_business_owner(&business)?;


    business.products_ids.iter_mut().for_each(|product_id| {
        let _ = delete_product(product_id.clone());

    });

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

// Function that allows users to fetch a product from the canister
#[ic_cdk::query]
fn get_product(id: u64) -> Result<Product, Error> {
    match _get_product(&id) {
        Some(product) => Ok(product),
        None => Err(Error::NotFound {
            msg: format!("a product with id={} not found", id),
        }),
    }
}

// Function that allows a business owner to add a new product
#[ic_cdk::update]
fn add_product(product: ProductPayload) -> Result<Product, Error> {
    let mut business = get_business(product.business_id)?;
    is_caller_business_owner(&business)?;
    validate_product_payload(&product)?;
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    let product = Product {
        id,
        business_id: product.business_id,
        name: product.name,
        description: product.description,
        price: product.price,
        created_at: time(),
        updated_at: None,
    };
    business.products_ids.push(product.id);
    do_insert_business(&business);
    do_insert_product(&product);
    Ok(product)
}
// Function that allows the business owner of a product to update a product
#[ic_cdk::update]
fn update_product(id: u64, payload: ProductPayload) -> Result<Product, Error> {
    match PRODUCT_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut product) => {
            let business = get_business(product.business_id)?;
            is_caller_business_owner(&business)?;
            validate_product_payload(&payload)?;
            // Update product's fields with the new payload values
            // business_id is only set when creating the product but can't be updated.
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

// Function that allows the business owner of a product to delete the product from the canister's storage
#[ic_cdk::update]
fn delete_product(id: u64) -> Result<Product, Error> {
    let product= get_product(id)?;
    let mut business = get_business(product.business_id)?;

    is_caller_business_owner(&business)?;

    business.products_ids.retain(|&product_id| product_id != id);

    do_insert_business(&business);

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

// Function to fetch an order from the canister
#[ic_cdk::query]
fn get_order(id: u64) -> Result<Order, Error> {
    match _get_order(&id) {
        Some(order) => Ok(order),
        None => Err(Error::NotFound {
            msg: format!("an order with id={} not found", id),
        }),
    }
}

// Function that allows customers to create an order
#[ic_cdk::update]
fn create_order(order_payload: OrderPayload) -> Result<Order, Error> {
    let mut order_products = Vec::new();
    let mut total_price = 0;
    // loops through the product_ids field of the payload to fetch the details of all the products
    // and to ensure that all the product_id points to existing products
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
    let order_id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let order = Order {
        id: order_id,
        client_principal: caller().to_string(),
        products: order_products,
        total_price,
        created_at: time(),
        updated_at: None,
    };

    do_insert_order(&order);
    Ok(order)
}

// Function that allows the client of an order to update the order's details
#[ic_cdk::update]
fn update_order(id: u64, order_payload: OrderPayload) -> Result<Order, Error> {
    match ORDER_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut order) => {
            // authentication checks to ensure that the caller is the principal of the client
            is_caller_order_client(&order)?;

            let mut new_products = Vec::new();
            let mut new_total_price = 0;
            // loops through the product_ids field of the payload to fetch the details of all the products
            // and to ensure that all the product_id points to existing products
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

// Function that allows a client to delete his order
#[ic_cdk::update]
fn delete_order(id: u64) -> Result<Order, Error> {
    let order = get_order(id)?;
    // authentication checks to ensure that the caller is the principal of the client
    is_caller_order_client(&order)?;

    match ORDER_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(order) => Ok(order),
        None => Err(Error::NotFound {
            msg: format!("couldn't delete an order with id={}. order not found", id),
        }),
    }
}

// Helper function to save a business to the canister's storage
fn do_insert_business(business: &Business) {
    BUSINESS_STORAGE
        .with(|service| service.borrow_mut().insert(business.id, business.clone()));
}
// Helper function to save a product to the canister's storage
fn do_insert_product(product: &Product) {
    PRODUCT_STORAGE
        .with(|service| service.borrow_mut().insert(product.id, product.clone()));
}
// Helper function to save an order to the canister's storage
fn do_insert_order(order: &Order) {
    ORDER_STORAGE
        .with(|service| service.borrow_mut().insert(order.id, order.clone()));
}
// Helper function to get a business from the canister's storage
fn _get_business(id: &u64) -> Option<Business> {
    BUSINESS_STORAGE.with(|service| service.borrow().get(id))
}
// Helper function to get a product from the canister's storage
fn _get_product(id: &u64) -> Option<Product> {
    PRODUCT_STORAGE.with(|service| service.borrow().get(id))
}

// Helper function to get an order from the canister's storage
fn _get_order(id: &u64) -> Option<Order> {
    ORDER_STORAGE.with(|service| service.borrow().get(id))
}

ic_cdk::export_candid!();
