pub mod instruction;
pub mod state;

use std::collections::HashMap;

use crate::instruction::Controller;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},  entrypoint, entrypoint::ProgramResult , msg, program_error::ProgramError, pubkey::Pubkey
};
use state::{Account, Applicant, DappHireService, Employer, Job, Status};
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {
    let instruction: Controller = Controller::unpack(instructions_data)?;
    let accounts_iter = &mut accounts.iter();
    let global_account = next_account_info(accounts_iter)?;
    let caller_account = next_account_info(accounts_iter)?;
    if caller_account.is_signer == false {
        msg!("Missing required Signature");
        return Err(ProgramError::MissingRequiredSignature);
    }
    let mut dapphire_service = DappHireService::try_from_slice(&global_account.data.borrow())?;
    let caller = caller_account.key;
    match instruction {
        Controller::CreateApplicantAccount {name, bio} => {
            let profile_option = dapphire_service.profiles.get(&caller);
            match profile_option {
                Some(_account) => Err(ProgramError::AccountAlreadyInitialized),
                None => {
                    let new_profile = Account::Applicant(Applicant {
                        applicant_id: caller.clone(),
                        name,
                        bio,
                        applied_jobs: Vec::new(),
                    });
                    dapphire_service.profiles.insert(caller.clone(), new_profile);
                    dapphire_service.serialize(&mut &mut global_account.data.borrow_mut()[..])?;
                    Ok(())
                }
            }
        },
        Controller::CreateEmployerAccount { name, organisation } => {
            let profile_option = dapphire_service.profiles.get(&caller);
            match profile_option {
                Some(_account) => {return Err(ProgramError::AccountAlreadyInitialized)},
                None => {
                    let new_profile = Account::Employer(Employer {
                        name,
                        organisation
                    });
                    dapphire_service.profiles.insert(caller.clone(), new_profile);
                    dapphire_service.serialize(&mut &mut global_account.data.borrow_mut()[..])?;
                    return Ok(());
                }
            }
        },
        Controller::CreateJob { name,description, num_rounds } => {
            let profile_option = dapphire_service.profiles.get(&caller);
            match profile_option {
                Some(Account::Applicant(_)) => {
                    return Err(ProgramError::BorshIoError("Create an Employer Account to create job".to_string()))
                },
                Some(Account::Employer(_)) => {
                    let len = dapphire_service.jobs.len() as u64;
                    let job = Job {
                        id: len,
                        num_rounds: num_rounds,
                        name: name,
                        description: description,
                        owner: caller.clone(),
                    };
                    dapphire_service.jobs.insert(len, job);
                    let mut status_map: HashMap<Status, Vec<Pubkey>> = HashMap::new(); 
                    status_map.insert(Status::Accepted,Vec::new());
                    status_map.insert(Status::Applied, Vec::new());
                    status_map.insert(Status::Rejected, Vec::new());
                    for i in 0..num_rounds {
                        status_map.insert(Status::Round(i),Vec::new());
                    }                    
                    dapphire_service.job_status_list.insert(len,status_map);
                    dapphire_service.serialize(&mut &mut global_account.data.borrow_mut()[..])?;
                    return Ok(());
                },
                None => {return Err(ProgramError::BorshIoError("Create an Employer Account to create job".to_string()))}
            }
        },
        Controller::MoveApplicationStatus { job_id, applicant_id } => {
            let job = dapphire_service.jobs.get(&job_id).ok_or(ProgramError::BorshIoError("Job Not Found!".to_string()))?;
            if let Some(account) = dapphire_service.profiles.get(&caller) {
                if let Account::Employer(_) = account {
                    let job_status = dapphire_service.job_status_list.entry(job_id)
                        .or_insert_with(HashMap::new);
                    if job.owner != *caller {
                        return Err(ProgramError::BorshIoError("You dont own the Job".to_string()));
                    }
                    let jobs = &mut job_status.clone();
                    let status_list = job_status.entry(Status::Applied).or_default();
                    let rejected_list = jobs.entry(Status::Rejected).or_default();
                    if status_list.contains(&applicant_id) {
                        // Move from Applied to Round(1)
                        status_list.retain(|x| *x != applicant_id);
                        job_status.entry(Status::Round(1)).or_default().push(applicant_id);
                        dapphire_service.serialize(&mut &mut global_account.data.borrow_mut()[..])?;
                        return Ok(());
                    }
                    else if rejected_list.contains(&applicant_id) {
                        return Err(ProgramError::BorshIoError("Application already rejected!".to_string()));
                    } else {
                        // Check if the applicant is already in a round
                        for round in 1..=job.num_rounds {
                            let status_list = job_status.entry(Status::Round(round)).or_default();
                            if status_list.contains(&applicant_id) {
                                // Move to the next round or accept
                                if round == job.num_rounds {
                                    // Accept the applicant
                                    status_list.retain(|x| *x != applicant_id);
                                    job_status.entry(Status::Accepted).or_default().push(applicant_id);
                                } else {
                                    // Move to the next round
                                    status_list.retain(|x| *x != applicant_id);
                                    job_status.entry(Status::Round(round + 1)).or_default().push(applicant_id);
                                }
                                break;
                            }
                        }
                        dapphire_service.serialize(&mut &mut global_account.data.borrow_mut()[..])?;
                        return Ok(());
                    }
                }
                else {
                    Err(ProgramError::BorshIoError("Applicants are not allowed to do this".to_string()))
                }
            }
            else {
                return Err(ProgramError::BorshIoError("You have to create an employer account to move application".to_string()));
            }
        },
        Controller::RejectApplicationStatus { job_id, applicant_id } => {
            let job = dapphire_service.jobs.get(&job_id).ok_or(ProgramError::BorshIoError("Job not found".to_string()))?;
            if job.owner != *caller {
                return Err(ProgramError::BorshIoError("You dont own the Job".to_string()));
            }
            if let Some(account) = dapphire_service.profiles.get(caller) {
                if let Account::Employer(_) = account {
                    let job_status = dapphire_service.job_status_list.entry(job_id)
                        .or_insert_with(HashMap::new);
                    let jobs = &mut job_status.clone();
                    let status_list = jobs.entry(Status::Applied).or_default();
                    let rejected_list = job_status.entry(Status::Rejected).or_default();
                    if status_list.contains(&applicant_id) {
                        status_list.retain(|x| *x != applicant_id);
                        job_status.entry(Status::Rejected).or_default().push(applicant_id);
                    } 
                    else if rejected_list.contains(&applicant_id) {
                        return Err(ProgramError::BorshIoError("Application already rejected!".to_string()));
                    }
                    else {
                        for round in 1..=job.num_rounds {
                            let status_list = job_status.entry(Status::Round(round)).or_default();
                            if status_list.contains(caller){
                                status_list.retain(|x| *x != applicant_id);
                                job_status.entry(Status::Rejected).or_default().push(applicant_id);
                            }
                        }
                    }
                    dapphire_service.serialize(&mut &mut global_account.data.borrow_mut()[..])?;
                    return Ok(());
                } else {
                    return Err(ProgramError::BorshIoError("Applicants are not allowed to do this".to_string()))
                }
            } else {
                return Err(ProgramError::BorshIoError("You have to create an employer account to move application".to_string()));
            }
        },
        Controller::ApplyJob { job_id } => {
            let job = dapphire_service.jobs.get(&job_id);
            let profile = dapphire_service.profiles.get_mut(caller);
            return match job {
                None => Err(ProgramError::BorshIoError("No job exists with this jobID".to_string())),
                Some(job_apply) => return match profile {
                    Some(account) => match account {
                        Account::Applicant(applicant) => {
                            applicant.applied_jobs.push(job_apply.id);
                            let status_lists = dapphire_service.job_status_list.entry(job_id).or_insert_with(HashMap::new);
                            if !status_lists.values().any(|list| list.contains(caller)) {
                                dapphire_service.job_status_list.insert(job_id,HashMap::new());
                                return Err(ProgramError::BorshIoError("Already Applied!".to_string()));
                            }
                            
                            let status_map = dapphire_service.job_status_list.get_mut(&job_id).ok_or_else(|| ProgramError::BorshIoError("weird error! job status list not filled".to_string())).unwrap();
                            applicant.applied_jobs.push(job_apply.id);
                            let mut vec_principal = status_map.get(&Status::Applied).unwrap().clone();
                            vec_principal.push(*caller);
                            status_map.insert(Status::Applied,vec_principal);
                            dapphire_service.serialize(&mut &mut global_account.data.borrow_mut()[..])?;
                            Ok(())
                        },
                        Account::Employer(_) => Err(ProgramError::BorshIoError("Employer Account can't apply".to_string())),
                    },
                    None => Err(ProgramError::BorshIoError("Applicant Profile Doesnt exist for this Account".to_string()))
                }
            };
        },
    }
}