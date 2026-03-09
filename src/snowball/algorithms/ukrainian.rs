//! Generated from ukrainian.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use crate::snowball::Among;
use crate::snowball::SnowballEnv;

#[derive(Clone)]
struct Context {
    i_p2: i32,
    i_pV: i32,
}

static A_0: &'static [Among<Context>; 4] = &[
    Among("вшись", -1, 1, None),
    Among("ившись", 0, 2, None),
    Among("вши", -1, 1, None),
    Among("ивши", 2, 2, None),
];

static A_1: &'static [Among<Context>; 7] = &[
    Among("сь", -1, 1, None),
    Among("лась", 0, 1, None),
    Among("всь", 0, 1, None),
    Among("тись", 0, 1, None),
    Among("ся", -1, 1, None),
    Among("вся", 4, 1, None),
    Among("тися", 4, 1, None),
];

static A_2: &'static [Among<Context>; 127] = &[
    Among("ому", -1, 2, None),
    Among("ььому", 0, 2, None),
    Among("ічному", 0, 1, None),
    Among("ичному", 0, 1, None),
    Among("уальному", 0, 1, None),
    Among("ійному", 0, 1, None),
    Among("аційному", 5, 1, None),
    Among("онному", 0, 1, None),
    Among("аціях", -1, 1, None),
    Among("іх", -1, 2, None),
    Among("їх", -1, 2, None),
    Among("их", -1, 2, None),
    Among("ічних", 11, 1, None),
    Among("ичних", 11, 1, None),
    Among("уальних", 11, 1, None),
    Among("ійних", 11, 1, None),
    Among("аційних", 15, 1, None),
    Among("онних", 11, 1, None),
    Among("ацією", -1, 1, None),
    Among("ацію", -1, 1, None),
    Among("ою", -1, 2, None),
    Among("ььою", 20, 2, None),
    Among("ічною", 20, 1, None),
    Among("ичною", 20, 1, None),
    Among("уальною", 20, 1, None),
    Among("ійною", 20, 1, None),
    Among("аційною", 25, 1, None),
    Among("онною", 20, 1, None),
    Among("ація", -1, 1, None),
    Among("учі", -1, 2, None),
    Among("ючі", -1, 2, None),
    Among("ічні", -1, 1, None),
    Among("ичні", -1, 1, None),
    Among("уальні", -1, 1, None),
    Among("увані", -1, 2, None),
    Among("овані", -1, 2, None),
    Among("ійні", -1, 1, None),
    Among("аційні", 36, 1, None),
    Among("онні", -1, 1, None),
    Among("оі", -1, 2, None),
    Among("ььоі", 39, 2, None),
    Among("ічноі", 39, 1, None),
    Among("ичноі", 39, 1, None),
    Among("уальноі", 39, 1, None),
    Among("ійноі", 39, 1, None),
    Among("аційноі", 44, 1, None),
    Among("онноі", 39, 1, None),
    Among("ації", -1, 1, None),
    Among("ої", -1, 2, None),
    Among("ььої", 48, 2, None),
    Among("ічної", 48, 1, None),
    Among("ичної", 48, 1, None),
    Among("уальної", 48, 1, None),
    Among("ійної", 48, 1, None),
    Among("аційної", 53, 1, None),
    Among("онної", 48, 1, None),
    Among("уча", -1, 2, None),
    Among("юча", -1, 2, None),
    Among("ічна", -1, 1, None),
    Among("ична", -1, 1, None),
    Among("уальна", -1, 1, None),
    Among("увана", -1, 2, None),
    Among("ована", -1, 2, None),
    Among("ійна", -1, 1, None),
    Among("аційна", 63, 1, None),
    Among("онна", -1, 1, None),
    Among("уче", -1, 2, None),
    Among("юче", -1, 2, None),
    Among("ічне", -1, 1, None),
    Among("ичне", -1, 1, None),
    Among("уальне", -1, 1, None),
    Among("уване", -1, 2, None),
    Among("оване", -1, 2, None),
    Among("ійне", -1, 1, None),
    Among("аційне", 73, 1, None),
    Among("онне", -1, 1, None),
    Among("аціями", -1, 1, None),
    Among("іми", -1, 2, None),
    Among("їми", -1, 2, None),
    Among("ими", -1, 2, None),
    Among("ічними", 79, 1, None),
    Among("ичними", 79, 1, None),
    Among("уальними", 79, 1, None),
    Among("ійними", 79, 1, None),
    Among("аційними", 83, 1, None),
    Among("онними", 79, 1, None),
    Among("ій", -1, 2, None),
    Among("ічній", 86, 1, None),
    Among("ичній", 86, 1, None),
    Among("уальній", 86, 1, None),
    Among("ійній", 86, 1, None),
    Among("аційній", 90, 1, None),
    Among("онній", 86, 1, None),
    Among("ий", -1, 2, None),
    Among("учий", 93, 2, None),
    Among("ючий", 93, 2, None),
    Among("вший", 93, 2, None),
    Among("ічний", 93, 1, None),
    Among("ичний", 93, 1, None),
    Among("уальний", 93, 1, None),
    Among("уваний", 93, 2, None),
    Among("ований", 93, 2, None),
    Among("ійний", 93, 1, None),
    Among("аційний", 102, 1, None),
    Among("онний", 93, 1, None),
    Among("ой", -1, 2, None),
    Among("аціям", -1, 1, None),
    Among("ім", -1, 2, None),
    Among("їм", -1, 2, None),
    Among("им", -1, 2, None),
    Among("ічним", 109, 1, None),
    Among("ичним", 109, 1, None),
    Among("уальним", 109, 1, None),
    Among("ійним", 109, 1, None),
    Among("аційним", 113, 1, None),
    Among("онним", 109, 1, None),
    Among("єго", -1, 2, None),
    Among("ого", -1, 2, None),
    Among("ьього", 117, 2, None),
    Among("ічного", 117, 1, None),
    Among("ичного", 117, 1, None),
    Among("уального", 117, 1, None),
    Among("ійного", 117, 1, None),
    Among("аційного", 122, 1, None),
    Among("онного", 117, 1, None),
    Among("увано", -1, 2, None),
    Among("овано", -1, 2, None),
];

