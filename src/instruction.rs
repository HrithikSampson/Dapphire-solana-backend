use borsh::BorshDeserialize;
use solana_program::{program_error::ProgramError,pubkey::Pubkey};

pub mod arguments;
use crate::instruction::arguments::{
    ArgsCreateEmployerAccount,
    ArgsApplicantAccountCreation,
    ArgsCreateJob,
    ArgsMoveApplicationStatus,
    ArgsRejectApplicationStatus,
};

use self::arguments::ArgsApplyJob;

#[derive(Debug)]
pub enum Controller {
    CreateApplicantAccount {
        name: String,
        bio: String,
    },
    CreateEmployerAccount {
        name: String,
        organisation: String,
    },
    CreateJob {
        name: String,
        description: String,
        num_rounds: u8,
    },
    MoveApplicationStatus {
        job_id: u64,
        applicant_id: Pubkey,
    },
    RejectApplicationStatus {
        job_id: u64,
        applicant_id: Pubkey,
    },
    ApplyJob {
        job_id: u64,
    }
}

impl Controller {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&code, data) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        match code {
            0 => {
                let args = ArgsApplicantAccountCreation::try_from_slice(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::CreateApplicantAccount {
                    name: args.name,
                    bio: args.bio,
                })
            },
            1 => {
                let args = ArgsCreateEmployerAccount::try_from_slice(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::CreateEmployerAccount {
                    name: args.name,
                    organisation: args.organisation,
                })
            },
            2 => {
                let args = ArgsCreateJob::try_from_slice(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::CreateJob {
                    name: args.name,
                    description: args.description,
                    num_rounds: args.num_rounds,
                })
            },
            3 => {
                let args = ArgsMoveApplicationStatus::try_from_slice(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::MoveApplicationStatus {
                    job_id: args.job_id,
                    applicant_id: args.applicant_id,
                })
            },
            4 => {
                let args = ArgsRejectApplicationStatus::try_from_slice(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::RejectApplicationStatus {
                    job_id: args.job_id,
                    applicant_id: args.applicant_id,
                })
            },
            5 => {
                let args = ArgsApplyJob::try_from_slice(data)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;
                Ok(Self::ApplyJob { job_id: args.job_id })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
