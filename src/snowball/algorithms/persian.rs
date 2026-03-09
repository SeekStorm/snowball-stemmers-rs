//! Generated from persian.sbl by Snowball 3.0.0 - https://snowballstem.org/

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
use crate::snowball::Among;
use crate::snowball::SnowballEnv;

#[derive(Clone)]
struct Context {
    i_temp_mark: i32,
    i_start_mark: i32,
    b_saw_inf_or_part: bool,
    b_saw_present_prefix: bool,
    b_freeze_pass: bool,
    b_protect_lex_an: bool,
    b_changed: bool,
}

static A_0: &'static [Among<Context>; 12] = &[
    Among("", -1, 7, None),
    Among(" ", 0, 6, None),
    Among("\u{0623}", 0, 4, None),
    Among("\u{0624}", 0, 5, None),
    Among("\u{0625}", 0, 4, None),
    Among("\u{0626}", 0, 2, None),
    Among("\u{0629}", 0, 3, None),
    Among("\u{0643}", 0, 1, None),
    Among("\u{064A}", 0, 2, None),
    Among("\u{06C1}", 0, 3, None),
    Among("\u{200C}", 0, 6, None),
    Among("\u{200D}", 0, 6, None),
];

static A_1: &'static [Among<Context>; 4] = &[
    Among("\u{0628}\u{06CC}", -1, 3, None),
    Among("\u{0645}\u{06CC}", -1, 2, None),
    Among("\u{0646}\u{0627}", -1, 3, None),
    Among("\u{0646}\u{0645}\u{06CC}", -1, 1, None),
];

static A_2: &'static [Among<Context>; 3] = &[
    Among("\u{0633}\u{062A}\u{0627}\u{0646}", -1, 1, None),
    Among("\u{0631}\u{0627}\u{0646}", -1, 1, None),
    Among("\u{0633}\u{0627}\u{0646}", -1, 1, None),
];

static A_3: &'static [Among<Context>; 9] = &[
    Among("\u{0622}\u{0644}\u{0645}\u{0627}\u{0646}", -1, 1, None),
    Among("\u{0627}\u{06CC}\u{0645}\u{0627}\u{0646}", -1, 1, None),
    Among("\u{067E}\u{06CC}\u{0645}\u{0627}\u{0646}", -1, 1, None),
    Among("\u{0631}\u{0645}\u{0627}\u{0646}", -1, 1, None),
    Among("\u{062F}\u{0631}\u{0645}\u{0627}\u{0646}", 3, 1, None),
    Among("\u{062A}\u{0647}\u{0631}\u{0627}\u{0646}", -1, 1, None),
    Among("\u{0627}\u{06CC}\u{0631}\u{0627}\u{0646}", -1, 1, None),
    Among("\u{0627}\u{0646}\u{0633}\u{0627}\u{0646}", -1, 1, None),
    Among("\u{0622}\u{0633}\u{0627}\u{0646}", -1, 1, None),
];

static A_4: &'static [Among<Context>; 3] = &[
    Among(
        "\u{0627}\u{0633}\u{0627}\u{062A}\u{06CC}\u{062F}",
        -1,
        3,
        None,
    ),
    Among("\u{0627}\u{062E}\u{0628}\u{0627}\u{0631}", -1, 1, None),
    Among("\u{062E}\u{0648}\u{0627}\u{0633}", -1, 2, None),
];

static A_5: &'static [Among<Context>; 9] = &[
    Among("\u{0645}", -1, 3, None),
    Among("\u{0627}\u{0645}", 0, 1, None),
    Among("\u{0645}\u{0627}\u{0646}", -1, 2, None),
    Among("\u{062A}\u{0627}\u{0646}", -1, 2, None),
    Among("\u{0634}\u{0627}\u{0646}", -1, 2, None),
    Among("\u{062A}", -1, 3, None),
    Among("\u{0627}\u{062A}", 5, 1, None),
    Among("\u{0634}", -1, 3, None),
    Among("\u{0627}\u{0634}", 7, 1, None),
];

static A_6: &'static [Among<Context>; 9] = &[
    Among("\u{06CC}\u{0646}", -1, 3, None),
    Among("\u{0627}\u{0646}", -1, 4, None),
    Among("\u{06CC}\u{0627}\u{0646}", 1, 1, None),
    Among("\u{06AF}\u{0627}\u{0646}", 1, 1, None),
    Among("\u{0627}\u{0646}\u{06CC}", -1, 1, None),
    Among("\u{0647}\u{0627}\u{06CC}\u{06CC}", -1, 2, None),
    Among("\u{0647}\u{0627}\u{06CC}", -1, 1, None),
    Among("\u{0647}\u{0627}", -1, 3, None),
    Among("\u{0627}\u{062A}", -1, 3, None),
];

