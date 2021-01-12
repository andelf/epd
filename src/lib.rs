pub mod command;

/*
const unsigned char EPD_2IN13_lut_full_update[] = {
    0x22, 0x55, 0xAA, 0x55, 0xAA, 0x55, 0xAA, 0x11,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x1E, 0x1E, 0x1E, 0x1E, 0x1E, 0x1E, 0x1E, 0x1E,
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00
};

const unsigned char EPD_2IN13_lut_partial_update[] = {
    0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0F, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00
};
*/

/// phase0A phase0B phase1A phase1B phase2A phase2B phase3A phase3B phase4A phase4B
///
/// VS[n-XY], TP[n#], RP[n]
///
/// - The phase period defined as TP[n#] * T_FRAME, where TP[n#] range from 0 to 31 (5 bits)
///   - TP[n#] = 0 indicates phase skipped.
/// - The Repeat counter defined as RP[n], which represents repeating TP[nA] and TP[nB].
///   - RP[n] = 0 indicates run time =1, where RP[n] range from 0 to 63.
/// - Source Voltage Level: VS[n#-XY] is constant in each phase.
/// - VS[n-XY] indicates the voltage in phase n for transition from X to Y:
///   - X, Y: H, L
///   - 00 – VSS
///   - 01 – VSH
///   - 10 – VSL
#[rustfmt::skip]
pub const LUT_FULL_UPDATE: [u8; 30] = [
    0x22, // VS
    0x55,
    0xAA,
    0x55,
    0xAA,
    0x55,
    0xAA,
    0x11,
    0x00,
    0x00,
    0x00, // reserved
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x1E, // RP, TP. 0b000_11110,
    0x1E,
    0x1E,
    0x1E,
    0x1E,
    0x1E,
    0x1E,
    0x1E,
    0x01,
    0x00,
    0x00, // reserved
    0x00,
    0x00,
    0x00, // not used
];