static A_3: &'static [Among<Context>; 32] = &[
    Among("у", -1, 1, None),
    Among("єш", -1, 1, None),
    Among("иш", -1, 1, None),
    Among("уть", -1, 1, None),
    Among("ють", -1, 1, None),
    Among("ають", 4, 1, None),
    Among("ю", -1, 1, None),
    Among("ію", 6, 1, None),
    Among("аю", 6, 1, None),
    Among("є", -1, 1, None),
    Among("ла", -1, 1, None),
    Among("в", -1, 1, None),
    Among("ів", 11, 1, None),
    Among("ав", 11, 1, None),
    Among("ив", 11, 1, None),
    Among("єте", -1, 1, None),
    Among("іте", -1, 1, None),
    Among("ите", -1, 1, None),
    Among("йте", -1, 1, None),
    Among("ти", -1, 1, None),
    Among("іти", 19, 1, None),
    Among("ати", 19, 1, None),
    Among("ити", 19, 1, None),
    Among("ли", -1, 1, None),
    Among("й", -1, 1, None),
    Among("єм", -1, 1, None),
    Among("ім", -1, 1, None),
    Among("им", -1, 1, None),
    Among("ло", -1, 1, None),
    Among("ємо", -1, 1, None),
    Among("імо", -1, 1, None),
    Among("имо", -1, 1, None),
];

static A_4: &'static [Among<Context>; 29] = &[
    Among("у", -1, 1, None),
    Among("ях", -1, 1, None),
    Among("ах", -1, 1, None),
    Among("ю", -1, 1, None),
    Among("єю", 3, 1, None),
    Among("ею", 3, 1, None),
    Among("ою", 3, 1, None),
    Among("я", -1, 1, None),
    Among("є", -1, 1, None),
    Among("і", -1, 1, None),
    Among("еві", 9, 1, None),
    Among("ові", 9, 1, None),
    Among("ї", -1, 1, None),
    Among("а", -1, 1, None),
    Among("ів", -1, 1, None),
    Among("ев", -1, 1, None),
    Among("е", -1, 1, None),
    Among("и", -1, 1, None),
    Among("ями", 17, 1, None),
    Among("ами", 17, 1, None),
    Among("ей", -1, 1, None),
    Among("ям", -1, 1, None),
    Among("єм", -1, 1, None),
    Among("ам", -1, 1, None),
    Among("ем", -1, 1, None),
    Among("ом", -1, 1, None),
    Among("о", -1, 1, None),
    Among("тво", 26, 1, None),
    Among("ство", 27, 1, None),
];

