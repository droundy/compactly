//! Generated with `cargo run --example bit-context > src/bit_context.rs`
use crate::adapt::SplitMix64;
use crate::arith::Probability;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitContext {
    #[default]
    Count0_0, // Probability { prob: 2, shift: 2 } = 0.5
    Count1_0,   // Probability { prob: 85, shift: 8 } = 0.33203125
    Count0_1,   // Probability { prob: 85, shift: 7 } = 0.6640625
    Count2_0,   // Probability { prob: 1, shift: 2 } = 0.25
    Count1_1,   // Probability { prob: 2, shift: 2 } = 0.5
    Count0_2,   // Probability { prob: 3, shift: 2 } = 0.75
    Count3_0,   // Probability { prob: 51, shift: 8 } = 0.19921875
    Count2_1,   // Probability { prob: 85, shift: 8 } = 0.33203125
    Count1_2,   // Probability { prob: 85, shift: 7 } = 0.6640625
    Count0_3,   // Probability { prob: 51, shift: 6 } = 0.796875
    Count4_0,   // Probability { prob: 21, shift: 7 } = 0.1640625
    Count3_1,   // Probability { prob: 1, shift: 2 } = 0.25
    Count2_2,   // Probability { prob: 2, shift: 2 } = 0.5
    Count1_3,   // Probability { prob: 3, shift: 2 } = 0.75
    Count0_4,   // Probability { prob: 213, shift: 8 } = 0.83203125
    Count5_0,   // Probability { prob: 9, shift: 6 } = 0.140625
    Count4_1,   // Probability { prob: 51, shift: 8 } = 0.19921875
    Count3_2,   // Probability { prob: 51, shift: 7 } = 0.3984375
    Count2_3,   // Probability { prob: 153, shift: 8 } = 0.59765625
    Count1_4,   // Probability { prob: 51, shift: 6 } = 0.796875
    Count0_5,   // Probability { prob: 219, shift: 8 } = 0.85546875
    Count6_0,   // Probability { prob: 1, shift: 3 } = 0.125
    Count5_1,   // Probability { prob: 21, shift: 7 } = 0.1640625
    Count4_2,   // Probability { prob: 85, shift: 8 } = 0.33203125
    Count3_3,   // Probability { prob: 2, shift: 2 } = 0.5
    Count2_4,   // Probability { prob: 85, shift: 7 } = 0.6640625
    Count1_5,   // Probability { prob: 213, shift: 8 } = 0.83203125
    Count0_6,   // Probability { prob: 7, shift: 3 } = 0.875
    Count7_0,   // Probability { prob: 7, shift: 6 } = 0.109375
    Count6_1,   // Probability { prob: 9, shift: 6 } = 0.140625
    Count5_2,   // Probability { prob: 73, shift: 8 } = 0.28515625
    Count4_3,   // Probability { prob: 109, shift: 8 } = 0.42578125
    Count3_4,   // Probability { prob: 73, shift: 7 } = 0.5703125
    Count2_5,   // Probability { prob: 91, shift: 7 } = 0.7109375
    Count1_6,   // Probability { prob: 219, shift: 8 } = 0.85546875
    Count0_7,   // Probability { prob: 227, shift: 8 } = 0.88671875
    Count8_0,   // Probability { prob: 25, shift: 8 } = 0.09765625
    Count7_1,   // Probability { prob: 1, shift: 3 } = 0.125
    Count6_2,   // Probability { prob: 1, shift: 2 } = 0.25
    Count5_3,   // Probability { prob: 3, shift: 3 } = 0.375
    Count4_4,   // Probability { prob: 2, shift: 2 } = 0.5
    Count3_5,   // Probability { prob: 5, shift: 3 } = 0.625
    Count2_6,   // Probability { prob: 3, shift: 2 } = 0.75
    Count1_7,   // Probability { prob: 7, shift: 3 } = 0.875
    Count0_8,   // Probability { prob: 115, shift: 7 } = 0.8984375
    Count9_0,   // Probability { prob: 23, shift: 8 } = 0.08984375
    Count8_1,   // Probability { prob: 7, shift: 6 } = 0.109375
    Count7_2,   // Probability { prob: 7, shift: 5 } = 0.21875
    Count6_3,   // Probability { prob: 85, shift: 8 } = 0.33203125
    Count5_4,   // Probability { prob: 113, shift: 8 } = 0.44140625
    Count4_5,   // Probability { prob: 71, shift: 7 } = 0.5546875
    Count3_6,   // Probability { prob: 85, shift: 7 } = 0.6640625
    Count2_7,   // Probability { prob: 199, shift: 8 } = 0.77734375
    Count1_8,   // Probability { prob: 227, shift: 8 } = 0.88671875
    Count0_9,   // Probability { prob: 29, shift: 5 } = 0.90625
    Count10_0,  // Probability { prob: 21, shift: 8 } = 0.08203125
    Count9_1,   // Probability { prob: 25, shift: 8 } = 0.09765625
    Count8_2,   // Probability { prob: 51, shift: 8 } = 0.19921875
    Count7_3,   // Probability { prob: 19, shift: 6 } = 0.296875
    Count6_4,   // Probability { prob: 51, shift: 7 } = 0.3984375
    Count5_5,   // Probability { prob: 2, shift: 2 } = 0.5
    Count4_6,   // Probability { prob: 153, shift: 8 } = 0.59765625
    Count3_7,   // Probability { prob: 179, shift: 8 } = 0.69921875
    Count2_8,   // Probability { prob: 51, shift: 6 } = 0.796875
    Count1_9,   // Probability { prob: 115, shift: 7 } = 0.8984375
    Count0_10,  // Probability { prob: 117, shift: 7 } = 0.9140625
    Count11_0,  // Probability { prob: 19, shift: 8 } = 0.07421875
    Count10_1,  // Probability { prob: 23, shift: 8 } = 0.08984375
    Count9_2,   // Probability { prob: 23, shift: 7 } = 0.1796875
    Count8_3,   // Probability { prob: 69, shift: 8 } = 0.26953125
    Count7_4,   // Probability { prob: 93, shift: 8 } = 0.36328125
    Count6_5,   // Probability { prob: 29, shift: 6 } = 0.453125
    Count5_6,   // Probability { prob: 139, shift: 8 } = 0.54296875
    Count4_7,   // Probability { prob: 81, shift: 7 } = 0.6328125
    Count3_8,   // Probability { prob: 93, shift: 7 } = 0.7265625
    Count2_9,   // Probability { prob: 209, shift: 8 } = 0.81640625
    Count1_10,  // Probability { prob: 29, shift: 5 } = 0.90625
    Count0_11,  // Probability { prob: 59, shift: 6 } = 0.921875
    Count12_0,  // Probability { prob: 9, shift: 7 } = 0.0703125
    Count11_1,  // Probability { prob: 21, shift: 8 } = 0.08203125
    Count10_2,  // Probability { prob: 21, shift: 7 } = 0.1640625
    Count9_3,   // Probability { prob: 1, shift: 2 } = 0.25
    Count8_4,   // Probability { prob: 85, shift: 8 } = 0.33203125
    Count7_5,   // Probability { prob: 53, shift: 7 } = 0.4140625
    Count6_6,   // Probability { prob: 2, shift: 2 } = 0.5
    Count5_7,   // Probability { prob: 149, shift: 8 } = 0.58203125
    Count4_8,   // Probability { prob: 85, shift: 7 } = 0.6640625
    Count3_9,   // Probability { prob: 3, shift: 2 } = 0.75
    Count2_10,  // Probability { prob: 213, shift: 8 } = 0.83203125
    Count1_11,  // Probability { prob: 117, shift: 7 } = 0.9140625
    Count0_12,  // Probability { prob: 237, shift: 8 } = 0.92578125
    Count13_0,  // Probability { prob: 17, shift: 8 } = 0.06640625
    Count12_1,  // Probability { prob: 19, shift: 8 } = 0.07421875
    Count11_2,  // Probability { prob: 39, shift: 8 } = 0.15234375
    Count10_3,  // Probability { prob: 59, shift: 8 } = 0.23046875
    Count9_4,   // Probability { prob: 39, shift: 7 } = 0.3046875
    Count8_5,   // Probability { prob: 49, shift: 7 } = 0.3828125
    Count7_6,   // Probability { prob: 59, shift: 7 } = 0.4609375
    Count6_7,   // Probability { prob: 137, shift: 8 } = 0.53515625
    Count5_8,   // Probability { prob: 157, shift: 8 } = 0.61328125
    Count4_9,   // Probability { prob: 177, shift: 8 } = 0.69140625
    Count3_10,  // Probability { prob: 49, shift: 6 } = 0.765625
    Count2_11,  // Probability { prob: 27, shift: 5 } = 0.84375
    Count1_12,  // Probability { prob: 59, shift: 6 } = 0.921875
    Count0_13,  // Probability { prob: 119, shift: 7 } = 0.9296875
    Count14_0,  // Probability { prob: 1, shift: 4 } = 0.0625
    Count13_1,  // Probability { prob: 9, shift: 7 } = 0.0703125
    Count12_2,  // Probability { prob: 9, shift: 6 } = 0.140625
    Count11_3,  // Probability { prob: 27, shift: 7 } = 0.2109375
    Count10_4,  // Probability { prob: 73, shift: 8 } = 0.28515625
    Count9_5,   // Probability { prob: 91, shift: 8 } = 0.35546875
    Count8_6,   // Probability { prob: 109, shift: 8 } = 0.42578125
    Count7_7,   // Probability { prob: 2, shift: 2 } = 0.5
    Count6_8,   // Probability { prob: 73, shift: 7 } = 0.5703125
    Count5_9,   // Probability { prob: 41, shift: 6 } = 0.640625
    Count4_10,  // Probability { prob: 91, shift: 7 } = 0.7109375
    Count3_11,  // Probability { prob: 201, shift: 8 } = 0.78515625
    Count2_12,  // Probability { prob: 219, shift: 8 } = 0.85546875
    Count1_13,  // Probability { prob: 237, shift: 8 } = 0.92578125
    Count0_14,  // Probability { prob: 15, shift: 4 } = 0.9375
    Count15_0,  // Probability { prob: 15, shift: 8 } = 0.05859375
    Count14_1,  // Probability { prob: 17, shift: 8 } = 0.06640625
    Count13_2,  // Probability { prob: 17, shift: 7 } = 0.1328125
    Count12_3,  // Probability { prob: 51, shift: 8 } = 0.19921875
    Count11_4,  // Probability { prob: 17, shift: 6 } = 0.265625
    Count10_5,  // Probability { prob: 85, shift: 8 } = 0.33203125
    Count9_6,   // Probability { prob: 51, shift: 7 } = 0.3984375
    Count8_7,   // Probability { prob: 119, shift: 8 } = 0.46484375
    Count7_8,   // Probability { prob: 17, shift: 5 } = 0.53125
    Count6_9,   // Probability { prob: 153, shift: 8 } = 0.59765625
    Count5_10,  // Probability { prob: 85, shift: 7 } = 0.6640625
    Count4_11,  // Probability { prob: 187, shift: 8 } = 0.73046875
    Count3_12,  // Probability { prob: 51, shift: 6 } = 0.796875
    Count2_13,  // Probability { prob: 221, shift: 8 } = 0.86328125
    Count1_14,  // Probability { prob: 119, shift: 7 } = 0.9296875
    Count0_15,  // Probability { prob: 15, shift: 4 } = 0.9375
    Count16_0,  // Probability { prob: 7, shift: 7 } = 0.0546875
    Count15_1,  // Probability { prob: 1, shift: 4 } = 0.0625
    Count14_2,  // Probability { prob: 1, shift: 3 } = 0.125
    Count13_3,  // Probability { prob: 3, shift: 4 } = 0.1875
    Count12_4,  // Probability { prob: 1, shift: 2 } = 0.25
    Count11_5,  // Probability { prob: 5, shift: 4 } = 0.3125
    Count10_6,  // Probability { prob: 3, shift: 3 } = 0.375
    Count9_7,   // Probability { prob: 7, shift: 4 } = 0.4375
    Count8_8,   // Probability { prob: 2, shift: 2 } = 0.5
    Count7_9,   // Probability { prob: 9, shift: 4 } = 0.5625
    Count6_10,  // Probability { prob: 5, shift: 3 } = 0.625
    Count5_11,  // Probability { prob: 11, shift: 4 } = 0.6875
    Count4_12,  // Probability { prob: 3, shift: 2 } = 0.75
    Count3_13,  // Probability { prob: 13, shift: 4 } = 0.8125
    Count2_14,  // Probability { prob: 7, shift: 3 } = 0.875
    Count1_15,  // Probability { prob: 15, shift: 4 } = 0.9375
    Count0_16,  // Probability { prob: 241, shift: 8 } = 0.94140625
    Count17_0,  // Probability { prob: 13, shift: 8 } = 0.05078125
    Count16_1,  // Probability { prob: 15, shift: 8 } = 0.05859375
    Count15_2,  // Probability { prob: 15, shift: 7 } = 0.1171875
    Count14_3,  // Probability { prob: 45, shift: 8 } = 0.17578125
    Count13_4,  // Probability { prob: 15, shift: 6 } = 0.234375
    Count12_5,  // Probability { prob: 75, shift: 8 } = 0.29296875
    Count11_6,  // Probability { prob: 45, shift: 7 } = 0.3515625
    Count10_7,  // Probability { prob: 105, shift: 8 } = 0.41015625
    Count9_8,   // Probability { prob: 15, shift: 5 } = 0.46875
    Count8_9,   // Probability { prob: 135, shift: 8 } = 0.52734375
    Count7_10,  // Probability { prob: 75, shift: 7 } = 0.5859375
    Count6_11,  // Probability { prob: 165, shift: 8 } = 0.64453125
    Count5_12,  // Probability { prob: 45, shift: 6 } = 0.703125
    Count4_13,  // Probability { prob: 195, shift: 8 } = 0.76171875
    Count3_14,  // Probability { prob: 105, shift: 7 } = 0.8203125
    Count2_15,  // Probability { prob: 225, shift: 8 } = 0.87890625
    Count1_16,  // Probability { prob: 15, shift: 4 } = 0.9375
    Count0_17,  // Probability { prob: 121, shift: 7 } = 0.9453125
    Count18_0,  // Probability { prob: 3, shift: 6 } = 0.046875
    Count17_1,  // Probability { prob: 7, shift: 7 } = 0.0546875
    Count16_2,  // Probability { prob: 7, shift: 6 } = 0.109375
    Count15_3,  // Probability { prob: 21, shift: 7 } = 0.1640625
    Count14_4,  // Probability { prob: 7, shift: 5 } = 0.21875
    Count13_5,  // Probability { prob: 71, shift: 8 } = 0.27734375
    Count12_6,  // Probability { prob: 85, shift: 8 } = 0.33203125
    Count11_7,  // Probability { prob: 99, shift: 8 } = 0.38671875
    Count10_8,  // Probability { prob: 113, shift: 8 } = 0.44140625
    Count9_9,   // Probability { prob: 2, shift: 2 } = 0.5
    Count8_10,  // Probability { prob: 71, shift: 7 } = 0.5546875
    Count7_11,  // Probability { prob: 39, shift: 6 } = 0.609375
    Count6_12,  // Probability { prob: 85, shift: 7 } = 0.6640625
    Count5_13,  // Probability { prob: 23, shift: 5 } = 0.71875
    Count4_14,  // Probability { prob: 199, shift: 8 } = 0.77734375
    Count3_15,  // Probability { prob: 213, shift: 8 } = 0.83203125
    Count2_16,  // Probability { prob: 227, shift: 8 } = 0.88671875
    Count1_17,  // Probability { prob: 241, shift: 8 } = 0.94140625
    Count0_18,  // Probability { prob: 243, shift: 8 } = 0.94921875
    Count19_0,  // Probability { prob: 3, shift: 6 } = 0.046875
    Count18_1,  // Probability { prob: 13, shift: 8 } = 0.05078125
    Count17_2,  // Probability { prob: 13, shift: 7 } = 0.1015625
    Count16_3,  // Probability { prob: 5, shift: 5 } = 0.15625
    Count15_4,  // Probability { prob: 53, shift: 8 } = 0.20703125
    Count14_5,  // Probability { prob: 67, shift: 8 } = 0.26171875
    Count13_6,  // Probability { prob: 5, shift: 4 } = 0.3125
    Count12_7,  // Probability { prob: 47, shift: 7 } = 0.3671875
    Count11_8,  // Probability { prob: 107, shift: 8 } = 0.41796875
    Count10_9,  // Probability { prob: 121, shift: 8 } = 0.47265625
    Count9_10,  // Probability { prob: 67, shift: 7 } = 0.5234375
    Count8_11,  // Probability { prob: 37, shift: 6 } = 0.578125
    Count7_12,  // Probability { prob: 161, shift: 8 } = 0.62890625
    Count6_13,  // Probability { prob: 175, shift: 8 } = 0.68359375
    Count5_14,  // Probability { prob: 47, shift: 6 } = 0.734375
    Count4_15,  // Probability { prob: 101, shift: 7 } = 0.7890625
    Count3_16,  // Probability { prob: 215, shift: 8 } = 0.83984375
    Count2_17,  // Probability { prob: 229, shift: 8 } = 0.89453125
    Count1_18,  // Probability { prob: 121, shift: 7 } = 0.9453125
    Count0_19,  // Probability { prob: 243, shift: 8 } = 0.94921875
    Count20_0,  // Probability { prob: 11, shift: 8 } = 0.04296875
    Count19_1,  // Probability { prob: 3, shift: 6 } = 0.046875
    Count18_2,  // Probability { prob: 25, shift: 8 } = 0.09765625
    Count17_3,  // Probability { prob: 19, shift: 7 } = 0.1484375
    Count16_4,  // Probability { prob: 51, shift: 8 } = 0.19921875
    Count15_5,  // Probability { prob: 1, shift: 2 } = 0.25
    Count14_6,  // Probability { prob: 19, shift: 6 } = 0.296875
    Count13_7,  // Probability { prob: 89, shift: 8 } = 0.34765625
    Count12_8,  // Probability { prob: 51, shift: 7 } = 0.3984375
    Count11_9,  // Probability { prob: 115, shift: 8 } = 0.44921875
    Count10_10, // Probability { prob: 2, shift: 2 } = 0.5
    Count9_11,  // Probability { prob: 35, shift: 6 } = 0.546875
    Count8_12,  // Probability { prob: 153, shift: 8 } = 0.59765625
    Count7_13,  // Probability { prob: 83, shift: 7 } = 0.6484375
    Count6_14,  // Probability { prob: 179, shift: 8 } = 0.69921875
    Count5_15,  // Probability { prob: 3, shift: 2 } = 0.75
    Count4_16,  // Probability { prob: 51, shift: 6 } = 0.796875
    Count3_17,  // Probability { prob: 217, shift: 8 } = 0.84765625
    Count2_18,  // Probability { prob: 115, shift: 7 } = 0.8984375
    Count1_19,  // Probability { prob: 243, shift: 8 } = 0.94921875
    Count0_20,  // Probability { prob: 61, shift: 6 } = 0.953125
    AllFalse4,  // Probability { prob: 31, shift: 5 } = 0.96875
    AllTrue4,   // Probability { prob: 1, shift: 5 } = 0.03125
    AllFalse5,  // Probability { prob: 63, shift: 6 } = 0.984375
    AllTrue5,   // Probability { prob: 1, shift: 6 } = 0.015625
    AllFalse6,  // Probability { prob: 127, shift: 7 } = 0.9921875
    AllTrue6,   // Probability { prob: 1, shift: 7 } = 0.0078125
    AllFalse7,  // Probability { prob: 255, shift: 8 } = 0.99609375
    AllTrue7,   // Probability { prob: 1, shift: 8 } = 0.00390625
    AllFalse8,  // Probability { prob: 511, shift: 9 } = 0.998046875
    AllTrue8,   // Probability { prob: 1, shift: 9 } = 0.001953125
    AllFalse9,  // Probability { prob: 1023, shift: 10 } = 0.9990234375
    AllTrue9,   // Probability { prob: 1, shift: 10 } = 0.0009765625
    AllFalse10, // Probability { prob: 2047, shift: 11 } = 0.99951171875
    AllTrue10,  // Probability { prob: 1, shift: 11 } = 0.00048828125
    AllFalse11, // Probability { prob: 4095, shift: 12 } = 0.999755859375
    AllTrue11,  // Probability { prob: 1, shift: 12 } = 0.000244140625
    AllFalse12, // Probability { prob: 8191, shift: 13 } = 0.9998779296875
    AllTrue12,  // Probability { prob: 1, shift: 13 } = 0.0001220703125
    AllFalse13, // Probability { prob: 16383, shift: 14 } = 0.99993896484375
    AllTrue13,  // Probability { prob: 1, shift: 14 } = 0.00006103515625
    AllFalse14, // Probability { prob: 32767, shift: 15 } = 0.999969482421875
    AllTrue14,  // Probability { prob: 1, shift: 15 } = 0.000030517578125
    AllFalse15, // Probability { prob: 65535, shift: 16 } = 0.9999847412109375
    AllTrue15,  // Probability { prob: 1, shift: 16 } = 0.0000152587890625
}
use BitContext::*;

