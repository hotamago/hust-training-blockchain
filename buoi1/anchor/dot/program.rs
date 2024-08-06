#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use anchor_lang::{prelude::*, solana_program};
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::{id, seahorse_util::*};
use std::{cell::RefCell, rc::Rc};

#[account]
#[derive(Debug)]
pub struct HopDong {
    pub owner: Pubkey,
    pub student: Pubkey,
    pub is_done: bool,
    pub price: u64,
    pub message: [u16; 128],
}

impl<'info, 'entrypoint> HopDong {
    pub fn load(
        account: &'entrypoint mut Box<Account<'info, Self>>,
        programs_map: &'entrypoint ProgramsMap<'info>,
    ) -> Mutable<LoadedHopDong<'info, 'entrypoint>> {
        let owner = account.owner.clone();
        let student = account.student.clone();
        let is_done = account.is_done.clone();
        let price = account.price;
        let message = Mutable::new(account.message.clone());

        Mutable::new(LoadedHopDong {
            __account__: account,
            __programs__: programs_map,
            owner,
            student,
            is_done,
            price,
            message,
        })
    }

    pub fn store(loaded: Mutable<LoadedHopDong>) {
        let mut loaded = loaded.borrow_mut();
        let owner = loaded.owner.clone();

        loaded.__account__.owner = owner;

        let student = loaded.student.clone();

        loaded.__account__.student = student;

        let is_done = loaded.is_done.clone();

        loaded.__account__.is_done = is_done;

        let price = loaded.price;

        loaded.__account__.price = price;

        let message = loaded.message.borrow().clone();

        loaded.__account__.message = message;
    }
}

#[derive(Debug)]
pub struct LoadedHopDong<'info, 'entrypoint> {
    pub __account__: &'entrypoint mut Box<Account<'info, HopDong>>,
    pub __programs__: &'entrypoint ProgramsMap<'info>,
    pub owner: Pubkey,
    pub student: Pubkey,
    pub is_done: bool,
    pub price: u64,
    pub message: Mutable<[u16; 128]>,
}

pub fn init_hopdong_handler<'info>(
    mut payer: SeahorseSigner<'info, '_>,
    mut owner: SeahorseSigner<'info, '_>,
    mut student: SeahorseSigner<'info, '_>,
    mut hopdong: Empty<Mutable<LoadedHopDong<'info, '_>>>,
    mut seed_sha256: u128,
    mut price: u64,
    mut message: [u16; 128],
) -> () {
    let mut hopdong = hopdong.account.clone();

    assign!(hopdong.borrow_mut().student, student.key());

    assign!(hopdong.borrow_mut().price, price);

    assign!(
        hopdong.borrow_mut().message,
        Mutable::<[u16; 128]>::new(message)
    );

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &student.key(),
            &owner.clone().key(),
            hopdong.borrow().price.clone(),
        ),
        &[
            student.to_account_info(),
            owner.clone().to_account_info(),
            student.programs.get("system_program").clone(),
        ],
    ).unwrap();
}

pub fn xacnhan_done_handler<'info>(
    mut payer: SeahorseSigner<'info, '_>,
    mut owner: SeahorseSigner<'info, '_>,
    mut student: SeahorseSigner<'info, '_>,
    mut hopdong: Mutable<LoadedHopDong<'info, '_>>,
) -> () {
    if !(owner.key() == hopdong.borrow().owner) {
        panic!("Hopdong owner not same!");
    }

    if !(student.key() == hopdong.borrow().student) {
        panic!("Hopdong student not same!");
    }

    if !(hopdong.borrow().is_done == false) {
        panic!("Hopdong already done!");
    }

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            &owner.key(),
            &student.clone().key(),
            hopdong.borrow().price.clone(),
        ),
        &[
            owner.to_account_info(),
            student.clone().to_account_info(),
            owner.programs.get("system_program").clone(),
        ],
    ).unwrap();
}