static A_5: &'static [Among<Context>; 7] = &[
    Among("яр", -1, 1, None),
    Among("ар", -1, 1, None),
    Among("ач", -1, 1, None),
    Among("ник", -1, 1, None),
    Among("альник", 3, 1, None),
    Among("ельник", 3, 1, None),
    Among("івник", 3, 1, None),
];

static A_6: &'static [Among<Context>; 61] = &[
    Among("еньку", -1, 1, None),
    Among("усь", -1, 1, None),
    Among("ісь", -1, 1, None),
    Among("ось", -1, 1, None),
    Among("унь", -1, 1, None),
    Among("інь", -1, 1, None),
    Among("ень", -1, 1, None),
    Among("еня", -1, 1, None),
    Among("ічкі", -1, 1, None),
    Among("ечкі", -1, 1, None),
    Among("очечкі", 9, 1, None),
    Among("очкі", -1, 1, None),
    Among("інькі", -1, 1, None),
    Among("юсінькі", 12, 1, None),
    Among("ісінькі", 12, 1, None),
    Among("енькі", -1, 1, None),
    Among("есенькі", 15, 1, None),
    Among("онькі", -1, 1, None),
    Among("ічка", -1, 1, None),
    Among("ечка", -1, 1, None),
    Among("очечка", 19, 1, None),
    Among("очка", -1, 1, None),
    Among("інька", -1, 1, None),
    Among("юсінька", 22, 1, None),
    Among("ісінька", 22, 1, None),
    Among("енька", -1, 1, None),
    Among("есенька", 25, 1, None),
    Among("онька", -1, 1, None),
    Among("інка", -1, 1, None),
    Among("инка", -1, 1, None),
    Among("ічки", -1, 1, None),
    Among("ечки", -1, 1, None),
    Among("очечки", 31, 1, None),
    Among("очки", -1, 1, None),
    Among("іньки", -1, 1, None),
    Among("юсіньки", 34, 1, None),
    Among("ісіньки", 34, 1, None),
    Among("еньки", -1, 1, None),
    Among("есеньки", 37, 1, None),
    Among("оньки", -1, 1, None),
    Among("ік", -1, 1, None),
    Among("ик", -1, 1, None),
    Among("чик", 41, 1, None),
    Among("ок", -1, 1, None),
    Among("ічок", 43, 1, None),
    Among("ечок", 43, 1, None),
    Among("очечок", 45, 1, None),
    Among("очок", 43, 1, None),
    Among("ятко", -1, 1, None),
    Among("ітко", -1, 1, None),
    Among("атко", -1, 1, None),
    Among("ічко", -1, 1, None),
    Among("ечко", -1, 1, None),
    Among("очечко", 52, 1, None),
    Among("очко", -1, 1, None),
    Among("інько", -1, 1, None),
    Among("юсінько", 55, 1, None),
    Among("ісінько", 55, 1, None),
    Among("енько", -1, 1, None),
    Among("есенько", 58, 1, None),
    Among("онько", -1, 1, None),
];

static A_7: &'static [Among<Context>; 9] = &[
    Among("очеч", -1, 1, None),
    Among("ітк", -1, 1, None),
    Among("атк", -1, 1, None),
    Among("іньк", -1, 1, None),
    Among("юсіньк", 3, 1, None),
    Among("ісіньк", 3, 1, None),
    Among("еньк", -1, 1, None),
    Among("есеньк", 6, 1, None),
    Among("оньк", -1, 1, None),
];

static A_8: &'static [Among<Context>; 4] = &[
    Among("яння", -1, 1, None),
    Among("іння", -1, 1, None),
    Among("ання", -1, 1, None),
    Among("ення", -1, 1, None),
];

