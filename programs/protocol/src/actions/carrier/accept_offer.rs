use anchor_lang::{prelude::*, solana_program::system_instruction};

use crate::{
    AcceptedOffer, Carrier, Error, Forwarder, OfferAccepted, Shipment, ShipmentOffer,
    ShipmentStatusUpdated,
};

#[derive(Accounts)]
pub struct AcceptOffer<'info> {
    #[account(init,
        seeds = [b"task", carrier.load().unwrap().creator.as_ref(), &carrier.load().unwrap().tasks_count.to_le_bytes()], bump,
        payer = payer,
        space = 8 + std::mem::size_of::<AcceptedOffer>()
    )]
    pub task: AccountLoader<'info, AcceptedOffer>,
    #[account(mut,
        seeds = [b"offer", carrier.load().unwrap().creator.as_ref(), &offer.load().unwrap().no.to_le_bytes()], bump,
        close = signer,
    )]
    pub offer: AccountLoader<'info, ShipmentOffer>,
    #[account(mut,
        seeds = [b"shipment", shipment.load().unwrap().shipper.as_ref(), &shipment.load().unwrap().no.to_le_bytes()], bump,
    )]
    pub shipment: AccountLoader<'info, Shipment>,
    #[account(mut,
        seeds = [b"forwarder", forwarder.load().unwrap().creator.as_ref()], bump,
    )]
    pub forwarder: AccountLoader<'info, Forwarder>,
    #[account(mut,
        seeds = [b"carrier", carrier.load().unwrap().creator.as_ref()], bump,
        constraint = carrier.load().unwrap().authority == *signer.key @ Error::SignerNotAnAuthority
    )]
    pub carrier: AccountLoader<'info, Carrier>,
    pub signer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AcceptOffer>) -> Result<()> {
    let collateral = {
        let task = &mut ctx.accounts.task.load_init()?;
        let offer = &mut ctx.accounts.offer.load_mut()?;
        let carrier = &mut ctx.accounts.carrier.load_mut()?;
        let shipment = &mut ctx.accounts.shipment.load_mut()?;

        // check if the shipment is for sale
        require_eq!(shipment.carrier, Pubkey::default(), Error::ShipmentSold);
        shipment.carrier = carrier.creator;
        shipment.status = 4;

        emit!(ShipmentStatusUpdated {
            shipment: ctx.accounts.shipment.key(),
            status: shipment.status,
        });

        // lock the funds as a collateral
        require_gte!(
            ctx.accounts.payer.get_lamports(),
            shipment.shipment.collateral,
            Error::NotEnoughFunds
        );

        **task = AcceptedOffer {
            owner: carrier.creator,
            details: offer.details,
            shipment: ctx.accounts.shipment.key(),
            no: carrier.tasks_count,
            accepted: Clock::get()?.unix_timestamp,
            reserved: [0; 4],
        };

        carrier.tasks_count += 1;

        emit!(OfferAccepted {
            from: shipment.forwarder,
            to: shipment.shipper,
            offer: ctx.accounts.offer.key(),
            shipment: ctx.accounts.shipment.key(),
        });

        shipment.shipment.collateral
    };

    // Invoke the transfer instruction
    let transfer_instruction = system_instruction::transfer(
        ctx.accounts.payer.key,
        &ctx.accounts.shipment.key(),
        collateral,
    );

    anchor_lang::solana_program::program::invoke_signed(
        &transfer_instruction,
        &[
            ctx.accounts.signer.to_account_info(),
            ctx.accounts.shipment.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
        &[],
    )?;

    Ok(())
}
