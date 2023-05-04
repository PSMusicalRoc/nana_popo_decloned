use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use std::convert::TryInto;

/*pub const fn get_battle_object_from_id() -> usize {
    0x3ac540
}*/

#[skyline::from_offset(0x3ac540)]
pub fn get_battle_object_from_id(id: u32) -> *mut BattleObject;

#[acmd_script( agent = "nana", script = "game_specialairhistart_nana", category = ACMD_GAME, low_priority )]
unsafe fn nana_specialairhistart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_HI_FLAG_CHECK_COUPLE);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_HI_FLAG_CHECK_COUPLE);
    }
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 1);
    frame(fighter.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(fighter, 2);
    frame(fighter.lua_state_agent, 15.0);
    get_value_float(fighter.lua_state_agent, *FT_VAR_FLOAT_STICK_X_BACK);
    //WorkModule::get_param_float(fighter.module_accessor, 0, Hash40::new("param_special_hi"), Hash40::new("start_turn_cont_x"));
    let start_turn_cont_x = WorkModule::get_param_float(fighter.module_accessor, /*Hash40::new("param_special_hi")*/0x1086bc4a93, /*Hash40::new("start_turn_cont_x")*/0x11df3c6b30);
        //Hash40::new("param_special_hi"), Hash40::new("start_turn_cont_x"));
    if get_value_float(fighter.lua_state_agent, *FT_VAR_FLOAT_STICK_X_BACK) <  start_turn_cont_x {
        if macros::is_excute(fighter) {
            STICK_LR(fighter.lua_state_agent);
            UPDATE_ROT(fighter.lua_state_agent);
        }
    }
    // if(0x1478c0(32)){
    //     if macros::is_excute(fighter) {
    //         macros::STICK_LR(fighter);
    //         macros::UPDATE_ROT(fighter);
    //     }
    // }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_HI_FLAG_DETACH_PARTNER);
    }
}

#[acmd_script( agent = "nana", script = "game_specialn_nana", category = ACMD_GAME, low_priority )]
unsafe fn game_specialn_nana(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.4)
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_ENABLE_COUPLE);
    }
}

#[acmd_script( agent = "nana", script = "game_specialairn_nana", category = ACMD_GAME, low_priority )]
unsafe fn game_specialairn_nana(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.4)
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }

    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_ENABLE_COUPLE);
    }

    /*macros::ATTACK(fighter,
        0,
        0,
        Hash40::new("top"),
        3.0,
        361,
        0,
        0,
        0,
        1.44,
        0.0,
        0.0,
        0.8,
        Some(0.0),
        Some(0.0),
        Some(9.0),
        1.0,
        1.0,
        *ATTACK_SETOFF_KIND_OFF,
        *ATTACK_LR_CHECK_SPEED,
        false,
        0,
        0.0,
        0,
        true,
        true,
        false,
        false,
        false,
        *COLLISION_SITUATION_MASK_GA,
        *COLLISION_CATEGORY_MASK_ALL,
        *COLLISION_PART_MASK_ALL,
        false,
        Hash40::new("collision_attr_blaster_throw_down"),
        *ATTACK_SOUND_LEVEL_M,
        *COLLISION_SOUND_ATTR_FOX_BLASTER,
        *ATTACK_REGION_ENERGY);*/
}

#[acmd_script( agent = "popo_iceshot", script = "game_shoot", category = ACMD_GAME, low_priority )]
unsafe fn game_shoot(fighter: &mut L2CAgentBase) {
    println!("line1");
    let owner_id = WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER);
    let owner = get_battle_object_from_id(owner_id.try_into().unwrap());
    let is_nana = (*owner).kind == <i32 as TryInto<u32>>::try_into(*FIGHTER_KIND_NANA).unwrap();
    println!("ifstatement");
    if is_nana {
        println!("NANA ICE SHOT!!! (Popo)");
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, (*WEAPON_POPO_ICE_SHOT_ATTACK_ID_MAIN).try_into().unwrap(), 0, Hash40::new("top"), 1.0, 361, 50, 0, 5, 3.3, 0.0, 3.4, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, (*WEAPON_POPO_ICE_SHOT_ATTACK_ID_MAIN).try_into().unwrap(), 0, Hash40::new("top"), 1.0, 361, 50, 0, 5, 3.3, 0.0, 3.4, 0.0, Some(0.0), Some(4.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, true, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        }
    }
    else {
        println!("Popo Ice Shot");
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, (*WEAPON_POPO_ICE_SHOT_ATTACK_ID_MAIN).try_into().unwrap(), 0, Hash40::new("top"), 3.5, 361, 0, 0, 0, 3.3, 0.0, 3.4, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
            AttackModule::enable_safe_pos(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, (*WEAPON_POPO_ICE_SHOT_ATTACK_ID_MAIN).try_into().unwrap(), 0, Hash40::new("top"), 3.5, 361, 0, 0, 0, 3.3, 0.0, 3.4, 0.0, Some(0.0), Some(4.0), Some(0.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        }
    }
}

pub fn install() {
    //skyline::patching::Patch::in_text(0x501c4b4).data(10u32).unwrap();
    smashline::install_acmd_scripts!(
        //nana_specialairhistart
        game_specialairn_nana,
        game_specialn_nana,
        game_shoot
    );
}