static A_9: &'static [Among<Context>; 23] = &[
    Among("іст", -1, 1, None),
    Among("уват", -1, 1, None),
    Among("оват", -1, 1, None),
    Among("ищ", -1, 1, None),
    Among("ість", -1, 1, None),
    Among("овість", 4, 1, None),
    Among("ність", 4, 1, None),
    Among("ниця", -1, 1, None),
    Among("ація", -1, 1, None),
    Among("ізація", 8, 1, None),
    Among("арня", -1, 1, None),
    Among("івня", -1, 1, None),
    Among("івщина", -1, 1, None),
    Among("ов", -1, 1, None),
    Among("ськ", -1, 1, None),
    Among("зьк", -1, 1, None),
    Among("ізм", -1, 1, None),
    Among("льн", -1, 1, None),
    Among("ельн", 17, 1, None),
    Among("ян", -1, 1, None),
    Among("івн", -1, 1, None),
    Among("ство", -1, 1, None),
    Among("цтво", -1, 1, None),
];

static A_10: &'static [Among<Context>; 4] = &[
    Among("іш", -1, 1, None),
    Among("ь", -1, 4, None),
    Among("л", -1, 2, None),
    Among("н", -1, 3, None),
];

static G_v: &'static [u8; 5] = &[33, 65, 8, 192, 208];

fn r_mark_regions(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.i_pV = env.limit;
    context.i_p2 = env.limit;
    let v_1 = env.cursor;
    'lab0: loop {
        if !env.go_out_grouping(G_v, 1072, 1111) {
            break 'lab0;
        }
        env.next_char();
        context.i_pV = env.cursor;
        if !env.go_in_grouping(G_v, 1072, 1111) {
            break 'lab0;
        }
        env.next_char();
        if !env.go_out_grouping(G_v, 1072, 1111) {
            break 'lab0;
        }
        env.next_char();
        if !env.go_in_grouping(G_v, 1072, 1111) {
            break 'lab0;
        }
        env.next_char();
        context.i_p2 = env.cursor;
        break 'lab0;
    }
    env.cursor = v_1;
    return true;
}

fn r_R2(env: &mut SnowballEnv, context: &mut Context) -> bool {
    return context.i_p2 <= env.cursor;
}

fn r_perfective_gerund(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 5 <= env.limit_backward
        || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 140 as u8
            && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 184 as u8))
    {
        return false;
    }

    among_var = env.find_among_b(A_0, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            'lab0: loop {
                let v_1 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"а") {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                if !env.eq_s_b(&"я") {
                    return false;
                }
                break 'lab0;
            }
            env.slice_del();
        }
        2 => {
            env.slice_del();
        }
        _ => (),
    }
    return true;
}

fn r_reflexive(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 3 <= env.limit_backward
        || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 140 as u8
            && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 143 as u8))
    {
        return false;
    }

    if env.find_among_b(A_1, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true;
}

fn r_adjective(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_2, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if !r_R2(env, context) {
                return false;
            }
            env.slice_del();
        }
        2 => {
            env.slice_del();
        }
        _ => (),
    }
    return true;
}

fn r_verb(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_3, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true;
}

fn r_noun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_4, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true;
}

fn r_professional(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_5, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true;
}

fn r_diminutive(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_6, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true;
}

fn r_diminutive_stem(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 5 <= env.limit_backward
        || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 135 as u8
            && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 186 as u8))
    {
        return false;
    }

    if env.find_among_b(A_7, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    env.slice_del();
    return true;
}

fn r_verbal_noun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 7 <= env.limit_backward
        || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 143 as u8)
    {
        return false;
    }

    if env.find_among_b(A_8, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R2(env, context) {
        return false;
    }
    env.slice_del();
    return true;
}

