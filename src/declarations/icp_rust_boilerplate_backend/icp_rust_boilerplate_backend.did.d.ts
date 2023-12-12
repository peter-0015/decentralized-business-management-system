import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface Business {
  'id' : bigint,
  'updated_at' : [] | [bigint],
  'name' : string,
  'description' : string,
  'created_at' : bigint,
  'address' : string,
}
export interface BusinessPayload {
  'name' : string,
  'description' : string,
  'address' : string,
}
export type Error = { 'NotFound' : { 'msg' : string } };
export interface Order {
  'id' : bigint,
  'updated_at' : [] | [bigint],
  'total_price' : bigint,
  'created_at' : bigint,
  'products' : Array<Product>,
}
export interface OrderPayload { 'product_ids' : BigUint64Array | bigint[] }
export interface Product {
  'id' : bigint,
  'updated_at' : [] | [bigint],
  'name' : string,
  'description' : string,
  'created_at' : bigint,
  'price' : bigint,
}
export interface ProductPayload {
  'name' : string,
  'description' : string,
  'price' : bigint,
}
export type Result = { 'Ok' : Order } |
  { 'Err' : Error };
export type Result_1 = { 'Ok' : Business } |
  { 'Err' : Error };
export type Result_2 = { 'Ok' : Product } |
  { 'Err' : Error };
export interface _SERVICE {
  'add_business' : ActorMethod<[BusinessPayload], [] | [Business]>,
  'add_product' : ActorMethod<[ProductPayload], [] | [Product]>,
  'create_order' : ActorMethod<[OrderPayload], Result>,
  'delete_business' : ActorMethod<[bigint], Result_1>,
  'delete_order' : ActorMethod<[bigint], Result>,
  'delete_product' : ActorMethod<[bigint], Result_2>,
  'get_business' : ActorMethod<[bigint], Result_1>,
  'get_order' : ActorMethod<[bigint], Result>,
  'get_product' : ActorMethod<[bigint], Result_2>,
  'update_business' : ActorMethod<[bigint, BusinessPayload], Result_1>,
  'update_order' : ActorMethod<[bigint, OrderPayload], Result>,
  'update_product' : ActorMethod<[bigint, ProductPayload], Result_2>,
}
