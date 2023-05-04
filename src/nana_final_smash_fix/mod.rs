use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;

#[acmd_script( agent = "nana", script = "game_final1l_nana", category = ACMD_GAME, low_priority )]
unsafe fn game_final1l_nana(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        macros::FT_SET_FINAL_SMASH_LIGHT(fighter, true);
        macros::FT_REMOVE_FINAL_AURA(fighter, false);
        PostureModule::scale(fighter.module_accessor, 20, 0);
        //0x1478c0(1872348090, 2.1);
        CAM_ZOOM_IN_arg5(0, 0);
        macros::FT_START_CUTIN(fighter);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
        macros::SLOW_OPPONENT(fighter, 10, 60);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::CAM_ZOOM_OUT(fighter);
    }
    frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        macros::CANCEL_FILL_SCREEN(fighter, 0, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICEBERG, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICEBERG_HIT, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_WHITEBEAR, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, false, -1);
    }
    frame(fighter.lua_state_agent, 52.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_FINAL_FLAG_FINAL_CAMERA);
    }
}

#[acmd_script( agent = "nana", script = "game_final1r_nana", category = ACMD_GAME, low_priority )]
unsafe fn game_final1r_nana(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        macros::FT_SET_FINAL_SMASH_LIGHT(fighter, true);
        macros::FT_REMOVE_FINAL_AURA(fighter, false);
        PostureModule::scale(fighter.module_accessor, 20, 0);
        0x1478c0(1872348090, 2.1);
        CAM_ZOOM_IN_arg5(0, 0);
        macros::FT_START_CUTIN(fighter);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
        macros::SLOW_OPPONENT(fighter, 10, 60);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::CAM_ZOOM_OUT(fighter);
    }
    frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        macros::CANCEL_FILL_SCREEN(fighter, 0, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICEBERG, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICEBERG_HIT, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_WHITEBEAR, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, false, -1);
    }
    frame(fighter.lua_state_agent, 52.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_FINAL_FLAG_FINAL_CAMERA);
    }
}

#[acmd_script( agent = "nana", script = "game_final2_nana", category = ACMD_GAME, low_priority )]
unsafe fn game_final2_nana(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        macros::FT_SET_FINAL_SMASH_LIGHT(fighter, true);
        macros::CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(fighter, 10, 60);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::FT_SET_FINAL_FEAR_FACE(fighter, 60);
            macros::REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final2.nuanmb"), true);
            macros::FT_START_CUTIN(fighter);
        }
        else{
        if macros::is_excute(fighter) {
            macros::FT_REMOVE_FINAL_AURA(fighter, false);
            PostureModule::scale(fighter.module_accessor, 5, 0);
            0x1478c0(1872348090, 2.1);
            CAM_ZOOM_IN_arg5(0, 0);
            macros::FT_START_CUTIN(fighter);
        }
        if get_value_float(fighter, *SO_VAR_FLOAT_LR) < 0.0 {
            if macros::is_excute(fighter) {
                camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 3, 0);
            }
            else{
            if macros::is_excute(fighter) {
                camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -3, 0);
            }
        }
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::CAM_ZOOM_OUT(fighter);
    }
}
}
frame(fighter.lua_state_agent, 51.0);
if macros::is_excute(fighter) {
macros::CANCEL_FILL_SCREEN(fighter, 0, 0);
ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICEBERG, false, -1);
ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICEBERG_HIT, false, -1);
ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_WHITEBEAR, false, -1);
ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, false, -1);
}
frame(fighter.lua_state_agent, 52.0);
if macros::is_excute(fighter) {
camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RESET);
WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_FINAL_FLAG_FINAL_CAMERA);
}
}

#[acmd_script( agent = "nana", script = "game_finalair1l_nana", category = ACMD_GAME, low_priority )]
unsafe fn game_finalair1l_nana(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        macros::FT_SET_FINAL_SMASH_LIGHT(fighter, true);
        macros::FT_REMOVE_FINAL_AURA(fighter, false);
        PostureModule::scale(fighter.module_accessor, 20, 0);
        0x1478c0(1872348090, 2.1);
        CAM_ZOOM_IN_arg5(0, 0);
        macros::FT_START_CUTIN(fighter);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
        macros::SLOW_OPPONENT(fighter, 10, 60);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::CAM_ZOOM_OUT(fighter);
    }
    frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        macros::CANCEL_FILL_SCREEN(fighter, 0, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICEBERG, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICEBERG_HIT, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_WHITEBEAR, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, false, -1);
    }
    frame(fighter.lua_state_agent, 52.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_FINAL_FLAG_FINAL_CAMERA);
    }
}

#[acmd_script( agent = "nana", script = "game_finalair1r_nana", category = ACMD_GAME, low_priority )]
unsafe fn game_finalair1r_nana(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        macros::FT_SET_FINAL_SMASH_LIGHT(fighter, true);
        macros::FT_REMOVE_FINAL_AURA(fighter, false);
        PostureModule::scale(fighter.module_accessor, 20, 0);
        0x1478c0(1872348090, 2.1);
        CAM_ZOOM_IN_arg5(0, 0);
        macros::FT_START_CUTIN(fighter);
        camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
        macros::SLOW_OPPONENT(fighter, 10, 60);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::CAM_ZOOM_OUT(fighter);
    }
    frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        macros::CANCEL_FILL_SCREEN(fighter, 0, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICEBERG, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICEBERG_HIT, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_WHITEBEAR, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, false, -1);
    }
    frame(fighter.lua_state_agent, 52.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_FINAL_FLAG_FINAL_CAMERA);
    }
}

#[acmd_script( agent = "nana", script = "game_finalair2_nana", category = ACMD_GAME, low_priority )]
unsafe fn game_finalair2_nana(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        macros::FT_SET_FINAL_SMASH_LIGHT(fighter, true);
        macros::CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(fighter, 10, 60);
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::FT_SET_FINAL_FEAR_FACE(fighter, 60);
            macros::REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final2.nuanmb"), true);
            macros::FT_START_CUTIN(fighter);
        }
        else{
        if macros::is_excute(fighter) {
            macros::FT_REMOVE_FINAL_AURA(fighter, false);
            PostureModule::scale(fighter.module_accessor, 5, 0);
            0x1478c0(1872348090, 2.1);
            CAM_ZOOM_IN_arg5(0, 0);
            macros::FT_START_CUTIN(fighter);
        }
        if get_value_float(fighter, *SO_VAR_FLOAT_LR) < 0.0 {
            if macros::is_excute(fighter) {
                camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 3, 0);
            }
            else{
            if macros::is_excute(fighter) {
                camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_OFFSET, -3, 0);
            }
        }
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::CAM_ZOOM_OUT(fighter);
    }
}
}
frame(fighter.lua_state_agent, 51.0);
if macros::is_excute(fighter) {
macros::CANCEL_FILL_SCREEN(fighter, 0, 0);
ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICEBERG, false, -1);
ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_ICEBERG_HIT, false, -1);
ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_WHITEBEAR, false, -1);
ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_POPO_GENERATE_ARTICLE_CONDOR, false, -1);
}
frame(fighter.lua_state_agent, 52.0);
if macros::is_excute(fighter) {
camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RESET);
WorkModule::on_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_FINAL_FLAG_FINAL_CAMERA);
}
}

pub fn install() {
    smashline::install_acmd_scripts!(
        game_final1l_nana,
        game_final1r_nana,
        game_final2_nana,
        game_finalair1l_nana,
        game_finalair1r_nana,
        game_finalair2_nana
    );
}