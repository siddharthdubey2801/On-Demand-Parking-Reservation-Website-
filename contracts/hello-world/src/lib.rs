#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, Env, Symbol, String, symbol_short, log};

// Parking reservation struct
#[contracttype]
#[derive(Clone)]
pub struct Reservation {
    pub res_id: u64,
    pub user: String,
    pub location: String,
    pub timestamp: u64,
    pub duration_minutes: u32,
    pub is_active: bool,
}

const RES_COUNT: Symbol = symbol_short!("RES_CT");

#[contracttype]
pub enum ReservationBook {
    Reservation(u64),
}

#[contract]
pub struct ParkingReservation;

#[contractimpl]
impl ParkingReservation {
    // Create a new reservation
    pub fn create_reservation(env: Env, user: String, location: String, duration_minutes: u32) -> u64 {
        let mut count: u64 = env.storage().instance().get(&RES_COUNT).unwrap_or(0);
        count += 1;

        let timestamp = env.ledger().timestamp();

        let res = Reservation {
            res_id: count,
            user,
            location,
            timestamp,
            duration_minutes,
            is_active: true,
        };

        env.storage().instance().set(&ReservationBook::Reservation(count), &res);
        env.storage().instance().set(&RES_COUNT, &count);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Reservation created with ID: {}", count);
        count
    }

    // View reservation by ID
    pub fn view_reservation(env: Env, res_id: u64) -> Reservation {
        env.storage().instance().get(&ReservationBook::Reservation(res_id)).unwrap_or(Reservation {
            res_id: 0,
            user: String::from_str(&env, "Not_Found"),
            location: String::from_str(&env, "Not_Found"),
            timestamp: 0,
            duration_minutes: 0,
            is_active: false,
        })
    }

    // Mark reservation as expired
    pub fn expire_reservation(env: Env, res_id: u64) {
        let mut res = Self::view_reservation(env.clone(), res_id);
        if res.is_active {
            res.is_active = false;
            env.storage().instance().set(&ReservationBook::Reservation(res_id), &res);
            log!(&env, "Reservation {} is now expired.", res_id);
        } else {
            panic!("Reservation already expired or doesn't exist");
        }
    }
}
