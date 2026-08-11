// =======================================================================
// pratt.rs
// =======================================================================
// Utility functions for Pratt parsing

#[inline(always)]
pub(crate) fn left_assoc(bp: u8) -> (u8, u8) {
    let scaled_bp = bp * 2;
    (scaled_bp - 1, scaled_bp)
}

#[inline(always)]
pub(crate) fn right_assoc(bp: u8) -> (u8, u8) {
    let scaled_bp = bp * 2;
    (scaled_bp, scaled_bp - 1)
}

#[inline(always)]
pub(crate) fn no_assoc(bp: u8) -> u8 {
    bp * 2
}
