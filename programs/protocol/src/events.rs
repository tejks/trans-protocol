use anchor_lang::prelude::*;

#[event]
pub struct ShipmentTransferred {
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub shipment: Pubkey,
    pub forwarded: Pubkey,
}

#[event]
pub struct ShipmentCreated {
    pub shipper: Pubkey,
    pub shipment: Pubkey,
}

#[event]
pub struct OfferMade {
    pub from: Pubkey,
    pub to: Pubkey,
    pub offer: Pubkey,
    pub shipment: Pubkey,
}

#[event]
pub struct OfferAccepted {
    pub from: Pubkey,
    pub to: Pubkey,
    pub offer: Pubkey,
    pub shipment: Pubkey,
}
