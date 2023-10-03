// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#[allow(unused)]
use spacetimedb_sdk::{
    anyhow::{anyhow, Result},
    identity::Identity,
    reducer::{Reducer, ReducerCallbackId, Status},
    sats::{de::Deserialize, ser::Serialize},
    spacetimedb_lib,
    table::{TableIter, TableType, TableWithPrimaryKey},
    Address,
};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UpdatePkU128Args {
    pub n: u128,
    pub data: i32,
}

impl Reducer for UpdatePkU128Args {
    const REDUCER_NAME: &'static str = "update_pk_u128";
}

#[allow(unused)]
pub fn update_pk_u_128(n: u128, data: i32) {
    UpdatePkU128Args { n, data }.invoke();
}

#[allow(unused)]
pub fn on_update_pk_u_128(
    mut __callback: impl FnMut(&Identity, Option<Address>, &Status, &u128, &i32) + Send + 'static,
) -> ReducerCallbackId<UpdatePkU128Args> {
    UpdatePkU128Args::on_reducer(move |__identity, __addr, __status, __args| {
        let UpdatePkU128Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn once_on_update_pk_u_128(
    __callback: impl FnOnce(&Identity, Option<Address>, &Status, &u128, &i32) + Send + 'static,
) -> ReducerCallbackId<UpdatePkU128Args> {
    UpdatePkU128Args::once_on_reducer(move |__identity, __addr, __status, __args| {
        let UpdatePkU128Args { n, data } = __args;
        __callback(__identity, __addr, __status, n, data);
    })
}

#[allow(unused)]
pub fn remove_on_update_pk_u_128(id: ReducerCallbackId<UpdatePkU128Args>) {
    UpdatePkU128Args::remove_on_reducer(id);
}