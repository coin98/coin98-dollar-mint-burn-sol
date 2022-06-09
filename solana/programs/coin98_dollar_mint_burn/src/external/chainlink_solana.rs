use chainlink_solana::{
  decimals,
  latest_round_data,
};
use solana_program::{
  account_info::{
    AccountInfo,
  },
  declare_id,
};
use std::convert::{
  TryFrom,
};

declare_id!("HEvSKofvBgfaexv23kMabbYqxasxU3mQ4ibBMEmJWHny");

pub fn get_price_feed<'i>(
  chainlink_program: &AccountInfo<'i>,
  feed_account: &AccountInfo<'i>,
) -> (u64, u64) {

  let price = latest_round_data(
      chainlink_program.clone(),
      feed_account.clone(),
    ).unwrap();
  let precision = decimals(
      chainlink_program.clone(),
      feed_account.clone(),
    ).unwrap();

  let price = u64::try_from(price.answer).unwrap();
  let precision = u64::from(precision);

  (price, precision)
}

pub fn is_chainlink_program<'a>(account: &AccountInfo<'a>) -> bool {
  *account.key == ID
}