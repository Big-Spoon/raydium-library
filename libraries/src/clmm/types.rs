use solana_sdk::pubkey::Pubkey;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
pub struct RewardItem {
    pub token_program: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_vault: Pubkey,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClmmCreatePoolResult {
    pub mint0: Pubkey,
    pub mint1: Pubkey,
    pub mint0_token_program: Pubkey,
    pub mint1_token_program: Pubkey,
    pub price: f64,
    pub sqrt_price_x64: u128,
    pub tick: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClmmLiquidityChangeResult {
    pub mint0: Pubkey,
    pub mint1: Pubkey,
    pub vault0: Pubkey,
    pub vault1: Pubkey,
    pub mint0_token_program: Pubkey,
    pub mint1_token_program: Pubkey,
    pub reward_items: Vec<RewardItem>,
    pub liquidity: u128,
    pub amount_0: u64,
    pub amount_1: u64,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,
    pub tick_array_lower_start_index: i32,
    pub tick_array_upper_start_index: i32,
}

/// Result of a CLMM swap calculation. Contains everything needed to build
/// the on-chain swap_v2 instruction.
///
/// ## Field Reference
///
/// ### Pool accounts (pass directly to swap_v2_instr)
/// - `pool_amm_config`    - the AmmConfig account governing this pool (fee rate etc.)
/// - `pool_id`            - the pool state account
/// - `pool_observation`   - oracle/TWAP observation account
/// - `input_vault`        - pool vault that holds the input token
/// - `output_vault`       - pool vault that holds the output token
///
/// ### Token mints and programs
/// - `input_vault_mint`   - mint of the token being swapped IN
/// - `output_vault_mint`  - mint of the token being received OUT
/// - `input_token_program`  - SPL Token or Token-2022 program for input mint
/// - `output_token_program` - SPL Token or Token-2022 program for output mint
///
/// ### User accounts
/// - `user_input_token`   - the USER'S token account for the input token.
///                          THIS IS CRITICAL: it is set directly from the
///                          `input_token_pubkey` passed to calculate_swap_change*.
///                          For buy (SOL→token): must be the WSOL ATA.
///                          For sell (token→SOL): must be the TOKEN ATA (e.g. USDC ATA).
///                          Passing the wrong pubkey here causes InsufficientFunds
///                          because the CLMM program debits from this account.
///
/// ### Tick array accounts (remaining_accounts for the instruction)
/// - `remaining_tick_array_keys` - ordered list of tick array PDAs needed for the swap.
///                                 Order matters. Prepend the bitmap extension account
///                                 before passing to swap_v2_instr.
///
/// ### Amounts
/// - `amount`                - the exact input amount in lamports (base_in=true)
///                             or exact output amount (base_in=false)
/// - `other_amount_threshold`- slippage-adjusted counterpart:
///                             base_in=true:  minimum output tokens guaranteed
///                             base_in=false: maximum input tokens allowed
///                             For arb profit calculation: sell.other_amount_threshold
///                             is the minimum SOL received back from the sell leg.
/// - `sqrt_price_limit_x64` - optional price limit as Q64.64; None = no limit
/// - `is_base_input`         - true = exact-in swap (you specify input, get output)
///                             false = exact-out swap (you specify output, give input)
#[derive(Default, Clone, Debug, PartialEq)]
pub struct ClmmSwapChangeResult {
    pub pool_amm_config: Pubkey,
    pub pool_id: Pubkey,
    pub pool_observation: Pubkey,
    pub input_vault: Pubkey,
    pub output_vault: Pubkey,
    pub input_vault_mint: Pubkey,
    pub output_vault_mint: Pubkey,
    pub input_token_program: Pubkey,
    pub output_token_program: Pubkey,
    pub user_input_token: Pubkey,
    pub remaining_tick_array_keys: VecDeque<Pubkey>,
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit_x64: Option<u128>,
    pub is_base_input: bool,
}

// the top level state of the swap, the results of which are recorded in storage at the end
#[derive(Debug)]
pub struct SwapState {
    // the amount remaining to be swapped in/out of the input/output asset
    pub amount_specified_remaining: u64,
    // the amount already swapped out/in of the output/input asset
    pub amount_calculated: u64,
    // current sqrt(price)
    pub sqrt_price_x64: u128,
    // the tick associated with the current price
    pub tick: i32,
    // the current liquidity in range
    pub liquidity: u128,
}
#[derive(Default)]
pub struct StepComputations {
    // the price at the beginning of the step
    pub sqrt_price_start_x64: u128,
    // the next tick to swap to from the current tick in the swap direction
    pub tick_next: i32,
    // whether tick_next is initialized or not
    pub initialized: bool,
    // sqrt(price) for the next tick (1/0)
    pub sqrt_price_next_x64: u128,
    // how much is being swapped in in this step
    pub amount_in: u64,
    // how much is being swapped out
    pub amount_out: u64,
    // how much fee is being paid in
    pub fee_amount: u64,
}
