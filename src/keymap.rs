use rmk::action::{EncoderAction, KeyAction};
use rmk::{a, encoder, k, mo};
pub(crate) const COL: usize = 10;
pub(crate) const ROW: usize = 4;
pub(crate) const NUM_LAYER: usize = 1;
pub(crate) const NUM_ENCODER: usize = 0;
pub(crate) const COL_MASTER: usize = COL / 2;
#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        [
            [k!(Q), k!(W), k!(E), k!(R), k!(T), k!(Y), k!(U), k!(I), k!(O), k!(P)],
            [k!(A), k!(S), k!(D), k!(F), k!(G), k!(F), k!(H), k!(J), k!(K), k!(L)],
            [k!(Z), k!(X), k!(C), k!(V), k!(B), k!(V), k!(B), k!(N), k!(M), k!(Comma)],
            [k!(Z), k!(X), k!(C), k!(V), k!(B), k!(V), k!(B), k!(N), k!(M), k!(Comma)],
        ],
    ]
}

#[rustfmt::skip]
pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [[]]
}