impl BitContext {
    pub fn probability(self) -> Probability {
        match self {
            Count0_0 => Probability { prob: 2, shift: 2 },
            Count1_0 => Probability { prob: 85, shift: 8 },
            Count0_1 => Probability { prob: 85, shift: 7 },
            Count2_0 => Probability { prob: 1, shift: 2 },
            Count1_1 => Probability { prob: 2, shift: 2 },
            Count0_2 => Probability { prob: 3, shift: 2 },
            Count3_0 => Probability { prob: 51, shift: 8 },
            Count2_1 => Probability { prob: 85, shift: 8 },
            Count1_2 => Probability { prob: 85, shift: 7 },
            Count0_3 => Probability { prob: 51, shift: 6 },
            Count4_0 => Probability { prob: 21, shift: 7 },
            Count3_1 => Probability { prob: 1, shift: 2 },
            Count2_2 => Probability { prob: 2, shift: 2 },
            Count1_3 => Probability { prob: 3, shift: 2 },
            Count0_4 => Probability {
                prob: 213,
                shift: 8,
            },
            Count5_0 => Probability { prob: 9, shift: 6 },
            Count4_1 => Probability { prob: 51, shift: 8 },
            Count3_2 => Probability { prob: 51, shift: 7 },
            Count2_3 => Probability {
                prob: 153,
                shift: 8,
            },
            Count1_4 => Probability { prob: 51, shift: 6 },
            Count0_5 => Probability {
                prob: 219,
                shift: 8,
            },
            Count6_0 => Probability { prob: 1, shift: 3 },
            Count5_1 => Probability { prob: 21, shift: 7 },
            Count4_2 => Probability { prob: 85, shift: 8 },
            Count3_3 => Probability { prob: 2, shift: 2 },
            Count2_4 => Probability { prob: 85, shift: 7 },
            Count1_5 => Probability {
                prob: 213,
                shift: 8,
            },
            Count0_6 => Probability { prob: 7, shift: 3 },
            Count7_0 => Probability { prob: 7, shift: 6 },
            Count6_1 => Probability { prob: 9, shift: 6 },
            Count5_2 => Probability { prob: 73, shift: 8 },
            Count4_3 => Probability {
                prob: 109,
                shift: 8,
            },
            Count3_4 => Probability { prob: 73, shift: 7 },
            Count2_5 => Probability { prob: 91, shift: 7 },
            Count1_6 => Probability {
                prob: 219,
                shift: 8,
            },
            Count0_7 => Probability {
                prob: 227,
                shift: 8,
            },
            Count8_0 => Probability { prob: 25, shift: 8 },
            Count7_1 => Probability { prob: 1, shift: 3 },
            Count6_2 => Probability { prob: 1, shift: 2 },
            Count5_3 => Probability { prob: 3, shift: 3 },
            Count4_4 => Probability { prob: 2, shift: 2 },
            Count3_5 => Probability { prob: 5, shift: 3 },
            Count2_6 => Probability { prob: 3, shift: 2 },
            Count1_7 => Probability { prob: 7, shift: 3 },
            Count0_8 => Probability {
                prob: 115,
                shift: 7,
            },
            Count9_0 => Probability { prob: 23, shift: 8 },
            Count8_1 => Probability { prob: 7, shift: 6 },
            Count7_2 => Probability { prob: 7, shift: 5 },
            Count6_3 => Probability { prob: 85, shift: 8 },
            Count5_4 => Probability {
                prob: 113,
                shift: 8,
            },
            Count4_5 => Probability { prob: 71, shift: 7 },
            Count3_6 => Probability { prob: 85, shift: 7 },
            Count2_7 => Probability {
                prob: 199,
                shift: 8,
            },
            Count1_8 => Probability {
                prob: 227,
                shift: 8,
            },
            Count0_9 => Probability { prob: 29, shift: 5 },
            Count10_0 => Probability { prob: 21, shift: 8 },
            Count9_1 => Probability { prob: 25, shift: 8 },
            Count8_2 => Probability { prob: 51, shift: 8 },
            Count7_3 => Probability { prob: 19, shift: 6 },
            Count6_4 => Probability { prob: 51, shift: 7 },
            Count5_5 => Probability { prob: 2, shift: 2 },
            Count4_6 => Probability {
                prob: 153,
                shift: 8,
            },
            Count3_7 => Probability {
                prob: 179,
                shift: 8,
            },
            Count2_8 => Probability { prob: 51, shift: 6 },
            Count1_9 => Probability {
                prob: 115,
                shift: 7,
            },
            Count0_10 => Probability {
                prob: 117,
                shift: 7,
            },
            Count11_0 => Probability { prob: 19, shift: 8 },
            Count10_1 => Probability { prob: 23, shift: 8 },
            Count9_2 => Probability { prob: 23, shift: 7 },
            Count8_3 => Probability { prob: 69, shift: 8 },
            Count7_4 => Probability { prob: 93, shift: 8 },
            Count6_5 => Probability { prob: 29, shift: 6 },
            Count5_6 => Probability {
                prob: 139,
                shift: 8,
            },
            Count4_7 => Probability { prob: 81, shift: 7 },
            Count3_8 => Probability { prob: 93, shift: 7 },
            Count2_9 => Probability {
                prob: 209,
                shift: 8,
            },
            Count1_10 => Probability { prob: 29, shift: 5 },
            Count0_11 => Probability { prob: 59, shift: 6 },
            Count12_0 => Probability { prob: 9, shift: 7 },
            Count11_1 => Probability { prob: 21, shift: 8 },
            Count10_2 => Probability { prob: 21, shift: 7 },
            Count9_3 => Probability { prob: 1, shift: 2 },
            Count8_4 => Probability { prob: 85, shift: 8 },
            Count7_5 => Probability { prob: 53, shift: 7 },
            Count6_6 => Probability { prob: 2, shift: 2 },
            Count5_7 => Probability {
                prob: 149,
                shift: 8,
            },
            Count4_8 => Probability { prob: 85, shift: 7 },
            Count3_9 => Probability { prob: 3, shift: 2 },
            Count2_10 => Probability {
                prob: 213,
                shift: 8,
            },
            Count1_11 => Probability {
                prob: 117,
                shift: 7,
            },
            Count0_12 => Probability {
                prob: 237,
                shift: 8,
            },
            Count13_0 => Probability { prob: 17, shift: 8 },
            Count12_1 => Probability { prob: 19, shift: 8 },
            Count11_2 => Probability { prob: 39, shift: 8 },
            Count10_3 => Probability { prob: 59, shift: 8 },
            Count9_4 => Probability { prob: 39, shift: 7 },
            Count8_5 => Probability { prob: 49, shift: 7 },
            Count7_6 => Probability { prob: 59, shift: 7 },
            Count6_7 => Probability {
                prob: 137,
                shift: 8,
            },
            Count5_8 => Probability {
                prob: 157,
                shift: 8,
            },
            Count4_9 => Probability {
                prob: 177,
                shift: 8,
            },
            Count3_10 => Probability { prob: 49, shift: 6 },
            Count2_11 => Probability { prob: 27, shift: 5 },
            Count1_12 => Probability { prob: 59, shift: 6 },
            Count0_13 => Probability {
                prob: 119,
                shift: 7,
            },
            Count14_0 => Probability { prob: 1, shift: 4 },
            Count13_1 => Probability { prob: 9, shift: 7 },
            Count12_2 => Probability { prob: 9, shift: 6 },
            Count11_3 => Probability { prob: 27, shift: 7 },
            Count10_4 => Probability { prob: 73, shift: 8 },
            Count9_5 => Probability { prob: 91, shift: 8 },
            Count8_6 => Probability {
                prob: 109,
                shift: 8,
            },
            Count7_7 => Probability { prob: 2, shift: 2 },
            Count6_8 => Probability { prob: 73, shift: 7 },
            Count5_9 => Probability { prob: 41, shift: 6 },
            Count4_10 => Probability { prob: 91, shift: 7 },
            Count3_11 => Probability {
                prob: 201,
                shift: 8,
            },
            Count2_12 => Probability {
                prob: 219,
                shift: 8,
            },
            Count1_13 => Probability {
                prob: 237,
                shift: 8,
            },
            Count0_14 => Probability { prob: 15, shift: 4 },
            Count15_0 => Probability { prob: 15, shift: 8 },
            Count14_1 => Probability { prob: 17, shift: 8 },
            Count13_2 => Probability { prob: 17, shift: 7 },
            Count12_3 => Probability { prob: 51, shift: 8 },
            Count11_4 => Probability { prob: 17, shift: 6 },
            Count10_5 => Probability { prob: 85, shift: 8 },
            Count9_6 => Probability { prob: 51, shift: 7 },
            Count8_7 => Probability {
                prob: 119,
                shift: 8,
            },
            Count7_8 => Probability { prob: 17, shift: 5 },
            Count6_9 => Probability {
                prob: 153,
                shift: 8,
            },
            Count5_10 => Probability { prob: 85, shift: 7 },
            Count4_11 => Probability {
                prob: 187,
                shift: 8,
            },
            Count3_12 => Probability { prob: 51, shift: 6 },
            Count2_13 => Probability {
                prob: 221,
                shift: 8,
            },
            Count1_14 => Probability {
                prob: 119,
                shift: 7,
            },
            Count0_15 => Probability { prob: 15, shift: 4 },
            Count16_0 => Probability { prob: 7, shift: 7 },
            Count15_1 => Probability { prob: 1, shift: 4 },
            Count14_2 => Probability { prob: 1, shift: 3 },
            Count13_3 => Probability { prob: 3, shift: 4 },
            Count12_4 => Probability { prob: 1, shift: 2 },
            Count11_5 => Probability { prob: 5, shift: 4 },
            Count10_6 => Probability { prob: 3, shift: 3 },
            Count9_7 => Probability { prob: 7, shift: 4 },
            Count8_8 => Probability { prob: 2, shift: 2 },
            Count7_9 => Probability { prob: 9, shift: 4 },
            Count6_10 => Probability { prob: 5, shift: 3 },
            Count5_11 => Probability { prob: 11, shift: 4 },
            Count4_12 => Probability { prob: 3, shift: 2 },
            Count3_13 => Probability { prob: 13, shift: 4 },
            Count2_14 => Probability { prob: 7, shift: 3 },
            Count1_15 => Probability { prob: 15, shift: 4 },
            Count0_16 => Probability {
                prob: 241,
                shift: 8,
            },
            Count17_0 => Probability { prob: 13, shift: 8 },
            Count16_1 => Probability { prob: 15, shift: 8 },
            Count15_2 => Probability { prob: 15, shift: 7 },
            Count14_3 => Probability { prob: 45, shift: 8 },
            Count13_4 => Probability { prob: 15, shift: 6 },
            Count12_5 => Probability { prob: 75, shift: 8 },
            Count11_6 => Probability { prob: 45, shift: 7 },
            Count10_7 => Probability {
                prob: 105,
                shift: 8,
            },
            Count9_8 => Probability { prob: 15, shift: 5 },
            Count8_9 => Probability {
                prob: 135,
                shift: 8,
            },
            Count7_10 => Probability { prob: 75, shift: 7 },
            Count6_11 => Probability {
                prob: 165,
                shift: 8,
            },
            Count5_12 => Probability { prob: 45, shift: 6 },
            Count4_13 => Probability {
                prob: 195,
                shift: 8,
            },
            Count3_14 => Probability {
                prob: 105,
                shift: 7,
            },
            Count2_15 => Probability {
                prob: 225,
                shift: 8,
            },
            Count1_16 => Probability { prob: 15, shift: 4 },
            Count0_17 => Probability {
                prob: 121,
                shift: 7,
            },
            Count18_0 => Probability { prob: 3, shift: 6 },
            Count17_1 => Probability { prob: 7, shift: 7 },
            Count16_2 => Probability { prob: 7, shift: 6 },
            Count15_3 => Probability { prob: 21, shift: 7 },
            Count14_4 => Probability { prob: 7, shift: 5 },
            Count13_5 => Probability { prob: 71, shift: 8 },
            Count12_6 => Probability { prob: 85, shift: 8 },
            Count11_7 => Probability { prob: 99, shift: 8 },
            Count10_8 => Probability {
                prob: 113,
                shift: 8,
            },
            Count9_9 => Probability { prob: 2, shift: 2 },
            Count8_10 => Probability { prob: 71, shift: 7 },
            Count7_11 => Probability { prob: 39, shift: 6 },
            Count6_12 => Probability { prob: 85, shift: 7 },
            Count5_13 => Probability { prob: 23, shift: 5 },
            Count4_14 => Probability {
                prob: 199,
                shift: 8,
            },
            Count3_15 => Probability {
                prob: 213,
                shift: 8,
            },
            Count2_16 => Probability {
                prob: 227,
                shift: 8,
            },
            Count1_17 => Probability {
                prob: 241,
                shift: 8,
            },
            Count0_18 => Probability {
                prob: 243,
                shift: 8,
            },
            Count19_0 => Probability { prob: 3, shift: 6 },
            Count18_1 => Probability { prob: 13, shift: 8 },
            Count17_2 => Probability { prob: 13, shift: 7 },
            Count16_3 => Probability { prob: 5, shift: 5 },
            Count15_4 => Probability { prob: 53, shift: 8 },
            Count14_5 => Probability { prob: 67, shift: 8 },
            Count13_6 => Probability { prob: 5, shift: 4 },
            Count12_7 => Probability { prob: 47, shift: 7 },
            Count11_8 => Probability {
                prob: 107,
                shift: 8,
            },
            Count10_9 => Probability {
                prob: 121,
                shift: 8,
            },
            Count9_10 => Probability { prob: 67, shift: 7 },
            Count8_11 => Probability { prob: 37, shift: 6 },
            Count7_12 => Probability {
                prob: 161,
                shift: 8,
            },
            Count6_13 => Probability {
                prob: 175,
                shift: 8,
            },
            Count5_14 => Probability { prob: 47, shift: 6 },
            Count4_15 => Probability {
                prob: 101,
                shift: 7,
            },
            Count3_16 => Probability {
                prob: 215,
                shift: 8,
            },
            Count2_17 => Probability {
                prob: 229,
                shift: 8,
            },
            Count1_18 => Probability {
                prob: 121,
                shift: 7,
            },
            Count0_19 => Probability {
                prob: 243,
                shift: 8,
            },
            Count20_0 => Probability { prob: 11, shift: 8 },
            Count19_1 => Probability { prob: 3, shift: 6 },
            Count18_2 => Probability { prob: 25, shift: 8 },
            Count17_3 => Probability { prob: 19, shift: 7 },
            Count16_4 => Probability { prob: 51, shift: 8 },
            Count15_5 => Probability { prob: 1, shift: 2 },
            Count14_6 => Probability { prob: 19, shift: 6 },
            Count13_7 => Probability { prob: 89, shift: 8 },
            Count12_8 => Probability { prob: 51, shift: 7 },
            Count11_9 => Probability {
                prob: 115,
                shift: 8,
            },
            Count10_10 => Probability { prob: 2, shift: 2 },
            Count9_11 => Probability { prob: 35, shift: 6 },
            Count8_12 => Probability {
                prob: 153,
                shift: 8,
            },
            Count7_13 => Probability { prob: 83, shift: 7 },
            Count6_14 => Probability {
                prob: 179,
                shift: 8,
            },
            Count5_15 => Probability { prob: 3, shift: 2 },
            Count4_16 => Probability { prob: 51, shift: 6 },
            Count3_17 => Probability {
                prob: 217,
                shift: 8,
            },
            Count2_18 => Probability {
                prob: 115,
                shift: 7,
            },
            Count1_19 => Probability {
                prob: 243,
                shift: 8,
            },
            Count0_20 => Probability { prob: 61, shift: 6 },
            AllFalse4 => Probability { prob: 31, shift: 5 },
            AllTrue4 => Probability { prob: 1, shift: 5 },
            AllFalse5 => Probability { prob: 63, shift: 6 },
            AllTrue5 => Probability { prob: 1, shift: 6 },
            AllFalse6 => Probability {
                prob: 127,
                shift: 7,
            },
            AllTrue6 => Probability { prob: 1, shift: 7 },
            AllFalse7 => Probability {
                prob: 255,
                shift: 8,
            },
            AllTrue7 => Probability { prob: 1, shift: 8 },
            AllFalse8 => Probability {
                prob: 511,
                shift: 9,
            },
            AllTrue8 => Probability { prob: 1, shift: 9 },
            AllFalse9 => Probability {
                prob: 1023,
                shift: 10,
            },
            AllTrue9 => Probability { prob: 1, shift: 10 },
            AllFalse10 => Probability {
                prob: 2047,
                shift: 11,
            },
            AllTrue10 => Probability { prob: 1, shift: 11 },
            AllFalse11 => Probability {
                prob: 4095,
                shift: 12,
            },
            AllTrue11 => Probability { prob: 1, shift: 12 },
            AllFalse12 => Probability {
                prob: 8191,
                shift: 13,
            },
            AllTrue12 => Probability { prob: 1, shift: 13 },
            AllFalse13 => Probability {
                prob: 16383,
                shift: 14,
            },
            AllTrue13 => Probability { prob: 1, shift: 14 },
            AllFalse14 => Probability {
                prob: 32767,
                shift: 15,
            },
            AllTrue14 => Probability { prob: 1, shift: 15 },
            AllFalse15 => Probability {
                prob: 65535,
                shift: 16,
            },
            AllTrue15 => Probability { prob: 1, shift: 16 },
        }
    }

