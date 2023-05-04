use smash::lib::lua_const::*;
use smash::lib::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;

#[smashline::status_script(agent = "nana", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn special_n_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("Beginning of nana status func");
    let boma : *mut BattleObjectModuleAccessor = fighter.module_accessor;
    let situation : i32 = StatusModule::situation_kind(boma);
    println!("if statement");
    if situation == *SITUATION_KIND_GROUND {
        println!("nspecial-ground");
        //KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(boma, Hash40::new("special_n"),
            0.0, 1.0, false, 0.0, false, false);
    }
    else {
        println!("nspecial-air");
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_AIR_N);
        fighter.set_situation(L2CValue::I32(*SITUATION_KIND_AIR));
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(boma, Hash40::new("special_air_n"),
            0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::set_int(boma, *FIGHTER_POPO_STATUS_SPECIAL_N_WORK_INT_SITUATION, fighter.global_table[22].get_i32());
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_loop_status as *const () as _))
}


unsafe fn special_n_loop_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma : *mut BattleObjectModuleAccessor = fighter.module_accessor;
    //let is_cancel : bool = CancelModule::is_enable_cancel(boma);

    const MAX_COUNT_ICESHOT : u64 = 10;

    println!("New Frame: Before CancelModule::is_enable_cancel()");

    if CancelModule::is_enable_cancel(fighter.module_accessor) && (fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool()) {
        return 1.into()
    }

    println!("(!is_cancel) = true");
    if WorkModule::is_flag(boma, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE) {
        println!("is_flag(GenerateArticle) = true");
        let count: u64 = ArticleModule::get_active_num(boma, *FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT);
        if count < MAX_COUNT_ICESHOT {
            println!("Generating Article...");
            ArticleModule::generate_article(boma, *FIGHTER_POPO_GENERATE_ARTICLE_ICESHOT, false, -1);
            WorkModule::on_flag(boma, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE_SUCCESS);
        }
        else {
            WorkModule::off_flag(boma, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE_SUCCESS);
        }
        println!("Turning off GenerateArticle flag...");
        WorkModule::off_flag(boma, *FIGHTER_POPO_STATUS_SPECIAL_N_FLAG_GENERATE_ARTICLE);
    }


    if fighter.global_table[22].get_i32() == *SITUATION_KIND_GROUND {
        println!("SituationKind == GROUND");
        if fighter.global_table[22].get_i32() == *SITUATION_KIND_AIR {
            println!("Changed from Ground to Air");
            fighter.change_status(L2CValue::I32(*FIGHTER_STATUS_KIND_FALL), L2CValue::Bool(false));
        }
        else {
            println!("Stayed on the ground");
            if !MotionModule::is_end(boma) {
                println!("End Frame\n");
                return 0.into()
            }
            println!("Ended on the ground... transitioning into STATUS_KIND_WAIT");
            fighter.change_status(L2CValue::I32(*FIGHTER_STATUS_KIND_WAIT), L2CValue::Bool(false));
        }
    }
    else {
        println!("SituationKind == AIR");
        if fighter.global_table[22].get_i32() == *SITUATION_KIND_GROUND {
            if CancelModule::is_enable_cancel(boma) {
                println!("Changed from Ground to Air - cancelable");
                fighter.change_status(L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING), L2CValue::Bool(false));
            }
            else {
                println!("Changed from Ground to Air - non-cancelable");
                fighter.change_status(L2CValue::I32(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL), L2CValue::Bool(false));
            }
        }
        else {
            println!("Stayed in the air");
            if !MotionModule::is_end(boma) {
                println!("End Frame\n");
                return 0.into()
            }
            println!("Ended in the air... transitioning into STATUS_KIND_FALL");
            fighter.change_status(L2CValue::I32(*FIGHTER_STATUS_KIND_FALL), L2CValue::Bool(false));
        }
    }

    println!("End Frame\n");
    L2CValue::I32(1)
}

pub fn install() {
    smashline::install_status_scripts!(
        special_n_status
    );
}