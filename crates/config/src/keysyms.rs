#![allow(non_camel_case_types)]

use anyhow::Result;
use std::convert::TryFrom;

use crate::config_loader::ConfigError;

/// map of all the keysyms available. you can see a full list here:
/// https://www.cl.cam.ac.uk/~mgk25/ucs/keysymdef.h
#[derive(Debug, Clone, PartialEq)]
pub enum Keysym {
    XK_BackSpace = 0xff08,
    XK_Return = 0xff0d,
    XK_0 = 0x030,
    XK_1 = 0x031,
    XK_2 = 0x032,
    XK_3 = 0x033,
    XK_4 = 0x034,
    XK_5 = 0x035,
    XK_6 = 0x036,
    XK_7 = 0x037,
    XK_8 = 0x038,
    XK_9 = 0x039,
    XK_A = 0x041,
    XK_B = 0x042,
    XK_C = 0x043,
    XK_D = 0x044,
    XK_E = 0x045,
    XK_F = 0x046,
    XK_G = 0x047,
    XK_H = 0x048,
    XK_I = 0x049,
    XK_J = 0x04a,
    XK_K = 0x04b,
    XK_L = 0x04c,
    XK_M = 0x04d,
    XK_N = 0x04e,
    XK_O = 0x04f,
    XK_P = 0x050,
    XK_Q = 0x051,
    XK_R = 0x052,
    XK_S = 0x053,
    XK_T = 0x054,
    XK_U = 0x055,
    XK_V = 0x056,
    XK_W = 0x057,
    XK_X = 0x058,
    XK_Y = 0x059,
    XK_Z = 0x05a,
    XK_a = 0x061,
    XK_b = 0x062,
    XK_c = 0x063,
    XK_d = 0x064,
    XK_e = 0x065,
    XK_f = 0x066,
    XK_g = 0x067,
    XK_h = 0x068,
    XK_i = 0x069,
    XK_j = 0x06a,
    XK_k = 0x06b,
    XK_l = 0x06c,
    XK_m = 0x06d,
    XK_n = 0x06e,
    XK_o = 0x06f,
    XK_p = 0x070,
    XK_q = 0x071,
    XK_r = 0x072,
    XK_s = 0x073,
    XK_t = 0x074,
    XK_u = 0x075,
    XK_v = 0x076,
    XK_w = 0x077,
    XK_x = 0x078,
    XK_y = 0x079,
    XK_z = 0x07a,
}

impl Keysym {
    /// convert a keysym to its canonical name, this is used to map the firmware keycodes to
    /// keysymbols which are used to match on runtime against Keypress Events
    pub fn canonical_name(&self) -> &str {
        match self {
            Keysym::XK_BackSpace => "XK_BackSpace",
            Keysym::XK_Return => "XK_Return",
            Keysym::XK_0 => "XK_0",
            Keysym::XK_1 => "XK_1",
            Keysym::XK_2 => "XK_2",
            Keysym::XK_3 => "XK_3",
            Keysym::XK_4 => "XK_4",
            Keysym::XK_5 => "XK_5",
            Keysym::XK_6 => "XK_6",
            Keysym::XK_7 => "XK_7",
            Keysym::XK_8 => "XK_8",
            Keysym::XK_9 => "XK_9",
            Keysym::XK_A => "XK_A",
            Keysym::XK_B => "XK_B",
            Keysym::XK_C => "XK_C",
            Keysym::XK_D => "XK_D",
            Keysym::XK_E => "XK_E",
            Keysym::XK_F => "XK_F",
            Keysym::XK_G => "XK_G",
            Keysym::XK_H => "XK_H",
            Keysym::XK_I => "XK_I",
            Keysym::XK_J => "XK_J",
            Keysym::XK_K => "XK_K",
            Keysym::XK_L => "XK_L",
            Keysym::XK_M => "XK_M",
            Keysym::XK_N => "XK_N",
            Keysym::XK_O => "XK_O",
            Keysym::XK_P => "XK_P",
            Keysym::XK_Q => "XK_Q",
            Keysym::XK_R => "XK_R",
            Keysym::XK_S => "XK_S",
            Keysym::XK_T => "XK_T",
            Keysym::XK_U => "XK_U",
            Keysym::XK_V => "XK_V",
            Keysym::XK_W => "XK_W",
            Keysym::XK_X => "XK_X",
            Keysym::XK_Y => "XK_Y",
            Keysym::XK_Z => "XK_Z",
            Keysym::XK_a => "XK_a",
            Keysym::XK_b => "XK_b",
            Keysym::XK_c => "XK_c",
            Keysym::XK_d => "XK_d",
            Keysym::XK_e => "XK_e",
            Keysym::XK_f => "XK_f",
            Keysym::XK_g => "XK_g",
            Keysym::XK_h => "XK_h",
            Keysym::XK_i => "XK_i",
            Keysym::XK_j => "XK_j",
            Keysym::XK_k => "XK_k",
            Keysym::XK_l => "XK_l",
            Keysym::XK_m => "XK_m",
            Keysym::XK_n => "XK_n",
            Keysym::XK_o => "XK_o",
            Keysym::XK_p => "XK_p",
            Keysym::XK_q => "XK_q",
            Keysym::XK_r => "XK_r",
            Keysym::XK_s => "XK_s",
            Keysym::XK_t => "XK_t",
            Keysym::XK_u => "XK_u",
            Keysym::XK_v => "XK_v",
            Keysym::XK_w => "XK_w",
            Keysym::XK_x => "XK_x",
            Keysym::XK_y => "XK_y",
            Keysym::XK_z => "XK_z",
        }
    }
}

