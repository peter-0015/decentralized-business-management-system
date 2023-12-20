use crate::types::*;
use ic_cdk::caller;


fn is_invalid_string_input(str_input: &str, comparison_length: usize) -> bool {
    return str_input.trim().len() == comparison_length;
}

pub fn validate_business_payload(payload: &BusinessPayload) -> Result<(), Error>{
    let mut errors: Vec<String> = Vec::new();
    if is_invalid_string_input(&payload.name, 0){
        errors.push(format!("Business name='{}' cannot be empty.", payload.name))
    }
    let is_description_empty = is_invalid_string_input(&payload.description, 0);
    let is_description_descriptive: Vec<&str> = payload.description.trim().split(" ").collect();

    if is_description_empty || is_description_descriptive.len() < 2{
        errors.push(format!("Description='{}' needs to be descriptive.", payload.description))
    }
    if is_invalid_string_input(&payload.address, 0){
        errors.push(format!("Address='{}' cannot be empty.", payload.address));
    }
    if errors.is_empty(){
        Ok(())
    }else{
        return Err(Error::InvalidPayload { errors })
    }
}
pub fn validate_product_payload(payload: &ProductPayload) -> Result<(), Error>{
    let mut errors: Vec<String> = Vec::new();
    if is_invalid_string_input(&payload.name, 0){
        errors.push(format!("Product name='{}' cannot be empty.", payload.name))
    }
    let is_description_empty = is_invalid_string_input(&payload.description, 0);
    let is_description_descriptive: Vec<&str> = payload.description.trim().split(" ").collect();

    if is_description_empty || is_description_descriptive.len() < 2{
        errors.push(format!("Description='{}' needs to be descriptive.", payload.description))
    }
    if payload.price == 0{
        errors.push(format!("Price='{}' must be greater than zero.", payload.price));
    }
    if errors.is_empty(){
        Ok(())
    }else{
        return Err(Error::InvalidPayload { errors })
    }
}



// Helper function to check whether the caller is the principal of the business owner
pub fn is_caller_business_owner(business: &Business) -> Result<(), Error>{
    if business.owner_principal != caller().to_string(){
        return Err(Error::NotBusinessOwner { msg: format!("Caller is not the principal of the business owner") })
    }else{
        Ok(())
    }
}
// Helper function to check whether the caller is the principal of the client of an order
pub fn is_caller_order_client(order: &Order) -> Result<(), Error>{
    if order.client_principal != caller().to_string(){
        return Err(Error::NotOrderClient { msg: format!("Caller is not the principal of the order's client") })
    }else{
        Ok(())
    }
}