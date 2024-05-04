use std::str::FromStr;
use borsh::{BorshDeserialize, BorshSerialize};
// use solana_program::pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LiquidityPoolState {
    status: u64,
    nonce: u64,
    max_order: u64,
    depth: u64,
    base_decimal: u64,
    quote_decimal: u64,
    state: u64,
    reset_flag: u64,
    min_size: u64,
    vol_max_cut_ratio: u64,
    amount_wave_ratio: u64,
    base_lot_size: u64,
    quote_lot_size: u64,
    min_price_multiplier: u64,
    max_price_multiplier: u64,
    system_decimal_value: u64,
    min_separate_numerator: u64,
    min_separate_denominator: u64,
    trade_fee_numerator: u64,
    trade_fee_denominator: u64,
    pnl_numerator: u64,
    pnl_denominator: u64,
    swap_fee_numerator: u64,
    swap_fee_denominator: u64,
    base_need_take_pnl: u64,
    quote_need_take_pnl: u64,
    quote_total_pnl: u64,
    base_total_pnl: u64,
    pool_open_time: u64,
    punish_pc_amount: u64,
    punish_coin_amount: u64,
    orderbook_to_init_time: u64,
    swap_base_in_amount: u128,
    swap_quote_out_amount: u128,
    swap_base2_quote_fee: u64,
    swap_quote_in_amount: u128,
    swap_base_out_amount: u128,
    swap_quote2_base_fee: u64,
    base_vault: Pubkey,
    quote_vault: Pubkey,
    base_mint: Pubkey,
    quote_mint: Pubkey,
    lp_mint: Pubkey,
    open_orders: Pubkey,
    market_id: Pubkey,
    market_program_id: Pubkey,
    target_orders: Pubkey,
    withdraw_queue: Pubkey,
    lp_vault: Pubkey,
    owner: Pubkey,
    lp_reserve: u64,
    padding: [u64; 3],
}


const RAYDIUM_POOL_STR: &'static str = "AefWzbX8jCyjzzbkTk4kDqTQFWk7pk9AXnjZbLdVfKaR";
// https://www.geckoterminal.com/solana/pools/AefWzbX8jCyjzzbkTk4kDqTQFWk7pk9AXnjZbLdVfKaR


fn fetch_and_deserialize_pool_state(client: &RpcClient, pool_pubkey: &Pubkey) -> Result<LiquidityPoolState, Box<dyn std::error::Error>> {
    let account_data = client.get_account_data(pool_pubkey)?;
    let pool_state = LiquidityPoolState::try_from_slice(&account_data)?;
    Ok(pool_state)
}

fn main() {
    let rpc_url = String::from("https://api.mainnet-beta.solana.com");
    let client = RpcClient::new(rpc_url);
    let pool_pubkey = Pubkey::from_str(RAYDIUM_POOL_STR).unwrap();

    let pool_state = fetch_and_deserialize_pool_state(&client, &pool_pubkey).expect("Failed to fetch pool state");

    println!("{:?}", pool_state);
}
