//! Generated with `cargo run --example bit-context > src/bit_context.rs`
use super::adapt::SplitMix64;
use super::arith::Probability;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitContext {
    #[default]
    True0False0, // Probability { prob: 2, shift: 2 } = 0.5
    True0False1,   // Probability { prob: 85, shift: 7 } = 0.6640625
    True1False0,   // Probability { prob: 85, shift: 8 } = 0.33203125
    True0False2,   // Probability { prob: 3, shift: 2 } = 0.75
    True1False1,   // Probability { prob: 2, shift: 2 } = 0.5
    True2False0,   // Probability { prob: 1, shift: 2 } = 0.25
    True0False3,   // Probability { prob: 51, shift: 6 } = 0.796875
    True1False2,   // Probability { prob: 85, shift: 7 } = 0.6640625
    True2False1,   // Probability { prob: 85, shift: 8 } = 0.33203125
    True3False0,   // Probability { prob: 51, shift: 8 } = 0.19921875
    True0False4,   // Probability { prob: 213, shift: 8 } = 0.83203125
    True1False3,   // Probability { prob: 3, shift: 2 } = 0.75
    True2False2,   // Probability { prob: 2, shift: 2 } = 0.5
    True3False1,   // Probability { prob: 1, shift: 2 } = 0.25
    True4False0,   // Probability { prob: 21, shift: 7 } = 0.1640625
    True0False5,   // Probability { prob: 219, shift: 8 } = 0.85546875
    True1False4,   // Probability { prob: 51, shift: 6 } = 0.796875
    True2False3,   // Probability { prob: 153, shift: 8 } = 0.59765625
    True3False2,   // Probability { prob: 51, shift: 7 } = 0.3984375
    True4False1,   // Probability { prob: 51, shift: 8 } = 0.19921875
    True5False0,   // Probability { prob: 9, shift: 6 } = 0.140625
    True0False6,   // Probability { prob: 7, shift: 3 } = 0.875
    True1False5,   // Probability { prob: 213, shift: 8 } = 0.83203125
    True2False4,   // Probability { prob: 85, shift: 7 } = 0.6640625
    True3False3,   // Probability { prob: 2, shift: 2 } = 0.5
    True4False2,   // Probability { prob: 85, shift: 8 } = 0.33203125
    True5False1,   // Probability { prob: 21, shift: 7 } = 0.1640625
    True6False0,   // Probability { prob: 1, shift: 3 } = 0.125
    True0False7,   // Probability { prob: 227, shift: 8 } = 0.88671875
    True1False6,   // Probability { prob: 219, shift: 8 } = 0.85546875
    True2False5,   // Probability { prob: 91, shift: 7 } = 0.7109375
    True3False4,   // Probability { prob: 73, shift: 7 } = 0.5703125
    True4False3,   // Probability { prob: 109, shift: 8 } = 0.42578125
    True5False2,   // Probability { prob: 73, shift: 8 } = 0.28515625
    True6False1,   // Probability { prob: 9, shift: 6 } = 0.140625
    True7False0,   // Probability { prob: 7, shift: 6 } = 0.109375
    True0False8,   // Probability { prob: 115, shift: 7 } = 0.8984375
    True1False7,   // Probability { prob: 7, shift: 3 } = 0.875
    True2False6,   // Probability { prob: 3, shift: 2 } = 0.75
    True3False5,   // Probability { prob: 5, shift: 3 } = 0.625
    True4False4,   // Probability { prob: 2, shift: 2 } = 0.5
    True5False3,   // Probability { prob: 3, shift: 3 } = 0.375
    True6False2,   // Probability { prob: 1, shift: 2 } = 0.25
    True7False1,   // Probability { prob: 1, shift: 3 } = 0.125
    True8False0,   // Probability { prob: 25, shift: 8 } = 0.09765625
    True0False9,   // Probability { prob: 29, shift: 5 } = 0.90625
    True1False8,   // Probability { prob: 227, shift: 8 } = 0.88671875
    True2False7,   // Probability { prob: 199, shift: 8 } = 0.77734375
    True3False6,   // Probability { prob: 85, shift: 7 } = 0.6640625
    True4False5,   // Probability { prob: 71, shift: 7 } = 0.5546875
    True5False4,   // Probability { prob: 113, shift: 8 } = 0.44140625
    True6False3,   // Probability { prob: 85, shift: 8 } = 0.33203125
    True7False2,   // Probability { prob: 7, shift: 5 } = 0.21875
    True8False1,   // Probability { prob: 7, shift: 6 } = 0.109375
    True9False0,   // Probability { prob: 23, shift: 8 } = 0.08984375
    True0False10,  // Probability { prob: 117, shift: 7 } = 0.9140625
    True1False9,   // Probability { prob: 115, shift: 7 } = 0.8984375
    True2False8,   // Probability { prob: 51, shift: 6 } = 0.796875
    True3False7,   // Probability { prob: 179, shift: 8 } = 0.69921875
    True4False6,   // Probability { prob: 153, shift: 8 } = 0.59765625
    True5False5,   // Probability { prob: 2, shift: 2 } = 0.5
    True6False4,   // Probability { prob: 51, shift: 7 } = 0.3984375
    True7False3,   // Probability { prob: 19, shift: 6 } = 0.296875
    True8False2,   // Probability { prob: 51, shift: 8 } = 0.19921875
    True9False1,   // Probability { prob: 25, shift: 8 } = 0.09765625
    True10False0,  // Probability { prob: 21, shift: 8 } = 0.08203125
    True0False11,  // Probability { prob: 59, shift: 6 } = 0.921875
    True1False10,  // Probability { prob: 29, shift: 5 } = 0.90625
    True2False9,   // Probability { prob: 209, shift: 8 } = 0.81640625
    True3False8,   // Probability { prob: 93, shift: 7 } = 0.7265625
    True4False7,   // Probability { prob: 81, shift: 7 } = 0.6328125
    True5False6,   // Probability { prob: 139, shift: 8 } = 0.54296875
    True6False5,   // Probability { prob: 29, shift: 6 } = 0.453125
    True7False4,   // Probability { prob: 93, shift: 8 } = 0.36328125
    True8False3,   // Probability { prob: 69, shift: 8 } = 0.26953125
    True9False2,   // Probability { prob: 23, shift: 7 } = 0.1796875
    True10False1,  // Probability { prob: 23, shift: 8 } = 0.08984375
    True11False0,  // Probability { prob: 19, shift: 8 } = 0.07421875
    True0False12,  // Probability { prob: 237, shift: 8 } = 0.92578125
    True1False11,  // Probability { prob: 117, shift: 7 } = 0.9140625
    True2False10,  // Probability { prob: 213, shift: 8 } = 0.83203125
    True3False9,   // Probability { prob: 3, shift: 2 } = 0.75
    True4False8,   // Probability { prob: 85, shift: 7 } = 0.6640625
    True5False7,   // Probability { prob: 149, shift: 8 } = 0.58203125
    True6False6,   // Probability { prob: 2, shift: 2 } = 0.5
    True7False5,   // Probability { prob: 53, shift: 7 } = 0.4140625
    True8False4,   // Probability { prob: 85, shift: 8 } = 0.33203125
    True9False3,   // Probability { prob: 1, shift: 2 } = 0.25
    True10False2,  // Probability { prob: 21, shift: 7 } = 0.1640625
    True11False1,  // Probability { prob: 21, shift: 8 } = 0.08203125
    True12False0,  // Probability { prob: 9, shift: 7 } = 0.0703125
    True0False13,  // Probability { prob: 119, shift: 7 } = 0.9296875
    True1False12,  // Probability { prob: 59, shift: 6 } = 0.921875
    True2False11,  // Probability { prob: 27, shift: 5 } = 0.84375
    True3False10,  // Probability { prob: 49, shift: 6 } = 0.765625
    True4False9,   // Probability { prob: 177, shift: 8 } = 0.69140625
    True5False8,   // Probability { prob: 157, shift: 8 } = 0.61328125
    True6False7,   // Probability { prob: 137, shift: 8 } = 0.53515625
    True7False6,   // Probability { prob: 59, shift: 7 } = 0.4609375
    True8False5,   // Probability { prob: 49, shift: 7 } = 0.3828125
    True9False4,   // Probability { prob: 39, shift: 7 } = 0.3046875
    True10False3,  // Probability { prob: 59, shift: 8 } = 0.23046875
    True11False2,  // Probability { prob: 39, shift: 8 } = 0.15234375
    True12False1,  // Probability { prob: 19, shift: 8 } = 0.07421875
    True13False0,  // Probability { prob: 17, shift: 8 } = 0.06640625
    True0False14,  // Probability { prob: 15, shift: 4 } = 0.9375
    True1False13,  // Probability { prob: 237, shift: 8 } = 0.92578125
    True2False12,  // Probability { prob: 219, shift: 8 } = 0.85546875
    True3False11,  // Probability { prob: 201, shift: 8 } = 0.78515625
    True4False10,  // Probability { prob: 91, shift: 7 } = 0.7109375
    True5False9,   // Probability { prob: 41, shift: 6 } = 0.640625
    True6False8,   // Probability { prob: 73, shift: 7 } = 0.5703125
    True7False7,   // Probability { prob: 2, shift: 2 } = 0.5
    True8False6,   // Probability { prob: 109, shift: 8 } = 0.42578125
    True9False5,   // Probability { prob: 91, shift: 8 } = 0.35546875
    True10False4,  // Probability { prob: 73, shift: 8 } = 0.28515625
    True11False3,  // Probability { prob: 27, shift: 7 } = 0.2109375
    True12False2,  // Probability { prob: 9, shift: 6 } = 0.140625
    True13False1,  // Probability { prob: 9, shift: 7 } = 0.0703125
    True14False0,  // Probability { prob: 1, shift: 4 } = 0.0625
    True0False15,  // Probability { prob: 15, shift: 4 } = 0.9375
    True1False14,  // Probability { prob: 119, shift: 7 } = 0.9296875
    True2False13,  // Probability { prob: 221, shift: 8 } = 0.86328125
    True3False12,  // Probability { prob: 51, shift: 6 } = 0.796875
    True4False11,  // Probability { prob: 187, shift: 8 } = 0.73046875
    True5False10,  // Probability { prob: 85, shift: 7 } = 0.6640625
    True6False9,   // Probability { prob: 153, shift: 8 } = 0.59765625
    True7False8,   // Probability { prob: 17, shift: 5 } = 0.53125
    True8False7,   // Probability { prob: 119, shift: 8 } = 0.46484375
    True9False6,   // Probability { prob: 51, shift: 7 } = 0.3984375
    True10False5,  // Probability { prob: 85, shift: 8 } = 0.33203125
    True11False4,  // Probability { prob: 17, shift: 6 } = 0.265625
    True12False3,  // Probability { prob: 51, shift: 8 } = 0.19921875
    True13False2,  // Probability { prob: 17, shift: 7 } = 0.1328125
    True14False1,  // Probability { prob: 17, shift: 8 } = 0.06640625
    True15False0,  // Probability { prob: 15, shift: 8 } = 0.05859375
    True0False16,  // Probability { prob: 241, shift: 8 } = 0.94140625
    True1False15,  // Probability { prob: 15, shift: 4 } = 0.9375
    True2False14,  // Probability { prob: 7, shift: 3 } = 0.875
    True3False13,  // Probability { prob: 13, shift: 4 } = 0.8125
    True4False12,  // Probability { prob: 3, shift: 2 } = 0.75
    True5False11,  // Probability { prob: 11, shift: 4 } = 0.6875
    True6False10,  // Probability { prob: 5, shift: 3 } = 0.625
    True7False9,   // Probability { prob: 9, shift: 4 } = 0.5625
    True8False8,   // Probability { prob: 2, shift: 2 } = 0.5
    True9False7,   // Probability { prob: 7, shift: 4 } = 0.4375
    True10False6,  // Probability { prob: 3, shift: 3 } = 0.375
    True11False5,  // Probability { prob: 5, shift: 4 } = 0.3125
    True12False4,  // Probability { prob: 1, shift: 2 } = 0.25
    True13False3,  // Probability { prob: 3, shift: 4 } = 0.1875
    True14False2,  // Probability { prob: 1, shift: 3 } = 0.125
    True15False1,  // Probability { prob: 1, shift: 4 } = 0.0625
    True16False0,  // Probability { prob: 7, shift: 7 } = 0.0546875
    True0False17,  // Probability { prob: 121, shift: 7 } = 0.9453125
    True1False16,  // Probability { prob: 15, shift: 4 } = 0.9375
    True2False15,  // Probability { prob: 225, shift: 8 } = 0.87890625
    True3False14,  // Probability { prob: 105, shift: 7 } = 0.8203125
    True4False13,  // Probability { prob: 195, shift: 8 } = 0.76171875
    True5False12,  // Probability { prob: 45, shift: 6 } = 0.703125
    True6False11,  // Probability { prob: 165, shift: 8 } = 0.64453125
    True7False10,  // Probability { prob: 75, shift: 7 } = 0.5859375
    True8False9,   // Probability { prob: 135, shift: 8 } = 0.52734375
    True9False8,   // Probability { prob: 15, shift: 5 } = 0.46875
    True10False7,  // Probability { prob: 105, shift: 8 } = 0.41015625
    True11False6,  // Probability { prob: 45, shift: 7 } = 0.3515625
    True12False5,  // Probability { prob: 75, shift: 8 } = 0.29296875
    True13False4,  // Probability { prob: 15, shift: 6 } = 0.234375
    True14False3,  // Probability { prob: 45, shift: 8 } = 0.17578125
    True15False2,  // Probability { prob: 15, shift: 7 } = 0.1171875
    True16False1,  // Probability { prob: 15, shift: 8 } = 0.05859375
    True17False0,  // Probability { prob: 13, shift: 8 } = 0.05078125
    True0False18,  // Probability { prob: 243, shift: 8 } = 0.94921875
    True1False17,  // Probability { prob: 241, shift: 8 } = 0.94140625
    True2False16,  // Probability { prob: 227, shift: 8 } = 0.88671875
    True3False15,  // Probability { prob: 213, shift: 8 } = 0.83203125
    True4False14,  // Probability { prob: 199, shift: 8 } = 0.77734375
    True5False13,  // Probability { prob: 23, shift: 5 } = 0.71875
    True6False12,  // Probability { prob: 85, shift: 7 } = 0.6640625
    True7False11,  // Probability { prob: 39, shift: 6 } = 0.609375
    True8False10,  // Probability { prob: 71, shift: 7 } = 0.5546875
    True9False9,   // Probability { prob: 2, shift: 2 } = 0.5
    True10False8,  // Probability { prob: 113, shift: 8 } = 0.44140625
    True11False7,  // Probability { prob: 99, shift: 8 } = 0.38671875
    True12False6,  // Probability { prob: 85, shift: 8 } = 0.33203125
    True13False5,  // Probability { prob: 71, shift: 8 } = 0.27734375
    True14False4,  // Probability { prob: 7, shift: 5 } = 0.21875
    True15False3,  // Probability { prob: 21, shift: 7 } = 0.1640625
    True16False2,  // Probability { prob: 7, shift: 6 } = 0.109375
    True17False1,  // Probability { prob: 7, shift: 7 } = 0.0546875
    True18False0,  // Probability { prob: 3, shift: 6 } = 0.046875
    True0False19,  // Probability { prob: 243, shift: 8 } = 0.94921875
    True1False18,  // Probability { prob: 121, shift: 7 } = 0.9453125
    True2False17,  // Probability { prob: 229, shift: 8 } = 0.89453125
    True3False16,  // Probability { prob: 215, shift: 8 } = 0.83984375
    True4False15,  // Probability { prob: 101, shift: 7 } = 0.7890625
    True5False14,  // Probability { prob: 47, shift: 6 } = 0.734375
    True6False13,  // Probability { prob: 175, shift: 8 } = 0.68359375
    True7False12,  // Probability { prob: 161, shift: 8 } = 0.62890625
    True8False11,  // Probability { prob: 37, shift: 6 } = 0.578125
    True9False10,  // Probability { prob: 67, shift: 7 } = 0.5234375
    True10False9,  // Probability { prob: 121, shift: 8 } = 0.47265625
    True11False8,  // Probability { prob: 107, shift: 8 } = 0.41796875
    True12False7,  // Probability { prob: 47, shift: 7 } = 0.3671875
    True13False6,  // Probability { prob: 5, shift: 4 } = 0.3125
    True14False5,  // Probability { prob: 67, shift: 8 } = 0.26171875
    True15False4,  // Probability { prob: 53, shift: 8 } = 0.20703125
    True16False3,  // Probability { prob: 5, shift: 5 } = 0.15625
    True17False2,  // Probability { prob: 13, shift: 7 } = 0.1015625
    True18False1,  // Probability { prob: 13, shift: 8 } = 0.05078125
    True19False0,  // Probability { prob: 3, shift: 6 } = 0.046875
    True0False20,  // Probability { prob: 61, shift: 6 } = 0.953125
    True1False19,  // Probability { prob: 243, shift: 8 } = 0.94921875
    True2False18,  // Probability { prob: 115, shift: 7 } = 0.8984375
    True3False17,  // Probability { prob: 217, shift: 8 } = 0.84765625
    True4False16,  // Probability { prob: 51, shift: 6 } = 0.796875
    True5False15,  // Probability { prob: 3, shift: 2 } = 0.75
    True6False14,  // Probability { prob: 179, shift: 8 } = 0.69921875
    True7False13,  // Probability { prob: 83, shift: 7 } = 0.6484375
    True8False12,  // Probability { prob: 153, shift: 8 } = 0.59765625
    True9False11,  // Probability { prob: 35, shift: 6 } = 0.546875
    True10False10, // Probability { prob: 2, shift: 2 } = 0.5
    True11False9,  // Probability { prob: 115, shift: 8 } = 0.44921875
    True12False8,  // Probability { prob: 51, shift: 7 } = 0.3984375
    True13False7,  // Probability { prob: 89, shift: 8 } = 0.34765625
    True14False6,  // Probability { prob: 19, shift: 6 } = 0.296875
    True15False5,  // Probability { prob: 1, shift: 2 } = 0.25
    True16False4,  // Probability { prob: 51, shift: 8 } = 0.19921875
    True17False3,  // Probability { prob: 19, shift: 7 } = 0.1484375
    True18False2,  // Probability { prob: 25, shift: 8 } = 0.09765625
    True19False1,  // Probability { prob: 3, shift: 6 } = 0.046875
    True20False0,  // Probability { prob: 11, shift: 8 } = 0.04296875
    AllFalse4,     // Probability { prob: 31, shift: 5 } = 0.96875
    AllTrue4,      // Probability { prob: 1, shift: 5 } = 0.03125
    AllFalse5,     // Probability { prob: 63, shift: 6 } = 0.984375
    AllTrue5,      // Probability { prob: 1, shift: 6 } = 0.015625
    AllFalse6,     // Probability { prob: 127, shift: 7 } = 0.9921875
    AllTrue6,      // Probability { prob: 1, shift: 7 } = 0.0078125
    AllFalse7,     // Probability { prob: 255, shift: 8 } = 0.99609375
    AllTrue7,      // Probability { prob: 1, shift: 8 } = 0.00390625
    AllFalse8,     // Probability { prob: 511, shift: 9 } = 0.998046875
    AllTrue8,      // Probability { prob: 1, shift: 9 } = 0.001953125
    AllFalse9,     // Probability { prob: 1023, shift: 10 } = 0.9990234375
    AllTrue9,      // Probability { prob: 1, shift: 10 } = 0.0009765625
    AllFalse10,    // Probability { prob: 2047, shift: 11 } = 0.99951171875
    AllTrue10,     // Probability { prob: 1, shift: 11 } = 0.00048828125
    AllFalse11,    // Probability { prob: 4095, shift: 12 } = 0.999755859375
    AllTrue11,     // Probability { prob: 1, shift: 12 } = 0.000244140625
    AllFalse12,    // Probability { prob: 8191, shift: 13 } = 0.9998779296875
    AllTrue12,     // Probability { prob: 1, shift: 13 } = 0.0001220703125
    AllFalse13,    // Probability { prob: 16383, shift: 14 } = 0.99993896484375
    AllTrue13,     // Probability { prob: 1, shift: 14 } = 0.00006103515625
    AllFalse14,    // Probability { prob: 32767, shift: 15 } = 0.999969482421875
    AllTrue14,     // Probability { prob: 1, shift: 15 } = 0.000030517578125
    AllFalse15,    // Probability { prob: 65535, shift: 16 } = 0.9999847412109375
    AllTrue15,     // Probability { prob: 1, shift: 16 } = 0.0000152587890625
}
use BitContext::*;

