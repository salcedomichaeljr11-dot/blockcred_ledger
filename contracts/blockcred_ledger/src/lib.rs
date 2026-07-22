#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Record {
    pub course: String,
    pub grade: u32,
    pub attendance: u32,
    pub internship_hours: u32,
}

#[contracttype]
pub enum DataKey {
    Professor(Address),
    Student(Address),
}

#[contract]
pub struct StudentLogbook;

#[contractimpl]
impl StudentLogbook {

    pub fn authorize_professor(env: Env, professor: Address) {
        professor.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Professor(professor), &true);
    }

    pub fn add_record(
        env: Env,
        professor: Address,
        student: Address,
        course: String,
        grade: u32,
        attendance: u32,
        internship_hours: u32,
    ) {

        professor.require_auth();

        let allowed: bool = env
            .storage()
            .instance()
            .get(&DataKey::Professor(professor.clone()))
            .unwrap_or(false);

        if !allowed {
            panic!("Unauthorized Professor");
        }

        let mut records: Vec<Record> = env
            .storage()
            .instance()
            .get(&DataKey::Student(student.clone()))
            .unwrap_or(Vec::new(&env));

        records.push_back(Record {
            course,
            grade,
            attendance,
            internship_hours,
        });

        env.storage()
            .instance()
            .set(&DataKey::Student(student), &records);
    }

    pub fn get_records(env: Env, student: Address) -> Vec<Record> {

        env.storage()
            .instance()
            .get(&DataKey::Student(student))
            .unwrap_or(Vec::new(&env))
    }
}