static A_7: &'static [Among<Context>; 6] = &[
    Among("\u{0628}\u{0627}\u{0646}", -1, 2, None),
    Among("\u{062F}\u{0627}\u{0646}", -1, 1, None),
    Among("\u{06AF}\u{0627}\u{0647}", -1, 1, None),
    Among("\u{06CC}\u{06CC}", -1, 3, None),
    Among("\u{06AF}\u{06CC}", -1, 3, None),
    Among("\u{06CC}\u{062A}", -1, 3, None),
];

static A_8: &'static [Among<Context>; 2] = &[
    Among("\u{062A}\u{0631}\u{06CC}\u{0646}", -1, 1, None),
    Among("\u{062A}\u{0631}", -1, 2, None),
];

static A_9: &'static [Among<Context>; 6] = &[
    Among("\u{06CC}\u{0646}", -1, 2, None),
    Among("\u{0627}\u{0646}\u{0647}", -1, 1, None),
    Among("\u{0646}\u{0627}\u{06A9}", -1, 1, None),
    Among("\u{0645}\u{0646}\u{062F}", -1, 1, None),
    Among("\u{0648}\u{0627}\u{0631}", -1, 1, None),
    Among("\u{06AF}\u{0627}\u{0631}", -1, 1, None),
];

static A_10: &'static [Among<Context>; 10] = &[
    Among("\u{0645}", -1, 3, None),
    Among("\u{06CC}\u{0645}", 0, 2, None),
    Among("\u{0627}\u{06CC}\u{0645}", 1, 1, None),
    Among("\u{0627}\u{06CC}", -1, 2, None),
    Among("\u{0627}\u{0633}\u{062A}", -1, 1, None),
    Among("\u{062F}", -1, 3, None),
    Among("\u{0627}\u{0646}\u{062F}", 5, 1, None),
    Among("\u{06CC}\u{062F}", 5, 2, None),
    Among("\u{0627}\u{06CC}\u{062F}", 7, 1, None),
    Among("\u{0627}\u{0633}", -1, 2, None),
];

static A_11: &'static [Among<Context>; 5] = &[
    Among("\u{0631}\u{0641}\u{062A}\u{06CC}\u{0645}", -1, 1, None),
    Among("\u{0631}\u{0641}\u{062A}\u{0645}", -1, 1, None),
    Among("\u{0631}\u{0641}\u{062A}\u{06CC}", -1, 1, None),
    Among(
        "\u{0631}\u{0641}\u{062A}\u{0627}\u{0646}\u{062F}",
        -1,
        1,
        None,
    ),
    Among("\u{0631}\u{0641}\u{062A}\u{06CC}\u{062F}", -1, 1, None),
];

static A_12: &'static [Among<Context>; 6] = &[
    Among("\u{0645}", -1, 3, None),
    Among("\u{06CC}\u{0645}", 0, 2, None),
    Among("\u{0627}\u{0645}", 0, 2, None),
    Among("\u{062F}", -1, 3, None),
    Among("\u{0627}\u{0646}\u{062F}", 3, 1, None),
    Among("\u{06CC}\u{062F}", 3, 2, None),
];

static A_13: &'static [Among<Context>; 2] = &[
    Among("\u{0627}\u{0646}", -1, 2, None),
    Among("\u{0647}", -1, 1, None),
];

fn r_Normalize_Characters(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    'replab0: loop {
        let v_1 = env.cursor;
        'lab1: for _ in 0..1 {
            env.bra = env.cursor;
            among_var = env.find_among(A_0, context);
            env.ket = env.cursor;
            match among_var {
                1 => {
                    env.slice_from("\u{06A9}");
                }
                2 => {
                    env.slice_from("\u{06CC}");
                }
                3 => {
                    env.slice_from("\u{0647}");
                }
                4 => {
                    env.slice_from("\u{0627}");
                }
                5 => {
                    env.slice_from("\u{0648}");
                }
                6 => {
                    env.slice_del();
                }
                7 => {
                    if env.cursor >= env.limit {
                        break 'lab1;
                    }
                    env.next_char();
                }
                _ => (),
            }
            continue 'replab0;
        }
        env.cursor = v_1;
        break 'replab0;
    }
    return true;
}

fn r_Strip_Prefix_At_Start(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    context.i_temp_mark = env.cursor;
    if env.cursor > context.i_start_mark {
        return false;
    }
    env.cursor = context.i_start_mark;
    env.bra = env.cursor;
    among_var = env.find_among(A_1, context);
    if among_var == 0 {
        return false;
    }
    env.ket = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 4 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
            context.b_saw_present_prefix = true;
        }
        2 => {
            if (env.current.chars().count() as i32) <= 3 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
            context.b_saw_present_prefix = true;
        }
        3 => {
            if (env.current.chars().count() as i32) <= 3 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        _ => (),
    }
    if env.cursor > context.i_temp_mark {
        return false;
    }
    env.cursor = context.i_temp_mark;
    return true;
}

