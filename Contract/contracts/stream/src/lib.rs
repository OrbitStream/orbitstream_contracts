//! # OrbitStream — Token Stream Contract
//!
//! Continuous real-time token streaming on Stellar/Soroban.
//!
//! ## Stream lifecycle
//! ```
//! Active ──pause──► Paused ──resume──► Active
//!   │                                    │
//!   └──cancel──────────────────────────► Cancelled  (prorated settlement)
//!   └──(end_time reached + fully claimed)► Completed
//! ```
//!
//! ## Time model
//! Uses `env.ledger().timestamp()` (Unix seconds) so `rate_per_second`
//! is a true per-second rate. Paused time is excluded from earnings.
//!
//! ## Solvency invariant
//! At every moment: `total_deposited >= total_claimed + claimable_now`

#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror,
    symbol_short, token, Address, BytesN, Env,
};

// ── Storage TTLs ──────────────────────────────────────────────────────────────
const INSTANCE_TTL: u32 = 518_400;   // ~30 days
const STREAM_TTL:   u32 = 6_307_200; // ~1 year

// ── Types ─────────────────────────────────────────────────────────────────────
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamStatus { Active, Paused, Cancelled, Completed }

#[contracttype]
#[derive(Clone, Debug)]
pub struct Stream {
    pub id:                   u64,
    pub sender:               Address,
    pub recipient:            Address,
    /// Stellar asset contract (XLM, USDC, etc.)
    pub token:                Address,
    /// Token units earned per active second.
    pub rate_per_second:      i128,
    /// Unix timestamp when the stream started.
    pub start_time:           u64,
    /// Unix timestamp when the stream ends (0 = open-ended).
    pub end_time:             u64,
    /// Total tokens deposited to fund this stream.
    pub total_deposited:      i128,
    /// Total tokens already claimed by recipient.
    pub total_claimed:        i128,
    pub status:               StreamStatus,
    /// Timestamp when last paused (0 if not paused).
    pub paused_at:            u64,
    /// Cumulative seconds this stream has been paused.
    pub total_paused_seconds: u64,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    StreamCount,
    Stream(u64),
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized     = 2,
    Unauthorized       = 3,
    NotFound           = 4,
    InvalidRate        = 5,
    InvalidDeposit     = 6,
    SelfStream         = 7,
    StreamNotActive    = 8,
    StreamNotPaused    = 9,
    NothingToClaim     = 10,
    AlreadyTerminated  = 11,
    InsufficientFunds  = 12,
}

// ── Contract ──────────────────────────────────────────────────────────────────
#[contract]
pub struct StreamContract;