fn r_derivational(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if env.find_among_b(A_9, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    if !r_R2(env, context) {
        return false;
    }
    env.slice_del();
    return true;
}

fn r_tidy_up(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_10, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_del();
            env.ket = env.cursor;
            if !env.eq_s_b(&"н") {
                return false;
            }
            env.bra = env.cursor;
            if !env.eq_s_b(&"н") {
                return false;
            }
            env.slice_del();
        }
        2 => {
            'lab0: loop {
                let v_1 = env.limit - env.cursor;
                'lab1: loop {
                    if !env.eq_s_b(&"б") {
                        break 'lab1;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                'lab2: loop {
                    if !env.eq_s_b(&"п") {
                        break 'lab2;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                'lab3: loop {
                    if !env.eq_s_b(&"в") {
                        break 'lab3;
                    }
                    break 'lab0;
                }
                env.cursor = env.limit - v_1;
                if !env.eq_s_b(&"м") {
                    return false;
                }
                break 'lab0;
            }
            env.slice_del();
        }
        3 => {
            if !env.eq_s_b(&"н") {
                return false;
            }
            env.slice_del();
        }
        4 => {
            env.slice_del();
        }
        _ => (),
    }
    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context { i_p2: 0, i_pV: 0 };
    let mut b_found_professional: bool;
    let mut b_found_diminutive: bool;
    let v_1 = env.cursor;
    'lab0: loop {
        env.bra = env.cursor;
        if !env.eq_s(&"най") {
            break 'lab0;
        }
        env.ket = env.cursor;
        env.slice_del();
        break 'lab0;
    }
    env.cursor = v_1;
    b_found_diminutive = false;
    b_found_professional = false;
    r_mark_regions(env, context);
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    if env.cursor < context.i_pV {
        return false;
    }
    let v_2 = env.limit_backward;
    env.limit_backward = context.i_pV;
    let v_3 = env.limit - env.cursor;
    'lab1: loop {
        'lab2: loop {
            let v_4 = env.limit - env.cursor;
            'lab3: loop {
                if !r_perfective_gerund(env, context) {
                    break 'lab3;
                }
                break 'lab2;
            }
            env.cursor = env.limit - v_4;
            let v_5 = env.limit - env.cursor;
            'lab4: loop {
                if !r_reflexive(env, context) {
                    env.cursor = env.limit - v_5;
                    break 'lab4;
                }
                break 'lab4;
            }
            'lab5: loop {
                let v_6 = env.limit - env.cursor;
                'lab6: loop {
                    if !r_verbal_noun(env, context) {
                        break 'lab6;
                    }
                    break 'lab5;
                }
                env.cursor = env.limit - v_6;
                let v_7 = env.limit - env.cursor;
                'lab7: loop {
                    'lab8: loop {
                        let v_8 = env.limit - env.cursor;
                        'lab9: loop {
                            if !r_professional(env, context) {
                                break 'lab9;
                            }
                            b_found_professional = true;
                            break 'lab8;
                        }
                        env.cursor = env.limit - v_8;
                        if !r_diminutive(env, context) {
                            env.cursor = env.limit - v_7;
                            break 'lab7;
                        }
                        b_found_diminutive = true;
                        break 'lab8;
                    }
                    break 'lab7;
                }
                if b_found_professional {
                    break 'lab1;
                }
                'lab10: loop {
                    let v_9 = env.limit - env.cursor;
                    'lab11: loop {
                        if !r_adjective(env, context) {
                            break 'lab11;
                        }
                        break 'lab10;
                    }
                    env.cursor = env.limit - v_9;
                    'lab12: loop {
                        if !r_verb(env, context) {
                            break 'lab12;
                        }
                        break 'lab10;
                    }
                    env.cursor = env.limit - v_9;
                    if !r_noun(env, context) {
                        break 'lab1;
                    }
                    break 'lab10;
                }
                break 'lab5;
            }
            break 'lab2;
        }
        break 'lab1;
    }
    env.cursor = env.limit - v_3;
    let v_10 = env.limit - env.cursor;
    'lab13: loop {
        if !r_diminutive_stem(env, context) {
            env.cursor = env.limit - v_10;
            break 'lab13;
        }
        b_found_diminutive = true;
        break 'lab13;
    }
    if b_found_diminutive {
        env.limit_backward = v_2;
        return false;
    }
    if b_found_professional {
        env.limit_backward = v_2;
        return false;
    }
    let v_11 = env.limit - env.cursor;
    'lab14: loop {
        if !r_professional(env, context) {
            env.cursor = env.limit - v_11;
            break 'lab14;
        }
        break 'lab14;
    }
    let v_12 = env.limit - env.cursor;
    r_derivational(env, context);
    env.cursor = env.limit - v_12;
    let v_13 = env.limit - env.cursor;
    r_tidy_up(env, context);
    env.cursor = env.limit - v_13;
    env.limit_backward = v_2;
    env.cursor = env.limit_backward;
    return true;
}
