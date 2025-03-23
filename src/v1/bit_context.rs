//! Generated with `src/v1/bit-context.sh`
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
        const LOOKUP: [Probability; 255] = [
            Probability { prob: 2, shift: 2 },
            Probability { prob: 85, shift: 7 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 3, shift: 2 },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 51, shift: 6 },
            Probability { prob: 85, shift: 7 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 51, shift: 8 },
            Probability {
                prob: 213,
                shift: 8,
            },
            Probability { prob: 3, shift: 2 },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 21, shift: 7 },
            Probability {
                prob: 219,
                shift: 8,
            },
            Probability { prob: 51, shift: 6 },
            Probability {
                prob: 153,
                shift: 8,
            },
            Probability { prob: 51, shift: 7 },
            Probability { prob: 51, shift: 8 },
            Probability { prob: 9, shift: 6 },
            Probability { prob: 7, shift: 3 },
            Probability {
                prob: 213,
                shift: 8,
            },
            Probability { prob: 85, shift: 7 },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 21, shift: 7 },
            Probability { prob: 1, shift: 3 },
            Probability {
                prob: 227,
                shift: 8,
            },
            Probability {
                prob: 219,
                shift: 8,
            },
            Probability { prob: 91, shift: 7 },
            Probability { prob: 73, shift: 7 },
            Probability {
                prob: 109,
                shift: 8,
            },
            Probability { prob: 73, shift: 8 },
            Probability { prob: 9, shift: 6 },
            Probability { prob: 7, shift: 6 },
            Probability {
                prob: 115,
                shift: 7,
            },
            Probability { prob: 7, shift: 3 },
            Probability { prob: 3, shift: 2 },
            Probability { prob: 5, shift: 3 },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 3, shift: 3 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 1, shift: 3 },
            Probability { prob: 25, shift: 8 },
            Probability { prob: 29, shift: 5 },
            Probability {
                prob: 227,
                shift: 8,
            },
            Probability {
                prob: 199,
                shift: 8,
            },
            Probability { prob: 85, shift: 7 },
            Probability { prob: 71, shift: 7 },
            Probability {
                prob: 113,
                shift: 8,
            },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 7, shift: 5 },
            Probability { prob: 7, shift: 6 },
            Probability { prob: 23, shift: 8 },
            Probability {
                prob: 117,
                shift: 7,
            },
            Probability {
                prob: 115,
                shift: 7,
            },
            Probability { prob: 51, shift: 6 },
            Probability {
                prob: 179,
                shift: 8,
            },
            Probability {
                prob: 153,
                shift: 8,
            },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 51, shift: 7 },
            Probability { prob: 19, shift: 6 },
            Probability { prob: 51, shift: 8 },
            Probability { prob: 25, shift: 8 },
            Probability { prob: 21, shift: 8 },
            Probability { prob: 59, shift: 6 },
            Probability { prob: 29, shift: 5 },
            Probability {
                prob: 209,
                shift: 8,
            },
            Probability { prob: 93, shift: 7 },
            Probability { prob: 81, shift: 7 },
            Probability {
                prob: 139,
                shift: 8,
            },
            Probability { prob: 29, shift: 6 },
            Probability { prob: 93, shift: 8 },
            Probability { prob: 69, shift: 8 },
            Probability { prob: 23, shift: 7 },
            Probability { prob: 23, shift: 8 },
            Probability { prob: 19, shift: 8 },
            Probability {
                prob: 237,
                shift: 8,
            },
            Probability {
                prob: 117,
                shift: 7,
            },
            Probability {
                prob: 213,
                shift: 8,
            },
            Probability { prob: 3, shift: 2 },
            Probability { prob: 85, shift: 7 },
            Probability {
                prob: 149,
                shift: 8,
            },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 53, shift: 7 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 21, shift: 7 },
            Probability { prob: 21, shift: 8 },
            Probability { prob: 9, shift: 7 },
            Probability {
                prob: 119,
                shift: 7,
            },
            Probability { prob: 59, shift: 6 },
            Probability { prob: 27, shift: 5 },
            Probability { prob: 49, shift: 6 },
            Probability {
                prob: 177,
                shift: 8,
            },
            Probability {
                prob: 157,
                shift: 8,
            },
            Probability {
                prob: 137,
                shift: 8,
            },
            Probability { prob: 59, shift: 7 },
            Probability { prob: 49, shift: 7 },
            Probability { prob: 39, shift: 7 },
            Probability { prob: 59, shift: 8 },
            Probability { prob: 39, shift: 8 },
            Probability { prob: 19, shift: 8 },
            Probability { prob: 17, shift: 8 },
            Probability { prob: 15, shift: 4 },
            Probability {
                prob: 237,
                shift: 8,
            },
            Probability {
                prob: 219,
                shift: 8,
            },
            Probability {
                prob: 201,
                shift: 8,
            },
            Probability { prob: 91, shift: 7 },
            Probability { prob: 41, shift: 6 },
            Probability { prob: 73, shift: 7 },
            Probability { prob: 2, shift: 2 },
            Probability {
                prob: 109,
                shift: 8,
            },
            Probability { prob: 91, shift: 8 },
            Probability { prob: 73, shift: 8 },
            Probability { prob: 27, shift: 7 },
            Probability { prob: 9, shift: 6 },
            Probability { prob: 9, shift: 7 },
            Probability { prob: 1, shift: 4 },
            Probability { prob: 15, shift: 4 },
            Probability {
                prob: 119,
                shift: 7,
            },
            Probability {
                prob: 221,
                shift: 8,
            },
            Probability { prob: 51, shift: 6 },
            Probability {
                prob: 187,
                shift: 8,
            },
            Probability { prob: 85, shift: 7 },
            Probability {
                prob: 153,
                shift: 8,
            },
            Probability { prob: 17, shift: 5 },
            Probability {
                prob: 119,
                shift: 8,
            },
            Probability { prob: 51, shift: 7 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 17, shift: 6 },
            Probability { prob: 51, shift: 8 },
            Probability { prob: 17, shift: 7 },
            Probability { prob: 17, shift: 8 },
            Probability { prob: 15, shift: 8 },
            Probability {
                prob: 241,
                shift: 8,
            },
            Probability { prob: 15, shift: 4 },
            Probability { prob: 7, shift: 3 },
            Probability { prob: 13, shift: 4 },
            Probability { prob: 3, shift: 2 },
            Probability { prob: 11, shift: 4 },
            Probability { prob: 5, shift: 3 },
            Probability { prob: 9, shift: 4 },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 7, shift: 4 },
            Probability { prob: 3, shift: 3 },
            Probability { prob: 5, shift: 4 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 3, shift: 4 },
            Probability { prob: 1, shift: 3 },
            Probability { prob: 1, shift: 4 },
            Probability { prob: 7, shift: 7 },
            Probability {
                prob: 121,
                shift: 7,
            },
            Probability { prob: 15, shift: 4 },
            Probability {
                prob: 225,
                shift: 8,
            },
            Probability {
                prob: 105,
                shift: 7,
            },
            Probability {
                prob: 195,
                shift: 8,
            },
            Probability { prob: 45, shift: 6 },
            Probability {
                prob: 165,
                shift: 8,
            },
            Probability { prob: 75, shift: 7 },
            Probability {
                prob: 135,
                shift: 8,
            },
            Probability { prob: 15, shift: 5 },
            Probability {
                prob: 105,
                shift: 8,
            },
            Probability { prob: 45, shift: 7 },
            Probability { prob: 75, shift: 8 },
            Probability { prob: 15, shift: 6 },
            Probability { prob: 45, shift: 8 },
            Probability { prob: 15, shift: 7 },
            Probability { prob: 15, shift: 8 },
            Probability { prob: 13, shift: 8 },
            Probability {
                prob: 243,
                shift: 8,
            },
            Probability {
                prob: 241,
                shift: 8,
            },
            Probability {
                prob: 227,
                shift: 8,
            },
            Probability {
                prob: 213,
                shift: 8,
            },
            Probability {
                prob: 199,
                shift: 8,
            },
            Probability { prob: 23, shift: 5 },
            Probability { prob: 85, shift: 7 },
            Probability { prob: 39, shift: 6 },
            Probability { prob: 71, shift: 7 },
            Probability { prob: 2, shift: 2 },
            Probability {
                prob: 113,
                shift: 8,
            },
            Probability { prob: 99, shift: 8 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 71, shift: 8 },
            Probability { prob: 7, shift: 5 },
            Probability { prob: 21, shift: 7 },
            Probability { prob: 7, shift: 6 },
            Probability { prob: 7, shift: 7 },
            Probability { prob: 3, shift: 6 },
            Probability {
                prob: 243,
                shift: 8,
            },
            Probability {
                prob: 121,
                shift: 7,
            },
            Probability {
                prob: 229,
                shift: 8,
            },
            Probability {
                prob: 215,
                shift: 8,
            },
            Probability {
                prob: 101,
                shift: 7,
            },
            Probability { prob: 47, shift: 6 },
            Probability {
                prob: 175,
                shift: 8,
            },
            Probability {
                prob: 161,
                shift: 8,
            },
            Probability { prob: 37, shift: 6 },
            Probability { prob: 67, shift: 7 },
            Probability {
                prob: 121,
                shift: 8,
            },
            Probability {
                prob: 107,
                shift: 8,
            },
            Probability { prob: 47, shift: 7 },
            Probability { prob: 5, shift: 4 },
            Probability { prob: 67, shift: 8 },
            Probability { prob: 53, shift: 8 },
            Probability { prob: 5, shift: 5 },
            Probability { prob: 13, shift: 7 },
            Probability { prob: 13, shift: 8 },
            Probability { prob: 3, shift: 6 },
            Probability { prob: 61, shift: 6 },
            Probability {
                prob: 243,
                shift: 8,
            },
            Probability {
                prob: 115,
                shift: 7,
            },
            Probability {
                prob: 217,
                shift: 8,
            },
            Probability { prob: 51, shift: 6 },
            Probability { prob: 3, shift: 2 },
            Probability {
                prob: 179,
                shift: 8,
            },
            Probability { prob: 83, shift: 7 },
            Probability {
                prob: 153,
                shift: 8,
            },
            Probability { prob: 35, shift: 6 },
            Probability { prob: 2, shift: 2 },
            Probability {
                prob: 115,
                shift: 8,
            },
            Probability { prob: 51, shift: 7 },
            Probability { prob: 89, shift: 8 },
            Probability { prob: 19, shift: 6 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 51, shift: 8 },
            Probability { prob: 19, shift: 7 },
            Probability { prob: 25, shift: 8 },
            Probability { prob: 3, shift: 6 },
            Probability { prob: 11, shift: 8 },
            Probability { prob: 31, shift: 5 },
            Probability { prob: 1, shift: 5 },
            Probability { prob: 63, shift: 6 },
            Probability { prob: 1, shift: 6 },
            Probability {
                prob: 127,
                shift: 7,
            },
            Probability { prob: 1, shift: 7 },
            Probability {
                prob: 255,
                shift: 8,
            },
            Probability { prob: 1, shift: 8 },
            Probability {
                prob: 511,
                shift: 9,
            },
            Probability { prob: 1, shift: 9 },
            Probability {
                prob: 1023,
                shift: 10,
            },
            Probability { prob: 1, shift: 10 },
            Probability {
                prob: 2047,
                shift: 11,
            },
            Probability { prob: 1, shift: 11 },
            Probability {
                prob: 4095,
                shift: 12,
            },
            Probability { prob: 1, shift: 12 },
            Probability {
                prob: 8191,
                shift: 13,
            },
            Probability { prob: 1, shift: 13 },
            Probability {
                prob: 16383,
                shift: 14,
            },
            Probability { prob: 1, shift: 14 },
            Probability {
                prob: 32767,
                shift: 15,
            },
            Probability { prob: 1, shift: 15 },
            Probability {
                prob: 65535,
                shift: 16,
            },
            Probability { prob: 1, shift: 16 },
        ];
        LOOKUP[self as usize]
    }

    pub fn adapt(self, bit: bool, rng: &mut SplitMix64) -> Self {
        struct Outcome {
            a: BitContext,
            b: BitContext,
            prob_a: u64,
        }
        const OUTCOMES: [Outcome; 2 * 255] = [
            Outcome {
                a: True0False1,
                b: True0False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False2,
                b: True0False2,
                prob_a: 0,
            },
            Outcome {
                a: True1False1,
                b: True1False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False3,
                b: True0False3,
                prob_a: 0,
            },
            Outcome {
                a: True1False2,
                b: True1False2,
                prob_a: 0,
            },
            Outcome {
                a: True2False1,
                b: True2False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False4,
                b: True0False4,
                prob_a: 0,
            },
            Outcome {
                a: True1False3,
                b: True1False3,
                prob_a: 0,
            },
            Outcome {
                a: True2False2,
                b: True2False2,
                prob_a: 0,
            },
            Outcome {
                a: True3False1,
                b: True3False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False5,
                b: True0False5,
                prob_a: 0,
            },
            Outcome {
                a: True1False4,
                b: True1False4,
                prob_a: 0,
            },
            Outcome {
                a: True2False3,
                b: True2False3,
                prob_a: 0,
            },
            Outcome {
                a: True3False2,
                b: True3False2,
                prob_a: 0,
            },
            Outcome {
                a: True4False1,
                b: True4False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False6,
                b: True0False6,
                prob_a: 0,
            },
            Outcome {
                a: True1False5,
                b: True1False5,
                prob_a: 0,
            },
            Outcome {
                a: True2False4,
                b: True2False4,
                prob_a: 0,
            },
            Outcome {
                a: True3False3,
                b: True3False3,
                prob_a: 0,
            },
            Outcome {
                a: True4False2,
                b: True4False2,
                prob_a: 0,
            },
            Outcome {
                a: True5False1,
                b: True5False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False7,
                b: True0False7,
                prob_a: 0,
            },
            Outcome {
                a: True1False6,
                b: True1False6,
                prob_a: 0,
            },
            Outcome {
                a: True2False5,
                b: True2False5,
                prob_a: 0,
            },
            Outcome {
                a: True3False4,
                b: True3False4,
                prob_a: 0,
            },
            Outcome {
                a: True4False3,
                b: True4False3,
                prob_a: 0,
            },
            Outcome {
                a: True5False2,
                b: True5False2,
                prob_a: 0,
            },
            Outcome {
                a: True6False1,
                b: True6False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False8,
                b: True0False8,
                prob_a: 0,
            },
            Outcome {
                a: True1False7,
                b: True1False7,
                prob_a: 0,
            },
            Outcome {
                a: True2False6,
                b: True2False6,
                prob_a: 0,
            },
            Outcome {
                a: True3False5,
                b: True3False5,
                prob_a: 0,
            },
            Outcome {
                a: True4False4,
                b: True4False4,
                prob_a: 0,
            },
            Outcome {
                a: True5False3,
                b: True5False3,
                prob_a: 0,
            },
            Outcome {
                a: True6False2,
                b: True6False2,
                prob_a: 0,
            },
            Outcome {
                a: True7False1,
                b: True7False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False9,
                b: True0False9,
                prob_a: 0,
            },
            Outcome {
                a: True1False8,
                b: True1False8,
                prob_a: 0,
            },
            Outcome {
                a: True2False7,
                b: True2False7,
                prob_a: 0,
            },
            Outcome {
                a: True3False6,
                b: True3False6,
                prob_a: 0,
            },
            Outcome {
                a: True4False5,
                b: True4False5,
                prob_a: 0,
            },
            Outcome {
                a: True5False4,
                b: True5False4,
                prob_a: 0,
            },
            Outcome {
                a: True6False3,
                b: True6False3,
                prob_a: 0,
            },
            Outcome {
                a: True7False2,
                b: True7False2,
                prob_a: 0,
            },
            Outcome {
                a: True8False1,
                b: True8False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False10,
                b: True0False10,
                prob_a: 0,
            },
            Outcome {
                a: True1False9,
                b: True1False9,
                prob_a: 0,
            },
            Outcome {
                a: True2False8,
                b: True2False8,
                prob_a: 0,
            },
            Outcome {
                a: True3False7,
                b: True3False7,
                prob_a: 0,
            },
            Outcome {
                a: True4False6,
                b: True4False6,
                prob_a: 0,
            },
            Outcome {
                a: True5False5,
                b: True5False5,
                prob_a: 0,
            },
            Outcome {
                a: True6False4,
                b: True6False4,
                prob_a: 0,
            },
            Outcome {
                a: True7False3,
                b: True7False3,
                prob_a: 0,
            },
            Outcome {
                a: True8False2,
                b: True8False2,
                prob_a: 0,
            },
            Outcome {
                a: True9False1,
                b: True9False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False11,
                b: True0False11,
                prob_a: 0,
            },
            Outcome {
                a: True1False10,
                b: True1False10,
                prob_a: 0,
            },
            Outcome {
                a: True2False9,
                b: True2False9,
                prob_a: 0,
            },
            Outcome {
                a: True3False8,
                b: True3False8,
                prob_a: 0,
            },
            Outcome {
                a: True4False7,
                b: True4False7,
                prob_a: 0,
            },
            Outcome {
                a: True5False6,
                b: True5False6,
                prob_a: 0,
            },
            Outcome {
                a: True6False5,
                b: True6False5,
                prob_a: 0,
            },
            Outcome {
                a: True7False4,
                b: True7False4,
                prob_a: 0,
            },
            Outcome {
                a: True8False3,
                b: True8False3,
                prob_a: 0,
            },
            Outcome {
                a: True9False2,
                b: True9False2,
                prob_a: 0,
            },
            Outcome {
                a: True10False1,
                b: True10False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False12,
                b: True0False12,
                prob_a: 0,
            },
            Outcome {
                a: True1False11,
                b: True1False11,
                prob_a: 0,
            },
            Outcome {
                a: True2False10,
                b: True2False10,
                prob_a: 0,
            },
            Outcome {
                a: True3False9,
                b: True3False9,
                prob_a: 0,
            },
            Outcome {
                a: True4False8,
                b: True4False8,
                prob_a: 0,
            },
            Outcome {
                a: True5False7,
                b: True5False7,
                prob_a: 0,
            },
            Outcome {
                a: True6False6,
                b: True6False6,
                prob_a: 0,
            },
            Outcome {
                a: True7False5,
                b: True7False5,
                prob_a: 0,
            },
            Outcome {
                a: True8False4,
                b: True8False4,
                prob_a: 0,
            },
            Outcome {
                a: True9False3,
                b: True9False3,
                prob_a: 0,
            },
            Outcome {
                a: True10False2,
                b: True10False2,
                prob_a: 0,
            },
            Outcome {
                a: True11False1,
                b: True11False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False13,
                b: True0False13,
                prob_a: 0,
            },
            Outcome {
                a: True1False12,
                b: True1False12,
                prob_a: 0,
            },
            Outcome {
                a: True2False11,
                b: True2False11,
                prob_a: 0,
            },
            Outcome {
                a: True3False10,
                b: True3False10,
                prob_a: 0,
            },
            Outcome {
                a: True4False9,
                b: True4False9,
                prob_a: 0,
            },
            Outcome {
                a: True5False8,
                b: True5False8,
                prob_a: 0,
            },
            Outcome {
                a: True6False7,
                b: True6False7,
                prob_a: 0,
            },
            Outcome {
                a: True7False6,
                b: True7False6,
                prob_a: 0,
            },
            Outcome {
                a: True8False5,
                b: True8False5,
                prob_a: 0,
            },
            Outcome {
                a: True9False4,
                b: True9False4,
                prob_a: 0,
            },
            Outcome {
                a: True10False3,
                b: True10False3,
                prob_a: 0,
            },
            Outcome {
                a: True11False2,
                b: True11False2,
                prob_a: 0,
            },
            Outcome {
                a: True12False1,
                b: True12False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False14,
                b: True0False14,
                prob_a: 0,
            },
            Outcome {
                a: True1False13,
                b: True1False13,
                prob_a: 0,
            },
            Outcome {
                a: True2False12,
                b: True2False12,
                prob_a: 0,
            },
            Outcome {
                a: True3False11,
                b: True3False11,
                prob_a: 0,
            },
            Outcome {
                a: True4False10,
                b: True4False10,
                prob_a: 0,
            },
            Outcome {
                a: True5False9,
                b: True5False9,
                prob_a: 0,
            },
            Outcome {
                a: True6False8,
                b: True6False8,
                prob_a: 0,
            },
            Outcome {
                a: True7False7,
                b: True7False7,
                prob_a: 0,
            },
            Outcome {
                a: True8False6,
                b: True8False6,
                prob_a: 0,
            },
            Outcome {
                a: True9False5,
                b: True9False5,
                prob_a: 0,
            },
            Outcome {
                a: True10False4,
                b: True10False4,
                prob_a: 0,
            },
            Outcome {
                a: True11False3,
                b: True11False3,
                prob_a: 0,
            },
            Outcome {
                a: True12False2,
                b: True12False2,
                prob_a: 0,
            },
            Outcome {
                a: True13False1,
                b: True13False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False15,
                b: True0False15,
                prob_a: 0,
            },
            Outcome {
                a: True1False14,
                b: True1False14,
                prob_a: 0,
            },
            Outcome {
                a: True2False13,
                b: True2False13,
                prob_a: 0,
            },
            Outcome {
                a: True3False12,
                b: True3False12,
                prob_a: 0,
            },
            Outcome {
                a: True4False11,
                b: True4False11,
                prob_a: 0,
            },
            Outcome {
                a: True5False10,
                b: True5False10,
                prob_a: 0,
            },
            Outcome {
                a: True6False9,
                b: True6False9,
                prob_a: 0,
            },
            Outcome {
                a: True7False8,
                b: True7False8,
                prob_a: 0,
            },
            Outcome {
                a: True8False7,
                b: True8False7,
                prob_a: 0,
            },
            Outcome {
                a: True9False6,
                b: True9False6,
                prob_a: 0,
            },
            Outcome {
                a: True10False5,
                b: True10False5,
                prob_a: 0,
            },
            Outcome {
                a: True11False4,
                b: True11False4,
                prob_a: 0,
            },
            Outcome {
                a: True12False3,
                b: True12False3,
                prob_a: 0,
            },
            Outcome {
                a: True13False2,
                b: True13False2,
                prob_a: 0,
            },
            Outcome {
                a: True14False1,
                b: True14False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False16,
                b: True0False16,
                prob_a: 0,
            },
            Outcome {
                a: True1False15,
                b: True1False15,
                prob_a: 0,
            },
            Outcome {
                a: True2False14,
                b: True2False14,
                prob_a: 0,
            },
            Outcome {
                a: True3False13,
                b: True3False13,
                prob_a: 0,
            },
            Outcome {
                a: True4False12,
                b: True4False12,
                prob_a: 0,
            },
            Outcome {
                a: True5False11,
                b: True5False11,
                prob_a: 0,
            },
            Outcome {
                a: True6False10,
                b: True6False10,
                prob_a: 0,
            },
            Outcome {
                a: True7False9,
                b: True7False9,
                prob_a: 0,
            },
            Outcome {
                a: True8False8,
                b: True8False8,
                prob_a: 0,
            },
            Outcome {
                a: True9False7,
                b: True9False7,
                prob_a: 0,
            },
            Outcome {
                a: True10False6,
                b: True10False6,
                prob_a: 0,
            },
            Outcome {
                a: True11False5,
                b: True11False5,
                prob_a: 0,
            },
            Outcome {
                a: True12False4,
                b: True12False4,
                prob_a: 0,
            },
            Outcome {
                a: True13False3,
                b: True13False3,
                prob_a: 0,
            },
            Outcome {
                a: True14False2,
                b: True14False2,
                prob_a: 0,
            },
            Outcome {
                a: True15False1,
                b: True15False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False17,
                b: True0False17,
                prob_a: 0,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: True2False15,
                b: True2False15,
                prob_a: 0,
            },
            Outcome {
                a: True3False14,
                b: True3False14,
                prob_a: 0,
            },
            Outcome {
                a: True4False13,
                b: True4False13,
                prob_a: 0,
            },
            Outcome {
                a: True5False12,
                b: True5False12,
                prob_a: 0,
            },
            Outcome {
                a: True6False11,
                b: True6False11,
                prob_a: 0,
            },
            Outcome {
                a: True7False10,
                b: True7False10,
                prob_a: 0,
            },
            Outcome {
                a: True8False9,
                b: True8False9,
                prob_a: 0,
            },
            Outcome {
                a: True9False8,
                b: True9False8,
                prob_a: 0,
            },
            Outcome {
                a: True10False7,
                b: True10False7,
                prob_a: 0,
            },
            Outcome {
                a: True11False6,
                b: True11False6,
                prob_a: 0,
            },
            Outcome {
                a: True12False5,
                b: True12False5,
                prob_a: 0,
            },
            Outcome {
                a: True13False4,
                b: True13False4,
                prob_a: 0,
            },
            Outcome {
                a: True14False3,
                b: True14False3,
                prob_a: 0,
            },
            Outcome {
                a: True15False2,
                b: True15False2,
                prob_a: 0,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False18,
                b: True0False18,
                prob_a: 0,
            },
            Outcome {
                a: True1False17,
                b: True1False17,
                prob_a: 0,
            },
            Outcome {
                a: True2False16,
                b: True2False16,
                prob_a: 0,
            },
            Outcome {
                a: True3False15,
                b: True3False15,
                prob_a: 0,
            },
            Outcome {
                a: True4False14,
                b: True4False14,
                prob_a: 0,
            },
            Outcome {
                a: True5False13,
                b: True5False13,
                prob_a: 0,
            },
            Outcome {
                a: True6False12,
                b: True6False12,
                prob_a: 0,
            },
            Outcome {
                a: True7False11,
                b: True7False11,
                prob_a: 0,
            },
            Outcome {
                a: True8False10,
                b: True8False10,
                prob_a: 0,
            },
            Outcome {
                a: True9False9,
                b: True9False9,
                prob_a: 0,
            },
            Outcome {
                a: True10False8,
                b: True10False8,
                prob_a: 0,
            },
            Outcome {
                a: True11False7,
                b: True11False7,
                prob_a: 0,
            },
            Outcome {
                a: True12False6,
                b: True12False6,
                prob_a: 0,
            },
            Outcome {
                a: True13False5,
                b: True13False5,
                prob_a: 0,
            },
            Outcome {
                a: True14False4,
                b: True14False4,
                prob_a: 0,
            },
            Outcome {
                a: True15False3,
                b: True15False3,
                prob_a: 0,
            },
            Outcome {
                a: True16False2,
                b: True16False2,
                prob_a: 0,
            },
            Outcome {
                a: True17False1,
                b: True17False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False19,
                b: True0False19,
                prob_a: 0,
            },
            Outcome {
                a: True1False18,
                b: True1False18,
                prob_a: 0,
            },
            Outcome {
                a: True2False17,
                b: True2False17,
                prob_a: 0,
            },
            Outcome {
                a: True3False16,
                b: True3False16,
                prob_a: 0,
            },
            Outcome {
                a: True4False15,
                b: True4False15,
                prob_a: 0,
            },
            Outcome {
                a: True5False14,
                b: True5False14,
                prob_a: 0,
            },
            Outcome {
                a: True6False13,
                b: True6False13,
                prob_a: 0,
            },
            Outcome {
                a: True7False12,
                b: True7False12,
                prob_a: 0,
            },
            Outcome {
                a: True8False11,
                b: True8False11,
                prob_a: 0,
            },
            Outcome {
                a: True9False10,
                b: True9False10,
                prob_a: 0,
            },
            Outcome {
                a: True10False9,
                b: True10False9,
                prob_a: 0,
            },
            Outcome {
                a: True11False8,
                b: True11False8,
                prob_a: 0,
            },
            Outcome {
                a: True12False7,
                b: True12False7,
                prob_a: 0,
            },
            Outcome {
                a: True13False6,
                b: True13False6,
                prob_a: 0,
            },
            Outcome {
                a: True14False5,
                b: True14False5,
                prob_a: 0,
            },
            Outcome {
                a: True15False4,
                b: True15False4,
                prob_a: 0,
            },
            Outcome {
                a: True16False3,
                b: True16False3,
                prob_a: 0,
            },
            Outcome {
                a: True17False2,
                b: True17False2,
                prob_a: 0,
            },
            Outcome {
                a: True18False1,
                b: True18False1,
                prob_a: 0,
            },
            Outcome {
                a: True0False20,
                b: True0False20,
                prob_a: 0,
            },
            Outcome {
                a: True1False19,
                b: True1False19,
                prob_a: 0,
            },
            Outcome {
                a: True2False18,
                b: True2False18,
                prob_a: 0,
            },
            Outcome {
                a: True3False17,
                b: True3False17,
                prob_a: 0,
            },
            Outcome {
                a: True4False16,
                b: True4False16,
                prob_a: 0,
            },
            Outcome {
                a: True5False15,
                b: True5False15,
                prob_a: 0,
            },
            Outcome {
                a: True6False14,
                b: True6False14,
                prob_a: 0,
            },
            Outcome {
                a: True7False13,
                b: True7False13,
                prob_a: 0,
            },
            Outcome {
                a: True8False12,
                b: True8False12,
                prob_a: 0,
            },
            Outcome {
                a: True9False11,
                b: True9False11,
                prob_a: 0,
            },
            Outcome {
                a: True10False10,
                b: True10False10,
                prob_a: 0,
            },
            Outcome {
                a: True11False9,
                b: True11False9,
                prob_a: 0,
            },
            Outcome {
                a: True12False8,
                b: True12False8,
                prob_a: 0,
            },
            Outcome {
                a: True13False7,
                b: True13False7,
                prob_a: 0,
            },
            Outcome {
                a: True14False6,
                b: True14False6,
                prob_a: 0,
            },
            Outcome {
                a: True15False5,
                b: True15False5,
                prob_a: 0,
            },
            Outcome {
                a: True16False4,
                b: True16False4,
                prob_a: 0,
            },
            Outcome {
                a: True17False3,
                b: True17False3,
                prob_a: 0,
            },
            Outcome {
                a: True18False2,
                b: True18False2,
                prob_a: 0,
            },
            Outcome {
                a: True19False1,
                b: True19False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse4,
                b: AllFalse4,
                prob_a: 0,
            },
            Outcome {
                a: True1False10,
                b: True1False10,
                prob_a: 0,
            },
            Outcome {
                a: True1False10,
                b: True1False10,
                prob_a: 0,
            },
            Outcome {
                a: True2False9,
                b: True2False9,
                prob_a: 0,
            },
            Outcome {
                a: True2False9,
                b: True2False9,
                prob_a: 0,
            },
            Outcome {
                a: True3False8,
                b: True3False8,
                prob_a: 0,
            },
            Outcome {
                a: True3False8,
                b: True3False8,
                prob_a: 0,
            },
            Outcome {
                a: True4False7,
                b: True4False7,
                prob_a: 0,
            },
            Outcome {
                a: True4False7,
                b: True4False7,
                prob_a: 0,
            },
            Outcome {
                a: True5False6,
                b: True5False6,
                prob_a: 0,
            },
            Outcome {
                a: True5False6,
                b: True5False6,
                prob_a: 0,
            },
            Outcome {
                a: True6False5,
                b: True6False5,
                prob_a: 0,
            },
            Outcome {
                a: True6False5,
                b: True6False5,
                prob_a: 0,
            },
            Outcome {
                a: True7False4,
                b: True7False4,
                prob_a: 0,
            },
            Outcome {
                a: True7False4,
                b: True7False4,
                prob_a: 0,
            },
            Outcome {
                a: True8False3,
                b: True8False3,
                prob_a: 0,
            },
            Outcome {
                a: True8False3,
                b: True8False3,
                prob_a: 0,
            },
            Outcome {
                a: True9False2,
                b: True9False2,
                prob_a: 0,
            },
            Outcome {
                a: True9False2,
                b: True9False2,
                prob_a: 0,
            },
            Outcome {
                a: True10False1,
                b: True10False1,
                prob_a: 0,
            },
            Outcome {
                a: True10False1,
                b: True10False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse5,
                b: AllFalse4,
                prob_a: 0xf800000000000000,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse6,
                b: AllFalse5,
                prob_a: 0xfc00000000000000,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse7,
                b: AllFalse6,
                prob_a: 0xfe00000000000000,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse8,
                b: AllFalse7,
                prob_a: 0xff00000000000000,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse9,
                b: AllFalse8,
                prob_a: 0xff80000000000000,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse10,
                b: AllFalse9,
                prob_a: 0xffc0000000000000,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse11,
                b: AllFalse10,
                prob_a: 0xffe0000000000000,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse12,
                b: AllFalse11,
                prob_a: 0xfff0000000000000,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse13,
                b: AllFalse12,
                prob_a: 0xfff8000000000000,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse14,
                b: AllFalse13,
                prob_a: 0xfffc000000000000,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse15,
                b: AllFalse14,
                prob_a: 0xfffe000000000000,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: AllFalse15,
                b: AllFalse15,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue15,
                b: AllTrue15,
                prob_a: 0,
            },
            Outcome {
                a: True1False0,
                b: True1False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False1,
                b: True1False1,
                prob_a: 0,
            },
            Outcome {
                a: True2False0,
                b: True2False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False2,
                b: True1False2,
                prob_a: 0,
            },
            Outcome {
                a: True2False1,
                b: True2False1,
                prob_a: 0,
            },
            Outcome {
                a: True3False0,
                b: True3False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False3,
                b: True1False3,
                prob_a: 0,
            },
            Outcome {
                a: True2False2,
                b: True2False2,
                prob_a: 0,
            },
            Outcome {
                a: True3False1,
                b: True3False1,
                prob_a: 0,
            },
            Outcome {
                a: True4False0,
                b: True4False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False4,
                b: True1False4,
                prob_a: 0,
            },
            Outcome {
                a: True2False3,
                b: True2False3,
                prob_a: 0,
            },
            Outcome {
                a: True3False2,
                b: True3False2,
                prob_a: 0,
            },
            Outcome {
                a: True4False1,
                b: True4False1,
                prob_a: 0,
            },
            Outcome {
                a: True5False0,
                b: True5False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False5,
                b: True1False5,
                prob_a: 0,
            },
            Outcome {
                a: True2False4,
                b: True2False4,
                prob_a: 0,
            },
            Outcome {
                a: True3False3,
                b: True3False3,
                prob_a: 0,
            },
            Outcome {
                a: True4False2,
                b: True4False2,
                prob_a: 0,
            },
            Outcome {
                a: True5False1,
                b: True5False1,
                prob_a: 0,
            },
            Outcome {
                a: True6False0,
                b: True6False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False6,
                b: True1False6,
                prob_a: 0,
            },
            Outcome {
                a: True2False5,
                b: True2False5,
                prob_a: 0,
            },
            Outcome {
                a: True3False4,
                b: True3False4,
                prob_a: 0,
            },
            Outcome {
                a: True4False3,
                b: True4False3,
                prob_a: 0,
            },
            Outcome {
                a: True5False2,
                b: True5False2,
                prob_a: 0,
            },
            Outcome {
                a: True6False1,
                b: True6False1,
                prob_a: 0,
            },
            Outcome {
                a: True7False0,
                b: True7False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False7,
                b: True1False7,
                prob_a: 0,
            },
            Outcome {
                a: True2False6,
                b: True2False6,
                prob_a: 0,
            },
            Outcome {
                a: True3False5,
                b: True3False5,
                prob_a: 0,
            },
            Outcome {
                a: True4False4,
                b: True4False4,
                prob_a: 0,
            },
            Outcome {
                a: True5False3,
                b: True5False3,
                prob_a: 0,
            },
            Outcome {
                a: True6False2,
                b: True6False2,
                prob_a: 0,
            },
            Outcome {
                a: True7False1,
                b: True7False1,
                prob_a: 0,
            },
            Outcome {
                a: True8False0,
                b: True8False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False8,
                b: True1False8,
                prob_a: 0,
            },
            Outcome {
                a: True2False7,
                b: True2False7,
                prob_a: 0,
            },
            Outcome {
                a: True3False6,
                b: True3False6,
                prob_a: 0,
            },
            Outcome {
                a: True4False5,
                b: True4False5,
                prob_a: 0,
            },
            Outcome {
                a: True5False4,
                b: True5False4,
                prob_a: 0,
            },
            Outcome {
                a: True6False3,
                b: True6False3,
                prob_a: 0,
            },
            Outcome {
                a: True7False2,
                b: True7False2,
                prob_a: 0,
            },
            Outcome {
                a: True8False1,
                b: True8False1,
                prob_a: 0,
            },
            Outcome {
                a: True9False0,
                b: True9False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False9,
                b: True1False9,
                prob_a: 0,
            },
            Outcome {
                a: True2False8,
                b: True2False8,
                prob_a: 0,
            },
            Outcome {
                a: True3False7,
                b: True3False7,
                prob_a: 0,
            },
            Outcome {
                a: True4False6,
                b: True4False6,
                prob_a: 0,
            },
            Outcome {
                a: True5False5,
                b: True5False5,
                prob_a: 0,
            },
            Outcome {
                a: True6False4,
                b: True6False4,
                prob_a: 0,
            },
            Outcome {
                a: True7False3,
                b: True7False3,
                prob_a: 0,
            },
            Outcome {
                a: True8False2,
                b: True8False2,
                prob_a: 0,
            },
            Outcome {
                a: True9False1,
                b: True9False1,
                prob_a: 0,
            },
            Outcome {
                a: True10False0,
                b: True10False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False10,
                b: True1False10,
                prob_a: 0,
            },
            Outcome {
                a: True2False9,
                b: True2False9,
                prob_a: 0,
            },
            Outcome {
                a: True3False8,
                b: True3False8,
                prob_a: 0,
            },
            Outcome {
                a: True4False7,
                b: True4False7,
                prob_a: 0,
            },
            Outcome {
                a: True5False6,
                b: True5False6,
                prob_a: 0,
            },
            Outcome {
                a: True6False5,
                b: True6False5,
                prob_a: 0,
            },
            Outcome {
                a: True7False4,
                b: True7False4,
                prob_a: 0,
            },
            Outcome {
                a: True8False3,
                b: True8False3,
                prob_a: 0,
            },
            Outcome {
                a: True9False2,
                b: True9False2,
                prob_a: 0,
            },
            Outcome {
                a: True10False1,
                b: True10False1,
                prob_a: 0,
            },
            Outcome {
                a: True11False0,
                b: True11False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False11,
                b: True1False11,
                prob_a: 0,
            },
            Outcome {
                a: True2False10,
                b: True2False10,
                prob_a: 0,
            },
            Outcome {
                a: True3False9,
                b: True3False9,
                prob_a: 0,
            },
            Outcome {
                a: True4False8,
                b: True4False8,
                prob_a: 0,
            },
            Outcome {
                a: True5False7,
                b: True5False7,
                prob_a: 0,
            },
            Outcome {
                a: True6False6,
                b: True6False6,
                prob_a: 0,
            },
            Outcome {
                a: True7False5,
                b: True7False5,
                prob_a: 0,
            },
            Outcome {
                a: True8False4,
                b: True8False4,
                prob_a: 0,
            },
            Outcome {
                a: True9False3,
                b: True9False3,
                prob_a: 0,
            },
            Outcome {
                a: True10False2,
                b: True10False2,
                prob_a: 0,
            },
            Outcome {
                a: True11False1,
                b: True11False1,
                prob_a: 0,
            },
            Outcome {
                a: True12False0,
                b: True12False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False12,
                b: True1False12,
                prob_a: 0,
            },
            Outcome {
                a: True2False11,
                b: True2False11,
                prob_a: 0,
            },
            Outcome {
                a: True3False10,
                b: True3False10,
                prob_a: 0,
            },
            Outcome {
                a: True4False9,
                b: True4False9,
                prob_a: 0,
            },
            Outcome {
                a: True5False8,
                b: True5False8,
                prob_a: 0,
            },
            Outcome {
                a: True6False7,
                b: True6False7,
                prob_a: 0,
            },
            Outcome {
                a: True7False6,
                b: True7False6,
                prob_a: 0,
            },
            Outcome {
                a: True8False5,
                b: True8False5,
                prob_a: 0,
            },
            Outcome {
                a: True9False4,
                b: True9False4,
                prob_a: 0,
            },
            Outcome {
                a: True10False3,
                b: True10False3,
                prob_a: 0,
            },
            Outcome {
                a: True11False2,
                b: True11False2,
                prob_a: 0,
            },
            Outcome {
                a: True12False1,
                b: True12False1,
                prob_a: 0,
            },
            Outcome {
                a: True13False0,
                b: True13False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False13,
                b: True1False13,
                prob_a: 0,
            },
            Outcome {
                a: True2False12,
                b: True2False12,
                prob_a: 0,
            },
            Outcome {
                a: True3False11,
                b: True3False11,
                prob_a: 0,
            },
            Outcome {
                a: True4False10,
                b: True4False10,
                prob_a: 0,
            },
            Outcome {
                a: True5False9,
                b: True5False9,
                prob_a: 0,
            },
            Outcome {
                a: True6False8,
                b: True6False8,
                prob_a: 0,
            },
            Outcome {
                a: True7False7,
                b: True7False7,
                prob_a: 0,
            },
            Outcome {
                a: True8False6,
                b: True8False6,
                prob_a: 0,
            },
            Outcome {
                a: True9False5,
                b: True9False5,
                prob_a: 0,
            },
            Outcome {
                a: True10False4,
                b: True10False4,
                prob_a: 0,
            },
            Outcome {
                a: True11False3,
                b: True11False3,
                prob_a: 0,
            },
            Outcome {
                a: True12False2,
                b: True12False2,
                prob_a: 0,
            },
            Outcome {
                a: True13False1,
                b: True13False1,
                prob_a: 0,
            },
            Outcome {
                a: True14False0,
                b: True14False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False14,
                b: True1False14,
                prob_a: 0,
            },
            Outcome {
                a: True2False13,
                b: True2False13,
                prob_a: 0,
            },
            Outcome {
                a: True3False12,
                b: True3False12,
                prob_a: 0,
            },
            Outcome {
                a: True4False11,
                b: True4False11,
                prob_a: 0,
            },
            Outcome {
                a: True5False10,
                b: True5False10,
                prob_a: 0,
            },
            Outcome {
                a: True6False9,
                b: True6False9,
                prob_a: 0,
            },
            Outcome {
                a: True7False8,
                b: True7False8,
                prob_a: 0,
            },
            Outcome {
                a: True8False7,
                b: True8False7,
                prob_a: 0,
            },
            Outcome {
                a: True9False6,
                b: True9False6,
                prob_a: 0,
            },
            Outcome {
                a: True10False5,
                b: True10False5,
                prob_a: 0,
            },
            Outcome {
                a: True11False4,
                b: True11False4,
                prob_a: 0,
            },
            Outcome {
                a: True12False3,
                b: True12False3,
                prob_a: 0,
            },
            Outcome {
                a: True13False2,
                b: True13False2,
                prob_a: 0,
            },
            Outcome {
                a: True14False1,
                b: True14False1,
                prob_a: 0,
            },
            Outcome {
                a: True15False0,
                b: True15False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False15,
                b: True1False15,
                prob_a: 0,
            },
            Outcome {
                a: True2False14,
                b: True2False14,
                prob_a: 0,
            },
            Outcome {
                a: True3False13,
                b: True3False13,
                prob_a: 0,
            },
            Outcome {
                a: True4False12,
                b: True4False12,
                prob_a: 0,
            },
            Outcome {
                a: True5False11,
                b: True5False11,
                prob_a: 0,
            },
            Outcome {
                a: True6False10,
                b: True6False10,
                prob_a: 0,
            },
            Outcome {
                a: True7False9,
                b: True7False9,
                prob_a: 0,
            },
            Outcome {
                a: True8False8,
                b: True8False8,
                prob_a: 0,
            },
            Outcome {
                a: True9False7,
                b: True9False7,
                prob_a: 0,
            },
            Outcome {
                a: True10False6,
                b: True10False6,
                prob_a: 0,
            },
            Outcome {
                a: True11False5,
                b: True11False5,
                prob_a: 0,
            },
            Outcome {
                a: True12False4,
                b: True12False4,
                prob_a: 0,
            },
            Outcome {
                a: True13False3,
                b: True13False3,
                prob_a: 0,
            },
            Outcome {
                a: True14False2,
                b: True14False2,
                prob_a: 0,
            },
            Outcome {
                a: True15False1,
                b: True15False1,
                prob_a: 0,
            },
            Outcome {
                a: True16False0,
                b: True16False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: True2False15,
                b: True2False15,
                prob_a: 0,
            },
            Outcome {
                a: True3False14,
                b: True3False14,
                prob_a: 0,
            },
            Outcome {
                a: True4False13,
                b: True4False13,
                prob_a: 0,
            },
            Outcome {
                a: True5False12,
                b: True5False12,
                prob_a: 0,
            },
            Outcome {
                a: True6False11,
                b: True6False11,
                prob_a: 0,
            },
            Outcome {
                a: True7False10,
                b: True7False10,
                prob_a: 0,
            },
            Outcome {
                a: True8False9,
                b: True8False9,
                prob_a: 0,
            },
            Outcome {
                a: True9False8,
                b: True9False8,
                prob_a: 0,
            },
            Outcome {
                a: True10False7,
                b: True10False7,
                prob_a: 0,
            },
            Outcome {
                a: True11False6,
                b: True11False6,
                prob_a: 0,
            },
            Outcome {
                a: True12False5,
                b: True12False5,
                prob_a: 0,
            },
            Outcome {
                a: True13False4,
                b: True13False4,
                prob_a: 0,
            },
            Outcome {
                a: True14False3,
                b: True14False3,
                prob_a: 0,
            },
            Outcome {
                a: True15False2,
                b: True15False2,
                prob_a: 0,
            },
            Outcome {
                a: True16False1,
                b: True16False1,
                prob_a: 0,
            },
            Outcome {
                a: True17False0,
                b: True17False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False17,
                b: True1False17,
                prob_a: 0,
            },
            Outcome {
                a: True2False16,
                b: True2False16,
                prob_a: 0,
            },
            Outcome {
                a: True3False15,
                b: True3False15,
                prob_a: 0,
            },
            Outcome {
                a: True4False14,
                b: True4False14,
                prob_a: 0,
            },
            Outcome {
                a: True5False13,
                b: True5False13,
                prob_a: 0,
            },
            Outcome {
                a: True6False12,
                b: True6False12,
                prob_a: 0,
            },
            Outcome {
                a: True7False11,
                b: True7False11,
                prob_a: 0,
            },
            Outcome {
                a: True8False10,
                b: True8False10,
                prob_a: 0,
            },
            Outcome {
                a: True9False9,
                b: True9False9,
                prob_a: 0,
            },
            Outcome {
                a: True10False8,
                b: True10False8,
                prob_a: 0,
            },
            Outcome {
                a: True11False7,
                b: True11False7,
                prob_a: 0,
            },
            Outcome {
                a: True12False6,
                b: True12False6,
                prob_a: 0,
            },
            Outcome {
                a: True13False5,
                b: True13False5,
                prob_a: 0,
            },
            Outcome {
                a: True14False4,
                b: True14False4,
                prob_a: 0,
            },
            Outcome {
                a: True15False3,
                b: True15False3,
                prob_a: 0,
            },
            Outcome {
                a: True16False2,
                b: True16False2,
                prob_a: 0,
            },
            Outcome {
                a: True17False1,
                b: True17False1,
                prob_a: 0,
            },
            Outcome {
                a: True18False0,
                b: True18False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False18,
                b: True1False18,
                prob_a: 0,
            },
            Outcome {
                a: True2False17,
                b: True2False17,
                prob_a: 0,
            },
            Outcome {
                a: True3False16,
                b: True3False16,
                prob_a: 0,
            },
            Outcome {
                a: True4False15,
                b: True4False15,
                prob_a: 0,
            },
            Outcome {
                a: True5False14,
                b: True5False14,
                prob_a: 0,
            },
            Outcome {
                a: True6False13,
                b: True6False13,
                prob_a: 0,
            },
            Outcome {
                a: True7False12,
                b: True7False12,
                prob_a: 0,
            },
            Outcome {
                a: True8False11,
                b: True8False11,
                prob_a: 0,
            },
            Outcome {
                a: True9False10,
                b: True9False10,
                prob_a: 0,
            },
            Outcome {
                a: True10False9,
                b: True10False9,
                prob_a: 0,
            },
            Outcome {
                a: True11False8,
                b: True11False8,
                prob_a: 0,
            },
            Outcome {
                a: True12False7,
                b: True12False7,
                prob_a: 0,
            },
            Outcome {
                a: True13False6,
                b: True13False6,
                prob_a: 0,
            },
            Outcome {
                a: True14False5,
                b: True14False5,
                prob_a: 0,
            },
            Outcome {
                a: True15False4,
                b: True15False4,
                prob_a: 0,
            },
            Outcome {
                a: True16False3,
                b: True16False3,
                prob_a: 0,
            },
            Outcome {
                a: True17False2,
                b: True17False2,
                prob_a: 0,
            },
            Outcome {
                a: True18False1,
                b: True18False1,
                prob_a: 0,
            },
            Outcome {
                a: True19False0,
                b: True19False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False19,
                b: True1False19,
                prob_a: 0,
            },
            Outcome {
                a: True2False18,
                b: True2False18,
                prob_a: 0,
            },
            Outcome {
                a: True3False17,
                b: True3False17,
                prob_a: 0,
            },
            Outcome {
                a: True4False16,
                b: True4False16,
                prob_a: 0,
            },
            Outcome {
                a: True5False15,
                b: True5False15,
                prob_a: 0,
            },
            Outcome {
                a: True6False14,
                b: True6False14,
                prob_a: 0,
            },
            Outcome {
                a: True7False13,
                b: True7False13,
                prob_a: 0,
            },
            Outcome {
                a: True8False12,
                b: True8False12,
                prob_a: 0,
            },
            Outcome {
                a: True9False11,
                b: True9False11,
                prob_a: 0,
            },
            Outcome {
                a: True10False10,
                b: True10False10,
                prob_a: 0,
            },
            Outcome {
                a: True11False9,
                b: True11False9,
                prob_a: 0,
            },
            Outcome {
                a: True12False8,
                b: True12False8,
                prob_a: 0,
            },
            Outcome {
                a: True13False7,
                b: True13False7,
                prob_a: 0,
            },
            Outcome {
                a: True14False6,
                b: True14False6,
                prob_a: 0,
            },
            Outcome {
                a: True15False5,
                b: True15False5,
                prob_a: 0,
            },
            Outcome {
                a: True16False4,
                b: True16False4,
                prob_a: 0,
            },
            Outcome {
                a: True17False3,
                b: True17False3,
                prob_a: 0,
            },
            Outcome {
                a: True18False2,
                b: True18False2,
                prob_a: 0,
            },
            Outcome {
                a: True19False1,
                b: True19False1,
                prob_a: 0,
            },
            Outcome {
                a: True20False0,
                b: True20False0,
                prob_a: 0,
            },
            Outcome {
                a: True1False10,
                b: True1False10,
                prob_a: 0,
            },
            Outcome {
                a: True1False10,
                b: True1False10,
                prob_a: 0,
            },
            Outcome {
                a: True2False9,
                b: True2False9,
                prob_a: 0,
            },
            Outcome {
                a: True2False9,
                b: True2False9,
                prob_a: 0,
            },
            Outcome {
                a: True3False8,
                b: True3False8,
                prob_a: 0,
            },
            Outcome {
                a: True3False8,
                b: True3False8,
                prob_a: 0,
            },
            Outcome {
                a: True4False7,
                b: True4False7,
                prob_a: 0,
            },
            Outcome {
                a: True4False7,
                b: True4False7,
                prob_a: 0,
            },
            Outcome {
                a: True5False6,
                b: True5False6,
                prob_a: 0,
            },
            Outcome {
                a: True5False6,
                b: True5False6,
                prob_a: 0,
            },
            Outcome {
                a: True6False5,
                b: True6False5,
                prob_a: 0,
            },
            Outcome {
                a: True6False5,
                b: True6False5,
                prob_a: 0,
            },
            Outcome {
                a: True7False4,
                b: True7False4,
                prob_a: 0,
            },
            Outcome {
                a: True7False4,
                b: True7False4,
                prob_a: 0,
            },
            Outcome {
                a: True8False3,
                b: True8False3,
                prob_a: 0,
            },
            Outcome {
                a: True8False3,
                b: True8False3,
                prob_a: 0,
            },
            Outcome {
                a: True9False2,
                b: True9False2,
                prob_a: 0,
            },
            Outcome {
                a: True9False2,
                b: True9False2,
                prob_a: 0,
            },
            Outcome {
                a: True10False1,
                b: True10False1,
                prob_a: 0,
            },
            Outcome {
                a: True10False1,
                b: True10False1,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue4,
                b: AllTrue4,
                prob_a: 0,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue5,
                b: AllTrue4,
                prob_a: 0xf800000000000000,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue6,
                b: AllTrue5,
                prob_a: 0xfc00000000000000,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue7,
                b: AllTrue6,
                prob_a: 0xfe00000000000000,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue8,
                b: AllTrue7,
                prob_a: 0xff00000000000000,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue9,
                b: AllTrue8,
                prob_a: 0xff80000000000000,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue10,
                b: AllTrue9,
                prob_a: 0xffc0000000000000,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue11,
                b: AllTrue10,
                prob_a: 0xffe0000000000000,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue12,
                b: AllTrue11,
                prob_a: 0xfff0000000000000,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue13,
                b: AllTrue12,
                prob_a: 0xfff8000000000000,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue14,
                b: AllTrue13,
                prob_a: 0xfffc000000000000,
            },
            Outcome {
                a: True1False16,
                b: True1False16,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue15,
                b: AllTrue14,
                prob_a: 0xfffe000000000000,
            },
            Outcome {
                a: AllFalse15,
                b: AllFalse15,
                prob_a: 0,
            },
            Outcome {
                a: AllTrue15,
                b: AllTrue15,
                prob_a: 0,
            },
        ];
        let idx = (self as usize) + (bit as usize) * 255;
        let Outcome { a, b, prob_a } = OUTCOMES[idx];
        if prob_a == 0 {
            a
        } else if rng.next() < prob_a {
            a
        } else {
            b
        }
    }
}
// Count of variants: 255