#[contractimpl]
impl StreamContract {

    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin,       &admin);
        env.storage().instance().set(&DataKey::StreamCount, &0u64);
        env.storage().instance().extend_ttl(INSTANCE_TTL, INSTANCE_TTL);
        Ok(())
    }

    pub fn upgrade(env: Env, new_wasm: BytesN<32>) -> Result<(), Error> {
        Self::require_admin(&env)?;
        env.deployer().update_current_contract_wasm(new_wasm);
        Ok(())
    }

    // ── Stream creation ───────────────────────────────────────────────────────

    /// Open a new stream and lock the initial deposit.
    ///
    /// * `rate_per_second`  — token units (e.g. stroops) earned every active second.
    /// * `duration_seconds` — stream length; 0 = open-ended.
    /// * `deposit`          — tokens locked upfront (must cover ≥ 1 second).
    pub fn create_stream(
        env:              Env,
        sender:           Address,
        recipient:        Address,
        token:            Address,
        rate_per_second:  i128,
        duration_seconds: u64,
        deposit:          i128,
    ) -> Result<u64, Error> {
        sender.require_auth();

        if sender == recipient           { return Err(Error::SelfStream);         }
        if rate_per_second <= 0          { return Err(Error::InvalidRate);        }
        if deposit <= 0                  { return Err(Error::InvalidDeposit);     }
        if deposit < rate_per_second     { return Err(Error::InsufficientFunds);  }

        token::Client::new(&env, &token)
            .transfer(&sender, &env.current_contract_address(), &deposit);

        env.storage().instance().extend_ttl(INSTANCE_TTL, INSTANCE_TTL);
        let count: u64 = env.storage().instance()
            .get(&DataKey::StreamCount).unwrap_or(0);
        let id = count + 1;
        env.storage().instance().set(&DataKey::StreamCount, &id);

        let now = env.ledger().timestamp();
        let stream = Stream {
            id,
            sender:               sender.clone(),
            recipient:            recipient.clone(),
            token,
            rate_per_second,
            start_time:           now,
            end_time:             if duration_seconds > 0 { now + duration_seconds } else { 0 },
            total_deposited:      deposit,
            total_claimed:        0,
            status:               StreamStatus::Active,
            paused_at:            0,
            total_paused_seconds: 0,
        };
        Self::save(&env, &stream);

        env.events().publish(
            (symbol_short!("created"),),
            (id, sender, recipient, rate_per_second, deposit),
        );

        Ok(id)
    }

    /// Add more tokens to keep a stream running.
    pub fn top_up(env: Env, sender: Address, stream_id: u64, amount: i128) -> Result<i128, Error> {
        sender.require_auth();
        if amount <= 0 { return Err(Error::InvalidDeposit); }

        let mut s = Self::load(&env, stream_id)?;
        if s.sender != sender { return Err(Error::Unauthorized); }
        Self::ensure_live(&s)?;

        token::Client::new(&env, &s.token)
            .transfer(&sender, &env.current_contract_address(), &amount);

        s.total_deposited += amount;
        Self::save(&env, &s);

        env.events().publish((symbol_short!("topup"),), (stream_id, amount));
        Ok(s.total_deposited)
    }

    // ── Claiming ──────────────────────────────────────────────────────────────

    /// Recipient claims all accrued tokens up to this moment.
    pub fn claim(env: Env, recipient: Address, stream_id: u64) -> Result<i128, Error> {
        recipient.require_auth();

        let mut s = Self::load(&env, stream_id)?;
        if s.recipient != recipient        { return Err(Error::Unauthorized);     }
        if s.status == StreamStatus::Cancelled { return Err(Error::AlreadyTerminated); }

        let payout = Self::safe_claimable(&env, &s);
        if payout == 0 { return Err(Error::NothingToClaim); }

        s.total_claimed += payout;
        if s.end_time > 0 && env.ledger().timestamp() >= s.end_time {
            s.status = StreamStatus::Completed;
        }
        Self::save(&env, &s);

        token::Client::new(&env, &s.token)
            .transfer(&env.current_contract_address(), &recipient, &payout);

        env.events().publish(
            (symbol_short!("claimed"),),
            (stream_id, recipient, payout),
        );
        Ok(payout)
    }

    // ── Flow control ──────────────────────────────────────────────────────────

    pub fn pause_stream(env: Env, sender: Address, stream_id: u64) -> Result<(), Error> {
        sender.require_auth();
        let mut s = Self::load(&env, stream_id)?;
        if s.sender != sender              { return Err(Error::Unauthorized);    }
        if s.status != StreamStatus::Active { return Err(Error::StreamNotActive); }

        s.status    = StreamStatus::Paused;
        s.paused_at = env.ledger().timestamp();
        Self::save(&env, &s);

        env.events().publish((symbol_short!("paused"),), (stream_id,));
        Ok(())
    }

    pub fn resume_stream(env: Env, sender: Address, stream_id: u64) -> Result<(), Error> {
        sender.require_auth();
        let mut s = Self::load(&env, stream_id)?;
        if s.sender != sender              { return Err(Error::Unauthorized);    }
        if s.status != StreamStatus::Paused { return Err(Error::StreamNotPaused); }

        let now = env.ledger().timestamp();
        s.total_paused_seconds += now.saturating_sub(s.paused_at);
        s.paused_at = 0;
        s.status    = StreamStatus::Active;
        Self::save(&env, &s);

        env.events().publish((symbol_short!("resumed"),), (stream_id,));
        Ok(())
    }

    /// Cancel a stream — recipient receives earned tokens, sender gets the rest.
    pub fn cancel_stream(env: Env, sender: Address, stream_id: u64) -> Result<(), Error> {
        sender.require_auth();
        let mut s = Self::load(&env, stream_id)?;
        if s.sender != sender { return Err(Error::Unauthorized); }
        Self::ensure_live(&s)?;

        // Account for any outstanding pause time
        if s.status == StreamStatus::Paused {
            s.total_paused_seconds += env.ledger().timestamp().saturating_sub(s.paused_at);
        }

        let recipient_payout = Self::safe_claimable(&env, &s);
        if recipient_payout > 0 {
            token::Client::new(&env, &s.token)
                .transfer(&env.current_contract_address(), &s.recipient, &recipient_payout);
            s.total_claimed += recipient_payout;
        }

        let sender_refund = s.total_deposited.saturating_sub(s.total_claimed);
        if sender_refund > 0 {
            token::Client::new(&env, &s.token)
                .transfer(&env.current_contract_address(), &s.sender, &sender_refund);
        }

        s.status = StreamStatus::Cancelled;
        Self::save(&env, &s);

        env.events().publish(
            (symbol_short!("cancel"),),
            (stream_id, recipient_payout, sender_refund),
        );
        Ok(())
    }

    /// Admin emergency cancel (e.g. exploit mitigation).
    pub fn admin_cancel(env: Env, stream_id: u64) -> Result<(), Error> {
        Self::require_admin(&env)?;
        let mut s = Self::load(&env, stream_id)?;
        Self::ensure_live(&s)?;

        if s.status == StreamStatus::Paused {
            s.total_paused_seconds += env.ledger().timestamp().saturating_sub(s.paused_at);
        }

        let recipient_payout = Self::safe_claimable(&env, &s);
        if recipient_payout > 0 {
            token::Client::new(&env, &s.token)
                .transfer(&env.current_contract_address(), &s.recipient, &recipient_payout);
            s.total_claimed += recipient_payout;
        }
        let sender_refund = s.total_deposited.saturating_sub(s.total_claimed);
        if sender_refund > 0 {
            token::Client::new(&env, &s.token)
                .transfer(&env.current_contract_address(), &s.sender, &sender_refund);
        }

        s.status = StreamStatus::Cancelled;
        Self::save(&env, &s);

        env.events().publish((symbol_short!("admcancel"),), (stream_id,));
        Ok(())
    }

    // ── Views ──────────────────────────────────────────────────────────────────

    pub fn get_stream(env: Env, stream_id: u64) -> Option<Stream> {
        env.storage().persistent().get(&DataKey::Stream(stream_id))
    }

    pub fn claimable_amount(env: Env, stream_id: u64) -> Result<i128, Error> {
        let s = Self::load(&env, stream_id)?;
        Ok(Self::safe_claimable(&env, &s))
    }

    pub fn active_seconds(env: Env, stream_id: u64) -> Result<u64, Error> {
        let s = Self::load(&env, stream_id)?;
        Ok(Self::elapsed_active(&env, &s))
    }

    pub fn stream_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::StreamCount).unwrap_or(0)
    }

    // ── Internal ───────────────────────────────────────────────────────────────

    fn load(env: &Env, id: u64) -> Result<Stream, Error> {
        env.storage().persistent().get(&DataKey::Stream(id)).ok_or(Error::NotFound)
    }

    fn save(env: &Env, s: &Stream) {
        env.storage().persistent().set(&DataKey::Stream(s.id), s);
        env.storage().persistent().extend_ttl(&DataKey::Stream(s.id), STREAM_TTL, STREAM_TTL);
    }

    fn require_admin(env: &Env) -> Result<(), Error> {
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin).ok_or(Error::NotInitialized)?;
        admin.require_auth();
        Ok(())
    }

    fn ensure_live(s: &Stream) -> Result<(), Error> {
        match s.status {
            StreamStatus::Cancelled | StreamStatus::Completed => Err(Error::AlreadyTerminated),
            _ => Ok(()),
        }
    }

    fn elapsed_active(env: &Env, s: &Stream) -> u64 {
        let effective = match s.status {
            StreamStatus::Paused => s.paused_at,
            _ => {
                let t = env.ledger().timestamp();
                if s.end_time > 0 { t.min(s.end_time) } else { t }
            }
        };
        effective
            .saturating_sub(s.start_time)
            .saturating_sub(s.total_paused_seconds)
    }

    fn safe_claimable(env: &Env, s: &Stream) -> i128 {
        let active = Self::elapsed_active(env, s) as i128;
        let earned  = active.saturating_mul(s.rate_per_second);
        let pending = earned.saturating_sub(s.total_claimed).max(0);
        let headroom = s.total_deposited.saturating_sub(s.total_claimed).max(0);
        pending.min(headroom)
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        token::{Client as TokenClient, StellarAssetClient},
        Env,
    };

    fn setup(env: &Env) -> (StreamContractClient, Address, Address, Address, Address) {
        env.mock_all_auths();
        let contract  = env.register(StreamContract, ());
        let client    = StreamContractClient::new(env, &contract);
        let admin     = Address::generate(env);
        let sender    = Address::generate(env);
        let recipient = Address::generate(env);
        let tok_admin = Address::generate(env);
        let token     = env.register_stellar_asset_contract_v2(tok_admin.clone()).address();
        StellarAssetClient::new(env, &token).mint(&sender, &10_000_000_000);
        client.initialize(&admin);
        env.ledger().set_timestamp(1_000);
        (client, admin, sender, recipient, token)
    }

    fn tick(env: &Env, secs: u64) {
        env.ledger().with_mut(|l| { l.timestamp += secs; l.sequence_number += (secs / 5) as u32; });
    }

    #[test]
    fn test_create_and_claim() {
        let env = Env::default();
        let (c, _, sender, recipient, token) = setup(&env);

        let rate    = 1_000_i128;
        let deposit = 100_000_i128;
        let id = c.create_stream(&sender, &recipient, &token, &rate, &100, &deposit);

        tick(&env, 50);
        let payout = c.claim(&recipient, &id);
        assert_eq!(payout, 50_000); // 50s * 1000

        assert_eq!(TokenClient::new(&env, &token).balance(&recipient), 50_000);
    }

    #[test]
    fn test_solvency_cap() {
        let env = Env::default();
        let (c, _, sender, recipient, token) = setup(&env);

        let id = c.create_stream(&sender, &recipient, &token, &1_000, &0, &10_000);
        tick(&env, 9999); // way past deposit capacity

        let claimable = c.claimable_amount(&id);
        assert_eq!(claimable, 10_000); // capped at deposit
    }

    #[test]
    fn test_pause_stops_accrual() {
        let env = Env::default();
        let (c, _, sender, recipient, token) = setup(&env);

        let id = c.create_stream(&sender, &recipient, &token, &100, &0, &100_000);
        tick(&env, 50);                  // earn 5_000
        c.pause_stream(&sender, &id);
        tick(&env, 200);                 // paused — should not earn
        c.resume_stream(&sender, &id);
        tick(&env, 30);                  // earn 3_000

        let payout = c.claim(&recipient, &id);
        assert_eq!(payout, 8_000);       // 50 + 30 = 80s active
    }

    #[test]
    fn test_cancel_settles_both_parties() {
        let env = Env::default();
        let (c, _, sender, recipient, token) = setup(&env);

        let deposit = 100_000_i128;
        let rate    = 500_i128;
        let id      = c.create_stream(&sender, &recipient, &token, &rate, &0, &deposit);

        tick(&env, 100); // earned 50_000

        let sender_before = TokenClient::new(&env, &token).balance(&sender);
        c.cancel_stream(&sender, &id);

        let recipient_bal = TokenClient::new(&env, &token).balance(&recipient);
        let sender_bal    = TokenClient::new(&env, &token).balance(&sender);

        assert_eq!(recipient_bal, 50_000);
        assert_eq!(sender_bal - sender_before, 50_000);
    }

    #[test]
    fn test_top_up_extends_runway() {
        let env = Env::default();
        let (c, _, sender, recipient, token) = setup(&env);

        let id = c.create_stream(&sender, &recipient, &token, &1_000, &0, &5_000);
        tick(&env, 3);

        let new_total = c.top_up(&sender, &id, &45_000);
        assert_eq!(new_total, 50_000);

        tick(&env, 47);
        let payout = c.claim(&recipient, &id);
        assert_eq!(payout, 50_000); // 50 active seconds
    }

    #[test]
    fn test_stream_auto_completes() {
        let env = Env::default();
        let (c, _, sender, recipient, token) = setup(&env);

        let id = c.create_stream(&sender, &recipient, &token, &100, &60, &6_000);
        tick(&env, 120); // past end time

        c.claim(&recipient, &id);
        assert_eq!(c.get_stream(&id).unwrap().status, StreamStatus::Completed);
    }

    #[test]
    fn test_self_stream_rejected() {
        let env = Env::default();
        let (c, _, sender, _, token) = setup(&env);
        assert!(c.try_create_stream(&sender, &sender, &token, &100, &0, &10_000).is_err());
    }

    #[test]
    fn test_stream_count_increments() {
        let env = Env::default();
        let (c, _, sender, recipient, token) = setup(&env);

        c.create_stream(&sender, &recipient, &token, &100, &0, &10_000);
        c.create_stream(&sender, &recipient, &token, &200, &0, &20_000);
        assert_eq!(c.stream_count(), 2);
    }
}
