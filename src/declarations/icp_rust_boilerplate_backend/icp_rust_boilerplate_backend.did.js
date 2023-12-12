export const idlFactory = ({ IDL }) => {
  const BusinessPayload = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'address' : IDL.Text,
  });
  const Business = IDL.Record({
    'id' : IDL.Nat64,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'name' : IDL.Text,
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
    'address' : IDL.Text,
  });
  const ProductPayload = IDL.Record({
    'name' : IDL.Text,
    'description' : IDL.Text,
    'price' : IDL.Nat64,
  });
  const Product = IDL.Record({
    'id' : IDL.Nat64,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'name' : IDL.Text,
    'description' : IDL.Text,
    'created_at' : IDL.Nat64,
    'price' : IDL.Nat64,
  });
  const OrderPayload = IDL.Record({ 'product_ids' : IDL.Vec(IDL.Nat64) });
  const Order = IDL.Record({
    'id' : IDL.Nat64,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'total_price' : IDL.Nat64,
    'created_at' : IDL.Nat64,
    'products' : IDL.Vec(Product),
  });
  const Error = IDL.Variant({ 'NotFound' : IDL.Record({ 'msg' : IDL.Text }) });
  const Result = IDL.Variant({ 'Ok' : Order, 'Err' : Error });
  const Result_1 = IDL.Variant({ 'Ok' : Business, 'Err' : Error });
  const Result_2 = IDL.Variant({ 'Ok' : Product, 'Err' : Error });
  return IDL.Service({
    'add_business' : IDL.Func([BusinessPayload], [IDL.Opt(Business)], []),
    'add_product' : IDL.Func([ProductPayload], [IDL.Opt(Product)], []),
    'create_order' : IDL.Func([OrderPayload], [Result], []),
    'delete_business' : IDL.Func([IDL.Nat64], [Result_1], []),
    'delete_order' : IDL.Func([IDL.Nat64], [Result], []),
    'delete_product' : IDL.Func([IDL.Nat64], [Result_2], []),
    'get_business' : IDL.Func([IDL.Nat64], [Result_1], ['query']),
    'get_order' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'get_product' : IDL.Func([IDL.Nat64], [Result_2], ['query']),
    'update_business' : IDL.Func([IDL.Nat64, BusinessPayload], [Result_1], []),
    'update_order' : IDL.Func([IDL.Nat64, OrderPayload], [Result], []),
    'update_product' : IDL.Func([IDL.Nat64, ProductPayload], [Result_2], []),
  });
};
export const init = ({ IDL }) => { return []; };