    pub fn adapt(self, bit: bool, rng: &mut SplitMix64) -> Self {
        match self {
            Count0_0 => {
                if bit == false {
                    Count1_0
                } else {
                    Count0_1
                }
            }
            Count1_0 => {
                if bit == true {
                    Count2_0
                } else {
                    Count1_1
                }
            }
            Count0_1 => {
                if bit == false {
                    Count0_2
                } else {
                    Count1_1
                }
            }
            Count2_0 => {
                if bit == true {
                    Count3_0
                } else {
                    Count2_1
                }
            }
            Count1_1 => {
                if bit == false {
                    Count2_1
                } else {
                    Count1_2
                }
            }
            Count0_2 => {
                if bit == false {
                    Count0_3
                } else {
                    Count1_2
                }
            }
            Count3_0 => {
                if bit == true {
                    Count4_0
                } else {
                    Count3_1
                }
            }
            Count2_1 => {
                if bit == true {
                    Count3_1
                } else {
                    Count2_2
                }
            }
            Count1_2 => {
                if bit == false {
                    Count1_3
                } else {
                    Count2_2
                }
            }
            Count0_3 => {
                if bit == false {
                    Count0_4
                } else {
                    Count1_3
                }
            }
            Count4_0 => {
                if bit == true {
                    Count5_0
                } else {
                    Count4_1
                }
            }
            Count3_1 => {
                if bit == true {
                    Count4_1
                } else {
                    Count3_2
                }
            }
            Count2_2 => {
                if bit == false {
                    Count3_2
                } else {
                    Count2_3
                }
            }
            Count1_3 => {
                if bit == false {
                    Count1_4
                } else {
                    Count2_3
                }
            }
            Count0_4 => {
                if bit == false {
                    Count0_5
                } else {
                    Count1_4
                }
            }
            Count5_0 => {
                if bit == true {
                    Count6_0
                } else {
                    Count5_1
                }
            }
            Count4_1 => {
                if bit == true {
                    Count5_1
                } else {
                    Count4_2
                }
            }
            Count3_2 => {
                if bit == true {
                    Count4_2
                } else {
                    Count3_3
                }
            }
            Count2_3 => {
                if bit == false {
                    Count2_4
                } else {
                    Count3_3
                }
            }
            Count1_4 => {
                if bit == false {
                    Count1_5
                } else {
                    Count2_4
                }
            }
            Count0_5 => {
                if bit == false {
                    Count0_6
                } else {
                    Count1_5
                }
            }
            Count6_0 => {
                if bit == true {
                    Count7_0
                } else {
                    Count6_1
                }
            }
            Count5_1 => {
                if bit == true {
                    Count6_1
                } else {
                    Count5_2
                }
            }
            Count4_2 => {
                if bit == true {
                    Count5_2
                } else {
                    Count4_3
                }
            }
            Count3_3 => {
                if bit == false {
                    Count4_3
                } else {
                    Count3_4
                }
            }
            Count2_4 => {
                if bit == false {
                    Count2_5
                } else {
                    Count3_4
                }
            }
            Count1_5 => {
                if bit == false {
                    Count1_6
                } else {
                    Count2_5
                }
            }
            Count0_6 => {
                if bit == false {
                    Count0_7
                } else {
                    Count1_6
                }
            }
            Count7_0 => {
                if bit == true {
                    Count8_0
                } else {
                    Count7_1
                }
            }
            Count6_1 => {
                if bit == true {
                    Count7_1
                } else {
                    Count6_2
                }
            }
            Count5_2 => {
                if bit == true {
                    Count6_2
                } else {
                    Count5_3
                }
            }
            Count4_3 => {
                if bit == true {
                    Count5_3
                } else {
                    Count4_4
                }
            }
            Count3_4 => {
                if bit == false {
                    Count3_5
                } else {
                    Count4_4
                }
            }
            Count2_5 => {
                if bit == false {
                    Count2_6
                } else {
                    Count3_5
                }
            }
            Count1_6 => {
                if bit == false {
                    Count1_7
                } else {
                    Count2_6
                }
            }
            Count0_7 => {
                if bit == false {
                    Count0_8
                } else {
                    Count1_7
                }
            }
            Count8_0 => {
                if bit == true {
                    Count9_0
                } else {
                    Count8_1
                }
            }
            Count7_1 => {
                if bit == true {
                    Count8_1
                } else {
                    Count7_2
                }
            }
            Count6_2 => {
                if bit == true {
                    Count7_2
                } else {
                    Count6_3
                }
            }
            Count5_3 => {
                if bit == true {
                    Count6_3
                } else {
                    Count5_4
                }
            }
            Count4_4 => {
                if bit == false {
                    Count5_4
                } else {
                    Count4_5
                }
            }
            Count3_5 => {
                if bit == false {
                    Count3_6
                } else {
                    Count4_5
                }
            }
            Count2_6 => {
                if bit == false {
                    Count2_7
                } else {
                    Count3_6
                }
            }
            Count1_7 => {
                if bit == false {
                    Count1_8
                } else {
                    Count2_7
                }
            }
            Count0_8 => {
                if bit == false {
                    Count0_9
                } else {
                    Count1_8
                }
            }
            Count9_0 => {
                if bit == true {
                    Count10_0
                } else {
                    Count9_1
                }
            }
            Count8_1 => {
                if bit == true {
                    Count9_1
                } else {
                    Count8_2
                }
            }
            Count7_2 => {
                if bit == true {
                    Count8_2
                } else {
                    Count7_3
                }
            }
            Count6_3 => {
                if bit == true {
                    Count7_3
                } else {
                    Count6_4
                }
            }
            Count5_4 => {
                if bit == true {
                    Count6_4
                } else {
                    Count5_5
                }
            }
            Count4_5 => {
                if bit == false {
                    Count4_6
                } else {
                    Count5_5
                }
            }
            Count3_6 => {
                if bit == false {
                    Count3_7
                } else {
                    Count4_6
                }
            }
            Count2_7 => {
                if bit == false {
                    Count2_8
                } else {
                    Count3_7
                }
            }
            Count1_8 => {
                if bit == false {
                    Count1_9
                } else {
                    Count2_8
                }
            }
            Count0_9 => {
                if bit == false {
                    Count0_10
                } else {
                    Count1_9
                }
            }
            Count10_0 => {
                if bit == true {
                    Count11_0
                } else {
                    Count10_1
                }
            }
            Count9_1 => {
                if bit == true {
                    Count10_1
                } else {
                    Count9_2
                }
            }
            Count8_2 => {
                if bit == true {
                    Count9_2
                } else {
                    Count8_3
                }
            }
            Count7_3 => {
                if bit == true {
                    Count8_3
                } else {
                    Count7_4
                }
            }
            Count6_4 => {
                if bit == true {
                    Count7_4
                } else {
                    Count6_5
                }
            }
            Count5_5 => {
                if bit == false {
                    Count6_5
                } else {
                    Count5_6
                }
            }
            Count4_6 => {
                if bit == false {
                    Count4_7
                } else {
                    Count5_6
                }
            }
            Count3_7 => {
                if bit == false {
                    Count3_8
                } else {
                    Count4_7
                }
            }
            Count2_8 => {
                if bit == false {
                    Count2_9
                } else {
                    Count3_8
                }
            }
            Count1_9 => {
                if bit == false {
                    Count1_10
                } else {
                    Count2_9
                }
            }
            Count0_10 => {
                if bit == false {
                    Count0_11
                } else {
                    Count1_10
                }
            }
            Count11_0 => {
                if bit == true {
                    Count12_0
                } else {
                    Count11_1
                }
            }
            Count10_1 => {
                if bit == true {
                    Count11_1
                } else {
                    Count10_2
                }
            }
            Count9_2 => {
                if bit == true {
                    Count10_2
                } else {
                    Count9_3
                }
            }
            Count8_3 => {
                if bit == true {
                    Count9_3
                } else {
                    Count8_4
                }
            }
            Count7_4 => {
                if bit == true {
                    Count8_4
                } else {
                    Count7_5
                }
            }
            Count6_5 => {
                if bit == true {
                    Count7_5
                } else {
                    Count6_6
                }
            }
            Count5_6 => {
                if bit == false {
                    Count5_7
                } else {
                    Count6_6
                }
            }
            Count4_7 => {
                if bit == false {
                    Count4_8
                } else {
                    Count5_7
                }
            }
            Count3_8 => {
                if bit == false {
                    Count3_9
                } else {
                    Count4_8
                }
            }
            Count2_9 => {
                if bit == false {
                    Count2_10
                } else {
                    Count3_9
                }
            }
            Count1_10 => {
                if bit == false {
                    Count1_11
                } else {
                    Count2_10
                }
            }
            Count0_11 => {
                if bit == false {
                    Count0_12
                } else {
                    Count1_11
                }
            }
            Count12_0 => {
                if bit == true {
                    Count13_0
                } else {
                    Count12_1
                }
            }
            Count11_1 => {
                if bit == true {
                    Count12_1
                } else {
                    Count11_2
                }
            }
            Count10_2 => {
                if bit == true {
                    Count11_2
                } else {
                    Count10_3
                }
            }
            Count9_3 => {
                if bit == true {
                    Count10_3
                } else {
                    Count9_4
                }
            }
            Count8_4 => {
                if bit == true {
                    Count9_4
                } else {
                    Count8_5
                }
            }
            Count7_5 => {
                if bit == true {
                    Count8_5
                } else {
                    Count7_6
                }
            }
            Count6_6 => {
                if bit == false {
                    Count7_6
                } else {
                    Count6_7
                }
            }
            Count5_7 => {
                if bit == false {
                    Count5_8
                } else {
                    Count6_7
                }
            }
            Count4_8 => {
                if bit == false {
                    Count4_9
                } else {
                    Count5_8
                }
            }
            Count3_9 => {
                if bit == false {
                    Count3_10
                } else {
                    Count4_9
                }
            }
            Count2_10 => {
                if bit == false {
                    Count2_11
                } else {
                    Count3_10
                }
            }
            Count1_11 => {
                if bit == false {
                    Count1_12
                } else {
                    Count2_11
                }
            }
            Count0_12 => {
                if bit == false {
                    Count0_13
                } else {
                    Count1_12
                }
            }
            Count13_0 => {
                if bit == true {
                    Count14_0
                } else {
                    Count13_1
                }
            }
            Count12_1 => {
                if bit == true {
                    Count13_1
                } else {
                    Count12_2
                }
            }
            Count11_2 => {
                if bit == true {
                    Count12_2
                } else {
                    Count11_3
                }
            }
            Count10_3 => {
                if bit == true {
                    Count11_3
                } else {
                    Count10_4
                }
            }
            Count9_4 => {
                if bit == true {
                    Count10_4
                } else {
                    Count9_5
                }
            }
            Count8_5 => {
                if bit == true {
                    Count9_5
                } else {
                    Count8_6
                }
            }
            Count7_6 => {
                if bit == true {
                    Count8_6
                } else {
                    Count7_7
                }
            }
            Count6_7 => {
                if bit == false {
                    Count6_8
                } else {
                    Count7_7
                }
            }
            Count5_8 => {
                if bit == false {
                    Count5_9
                } else {
                    Count6_8
                }
            }
            Count4_9 => {
                if bit == false {
                    Count4_10
                } else {
                    Count5_9
                }
            }
            Count3_10 => {
                if bit == false {
                    Count3_11
                } else {
                    Count4_10
                }
            }
            Count2_11 => {
                if bit == false {
                    Count2_12
                } else {
                    Count3_11
                }
            }
            Count1_12 => {
                if bit == false {
                    Count1_13
                } else {
                    Count2_12
                }
            }
            Count0_13 => {
                if bit == false {
                    Count0_14
                } else {
                    Count1_13
                }
            }
            Count14_0 => {
                if bit == true {
                    Count15_0
                } else {
                    Count14_1
                }
            }
            Count13_1 => {
                if bit == true {
                    Count14_1
                } else {
                    Count13_2
                }
            }
            Count12_2 => {
                if bit == true {
                    Count13_2
                } else {
                    Count12_3
                }
            }
            Count11_3 => {
                if bit == true {
                    Count12_3
                } else {
                    Count11_4
                }
            }
            Count10_4 => {
                if bit == true {
                    Count11_4
                } else {
                    Count10_5
                }
            }
            Count9_5 => {
                if bit == true {
                    Count10_5
                } else {
                    Count9_6
                }
            }
            Count8_6 => {
                if bit == true {
                    Count9_6
                } else {
                    Count8_7
                }
            }
            Count7_7 => {
                if bit == false {
                    Count8_7
                } else {
                    Count7_8
                }
            }
            Count6_8 => {
                if bit == false {
                    Count6_9
                } else {
                    Count7_8
                }
            }
            Count5_9 => {
                if bit == false {
                    Count5_10
                } else {
                    Count6_9
                }
            }
            Count4_10 => {
                if bit == false {
                    Count4_11
                } else {
                    Count5_10
                }
            }
            Count3_11 => {
                if bit == false {
                    Count3_12
                } else {
                    Count4_11
                }
            }
            Count2_12 => {
                if bit == false {
                    Count2_13
                } else {
                    Count3_12
                }
            }
            Count1_13 => {
                if bit == false {
                    Count1_14
                } else {
                    Count2_13
                }
            }
            Count0_14 => {
                if bit == false {
                    Count0_15
                } else {
                    Count1_14
                }
            }
            Count15_0 => {
                if bit == true {
                    Count16_0
                } else {
                    Count15_1
                }
            }
            Count14_1 => {
                if bit == true {
                    Count15_1
                } else {
                    Count14_2
                }
            }
            Count13_2 => {
                if bit == true {
                    Count14_2
                } else {
                    Count13_3
                }
            }
            Count12_3 => {
                if bit == true {
                    Count13_3
                } else {
                    Count12_4
                }
            }
            Count11_4 => {
                if bit == true {
                    Count12_4
                } else {
                    Count11_5
                }
            }
            Count10_5 => {
                if bit == true {
                    Count11_5
                } else {
                    Count10_6
                }
            }
            Count9_6 => {
                if bit == true {
                    Count10_6
                } else {
                    Count9_7
                }
            }
            Count8_7 => {
                if bit == true {
                    Count9_7
                } else {
                    Count8_8
                }
            }
            Count7_8 => {
                if bit == false {
                    Count7_9
                } else {
                    Count8_8
                }
            }
            Count6_9 => {
                if bit == false {
                    Count6_10
                } else {
                    Count7_9
                }
            }
            Count5_10 => {
                if bit == false {
                    Count5_11
                } else {
                    Count6_10
                }
            }
            Count4_11 => {
                if bit == false {
                    Count4_12
                } else {
                    Count5_11
                }
            }
            Count3_12 => {
                if bit == false {
                    Count3_13
                } else {
                    Count4_12
                }
            }
            Count2_13 => {
                if bit == false {
                    Count2_14
                } else {
                    Count3_13
                }
            }
            Count1_14 => {
                if bit == false {
                    Count1_15
                } else {
                    Count2_14
                }
            }
            Count0_15 => {
                if bit == false {
                    Count0_16
                } else {
                    Count1_15
                }
            }
            Count16_0 => {
                if bit == true {
                    Count17_0
                } else {
                    Count16_1
                }
            }
            Count15_1 => {
                if bit == true {
                    Count16_1
                } else {
                    Count15_2
                }
            }
            Count14_2 => {
                if bit == true {
                    Count15_2
                } else {
                    Count14_3
                }
            }
            Count13_3 => {
                if bit == true {
                    Count14_3
                } else {
                    Count13_4
                }
            }
            Count12_4 => {
                if bit == true {
                    Count13_4
                } else {
                    Count12_5
                }
            }
            Count11_5 => {
                if bit == true {
                    Count12_5
                } else {
                    Count11_6
                }
            }
            Count10_6 => {
                if bit == true {
                    Count11_6
                } else {
                    Count10_7
                }
            }
            Count9_7 => {
                if bit == true {
                    Count10_7
                } else {
                    Count9_8
                }
            }
            Count8_8 => {
                if bit == false {
                    Count9_8
                } else {
                    Count8_9
                }
            }
            Count7_9 => {
                if bit == false {
                    Count7_10
                } else {
                    Count8_9
                }
            }
            Count6_10 => {
                if bit == false {
                    Count6_11
                } else {
                    Count7_10
                }
            }
            Count5_11 => {
                if bit == false {
                    Count5_12
                } else {
                    Count6_11
                }
            }
            Count4_12 => {
                if bit == false {
                    Count4_13
                } else {
                    Count5_12
                }
            }
            Count3_13 => {
                if bit == false {
                    Count3_14
                } else {
                    Count4_13
                }
            }
            Count2_14 => {
                if bit == false {
                    Count2_15
                } else {
                    Count3_14
                }
            }
            Count1_15 => {
                if bit == false {
                    Count1_16
                } else {
                    Count2_15
                }
            }
            Count0_16 => {
                if bit == false {
                    Count0_17
                } else {
                    Count1_16
                }
            }
            Count17_0 => {
                if bit == true {
                    Count18_0
                } else {
                    Count17_1
                }
            }
            Count16_1 => {
                if bit == true {
                    Count17_1
                } else {
                    Count16_2
                }
            }
            Count15_2 => {
                if bit == true {
                    Count16_2
                } else {
                    Count15_3
                }
            }
            Count14_3 => {
                if bit == true {
                    Count15_3
                } else {
                    Count14_4
                }
            }
            Count13_4 => {
                if bit == true {
                    Count14_4
                } else {
                    Count13_5
                }
            }
            Count12_5 => {
                if bit == true {
                    Count13_5
                } else {
                    Count12_6
                }
            }
            Count11_6 => {
                if bit == true {
                    Count12_6
                } else {
                    Count11_7
                }
            }
            Count10_7 => {
                if bit == true {
                    Count11_7
                } else {
                    Count10_8
                }
            }
            Count9_8 => {
                if bit == true {
                    Count10_8
                } else {
                    Count9_9
                }
            }
            Count8_9 => {
                if bit == false {
                    Count8_10
                } else {
                    Count9_9
                }
            }
            Count7_10 => {
                if bit == false {
                    Count7_11
                } else {
                    Count8_10
                }
            }
            Count6_11 => {
                if bit == false {
                    Count6_12
                } else {
                    Count7_11
                }
            }
            Count5_12 => {
                if bit == false {
                    Count5_13
                } else {
                    Count6_12
                }
            }
            Count4_13 => {
                if bit == false {
                    Count4_14
                } else {
                    Count5_13
                }
            }
            Count3_14 => {
                if bit == false {
                    Count3_15
                } else {
                    Count4_14
                }
            }
            Count2_15 => {
                if bit == false {
                    Count2_16
                } else {
                    Count3_15
                }
            }
            Count1_16 => {
                if bit == false {
                    Count1_17
                } else {
                    Count2_16
                }
            }
            Count0_17 => {
                if bit == false {
                    Count0_18
                } else {
                    Count1_17
                }
            }
            Count18_0 => {
                if bit == true {
                    Count19_0
                } else {
                    Count18_1
                }
            }
            Count17_1 => {
                if bit == true {
                    Count18_1
                } else {
                    Count17_2
                }
            }
            Count16_2 => {
                if bit == true {
                    Count17_2
                } else {
                    Count16_3
                }
            }
            Count15_3 => {
                if bit == true {
                    Count16_3
                } else {
                    Count15_4
                }
            }
            Count14_4 => {
                if bit == true {
                    Count15_4
                } else {
                    Count14_5
                }
            }
            Count13_5 => {
                if bit == true {
                    Count14_5
                } else {
                    Count13_6
                }
            }
            Count12_6 => {
                if bit == true {
                    Count13_6
                } else {
                    Count12_7
                }
            }
            Count11_7 => {
                if bit == true {
                    Count12_7
                } else {
                    Count11_8
                }
            }
            Count10_8 => {
                if bit == true {
                    Count11_8
                } else {
                    Count10_9
                }
            }
            Count9_9 => {
                if bit == false {
                    Count10_9
                } else {
                    Count9_10
                }
            }
            Count8_10 => {
                if bit == false {
                    Count8_11
                } else {
                    Count9_10
                }
            }
            Count7_11 => {
                if bit == false {
                    Count7_12
                } else {
                    Count8_11
                }
            }
            Count6_12 => {
                if bit == false {
                    Count6_13
                } else {
                    Count7_12
                }
            }
            Count5_13 => {
                if bit == false {
                    Count5_14
                } else {
                    Count6_13
                }
            }
            Count4_14 => {
                if bit == false {
                    Count4_15
                } else {
                    Count5_14
                }
            }
            Count3_15 => {
                if bit == false {
                    Count3_16
                } else {
                    Count4_15
                }
            }
            Count2_16 => {
                if bit == false {
                    Count2_17
                } else {
                    Count3_16
                }
            }
            Count1_17 => {
                if bit == false {
                    Count1_18
                } else {
                    Count2_17
                }
            }
            Count0_18 => {
                if bit == false {
                    Count0_19
                } else {
                    Count1_18
                }
            }
            Count19_0 => {
                if bit == true {
                    Count20_0
                } else {
                    Count19_1
                }
            }
            Count18_1 => {
                if bit == true {
                    Count19_1
                } else {
                    Count18_2
                }
            }
            Count17_2 => {
                if bit == true {
                    Count18_2
                } else {
                    Count17_3
                }
            }
            Count16_3 => {
                if bit == true {
                    Count17_3
                } else {
                    Count16_4
                }
            }
            Count15_4 => {
                if bit == true {
                    Count16_4
                } else {
                    Count15_5
                }
            }
            Count14_5 => {
                if bit == true {
                    Count15_5
                } else {
                    Count14_6
                }
            }
            Count13_6 => {
                if bit == true {
                    Count14_6
                } else {
                    Count13_7
                }
            }
            Count12_7 => {
                if bit == true {
                    Count13_7
                } else {
                    Count12_8
                }
            }
            Count11_8 => {
                if bit == true {
                    Count12_8
                } else {
                    Count11_9
                }
            }
            Count10_9 => {
                if bit == true {
                    Count11_9
                } else {
                    Count10_10
                }
            }
            Count9_10 => {
                if bit == false {
                    Count9_11
                } else {
                    Count10_10
                }
            }
            Count8_11 => {
                if bit == false {
                    Count8_12
                } else {
                    Count9_11
                }
            }
            Count7_12 => {
                if bit == false {
                    Count7_13
                } else {
                    Count8_12
                }
            }
            Count6_13 => {
                if bit == false {
                    Count6_14
                } else {
                    Count7_13
                }
            }
            Count5_14 => {
                if bit == false {
                    Count5_15
                } else {
                    Count6_14
                }
            }
            Count4_15 => {
                if bit == false {
                    Count4_16
                } else {
                    Count5_15
                }
            }
            Count3_16 => {
                if bit == false {
                    Count3_17
                } else {
                    Count4_16
                }
            }
            Count2_17 => {
                if bit == false {
                    Count2_18
                } else {
                    Count3_17
                }
            }
            Count1_18 => {
                if bit == false {
                    Count1_19
                } else {
                    Count2_18
                }
            }
            Count0_19 => {
                if bit == false {
                    Count0_20
                } else {
                    Count1_19
                }
            }
            Count20_0 => {
                if bit == true {
                    AllFalse4
                } else {
                    Count10_1
                }
            }
            Count19_1 => {
                if bit == true {
                    Count10_1
                } else {
                    Count10_1
                }
            }
            Count18_2 => {
                if bit == true {
                    Count10_1
                } else {
                    Count9_2
                }
            }
            Count17_3 => {
                if bit == true {
                    Count9_2
                } else {
                    Count9_2
                }
            }
            Count16_4 => {
                if bit == true {
                    Count9_2
                } else {
                    Count8_3
                }
            }
            Count15_5 => {
                if bit == true {
                    Count8_3
                } else {
                    Count8_3
                }
            }
            Count14_6 => {
                if bit == true {
                    Count8_3
                } else {
                    Count7_4
                }
            }
            Count13_7 => {
                if bit == true {
                    Count7_4
                } else {
                    Count7_4
                }
            }
            Count12_8 => {
                if bit == true {
                    Count7_4
                } else {
                    Count6_5
                }
            }
            Count11_9 => {
                if bit == true {
                    Count6_5
                } else {
                    Count6_5
                }
            }
            Count10_10 => {
                if bit == false {
                    Count6_5
                } else {
                    Count5_6
                }
            }
            Count9_11 => {
                if bit == false {
                    Count5_6
                } else {
                    Count5_6
                }
            }
            Count8_12 => {
                if bit == false {
                    Count4_7
                } else {
                    Count5_6
                }
            }
            Count7_13 => {
                if bit == false {
                    Count4_7
                } else {
                    Count4_7
                }
            }
            Count6_14 => {
                if bit == false {
                    Count3_8
                } else {
                    Count4_7
                }
            }
            Count5_15 => {
                if bit == false {
                    Count3_8
                } else {
                    Count3_8
                }
            }
            Count4_16 => {
                if bit == false {
                    Count2_9
                } else {
                    Count3_8
                }
            }
            Count3_17 => {
                if bit == false {
                    Count2_9
                } else {
                    Count2_9
                }
            }
            Count2_18 => {
                if bit == false {
                    Count1_10
                } else {
                    Count2_9
                }
            }
            Count1_19 => {
                if bit == false {
                    Count1_10
                } else {
                    Count1_10
                }
            }
            Count0_20 => {
                if bit == false {
                    AllTrue4
                } else {
                    Count1_10
                }
            }
            AllFalse4 => {
                if bit == false {
                    if rng.next() < 0xf800000000000000 {
                        AllFalse5
                    } else {
                        AllFalse4
                    }
                } else {
                    Count16_1
                }
            }
            AllTrue4 => {
                if bit == true {
                    if rng.next() < 0xf800000000000000 {
                        AllTrue5
                    } else {
                        AllTrue4
                    }
                } else {
                    Count1_16
                }
            }
            AllFalse5 => {
                if bit == false {
                    if rng.next() < 0xfc00000000000000 {
                        AllFalse6
                    } else {
                        AllFalse5
                    }
                } else {
                    Count16_1
                }
            }
            AllTrue5 => {
                if bit == true {
                    if rng.next() < 0xfc00000000000000 {
                        AllTrue6
                    } else {
                        AllTrue5
                    }
                } else {
                    Count1_16
                }
            }
            AllFalse6 => {
                if bit == false {
                    if rng.next() < 0xfe00000000000000 {
                        AllFalse7
                    } else {
                        AllFalse6
                    }
                } else {
                    Count16_1
                }
            }
            AllTrue6 => {
                if bit == true {
                    if rng.next() < 0xfe00000000000000 {
                        AllTrue7
                    } else {
                        AllTrue6
                    }
                } else {
                    Count1_16
                }
            }
            AllFalse7 => {
                if bit == false {
                    if rng.next() < 0xff00000000000000 {
                        AllFalse8
                    } else {
                        AllFalse7
                    }
                } else {
                    Count16_1
                }
            }
            AllTrue7 => {
                if bit == true {
                    if rng.next() < 0xff00000000000000 {
                        AllTrue8
                    } else {
                        AllTrue7
                    }
                } else {
                    Count1_16
                }
            }
            AllFalse8 => {
                if bit == false {
                    if rng.next() < 0xff80000000000000 {
                        AllFalse9
                    } else {
                        AllFalse8
                    }
                } else {
                    Count16_1
                }
            }
            AllTrue8 => {
                if bit == true {
                    if rng.next() < 0xff80000000000000 {
                        AllTrue9
                    } else {
                        AllTrue8
                    }
                } else {
                    Count1_16
                }
            }
            AllFalse9 => {
                if bit == false {
                    if rng.next() < 0xffc0000000000000 {
                        AllFalse10
                    } else {
                        AllFalse9
                    }
                } else {
                    Count16_1
                }
            }
            AllTrue9 => {
                if bit == true {
                    if rng.next() < 0xffc0000000000000 {
                        AllTrue10
                    } else {
                        AllTrue9
                    }
                } else {
                    Count1_16
                }
            }
            AllFalse10 => {
                if bit == false {
                    if rng.next() < 0xffe0000000000000 {
                        AllFalse11
                    } else {
                        AllFalse10
                    }
                } else {
                    Count16_1
                }
            }
            AllTrue10 => {
                if bit == true {
                    if rng.next() < 0xffe0000000000000 {
                        AllTrue11
                    } else {
                        AllTrue10
                    }
                } else {
                    Count1_16
                }
            }
            AllFalse11 => {
                if bit == false {
                    if rng.next() < 0xfff0000000000000 {
                        AllFalse12
                    } else {
                        AllFalse11
                    }
                } else {
                    Count16_1
                }
            }
            AllTrue11 => {
                if bit == true {
                    if rng.next() < 0xfff0000000000000 {
                        AllTrue12
                    } else {
                        AllTrue11
                    }
                } else {
                    Count1_16
                }
            }
            AllFalse12 => {
                if bit == false {
                    if rng.next() < 0xfff8000000000000 {
                        AllFalse13
                    } else {
                        AllFalse12
                    }
                } else {
                    Count16_1
                }
            }
            AllTrue12 => {
                if bit == true {
                    if rng.next() < 0xfff8000000000000 {
                        AllTrue13
                    } else {
                        AllTrue12
                    }
                } else {
                    Count1_16
                }
            }
            AllFalse13 => {
                if bit == false {
                    if rng.next() < 0xfffc000000000000 {
                        AllFalse14
                    } else {
                        AllFalse13
                    }
                } else {
                    Count16_1
                }
            }
            AllTrue13 => {
                if bit == true {
                    if rng.next() < 0xfffc000000000000 {
                        AllTrue14
                    } else {
                        AllTrue13
                    }
                } else {
                    Count1_16
                }
            }
            AllFalse14 => {
                if bit == false {
                    if rng.next() < 0xfffe000000000000 {
                        AllFalse15
                    } else {
                        AllFalse14
                    }
                } else {
                    Count16_1
                }
            }
            AllTrue14 => {
                if bit == true {
                    if rng.next() < 0xfffe000000000000 {
                        AllTrue15
                    } else {
                        AllTrue14
                    }
                } else {
                    Count1_16
                }
            }
            AllFalse15 => {
                if bit == false {
                    AllFalse15
                } else {
                    AllFalse15
                }
            }
            AllTrue15 => {
                if bit == true {
                    AllTrue15
                } else {
                    AllTrue15
                }
            }
        }
    }
}
// Count of variants: 255