fn r_Protect_Lexical_AN(env: &mut SnowballEnv, context: &mut Context) -> bool {
    context.b_protect_lex_an = false;
    context.i_temp_mark = env.cursor;
    env.limit_backward = env.cursor;
    env.cursor = env.limit;
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        r_AN_LexHash(env, context);
        env.cursor = env.limit - v_1;
        break 'lab0;
    }
    if (env.cursor - 5 <= env.limit_backward
        || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 134 as u8)
    {
        return false;
    }

    if env.find_among_b(A_2, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    context.b_protect_lex_an = true;
    env.cursor = env.limit_backward;
    if env.cursor > context.i_temp_mark {
        return false;
    }
    env.cursor = context.i_temp_mark;
    return true;
}

fn r_AN_LexHash(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if (env.cursor - 7 <= env.limit_backward
        || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 134 as u8)
    {
        return false;
    }

    if env.find_among_b(A_3, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    context.b_protect_lex_an = true;
    return true;
}

fn r_N_Hash(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 7 <= env.limit_backward
        || env.current.as_bytes()[(env.cursor - 1) as usize] as u8 >> 5 != 5 as u8
        || ((688128 as i32 >> (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 & 0x1f))
            & 1)
            == 0)
    {
        return false;
    }

    among_var = env.find_among_b(A_4, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            env.slice_from("\u{062E}\u{0628}\u{0631}");
            context.b_changed = true;
        }
        2 => {
            env.slice_from("\u{062E}\u{0627}\u{0633}\u{0647}");
            context.b_changed = true;
        }
        3 => {
            env.slice_from("\u{0627}\u{0633}\u{062A}\u{0627}\u{062F}");
            context.b_changed = true;
        }
        _ => (),
    }
    return true;
}

fn r_V_Hash(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if !env.eq_s_b(&"\u{0627}\u{0633}\u{062A}") {
        return false;
    }
    env.bra = env.cursor;
    env.slice_from("\u{0647}\u{0633}\u{062A}");
    context.b_changed = true;
    return true;
}

