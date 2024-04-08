use std::collections::HashMap;

use solana_program::pubkey::Pubkey;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize,BorshDeserialize,Debug,Clone,PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Status {
    Rejected,
    Applied,
    Round(u8),
    Accepted,
}

#[derive(BorshSerialize,BorshDeserialize,Debug,Clone)]
pub enum Account {
    Applicant(Applicant),
    Employer(Employer),
}


#[derive(BorshSerialize,BorshDeserialize,Debug,Clone)]
pub struct Employer{
    pub name: String,
    pub organisation: String,
}

#[derive(BorshSerialize,BorshDeserialize,Debug,Clone)]
pub struct Applicant{
    pub applicant_id: Pubkey,
    pub name: String,
    pub bio: String,
    pub applied_jobs: Vec<u64>,
}




#[derive(BorshSerialize,BorshDeserialize,Debug,Clone)]
pub struct Job {
    pub id: u64,
    pub num_rounds: u8,
    pub name: String,
    pub description: String,
    pub owner: Pubkey,
}

/*
 *
 * This is the main Object the Hiring App will interact with.
 * 
 */
#[derive(BorshSerialize,BorshDeserialize,Debug,Clone)]
pub struct DappHireService {
    pub job_status_list: HashMap<u64,HashMap<Status, Vec<Pubkey>>>,
    pub jobs: HashMap<u64,Job>,
    pub profiles: HashMap<Pubkey,Account>,
}