impl TryFrom<xkbcommon::xkb::Keysym> for Keysym {
    type Error = ();

    fn try_from(k: xkbcommon::xkb::Keysym) -> Result<Self, Self::Error> {
        match k {
            x if u32::from(x) == Keysym::XK_BackSpace as u32 => Ok(Keysym::XK_BackSpace),
            x if u32::from(x) == Keysym::XK_Return as u32 => Ok(Keysym::XK_Return),
            x if u32::from(x) == Keysym::XK_0 as u32 => Ok(Keysym::XK_0),
            x if u32::from(x) == Keysym::XK_1 as u32 => Ok(Keysym::XK_1),
            x if u32::from(x) == Keysym::XK_2 as u32 => Ok(Keysym::XK_2),
            x if u32::from(x) == Keysym::XK_3 as u32 => Ok(Keysym::XK_3),
            x if u32::from(x) == Keysym::XK_4 as u32 => Ok(Keysym::XK_4),
            x if u32::from(x) == Keysym::XK_5 as u32 => Ok(Keysym::XK_5),
            x if u32::from(x) == Keysym::XK_6 as u32 => Ok(Keysym::XK_6),
            x if u32::from(x) == Keysym::XK_7 as u32 => Ok(Keysym::XK_7),
            x if u32::from(x) == Keysym::XK_8 as u32 => Ok(Keysym::XK_8),
            x if u32::from(x) == Keysym::XK_9 as u32 => Ok(Keysym::XK_9),
            x if u32::from(x) == Keysym::XK_A as u32 => Ok(Keysym::XK_A),
            x if u32::from(x) == Keysym::XK_B as u32 => Ok(Keysym::XK_B),
            x if u32::from(x) == Keysym::XK_C as u32 => Ok(Keysym::XK_C),
            x if u32::from(x) == Keysym::XK_D as u32 => Ok(Keysym::XK_D),
            x if u32::from(x) == Keysym::XK_E as u32 => Ok(Keysym::XK_E),
            x if u32::from(x) == Keysym::XK_F as u32 => Ok(Keysym::XK_F),
            x if u32::from(x) == Keysym::XK_G as u32 => Ok(Keysym::XK_G),
            x if u32::from(x) == Keysym::XK_H as u32 => Ok(Keysym::XK_H),
            x if u32::from(x) == Keysym::XK_I as u32 => Ok(Keysym::XK_I),
            x if u32::from(x) == Keysym::XK_J as u32 => Ok(Keysym::XK_J),
            x if u32::from(x) == Keysym::XK_K as u32 => Ok(Keysym::XK_K),
            x if u32::from(x) == Keysym::XK_L as u32 => Ok(Keysym::XK_L),
            x if u32::from(x) == Keysym::XK_M as u32 => Ok(Keysym::XK_M),
            x if u32::from(x) == Keysym::XK_N as u32 => Ok(Keysym::XK_N),
            x if u32::from(x) == Keysym::XK_O as u32 => Ok(Keysym::XK_O),
            x if u32::from(x) == Keysym::XK_P as u32 => Ok(Keysym::XK_P),
            x if u32::from(x) == Keysym::XK_Q as u32 => Ok(Keysym::XK_Q),
            x if u32::from(x) == Keysym::XK_R as u32 => Ok(Keysym::XK_R),
            x if u32::from(x) == Keysym::XK_S as u32 => Ok(Keysym::XK_S),
            x if u32::from(x) == Keysym::XK_T as u32 => Ok(Keysym::XK_T),
            x if u32::from(x) == Keysym::XK_U as u32 => Ok(Keysym::XK_U),
            x if u32::from(x) == Keysym::XK_V as u32 => Ok(Keysym::XK_V),
            x if u32::from(x) == Keysym::XK_W as u32 => Ok(Keysym::XK_W),
            x if u32::from(x) == Keysym::XK_X as u32 => Ok(Keysym::XK_X),
            x if u32::from(x) == Keysym::XK_Y as u32 => Ok(Keysym::XK_Y),
            x if u32::from(x) == Keysym::XK_Z as u32 => Ok(Keysym::XK_Z),
            x if u32::from(x) == Keysym::XK_a as u32 => Ok(Keysym::XK_a),
            x if u32::from(x) == Keysym::XK_b as u32 => Ok(Keysym::XK_b),
            x if u32::from(x) == Keysym::XK_c as u32 => Ok(Keysym::XK_c),
            x if u32::from(x) == Keysym::XK_d as u32 => Ok(Keysym::XK_d),
            x if u32::from(x) == Keysym::XK_e as u32 => Ok(Keysym::XK_e),
            x if u32::from(x) == Keysym::XK_f as u32 => Ok(Keysym::XK_f),
            x if u32::from(x) == Keysym::XK_g as u32 => Ok(Keysym::XK_g),
            x if u32::from(x) == Keysym::XK_h as u32 => Ok(Keysym::XK_h),
            x if u32::from(x) == Keysym::XK_i as u32 => Ok(Keysym::XK_i),
            x if u32::from(x) == Keysym::XK_j as u32 => Ok(Keysym::XK_j),
            x if u32::from(x) == Keysym::XK_k as u32 => Ok(Keysym::XK_k),
            x if u32::from(x) == Keysym::XK_l as u32 => Ok(Keysym::XK_l),
            x if u32::from(x) == Keysym::XK_m as u32 => Ok(Keysym::XK_m),
            x if u32::from(x) == Keysym::XK_n as u32 => Ok(Keysym::XK_n),
            x if u32::from(x) == Keysym::XK_o as u32 => Ok(Keysym::XK_o),
            x if u32::from(x) == Keysym::XK_p as u32 => Ok(Keysym::XK_p),
            x if u32::from(x) == Keysym::XK_q as u32 => Ok(Keysym::XK_q),
            x if u32::from(x) == Keysym::XK_r as u32 => Ok(Keysym::XK_r),
            x if u32::from(x) == Keysym::XK_s as u32 => Ok(Keysym::XK_s),
            x if u32::from(x) == Keysym::XK_t as u32 => Ok(Keysym::XK_t),
            x if u32::from(x) == Keysym::XK_u as u32 => Ok(Keysym::XK_u),
            x if u32::from(x) == Keysym::XK_v as u32 => Ok(Keysym::XK_v),
            x if u32::from(x) == Keysym::XK_w as u32 => Ok(Keysym::XK_w),
            x if u32::from(x) == Keysym::XK_x as u32 => Ok(Keysym::XK_x),
            x if u32::from(x) == Keysym::XK_y as u32 => Ok(Keysym::XK_y),
            x if u32::from(x) == Keysym::XK_z as u32 => Ok(Keysym::XK_z),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Keysym {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Keysym::XK_0 => write!(f, "0"),
            Keysym::XK_1 => write!(f, "1"),
            Keysym::XK_2 => write!(f, "2"),
            Keysym::XK_3 => write!(f, "3"),
            Keysym::XK_4 => write!(f, "4"),
            Keysym::XK_5 => write!(f, "5"),
            Keysym::XK_6 => write!(f, "6"),
            Keysym::XK_7 => write!(f, "7"),
            Keysym::XK_8 => write!(f, "8"),
            Keysym::XK_9 => write!(f, "9"),
            Keysym::XK_A => write!(f, "A"),
            Keysym::XK_B => write!(f, "B"),
            Keysym::XK_C => write!(f, "C"),
            Keysym::XK_D => write!(f, "D"),
            Keysym::XK_E => write!(f, "E"),
            Keysym::XK_F => write!(f, "F"),
            Keysym::XK_G => write!(f, "G"),
            Keysym::XK_H => write!(f, "H"),
            Keysym::XK_I => write!(f, "I"),
            Keysym::XK_J => write!(f, "J"),
            Keysym::XK_K => write!(f, "K"),
            Keysym::XK_L => write!(f, "L"),
            Keysym::XK_M => write!(f, "M"),
            Keysym::XK_N => write!(f, "N"),
            Keysym::XK_O => write!(f, "O"),
            Keysym::XK_P => write!(f, "P"),
            Keysym::XK_Q => write!(f, "Q"),
            Keysym::XK_R => write!(f, "R"),
            Keysym::XK_S => write!(f, "S"),
            Keysym::XK_T => write!(f, "T"),
            Keysym::XK_U => write!(f, "U"),
            Keysym::XK_V => write!(f, "V"),
            Keysym::XK_W => write!(f, "W"),
            Keysym::XK_X => write!(f, "X"),
            Keysym::XK_Y => write!(f, "Y"),
            Keysym::XK_Z => write!(f, "Z"),
            Keysym::XK_a => write!(f, "a"),
            Keysym::XK_b => write!(f, "b"),
            Keysym::XK_c => write!(f, "c"),
            Keysym::XK_d => write!(f, "d"),
            Keysym::XK_e => write!(f, "e"),
            Keysym::XK_f => write!(f, "f"),
            Keysym::XK_g => write!(f, "g"),
            Keysym::XK_h => write!(f, "h"),
            Keysym::XK_i => write!(f, "i"),
            Keysym::XK_j => write!(f, "j"),
            Keysym::XK_k => write!(f, "k"),
            Keysym::XK_l => write!(f, "l"),
            Keysym::XK_m => write!(f, "m"),
            Keysym::XK_n => write!(f, "n"),
            Keysym::XK_o => write!(f, "o"),
            Keysym::XK_p => write!(f, "p"),
            Keysym::XK_q => write!(f, "q"),
            Keysym::XK_r => write!(f, "r"),
            Keysym::XK_s => write!(f, "s"),
            Keysym::XK_t => write!(f, "t"),
            Keysym::XK_u => write!(f, "u"),
            Keysym::XK_v => write!(f, "v"),
            Keysym::XK_w => write!(f, "w"),
            Keysym::XK_x => write!(f, "x"),
            Keysym::XK_y => write!(f, "y"),
            Keysym::XK_z => write!(f, "z"),
            _ => write!(f, ""),
        }
    }
}

impl TryFrom<&str> for Keysym {
    type Error = ConfigError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "0" => Ok(Keysym::XK_0),
            "1" => Ok(Keysym::XK_1),
            "2" => Ok(Keysym::XK_2),
            "3" => Ok(Keysym::XK_3),
            "4" => Ok(Keysym::XK_4),
            "5" => Ok(Keysym::XK_5),
            "6" => Ok(Keysym::XK_6),
            "7" => Ok(Keysym::XK_7),
            "8" => Ok(Keysym::XK_8),
            "9" => Ok(Keysym::XK_9),
            "a" => Ok(Keysym::XK_a),
            "b" => Ok(Keysym::XK_b),
            "c" => Ok(Keysym::XK_c),
            "d" => Ok(Keysym::XK_d),
            "e" => Ok(Keysym::XK_e),
            "f" => Ok(Keysym::XK_f),
            "g" => Ok(Keysym::XK_g),
            "h" => Ok(Keysym::XK_h),
            "i" => Ok(Keysym::XK_i),
            "j" => Ok(Keysym::XK_j),
            "k" => Ok(Keysym::XK_k),
            "l" => Ok(Keysym::XK_l),
            "m" => Ok(Keysym::XK_m),
            "n" => Ok(Keysym::XK_n),
            "o" => Ok(Keysym::XK_o),
            "p" => Ok(Keysym::XK_p),
            "q" => Ok(Keysym::XK_q),
            "r" => Ok(Keysym::XK_r),
            "s" => Ok(Keysym::XK_s),
            "t" => Ok(Keysym::XK_t),
            "u" => Ok(Keysym::XK_u),
            "v" => Ok(Keysym::XK_v),
            "w" => Ok(Keysym::XK_w),
            "x" => Ok(Keysym::XK_x),
            "y" => Ok(Keysym::XK_y),
            "z" => Ok(Keysym::XK_z),
            "Enter" => Ok(Keysym::XK_Return),
            _ => Err(ConfigError::InvalidKey(format!(
                "key {value} has invalid format"
            ))),
        }
    }
}