fn r_Noun_Possessive(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_5, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 4 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        2 => {
            if (env.current.chars().count() as i32) <= 5 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        3 => {
            if (env.current.chars().count() as i32) <= 3 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        _ => (),
    }
    return true;
}

fn r_Noun_Plural(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_6, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 5 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        2 => {
            if (env.current.chars().count() as i32) <= 6 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        3 => {
            if (env.current.chars().count() as i32) <= 4 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        4 => {
            if context.b_protect_lex_an {
                return false;
            }
            if (env.current.chars().count() as i32) <= 4 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        _ => (),
    }
    return true;
}

fn r_Noun_Other(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_7, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 5 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
            context.b_freeze_pass = true;
        }
        2 => {
            if (env.current.chars().count() as i32) <= 5 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        3 => {
            if (env.current.chars().count() as i32) <= 4 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        _ => (),
    }
    return true;
}

fn r_Stem_Noun(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !r_N_Hash(env, context) {
                break 'lab1;
            }
            context.b_freeze_pass = true;
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab2: loop {
            if !r_Noun_Other(env, context) {
                break 'lab2;
            }
            context.b_freeze_pass = true;
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab3: loop {
            if !r_Noun_Plural(env, context) {
                break 'lab3;
            }
            context.b_freeze_pass = true;
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        if !r_Noun_Possessive(env, context) {
            return false;
        }
        context.b_freeze_pass = true;
        break 'lab0;
    }
    return true;
}

fn r_Adjective_Superlative_Comparative(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 3 <= env.limit_backward
        || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 134 as u8
            && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 177 as u8))
    {
        return false;
    }

    among_var = env.find_among_b(A_8, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 6 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        2 => {
            if (env.current.chars().count() as i32) <= 5 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        _ => (),
    }
    return true;
}

fn r_Adjective_Derivational(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_9, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 5 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        2 => {
            if (env.current.chars().count() as i32) <= 4 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        _ => (),
    }
    return true;
}

fn r_Adjective_Relative(env: &mut SnowballEnv, context: &mut Context) -> bool {
    env.ket = env.cursor;
    if !env.eq_s_b(&"\u{06CC}\u{06CC}") {
        return false;
    }
    env.bra = env.cursor;
    if (env.current.chars().count() as i32) <= 4 {
        return false;
    }
    env.slice_del();
    context.b_changed = true;
    return true;
}

fn r_Stem_Adjective(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !r_Adjective_Superlative_Comparative(env, context) {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab2: loop {
            if !r_Adjective_Derivational(env, context) {
                break 'lab2;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        if !r_Adjective_Relative(env, context) {
            return false;
        }
        break 'lab0;
    }
    return true;
}

fn r_Participle_Clitic_Tails(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    among_var = env.find_among_b(A_10, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 5 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        2 => {
            if (env.current.chars().count() as i32) <= 4 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        3 => {
            if (env.current.chars().count() as i32) <= 3 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
        }
        _ => (),
    }
    return true;
}

fn r_Verb_Person_Endings(env: &mut SnowballEnv, context: &mut Context) -> bool {
    //let mut among_var;
    env.ket = env.cursor;
    if env.find_among_b(A_11, context) == 0 {
        return false;
    }
    env.bra = env.cursor;
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        env.slice_from("\u{0631}\u{0641}\u{062A}");
        context.b_changed = true;
        break 'lab0;
    }
    return true;
}

fn r_Verb_Tense_Mood_Markers(env: &mut SnowballEnv, context: &mut Context) -> bool {
    let mut among_var;
    env.ket = env.cursor;
    if (env.cursor - 1 <= env.limit_backward
        || (env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 134 as u8
            && env.current.as_bytes()[(env.cursor - 1) as usize] as u8 != 135 as u8))
    {
        return false;
    }

    among_var = env.find_among_b(A_13, context);
    if among_var == 0 {
        return false;
    }
    env.bra = env.cursor;
    match among_var {
        1 => {
            if (env.current.chars().count() as i32) <= 3 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
            context.b_saw_inf_or_part = true;
        }
        2 => {
            if (env.current.chars().count() as i32) <= 4 {
                return false;
            }
            env.slice_del();
            context.b_changed = true;
            context.b_saw_inf_or_part = true;
        }
        _ => (),
    }
    return true;
}

fn r_Stem_Verb(env: &mut SnowballEnv, context: &mut Context) -> bool {
    'lab0: loop {
        let v_1 = env.limit - env.cursor;
        'lab1: loop {
            if !r_Participle_Clitic_Tails(env, context) {
                break 'lab1;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab2: loop {
            if !r_V_Hash(env, context) {
                break 'lab2;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        'lab3: loop {
            if !r_Verb_Person_Endings(env, context) {
                break 'lab3;
            }
            break 'lab0;
        }
        env.cursor = env.limit - v_1;
        if !r_Verb_Tense_Mood_Markers(env, context) {
            return false;
        }
        break 'lab0;
    }
    return true;
}

pub fn stem(env: &mut SnowballEnv) -> bool {
    let mut context = &mut Context {
        i_temp_mark: 0,
        i_start_mark: 0,
        b_saw_inf_or_part: false,
        b_saw_present_prefix: false,
        b_freeze_pass: false,
        b_protect_lex_an: false,
        b_changed: false,
    };
    let v_1 = env.cursor;
    r_Normalize_Characters(env, context);
    env.cursor = v_1;
    context.i_start_mark = env.cursor;
    'replab0: loop {
        let v_2 = env.cursor;
        'lab1: for _ in 0..1 {
            context.b_changed = false;
            if !r_Strip_Prefix_At_Start(env, context) {
                break 'lab1;
            }
            if !context.b_changed {
                break 'lab1;
            }
            continue 'replab0;
        }
        env.cursor = v_2;
        break 'replab0;
    }
    'replab2: loop {
        let v_3 = env.cursor;
        'lab3: for _ in 0..1 {
            context.b_changed = false;
            context.b_saw_inf_or_part = false;
            let v_4 = env.cursor;
            r_Protect_Lexical_AN(env, context);
            env.cursor = v_4;
            env.limit_backward = env.cursor;
            env.cursor = env.limit;
            context.b_freeze_pass = false;
            'lab4: loop {
                let v_5 = env.limit - env.cursor;
                let v_6 = env.limit - env.cursor;
                let v_7 = env.limit - env.cursor;
                r_Stem_Noun(env, context);
                env.cursor = env.limit - v_7;
                env.cursor = env.limit - v_6;
                'lab5: loop {
                    let v_8 = env.limit - env.cursor;
                    'lab6: loop {
                        if !context.b_freeze_pass {
                            break 'lab6;
                        }
                        break 'lab5;
                    }
                    env.cursor = env.limit - v_8;
                    'lab7: loop {
                        let v_9 = env.limit - env.cursor;
                        r_Stem_Adjective(env, context);
                        env.cursor = env.limit - v_9;
                        break 'lab7;
                    }
                    break 'lab5;
                }
                break 'lab4;
            }
            env.cursor = env.limit_backward;
            if !context.b_changed {
                break 'lab3;
            }
            continue 'replab2;
        }
        env.cursor = v_3;
        break 'replab2;
    }
    return true;
}
