mod actions;
mod data;
mod errors;
mod events;

use actions::*;
use data::*;
use errors::Error;
use events::*;

use anchor_lang::prelude::*;

declare_id!("D6na56Wd31BXMmMUziNTsScQUnUVe6QVmxrpDJG7azpT");

#[program]
pub mod protocol {
    use super::*;

    pub fn initialize_state(ctx: Context<InitializeState>) -> Result<()> {
        actions::initialize_state::handler(ctx)
    }

    pub fn register_shipper(ctx: Context<RegisterShipper>, name: Name) -> Result<()> {
        actions::shipper::register::handler(ctx, name)
    }

    pub fn register_forwarder(ctx: Context<RegisterForwarder>, name: Name) -> Result<()> {
        actions::forwarder::register::handler(ctx, name)
    }

    pub fn register_carrier(
        ctx: Context<RegisterCarrier>,
        name: Name,
        availability: Option<Availability>,
    ) -> Result<()> {
        actions::carrier::register::handler(ctx, name, availability)
    }

    pub fn create_shipment(
        ctx: Context<CreateShipment>,
        price: u64,
        name: Name,
        shipment: ShipmentData,
    ) -> Result<()> {
        actions::create_shipment::handler(ctx, price, name, shipment)
    }

    pub fn buy_shipment(ctx: Context<BuyShipment>) -> Result<()> {
        actions::buy_shipment::handler(ctx)
    }

    pub fn make_offer(ctx: Context<MakeOffer>, payment: u64, timeout: u32) -> Result<()> {
        actions::make_offer::handler(ctx, payment, timeout)
    }

    pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
        actions::accept_offer::handler(ctx)
    }

    pub fn open_channel(ctx: Context<OpenChannel>, key: Public) -> Result<()> {
        actions::channel::open_channel::handler(ctx, key)
    }

    pub fn send_message(ctx: Context<SendMessage>, key: Public, message: Message) -> Result<()> {
        actions::channel::send_message::handler(ctx, key, message)
    }

    pub fn confirm_delivery(ctx: Context<Confirm>) -> Result<()> {
        actions::shipper::confirm::handler(ctx)
    }
}