impl BitContext {
    pub fn probability(self) -> Probability {
        match self {
            True0False0 => Probability { prob: 2, shift: 2 },
            True0False1 => Probability { prob: 85, shift: 7 },
            True1False0 => Probability { prob: 85, shift: 8 },
            True0False2 => Probability { prob: 3, shift: 2 },
            True1False1 => Probability { prob: 2, shift: 2 },
            True2False0 => Probability { prob: 1, shift: 2 },
            True0False3 => Probability { prob: 51, shift: 6 },
            True1False2 => Probability { prob: 85, shift: 7 },
            True2False1 => Probability { prob: 85, shift: 8 },
            True3False0 => Probability { prob: 51, shift: 8 },
            True0False4 => Probability {
                prob: 213,
                shift: 8,
            },
            True1False3 => Probability { prob: 3, shift: 2 },
            True2False2 => Probability { prob: 2, shift: 2 },
            True3False1 => Probability { prob: 1, shift: 2 },
            True4False0 => Probability { prob: 21, shift: 7 },
            True0False5 => Probability {
                prob: 219,
                shift: 8,
            },
            True1False4 => Probability { prob: 51, shift: 6 },
            True2False3 => Probability {
                prob: 153,
                shift: 8,
            },
            True3False2 => Probability { prob: 51, shift: 7 },
            True4False1 => Probability { prob: 51, shift: 8 },
            True5False0 => Probability { prob: 9, shift: 6 },
            True0False6 => Probability { prob: 7, shift: 3 },
            True1False5 => Probability {
                prob: 213,
                shift: 8,
            },
            True2False4 => Probability { prob: 85, shift: 7 },
            True3False3 => Probability { prob: 2, shift: 2 },
            True4False2 => Probability { prob: 85, shift: 8 },
            True5False1 => Probability { prob: 21, shift: 7 },
            True6False0 => Probability { prob: 1, shift: 3 },
            True0False7 => Probability {
                prob: 227,
                shift: 8,
            },
            True1False6 => Probability {
                prob: 219,
                shift: 8,
            },
            True2False5 => Probability { prob: 91, shift: 7 },
            True3False4 => Probability { prob: 73, shift: 7 },
            True4False3 => Probability {
                prob: 109,
                shift: 8,
            },
            True5False2 => Probability { prob: 73, shift: 8 },
            True6False1 => Probability { prob: 9, shift: 6 },
            True7False0 => Probability { prob: 7, shift: 6 },
            True0False8 => Probability {
                prob: 115,
                shift: 7,
            },
            True1False7 => Probability { prob: 7, shift: 3 },
            True2False6 => Probability { prob: 3, shift: 2 },
            True3False5 => Probability { prob: 5, shift: 3 },
            True4False4 => Probability { prob: 2, shift: 2 },
            True5False3 => Probability { prob: 3, shift: 3 },
            True6False2 => Probability { prob: 1, shift: 2 },
            True7False1 => Probability { prob: 1, shift: 3 },
            True8False0 => Probability { prob: 25, shift: 8 },
            True0False9 => Probability { prob: 29, shift: 5 },
            True1False8 => Probability {
                prob: 227,
                shift: 8,
            },
            True2False7 => Probability {
                prob: 199,
                shift: 8,
            },
            True3False6 => Probability { prob: 85, shift: 7 },
            True4False5 => Probability { prob: 71, shift: 7 },
            True5False4 => Probability {
                prob: 113,
                shift: 8,
            },
            True6False3 => Probability { prob: 85, shift: 8 },
            True7False2 => Probability { prob: 7, shift: 5 },
            True8False1 => Probability { prob: 7, shift: 6 },
            True9False0 => Probability { prob: 23, shift: 8 },
            True0False10 => Probability {
                prob: 117,
                shift: 7,
            },
            True1False9 => Probability {
                prob: 115,
                shift: 7,
            },
            True2False8 => Probability { prob: 51, shift: 6 },
            True3False7 => Probability {
                prob: 179,
                shift: 8,
            },
            True4False6 => Probability {
                prob: 153,
                shift: 8,
            },
            True5False5 => Probability { prob: 2, shift: 2 },
            True6False4 => Probability { prob: 51, shift: 7 },
            True7False3 => Probability { prob: 19, shift: 6 },
            True8False2 => Probability { prob: 51, shift: 8 },
            True9False1 => Probability { prob: 25, shift: 8 },
            True10False0 => Probability { prob: 21, shift: 8 },
            True0False11 => Probability { prob: 59, shift: 6 },
            True1False10 => Probability { prob: 29, shift: 5 },
            True2False9 => Probability {
                prob: 209,
                shift: 8,
            },
            True3False8 => Probability { prob: 93, shift: 7 },
            True4False7 => Probability { prob: 81, shift: 7 },
            True5False6 => Probability {
                prob: 139,
                shift: 8,
            },
            True6False5 => Probability { prob: 29, shift: 6 },
            True7False4 => Probability { prob: 93, shift: 8 },
            True8False3 => Probability { prob: 69, shift: 8 },
            True9False2 => Probability { prob: 23, shift: 7 },
            True10False1 => Probability { prob: 23, shift: 8 },
            True11False0 => Probability { prob: 19, shift: 8 },
            True0False12 => Probability {
                prob: 237,
                shift: 8,
            },
            True1False11 => Probability {
                prob: 117,
                shift: 7,
            },
            True2False10 => Probability {
                prob: 213,
                shift: 8,
            },
            True3False9 => Probability { prob: 3, shift: 2 },
            True4False8 => Probability { prob: 85, shift: 7 },
            True5False7 => Probability {
                prob: 149,
                shift: 8,
            },
            True6False6 => Probability { prob: 2, shift: 2 },
            True7False5 => Probability { prob: 53, shift: 7 },
            True8False4 => Probability { prob: 85, shift: 8 },
            True9False3 => Probability { prob: 1, shift: 2 },
            True10False2 => Probability { prob: 21, shift: 7 },
            True11False1 => Probability { prob: 21, shift: 8 },
            True12False0 => Probability { prob: 9, shift: 7 },
            True0False13 => Probability {
                prob: 119,
                shift: 7,
            },
            True1False12 => Probability { prob: 59, shift: 6 },
            True2False11 => Probability { prob: 27, shift: 5 },
            True3False10 => Probability { prob: 49, shift: 6 },
            True4False9 => Probability {
                prob: 177,
                shift: 8,
            },
            True5False8 => Probability {
                prob: 157,
                shift: 8,
            },
            True6False7 => Probability {
                prob: 137,
                shift: 8,
            },
            True7False6 => Probability { prob: 59, shift: 7 },
            True8False5 => Probability { prob: 49, shift: 7 },
            True9False4 => Probability { prob: 39, shift: 7 },
            True10False3 => Probability { prob: 59, shift: 8 },
            True11False2 => Probability { prob: 39, shift: 8 },
            True12False1 => Probability { prob: 19, shift: 8 },
            True13False0 => Probability { prob: 17, shift: 8 },
            True0False14 => Probability { prob: 15, shift: 4 },
            True1False13 => Probability {
                prob: 237,
                shift: 8,
            },
            True2False12 => Probability {
                prob: 219,
                shift: 8,
            },
            True3False11 => Probability {
                prob: 201,
                shift: 8,
            },
            True4False10 => Probability { prob: 91, shift: 7 },
            True5False9 => Probability { prob: 41, shift: 6 },
            True6False8 => Probability { prob: 73, shift: 7 },
            True7False7 => Probability { prob: 2, shift: 2 },
            True8False6 => Probability {
                prob: 109,
                shift: 8,
            },
            True9False5 => Probability { prob: 91, shift: 8 },
            True10False4 => Probability { prob: 73, shift: 8 },
            True11False3 => Probability { prob: 27, shift: 7 },
            True12False2 => Probability { prob: 9, shift: 6 },
            True13False1 => Probability { prob: 9, shift: 7 },
            True14False0 => Probability { prob: 1, shift: 4 },
            True0False15 => Probability { prob: 15, shift: 4 },
            True1False14 => Probability {
                prob: 119,
                shift: 7,
            },
            True2False13 => Probability {
                prob: 221,
                shift: 8,
            },
            True3False12 => Probability { prob: 51, shift: 6 },
            True4False11 => Probability {
                prob: 187,
                shift: 8,
            },
            True5False10 => Probability { prob: 85, shift: 7 },
            True6False9 => Probability {
                prob: 153,
                shift: 8,
            },
            True7False8 => Probability { prob: 17, shift: 5 },
            True8False7 => Probability {
                prob: 119,
                shift: 8,
            },
            True9False6 => Probability { prob: 51, shift: 7 },
            True10False5 => Probability { prob: 85, shift: 8 },
            True11False4 => Probability { prob: 17, shift: 6 },
            True12False3 => Probability { prob: 51, shift: 8 },
            True13False2 => Probability { prob: 17, shift: 7 },
            True14False1 => Probability { prob: 17, shift: 8 },
            True15False0 => Probability { prob: 15, shift: 8 },
            True0False16 => Probability {
                prob: 241,
                shift: 8,
            },
            True1False15 => Probability { prob: 15, shift: 4 },
            True2False14 => Probability { prob: 7, shift: 3 },
            True3False13 => Probability { prob: 13, shift: 4 },
            True4False12 => Probability { prob: 3, shift: 2 },
            True5False11 => Probability { prob: 11, shift: 4 },
            True6False10 => Probability { prob: 5, shift: 3 },
            True7False9 => Probability { prob: 9, shift: 4 },
            True8False8 => Probability { prob: 2, shift: 2 },
            True9False7 => Probability { prob: 7, shift: 4 },
            True10False6 => Probability { prob: 3, shift: 3 },
            True11False5 => Probability { prob: 5, shift: 4 },
            True12False4 => Probability { prob: 1, shift: 2 },
            True13False3 => Probability { prob: 3, shift: 4 },
            True14False2 => Probability { prob: 1, shift: 3 },
            True15False1 => Probability { prob: 1, shift: 4 },
            True16False0 => Probability { prob: 7, shift: 7 },
            True0False17 => Probability {
                prob: 121,
                shift: 7,
            },
            True1False16 => Probability { prob: 15, shift: 4 },
            True2False15 => Probability {
                prob: 225,
                shift: 8,
            },
            True3False14 => Probability {
                prob: 105,
                shift: 7,
            },
            True4False13 => Probability {
                prob: 195,
                shift: 8,
            },
            True5False12 => Probability { prob: 45, shift: 6 },
            True6False11 => Probability {
                prob: 165,
                shift: 8,
            },
            True7False10 => Probability { prob: 75, shift: 7 },
            True8False9 => Probability {
                prob: 135,
                shift: 8,
            },
            True9False8 => Probability { prob: 15, shift: 5 },
            True10False7 => Probability {
                prob: 105,
                shift: 8,
            },
            True11False6 => Probability { prob: 45, shift: 7 },
            True12False5 => Probability { prob: 75, shift: 8 },
            True13False4 => Probability { prob: 15, shift: 6 },
            True14False3 => Probability { prob: 45, shift: 8 },
            True15False2 => Probability { prob: 15, shift: 7 },
            True16False1 => Probability { prob: 15, shift: 8 },
            True17False0 => Probability { prob: 13, shift: 8 },
            True0False18 => Probability {
                prob: 243,
                shift: 8,
            },
            True1False17 => Probability {
                prob: 241,
                shift: 8,
            },
            True2False16 => Probability {
                prob: 227,
                shift: 8,
            },
            True3False15 => Probability {
                prob: 213,
                shift: 8,
            },
            True4False14 => Probability {
                prob: 199,
                shift: 8,
            },
            True5False13 => Probability { prob: 23, shift: 5 },
            True6False12 => Probability { prob: 85, shift: 7 },
            True7False11 => Probability { prob: 39, shift: 6 },
            True8False10 => Probability { prob: 71, shift: 7 },
            True9False9 => Probability { prob: 2, shift: 2 },
            True10False8 => Probability {
                prob: 113,
                shift: 8,
            },
            True11False7 => Probability { prob: 99, shift: 8 },
            True12False6 => Probability { prob: 85, shift: 8 },
            True13False5 => Probability { prob: 71, shift: 8 },
            True14False4 => Probability { prob: 7, shift: 5 },
            True15False3 => Probability { prob: 21, shift: 7 },
            True16False2 => Probability { prob: 7, shift: 6 },
            True17False1 => Probability { prob: 7, shift: 7 },
            True18False0 => Probability { prob: 3, shift: 6 },
            True0False19 => Probability {
                prob: 243,
                shift: 8,
            },
            True1False18 => Probability {
                prob: 121,
                shift: 7,
            },
            True2False17 => Probability {
                prob: 229,
                shift: 8,
            },
            True3False16 => Probability {
                prob: 215,
                shift: 8,
            },
            True4False15 => Probability {
                prob: 101,
                shift: 7,
            },
            True5False14 => Probability { prob: 47, shift: 6 },
            True6False13 => Probability {
                prob: 175,
                shift: 8,
            },
            True7False12 => Probability {
                prob: 161,
                shift: 8,
            },
            True8False11 => Probability { prob: 37, shift: 6 },
            True9False10 => Probability { prob: 67, shift: 7 },
            True10False9 => Probability {
                prob: 121,
                shift: 8,
            },
            True11False8 => Probability {
                prob: 107,
                shift: 8,
            },
            True12False7 => Probability { prob: 47, shift: 7 },
            True13False6 => Probability { prob: 5, shift: 4 },
            True14False5 => Probability { prob: 67, shift: 8 },
            True15False4 => Probability { prob: 53, shift: 8 },
            True16False3 => Probability { prob: 5, shift: 5 },
            True17False2 => Probability { prob: 13, shift: 7 },
            True18False1 => Probability { prob: 13, shift: 8 },
            True19False0 => Probability { prob: 3, shift: 6 },
            True0False20 => Probability { prob: 61, shift: 6 },
            True1False19 => Probability {
                prob: 243,
                shift: 8,
            },
            True2False18 => Probability {
                prob: 115,
                shift: 7,
            },
            True3False17 => Probability {
                prob: 217,
                shift: 8,
            },
            True4False16 => Probability { prob: 51, shift: 6 },
            True5False15 => Probability { prob: 3, shift: 2 },
            True6False14 => Probability {
                prob: 179,
                shift: 8,
            },
            True7False13 => Probability { prob: 83, shift: 7 },
            True8False12 => Probability {
                prob: 153,
                shift: 8,
            },
            True9False11 => Probability { prob: 35, shift: 6 },
            True10False10 => Probability { prob: 2, shift: 2 },
            True11False9 => Probability {
                prob: 115,
                shift: 8,
            },
            True12False8 => Probability { prob: 51, shift: 7 },
            True13False7 => Probability { prob: 89, shift: 8 },
            True14False6 => Probability { prob: 19, shift: 6 },
            True15False5 => Probability { prob: 1, shift: 2 },
            True16False4 => Probability { prob: 51, shift: 8 },
            True17False3 => Probability { prob: 19, shift: 7 },
            True18False2 => Probability { prob: 25, shift: 8 },
            True19False1 => Probability { prob: 3, shift: 6 },
            True20False0 => Probability { prob: 11, shift: 8 },
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
            True0False0 => {
                if bit == false {
                    True0False1
                } else {
                    True1False0
                }
            }
            True0False1 => {
                if bit == false {
                    True0False2
                } else {
                    True1False1
                }
            }
            True1False0 => {
                if bit == true {
                    True2False0
                } else {
                    True1False1
                }
            }
            True0False2 => {
                if bit == false {
                    True0False3
                } else {
                    True1False2
                }
            }
            True1False1 => {
                if bit == false {
                    True1False2
                } else {
                    True2False1
                }
            }
            True2False0 => {
                if bit == true {
                    True3False0
                } else {
                    True2False1
                }
            }
            True0False3 => {
                if bit == false {
                    True0False4
                } else {
                    True1False3
                }
            }
            True1False2 => {
                if bit == false {
                    True1False3
                } else {
                    True2False2
                }
            }
            True2False1 => {
                if bit == true {
                    True3False1
                } else {
                    True2False2
                }
            }
            True3False0 => {
                if bit == true {
                    True4False0
                } else {
                    True3False1
                }
            }
            True0False4 => {
                if bit == false {
                    True0False5
                } else {
                    True1False4
                }
            }
            True1False3 => {
                if bit == false {
                    True1False4
                } else {
                    True2False3
                }
            }
            True2False2 => {
                if bit == false {
                    True2False3
                } else {
                    True3False2
                }
            }
            True3False1 => {
                if bit == true {
                    True4False1
                } else {
                    True3False2
                }
            }
            True4False0 => {
                if bit == true {
                    True5False0
                } else {
                    True4False1
                }
            }
            True0False5 => {
                if bit == false {
                    True0False6
                } else {
                    True1False5
                }
            }
            True1False4 => {
                if bit == false {
                    True1False5
                } else {
                    True2False4
                }
            }
            True2False3 => {
                if bit == false {
                    True2False4
                } else {
                    True3False3
                }
            }
            True3False2 => {
                if bit == true {
                    True4False2
                } else {
                    True3False3
                }
            }
            True4False1 => {
                if bit == true {
                    True5False1
                } else {
                    True4False2
                }
            }
            True5False0 => {
                if bit == true {
                    True6False0
                } else {
                    True5False1
                }
            }
            True0False6 => {
                if bit == false {
                    True0False7
                } else {
                    True1False6
                }
            }
            True1False5 => {
                if bit == false {
                    True1False6
                } else {
                    True2False5
                }
            }
            True2False4 => {
                if bit == false {
                    True2False5
                } else {
                    True3False4
                }
            }
            True3False3 => {
                if bit == false {
                    True3False4
                } else {
                    True4False3
                }
            }
            True4False2 => {
                if bit == true {
                    True5False2
                } else {
                    True4False3
                }
            }
            True5False1 => {
                if bit == true {
                    True6False1
                } else {
                    True5False2
                }
            }
            True6False0 => {
                if bit == true {
                    True7False0
                } else {
                    True6False1
                }
            }
            True0False7 => {
                if bit == false {
                    True0False8
                } else {
                    True1False7
                }
            }
            True1False6 => {
                if bit == false {
                    True1False7
                } else {
                    True2False6
                }
            }
            True2False5 => {
                if bit == false {
                    True2False6
                } else {
                    True3False5
                }
            }
            True3False4 => {
                if bit == false {
                    True3False5
                } else {
                    True4False4
                }
            }
            True4False3 => {
                if bit == true {
                    True5False3
                } else {
                    True4False4
                }
            }
            True5False2 => {
                if bit == true {
                    True6False2
                } else {
                    True5False3
                }
            }
            True6False1 => {
                if bit == true {
                    True7False1
                } else {
                    True6False2
                }
            }
            True7False0 => {
                if bit == true {
                    True8False0
                } else {
                    True7False1
                }
            }
            True0False8 => {
                if bit == false {
                    True0False9
                } else {
                    True1False8
                }
            }
            True1False7 => {
                if bit == false {
                    True1False8
                } else {
                    True2False7
                }
            }
            True2False6 => {
                if bit == false {
                    True2False7
                } else {
                    True3False6
                }
            }
            True3False5 => {
                if bit == false {
                    True3False6
                } else {
                    True4False5
                }
            }
            True4False4 => {
                if bit == false {
                    True4False5
                } else {
                    True5False4
                }
            }
            True5False3 => {
                if bit == true {
                    True6False3
                } else {
                    True5False4
                }
            }
            True6False2 => {
                if bit == true {
                    True7False2
                } else {
                    True6False3
                }
            }
            True7False1 => {
                if bit == true {
                    True8False1
                } else {
                    True7False2
                }
            }
            True8False0 => {
                if bit == true {
                    True9False0
                } else {
                    True8False1
                }
            }
            True0False9 => {
                if bit == false {
                    True0False10
                } else {
                    True1False9
                }
            }
            True1False8 => {
                if bit == false {
                    True1False9
                } else {
                    True2False8
                }
            }
            True2False7 => {
                if bit == false {
                    True2False8
                } else {
                    True3False7
                }
            }
            True3False6 => {
                if bit == false {
                    True3False7
                } else {
                    True4False6
                }
            }
            True4False5 => {
                if bit == false {
                    True4False6
                } else {
                    True5False5
                }
            }
            True5False4 => {
                if bit == true {
                    True6False4
                } else {
                    True5False5
                }
            }
            True6False3 => {
                if bit == true {
                    True7False3
                } else {
                    True6False4
                }
            }
            True7False2 => {
                if bit == true {
                    True8False2
                } else {
                    True7False3
                }
            }
            True8False1 => {
                if bit == true {
                    True9False1
                } else {
                    True8False2
                }
            }
            True9False0 => {
                if bit == true {
                    True10False0
                } else {
                    True9False1
                }
            }
            True0False10 => {
                if bit == false {
                    True0False11
                } else {
                    True1False10
                }
            }
            True1False9 => {
                if bit == false {
                    True1False10
                } else {
                    True2False9
                }
            }
            True2False8 => {
                if bit == false {
                    True2False9
                } else {
                    True3False8
                }
            }
            True3False7 => {
                if bit == false {
                    True3False8
                } else {
                    True4False7
                }
            }
            True4False6 => {
                if bit == false {
                    True4False7
                } else {
                    True5False6
                }
            }
            True5False5 => {
                if bit == false {
                    True5False6
                } else {
                    True6False5
                }
            }
            True6False4 => {
                if bit == true {
                    True7False4
                } else {
                    True6False5
                }
            }
            True7False3 => {
                if bit == true {
                    True8False3
                } else {
                    True7False4
                }
            }
            True8False2 => {
                if bit == true {
                    True9False2
                } else {
                    True8False3
                }
            }
            True9False1 => {
                if bit == true {
                    True10False1
                } else {
                    True9False2
                }
            }
            True10False0 => {
                if bit == true {
                    True11False0
                } else {
                    True10False1
                }
            }
            True0False11 => {
                if bit == false {
                    True0False12
                } else {
                    True1False11
                }
            }
            True1False10 => {
                if bit == false {
                    True1False11
                } else {
                    True2False10
                }
            }
            True2False9 => {
                if bit == false {
                    True2False10
                } else {
                    True3False9
                }
            }
            True3False8 => {
                if bit == false {
                    True3False9
                } else {
                    True4False8
                }
            }
            True4False7 => {
                if bit == false {
                    True4False8
                } else {
                    True5False7
                }
            }
            True5False6 => {
                if bit == false {
                    True5False7
                } else {
                    True6False6
                }
            }
            True6False5 => {
                if bit == true {
                    True7False5
                } else {
                    True6False6
                }
            }
            True7False4 => {
                if bit == true {
                    True8False4
                } else {
                    True7False5
                }
            }
            True8False3 => {
                if bit == true {
                    True9False3
                } else {
                    True8False4
                }
            }
            True9False2 => {
                if bit == true {
                    True10False2
                } else {
                    True9False3
                }
            }
            True10False1 => {
                if bit == true {
                    True11False1
                } else {
                    True10False2
                }
            }
            True11False0 => {
                if bit == true {
                    True12False0
                } else {
                    True11False1
                }
            }
            True0False12 => {
                if bit == false {
                    True0False13
                } else {
                    True1False12
                }
            }
            True1False11 => {
                if bit == false {
                    True1False12
                } else {
                    True2False11
                }
            }
            True2False10 => {
                if bit == false {
                    True2False11
                } else {
                    True3False10
                }
            }
            True3False9 => {
                if bit == false {
                    True3False10
                } else {
                    True4False9
                }
            }
            True4False8 => {
                if bit == false {
                    True4False9
                } else {
                    True5False8
                }
            }
            True5False7 => {
                if bit == false {
                    True5False8
                } else {
                    True6False7
                }
            }
            True6False6 => {
                if bit == false {
                    True6False7
                } else {
                    True7False6
                }
            }
            True7False5 => {
                if bit == true {
                    True8False5
                } else {
                    True7False6
                }
            }
            True8False4 => {
                if bit == true {
                    True9False4
                } else {
                    True8False5
                }
            }
            True9False3 => {
                if bit == true {
                    True10False3
                } else {
                    True9False4
                }
            }
            True10False2 => {
                if bit == true {
                    True11False2
                } else {
                    True10False3
                }
            }
            True11False1 => {
                if bit == true {
                    True12False1
                } else {
                    True11False2
                }
            }
            True12False0 => {
                if bit == true {
                    True13False0
                } else {
                    True12False1
                }
            }
            True0False13 => {
                if bit == false {
                    True0False14
                } else {
                    True1False13
                }
            }
            True1False12 => {
                if bit == false {
                    True1False13
                } else {
                    True2False12
                }
            }
            True2False11 => {
                if bit == false {
                    True2False12
                } else {
                    True3False11
                }
            }
            True3False10 => {
                if bit == false {
                    True3False11
                } else {
                    True4False10
                }
            }
            True4False9 => {
                if bit == false {
                    True4False10
                } else {
                    True5False9
                }
            }
            True5False8 => {
                if bit == false {
                    True5False9
                } else {
                    True6False8
                }
            }
            True6False7 => {
                if bit == false {
                    True6False8
                } else {
                    True7False7
                }
            }
            True7False6 => {
                if bit == true {
                    True8False6
                } else {
                    True7False7
                }
            }
            True8False5 => {
                if bit == true {
                    True9False5
                } else {
                    True8False6
                }
            }
            True9False4 => {
                if bit == true {
                    True10False4
                } else {
                    True9False5
                }
            }
            True10False3 => {
                if bit == true {
                    True11False3
                } else {
                    True10False4
                }
            }
            True11False2 => {
                if bit == true {
                    True12False2
                } else {
                    True11False3
                }
            }
            True12False1 => {
                if bit == true {
                    True13False1
                } else {
                    True12False2
                }
            }
            True13False0 => {
                if bit == true {
                    True14False0
                } else {
                    True13False1
                }
            }
            True0False14 => {
                if bit == false {
                    True0False15
                } else {
                    True1False14
                }
            }
            True1False13 => {
                if bit == false {
                    True1False14
                } else {
                    True2False13
                }
            }
            True2False12 => {
                if bit == false {
                    True2False13
                } else {
                    True3False12
                }
            }
            True3False11 => {
                if bit == false {
                    True3False12
                } else {
                    True4False11
                }
            }
            True4False10 => {
                if bit == false {
                    True4False11
                } else {
                    True5False10
                }
            }
            True5False9 => {
                if bit == false {
                    True5False10
                } else {
                    True6False9
                }
            }
            True6False8 => {
                if bit == false {
                    True6False9
                } else {
                    True7False8
                }
            }
            True7False7 => {
                if bit == false {
                    True7False8
                } else {
                    True8False7
                }
            }
            True8False6 => {
                if bit == true {
                    True9False6
                } else {
                    True8False7
                }
            }
            True9False5 => {
                if bit == true {
                    True10False5
                } else {
                    True9False6
                }
            }
            True10False4 => {
                if bit == true {
                    True11False4
                } else {
                    True10False5
                }
            }
            True11False3 => {
                if bit == true {
                    True12False3
                } else {
                    True11False4
                }
            }
            True12False2 => {
                if bit == true {
                    True13False2
                } else {
                    True12False3
                }
            }
            True13False1 => {
                if bit == true {
                    True14False1
                } else {
                    True13False2
                }
            }
            True14False0 => {
                if bit == true {
                    True15False0
                } else {
                    True14False1
                }
            }
            True0False15 => {
                if bit == false {
                    True0False16
                } else {
                    True1False15
                }
            }
            True1False14 => {
                if bit == false {
                    True1False15
                } else {
                    True2False14
                }
            }
            True2False13 => {
                if bit == false {
                    True2False14
                } else {
                    True3False13
                }
            }
            True3False12 => {
                if bit == false {
                    True3False13
                } else {
                    True4False12
                }
            }
            True4False11 => {
                if bit == false {
                    True4False12
                } else {
                    True5False11
                }
            }
            True5False10 => {
                if bit == false {
                    True5False11
                } else {
                    True6False10
                }
            }
            True6False9 => {
                if bit == false {
                    True6False10
                } else {
                    True7False9
                }
            }
            True7False8 => {
                if bit == false {
                    True7False9
                } else {
                    True8False8
                }
            }
            True8False7 => {
                if bit == true {
                    True9False7
                } else {
                    True8False8
                }
            }
            True9False6 => {
                if bit == true {
                    True10False6
                } else {
                    True9False7
                }
            }
            True10False5 => {
                if bit == true {
                    True11False5
                } else {
                    True10False6
                }
            }
            True11False4 => {
                if bit == true {
                    True12False4
                } else {
                    True11False5
                }
            }
            True12False3 => {
                if bit == true {
                    True13False3
                } else {
                    True12False4
                }
            }
            True13False2 => {
                if bit == true {
                    True14False2
                } else {
                    True13False3
                }
            }
            True14False1 => {
                if bit == true {
                    True15False1
                } else {
                    True14False2
                }
            }
            True15False0 => {
                if bit == true {
                    True16False0
                } else {
                    True15False1
                }
            }
            True0False16 => {
                if bit == false {
                    True0False17
                } else {
                    True1False16
                }
            }
            True1False15 => {
                if bit == false {
                    True1False16
                } else {
                    True2False15
                }
            }
            True2False14 => {
                if bit == false {
                    True2False15
                } else {
                    True3False14
                }
            }
            True3False13 => {
                if bit == false {
                    True3False14
                } else {
                    True4False13
                }
            }
            True4False12 => {
                if bit == false {
                    True4False13
                } else {
                    True5False12
                }
            }
            True5False11 => {
                if bit == false {
                    True5False12
                } else {
                    True6False11
                }
            }
            True6False10 => {
                if bit == false {
                    True6False11
                } else {
                    True7False10
                }
            }
            True7False9 => {
                if bit == false {
                    True7False10
                } else {
                    True8False9
                }
            }
            True8False8 => {
                if bit == false {
                    True8False9
                } else {
                    True9False8
                }
            }
            True9False7 => {
                if bit == true {
                    True10False7
                } else {
                    True9False8
                }
            }
            True10False6 => {
                if bit == true {
                    True11False6
                } else {
                    True10False7
                }
            }
            True11False5 => {
                if bit == true {
                    True12False5
                } else {
                    True11False6
                }
            }
            True12False4 => {
                if bit == true {
                    True13False4
                } else {
                    True12False5
                }
            }
            True13False3 => {
                if bit == true {
                    True14False3
                } else {
                    True13False4
                }
            }
            True14False2 => {
                if bit == true {
                    True15False2
                } else {
                    True14False3
                }
            }
            True15False1 => {
                if bit == true {
                    True16False1
                } else {
                    True15False2
                }
            }
            True16False0 => {
                if bit == true {
                    True17False0
                } else {
                    True16False1
                }
            }
            True0False17 => {
                if bit == false {
                    True0False18
                } else {
                    True1False17
                }
            }
            True1False16 => {
                if bit == false {
                    True1False17
                } else {
                    True2False16
                }
            }
            True2False15 => {
                if bit == false {
                    True2False16
                } else {
                    True3False15
                }
            }
            True3False14 => {
                if bit == false {
                    True3False15
                } else {
                    True4False14
                }
            }
            True4False13 => {
                if bit == false {
                    True4False14
                } else {
                    True5False13
                }
            }
            True5False12 => {
                if bit == false {
                    True5False13
                } else {
                    True6False12
                }
            }
            True6False11 => {
                if bit == false {
                    True6False12
                } else {
                    True7False11
                }
            }
            True7False10 => {
                if bit == false {
                    True7False11
                } else {
                    True8False10
                }
            }
            True8False9 => {
                if bit == false {
                    True8False10
                } else {
                    True9False9
                }
            }
            True9False8 => {
                if bit == true {
                    True10False8
                } else {
                    True9False9
                }
            }
            True10False7 => {
                if bit == true {
                    True11False7
                } else {
                    True10False8
                }
            }
            True11False6 => {
                if bit == true {
                    True12False6
                } else {
                    True11False7
                }
            }
            True12False5 => {
                if bit == true {
                    True13False5
                } else {
                    True12False6
                }
            }
            True13False4 => {
                if bit == true {
                    True14False4
                } else {
                    True13False5
                }
            }
            True14False3 => {
                if bit == true {
                    True15False3
                } else {
                    True14False4
                }
            }
            True15False2 => {
                if bit == true {
                    True16False2
                } else {
                    True15False3
                }
            }
            True16False1 => {
                if bit == true {
                    True17False1
                } else {
                    True16False2
                }
            }
            True17False0 => {
                if bit == true {
                    True18False0
                } else {
                    True17False1
                }
            }
            True0False18 => {
                if bit == false {
                    True0False19
                } else {
                    True1False18
                }
            }
            True1False17 => {
                if bit == false {
                    True1False18
                } else {
                    True2False17
                }
            }
            True2False16 => {
                if bit == false {
                    True2False17
                } else {
                    True3False16
                }
            }
            True3False15 => {
                if bit == false {
                    True3False16
                } else {
                    True4False15
                }
            }
            True4False14 => {
                if bit == false {
                    True4False15
                } else {
                    True5False14
                }
            }
            True5False13 => {
                if bit == false {
                    True5False14
                } else {
                    True6False13
                }
            }
            True6False12 => {
                if bit == false {
                    True6False13
                } else {
                    True7False12
                }
            }
            True7False11 => {
                if bit == false {
                    True7False12
                } else {
                    True8False11
                }
            }
            True8False10 => {
                if bit == false {
                    True8False11
                } else {
                    True9False10
                }
            }
            True9False9 => {
                if bit == false {
                    True9False10
                } else {
                    True10False9
                }
            }
            True10False8 => {
                if bit == true {
                    True11False8
                } else {
                    True10False9
                }
            }
            True11False7 => {
                if bit == true {
                    True12False7
                } else {
                    True11False8
                }
            }
            True12False6 => {
                if bit == true {
                    True13False6
                } else {
                    True12False7
                }
            }
            True13False5 => {
                if bit == true {
                    True14False5
                } else {
                    True13False6
                }
            }
            True14False4 => {
                if bit == true {
                    True15False4
                } else {
                    True14False5
                }
            }
            True15False3 => {
                if bit == true {
                    True16False3
                } else {
                    True15False4
                }
            }
            True16False2 => {
                if bit == true {
                    True17False2
                } else {
                    True16False3
                }
            }
            True17False1 => {
                if bit == true {
                    True18False1
                } else {
                    True17False2
                }
            }
            True18False0 => {
                if bit == true {
                    True19False0
                } else {
                    True18False1
                }
            }
            True0False19 => {
                if bit == false {
                    True0False20
                } else {
                    True1False19
                }
            }
            True1False18 => {
                if bit == false {
                    True1False19
                } else {
                    True2False18
                }
            }
            True2False17 => {
                if bit == false {
                    True2False18
                } else {
                    True3False17
                }
            }
            True3False16 => {
                if bit == false {
                    True3False17
                } else {
                    True4False16
                }
            }
            True4False15 => {
                if bit == false {
                    True4False16
                } else {
                    True5False15
                }
            }
            True5False14 => {
                if bit == false {
                    True5False15
                } else {
                    True6False14
                }
            }
            True6False13 => {
                if bit == false {
                    True6False14
                } else {
                    True7False13
                }
            }
            True7False12 => {
                if bit == false {
                    True7False13
                } else {
                    True8False12
                }
            }
            True8False11 => {
                if bit == false {
                    True8False12
                } else {
                    True9False11
                }
            }
            True9False10 => {
                if bit == false {
                    True9False11
                } else {
                    True10False10
                }
            }
            True10False9 => {
                if bit == true {
                    True11False9
                } else {
                    True10False10
                }
            }
            True11False8 => {
                if bit == true {
                    True12False8
                } else {
                    True11False9
                }
            }
            True12False7 => {
                if bit == true {
                    True13False7
                } else {
                    True12False8
                }
            }
            True13False6 => {
                if bit == true {
                    True14False6
                } else {
                    True13False7
                }
            }
            True14False5 => {
                if bit == true {
                    True15False5
                } else {
                    True14False6
                }
            }
            True15False4 => {
                if bit == true {
                    True16False4
                } else {
                    True15False5
                }
            }
            True16False3 => {
                if bit == true {
                    True17False3
                } else {
                    True16False4
                }
            }
            True17False2 => {
                if bit == true {
                    True18False2
                } else {
                    True17False3
                }
            }
            True18False1 => {
                if bit == true {
                    True19False1
                } else {
                    True18False2
                }
            }
            True19False0 => {
                if bit == true {
                    True20False0
                } else {
                    True19False1
                }
            }
            True0False20 => {
                if bit == false {
                    AllFalse4
                } else {
                    True1False10
                }
            }
            True1False19 => {
                if bit == false {
                    True1False10
                } else {
                    True1False10
                }
            }
            True2False18 => {
                if bit == false {
                    True1False10
                } else {
                    True2False9
                }
            }
            True3False17 => {
                if bit == false {
                    True2False9
                } else {
                    True2False9
                }
            }
            True4False16 => {
                if bit == false {
                    True2False9
                } else {
                    True3False8
                }
            }
            True5False15 => {
                if bit == false {
                    True3False8
                } else {
                    True3False8
                }
            }
            True6False14 => {
                if bit == false {
                    True3False8
                } else {
                    True4False7
                }
            }
            True7False13 => {
                if bit == false {
                    True4False7
                } else {
                    True4False7
                }
            }
            True8False12 => {
                if bit == false {
                    True4False7
                } else {
                    True5False6
                }
            }
            True9False11 => {
                if bit == false {
                    True5False6
                } else {
                    True5False6
                }
            }
            True10False10 => {
                if bit == false {
                    True5False6
                } else {
                    True6False5
                }
            }
            True11False9 => {
                if bit == true {
                    True6False5
                } else {
                    True6False5
                }
            }
            True12False8 => {
                if bit == true {
                    True7False4
                } else {
                    True6False5
                }
            }
            True13False7 => {
                if bit == true {
                    True7False4
                } else {
                    True7False4
                }
            }
            True14False6 => {
                if bit == true {
                    True8False3
                } else {
                    True7False4
                }
            }
            True15False5 => {
                if bit == true {
                    True8False3
                } else {
                    True8False3
                }
            }
            True16False4 => {
                if bit == true {
                    True9False2
                } else {
                    True8False3
                }
            }
            True17False3 => {
                if bit == true {
                    True9False2
                } else {
                    True9False2
                }
            }
            True18False2 => {
                if bit == true {
                    True10False1
                } else {
                    True9False2
                }
            }
            True19False1 => {
                if bit == true {
                    True10False1
                } else {
                    True10False1
                }
            }
            True20False0 => {
                if bit == true {
                    AllTrue4
                } else {
                    True10False1
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
                    True1False16
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
                    True16False1
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
                    True1False16
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
                    True16False1
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
                    True1False16
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
                    True16False1
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
                    True1False16
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
                    True16False1
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
                    True1False16
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
                    True16False1
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
                    True1False16
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
                    True16False1
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
                    True1False16
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
                    True16False1
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
                    True1False16
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
                    True16False1
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
                    True1False16
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
                    True16False1
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
                    True1False16
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
                    True16False1
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
                    True1False16
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
                    True16False1
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
