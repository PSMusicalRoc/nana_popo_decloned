#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros,
    clippy::borrow_interior_mutable_const
)]

mod nana_grabbox;
mod nana_throws;
mod nana_specials;
mod nana_aerials;
mod nana_tilts;
//mod nana_final_smash_fix;

//mod nana_nspecial_status;
mod nana_sspecial_status;

#[skyline::main(name = "decloned_nana_popo")]
pub fn main() {
    // skyline::patching::Patch::in_text(0x710001f27c).data(0xe1u32).unwrap();
    // skyline::patching::Patch::in_text(0x710001f27d).data(0x03u32).unwrap();
    // skyline::patching::Patch::in_text(0x710001f27e).data(0x1fu32).unwrap();
    // skyline::patching::Patch::in_text(0x710001f27f).data(0x32u32).unwrap();




    nana_grabbox::install();
    nana_throws::install();
    nana_specials::install();
    nana_aerials::install();
    nana_tilts::install();
    //nana_final_smash_fix::install();
    //nana_nspecial_status::install();
    nana_sspecial_status::install();
}