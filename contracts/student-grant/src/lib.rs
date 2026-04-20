#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, contracterror, token, Address, Env, Symbol};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    TargetNotReached = 3,
    AlreadyWithdrawn = 4,
    InvalidAmount = 5,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct GrantProject {
    pub creator: Address,
    pub title: Symbol,
    pub target_amount: i128,
    pub raised_amount: i128,
    pub is_withdrawn: bool,
}

#[contracttype]
pub enum DataKey {
    Project,
    TokenAddress, // Address displit manual dari struct project berdasar kesepakatan desain
}

#[contract]
pub struct StudentGrantContract;

#[contractimpl]
impl StudentGrantContract {
    /// Menginisialisasi proyek dana hibah mahasiswa. Hanya dipanggil satu kali seumur hidup kontrak.
    pub fn initialize(
        env: Env,
        creator: Address,
        title: Symbol,
        target: i128,
        token_address: Address,
    ) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Project) {
            return Err(Error::AlreadyInitialized);
        }
        if target <= 0 {
            return Err(Error::InvalidAmount);
        }

        let project = GrantProject {
            creator,
            title,
            target_amount: target,
            raised_amount: 0,
            is_withdrawn: false,
        };

        env.storage().instance().set(&DataKey::Project, &project);
        env.storage().instance().set(&DataKey::TokenAddress, &token_address);
        Ok(())
    }

    /// Mentransfer dana (donasi) ke dalam pool smart contract.
    pub fn donate(env: Env, donor: Address, amount: i128) -> Result<(), Error> {
        donor.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let mut project: GrantProject = env.storage().instance().get(&DataKey::Project).unwrap();
        let token_address: Address = env.storage().instance().get(&DataKey::TokenAddress).unwrap();

        // Panggil kontrak token untuk memindahkan dana sponsor ke kontrak kita
        let client = token::Client::new(&env, &token_address);
        client.transfer(&donor, &env.current_contract_address(), &amount);

        project.raised_amount += amount;
        env.storage().instance().set(&DataKey::Project, &project);

        Ok(())
    }

    /// Hanya dapat dieksekusi oleh creator setelah dana mencapai target dan belum dicairkan.
    pub fn withdraw(env: Env) -> Result<(), Error> {
        let mut project: GrantProject = env.storage().instance().get(&DataKey::Project).unwrap();

        // Pastikan hanya creator yg dapat melakukan instruksi ini
        project.creator.require_auth();

        if project.is_withdrawn {
            return Err(Error::AlreadyWithdrawn);
        }

        if project.raised_amount < project.target_amount {
            return Err(Error::TargetNotReached);
        }

        // ATOMIC SWAP: Update state boolean terlebih dulu mencegah reentrancy / kegagalan eksekusi logis di tengah transfer
        project.is_withdrawn = true;
        env.storage().instance().set(&DataKey::Project, &project);

        // Buat client token dan lakukan transfer ke creator
        let token_address: Address = env.storage().instance().get(&DataKey::TokenAddress).unwrap();
        let client = token::Client::new(&env, &token_address);
        client.transfer(
            &env.current_contract_address(),
            &project.creator,
            &project.raised_amount,
        );

        Ok(())
    }

    /// Mengembalikan seluruh state info GrantProject untuk UI
    pub fn get_project(env: Env) -> GrantProject {
        env.storage().instance().get(&DataKey::Project).unwrap()
    }
}
