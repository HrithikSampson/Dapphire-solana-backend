/*
 * This file contains all the arguments for each function in the program. 
 */

use solana_program::pubkey::Pubkey;


use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize,BorshDeserialize,Debug,Clone)]
pub struct ArgsApplicantAccountCreation {
    pub name: String,
    pub bio: String,
}


#[derive(BorshSerialize,BorshDeserialize,Debug,Clone)]
pub struct ArgsCreateEmployerAccount {
    pub name: String,
    pub organisation: String,
}

#[derive(BorshSerialize,BorshDeserialize,Debug,Clone)]
pub struct ArgsCreateJob {
    pub name: String,
    pub description: String,
    pub num_rounds: u8,
}

#[derive(BorshSerialize,BorshDeserialize,Debug,Clone)]
pub struct ArgsMoveApplicationStatus {
    pub job_id: u64,
    pub applicant_id: Pubkey,
}

#[derive(BorshSerialize,BorshDeserialize,Debug,Clone)]
pub struct ArgsRejectApplicationStatus {
    pub job_id: u64,
    pub applicant_id: Pubkey,
}

#[derive(BorshSerialize,BorshDeserialize,Debug,Clone)]
pub struct ArgsApplyJob {
    pub job_id: u64,
}