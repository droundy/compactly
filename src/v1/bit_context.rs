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
    True0False21,  // Probability { prob: 61, shift: 6 } = 0.953125
    True1False20,  // Probability { prob: 243, shift: 8 } = 0.94921875
    True2False19,  // Probability { prob: 231, shift: 8 } = 0.90234375
    True3False18,  // Probability { prob: 219, shift: 8 } = 0.85546875
    True4False17,  // Probability { prob: 207, shift: 8 } = 0.80859375
    True5False16,  // Probability { prob: 195, shift: 8 } = 0.76171875
    True6False15,  // Probability { prob: 91, shift: 7 } = 0.7109375
    True7False14,  // Probability { prob: 85, shift: 7 } = 0.6640625
    True8False13,  // Probability { prob: 79, shift: 7 } = 0.6171875
    True9False12,  // Probability { prob: 73, shift: 7 } = 0.5703125
    True10False11, // Probability { prob: 67, shift: 7 } = 0.5234375
    True11False10, // Probability { prob: 121, shift: 8 } = 0.47265625
    True12False9,  // Probability { prob: 109, shift: 8 } = 0.42578125
    True13False8,  // Probability { prob: 97, shift: 8 } = 0.37890625
    True14False7,  // Probability { prob: 85, shift: 8 } = 0.33203125
    True15False6,  // Probability { prob: 73, shift: 8 } = 0.28515625
    True16False5,  // Probability { prob: 15, shift: 6 } = 0.234375
    True17False4,  // Probability { prob: 3, shift: 4 } = 0.1875
    True18False3,  // Probability { prob: 9, shift: 6 } = 0.140625
    True19False2,  // Probability { prob: 3, shift: 5 } = 0.09375
    True20False1,  // Probability { prob: 3, shift: 6 } = 0.046875
    True21False0,  // Probability { prob: 11, shift: 8 } = 0.04296875
    True0False22,  // Probability { prob: 245, shift: 8 } = 0.95703125
    True1False21,  // Probability { prob: 61, shift: 6 } = 0.953125
    True2False20,  // Probability { prob: 29, shift: 5 } = 0.90625
    True3False19,  // Probability { prob: 221, shift: 8 } = 0.86328125
    True4False18,  // Probability { prob: 209, shift: 8 } = 0.81640625
    True5False17,  // Probability { prob: 197, shift: 8 } = 0.76953125
    True6False16,  // Probability { prob: 93, shift: 7 } = 0.7265625
    True7False15,  // Probability { prob: 87, shift: 7 } = 0.6796875
    True8False14,  // Probability { prob: 81, shift: 7 } = 0.6328125
    True9False13,  // Probability { prob: 151, shift: 8 } = 0.58984375
    True10False12, // Probability { prob: 139, shift: 8 } = 0.54296875
    True11False11, // Probability { prob: 2, shift: 2 } = 0.5
    True12False10, // Probability { prob: 29, shift: 6 } = 0.453125
    True13False9,  // Probability { prob: 13, shift: 5 } = 0.40625
    True14False8,  // Probability { prob: 93, shift: 8 } = 0.36328125
    True15False7,  // Probability { prob: 81, shift: 8 } = 0.31640625
    True16False6,  // Probability { prob: 69, shift: 8 } = 0.26953125
    True17False5,  // Probability { prob: 29, shift: 7 } = 0.2265625
    True18False4,  // Probability { prob: 23, shift: 7 } = 0.1796875
    True19False3,  // Probability { prob: 17, shift: 7 } = 0.1328125
    True20False2,  // Probability { prob: 23, shift: 8 } = 0.08984375
    True21False1,  // Probability { prob: 11, shift: 8 } = 0.04296875
    True22False0,  // Probability { prob: 5, shift: 7 } = 0.0390625
    True0False23,  // Probability { prob: 245, shift: 8 } = 0.95703125
    True1False22,  // Probability { prob: 61, shift: 6 } = 0.953125
    True2False21,  // Probability { prob: 233, shift: 8 } = 0.91015625
    True3False20,  // Probability { prob: 111, shift: 7 } = 0.8671875
    True4False19,  // Probability { prob: 211, shift: 8 } = 0.82421875
    True5False18,  // Probability { prob: 25, shift: 5 } = 0.78125
    True6False17,  // Probability { prob: 189, shift: 8 } = 0.73828125
    True7False16,  // Probability { prob: 89, shift: 7 } = 0.6953125
    True8False15,  // Probability { prob: 83, shift: 7 } = 0.6484375
    True9False14,  // Probability { prob: 155, shift: 8 } = 0.60546875
    True10False13, // Probability { prob: 9, shift: 4 } = 0.5625
    True11False12, // Probability { prob: 133, shift: 8 } = 0.51953125
    True12False11, // Probability { prob: 61, shift: 7 } = 0.4765625
    True13False10, // Probability { prob: 111, shift: 8 } = 0.43359375
    True14False9,  // Probability { prob: 25, shift: 6 } = 0.390625
    True15False8,  // Probability { prob: 89, shift: 8 } = 0.34765625
    True16False7,  // Probability { prob: 77, shift: 8 } = 0.30078125
    True17False6,  // Probability { prob: 33, shift: 7 } = 0.2578125
    True18False5,  // Probability { prob: 55, shift: 8 } = 0.21484375
    True19False4,  // Probability { prob: 11, shift: 6 } = 0.171875
    True20False3,  // Probability { prob: 33, shift: 8 } = 0.12890625
    True21False2,  // Probability { prob: 11, shift: 7 } = 0.0859375
    True22False1,  // Probability { prob: 11, shift: 8 } = 0.04296875
    True23False0,  // Probability { prob: 5, shift: 7 } = 0.0390625
    True0False24,  // Probability { prob: 123, shift: 7 } = 0.9609375
    True1False23,  // Probability { prob: 245, shift: 8 } = 0.95703125
    True2False22,  // Probability { prob: 117, shift: 7 } = 0.9140625
    True3False21,  // Probability { prob: 7, shift: 3 } = 0.875
    True4False20,  // Probability { prob: 213, shift: 8 } = 0.83203125
    True5False19,  // Probability { prob: 101, shift: 7 } = 0.7890625
    True6False18,  // Probability { prob: 3, shift: 2 } = 0.75
    True7False17,  // Probability { prob: 181, shift: 8 } = 0.70703125
    True8False16,  // Probability { prob: 85, shift: 7 } = 0.6640625
    True9False15,  // Probability { prob: 5, shift: 3 } = 0.625
    True10False14, // Probability { prob: 149, shift: 8 } = 0.58203125
    True11False13, // Probability { prob: 69, shift: 7 } = 0.5390625
    True12False12, // Probability { prob: 2, shift: 2 } = 0.5
    True13False11, // Probability { prob: 117, shift: 8 } = 0.45703125
    True14False10, // Probability { prob: 53, shift: 7 } = 0.4140625
    True15False9,  // Probability { prob: 3, shift: 3 } = 0.375
    True16False8,  // Probability { prob: 85, shift: 8 } = 0.33203125
    True17False7,  // Probability { prob: 37, shift: 7 } = 0.2890625
    True18False6,  // Probability { prob: 1, shift: 2 } = 0.25
    True19False5,  // Probability { prob: 53, shift: 8 } = 0.20703125
    True20False4,  // Probability { prob: 21, shift: 7 } = 0.1640625
    True21False3,  // Probability { prob: 1, shift: 3 } = 0.125
    True22False2,  // Probability { prob: 21, shift: 8 } = 0.08203125
    True23False1,  // Probability { prob: 5, shift: 7 } = 0.0390625
    True24False0,  // Probability { prob: 9, shift: 8 } = 0.03515625
    True0False25,  // Probability { prob: 123, shift: 7 } = 0.9609375
    True1False24,  // Probability { prob: 245, shift: 8 } = 0.95703125
    True2False23,  // Probability { prob: 235, shift: 8 } = 0.91796875
    True3False22,  // Probability { prob: 225, shift: 8 } = 0.87890625
    True4False21,  // Probability { prob: 215, shift: 8 } = 0.83984375
    True5False20,  // Probability { prob: 51, shift: 6 } = 0.796875
    True6False19,  // Probability { prob: 97, shift: 7 } = 0.7578125
    True7False18,  // Probability { prob: 23, shift: 5 } = 0.71875
    True8False17,  // Probability { prob: 87, shift: 7 } = 0.6796875
    True9False16,  // Probability { prob: 163, shift: 8 } = 0.63671875
    True10False15, // Probability { prob: 153, shift: 8 } = 0.59765625
    True11False14, // Probability { prob: 143, shift: 8 } = 0.55859375
    True12False13, // Probability { prob: 133, shift: 8 } = 0.51953125
    True13False12, // Probability { prob: 61, shift: 7 } = 0.4765625
    True14False11, // Probability { prob: 7, shift: 4 } = 0.4375
    True15False10, // Probability { prob: 51, shift: 7 } = 0.3984375
    True16False9,  // Probability { prob: 23, shift: 6 } = 0.359375
    True17False8,  // Probability { prob: 81, shift: 8 } = 0.31640625
    True18False7,  // Probability { prob: 71, shift: 8 } = 0.27734375
    True19False6,  // Probability { prob: 61, shift: 8 } = 0.23828125
    True20False5,  // Probability { prob: 51, shift: 8 } = 0.19921875
    True21False4,  // Probability { prob: 5, shift: 5 } = 0.15625
    True22False3,  // Probability { prob: 15, shift: 7 } = 0.1171875
    True23False2,  // Probability { prob: 5, shift: 6 } = 0.078125
    True24False1,  // Probability { prob: 5, shift: 7 } = 0.0390625
    True25False0,  // Probability { prob: 9, shift: 8 } = 0.03515625
    True0False26,  // Probability { prob: 123, shift: 7 } = 0.9609375
    True1False25,  // Probability { prob: 123, shift: 7 } = 0.9609375
    True2False24,  // Probability { prob: 59, shift: 6 } = 0.921875
    True3False23,  // Probability { prob: 113, shift: 7 } = 0.8828125
    True4False22,  // Probability { prob: 27, shift: 5 } = 0.84375
    True5False21,  // Probability { prob: 103, shift: 7 } = 0.8046875
    True6False20,  // Probability { prob: 49, shift: 6 } = 0.765625
    True7False19,  // Probability { prob: 187, shift: 8 } = 0.73046875
    True8False18,  // Probability { prob: 177, shift: 8 } = 0.69140625
    True9False17,  // Probability { prob: 167, shift: 8 } = 0.65234375
    True10False16, // Probability { prob: 157, shift: 8 } = 0.61328125
    True11False15, // Probability { prob: 147, shift: 8 } = 0.57421875
    True12False14, // Probability { prob: 137, shift: 8 } = 0.53515625
    True13False13, // Probability { prob: 2, shift: 2 } = 0.5
    True14False12, // Probability { prob: 59, shift: 7 } = 0.4609375
    True15False11, // Probability { prob: 27, shift: 6 } = 0.421875
    True16False10, // Probability { prob: 49, shift: 7 } = 0.3828125
    True17False9,  // Probability { prob: 11, shift: 5 } = 0.34375
    True18False8,  // Probability { prob: 39, shift: 7 } = 0.3046875
    True19False7,  // Probability { prob: 17, shift: 6 } = 0.265625
    True20False6,  // Probability { prob: 59, shift: 8 } = 0.23046875
    True21False5,  // Probability { prob: 49, shift: 8 } = 0.19140625
    True22False4,  // Probability { prob: 39, shift: 8 } = 0.15234375
    True23False3,  // Probability { prob: 29, shift: 8 } = 0.11328125
    True24False2,  // Probability { prob: 19, shift: 8 } = 0.07421875
    True25False1,  // Probability { prob: 9, shift: 8 } = 0.03515625
    True26False0,  // Probability { prob: 9, shift: 8 } = 0.03515625
    True0False27,  // Probability { prob: 247, shift: 8 } = 0.96484375
    True1False26,  // Probability { prob: 123, shift: 7 } = 0.9609375
    True2False25,  // Probability { prob: 237, shift: 8 } = 0.92578125
    True3False24,  // Probability { prob: 227, shift: 8 } = 0.88671875
    True4False23,  // Probability { prob: 109, shift: 7 } = 0.8515625
    True5False22,  // Probability { prob: 13, shift: 4 } = 0.8125
    True6False21,  // Probability { prob: 199, shift: 8 } = 0.77734375
    True7False20,  // Probability { prob: 189, shift: 8 } = 0.73828125
    True8False19,  // Probability { prob: 45, shift: 6 } = 0.703125
    True9False18,  // Probability { prob: 85, shift: 7 } = 0.6640625
    True10False17, // Probability { prob: 161, shift: 8 } = 0.62890625
    True11False16, // Probability { prob: 151, shift: 8 } = 0.58984375
    True12False15, // Probability { prob: 71, shift: 7 } = 0.5546875
    True13False14, // Probability { prob: 33, shift: 6 } = 0.515625
    True14False13, // Probability { prob: 123, shift: 8 } = 0.48046875
    True15False12, // Probability { prob: 113, shift: 8 } = 0.44140625
    True16False11, // Probability { prob: 13, shift: 5 } = 0.40625
    True17False10, // Probability { prob: 47, shift: 7 } = 0.3671875
    True18False9,  // Probability { prob: 85, shift: 8 } = 0.33203125
    True19False8,  // Probability { prob: 75, shift: 8 } = 0.29296875
    True20False7,  // Probability { prob: 33, shift: 7 } = 0.2578125
    True21False6,  // Probability { prob: 7, shift: 5 } = 0.21875
    True22False5,  // Probability { prob: 47, shift: 8 } = 0.18359375
    True23False4,  // Probability { prob: 37, shift: 8 } = 0.14453125
    True24False3,  // Probability { prob: 7, shift: 6 } = 0.109375
    True25False2,  // Probability { prob: 9, shift: 7 } = 0.0703125
    True26False1,  // Probability { prob: 9, shift: 8 } = 0.03515625
    True27False0,  // Probability { prob: 1, shift: 5 } = 0.03125
    True0False28,  // Probability { prob: 247, shift: 8 } = 0.96484375
    True1False27,  // Probability { prob: 123, shift: 7 } = 0.9609375
    True2False26,  // Probability { prob: 237, shift: 8 } = 0.92578125
    True3False25,  // Probability { prob: 57, shift: 6 } = 0.890625
    True4False24,  // Probability { prob: 219, shift: 8 } = 0.85546875
    True5False23,  // Probability { prob: 105, shift: 7 } = 0.8203125
    True6False22,  // Probability { prob: 201, shift: 8 } = 0.78515625
    True7False21,  // Probability { prob: 3, shift: 2 } = 0.75
    True8False20,  // Probability { prob: 91, shift: 7 } = 0.7109375
    True9False19,  // Probability { prob: 173, shift: 8 } = 0.67578125
    True10False18, // Probability { prob: 41, shift: 6 } = 0.640625
    True11False17, // Probability { prob: 155, shift: 8 } = 0.60546875
    True12False16, // Probability { prob: 73, shift: 7 } = 0.5703125
    True13False15, // Probability { prob: 137, shift: 8 } = 0.53515625
    True14False14, // Probability { prob: 2, shift: 2 } = 0.5
    True15False13, // Probability { prob: 59, shift: 7 } = 0.4609375
    True16False12, // Probability { prob: 109, shift: 8 } = 0.42578125
    True17False11, // Probability { prob: 25, shift: 6 } = 0.390625
    True18False10, // Probability { prob: 91, shift: 8 } = 0.35546875
    True19False9,  // Probability { prob: 41, shift: 7 } = 0.3203125
    True20False8,  // Probability { prob: 73, shift: 8 } = 0.28515625
    True21False7,  // Probability { prob: 1, shift: 2 } = 0.25
    True22False6,  // Probability { prob: 27, shift: 7 } = 0.2109375
    True23False5,  // Probability { prob: 45, shift: 8 } = 0.17578125
    True24False4,  // Probability { prob: 9, shift: 6 } = 0.140625
    True25False3,  // Probability { prob: 27, shift: 8 } = 0.10546875
    True26False2,  // Probability { prob: 9, shift: 7 } = 0.0703125
    True27False1,  // Probability { prob: 9, shift: 8 } = 0.03515625
    True28False0,  // Probability { prob: 1, shift: 5 } = 0.03125
    True0False29,  // Probability { prob: 247, shift: 8 } = 0.96484375
    True1False28,  // Probability { prob: 247, shift: 8 } = 0.96484375
    True2False27,  // Probability { prob: 119, shift: 7 } = 0.9296875
    True3False26,  // Probability { prob: 229, shift: 8 } = 0.89453125
    True4False25,  // Probability { prob: 55, shift: 6 } = 0.859375
    True5False24,  // Probability { prob: 211, shift: 8 } = 0.82421875
    True6False23,  // Probability { prob: 203, shift: 8 } = 0.79296875
    True7False22,  // Probability { prob: 97, shift: 7 } = 0.7578125
    True8False21,  // Probability { prob: 185, shift: 8 } = 0.72265625
    True9False20,  // Probability { prob: 11, shift: 4 } = 0.6875
    True10False19, // Probability { prob: 167, shift: 8 } = 0.65234375
    True11False18, // Probability { prob: 79, shift: 7 } = 0.6171875
    True12False17, // Probability { prob: 75, shift: 7 } = 0.5859375
    True13False16, // Probability { prob: 141, shift: 8 } = 0.55078125
    True14False15, // Probability { prob: 33, shift: 6 } = 0.515625
    True15False14, // Probability { prob: 123, shift: 8 } = 0.48046875
    True16False13, // Probability { prob: 57, shift: 7 } = 0.4453125
    True17False12, // Probability { prob: 105, shift: 8 } = 0.41015625
    True18False11, // Probability { prob: 97, shift: 8 } = 0.37890625
    True19False10, // Probability { prob: 11, shift: 5 } = 0.34375
    True20False9,  // Probability { prob: 79, shift: 8 } = 0.30859375
    True21False8,  // Probability { prob: 35, shift: 7 } = 0.2734375
    True22False7,  // Probability { prob: 61, shift: 8 } = 0.23828125
    True23False6,  // Probability { prob: 13, shift: 6 } = 0.203125
    True24False5,  // Probability { prob: 11, shift: 6 } = 0.171875
    True25False4,  // Probability { prob: 35, shift: 8 } = 0.13671875
    True26False3,  // Probability { prob: 13, shift: 7 } = 0.1015625
    True27False2,  // Probability { prob: 17, shift: 8 } = 0.06640625
    True28False1,  // Probability { prob: 1, shift: 5 } = 0.03125
    True29False0,  // Probability { prob: 1, shift: 5 } = 0.03125
    True0False30,  // Probability { prob: 31, shift: 5 } = 0.96875
    True1False29,  // Probability { prob: 247, shift: 8 } = 0.96484375
    True2False28,  // Probability { prob: 119, shift: 7 } = 0.9296875
    True3False27,  // Probability { prob: 115, shift: 7 } = 0.8984375
    True4False26,  // Probability { prob: 221, shift: 8 } = 0.86328125
    True5False25,  // Probability { prob: 213, shift: 8 } = 0.83203125
    True6False24,  // Probability { prob: 51, shift: 6 } = 0.796875
    True7False23,  // Probability { prob: 49, shift: 6 } = 0.765625
    True8False22,  // Probability { prob: 187, shift: 8 } = 0.73046875
    True9False21,  // Probability { prob: 179, shift: 8 } = 0.69921875
    True10False20, // Probability { prob: 85, shift: 7 } = 0.6640625
    True11False19, // Probability { prob: 81, shift: 7 } = 0.6328125
    True12False18, // Probability { prob: 153, shift: 8 } = 0.59765625
    True13False17, // Probability { prob: 145, shift: 8 } = 0.56640625
    True14False16, // Probability { prob: 17, shift: 5 } = 0.53125
    True15False15, // Probability { prob: 2, shift: 2 } = 0.5
    True16False14, // Probability { prob: 119, shift: 8 } = 0.46484375
    True17False13, // Probability { prob: 55, shift: 7 } = 0.4296875
    True18False12, // Probability { prob: 51, shift: 7 } = 0.3984375
    True19False11, // Probability { prob: 93, shift: 8 } = 0.36328125
    True20False10, // Probability { prob: 85, shift: 8 } = 0.33203125
    True21False9,  // Probability { prob: 19, shift: 6 } = 0.296875
    True22False8,  // Probability { prob: 17, shift: 6 } = 0.265625
    True23False7,  // Probability { prob: 59, shift: 8 } = 0.23046875
    True24False6,  // Probability { prob: 51, shift: 8 } = 0.19921875
    True25False5,  // Probability { prob: 21, shift: 7 } = 0.1640625
    True26False4,  // Probability { prob: 17, shift: 7 } = 0.1328125
    True27False3,  // Probability { prob: 25, shift: 8 } = 0.09765625
    True28False2,  // Probability { prob: 17, shift: 8 } = 0.06640625
    True29False1,  // Probability { prob: 1, shift: 5 } = 0.03125
    True30False0,  // Probability { prob: 1, shift: 5 } = 0.03125
    True0False31,  // Probability { prob: 31, shift: 5 } = 0.96875
    True1False30,  // Probability { prob: 247, shift: 8 } = 0.96484375
    True2False29,  // Probability { prob: 239, shift: 8 } = 0.93359375
    True3False28,  // Probability { prob: 231, shift: 8 } = 0.90234375
    True4False27,  // Probability { prob: 111, shift: 7 } = 0.8671875
    True5False26,  // Probability { prob: 107, shift: 7 } = 0.8359375
    True6False25,  // Probability { prob: 103, shift: 7 } = 0.8046875
    True7False24,  // Probability { prob: 99, shift: 7 } = 0.7734375
    True8False23,  // Probability { prob: 189, shift: 8 } = 0.73828125
    True9False22,  // Probability { prob: 181, shift: 8 } = 0.70703125
    True10False21, // Probability { prob: 173, shift: 8 } = 0.67578125
    True11False20, // Probability { prob: 165, shift: 8 } = 0.64453125
    True12False19, // Probability { prob: 39, shift: 6 } = 0.609375
    True13False18, // Probability { prob: 37, shift: 6 } = 0.578125
    True14False17, // Probability { prob: 35, shift: 6 } = 0.546875
    True15False16, // Probability { prob: 33, shift: 6 } = 0.515625
    True16False15, // Probability { prob: 123, shift: 8 } = 0.48046875
    True17False14, // Probability { prob: 115, shift: 8 } = 0.44921875
    True18False13, // Probability { prob: 107, shift: 8 } = 0.41796875
    True19False12, // Probability { prob: 99, shift: 8 } = 0.38671875
    True20False11, // Probability { prob: 45, shift: 7 } = 0.3515625
    True21False10, // Probability { prob: 41, shift: 7 } = 0.3203125
    True22False9,  // Probability { prob: 37, shift: 7 } = 0.2890625
    True23False8,  // Probability { prob: 33, shift: 7 } = 0.2578125
    True24False7,  // Probability { prob: 57, shift: 8 } = 0.22265625
    True25False6,  // Probability { prob: 49, shift: 8 } = 0.19140625
    True26False5,  // Probability { prob: 41, shift: 8 } = 0.16015625
    True27False4,  // Probability { prob: 33, shift: 8 } = 0.12890625
    True28False3,  // Probability { prob: 3, shift: 5 } = 0.09375
    True29False2,  // Probability { prob: 1, shift: 4 } = 0.0625
    True30False1,  // Probability { prob: 1, shift: 5 } = 0.03125
    True31False0,  // Probability { prob: 7, shift: 8 } = 0.02734375
    True0False32,  // Probability { prob: 31, shift: 5 } = 0.96875
    True1False31,  // Probability { prob: 31, shift: 5 } = 0.96875
    True2False30,  // Probability { prob: 15, shift: 4 } = 0.9375
    True3False29,  // Probability { prob: 29, shift: 5 } = 0.90625
    True4False28,  // Probability { prob: 7, shift: 3 } = 0.875
    True5False27,  // Probability { prob: 27, shift: 5 } = 0.84375
    True6False26,  // Probability { prob: 13, shift: 4 } = 0.8125
    True7False25,  // Probability { prob: 25, shift: 5 } = 0.78125
    True8False24,  // Probability { prob: 3, shift: 2 } = 0.75
    True9False23,  // Probability { prob: 23, shift: 5 } = 0.71875
    True10False22, // Probability { prob: 11, shift: 4 } = 0.6875
    True11False21, // Probability { prob: 21, shift: 5 } = 0.65625
    True12False20, // Probability { prob: 5, shift: 3 } = 0.625
    True13False19, // Probability { prob: 19, shift: 5 } = 0.59375
    True14False18, // Probability { prob: 9, shift: 4 } = 0.5625
    True15False17, // Probability { prob: 17, shift: 5 } = 0.53125
    True16False16, // Probability { prob: 2, shift: 2 } = 0.5
    True17False15, // Probability { prob: 15, shift: 5 } = 0.46875
    True18False14, // Probability { prob: 7, shift: 4 } = 0.4375
    True19False13, // Probability { prob: 13, shift: 5 } = 0.40625
    True20False12, // Probability { prob: 3, shift: 3 } = 0.375
    True21False11, // Probability { prob: 11, shift: 5 } = 0.34375
    True22False10, // Probability { prob: 5, shift: 4 } = 0.3125
    True23False9,  // Probability { prob: 9, shift: 5 } = 0.28125
    True24False8,  // Probability { prob: 1, shift: 2 } = 0.25
    True25False7,  // Probability { prob: 7, shift: 5 } = 0.21875
    True26False6,  // Probability { prob: 3, shift: 4 } = 0.1875
    True27False5,  // Probability { prob: 5, shift: 5 } = 0.15625
    True28False4,  // Probability { prob: 1, shift: 3 } = 0.125
    True29False3,  // Probability { prob: 3, shift: 5 } = 0.09375
    True30False2,  // Probability { prob: 1, shift: 4 } = 0.0625
    True31False1,  // Probability { prob: 1, shift: 5 } = 0.03125
    True32False0,  // Probability { prob: 7, shift: 8 } = 0.02734375
    True0False33,  // Probability { prob: 31, shift: 5 } = 0.96875
    True1False32,  // Probability { prob: 31, shift: 5 } = 0.96875
    True2False31,  // Probability { prob: 15, shift: 4 } = 0.9375
    True3False30,  // Probability { prob: 29, shift: 5 } = 0.90625
    True4False29,  // Probability { prob: 7, shift: 3 } = 0.875
    True5False28,  // Probability { prob: 217, shift: 8 } = 0.84765625
    True6False27,  // Probability { prob: 209, shift: 8 } = 0.81640625
    True7False26,  // Probability { prob: 201, shift: 8 } = 0.78515625
    True8False25,  // Probability { prob: 193, shift: 8 } = 0.75390625
    True9False24,  // Probability { prob: 93, shift: 7 } = 0.7265625
    True10False23, // Probability { prob: 89, shift: 7 } = 0.6953125
    True11False22, // Probability { prob: 85, shift: 7 } = 0.6640625
    True12False21, // Probability { prob: 81, shift: 7 } = 0.6328125
    True13False20, // Probability { prob: 155, shift: 8 } = 0.60546875
    True14False19, // Probability { prob: 147, shift: 8 } = 0.57421875
    True15False18, // Probability { prob: 139, shift: 8 } = 0.54296875
    True16False17, // Probability { prob: 131, shift: 8 } = 0.51171875
    True17False16, // Probability { prob: 31, shift: 6 } = 0.484375
    True18False15, // Probability { prob: 29, shift: 6 } = 0.453125
    True19False14, // Probability { prob: 27, shift: 6 } = 0.421875
    True20False13, // Probability { prob: 25, shift: 6 } = 0.390625
    True21False12, // Probability { prob: 93, shift: 8 } = 0.36328125
    True22False11, // Probability { prob: 85, shift: 8 } = 0.33203125
    True23False10, // Probability { prob: 77, shift: 8 } = 0.30078125
    True24False9,  // Probability { prob: 69, shift: 8 } = 0.26953125
    True25False8,  // Probability { prob: 31, shift: 7 } = 0.2421875
    True26False7,  // Probability { prob: 27, shift: 7 } = 0.2109375
    True27False6,  // Probability { prob: 23, shift: 7 } = 0.1796875
    True28False5,  // Probability { prob: 19, shift: 7 } = 0.1484375
    True29False4,  // Probability { prob: 31, shift: 8 } = 0.12109375
    True30False3,  // Probability { prob: 23, shift: 8 } = 0.08984375
    True31False2,  // Probability { prob: 15, shift: 8 } = 0.05859375
    True32False1,  // Probability { prob: 7, shift: 8 } = 0.02734375
    True33False0,  // Probability { prob: 7, shift: 8 } = 0.02734375
    True0False34,  // Probability { prob: 31, shift: 5 } = 0.96875
    True1False33,  // Probability { prob: 31, shift: 5 } = 0.96875
    True2False32,  // Probability { prob: 15, shift: 4 } = 0.9375
    True3False31,  // Probability { prob: 233, shift: 8 } = 0.91015625
    True4False30,  // Probability { prob: 225, shift: 8 } = 0.87890625
    True5False29,  // Probability { prob: 109, shift: 7 } = 0.8515625
    True6False28,  // Probability { prob: 105, shift: 7 } = 0.8203125
    True7False27,  // Probability { prob: 203, shift: 8 } = 0.79296875
    True8False26,  // Probability { prob: 195, shift: 8 } = 0.76171875
    True9False25,  // Probability { prob: 47, shift: 6 } = 0.734375
    True10False24, // Probability { prob: 45, shift: 6 } = 0.703125
    True11False23, // Probability { prob: 173, shift: 8 } = 0.67578125
    True12False22, // Probability { prob: 165, shift: 8 } = 0.64453125
    True13False21, // Probability { prob: 79, shift: 7 } = 0.6171875
    True14False20, // Probability { prob: 75, shift: 7 } = 0.5859375
    True15False19, // Probability { prob: 143, shift: 8 } = 0.55859375
    True16False18, // Probability { prob: 135, shift: 8 } = 0.52734375
    True17False17, // Probability { prob: 2, shift: 2 } = 0.5
    True18False16, // Probability { prob: 15, shift: 5 } = 0.46875
    True19False15, // Probability { prob: 7, shift: 4 } = 0.4375
    True20False14, // Probability { prob: 105, shift: 8 } = 0.41015625
    True21False13, // Probability { prob: 97, shift: 8 } = 0.37890625
    True22False12, // Probability { prob: 45, shift: 7 } = 0.3515625
    True23False11, // Probability { prob: 41, shift: 7 } = 0.3203125
    True24False10, // Probability { prob: 75, shift: 8 } = 0.29296875
    True25False9,  // Probability { prob: 67, shift: 8 } = 0.26171875
    True26False8,  // Probability { prob: 15, shift: 6 } = 0.234375
    True27False7,  // Probability { prob: 13, shift: 6 } = 0.203125
    True28False6,  // Probability { prob: 45, shift: 8 } = 0.17578125
    True29False5,  // Probability { prob: 37, shift: 8 } = 0.14453125
    True30False4,  // Probability { prob: 15, shift: 7 } = 0.1171875
    True31False3,  // Probability { prob: 11, shift: 7 } = 0.0859375
    True32False2,  // Probability { prob: 15, shift: 8 } = 0.05859375
    True33False1,  // Probability { prob: 7, shift: 8 } = 0.02734375
    True34False0,  // Probability { prob: 7, shift: 8 } = 0.02734375
    True0False35,  // Probability { prob: 249, shift: 8 } = 0.97265625
    True1False34,  // Probability { prob: 31, shift: 5 } = 0.96875
    True2False33,  // Probability { prob: 241, shift: 8 } = 0.94140625
    True3False32,  // Probability { prob: 117, shift: 7 } = 0.9140625
    True4False31,  // Probability { prob: 113, shift: 7 } = 0.8828125
    True5False30,  // Probability { prob: 219, shift: 8 } = 0.85546875
    True6False29,  // Probability { prob: 53, shift: 6 } = 0.828125
    True7False28,  // Probability { prob: 51, shift: 6 } = 0.796875
    True8False27,  // Probability { prob: 197, shift: 8 } = 0.76953125
    True9False26,  // Probability { prob: 95, shift: 7 } = 0.7421875
    True10False25, // Probability { prob: 91, shift: 7 } = 0.7109375
    True11False24, // Probability { prob: 175, shift: 8 } = 0.68359375
    True12False23, // Probability { prob: 21, shift: 5 } = 0.65625
    True13False22, // Probability { prob: 5, shift: 3 } = 0.625
    True14False21, // Probability { prob: 153, shift: 8 } = 0.59765625
    True15False20, // Probability { prob: 73, shift: 7 } = 0.5703125
    True16False19, // Probability { prob: 69, shift: 7 } = 0.5390625
    True17False18, // Probability { prob: 131, shift: 8 } = 0.51171875
    True18False17, // Probability { prob: 31, shift: 6 } = 0.484375
    True19False16, // Probability { prob: 117, shift: 8 } = 0.45703125
    True20False15, // Probability { prob: 109, shift: 8 } = 0.42578125
    True21False14, // Probability { prob: 51, shift: 7 } = 0.3984375
    True22False13, // Probability { prob: 95, shift: 8 } = 0.37109375
    True23False12, // Probability { prob: 87, shift: 8 } = 0.33984375
    True24False11, // Probability { prob: 5, shift: 4 } = 0.3125
    True25False10, // Probability { prob: 73, shift: 8 } = 0.28515625
    True26False9,  // Probability { prob: 65, shift: 8 } = 0.25390625
    True27False8,  // Probability { prob: 29, shift: 7 } = 0.2265625
    True28False7,  // Probability { prob: 51, shift: 8 } = 0.19921875
    True29False6,  // Probability { prob: 43, shift: 8 } = 0.16796875
    True30False5,  // Probability { prob: 9, shift: 6 } = 0.140625
    True31False4,  // Probability { prob: 29, shift: 8 } = 0.11328125
    True32False3,  // Probability { prob: 21, shift: 8 } = 0.08203125
    True33False2,  // Probability { prob: 7, shift: 7 } = 0.0546875
    True34False1,  // Probability { prob: 7, shift: 8 } = 0.02734375
    True35False0,  // Probability { prob: 3, shift: 7 } = 0.0234375
    True0False36,  // Probability { prob: 249, shift: 8 } = 0.97265625
    True1False35,  // Probability { prob: 31, shift: 5 } = 0.96875
    True2False34,  // Probability { prob: 241, shift: 8 } = 0.94140625
    True3False33,  // Probability { prob: 117, shift: 7 } = 0.9140625
    True4False32,  // Probability { prob: 227, shift: 8 } = 0.88671875
    True5False31,  // Probability { prob: 55, shift: 6 } = 0.859375
    True6False30,  // Probability { prob: 213, shift: 8 } = 0.83203125
    True7False29,  // Probability { prob: 103, shift: 7 } = 0.8046875
    True8False28,  // Probability { prob: 199, shift: 8 } = 0.77734375
    True9False27,  // Probability { prob: 3, shift: 2 } = 0.75
    True10False26, // Probability { prob: 23, shift: 5 } = 0.71875
    True11False25, // Probability { prob: 177, shift: 8 } = 0.69140625
    True12False24, // Probability { prob: 85, shift: 7 } = 0.6640625
    True13False23, // Probability { prob: 163, shift: 8 } = 0.63671875
    True14False22, // Probability { prob: 39, shift: 6 } = 0.609375
    True15False21, // Probability { prob: 149, shift: 8 } = 0.58203125
    True16False20, // Probability { prob: 71, shift: 7 } = 0.5546875
    True17False19, // Probability { prob: 135, shift: 8 } = 0.52734375
    True18False18, // Probability { prob: 2, shift: 2 } = 0.5
    True19False17, // Probability { prob: 15, shift: 5 } = 0.46875
    True20False16, // Probability { prob: 113, shift: 8 } = 0.44140625
    True21False15, // Probability { prob: 53, shift: 7 } = 0.4140625
    True22False14, // Probability { prob: 99, shift: 8 } = 0.38671875
    True23False13, // Probability { prob: 23, shift: 6 } = 0.359375
    True24False12, // Probability { prob: 85, shift: 8 } = 0.33203125
    True25False11, // Probability { prob: 39, shift: 7 } = 0.3046875
    True26False10, // Probability { prob: 71, shift: 8 } = 0.27734375
    True27False9,  // Probability { prob: 1, shift: 2 } = 0.25
    True28False8,  // Probability { prob: 7, shift: 5 } = 0.21875
    True29False7,  // Probability { prob: 49, shift: 8 } = 0.19140625
    True30False6,  // Probability { prob: 21, shift: 7 } = 0.1640625
    True31False5,  // Probability { prob: 35, shift: 8 } = 0.13671875
    True32False4,  // Probability { prob: 7, shift: 6 } = 0.109375
    True33False3,  // Probability { prob: 21, shift: 8 } = 0.08203125
    True34False2,  // Probability { prob: 7, shift: 7 } = 0.0546875
    True35False1,  // Probability { prob: 7, shift: 8 } = 0.02734375
    True36False0,  // Probability { prob: 3, shift: 7 } = 0.0234375
    True0False37,  // Probability { prob: 249, shift: 8 } = 0.97265625
    True1False36,  // Probability { prob: 249, shift: 8 } = 0.97265625
    True2False35,  // Probability { prob: 121, shift: 7 } = 0.9453125
    True3False34,  // Probability { prob: 235, shift: 8 } = 0.91796875
    True4False33,  // Probability { prob: 57, shift: 6 } = 0.890625
    True5False32,  // Probability { prob: 221, shift: 8 } = 0.86328125
    True6False31,  // Probability { prob: 107, shift: 7 } = 0.8359375
    True7False30,  // Probability { prob: 207, shift: 8 } = 0.80859375
    True8False29,  // Probability { prob: 25, shift: 5 } = 0.78125
    True9False28,  // Probability { prob: 193, shift: 8 } = 0.75390625
    True10False27, // Probability { prob: 93, shift: 7 } = 0.7265625
    True11False26, // Probability { prob: 179, shift: 8 } = 0.69921875
    True12False25, // Probability { prob: 43, shift: 6 } = 0.671875
    True13False24, // Probability { prob: 83, shift: 7 } = 0.6484375
    True14False23, // Probability { prob: 159, shift: 8 } = 0.62109375
    True15False22, // Probability { prob: 19, shift: 5 } = 0.59375
    True16False21, // Probability { prob: 145, shift: 8 } = 0.56640625
    True17False20, // Probability { prob: 69, shift: 7 } = 0.5390625
    True18False19, // Probability { prob: 131, shift: 8 } = 0.51171875
    True19False18, // Probability { prob: 31, shift: 6 } = 0.484375
    True20False17, // Probability { prob: 117, shift: 8 } = 0.45703125
    True21False16, // Probability { prob: 55, shift: 7 } = 0.4296875
    True22False15, // Probability { prob: 103, shift: 8 } = 0.40234375
    True23False14, // Probability { prob: 3, shift: 3 } = 0.375
    True24False13, // Probability { prob: 89, shift: 8 } = 0.34765625
    True25False12, // Probability { prob: 83, shift: 8 } = 0.32421875
    True26False11, // Probability { prob: 19, shift: 6 } = 0.296875
    True27False10, // Probability { prob: 69, shift: 8 } = 0.26953125
    True28False9,  // Probability { prob: 31, shift: 7 } = 0.2421875
    True29False8,  // Probability { prob: 55, shift: 8 } = 0.21484375
    True30False7,  // Probability { prob: 3, shift: 4 } = 0.1875
    True31False6,  // Probability { prob: 41, shift: 8 } = 0.16015625
    True32False5,  // Probability { prob: 17, shift: 7 } = 0.1328125
    True33False4,  // Probability { prob: 27, shift: 8 } = 0.10546875
    True34False3,  // Probability { prob: 5, shift: 6 } = 0.078125
    True35False2,  // Probability { prob: 13, shift: 8 } = 0.05078125
    True36False1,  // Probability { prob: 3, shift: 7 } = 0.0234375
    True37False0,  // Probability { prob: 3, shift: 7 } = 0.0234375
    True0False38,  // Probability { prob: 249, shift: 8 } = 0.97265625
    True1False37,  // Probability { prob: 249, shift: 8 } = 0.97265625
    True2False36,  // Probability { prob: 121, shift: 7 } = 0.9453125
    True3False35,  // Probability { prob: 235, shift: 8 } = 0.91796875
    True4False34,  // Probability { prob: 229, shift: 8 } = 0.89453125
    True5False33,  // Probability { prob: 111, shift: 7 } = 0.8671875
    True6False32,  // Probability { prob: 215, shift: 8 } = 0.83984375
    True7False31,  // Probability { prob: 13, shift: 4 } = 0.8125
    True8False30,  // Probability { prob: 101, shift: 7 } = 0.7890625
    True9False29,  // Probability { prob: 195, shift: 8 } = 0.76171875
    True10False28, // Probability { prob: 47, shift: 6 } = 0.734375
    True11False27, // Probability { prob: 181, shift: 8 } = 0.70703125
    True12False26, // Probability { prob: 175, shift: 8 } = 0.68359375
    True13False25, // Probability { prob: 21, shift: 5 } = 0.65625
    True14False24, // Probability { prob: 161, shift: 8 } = 0.62890625
    True15False23, // Probability { prob: 77, shift: 7 } = 0.6015625
    True16False22, // Probability { prob: 37, shift: 6 } = 0.578125
    True17False21, // Probability { prob: 141, shift: 8 } = 0.55078125
    True18False20, // Probability { prob: 67, shift: 7 } = 0.5234375
    True19False19, // Probability { prob: 2, shift: 2 } = 0.5
    True20False18, // Probability { prob: 121, shift: 8 } = 0.47265625
    True21False17, // Probability { prob: 57, shift: 7 } = 0.4453125
    True22False16, // Probability { prob: 107, shift: 8 } = 0.41796875
    True23False15, // Probability { prob: 101, shift: 8 } = 0.39453125
    True24False14, // Probability { prob: 47, shift: 7 } = 0.3671875
    True25False13, // Probability { prob: 87, shift: 8 } = 0.33984375
    True26False12, // Probability { prob: 5, shift: 4 } = 0.3125
    True27False11, // Probability { prob: 37, shift: 7 } = 0.2890625
    True28False10, // Probability { prob: 67, shift: 8 } = 0.26171875
    True29False9,  // Probability { prob: 15, shift: 6 } = 0.234375
    True30False8,  // Probability { prob: 53, shift: 8 } = 0.20703125
    True31False7,  // Probability { prob: 47, shift: 8 } = 0.18359375
    True32False6,  // Probability { prob: 5, shift: 5 } = 0.15625
    True33False5,  // Probability { prob: 33, shift: 8 } = 0.12890625
    True34False4,  // Probability { prob: 13, shift: 7 } = 0.1015625
    True35False3,  // Probability { prob: 5, shift: 6 } = 0.078125
    True36False2,  // Probability { prob: 13, shift: 8 } = 0.05078125
    True37False1,  // Probability { prob: 3, shift: 7 } = 0.0234375
    True38False0,  // Probability { prob: 3, shift: 7 } = 0.0234375
    True0False39,  // Probability { prob: 249, shift: 8 } = 0.97265625
    True1False38,  // Probability { prob: 249, shift: 8 } = 0.97265625
    True2False37,  // Probability { prob: 121, shift: 7 } = 0.9453125
    True3False36,  // Probability { prob: 59, shift: 6 } = 0.921875
    True4False35,  // Probability { prob: 229, shift: 8 } = 0.89453125
    True5False34,  // Probability { prob: 223, shift: 8 } = 0.87109375
    True6False33,  // Probability { prob: 27, shift: 5 } = 0.84375
    True7False32,  // Probability { prob: 105, shift: 7 } = 0.8203125
    True8False31,  // Probability { prob: 203, shift: 8 } = 0.79296875
    True9False30,  // Probability { prob: 49, shift: 6 } = 0.765625
    True10False29, // Probability { prob: 95, shift: 7 } = 0.7421875
    True11False28, // Probability { prob: 183, shift: 8 } = 0.71484375
    True12False27, // Probability { prob: 177, shift: 8 } = 0.69140625
    True13False26, // Probability { prob: 85, shift: 7 } = 0.6640625
    True14False25, // Probability { prob: 41, shift: 6 } = 0.640625
    True15False24, // Probability { prob: 157, shift: 8 } = 0.61328125
    True16False23, // Probability { prob: 75, shift: 7 } = 0.5859375
    True17False22, // Probability { prob: 9, shift: 4 } = 0.5625
    True18False21, // Probability { prob: 137, shift: 8 } = 0.53515625
    True19False20, // Probability { prob: 131, shift: 8 } = 0.51171875
    True20False19, // Probability { prob: 31, shift: 6 } = 0.484375
    True21False18, // Probability { prob: 59, shift: 7 } = 0.4609375
    True22False17, // Probability { prob: 111, shift: 8 } = 0.43359375
    True23False16, // Probability { prob: 105, shift: 8 } = 0.41015625
    True24False15, // Probability { prob: 49, shift: 7 } = 0.3828125
    True25False14, // Probability { prob: 91, shift: 8 } = 0.35546875
    True26False13, // Probability { prob: 85, shift: 8 } = 0.33203125
    True27False12, // Probability { prob: 39, shift: 7 } = 0.3046875
    True28False11, // Probability { prob: 9, shift: 5 } = 0.28125
    True29False10, // Probability { prob: 65, shift: 8 } = 0.25390625
    True30False9,  // Probability { prob: 59, shift: 8 } = 0.23046875
    True31False8,  // Probability { prob: 13, shift: 6 } = 0.203125
    True32False7,  // Probability { prob: 45, shift: 8 } = 0.17578125
    True33False6,  // Probability { prob: 39, shift: 8 } = 0.15234375
    True34False5,  // Probability { prob: 1, shift: 3 } = 0.125
    True35False4,  // Probability { prob: 13, shift: 7 } = 0.1015625
    True36False3,  // Probability { prob: 19, shift: 8 } = 0.07421875
    True37False2,  // Probability { prob: 13, shift: 8 } = 0.05078125
    True38False1,  // Probability { prob: 3, shift: 7 } = 0.0234375
    True39False0,  // Probability { prob: 3, shift: 7 } = 0.0234375
    True0False40,  // Probability { prob: 249, shift: 8 } = 0.97265625
    True1False39,  // Probability { prob: 249, shift: 8 } = 0.97265625
    True2False38,  // Probability { prob: 243, shift: 8 } = 0.94921875
    True3False37,  // Probability { prob: 59, shift: 6 } = 0.921875
    True4False36,  // Probability { prob: 115, shift: 7 } = 0.8984375
    True5False35,  // Probability { prob: 7, shift: 3 } = 0.875
    True6False34,  // Probability { prob: 217, shift: 8 } = 0.84765625
    True7False33,  // Probability { prob: 211, shift: 8 } = 0.82421875
    True8False32,  // Probability { prob: 51, shift: 6 } = 0.796875
    True9False31,  // Probability { prob: 99, shift: 7 } = 0.7734375
    True10False30, // Probability { prob: 3, shift: 2 } = 0.75
    True11False29, // Probability { prob: 185, shift: 8 } = 0.72265625
    True12False28, // Probability { prob: 179, shift: 8 } = 0.69921875
    True13False27, // Probability { prob: 43, shift: 6 } = 0.671875
    True14False26, // Probability { prob: 83, shift: 7 } = 0.6484375
    True15False25, // Probability { prob: 5, shift: 3 } = 0.625
    True16False24, // Probability { prob: 153, shift: 8 } = 0.59765625
    True17False23, // Probability { prob: 147, shift: 8 } = 0.57421875
    True18False22, // Probability { prob: 35, shift: 6 } = 0.546875
    True19False21, // Probability { prob: 67, shift: 7 } = 0.5234375
    True20False20, // Probability { prob: 2, shift: 2 } = 0.5
    True21False19, // Probability { prob: 121, shift: 8 } = 0.47265625
    True22False18, // Probability { prob: 115, shift: 8 } = 0.44921875
    True23False17, // Probability { prob: 27, shift: 6 } = 0.421875
    True24False16, // Probability { prob: 51, shift: 7 } = 0.3984375
    True25False15, // Probability { prob: 3, shift: 3 } = 0.375
    True26False14, // Probability { prob: 89, shift: 8 } = 0.34765625
    True27False13, // Probability { prob: 83, shift: 8 } = 0.32421875
    True28False12, // Probability { prob: 19, shift: 6 } = 0.296875
    True29False11, // Probability { prob: 35, shift: 7 } = 0.2734375
    True30False10, // Probability { prob: 1, shift: 2 } = 0.25
    True31False9,  // Probability { prob: 57, shift: 8 } = 0.22265625
    True32False8,  // Probability { prob: 51, shift: 8 } = 0.19921875
    True33False7,  // Probability { prob: 11, shift: 6 } = 0.171875
    True34False6,  // Probability { prob: 19, shift: 7 } = 0.1484375
    True35False5,  // Probability { prob: 1, shift: 3 } = 0.125
    True36False4,  // Probability { prob: 25, shift: 8 } = 0.09765625
    True37False3,  // Probability { prob: 19, shift: 8 } = 0.07421875
    True38False2,  // Probability { prob: 3, shift: 6 } = 0.046875
    True39False1,  // Probability { prob: 3, shift: 7 } = 0.0234375
    True40False0,  // Probability { prob: 3, shift: 7 } = 0.0234375
    True0False41,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True1False40,  // Probability { prob: 249, shift: 8 } = 0.97265625
    True2False39,  // Probability { prob: 243, shift: 8 } = 0.94921875
    True3False38,  // Probability { prob: 237, shift: 8 } = 0.92578125
    True4False37,  // Probability { prob: 231, shift: 8 } = 0.90234375
    True5False36,  // Probability { prob: 7, shift: 3 } = 0.875
    True6False35,  // Probability { prob: 109, shift: 7 } = 0.8515625
    True7False34,  // Probability { prob: 53, shift: 6 } = 0.828125
    True8False33,  // Probability { prob: 103, shift: 7 } = 0.8046875
    True9False32,  // Probability { prob: 199, shift: 8 } = 0.77734375
    True10False31, // Probability { prob: 193, shift: 8 } = 0.75390625
    True11False30, // Probability { prob: 187, shift: 8 } = 0.73046875
    True12False29, // Probability { prob: 181, shift: 8 } = 0.70703125
    True13False28, // Probability { prob: 87, shift: 7 } = 0.6796875
    True14False27, // Probability { prob: 21, shift: 5 } = 0.65625
    True15False26, // Probability { prob: 81, shift: 7 } = 0.6328125
    True16False25, // Probability { prob: 39, shift: 6 } = 0.609375
    True17False24, // Probability { prob: 149, shift: 8 } = 0.58203125
    True18False23, // Probability { prob: 143, shift: 8 } = 0.55859375
    True19False22, // Probability { prob: 137, shift: 8 } = 0.53515625
    True20False21, // Probability { prob: 131, shift: 8 } = 0.51171875
    True21False20, // Probability { prob: 31, shift: 6 } = 0.484375
    True22False19, // Probability { prob: 59, shift: 7 } = 0.4609375
    True23False18, // Probability { prob: 7, shift: 4 } = 0.4375
    True24False17, // Probability { prob: 53, shift: 7 } = 0.4140625
    True25False16, // Probability { prob: 99, shift: 8 } = 0.38671875
    True26False15, // Probability { prob: 93, shift: 8 } = 0.36328125
    True27False14, // Probability { prob: 87, shift: 8 } = 0.33984375
    True28False13, // Probability { prob: 81, shift: 8 } = 0.31640625
    True29False12, // Probability { prob: 37, shift: 7 } = 0.2890625
    True30False11, // Probability { prob: 17, shift: 6 } = 0.265625
    True31False10, // Probability { prob: 31, shift: 7 } = 0.2421875
    True32False9,  // Probability { prob: 7, shift: 5 } = 0.21875
    True33False8,  // Probability { prob: 49, shift: 8 } = 0.19140625
    True34False7,  // Probability { prob: 43, shift: 8 } = 0.16796875
    True35False6,  // Probability { prob: 37, shift: 8 } = 0.14453125
    True36False5,  // Probability { prob: 31, shift: 8 } = 0.12109375
    True37False4,  // Probability { prob: 3, shift: 5 } = 0.09375
    True38False3,  // Probability { prob: 9, shift: 7 } = 0.0703125
    True39False2,  // Probability { prob: 3, shift: 6 } = 0.046875
    True40False1,  // Probability { prob: 3, shift: 7 } = 0.0234375
    True41False0,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True0False42,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True1False41,  // Probability { prob: 249, shift: 8 } = 0.97265625
    True2False40,  // Probability { prob: 243, shift: 8 } = 0.94921875
    True3False39,  // Probability { prob: 237, shift: 8 } = 0.92578125
    True4False38,  // Probability { prob: 231, shift: 8 } = 0.90234375
    True5False37,  // Probability { prob: 225, shift: 8 } = 0.87890625
    True6False36,  // Probability { prob: 219, shift: 8 } = 0.85546875
    True7False35,  // Probability { prob: 213, shift: 8 } = 0.83203125
    True8False34,  // Probability { prob: 207, shift: 8 } = 0.80859375
    True9False33,  // Probability { prob: 201, shift: 8 } = 0.78515625
    True10False32, // Probability { prob: 195, shift: 8 } = 0.76171875
    True11False31, // Probability { prob: 47, shift: 6 } = 0.734375
    True12False30, // Probability { prob: 91, shift: 7 } = 0.7109375
    True13False29, // Probability { prob: 11, shift: 4 } = 0.6875
    True14False28, // Probability { prob: 85, shift: 7 } = 0.6640625
    True15False27, // Probability { prob: 41, shift: 6 } = 0.640625
    True16False26, // Probability { prob: 79, shift: 7 } = 0.6171875
    True17False25, // Probability { prob: 19, shift: 5 } = 0.59375
    True18False24, // Probability { prob: 73, shift: 7 } = 0.5703125
    True19False23, // Probability { prob: 35, shift: 6 } = 0.546875
    True20False22, // Probability { prob: 67, shift: 7 } = 0.5234375
    True21False21, // Probability { prob: 2, shift: 2 } = 0.5
    True22False20, // Probability { prob: 121, shift: 8 } = 0.47265625
    True23False19, // Probability { prob: 115, shift: 8 } = 0.44921875
    True24False18, // Probability { prob: 109, shift: 8 } = 0.42578125
    True25False17, // Probability { prob: 103, shift: 8 } = 0.40234375
    True26False16, // Probability { prob: 97, shift: 8 } = 0.37890625
    True27False15, // Probability { prob: 91, shift: 8 } = 0.35546875
    True28False14, // Probability { prob: 85, shift: 8 } = 0.33203125
    True29False13, // Probability { prob: 79, shift: 8 } = 0.30859375
    True30False12, // Probability { prob: 73, shift: 8 } = 0.28515625
    True31False11, // Probability { prob: 67, shift: 8 } = 0.26171875
    True32False10, // Probability { prob: 15, shift: 6 } = 0.234375
    True33False9,  // Probability { prob: 27, shift: 7 } = 0.2109375
    True34False8,  // Probability { prob: 3, shift: 4 } = 0.1875
    True35False7,  // Probability { prob: 21, shift: 7 } = 0.1640625
    True36False6,  // Probability { prob: 9, shift: 6 } = 0.140625
    True37False5,  // Probability { prob: 15, shift: 7 } = 0.1171875
    True38False4,  // Probability { prob: 3, shift: 5 } = 0.09375
    True39False3,  // Probability { prob: 9, shift: 7 } = 0.0703125
    True40False2,  // Probability { prob: 3, shift: 6 } = 0.046875
    True41False1,  // Probability { prob: 3, shift: 7 } = 0.0234375
    True42False0,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True0False43,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True1False42,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True2False41,  // Probability { prob: 61, shift: 6 } = 0.953125
    True3False40,  // Probability { prob: 119, shift: 7 } = 0.9296875
    True4False39,  // Probability { prob: 29, shift: 5 } = 0.90625
    True5False38,  // Probability { prob: 113, shift: 7 } = 0.8828125
    True6False37,  // Probability { prob: 55, shift: 6 } = 0.859375
    True7False36,  // Probability { prob: 107, shift: 7 } = 0.8359375
    True8False35,  // Probability { prob: 13, shift: 4 } = 0.8125
    True9False34,  // Probability { prob: 101, shift: 7 } = 0.7890625
    True10False33, // Probability { prob: 49, shift: 6 } = 0.765625
    True11False32, // Probability { prob: 95, shift: 7 } = 0.7421875
    True12False31, // Probability { prob: 23, shift: 5 } = 0.71875
    True13False30, // Probability { prob: 89, shift: 7 } = 0.6953125
    True14False29, // Probability { prob: 43, shift: 6 } = 0.671875
    True15False28, // Probability { prob: 83, shift: 7 } = 0.6484375
    True16False27, // Probability { prob: 5, shift: 3 } = 0.625
    True17False26, // Probability { prob: 77, shift: 7 } = 0.6015625
    True18False25, // Probability { prob: 37, shift: 6 } = 0.578125
    True19False24, // Probability { prob: 71, shift: 7 } = 0.5546875
    True20False23, // Probability { prob: 17, shift: 5 } = 0.53125
    True21False22, // Probability { prob: 65, shift: 7 } = 0.5078125
    True22False21, // Probability { prob: 125, shift: 8 } = 0.48828125
    True23False20, // Probability { prob: 119, shift: 8 } = 0.46484375
    True24False19, // Probability { prob: 113, shift: 8 } = 0.44140625
    True25False18, // Probability { prob: 107, shift: 8 } = 0.41796875
    True26False17, // Probability { prob: 101, shift: 8 } = 0.39453125
    True27False16, // Probability { prob: 95, shift: 8 } = 0.37109375
    True28False15, // Probability { prob: 89, shift: 8 } = 0.34765625
    True29False14, // Probability { prob: 83, shift: 8 } = 0.32421875
    True30False13, // Probability { prob: 77, shift: 8 } = 0.30078125
    True31False12, // Probability { prob: 71, shift: 8 } = 0.27734375
    True32False11, // Probability { prob: 65, shift: 8 } = 0.25390625
    True33False10, // Probability { prob: 59, shift: 8 } = 0.23046875
    True34False9,  // Probability { prob: 53, shift: 8 } = 0.20703125
    True35False8,  // Probability { prob: 47, shift: 8 } = 0.18359375
    True36False7,  // Probability { prob: 41, shift: 8 } = 0.16015625
    True37False6,  // Probability { prob: 35, shift: 8 } = 0.13671875
    True38False5,  // Probability { prob: 29, shift: 8 } = 0.11328125
    True39False4,  // Probability { prob: 23, shift: 8 } = 0.08984375
    True40False3,  // Probability { prob: 17, shift: 8 } = 0.06640625
    True41False2,  // Probability { prob: 11, shift: 8 } = 0.04296875
    True42False1,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True43False0,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True0False44,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True1False43,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True2False42,  // Probability { prob: 61, shift: 6 } = 0.953125
    True3False41,  // Probability { prob: 119, shift: 7 } = 0.9296875
    True4False40,  // Probability { prob: 29, shift: 5 } = 0.90625
    True5False39,  // Probability { prob: 113, shift: 7 } = 0.8828125
    True6False38,  // Probability { prob: 221, shift: 8 } = 0.86328125
    True7False37,  // Probability { prob: 215, shift: 8 } = 0.83984375
    True8False36,  // Probability { prob: 209, shift: 8 } = 0.81640625
    True9False35,  // Probability { prob: 203, shift: 8 } = 0.79296875
    True10False34, // Probability { prob: 197, shift: 8 } = 0.76953125
    True11False33, // Probability { prob: 3, shift: 2 } = 0.75
    True12False32, // Probability { prob: 93, shift: 7 } = 0.7265625
    True13False31, // Probability { prob: 45, shift: 6 } = 0.703125
    True14False30, // Probability { prob: 87, shift: 7 } = 0.6796875
    True15False29, // Probability { prob: 21, shift: 5 } = 0.65625
    True16False28, // Probability { prob: 81, shift: 7 } = 0.6328125
    True17False27, // Probability { prob: 157, shift: 8 } = 0.61328125
    True18False26, // Probability { prob: 151, shift: 8 } = 0.58984375
    True19False25, // Probability { prob: 145, shift: 8 } = 0.56640625
    True20False24, // Probability { prob: 139, shift: 8 } = 0.54296875
    True21False23, // Probability { prob: 133, shift: 8 } = 0.51953125
    True22False22, // Probability { prob: 2, shift: 2 } = 0.5
    True23False21, // Probability { prob: 61, shift: 7 } = 0.4765625
    True24False20, // Probability { prob: 29, shift: 6 } = 0.453125
    True25False19, // Probability { prob: 55, shift: 7 } = 0.4296875
    True26False18, // Probability { prob: 13, shift: 5 } = 0.40625
    True27False17, // Probability { prob: 49, shift: 7 } = 0.3828125
    True28False16, // Probability { prob: 93, shift: 8 } = 0.36328125
    True29False15, // Probability { prob: 87, shift: 8 } = 0.33984375
    True30False14, // Probability { prob: 81, shift: 8 } = 0.31640625
    True31False13, // Probability { prob: 75, shift: 8 } = 0.29296875
    True32False12, // Probability { prob: 69, shift: 8 } = 0.26953125
    True33False11, // Probability { prob: 1, shift: 2 } = 0.25
    True34False10, // Probability { prob: 29, shift: 7 } = 0.2265625
    True35False9,  // Probability { prob: 13, shift: 6 } = 0.203125
    True36False8,  // Probability { prob: 23, shift: 7 } = 0.1796875
    True37False7,  // Probability { prob: 5, shift: 5 } = 0.15625
    True38False6,  // Probability { prob: 17, shift: 7 } = 0.1328125
    True39False5,  // Probability { prob: 29, shift: 8 } = 0.11328125
    True40False4,  // Probability { prob: 23, shift: 8 } = 0.08984375
    True41False3,  // Probability { prob: 17, shift: 8 } = 0.06640625
    True42False2,  // Probability { prob: 11, shift: 8 } = 0.04296875
    True43False1,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True44False0,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True0False45,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True1False44,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True2False43,  // Probability { prob: 61, shift: 6 } = 0.953125
    True3False42,  // Probability { prob: 119, shift: 7 } = 0.9296875
    True4False41,  // Probability { prob: 233, shift: 8 } = 0.91015625
    True5False40,  // Probability { prob: 227, shift: 8 } = 0.88671875
    True6False39,  // Probability { prob: 221, shift: 8 } = 0.86328125
    True7False38,  // Probability { prob: 27, shift: 5 } = 0.84375
    True8False37,  // Probability { prob: 105, shift: 7 } = 0.8203125
    True9False36,  // Probability { prob: 51, shift: 6 } = 0.796875
    True10False35, // Probability { prob: 199, shift: 8 } = 0.77734375
    True11False34, // Probability { prob: 193, shift: 8 } = 0.75390625
    True12False33, // Probability { prob: 187, shift: 8 } = 0.73046875
    True13False32, // Probability { prob: 91, shift: 7 } = 0.7109375
    True14False31, // Probability { prob: 11, shift: 4 } = 0.6875
    True15False30, // Probability { prob: 85, shift: 7 } = 0.6640625
    True16False29, // Probability { prob: 41, shift: 6 } = 0.640625
    True17False28, // Probability { prob: 159, shift: 8 } = 0.62109375
    True18False27, // Probability { prob: 153, shift: 8 } = 0.59765625
    True19False26, // Probability { prob: 147, shift: 8 } = 0.57421875
    True20False25, // Probability { prob: 71, shift: 7 } = 0.5546875
    True21False24, // Probability { prob: 17, shift: 5 } = 0.53125
    True22False23, // Probability { prob: 65, shift: 7 } = 0.5078125
    True23False22, // Probability { prob: 125, shift: 8 } = 0.48828125
    True24False21, // Probability { prob: 119, shift: 8 } = 0.46484375
    True25False20, // Probability { prob: 113, shift: 8 } = 0.44140625
    True26False19, // Probability { prob: 27, shift: 6 } = 0.421875
    True27False18, // Probability { prob: 51, shift: 7 } = 0.3984375
    True28False17, // Probability { prob: 3, shift: 3 } = 0.375
    True29False16, // Probability { prob: 91, shift: 8 } = 0.35546875
    True30False15, // Probability { prob: 85, shift: 8 } = 0.33203125
    True31False14, // Probability { prob: 79, shift: 8 } = 0.30859375
    True32False13, // Probability { prob: 73, shift: 8 } = 0.28515625
    True33False12, // Probability { prob: 17, shift: 6 } = 0.265625
    True34False11, // Probability { prob: 31, shift: 7 } = 0.2421875
    True35False10, // Probability { prob: 7, shift: 5 } = 0.21875
    True36False9,  // Probability { prob: 51, shift: 8 } = 0.19921875
    True37False8,  // Probability { prob: 45, shift: 8 } = 0.17578125
    True38False7,  // Probability { prob: 39, shift: 8 } = 0.15234375
    True39False6,  // Probability { prob: 17, shift: 7 } = 0.1328125
    True40False5,  // Probability { prob: 7, shift: 6 } = 0.109375
    True41False4,  // Probability { prob: 11, shift: 7 } = 0.0859375
    True42False3,  // Probability { prob: 17, shift: 8 } = 0.06640625
    True43False2,  // Probability { prob: 11, shift: 8 } = 0.04296875
    True44False1,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True45False0,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True0False46,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True1False45,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True2False44,  // Probability { prob: 61, shift: 6 } = 0.953125
    True3False43,  // Probability { prob: 239, shift: 8 } = 0.93359375
    True4False42,  // Probability { prob: 233, shift: 8 } = 0.91015625
    True5False41,  // Probability { prob: 57, shift: 6 } = 0.890625
    True6False40,  // Probability { prob: 111, shift: 7 } = 0.8671875
    True7False39,  // Probability { prob: 217, shift: 8 } = 0.84765625
    True8False38,  // Probability { prob: 211, shift: 8 } = 0.82421875
    True9False37,  // Probability { prob: 205, shift: 8 } = 0.80078125
    True10False36, // Probability { prob: 25, shift: 5 } = 0.78125
    True11False35, // Probability { prob: 97, shift: 7 } = 0.7578125
    True12False34, // Probability { prob: 189, shift: 8 } = 0.73828125
    True13False33, // Probability { prob: 183, shift: 8 } = 0.71484375
    True14False32, // Probability { prob: 89, shift: 7 } = 0.6953125
    True15False31, // Probability { prob: 43, shift: 6 } = 0.671875
    True16False30, // Probability { prob: 83, shift: 7 } = 0.6484375
    True17False29, // Probability { prob: 161, shift: 8 } = 0.62890625
    True18False28, // Probability { prob: 155, shift: 8 } = 0.60546875
    True19False27, // Probability { prob: 75, shift: 7 } = 0.5859375
    True20False26, // Probability { prob: 9, shift: 4 } = 0.5625
    True21False25, // Probability { prob: 139, shift: 8 } = 0.54296875
    True22False24, // Probability { prob: 133, shift: 8 } = 0.51953125
    True23False23, // Probability { prob: 2, shift: 2 } = 0.5
    True24False22, // Probability { prob: 61, shift: 7 } = 0.4765625
    True25False21, // Probability { prob: 29, shift: 6 } = 0.453125
    True26False20, // Probability { prob: 111, shift: 8 } = 0.43359375
    True27False19, // Probability { prob: 105, shift: 8 } = 0.41015625
    True28False18, // Probability { prob: 25, shift: 6 } = 0.390625
    True29False17, // Probability { prob: 47, shift: 7 } = 0.3671875
    True30False16, // Probability { prob: 89, shift: 8 } = 0.34765625
    True31False15, // Probability { prob: 83, shift: 8 } = 0.32421875
    True32False14, // Probability { prob: 77, shift: 8 } = 0.30078125
    True33False13, // Probability { prob: 9, shift: 5 } = 0.28125
    True34False12, // Probability { prob: 33, shift: 7 } = 0.2578125
    True35False11, // Probability { prob: 61, shift: 8 } = 0.23828125
    True36False10, // Probability { prob: 55, shift: 8 } = 0.21484375
    True37False9,  // Probability { prob: 25, shift: 7 } = 0.1953125
    True38False8,  // Probability { prob: 11, shift: 6 } = 0.171875
    True39False7,  // Probability { prob: 19, shift: 7 } = 0.1484375
    True40False6,  // Probability { prob: 33, shift: 8 } = 0.12890625
    True41False5,  // Probability { prob: 27, shift: 8 } = 0.10546875
    True42False4,  // Probability { prob: 11, shift: 7 } = 0.0859375
    True43False3,  // Probability { prob: 1, shift: 4 } = 0.0625
    True44False2,  // Probability { prob: 11, shift: 8 } = 0.04296875
    True45False1,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True46False0,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True0False47,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True1False46,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True2False45,  // Probability { prob: 245, shift: 8 } = 0.95703125
    True3False44,  // Probability { prob: 239, shift: 8 } = 0.93359375
    True4False43,  // Probability { prob: 117, shift: 7 } = 0.9140625
    True5False42,  // Probability { prob: 57, shift: 6 } = 0.890625
    True6False41,  // Probability { prob: 223, shift: 8 } = 0.87109375
    True7False40,  // Probability { prob: 217, shift: 8 } = 0.84765625
    True8False39,  // Probability { prob: 53, shift: 6 } = 0.828125
    True9False38,  // Probability { prob: 103, shift: 7 } = 0.8046875
    True10False37, // Probability { prob: 201, shift: 8 } = 0.78515625
    True11False36, // Probability { prob: 49, shift: 6 } = 0.765625
    True12False35, // Probability { prob: 95, shift: 7 } = 0.7421875
    True13False34, // Probability { prob: 185, shift: 8 } = 0.72265625
    True14False33, // Probability { prob: 179, shift: 8 } = 0.69921875
    True15False32, // Probability { prob: 87, shift: 7 } = 0.6796875
    True16False31, // Probability { prob: 21, shift: 5 } = 0.65625
    True17False30, // Probability { prob: 163, shift: 8 } = 0.63671875
    True18False29, // Probability { prob: 157, shift: 8 } = 0.61328125
    True19False28, // Probability { prob: 19, shift: 5 } = 0.59375
    True20False27, // Probability { prob: 147, shift: 8 } = 0.57421875
    True21False26, // Probability { prob: 141, shift: 8 } = 0.55078125
    True22False25, // Probability { prob: 17, shift: 5 } = 0.53125
    True23False24, // Probability { prob: 65, shift: 7 } = 0.5078125
    True24False23, // Probability { prob: 125, shift: 8 } = 0.48828125
    True25False22, // Probability { prob: 119, shift: 8 } = 0.46484375
    True26False21, // Probability { prob: 57, shift: 7 } = 0.4453125
    True27False20, // Probability { prob: 27, shift: 6 } = 0.421875
    True28False19, // Probability { prob: 103, shift: 8 } = 0.40234375
    True29False18, // Probability { prob: 49, shift: 7 } = 0.3828125
    True30False17, // Probability { prob: 23, shift: 6 } = 0.359375
    True31False16, // Probability { prob: 87, shift: 8 } = 0.33984375
    True32False15, // Probability { prob: 81, shift: 8 } = 0.31640625
    True33False14, // Probability { prob: 19, shift: 6 } = 0.296875
    True34False13, // Probability { prob: 35, shift: 7 } = 0.2734375
    True35False12, // Probability { prob: 65, shift: 8 } = 0.25390625
    True36False11, // Probability { prob: 59, shift: 8 } = 0.23046875
    True37False10, // Probability { prob: 27, shift: 7 } = 0.2109375
    True38False9,  // Probability { prob: 49, shift: 8 } = 0.19140625
    True39False8,  // Probability { prob: 43, shift: 8 } = 0.16796875
    True40False7,  // Probability { prob: 19, shift: 7 } = 0.1484375
    True41False6,  // Probability { prob: 1, shift: 3 } = 0.125
    True42False5,  // Probability { prob: 27, shift: 8 } = 0.10546875
    True43False4,  // Probability { prob: 21, shift: 8 } = 0.08203125
    True44False3,  // Probability { prob: 1, shift: 4 } = 0.0625
    True45False2,  // Probability { prob: 5, shift: 7 } = 0.0390625
    True46False1,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True47False0,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True0False48,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True1False47,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True2False46,  // Probability { prob: 245, shift: 8 } = 0.95703125
    True3False45,  // Probability { prob: 15, shift: 4 } = 0.9375
    True4False44,  // Probability { prob: 117, shift: 7 } = 0.9140625
    True5False43,  // Probability { prob: 229, shift: 8 } = 0.89453125
    True6False42,  // Probability { prob: 7, shift: 3 } = 0.875
    True7False41,  // Probability { prob: 109, shift: 7 } = 0.8515625
    True8False40,  // Probability { prob: 213, shift: 8 } = 0.83203125
    True9False39,  // Probability { prob: 13, shift: 4 } = 0.8125
    True10False38, // Probability { prob: 101, shift: 7 } = 0.7890625
    True11False37, // Probability { prob: 197, shift: 8 } = 0.76953125
    True12False36, // Probability { prob: 3, shift: 2 } = 0.75
    True13False35, // Probability { prob: 93, shift: 7 } = 0.7265625
    True14False34, // Probability { prob: 181, shift: 8 } = 0.70703125
    True15False33, // Probability { prob: 11, shift: 4 } = 0.6875
    True16False32, // Probability { prob: 85, shift: 7 } = 0.6640625
    True17False31, // Probability { prob: 165, shift: 8 } = 0.64453125
    True18False30, // Probability { prob: 5, shift: 3 } = 0.625
    True19False29, // Probability { prob: 77, shift: 7 } = 0.6015625
    True20False28, // Probability { prob: 149, shift: 8 } = 0.58203125
    True21False27, // Probability { prob: 9, shift: 4 } = 0.5625
    True22False26, // Probability { prob: 69, shift: 7 } = 0.5390625
    True23False25, // Probability { prob: 133, shift: 8 } = 0.51953125
    True24False24, // Probability { prob: 2, shift: 2 } = 0.5
    True25False23, // Probability { prob: 61, shift: 7 } = 0.4765625
    True26False22, // Probability { prob: 117, shift: 8 } = 0.45703125
    True27False21, // Probability { prob: 7, shift: 4 } = 0.4375
    True28False20, // Probability { prob: 53, shift: 7 } = 0.4140625
    True29False19, // Probability { prob: 101, shift: 8 } = 0.39453125
    True30False18, // Probability { prob: 3, shift: 3 } = 0.375
    True31False17, // Probability { prob: 45, shift: 7 } = 0.3515625
    True32False16, // Probability { prob: 85, shift: 8 } = 0.33203125
    True33False15, // Probability { prob: 5, shift: 4 } = 0.3125
    True34False14, // Probability { prob: 37, shift: 7 } = 0.2890625
    True35False13, // Probability { prob: 69, shift: 8 } = 0.26953125
    True36False12, // Probability { prob: 1, shift: 2 } = 0.25
    True37False11, // Probability { prob: 29, shift: 7 } = 0.2265625
    True38False10, // Probability { prob: 53, shift: 8 } = 0.20703125
    True39False9,  // Probability { prob: 3, shift: 4 } = 0.1875
    True40False8,  // Probability { prob: 21, shift: 7 } = 0.1640625
    True41False7,  // Probability { prob: 37, shift: 8 } = 0.14453125
    True42False6,  // Probability { prob: 1, shift: 3 } = 0.125
    True43False5,  // Probability { prob: 13, shift: 7 } = 0.1015625
    True44False4,  // Probability { prob: 21, shift: 8 } = 0.08203125
    True45False3,  // Probability { prob: 1, shift: 4 } = 0.0625
    True46False2,  // Probability { prob: 5, shift: 7 } = 0.0390625
    True47False1,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True48False0,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True0False49,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True1False48,  // Probability { prob: 125, shift: 7 } = 0.9765625
    True2False47,  // Probability { prob: 245, shift: 8 } = 0.95703125
    True3False46,  // Probability { prob: 15, shift: 4 } = 0.9375
    True4False45,  // Probability { prob: 235, shift: 8 } = 0.91796875
    True5False44,  // Probability { prob: 229, shift: 8 } = 0.89453125
    True6False43,  // Probability { prob: 7, shift: 3 } = 0.875
    True7False42,  // Probability { prob: 219, shift: 8 } = 0.85546875
    True8False41,  // Probability { prob: 107, shift: 7 } = 0.8359375
    True9False40,  // Probability { prob: 13, shift: 4 } = 0.8125
    True10False39, // Probability { prob: 203, shift: 8 } = 0.79296875
    True11False38, // Probability { prob: 99, shift: 7 } = 0.7734375
    True12False37, // Probability { prob: 193, shift: 8 } = 0.75390625
    True13False36, // Probability { prob: 47, shift: 6 } = 0.734375
    True14False35, // Probability { prob: 91, shift: 7 } = 0.7109375
    True15False34, // Probability { prob: 177, shift: 8 } = 0.69140625
    True16False33, // Probability { prob: 43, shift: 6 } = 0.671875
    True17False32, // Probability { prob: 167, shift: 8 } = 0.65234375
    True18False31, // Probability { prob: 161, shift: 8 } = 0.62890625
    True19False30, // Probability { prob: 39, shift: 6 } = 0.609375
    True20False29, // Probability { prob: 151, shift: 8 } = 0.58984375
    True21False28, // Probability { prob: 73, shift: 7 } = 0.5703125
    True22False27, // Probability { prob: 141, shift: 8 } = 0.55078125
    True23False26, // Probability { prob: 135, shift: 8 } = 0.52734375
    True24False25, // Probability { prob: 65, shift: 7 } = 0.5078125
    True25False24, // Probability { prob: 125, shift: 8 } = 0.48828125
    True26False23, // Probability { prob: 15, shift: 5 } = 0.46875
    True27False22, // Probability { prob: 57, shift: 7 } = 0.4453125
    True28False21, // Probability { prob: 109, shift: 8 } = 0.42578125
    True29False20, // Probability { prob: 13, shift: 5 } = 0.40625
    True30False19, // Probability { prob: 99, shift: 8 } = 0.38671875
    True31False18, // Probability { prob: 47, shift: 7 } = 0.3671875
    True32False17, // Probability { prob: 11, shift: 5 } = 0.34375
    True33False16, // Probability { prob: 83, shift: 8 } = 0.32421875
    True34False15, // Probability { prob: 39, shift: 7 } = 0.3046875
    True35False14, // Probability { prob: 73, shift: 8 } = 0.28515625
    True36False13, // Probability { prob: 67, shift: 8 } = 0.26171875
    True37False12, // Probability { prob: 31, shift: 7 } = 0.2421875
    True38False11, // Probability { prob: 57, shift: 8 } = 0.22265625
    True39False10, // Probability { prob: 13, shift: 6 } = 0.203125
    True40False9,  // Probability { prob: 47, shift: 8 } = 0.18359375
    True41False8,  // Probability { prob: 41, shift: 8 } = 0.16015625
    True42False7,  // Probability { prob: 9, shift: 6 } = 0.140625
    True43False6,  // Probability { prob: 31, shift: 8 } = 0.12109375
    True44False5,  // Probability { prob: 13, shift: 7 } = 0.1015625
    True45False4,  // Probability { prob: 5, shift: 6 } = 0.078125
    True46False3,  // Probability { prob: 15, shift: 8 } = 0.05859375
    True47False2,  // Probability { prob: 5, shift: 7 } = 0.0390625
    True48False1,  // Probability { prob: 5, shift: 8 } = 0.01953125
    True49False0,  // Probability { prob: 5, shift: 8 } = 0.01953125
}
use BitContext::*;

impl BitContext {
    #[inline]
    pub fn probability(self) -> Probability {
        const LOOKUP: [Probability; 1275] = [
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
            Probability { prob: 61, shift: 6 },
            Probability {
                prob: 243,
                shift: 8,
            },
            Probability {
                prob: 231,
                shift: 8,
            },
            Probability {
                prob: 219,
                shift: 8,
            },
            Probability {
                prob: 207,
                shift: 8,
            },
            Probability {
                prob: 195,
                shift: 8,
            },
            Probability { prob: 91, shift: 7 },
            Probability { prob: 85, shift: 7 },
            Probability { prob: 79, shift: 7 },
            Probability { prob: 73, shift: 7 },
            Probability { prob: 67, shift: 7 },
            Probability {
                prob: 121,
                shift: 8,
            },
            Probability {
                prob: 109,
                shift: 8,
            },
            Probability { prob: 97, shift: 8 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 73, shift: 8 },
            Probability { prob: 15, shift: 6 },
            Probability { prob: 3, shift: 4 },
            Probability { prob: 9, shift: 6 },
            Probability { prob: 3, shift: 5 },
            Probability { prob: 3, shift: 6 },
            Probability { prob: 11, shift: 8 },
            Probability {
                prob: 245,
                shift: 8,
            },
            Probability { prob: 61, shift: 6 },
            Probability { prob: 29, shift: 5 },
            Probability {
                prob: 221,
                shift: 8,
            },
            Probability {
                prob: 209,
                shift: 8,
            },
            Probability {
                prob: 197,
                shift: 8,
            },
            Probability { prob: 93, shift: 7 },
            Probability { prob: 87, shift: 7 },
            Probability { prob: 81, shift: 7 },
            Probability {
                prob: 151,
                shift: 8,
            },
            Probability {
                prob: 139,
                shift: 8,
            },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 29, shift: 6 },
            Probability { prob: 13, shift: 5 },
            Probability { prob: 93, shift: 8 },
            Probability { prob: 81, shift: 8 },
            Probability { prob: 69, shift: 8 },
            Probability { prob: 29, shift: 7 },
            Probability { prob: 23, shift: 7 },
            Probability { prob: 17, shift: 7 },
            Probability { prob: 23, shift: 8 },
            Probability { prob: 11, shift: 8 },
            Probability { prob: 5, shift: 7 },
            Probability {
                prob: 245,
                shift: 8,
            },
            Probability { prob: 61, shift: 6 },
            Probability {
                prob: 233,
                shift: 8,
            },
            Probability {
                prob: 111,
                shift: 7,
            },
            Probability {
                prob: 211,
                shift: 8,
            },
            Probability { prob: 25, shift: 5 },
            Probability {
                prob: 189,
                shift: 8,
            },
            Probability { prob: 89, shift: 7 },
            Probability { prob: 83, shift: 7 },
            Probability {
                prob: 155,
                shift: 8,
            },
            Probability { prob: 9, shift: 4 },
            Probability {
                prob: 133,
                shift: 8,
            },
            Probability { prob: 61, shift: 7 },
            Probability {
                prob: 111,
                shift: 8,
            },
            Probability { prob: 25, shift: 6 },
            Probability { prob: 89, shift: 8 },
            Probability { prob: 77, shift: 8 },
            Probability { prob: 33, shift: 7 },
            Probability { prob: 55, shift: 8 },
            Probability { prob: 11, shift: 6 },
            Probability { prob: 33, shift: 8 },
            Probability { prob: 11, shift: 7 },
            Probability { prob: 11, shift: 8 },
            Probability { prob: 5, shift: 7 },
            Probability {
                prob: 123,
                shift: 7,
            },
            Probability {
                prob: 245,
                shift: 8,
            },
            Probability {
                prob: 117,
                shift: 7,
            },
            Probability { prob: 7, shift: 3 },
            Probability {
                prob: 213,
                shift: 8,
            },
            Probability {
                prob: 101,
                shift: 7,
            },
            Probability { prob: 3, shift: 2 },
            Probability {
                prob: 181,
                shift: 8,
            },
            Probability { prob: 85, shift: 7 },
            Probability { prob: 5, shift: 3 },
            Probability {
                prob: 149,
                shift: 8,
            },
            Probability { prob: 69, shift: 7 },
            Probability { prob: 2, shift: 2 },
            Probability {
                prob: 117,
                shift: 8,
            },
            Probability { prob: 53, shift: 7 },
            Probability { prob: 3, shift: 3 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 37, shift: 7 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 53, shift: 8 },
            Probability { prob: 21, shift: 7 },
            Probability { prob: 1, shift: 3 },
            Probability { prob: 21, shift: 8 },
            Probability { prob: 5, shift: 7 },
            Probability { prob: 9, shift: 8 },
            Probability {
                prob: 123,
                shift: 7,
            },
            Probability {
                prob: 245,
                shift: 8,
            },
            Probability {
                prob: 235,
                shift: 8,
            },
            Probability {
                prob: 225,
                shift: 8,
            },
            Probability {
                prob: 215,
                shift: 8,
            },
            Probability { prob: 51, shift: 6 },
            Probability { prob: 97, shift: 7 },
            Probability { prob: 23, shift: 5 },
            Probability { prob: 87, shift: 7 },
            Probability {
                prob: 163,
                shift: 8,
            },
            Probability {
                prob: 153,
                shift: 8,
            },
            Probability {
                prob: 143,
                shift: 8,
            },
            Probability {
                prob: 133,
                shift: 8,
            },
            Probability { prob: 61, shift: 7 },
            Probability { prob: 7, shift: 4 },
            Probability { prob: 51, shift: 7 },
            Probability { prob: 23, shift: 6 },
            Probability { prob: 81, shift: 8 },
            Probability { prob: 71, shift: 8 },
            Probability { prob: 61, shift: 8 },
            Probability { prob: 51, shift: 8 },
            Probability { prob: 5, shift: 5 },
            Probability { prob: 15, shift: 7 },
            Probability { prob: 5, shift: 6 },
            Probability { prob: 5, shift: 7 },
            Probability { prob: 9, shift: 8 },
            Probability {
                prob: 123,
                shift: 7,
            },
            Probability {
                prob: 123,
                shift: 7,
            },
            Probability { prob: 59, shift: 6 },
            Probability {
                prob: 113,
                shift: 7,
            },
            Probability { prob: 27, shift: 5 },
            Probability {
                prob: 103,
                shift: 7,
            },
            Probability { prob: 49, shift: 6 },
            Probability {
                prob: 187,
                shift: 8,
            },
            Probability {
                prob: 177,
                shift: 8,
            },
            Probability {
                prob: 167,
                shift: 8,
            },
            Probability {
                prob: 157,
                shift: 8,
            },
            Probability {
                prob: 147,
                shift: 8,
            },
            Probability {
                prob: 137,
                shift: 8,
            },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 59, shift: 7 },
            Probability { prob: 27, shift: 6 },
            Probability { prob: 49, shift: 7 },
            Probability { prob: 11, shift: 5 },
            Probability { prob: 39, shift: 7 },
            Probability { prob: 17, shift: 6 },
            Probability { prob: 59, shift: 8 },
            Probability { prob: 49, shift: 8 },
            Probability { prob: 39, shift: 8 },
            Probability { prob: 29, shift: 8 },
            Probability { prob: 19, shift: 8 },
            Probability { prob: 9, shift: 8 },
            Probability { prob: 9, shift: 8 },
            Probability {
                prob: 247,
                shift: 8,
            },
            Probability {
                prob: 123,
                shift: 7,
            },
            Probability {
                prob: 237,
                shift: 8,
            },
            Probability {
                prob: 227,
                shift: 8,
            },
            Probability {
                prob: 109,
                shift: 7,
            },
            Probability { prob: 13, shift: 4 },
            Probability {
                prob: 199,
                shift: 8,
            },
            Probability {
                prob: 189,
                shift: 8,
            },
            Probability { prob: 45, shift: 6 },
            Probability { prob: 85, shift: 7 },
            Probability {
                prob: 161,
                shift: 8,
            },
            Probability {
                prob: 151,
                shift: 8,
            },
            Probability { prob: 71, shift: 7 },
            Probability { prob: 33, shift: 6 },
            Probability {
                prob: 123,
                shift: 8,
            },
            Probability {
                prob: 113,
                shift: 8,
            },
            Probability { prob: 13, shift: 5 },
            Probability { prob: 47, shift: 7 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 75, shift: 8 },
            Probability { prob: 33, shift: 7 },
            Probability { prob: 7, shift: 5 },
            Probability { prob: 47, shift: 8 },
            Probability { prob: 37, shift: 8 },
            Probability { prob: 7, shift: 6 },
            Probability { prob: 9, shift: 7 },
            Probability { prob: 9, shift: 8 },
            Probability { prob: 1, shift: 5 },
            Probability {
                prob: 247,
                shift: 8,
            },
            Probability {
                prob: 123,
                shift: 7,
            },
            Probability {
                prob: 237,
                shift: 8,
            },
            Probability { prob: 57, shift: 6 },
            Probability {
                prob: 219,
                shift: 8,
            },
            Probability {
                prob: 105,
                shift: 7,
            },
            Probability {
                prob: 201,
                shift: 8,
            },
            Probability { prob: 3, shift: 2 },
            Probability { prob: 91, shift: 7 },
            Probability {
                prob: 173,
                shift: 8,
            },
            Probability { prob: 41, shift: 6 },
            Probability {
                prob: 155,
                shift: 8,
            },
            Probability { prob: 73, shift: 7 },
            Probability {
                prob: 137,
                shift: 8,
            },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 59, shift: 7 },
            Probability {
                prob: 109,
                shift: 8,
            },
            Probability { prob: 25, shift: 6 },
            Probability { prob: 91, shift: 8 },
            Probability { prob: 41, shift: 7 },
            Probability { prob: 73, shift: 8 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 27, shift: 7 },
            Probability { prob: 45, shift: 8 },
            Probability { prob: 9, shift: 6 },
            Probability { prob: 27, shift: 8 },
            Probability { prob: 9, shift: 7 },
            Probability { prob: 9, shift: 8 },
            Probability { prob: 1, shift: 5 },
            Probability {
                prob: 247,
                shift: 8,
            },
            Probability {
                prob: 247,
                shift: 8,
            },
            Probability {
                prob: 119,
                shift: 7,
            },
            Probability {
                prob: 229,
                shift: 8,
            },
            Probability { prob: 55, shift: 6 },
            Probability {
                prob: 211,
                shift: 8,
            },
            Probability {
                prob: 203,
                shift: 8,
            },
            Probability { prob: 97, shift: 7 },
            Probability {
                prob: 185,
                shift: 8,
            },
            Probability { prob: 11, shift: 4 },
            Probability {
                prob: 167,
                shift: 8,
            },
            Probability { prob: 79, shift: 7 },
            Probability { prob: 75, shift: 7 },
            Probability {
                prob: 141,
                shift: 8,
            },
            Probability { prob: 33, shift: 6 },
            Probability {
                prob: 123,
                shift: 8,
            },
            Probability { prob: 57, shift: 7 },
            Probability {
                prob: 105,
                shift: 8,
            },
            Probability { prob: 97, shift: 8 },
            Probability { prob: 11, shift: 5 },
            Probability { prob: 79, shift: 8 },
            Probability { prob: 35, shift: 7 },
            Probability { prob: 61, shift: 8 },
            Probability { prob: 13, shift: 6 },
            Probability { prob: 11, shift: 6 },
            Probability { prob: 35, shift: 8 },
            Probability { prob: 13, shift: 7 },
            Probability { prob: 17, shift: 8 },
            Probability { prob: 1, shift: 5 },
            Probability { prob: 1, shift: 5 },
            Probability { prob: 31, shift: 5 },
            Probability {
                prob: 247,
                shift: 8,
            },
            Probability {
                prob: 119,
                shift: 7,
            },
            Probability {
                prob: 115,
                shift: 7,
            },
            Probability {
                prob: 221,
                shift: 8,
            },
            Probability {
                prob: 213,
                shift: 8,
            },
            Probability { prob: 51, shift: 6 },
            Probability { prob: 49, shift: 6 },
            Probability {
                prob: 187,
                shift: 8,
            },
            Probability {
                prob: 179,
                shift: 8,
            },
            Probability { prob: 85, shift: 7 },
            Probability { prob: 81, shift: 7 },
            Probability {
                prob: 153,
                shift: 8,
            },
            Probability {
                prob: 145,
                shift: 8,
            },
            Probability { prob: 17, shift: 5 },
            Probability { prob: 2, shift: 2 },
            Probability {
                prob: 119,
                shift: 8,
            },
            Probability { prob: 55, shift: 7 },
            Probability { prob: 51, shift: 7 },
            Probability { prob: 93, shift: 8 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 19, shift: 6 },
            Probability { prob: 17, shift: 6 },
            Probability { prob: 59, shift: 8 },
            Probability { prob: 51, shift: 8 },
            Probability { prob: 21, shift: 7 },
            Probability { prob: 17, shift: 7 },
            Probability { prob: 25, shift: 8 },
            Probability { prob: 17, shift: 8 },
            Probability { prob: 1, shift: 5 },
            Probability { prob: 1, shift: 5 },
            Probability { prob: 31, shift: 5 },
            Probability {
                prob: 247,
                shift: 8,
            },
            Probability {
                prob: 239,
                shift: 8,
            },
            Probability {
                prob: 231,
                shift: 8,
            },
            Probability {
                prob: 111,
                shift: 7,
            },
            Probability {
                prob: 107,
                shift: 7,
            },
            Probability {
                prob: 103,
                shift: 7,
            },
            Probability { prob: 99, shift: 7 },
            Probability {
                prob: 189,
                shift: 8,
            },
            Probability {
                prob: 181,
                shift: 8,
            },
            Probability {
                prob: 173,
                shift: 8,
            },
            Probability {
                prob: 165,
                shift: 8,
            },
            Probability { prob: 39, shift: 6 },
            Probability { prob: 37, shift: 6 },
            Probability { prob: 35, shift: 6 },
            Probability { prob: 33, shift: 6 },
            Probability {
                prob: 123,
                shift: 8,
            },
            Probability {
                prob: 115,
                shift: 8,
            },
            Probability {
                prob: 107,
                shift: 8,
            },
            Probability { prob: 99, shift: 8 },
            Probability { prob: 45, shift: 7 },
            Probability { prob: 41, shift: 7 },
            Probability { prob: 37, shift: 7 },
            Probability { prob: 33, shift: 7 },
            Probability { prob: 57, shift: 8 },
            Probability { prob: 49, shift: 8 },
            Probability { prob: 41, shift: 8 },
            Probability { prob: 33, shift: 8 },
            Probability { prob: 3, shift: 5 },
            Probability { prob: 1, shift: 4 },
            Probability { prob: 1, shift: 5 },
            Probability { prob: 7, shift: 8 },
            Probability { prob: 31, shift: 5 },
            Probability { prob: 31, shift: 5 },
            Probability { prob: 15, shift: 4 },
            Probability { prob: 29, shift: 5 },
            Probability { prob: 7, shift: 3 },
            Probability { prob: 27, shift: 5 },
            Probability { prob: 13, shift: 4 },
            Probability { prob: 25, shift: 5 },
            Probability { prob: 3, shift: 2 },
            Probability { prob: 23, shift: 5 },
            Probability { prob: 11, shift: 4 },
            Probability { prob: 21, shift: 5 },
            Probability { prob: 5, shift: 3 },
            Probability { prob: 19, shift: 5 },
            Probability { prob: 9, shift: 4 },
            Probability { prob: 17, shift: 5 },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 15, shift: 5 },
            Probability { prob: 7, shift: 4 },
            Probability { prob: 13, shift: 5 },
            Probability { prob: 3, shift: 3 },
            Probability { prob: 11, shift: 5 },
            Probability { prob: 5, shift: 4 },
            Probability { prob: 9, shift: 5 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 7, shift: 5 },
            Probability { prob: 3, shift: 4 },
            Probability { prob: 5, shift: 5 },
            Probability { prob: 1, shift: 3 },
            Probability { prob: 3, shift: 5 },
            Probability { prob: 1, shift: 4 },
            Probability { prob: 1, shift: 5 },
            Probability { prob: 7, shift: 8 },
            Probability { prob: 31, shift: 5 },
            Probability { prob: 31, shift: 5 },
            Probability { prob: 15, shift: 4 },
            Probability { prob: 29, shift: 5 },
            Probability { prob: 7, shift: 3 },
            Probability {
                prob: 217,
                shift: 8,
            },
            Probability {
                prob: 209,
                shift: 8,
            },
            Probability {
                prob: 201,
                shift: 8,
            },
            Probability {
                prob: 193,
                shift: 8,
            },
            Probability { prob: 93, shift: 7 },
            Probability { prob: 89, shift: 7 },
            Probability { prob: 85, shift: 7 },
            Probability { prob: 81, shift: 7 },
            Probability {
                prob: 155,
                shift: 8,
            },
            Probability {
                prob: 147,
                shift: 8,
            },
            Probability {
                prob: 139,
                shift: 8,
            },
            Probability {
                prob: 131,
                shift: 8,
            },
            Probability { prob: 31, shift: 6 },
            Probability { prob: 29, shift: 6 },
            Probability { prob: 27, shift: 6 },
            Probability { prob: 25, shift: 6 },
            Probability { prob: 93, shift: 8 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 77, shift: 8 },
            Probability { prob: 69, shift: 8 },
            Probability { prob: 31, shift: 7 },
            Probability { prob: 27, shift: 7 },
            Probability { prob: 23, shift: 7 },
            Probability { prob: 19, shift: 7 },
            Probability { prob: 31, shift: 8 },
            Probability { prob: 23, shift: 8 },
            Probability { prob: 15, shift: 8 },
            Probability { prob: 7, shift: 8 },
            Probability { prob: 7, shift: 8 },
            Probability { prob: 31, shift: 5 },
            Probability { prob: 31, shift: 5 },
            Probability { prob: 15, shift: 4 },
            Probability {
                prob: 233,
                shift: 8,
            },
            Probability {
                prob: 225,
                shift: 8,
            },
            Probability {
                prob: 109,
                shift: 7,
            },
            Probability {
                prob: 105,
                shift: 7,
            },
            Probability {
                prob: 203,
                shift: 8,
            },
            Probability {
                prob: 195,
                shift: 8,
            },
            Probability { prob: 47, shift: 6 },
            Probability { prob: 45, shift: 6 },
            Probability {
                prob: 173,
                shift: 8,
            },
            Probability {
                prob: 165,
                shift: 8,
            },
            Probability { prob: 79, shift: 7 },
            Probability { prob: 75, shift: 7 },
            Probability {
                prob: 143,
                shift: 8,
            },
            Probability {
                prob: 135,
                shift: 8,
            },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 15, shift: 5 },
            Probability { prob: 7, shift: 4 },
            Probability {
                prob: 105,
                shift: 8,
            },
            Probability { prob: 97, shift: 8 },
            Probability { prob: 45, shift: 7 },
            Probability { prob: 41, shift: 7 },
            Probability { prob: 75, shift: 8 },
            Probability { prob: 67, shift: 8 },
            Probability { prob: 15, shift: 6 },
            Probability { prob: 13, shift: 6 },
            Probability { prob: 45, shift: 8 },
            Probability { prob: 37, shift: 8 },
            Probability { prob: 15, shift: 7 },
            Probability { prob: 11, shift: 7 },
            Probability { prob: 15, shift: 8 },
            Probability { prob: 7, shift: 8 },
            Probability { prob: 7, shift: 8 },
            Probability {
                prob: 249,
                shift: 8,
            },
            Probability { prob: 31, shift: 5 },
            Probability {
                prob: 241,
                shift: 8,
            },
            Probability {
                prob: 117,
                shift: 7,
            },
            Probability {
                prob: 113,
                shift: 7,
            },
            Probability {
                prob: 219,
                shift: 8,
            },
            Probability { prob: 53, shift: 6 },
            Probability { prob: 51, shift: 6 },
            Probability {
                prob: 197,
                shift: 8,
            },
            Probability { prob: 95, shift: 7 },
            Probability { prob: 91, shift: 7 },
            Probability {
                prob: 175,
                shift: 8,
            },
            Probability { prob: 21, shift: 5 },
            Probability { prob: 5, shift: 3 },
            Probability {
                prob: 153,
                shift: 8,
            },
            Probability { prob: 73, shift: 7 },
            Probability { prob: 69, shift: 7 },
            Probability {
                prob: 131,
                shift: 8,
            },
            Probability { prob: 31, shift: 6 },
            Probability {
                prob: 117,
                shift: 8,
            },
            Probability {
                prob: 109,
                shift: 8,
            },
            Probability { prob: 51, shift: 7 },
            Probability { prob: 95, shift: 8 },
            Probability { prob: 87, shift: 8 },
            Probability { prob: 5, shift: 4 },
            Probability { prob: 73, shift: 8 },
            Probability { prob: 65, shift: 8 },
            Probability { prob: 29, shift: 7 },
            Probability { prob: 51, shift: 8 },
            Probability { prob: 43, shift: 8 },
            Probability { prob: 9, shift: 6 },
            Probability { prob: 29, shift: 8 },
            Probability { prob: 21, shift: 8 },
            Probability { prob: 7, shift: 7 },
            Probability { prob: 7, shift: 8 },
            Probability { prob: 3, shift: 7 },
            Probability {
                prob: 249,
                shift: 8,
            },
            Probability { prob: 31, shift: 5 },
            Probability {
                prob: 241,
                shift: 8,
            },
            Probability {
                prob: 117,
                shift: 7,
            },
            Probability {
                prob: 227,
                shift: 8,
            },
            Probability { prob: 55, shift: 6 },
            Probability {
                prob: 213,
                shift: 8,
            },
            Probability {
                prob: 103,
                shift: 7,
            },
            Probability {
                prob: 199,
                shift: 8,
            },
            Probability { prob: 3, shift: 2 },
            Probability { prob: 23, shift: 5 },
            Probability {
                prob: 177,
                shift: 8,
            },
            Probability { prob: 85, shift: 7 },
            Probability {
                prob: 163,
                shift: 8,
            },
            Probability { prob: 39, shift: 6 },
            Probability {
                prob: 149,
                shift: 8,
            },
            Probability { prob: 71, shift: 7 },
            Probability {
                prob: 135,
                shift: 8,
            },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 15, shift: 5 },
            Probability {
                prob: 113,
                shift: 8,
            },
            Probability { prob: 53, shift: 7 },
            Probability { prob: 99, shift: 8 },
            Probability { prob: 23, shift: 6 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 39, shift: 7 },
            Probability { prob: 71, shift: 8 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 7, shift: 5 },
            Probability { prob: 49, shift: 8 },
            Probability { prob: 21, shift: 7 },
            Probability { prob: 35, shift: 8 },
            Probability { prob: 7, shift: 6 },
            Probability { prob: 21, shift: 8 },
            Probability { prob: 7, shift: 7 },
            Probability { prob: 7, shift: 8 },
            Probability { prob: 3, shift: 7 },
            Probability {
                prob: 249,
                shift: 8,
            },
            Probability {
                prob: 249,
                shift: 8,
            },
            Probability {
                prob: 121,
                shift: 7,
            },
            Probability {
                prob: 235,
                shift: 8,
            },
            Probability { prob: 57, shift: 6 },
            Probability {
                prob: 221,
                shift: 8,
            },
            Probability {
                prob: 107,
                shift: 7,
            },
            Probability {
                prob: 207,
                shift: 8,
            },
            Probability { prob: 25, shift: 5 },
            Probability {
                prob: 193,
                shift: 8,
            },
            Probability { prob: 93, shift: 7 },
            Probability {
                prob: 179,
                shift: 8,
            },
            Probability { prob: 43, shift: 6 },
            Probability { prob: 83, shift: 7 },
            Probability {
                prob: 159,
                shift: 8,
            },
            Probability { prob: 19, shift: 5 },
            Probability {
                prob: 145,
                shift: 8,
            },
            Probability { prob: 69, shift: 7 },
            Probability {
                prob: 131,
                shift: 8,
            },
            Probability { prob: 31, shift: 6 },
            Probability {
                prob: 117,
                shift: 8,
            },
            Probability { prob: 55, shift: 7 },
            Probability {
                prob: 103,
                shift: 8,
            },
            Probability { prob: 3, shift: 3 },
            Probability { prob: 89, shift: 8 },
            Probability { prob: 83, shift: 8 },
            Probability { prob: 19, shift: 6 },
            Probability { prob: 69, shift: 8 },
            Probability { prob: 31, shift: 7 },
            Probability { prob: 55, shift: 8 },
            Probability { prob: 3, shift: 4 },
            Probability { prob: 41, shift: 8 },
            Probability { prob: 17, shift: 7 },
            Probability { prob: 27, shift: 8 },
            Probability { prob: 5, shift: 6 },
            Probability { prob: 13, shift: 8 },
            Probability { prob: 3, shift: 7 },
            Probability { prob: 3, shift: 7 },
            Probability {
                prob: 249,
                shift: 8,
            },
            Probability {
                prob: 249,
                shift: 8,
            },
            Probability {
                prob: 121,
                shift: 7,
            },
            Probability {
                prob: 235,
                shift: 8,
            },
            Probability {
                prob: 229,
                shift: 8,
            },
            Probability {
                prob: 111,
                shift: 7,
            },
            Probability {
                prob: 215,
                shift: 8,
            },
            Probability { prob: 13, shift: 4 },
            Probability {
                prob: 101,
                shift: 7,
            },
            Probability {
                prob: 195,
                shift: 8,
            },
            Probability { prob: 47, shift: 6 },
            Probability {
                prob: 181,
                shift: 8,
            },
            Probability {
                prob: 175,
                shift: 8,
            },
            Probability { prob: 21, shift: 5 },
            Probability {
                prob: 161,
                shift: 8,
            },
            Probability { prob: 77, shift: 7 },
            Probability { prob: 37, shift: 6 },
            Probability {
                prob: 141,
                shift: 8,
            },
            Probability { prob: 67, shift: 7 },
            Probability { prob: 2, shift: 2 },
            Probability {
                prob: 121,
                shift: 8,
            },
            Probability { prob: 57, shift: 7 },
            Probability {
                prob: 107,
                shift: 8,
            },
            Probability {
                prob: 101,
                shift: 8,
            },
            Probability { prob: 47, shift: 7 },
            Probability { prob: 87, shift: 8 },
            Probability { prob: 5, shift: 4 },
            Probability { prob: 37, shift: 7 },
            Probability { prob: 67, shift: 8 },
            Probability { prob: 15, shift: 6 },
            Probability { prob: 53, shift: 8 },
            Probability { prob: 47, shift: 8 },
            Probability { prob: 5, shift: 5 },
            Probability { prob: 33, shift: 8 },
            Probability { prob: 13, shift: 7 },
            Probability { prob: 5, shift: 6 },
            Probability { prob: 13, shift: 8 },
            Probability { prob: 3, shift: 7 },
            Probability { prob: 3, shift: 7 },
            Probability {
                prob: 249,
                shift: 8,
            },
            Probability {
                prob: 249,
                shift: 8,
            },
            Probability {
                prob: 121,
                shift: 7,
            },
            Probability { prob: 59, shift: 6 },
            Probability {
                prob: 229,
                shift: 8,
            },
            Probability {
                prob: 223,
                shift: 8,
            },
            Probability { prob: 27, shift: 5 },
            Probability {
                prob: 105,
                shift: 7,
            },
            Probability {
                prob: 203,
                shift: 8,
            },
            Probability { prob: 49, shift: 6 },
            Probability { prob: 95, shift: 7 },
            Probability {
                prob: 183,
                shift: 8,
            },
            Probability {
                prob: 177,
                shift: 8,
            },
            Probability { prob: 85, shift: 7 },
            Probability { prob: 41, shift: 6 },
            Probability {
                prob: 157,
                shift: 8,
            },
            Probability { prob: 75, shift: 7 },
            Probability { prob: 9, shift: 4 },
            Probability {
                prob: 137,
                shift: 8,
            },
            Probability {
                prob: 131,
                shift: 8,
            },
            Probability { prob: 31, shift: 6 },
            Probability { prob: 59, shift: 7 },
            Probability {
                prob: 111,
                shift: 8,
            },
            Probability {
                prob: 105,
                shift: 8,
            },
            Probability { prob: 49, shift: 7 },
            Probability { prob: 91, shift: 8 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 39, shift: 7 },
            Probability { prob: 9, shift: 5 },
            Probability { prob: 65, shift: 8 },
            Probability { prob: 59, shift: 8 },
            Probability { prob: 13, shift: 6 },
            Probability { prob: 45, shift: 8 },
            Probability { prob: 39, shift: 8 },
            Probability { prob: 1, shift: 3 },
            Probability { prob: 13, shift: 7 },
            Probability { prob: 19, shift: 8 },
            Probability { prob: 13, shift: 8 },
            Probability { prob: 3, shift: 7 },
            Probability { prob: 3, shift: 7 },
            Probability {
                prob: 249,
                shift: 8,
            },
            Probability {
                prob: 249,
                shift: 8,
            },
            Probability {
                prob: 243,
                shift: 8,
            },
            Probability { prob: 59, shift: 6 },
            Probability {
                prob: 115,
                shift: 7,
            },
            Probability { prob: 7, shift: 3 },
            Probability {
                prob: 217,
                shift: 8,
            },
            Probability {
                prob: 211,
                shift: 8,
            },
            Probability { prob: 51, shift: 6 },
            Probability { prob: 99, shift: 7 },
            Probability { prob: 3, shift: 2 },
            Probability {
                prob: 185,
                shift: 8,
            },
            Probability {
                prob: 179,
                shift: 8,
            },
            Probability { prob: 43, shift: 6 },
            Probability { prob: 83, shift: 7 },
            Probability { prob: 5, shift: 3 },
            Probability {
                prob: 153,
                shift: 8,
            },
            Probability {
                prob: 147,
                shift: 8,
            },
            Probability { prob: 35, shift: 6 },
            Probability { prob: 67, shift: 7 },
            Probability { prob: 2, shift: 2 },
            Probability {
                prob: 121,
                shift: 8,
            },
            Probability {
                prob: 115,
                shift: 8,
            },
            Probability { prob: 27, shift: 6 },
            Probability { prob: 51, shift: 7 },
            Probability { prob: 3, shift: 3 },
            Probability { prob: 89, shift: 8 },
            Probability { prob: 83, shift: 8 },
            Probability { prob: 19, shift: 6 },
            Probability { prob: 35, shift: 7 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 57, shift: 8 },
            Probability { prob: 51, shift: 8 },
            Probability { prob: 11, shift: 6 },
            Probability { prob: 19, shift: 7 },
            Probability { prob: 1, shift: 3 },
            Probability { prob: 25, shift: 8 },
            Probability { prob: 19, shift: 8 },
            Probability { prob: 3, shift: 6 },
            Probability { prob: 3, shift: 7 },
            Probability { prob: 3, shift: 7 },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability {
                prob: 249,
                shift: 8,
            },
            Probability {
                prob: 243,
                shift: 8,
            },
            Probability {
                prob: 237,
                shift: 8,
            },
            Probability {
                prob: 231,
                shift: 8,
            },
            Probability { prob: 7, shift: 3 },
            Probability {
                prob: 109,
                shift: 7,
            },
            Probability { prob: 53, shift: 6 },
            Probability {
                prob: 103,
                shift: 7,
            },
            Probability {
                prob: 199,
                shift: 8,
            },
            Probability {
                prob: 193,
                shift: 8,
            },
            Probability {
                prob: 187,
                shift: 8,
            },
            Probability {
                prob: 181,
                shift: 8,
            },
            Probability { prob: 87, shift: 7 },
            Probability { prob: 21, shift: 5 },
            Probability { prob: 81, shift: 7 },
            Probability { prob: 39, shift: 6 },
            Probability {
                prob: 149,
                shift: 8,
            },
            Probability {
                prob: 143,
                shift: 8,
            },
            Probability {
                prob: 137,
                shift: 8,
            },
            Probability {
                prob: 131,
                shift: 8,
            },
            Probability { prob: 31, shift: 6 },
            Probability { prob: 59, shift: 7 },
            Probability { prob: 7, shift: 4 },
            Probability { prob: 53, shift: 7 },
            Probability { prob: 99, shift: 8 },
            Probability { prob: 93, shift: 8 },
            Probability { prob: 87, shift: 8 },
            Probability { prob: 81, shift: 8 },
            Probability { prob: 37, shift: 7 },
            Probability { prob: 17, shift: 6 },
            Probability { prob: 31, shift: 7 },
            Probability { prob: 7, shift: 5 },
            Probability { prob: 49, shift: 8 },
            Probability { prob: 43, shift: 8 },
            Probability { prob: 37, shift: 8 },
            Probability { prob: 31, shift: 8 },
            Probability { prob: 3, shift: 5 },
            Probability { prob: 9, shift: 7 },
            Probability { prob: 3, shift: 6 },
            Probability { prob: 3, shift: 7 },
            Probability { prob: 5, shift: 8 },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability {
                prob: 249,
                shift: 8,
            },
            Probability {
                prob: 243,
                shift: 8,
            },
            Probability {
                prob: 237,
                shift: 8,
            },
            Probability {
                prob: 231,
                shift: 8,
            },
            Probability {
                prob: 225,
                shift: 8,
            },
            Probability {
                prob: 219,
                shift: 8,
            },
            Probability {
                prob: 213,
                shift: 8,
            },
            Probability {
                prob: 207,
                shift: 8,
            },
            Probability {
                prob: 201,
                shift: 8,
            },
            Probability {
                prob: 195,
                shift: 8,
            },
            Probability { prob: 47, shift: 6 },
            Probability { prob: 91, shift: 7 },
            Probability { prob: 11, shift: 4 },
            Probability { prob: 85, shift: 7 },
            Probability { prob: 41, shift: 6 },
            Probability { prob: 79, shift: 7 },
            Probability { prob: 19, shift: 5 },
            Probability { prob: 73, shift: 7 },
            Probability { prob: 35, shift: 6 },
            Probability { prob: 67, shift: 7 },
            Probability { prob: 2, shift: 2 },
            Probability {
                prob: 121,
                shift: 8,
            },
            Probability {
                prob: 115,
                shift: 8,
            },
            Probability {
                prob: 109,
                shift: 8,
            },
            Probability {
                prob: 103,
                shift: 8,
            },
            Probability { prob: 97, shift: 8 },
            Probability { prob: 91, shift: 8 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 79, shift: 8 },
            Probability { prob: 73, shift: 8 },
            Probability { prob: 67, shift: 8 },
            Probability { prob: 15, shift: 6 },
            Probability { prob: 27, shift: 7 },
            Probability { prob: 3, shift: 4 },
            Probability { prob: 21, shift: 7 },
            Probability { prob: 9, shift: 6 },
            Probability { prob: 15, shift: 7 },
            Probability { prob: 3, shift: 5 },
            Probability { prob: 9, shift: 7 },
            Probability { prob: 3, shift: 6 },
            Probability { prob: 3, shift: 7 },
            Probability { prob: 5, shift: 8 },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability { prob: 61, shift: 6 },
            Probability {
                prob: 119,
                shift: 7,
            },
            Probability { prob: 29, shift: 5 },
            Probability {
                prob: 113,
                shift: 7,
            },
            Probability { prob: 55, shift: 6 },
            Probability {
                prob: 107,
                shift: 7,
            },
            Probability { prob: 13, shift: 4 },
            Probability {
                prob: 101,
                shift: 7,
            },
            Probability { prob: 49, shift: 6 },
            Probability { prob: 95, shift: 7 },
            Probability { prob: 23, shift: 5 },
            Probability { prob: 89, shift: 7 },
            Probability { prob: 43, shift: 6 },
            Probability { prob: 83, shift: 7 },
            Probability { prob: 5, shift: 3 },
            Probability { prob: 77, shift: 7 },
            Probability { prob: 37, shift: 6 },
            Probability { prob: 71, shift: 7 },
            Probability { prob: 17, shift: 5 },
            Probability { prob: 65, shift: 7 },
            Probability {
                prob: 125,
                shift: 8,
            },
            Probability {
                prob: 119,
                shift: 8,
            },
            Probability {
                prob: 113,
                shift: 8,
            },
            Probability {
                prob: 107,
                shift: 8,
            },
            Probability {
                prob: 101,
                shift: 8,
            },
            Probability { prob: 95, shift: 8 },
            Probability { prob: 89, shift: 8 },
            Probability { prob: 83, shift: 8 },
            Probability { prob: 77, shift: 8 },
            Probability { prob: 71, shift: 8 },
            Probability { prob: 65, shift: 8 },
            Probability { prob: 59, shift: 8 },
            Probability { prob: 53, shift: 8 },
            Probability { prob: 47, shift: 8 },
            Probability { prob: 41, shift: 8 },
            Probability { prob: 35, shift: 8 },
            Probability { prob: 29, shift: 8 },
            Probability { prob: 23, shift: 8 },
            Probability { prob: 17, shift: 8 },
            Probability { prob: 11, shift: 8 },
            Probability { prob: 5, shift: 8 },
            Probability { prob: 5, shift: 8 },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability { prob: 61, shift: 6 },
            Probability {
                prob: 119,
                shift: 7,
            },
            Probability { prob: 29, shift: 5 },
            Probability {
                prob: 113,
                shift: 7,
            },
            Probability {
                prob: 221,
                shift: 8,
            },
            Probability {
                prob: 215,
                shift: 8,
            },
            Probability {
                prob: 209,
                shift: 8,
            },
            Probability {
                prob: 203,
                shift: 8,
            },
            Probability {
                prob: 197,
                shift: 8,
            },
            Probability { prob: 3, shift: 2 },
            Probability { prob: 93, shift: 7 },
            Probability { prob: 45, shift: 6 },
            Probability { prob: 87, shift: 7 },
            Probability { prob: 21, shift: 5 },
            Probability { prob: 81, shift: 7 },
            Probability {
                prob: 157,
                shift: 8,
            },
            Probability {
                prob: 151,
                shift: 8,
            },
            Probability {
                prob: 145,
                shift: 8,
            },
            Probability {
                prob: 139,
                shift: 8,
            },
            Probability {
                prob: 133,
                shift: 8,
            },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 61, shift: 7 },
            Probability { prob: 29, shift: 6 },
            Probability { prob: 55, shift: 7 },
            Probability { prob: 13, shift: 5 },
            Probability { prob: 49, shift: 7 },
            Probability { prob: 93, shift: 8 },
            Probability { prob: 87, shift: 8 },
            Probability { prob: 81, shift: 8 },
            Probability { prob: 75, shift: 8 },
            Probability { prob: 69, shift: 8 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 29, shift: 7 },
            Probability { prob: 13, shift: 6 },
            Probability { prob: 23, shift: 7 },
            Probability { prob: 5, shift: 5 },
            Probability { prob: 17, shift: 7 },
            Probability { prob: 29, shift: 8 },
            Probability { prob: 23, shift: 8 },
            Probability { prob: 17, shift: 8 },
            Probability { prob: 11, shift: 8 },
            Probability { prob: 5, shift: 8 },
            Probability { prob: 5, shift: 8 },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability { prob: 61, shift: 6 },
            Probability {
                prob: 119,
                shift: 7,
            },
            Probability {
                prob: 233,
                shift: 8,
            },
            Probability {
                prob: 227,
                shift: 8,
            },
            Probability {
                prob: 221,
                shift: 8,
            },
            Probability { prob: 27, shift: 5 },
            Probability {
                prob: 105,
                shift: 7,
            },
            Probability { prob: 51, shift: 6 },
            Probability {
                prob: 199,
                shift: 8,
            },
            Probability {
                prob: 193,
                shift: 8,
            },
            Probability {
                prob: 187,
                shift: 8,
            },
            Probability { prob: 91, shift: 7 },
            Probability { prob: 11, shift: 4 },
            Probability { prob: 85, shift: 7 },
            Probability { prob: 41, shift: 6 },
            Probability {
                prob: 159,
                shift: 8,
            },
            Probability {
                prob: 153,
                shift: 8,
            },
            Probability {
                prob: 147,
                shift: 8,
            },
            Probability { prob: 71, shift: 7 },
            Probability { prob: 17, shift: 5 },
            Probability { prob: 65, shift: 7 },
            Probability {
                prob: 125,
                shift: 8,
            },
            Probability {
                prob: 119,
                shift: 8,
            },
            Probability {
                prob: 113,
                shift: 8,
            },
            Probability { prob: 27, shift: 6 },
            Probability { prob: 51, shift: 7 },
            Probability { prob: 3, shift: 3 },
            Probability { prob: 91, shift: 8 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 79, shift: 8 },
            Probability { prob: 73, shift: 8 },
            Probability { prob: 17, shift: 6 },
            Probability { prob: 31, shift: 7 },
            Probability { prob: 7, shift: 5 },
            Probability { prob: 51, shift: 8 },
            Probability { prob: 45, shift: 8 },
            Probability { prob: 39, shift: 8 },
            Probability { prob: 17, shift: 7 },
            Probability { prob: 7, shift: 6 },
            Probability { prob: 11, shift: 7 },
            Probability { prob: 17, shift: 8 },
            Probability { prob: 11, shift: 8 },
            Probability { prob: 5, shift: 8 },
            Probability { prob: 5, shift: 8 },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability { prob: 61, shift: 6 },
            Probability {
                prob: 239,
                shift: 8,
            },
            Probability {
                prob: 233,
                shift: 8,
            },
            Probability { prob: 57, shift: 6 },
            Probability {
                prob: 111,
                shift: 7,
            },
            Probability {
                prob: 217,
                shift: 8,
            },
            Probability {
                prob: 211,
                shift: 8,
            },
            Probability {
                prob: 205,
                shift: 8,
            },
            Probability { prob: 25, shift: 5 },
            Probability { prob: 97, shift: 7 },
            Probability {
                prob: 189,
                shift: 8,
            },
            Probability {
                prob: 183,
                shift: 8,
            },
            Probability { prob: 89, shift: 7 },
            Probability { prob: 43, shift: 6 },
            Probability { prob: 83, shift: 7 },
            Probability {
                prob: 161,
                shift: 8,
            },
            Probability {
                prob: 155,
                shift: 8,
            },
            Probability { prob: 75, shift: 7 },
            Probability { prob: 9, shift: 4 },
            Probability {
                prob: 139,
                shift: 8,
            },
            Probability {
                prob: 133,
                shift: 8,
            },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 61, shift: 7 },
            Probability { prob: 29, shift: 6 },
            Probability {
                prob: 111,
                shift: 8,
            },
            Probability {
                prob: 105,
                shift: 8,
            },
            Probability { prob: 25, shift: 6 },
            Probability { prob: 47, shift: 7 },
            Probability { prob: 89, shift: 8 },
            Probability { prob: 83, shift: 8 },
            Probability { prob: 77, shift: 8 },
            Probability { prob: 9, shift: 5 },
            Probability { prob: 33, shift: 7 },
            Probability { prob: 61, shift: 8 },
            Probability { prob: 55, shift: 8 },
            Probability { prob: 25, shift: 7 },
            Probability { prob: 11, shift: 6 },
            Probability { prob: 19, shift: 7 },
            Probability { prob: 33, shift: 8 },
            Probability { prob: 27, shift: 8 },
            Probability { prob: 11, shift: 7 },
            Probability { prob: 1, shift: 4 },
            Probability { prob: 11, shift: 8 },
            Probability { prob: 5, shift: 8 },
            Probability { prob: 5, shift: 8 },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability {
                prob: 245,
                shift: 8,
            },
            Probability {
                prob: 239,
                shift: 8,
            },
            Probability {
                prob: 117,
                shift: 7,
            },
            Probability { prob: 57, shift: 6 },
            Probability {
                prob: 223,
                shift: 8,
            },
            Probability {
                prob: 217,
                shift: 8,
            },
            Probability { prob: 53, shift: 6 },
            Probability {
                prob: 103,
                shift: 7,
            },
            Probability {
                prob: 201,
                shift: 8,
            },
            Probability { prob: 49, shift: 6 },
            Probability { prob: 95, shift: 7 },
            Probability {
                prob: 185,
                shift: 8,
            },
            Probability {
                prob: 179,
                shift: 8,
            },
            Probability { prob: 87, shift: 7 },
            Probability { prob: 21, shift: 5 },
            Probability {
                prob: 163,
                shift: 8,
            },
            Probability {
                prob: 157,
                shift: 8,
            },
            Probability { prob: 19, shift: 5 },
            Probability {
                prob: 147,
                shift: 8,
            },
            Probability {
                prob: 141,
                shift: 8,
            },
            Probability { prob: 17, shift: 5 },
            Probability { prob: 65, shift: 7 },
            Probability {
                prob: 125,
                shift: 8,
            },
            Probability {
                prob: 119,
                shift: 8,
            },
            Probability { prob: 57, shift: 7 },
            Probability { prob: 27, shift: 6 },
            Probability {
                prob: 103,
                shift: 8,
            },
            Probability { prob: 49, shift: 7 },
            Probability { prob: 23, shift: 6 },
            Probability { prob: 87, shift: 8 },
            Probability { prob: 81, shift: 8 },
            Probability { prob: 19, shift: 6 },
            Probability { prob: 35, shift: 7 },
            Probability { prob: 65, shift: 8 },
            Probability { prob: 59, shift: 8 },
            Probability { prob: 27, shift: 7 },
            Probability { prob: 49, shift: 8 },
            Probability { prob: 43, shift: 8 },
            Probability { prob: 19, shift: 7 },
            Probability { prob: 1, shift: 3 },
            Probability { prob: 27, shift: 8 },
            Probability { prob: 21, shift: 8 },
            Probability { prob: 1, shift: 4 },
            Probability { prob: 5, shift: 7 },
            Probability { prob: 5, shift: 8 },
            Probability { prob: 5, shift: 8 },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability {
                prob: 245,
                shift: 8,
            },
            Probability { prob: 15, shift: 4 },
            Probability {
                prob: 117,
                shift: 7,
            },
            Probability {
                prob: 229,
                shift: 8,
            },
            Probability { prob: 7, shift: 3 },
            Probability {
                prob: 109,
                shift: 7,
            },
            Probability {
                prob: 213,
                shift: 8,
            },
            Probability { prob: 13, shift: 4 },
            Probability {
                prob: 101,
                shift: 7,
            },
            Probability {
                prob: 197,
                shift: 8,
            },
            Probability { prob: 3, shift: 2 },
            Probability { prob: 93, shift: 7 },
            Probability {
                prob: 181,
                shift: 8,
            },
            Probability { prob: 11, shift: 4 },
            Probability { prob: 85, shift: 7 },
            Probability {
                prob: 165,
                shift: 8,
            },
            Probability { prob: 5, shift: 3 },
            Probability { prob: 77, shift: 7 },
            Probability {
                prob: 149,
                shift: 8,
            },
            Probability { prob: 9, shift: 4 },
            Probability { prob: 69, shift: 7 },
            Probability {
                prob: 133,
                shift: 8,
            },
            Probability { prob: 2, shift: 2 },
            Probability { prob: 61, shift: 7 },
            Probability {
                prob: 117,
                shift: 8,
            },
            Probability { prob: 7, shift: 4 },
            Probability { prob: 53, shift: 7 },
            Probability {
                prob: 101,
                shift: 8,
            },
            Probability { prob: 3, shift: 3 },
            Probability { prob: 45, shift: 7 },
            Probability { prob: 85, shift: 8 },
            Probability { prob: 5, shift: 4 },
            Probability { prob: 37, shift: 7 },
            Probability { prob: 69, shift: 8 },
            Probability { prob: 1, shift: 2 },
            Probability { prob: 29, shift: 7 },
            Probability { prob: 53, shift: 8 },
            Probability { prob: 3, shift: 4 },
            Probability { prob: 21, shift: 7 },
            Probability { prob: 37, shift: 8 },
            Probability { prob: 1, shift: 3 },
            Probability { prob: 13, shift: 7 },
            Probability { prob: 21, shift: 8 },
            Probability { prob: 1, shift: 4 },
            Probability { prob: 5, shift: 7 },
            Probability { prob: 5, shift: 8 },
            Probability { prob: 5, shift: 8 },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability {
                prob: 125,
                shift: 7,
            },
            Probability {
                prob: 245,
                shift: 8,
            },
            Probability { prob: 15, shift: 4 },
            Probability {
                prob: 235,
                shift: 8,
            },
            Probability {
                prob: 229,
                shift: 8,
            },
            Probability { prob: 7, shift: 3 },
            Probability {
                prob: 219,
                shift: 8,
            },
            Probability {
                prob: 107,
                shift: 7,
            },
            Probability { prob: 13, shift: 4 },
            Probability {
                prob: 203,
                shift: 8,
            },
            Probability { prob: 99, shift: 7 },
            Probability {
                prob: 193,
                shift: 8,
            },
            Probability { prob: 47, shift: 6 },
            Probability { prob: 91, shift: 7 },
            Probability {
                prob: 177,
                shift: 8,
            },
            Probability { prob: 43, shift: 6 },
            Probability {
                prob: 167,
                shift: 8,
            },
            Probability {
                prob: 161,
                shift: 8,
            },
            Probability { prob: 39, shift: 6 },
            Probability {
                prob: 151,
                shift: 8,
            },
            Probability { prob: 73, shift: 7 },
            Probability {
                prob: 141,
                shift: 8,
            },
            Probability {
                prob: 135,
                shift: 8,
            },
            Probability { prob: 65, shift: 7 },
            Probability {
                prob: 125,
                shift: 8,
            },
            Probability { prob: 15, shift: 5 },
            Probability { prob: 57, shift: 7 },
            Probability {
                prob: 109,
                shift: 8,
            },
            Probability { prob: 13, shift: 5 },
            Probability { prob: 99, shift: 8 },
            Probability { prob: 47, shift: 7 },
            Probability { prob: 11, shift: 5 },
            Probability { prob: 83, shift: 8 },
            Probability { prob: 39, shift: 7 },
            Probability { prob: 73, shift: 8 },
            Probability { prob: 67, shift: 8 },
            Probability { prob: 31, shift: 7 },
            Probability { prob: 57, shift: 8 },
            Probability { prob: 13, shift: 6 },
            Probability { prob: 47, shift: 8 },
            Probability { prob: 41, shift: 8 },
            Probability { prob: 9, shift: 6 },
            Probability { prob: 31, shift: 8 },
            Probability { prob: 13, shift: 7 },
            Probability { prob: 5, shift: 6 },
            Probability { prob: 15, shift: 8 },
            Probability { prob: 5, shift: 7 },
            Probability { prob: 5, shift: 8 },
            Probability { prob: 5, shift: 8 },
        ];
        LOOKUP[self as usize]
    }

    #[inline]
    pub fn adapt(self, bit: bool, _rng: &mut SplitMix64) -> Self {
        const OUTCOMES: [BitContext; 2 * 1275] = [
            True0False1,
            True0False2,
            True1False1,
            True0False3,
            True1False2,
            True2False1,
            True0False4,
            True1False3,
            True2False2,
            True3False1,
            True0False5,
            True1False4,
            True2False3,
            True3False2,
            True4False1,
            True0False6,
            True1False5,
            True2False4,
            True3False3,
            True4False2,
            True5False1,
            True0False7,
            True1False6,
            True2False5,
            True3False4,
            True4False3,
            True5False2,
            True6False1,
            True0False8,
            True1False7,
            True2False6,
            True3False5,
            True4False4,
            True5False3,
            True6False2,
            True7False1,
            True0False9,
            True1False8,
            True2False7,
            True3False6,
            True4False5,
            True5False4,
            True6False3,
            True7False2,
            True8False1,
            True0False10,
            True1False9,
            True2False8,
            True3False7,
            True4False6,
            True5False5,
            True6False4,
            True7False3,
            True8False2,
            True9False1,
            True0False11,
            True1False10,
            True2False9,
            True3False8,
            True4False7,
            True5False6,
            True6False5,
            True7False4,
            True8False3,
            True9False2,
            True10False1,
            True0False12,
            True1False11,
            True2False10,
            True3False9,
            True4False8,
            True5False7,
            True6False6,
            True7False5,
            True8False4,
            True9False3,
            True10False2,
            True11False1,
            True0False13,
            True1False12,
            True2False11,
            True3False10,
            True4False9,
            True5False8,
            True6False7,
            True7False6,
            True8False5,
            True9False4,
            True10False3,
            True11False2,
            True12False1,
            True0False14,
            True1False13,
            True2False12,
            True3False11,
            True4False10,
            True5False9,
            True6False8,
            True7False7,
            True8False6,
            True9False5,
            True10False4,
            True11False3,
            True12False2,
            True13False1,
            True0False15,
            True1False14,
            True2False13,
            True3False12,
            True4False11,
            True5False10,
            True6False9,
            True7False8,
            True8False7,
            True9False6,
            True10False5,
            True11False4,
            True12False3,
            True13False2,
            True14False1,
            True0False16,
            True1False15,
            True2False14,
            True3False13,
            True4False12,
            True5False11,
            True6False10,
            True7False9,
            True8False8,
            True9False7,
            True10False6,
            True11False5,
            True12False4,
            True13False3,
            True14False2,
            True15False1,
            True0False17,
            True1False16,
            True2False15,
            True3False14,
            True4False13,
            True5False12,
            True6False11,
            True7False10,
            True8False9,
            True9False8,
            True10False7,
            True11False6,
            True12False5,
            True13False4,
            True14False3,
            True15False2,
            True16False1,
            True0False18,
            True1False17,
            True2False16,
            True3False15,
            True4False14,
            True5False13,
            True6False12,
            True7False11,
            True8False10,
            True9False9,
            True10False8,
            True11False7,
            True12False6,
            True13False5,
            True14False4,
            True15False3,
            True16False2,
            True17False1,
            True0False19,
            True1False18,
            True2False17,
            True3False16,
            True4False15,
            True5False14,
            True6False13,
            True7False12,
            True8False11,
            True9False10,
            True10False9,
            True11False8,
            True12False7,
            True13False6,
            True14False5,
            True15False4,
            True16False3,
            True17False2,
            True18False1,
            True0False20,
            True1False19,
            True2False18,
            True3False17,
            True4False16,
            True5False15,
            True6False14,
            True7False13,
            True8False12,
            True9False11,
            True10False10,
            True11False9,
            True12False8,
            True13False7,
            True14False6,
            True15False5,
            True16False4,
            True17False3,
            True18False2,
            True19False1,
            True0False21,
            True1False20,
            True2False19,
            True3False18,
            True4False17,
            True5False16,
            True6False15,
            True7False14,
            True8False13,
            True9False12,
            True10False11,
            True11False10,
            True12False9,
            True13False8,
            True14False7,
            True15False6,
            True16False5,
            True17False4,
            True18False3,
            True19False2,
            True20False1,
            True0False22,
            True1False21,
            True2False20,
            True3False19,
            True4False18,
            True5False17,
            True6False16,
            True7False15,
            True8False14,
            True9False13,
            True10False12,
            True11False11,
            True12False10,
            True13False9,
            True14False8,
            True15False7,
            True16False6,
            True17False5,
            True18False4,
            True19False3,
            True20False2,
            True21False1,
            True0False23,
            True1False22,
            True2False21,
            True3False20,
            True4False19,
            True5False18,
            True6False17,
            True7False16,
            True8False15,
            True9False14,
            True10False13,
            True11False12,
            True12False11,
            True13False10,
            True14False9,
            True15False8,
            True16False7,
            True17False6,
            True18False5,
            True19False4,
            True20False3,
            True21False2,
            True22False1,
            True0False24,
            True1False23,
            True2False22,
            True3False21,
            True4False20,
            True5False19,
            True6False18,
            True7False17,
            True8False16,
            True9False15,
            True10False14,
            True11False13,
            True12False12,
            True13False11,
            True14False10,
            True15False9,
            True16False8,
            True17False7,
            True18False6,
            True19False5,
            True20False4,
            True21False3,
            True22False2,
            True23False1,
            True0False25,
            True1False24,
            True2False23,
            True3False22,
            True4False21,
            True5False20,
            True6False19,
            True7False18,
            True8False17,
            True9False16,
            True10False15,
            True11False14,
            True12False13,
            True13False12,
            True14False11,
            True15False10,
            True16False9,
            True17False8,
            True18False7,
            True19False6,
            True20False5,
            True21False4,
            True22False3,
            True23False2,
            True24False1,
            True0False26,
            True1False25,
            True2False24,
            True3False23,
            True4False22,
            True5False21,
            True6False20,
            True7False19,
            True8False18,
            True9False17,
            True10False16,
            True11False15,
            True12False14,
            True13False13,
            True14False12,
            True15False11,
            True16False10,
            True17False9,
            True18False8,
            True19False7,
            True20False6,
            True21False5,
            True22False4,
            True23False3,
            True24False2,
            True25False1,
            True0False27,
            True1False26,
            True2False25,
            True3False24,
            True4False23,
            True5False22,
            True6False21,
            True7False20,
            True8False19,
            True9False18,
            True10False17,
            True11False16,
            True12False15,
            True13False14,
            True14False13,
            True15False12,
            True16False11,
            True17False10,
            True18False9,
            True19False8,
            True20False7,
            True21False6,
            True22False5,
            True23False4,
            True24False3,
            True25False2,
            True26False1,
            True0False28,
            True1False27,
            True2False26,
            True3False25,
            True4False24,
            True5False23,
            True6False22,
            True7False21,
            True8False20,
            True9False19,
            True10False18,
            True11False17,
            True12False16,
            True13False15,
            True14False14,
            True15False13,
            True16False12,
            True17False11,
            True18False10,
            True19False9,
            True20False8,
            True21False7,
            True22False6,
            True23False5,
            True24False4,
            True25False3,
            True26False2,
            True27False1,
            True0False29,
            True1False28,
            True2False27,
            True3False26,
            True4False25,
            True5False24,
            True6False23,
            True7False22,
            True8False21,
            True9False20,
            True10False19,
            True11False18,
            True12False17,
            True13False16,
            True14False15,
            True15False14,
            True16False13,
            True17False12,
            True18False11,
            True19False10,
            True20False9,
            True21False8,
            True22False7,
            True23False6,
            True24False5,
            True25False4,
            True26False3,
            True27False2,
            True28False1,
            True0False30,
            True1False29,
            True2False28,
            True3False27,
            True4False26,
            True5False25,
            True6False24,
            True7False23,
            True8False22,
            True9False21,
            True10False20,
            True11False19,
            True12False18,
            True13False17,
            True14False16,
            True15False15,
            True16False14,
            True17False13,
            True18False12,
            True19False11,
            True20False10,
            True21False9,
            True22False8,
            True23False7,
            True24False6,
            True25False5,
            True26False4,
            True27False3,
            True28False2,
            True29False1,
            True0False31,
            True1False30,
            True2False29,
            True3False28,
            True4False27,
            True5False26,
            True6False25,
            True7False24,
            True8False23,
            True9False22,
            True10False21,
            True11False20,
            True12False19,
            True13False18,
            True14False17,
            True15False16,
            True16False15,
            True17False14,
            True18False13,
            True19False12,
            True20False11,
            True21False10,
            True22False9,
            True23False8,
            True24False7,
            True25False6,
            True26False5,
            True27False4,
            True28False3,
            True29False2,
            True30False1,
            True0False32,
            True1False31,
            True2False30,
            True3False29,
            True4False28,
            True5False27,
            True6False26,
            True7False25,
            True8False24,
            True9False23,
            True10False22,
            True11False21,
            True12False20,
            True13False19,
            True14False18,
            True15False17,
            True16False16,
            True17False15,
            True18False14,
            True19False13,
            True20False12,
            True21False11,
            True22False10,
            True23False9,
            True24False8,
            True25False7,
            True26False6,
            True27False5,
            True28False4,
            True29False3,
            True30False2,
            True31False1,
            True0False33,
            True1False32,
            True2False31,
            True3False30,
            True4False29,
            True5False28,
            True6False27,
            True7False26,
            True8False25,
            True9False24,
            True10False23,
            True11False22,
            True12False21,
            True13False20,
            True14False19,
            True15False18,
            True16False17,
            True17False16,
            True18False15,
            True19False14,
            True20False13,
            True21False12,
            True22False11,
            True23False10,
            True24False9,
            True25False8,
            True26False7,
            True27False6,
            True28False5,
            True29False4,
            True30False3,
            True31False2,
            True32False1,
            True0False34,
            True1False33,
            True2False32,
            True3False31,
            True4False30,
            True5False29,
            True6False28,
            True7False27,
            True8False26,
            True9False25,
            True10False24,
            True11False23,
            True12False22,
            True13False21,
            True14False20,
            True15False19,
            True16False18,
            True17False17,
            True18False16,
            True19False15,
            True20False14,
            True21False13,
            True22False12,
            True23False11,
            True24False10,
            True25False9,
            True26False8,
            True27False7,
            True28False6,
            True29False5,
            True30False4,
            True31False3,
            True32False2,
            True33False1,
            True0False35,
            True1False34,
            True2False33,
            True3False32,
            True4False31,
            True5False30,
            True6False29,
            True7False28,
            True8False27,
            True9False26,
            True10False25,
            True11False24,
            True12False23,
            True13False22,
            True14False21,
            True15False20,
            True16False19,
            True17False18,
            True18False17,
            True19False16,
            True20False15,
            True21False14,
            True22False13,
            True23False12,
            True24False11,
            True25False10,
            True26False9,
            True27False8,
            True28False7,
            True29False6,
            True30False5,
            True31False4,
            True32False3,
            True33False2,
            True34False1,
            True0False36,
            True1False35,
            True2False34,
            True3False33,
            True4False32,
            True5False31,
            True6False30,
            True7False29,
            True8False28,
            True9False27,
            True10False26,
            True11False25,
            True12False24,
            True13False23,
            True14False22,
            True15False21,
            True16False20,
            True17False19,
            True18False18,
            True19False17,
            True20False16,
            True21False15,
            True22False14,
            True23False13,
            True24False12,
            True25False11,
            True26False10,
            True27False9,
            True28False8,
            True29False7,
            True30False6,
            True31False5,
            True32False4,
            True33False3,
            True34False2,
            True35False1,
            True0False37,
            True1False36,
            True2False35,
            True3False34,
            True4False33,
            True5False32,
            True6False31,
            True7False30,
            True8False29,
            True9False28,
            True10False27,
            True11False26,
            True12False25,
            True13False24,
            True14False23,
            True15False22,
            True16False21,
            True17False20,
            True18False19,
            True19False18,
            True20False17,
            True21False16,
            True22False15,
            True23False14,
            True24False13,
            True25False12,
            True26False11,
            True27False10,
            True28False9,
            True29False8,
            True30False7,
            True31False6,
            True32False5,
            True33False4,
            True34False3,
            True35False2,
            True36False1,
            True0False38,
            True1False37,
            True2False36,
            True3False35,
            True4False34,
            True5False33,
            True6False32,
            True7False31,
            True8False30,
            True9False29,
            True10False28,
            True11False27,
            True12False26,
            True13False25,
            True14False24,
            True15False23,
            True16False22,
            True17False21,
            True18False20,
            True19False19,
            True20False18,
            True21False17,
            True22False16,
            True23False15,
            True24False14,
            True25False13,
            True26False12,
            True27False11,
            True28False10,
            True29False9,
            True30False8,
            True31False7,
            True32False6,
            True33False5,
            True34False4,
            True35False3,
            True36False2,
            True37False1,
            True0False39,
            True1False38,
            True2False37,
            True3False36,
            True4False35,
            True5False34,
            True6False33,
            True7False32,
            True8False31,
            True9False30,
            True10False29,
            True11False28,
            True12False27,
            True13False26,
            True14False25,
            True15False24,
            True16False23,
            True17False22,
            True18False21,
            True19False20,
            True20False19,
            True21False18,
            True22False17,
            True23False16,
            True24False15,
            True25False14,
            True26False13,
            True27False12,
            True28False11,
            True29False10,
            True30False9,
            True31False8,
            True32False7,
            True33False6,
            True34False5,
            True35False4,
            True36False3,
            True37False2,
            True38False1,
            True0False40,
            True1False39,
            True2False38,
            True3False37,
            True4False36,
            True5False35,
            True6False34,
            True7False33,
            True8False32,
            True9False31,
            True10False30,
            True11False29,
            True12False28,
            True13False27,
            True14False26,
            True15False25,
            True16False24,
            True17False23,
            True18False22,
            True19False21,
            True20False20,
            True21False19,
            True22False18,
            True23False17,
            True24False16,
            True25False15,
            True26False14,
            True27False13,
            True28False12,
            True29False11,
            True30False10,
            True31False9,
            True32False8,
            True33False7,
            True34False6,
            True35False5,
            True36False4,
            True37False3,
            True38False2,
            True39False1,
            True0False41,
            True1False40,
            True2False39,
            True3False38,
            True4False37,
            True5False36,
            True6False35,
            True7False34,
            True8False33,
            True9False32,
            True10False31,
            True11False30,
            True12False29,
            True13False28,
            True14False27,
            True15False26,
            True16False25,
            True17False24,
            True18False23,
            True19False22,
            True20False21,
            True21False20,
            True22False19,
            True23False18,
            True24False17,
            True25False16,
            True26False15,
            True27False14,
            True28False13,
            True29False12,
            True30False11,
            True31False10,
            True32False9,
            True33False8,
            True34False7,
            True35False6,
            True36False5,
            True37False4,
            True38False3,
            True39False2,
            True40False1,
            True0False42,
            True1False41,
            True2False40,
            True3False39,
            True4False38,
            True5False37,
            True6False36,
            True7False35,
            True8False34,
            True9False33,
            True10False32,
            True11False31,
            True12False30,
            True13False29,
            True14False28,
            True15False27,
            True16False26,
            True17False25,
            True18False24,
            True19False23,
            True20False22,
            True21False21,
            True22False20,
            True23False19,
            True24False18,
            True25False17,
            True26False16,
            True27False15,
            True28False14,
            True29False13,
            True30False12,
            True31False11,
            True32False10,
            True33False9,
            True34False8,
            True35False7,
            True36False6,
            True37False5,
            True38False4,
            True39False3,
            True40False2,
            True41False1,
            True0False43,
            True1False42,
            True2False41,
            True3False40,
            True4False39,
            True5False38,
            True6False37,
            True7False36,
            True8False35,
            True9False34,
            True10False33,
            True11False32,
            True12False31,
            True13False30,
            True14False29,
            True15False28,
            True16False27,
            True17False26,
            True18False25,
            True19False24,
            True20False23,
            True21False22,
            True22False21,
            True23False20,
            True24False19,
            True25False18,
            True26False17,
            True27False16,
            True28False15,
            True29False14,
            True30False13,
            True31False12,
            True32False11,
            True33False10,
            True34False9,
            True35False8,
            True36False7,
            True37False6,
            True38False5,
            True39False4,
            True40False3,
            True41False2,
            True42False1,
            True0False44,
            True1False43,
            True2False42,
            True3False41,
            True4False40,
            True5False39,
            True6False38,
            True7False37,
            True8False36,
            True9False35,
            True10False34,
            True11False33,
            True12False32,
            True13False31,
            True14False30,
            True15False29,
            True16False28,
            True17False27,
            True18False26,
            True19False25,
            True20False24,
            True21False23,
            True22False22,
            True23False21,
            True24False20,
            True25False19,
            True26False18,
            True27False17,
            True28False16,
            True29False15,
            True30False14,
            True31False13,
            True32False12,
            True33False11,
            True34False10,
            True35False9,
            True36False8,
            True37False7,
            True38False6,
            True39False5,
            True40False4,
            True41False3,
            True42False2,
            True43False1,
            True0False45,
            True1False44,
            True2False43,
            True3False42,
            True4False41,
            True5False40,
            True6False39,
            True7False38,
            True8False37,
            True9False36,
            True10False35,
            True11False34,
            True12False33,
            True13False32,
            True14False31,
            True15False30,
            True16False29,
            True17False28,
            True18False27,
            True19False26,
            True20False25,
            True21False24,
            True22False23,
            True23False22,
            True24False21,
            True25False20,
            True26False19,
            True27False18,
            True28False17,
            True29False16,
            True30False15,
            True31False14,
            True32False13,
            True33False12,
            True34False11,
            True35False10,
            True36False9,
            True37False8,
            True38False7,
            True39False6,
            True40False5,
            True41False4,
            True42False3,
            True43False2,
            True44False1,
            True0False46,
            True1False45,
            True2False44,
            True3False43,
            True4False42,
            True5False41,
            True6False40,
            True7False39,
            True8False38,
            True9False37,
            True10False36,
            True11False35,
            True12False34,
            True13False33,
            True14False32,
            True15False31,
            True16False30,
            True17False29,
            True18False28,
            True19False27,
            True20False26,
            True21False25,
            True22False24,
            True23False23,
            True24False22,
            True25False21,
            True26False20,
            True27False19,
            True28False18,
            True29False17,
            True30False16,
            True31False15,
            True32False14,
            True33False13,
            True34False12,
            True35False11,
            True36False10,
            True37False9,
            True38False8,
            True39False7,
            True40False6,
            True41False5,
            True42False4,
            True43False3,
            True44False2,
            True45False1,
            True0False47,
            True1False46,
            True2False45,
            True3False44,
            True4False43,
            True5False42,
            True6False41,
            True7False40,
            True8False39,
            True9False38,
            True10False37,
            True11False36,
            True12False35,
            True13False34,
            True14False33,
            True15False32,
            True16False31,
            True17False30,
            True18False29,
            True19False28,
            True20False27,
            True21False26,
            True22False25,
            True23False24,
            True24False23,
            True25False22,
            True26False21,
            True27False20,
            True28False19,
            True29False18,
            True30False17,
            True31False16,
            True32False15,
            True33False14,
            True34False13,
            True35False12,
            True36False11,
            True37False10,
            True38False9,
            True39False8,
            True40False7,
            True41False6,
            True42False5,
            True43False4,
            True44False3,
            True45False2,
            True46False1,
            True0False48,
            True1False47,
            True2False46,
            True3False45,
            True4False44,
            True5False43,
            True6False42,
            True7False41,
            True8False40,
            True9False39,
            True10False38,
            True11False37,
            True12False36,
            True13False35,
            True14False34,
            True15False33,
            True16False32,
            True17False31,
            True18False30,
            True19False29,
            True20False28,
            True21False27,
            True22False26,
            True23False25,
            True24False24,
            True25False23,
            True26False22,
            True27False21,
            True28False20,
            True29False19,
            True30False18,
            True31False17,
            True32False16,
            True33False15,
            True34False14,
            True35False13,
            True36False12,
            True37False11,
            True38False10,
            True39False9,
            True40False8,
            True41False7,
            True42False6,
            True43False5,
            True44False4,
            True45False3,
            True46False2,
            True47False1,
            True0False49,
            True1False48,
            True2False47,
            True3False46,
            True4False45,
            True5False44,
            True6False43,
            True7False42,
            True8False41,
            True9False40,
            True10False39,
            True11False38,
            True12False37,
            True13False36,
            True14False35,
            True15False34,
            True16False33,
            True17False32,
            True18False31,
            True19False30,
            True20False29,
            True21False28,
            True22False27,
            True23False26,
            True24False25,
            True25False24,
            True26False23,
            True27False22,
            True28False21,
            True29False20,
            True30False19,
            True31False18,
            True32False17,
            True33False16,
            True34False15,
            True35False14,
            True36False13,
            True37False12,
            True38False11,
            True39False10,
            True40False9,
            True41False8,
            True42False7,
            True43False6,
            True44False5,
            True45False4,
            True46False3,
            True47False2,
            True48False1,
            True0False25,
            True0False24,
            True1False24,
            True1False23,
            True2False23,
            True2False22,
            True3False22,
            True3False21,
            True4False21,
            True4False20,
            True5False20,
            True5False19,
            True6False19,
            True6False18,
            True7False18,
            True7False17,
            True8False17,
            True8False16,
            True9False16,
            True9False15,
            True10False15,
            True10False14,
            True11False14,
            True11False13,
            True12False13,
            True12False12,
            True13False12,
            True13False11,
            True14False11,
            True14False10,
            True15False10,
            True15False9,
            True16False9,
            True16False8,
            True17False8,
            True17False7,
            True18False7,
            True18False6,
            True19False6,
            True19False5,
            True20False5,
            True20False4,
            True21False4,
            True21False3,
            True22False3,
            True22False2,
            True23False2,
            True23False1,
            True24False1,
            True24False0,
            True1False0,
            True1False1,
            True2False0,
            True1False2,
            True2False1,
            True3False0,
            True1False3,
            True2False2,
            True3False1,
            True4False0,
            True1False4,
            True2False3,
            True3False2,
            True4False1,
            True5False0,
            True1False5,
            True2False4,
            True3False3,
            True4False2,
            True5False1,
            True6False0,
            True1False6,
            True2False5,
            True3False4,
            True4False3,
            True5False2,
            True6False1,
            True7False0,
            True1False7,
            True2False6,
            True3False5,
            True4False4,
            True5False3,
            True6False2,
            True7False1,
            True8False0,
            True1False8,
            True2False7,
            True3False6,
            True4False5,
            True5False4,
            True6False3,
            True7False2,
            True8False1,
            True9False0,
            True1False9,
            True2False8,
            True3False7,
            True4False6,
            True5False5,
            True6False4,
            True7False3,
            True8False2,
            True9False1,
            True10False0,
            True1False10,
            True2False9,
            True3False8,
            True4False7,
            True5False6,
            True6False5,
            True7False4,
            True8False3,
            True9False2,
            True10False1,
            True11False0,
            True1False11,
            True2False10,
            True3False9,
            True4False8,
            True5False7,
            True6False6,
            True7False5,
            True8False4,
            True9False3,
            True10False2,
            True11False1,
            True12False0,
            True1False12,
            True2False11,
            True3False10,
            True4False9,
            True5False8,
            True6False7,
            True7False6,
            True8False5,
            True9False4,
            True10False3,
            True11False2,
            True12False1,
            True13False0,
            True1False13,
            True2False12,
            True3False11,
            True4False10,
            True5False9,
            True6False8,
            True7False7,
            True8False6,
            True9False5,
            True10False4,
            True11False3,
            True12False2,
            True13False1,
            True14False0,
            True1False14,
            True2False13,
            True3False12,
            True4False11,
            True5False10,
            True6False9,
            True7False8,
            True8False7,
            True9False6,
            True10False5,
            True11False4,
            True12False3,
            True13False2,
            True14False1,
            True15False0,
            True1False15,
            True2False14,
            True3False13,
            True4False12,
            True5False11,
            True6False10,
            True7False9,
            True8False8,
            True9False7,
            True10False6,
            True11False5,
            True12False4,
            True13False3,
            True14False2,
            True15False1,
            True16False0,
            True1False16,
            True2False15,
            True3False14,
            True4False13,
            True5False12,
            True6False11,
            True7False10,
            True8False9,
            True9False8,
            True10False7,
            True11False6,
            True12False5,
            True13False4,
            True14False3,
            True15False2,
            True16False1,
            True17False0,
            True1False17,
            True2False16,
            True3False15,
            True4False14,
            True5False13,
            True6False12,
            True7False11,
            True8False10,
            True9False9,
            True10False8,
            True11False7,
            True12False6,
            True13False5,
            True14False4,
            True15False3,
            True16False2,
            True17False1,
            True18False0,
            True1False18,
            True2False17,
            True3False16,
            True4False15,
            True5False14,
            True6False13,
            True7False12,
            True8False11,
            True9False10,
            True10False9,
            True11False8,
            True12False7,
            True13False6,
            True14False5,
            True15False4,
            True16False3,
            True17False2,
            True18False1,
            True19False0,
            True1False19,
            True2False18,
            True3False17,
            True4False16,
            True5False15,
            True6False14,
            True7False13,
            True8False12,
            True9False11,
            True10False10,
            True11False9,
            True12False8,
            True13False7,
            True14False6,
            True15False5,
            True16False4,
            True17False3,
            True18False2,
            True19False1,
            True20False0,
            True1False20,
            True2False19,
            True3False18,
            True4False17,
            True5False16,
            True6False15,
            True7False14,
            True8False13,
            True9False12,
            True10False11,
            True11False10,
            True12False9,
            True13False8,
            True14False7,
            True15False6,
            True16False5,
            True17False4,
            True18False3,
            True19False2,
            True20False1,
            True21False0,
            True1False21,
            True2False20,
            True3False19,
            True4False18,
            True5False17,
            True6False16,
            True7False15,
            True8False14,
            True9False13,
            True10False12,
            True11False11,
            True12False10,
            True13False9,
            True14False8,
            True15False7,
            True16False6,
            True17False5,
            True18False4,
            True19False3,
            True20False2,
            True21False1,
            True22False0,
            True1False22,
            True2False21,
            True3False20,
            True4False19,
            True5False18,
            True6False17,
            True7False16,
            True8False15,
            True9False14,
            True10False13,
            True11False12,
            True12False11,
            True13False10,
            True14False9,
            True15False8,
            True16False7,
            True17False6,
            True18False5,
            True19False4,
            True20False3,
            True21False2,
            True22False1,
            True23False0,
            True1False23,
            True2False22,
            True3False21,
            True4False20,
            True5False19,
            True6False18,
            True7False17,
            True8False16,
            True9False15,
            True10False14,
            True11False13,
            True12False12,
            True13False11,
            True14False10,
            True15False9,
            True16False8,
            True17False7,
            True18False6,
            True19False5,
            True20False4,
            True21False3,
            True22False2,
            True23False1,
            True24False0,
            True1False24,
            True2False23,
            True3False22,
            True4False21,
            True5False20,
            True6False19,
            True7False18,
            True8False17,
            True9False16,
            True10False15,
            True11False14,
            True12False13,
            True13False12,
            True14False11,
            True15False10,
            True16False9,
            True17False8,
            True18False7,
            True19False6,
            True20False5,
            True21False4,
            True22False3,
            True23False2,
            True24False1,
            True25False0,
            True1False25,
            True2False24,
            True3False23,
            True4False22,
            True5False21,
            True6False20,
            True7False19,
            True8False18,
            True9False17,
            True10False16,
            True11False15,
            True12False14,
            True13False13,
            True14False12,
            True15False11,
            True16False10,
            True17False9,
            True18False8,
            True19False7,
            True20False6,
            True21False5,
            True22False4,
            True23False3,
            True24False2,
            True25False1,
            True26False0,
            True1False26,
            True2False25,
            True3False24,
            True4False23,
            True5False22,
            True6False21,
            True7False20,
            True8False19,
            True9False18,
            True10False17,
            True11False16,
            True12False15,
            True13False14,
            True14False13,
            True15False12,
            True16False11,
            True17False10,
            True18False9,
            True19False8,
            True20False7,
            True21False6,
            True22False5,
            True23False4,
            True24False3,
            True25False2,
            True26False1,
            True27False0,
            True1False27,
            True2False26,
            True3False25,
            True4False24,
            True5False23,
            True6False22,
            True7False21,
            True8False20,
            True9False19,
            True10False18,
            True11False17,
            True12False16,
            True13False15,
            True14False14,
            True15False13,
            True16False12,
            True17False11,
            True18False10,
            True19False9,
            True20False8,
            True21False7,
            True22False6,
            True23False5,
            True24False4,
            True25False3,
            True26False2,
            True27False1,
            True28False0,
            True1False28,
            True2False27,
            True3False26,
            True4False25,
            True5False24,
            True6False23,
            True7False22,
            True8False21,
            True9False20,
            True10False19,
            True11False18,
            True12False17,
            True13False16,
            True14False15,
            True15False14,
            True16False13,
            True17False12,
            True18False11,
            True19False10,
            True20False9,
            True21False8,
            True22False7,
            True23False6,
            True24False5,
            True25False4,
            True26False3,
            True27False2,
            True28False1,
            True29False0,
            True1False29,
            True2False28,
            True3False27,
            True4False26,
            True5False25,
            True6False24,
            True7False23,
            True8False22,
            True9False21,
            True10False20,
            True11False19,
            True12False18,
            True13False17,
            True14False16,
            True15False15,
            True16False14,
            True17False13,
            True18False12,
            True19False11,
            True20False10,
            True21False9,
            True22False8,
            True23False7,
            True24False6,
            True25False5,
            True26False4,
            True27False3,
            True28False2,
            True29False1,
            True30False0,
            True1False30,
            True2False29,
            True3False28,
            True4False27,
            True5False26,
            True6False25,
            True7False24,
            True8False23,
            True9False22,
            True10False21,
            True11False20,
            True12False19,
            True13False18,
            True14False17,
            True15False16,
            True16False15,
            True17False14,
            True18False13,
            True19False12,
            True20False11,
            True21False10,
            True22False9,
            True23False8,
            True24False7,
            True25False6,
            True26False5,
            True27False4,
            True28False3,
            True29False2,
            True30False1,
            True31False0,
            True1False31,
            True2False30,
            True3False29,
            True4False28,
            True5False27,
            True6False26,
            True7False25,
            True8False24,
            True9False23,
            True10False22,
            True11False21,
            True12False20,
            True13False19,
            True14False18,
            True15False17,
            True16False16,
            True17False15,
            True18False14,
            True19False13,
            True20False12,
            True21False11,
            True22False10,
            True23False9,
            True24False8,
            True25False7,
            True26False6,
            True27False5,
            True28False4,
            True29False3,
            True30False2,
            True31False1,
            True32False0,
            True1False32,
            True2False31,
            True3False30,
            True4False29,
            True5False28,
            True6False27,
            True7False26,
            True8False25,
            True9False24,
            True10False23,
            True11False22,
            True12False21,
            True13False20,
            True14False19,
            True15False18,
            True16False17,
            True17False16,
            True18False15,
            True19False14,
            True20False13,
            True21False12,
            True22False11,
            True23False10,
            True24False9,
            True25False8,
            True26False7,
            True27False6,
            True28False5,
            True29False4,
            True30False3,
            True31False2,
            True32False1,
            True33False0,
            True1False33,
            True2False32,
            True3False31,
            True4False30,
            True5False29,
            True6False28,
            True7False27,
            True8False26,
            True9False25,
            True10False24,
            True11False23,
            True12False22,
            True13False21,
            True14False20,
            True15False19,
            True16False18,
            True17False17,
            True18False16,
            True19False15,
            True20False14,
            True21False13,
            True22False12,
            True23False11,
            True24False10,
            True25False9,
            True26False8,
            True27False7,
            True28False6,
            True29False5,
            True30False4,
            True31False3,
            True32False2,
            True33False1,
            True34False0,
            True1False34,
            True2False33,
            True3False32,
            True4False31,
            True5False30,
            True6False29,
            True7False28,
            True8False27,
            True9False26,
            True10False25,
            True11False24,
            True12False23,
            True13False22,
            True14False21,
            True15False20,
            True16False19,
            True17False18,
            True18False17,
            True19False16,
            True20False15,
            True21False14,
            True22False13,
            True23False12,
            True24False11,
            True25False10,
            True26False9,
            True27False8,
            True28False7,
            True29False6,
            True30False5,
            True31False4,
            True32False3,
            True33False2,
            True34False1,
            True35False0,
            True1False35,
            True2False34,
            True3False33,
            True4False32,
            True5False31,
            True6False30,
            True7False29,
            True8False28,
            True9False27,
            True10False26,
            True11False25,
            True12False24,
            True13False23,
            True14False22,
            True15False21,
            True16False20,
            True17False19,
            True18False18,
            True19False17,
            True20False16,
            True21False15,
            True22False14,
            True23False13,
            True24False12,
            True25False11,
            True26False10,
            True27False9,
            True28False8,
            True29False7,
            True30False6,
            True31False5,
            True32False4,
            True33False3,
            True34False2,
            True35False1,
            True36False0,
            True1False36,
            True2False35,
            True3False34,
            True4False33,
            True5False32,
            True6False31,
            True7False30,
            True8False29,
            True9False28,
            True10False27,
            True11False26,
            True12False25,
            True13False24,
            True14False23,
            True15False22,
            True16False21,
            True17False20,
            True18False19,
            True19False18,
            True20False17,
            True21False16,
            True22False15,
            True23False14,
            True24False13,
            True25False12,
            True26False11,
            True27False10,
            True28False9,
            True29False8,
            True30False7,
            True31False6,
            True32False5,
            True33False4,
            True34False3,
            True35False2,
            True36False1,
            True37False0,
            True1False37,
            True2False36,
            True3False35,
            True4False34,
            True5False33,
            True6False32,
            True7False31,
            True8False30,
            True9False29,
            True10False28,
            True11False27,
            True12False26,
            True13False25,
            True14False24,
            True15False23,
            True16False22,
            True17False21,
            True18False20,
            True19False19,
            True20False18,
            True21False17,
            True22False16,
            True23False15,
            True24False14,
            True25False13,
            True26False12,
            True27False11,
            True28False10,
            True29False9,
            True30False8,
            True31False7,
            True32False6,
            True33False5,
            True34False4,
            True35False3,
            True36False2,
            True37False1,
            True38False0,
            True1False38,
            True2False37,
            True3False36,
            True4False35,
            True5False34,
            True6False33,
            True7False32,
            True8False31,
            True9False30,
            True10False29,
            True11False28,
            True12False27,
            True13False26,
            True14False25,
            True15False24,
            True16False23,
            True17False22,
            True18False21,
            True19False20,
            True20False19,
            True21False18,
            True22False17,
            True23False16,
            True24False15,
            True25False14,
            True26False13,
            True27False12,
            True28False11,
            True29False10,
            True30False9,
            True31False8,
            True32False7,
            True33False6,
            True34False5,
            True35False4,
            True36False3,
            True37False2,
            True38False1,
            True39False0,
            True1False39,
            True2False38,
            True3False37,
            True4False36,
            True5False35,
            True6False34,
            True7False33,
            True8False32,
            True9False31,
            True10False30,
            True11False29,
            True12False28,
            True13False27,
            True14False26,
            True15False25,
            True16False24,
            True17False23,
            True18False22,
            True19False21,
            True20False20,
            True21False19,
            True22False18,
            True23False17,
            True24False16,
            True25False15,
            True26False14,
            True27False13,
            True28False12,
            True29False11,
            True30False10,
            True31False9,
            True32False8,
            True33False7,
            True34False6,
            True35False5,
            True36False4,
            True37False3,
            True38False2,
            True39False1,
            True40False0,
            True1False40,
            True2False39,
            True3False38,
            True4False37,
            True5False36,
            True6False35,
            True7False34,
            True8False33,
            True9False32,
            True10False31,
            True11False30,
            True12False29,
            True13False28,
            True14False27,
            True15False26,
            True16False25,
            True17False24,
            True18False23,
            True19False22,
            True20False21,
            True21False20,
            True22False19,
            True23False18,
            True24False17,
            True25False16,
            True26False15,
            True27False14,
            True28False13,
            True29False12,
            True30False11,
            True31False10,
            True32False9,
            True33False8,
            True34False7,
            True35False6,
            True36False5,
            True37False4,
            True38False3,
            True39False2,
            True40False1,
            True41False0,
            True1False41,
            True2False40,
            True3False39,
            True4False38,
            True5False37,
            True6False36,
            True7False35,
            True8False34,
            True9False33,
            True10False32,
            True11False31,
            True12False30,
            True13False29,
            True14False28,
            True15False27,
            True16False26,
            True17False25,
            True18False24,
            True19False23,
            True20False22,
            True21False21,
            True22False20,
            True23False19,
            True24False18,
            True25False17,
            True26False16,
            True27False15,
            True28False14,
            True29False13,
            True30False12,
            True31False11,
            True32False10,
            True33False9,
            True34False8,
            True35False7,
            True36False6,
            True37False5,
            True38False4,
            True39False3,
            True40False2,
            True41False1,
            True42False0,
            True1False42,
            True2False41,
            True3False40,
            True4False39,
            True5False38,
            True6False37,
            True7False36,
            True8False35,
            True9False34,
            True10False33,
            True11False32,
            True12False31,
            True13False30,
            True14False29,
            True15False28,
            True16False27,
            True17False26,
            True18False25,
            True19False24,
            True20False23,
            True21False22,
            True22False21,
            True23False20,
            True24False19,
            True25False18,
            True26False17,
            True27False16,
            True28False15,
            True29False14,
            True30False13,
            True31False12,
            True32False11,
            True33False10,
            True34False9,
            True35False8,
            True36False7,
            True37False6,
            True38False5,
            True39False4,
            True40False3,
            True41False2,
            True42False1,
            True43False0,
            True1False43,
            True2False42,
            True3False41,
            True4False40,
            True5False39,
            True6False38,
            True7False37,
            True8False36,
            True9False35,
            True10False34,
            True11False33,
            True12False32,
            True13False31,
            True14False30,
            True15False29,
            True16False28,
            True17False27,
            True18False26,
            True19False25,
            True20False24,
            True21False23,
            True22False22,
            True23False21,
            True24False20,
            True25False19,
            True26False18,
            True27False17,
            True28False16,
            True29False15,
            True30False14,
            True31False13,
            True32False12,
            True33False11,
            True34False10,
            True35False9,
            True36False8,
            True37False7,
            True38False6,
            True39False5,
            True40False4,
            True41False3,
            True42False2,
            True43False1,
            True44False0,
            True1False44,
            True2False43,
            True3False42,
            True4False41,
            True5False40,
            True6False39,
            True7False38,
            True8False37,
            True9False36,
            True10False35,
            True11False34,
            True12False33,
            True13False32,
            True14False31,
            True15False30,
            True16False29,
            True17False28,
            True18False27,
            True19False26,
            True20False25,
            True21False24,
            True22False23,
            True23False22,
            True24False21,
            True25False20,
            True26False19,
            True27False18,
            True28False17,
            True29False16,
            True30False15,
            True31False14,
            True32False13,
            True33False12,
            True34False11,
            True35False10,
            True36False9,
            True37False8,
            True38False7,
            True39False6,
            True40False5,
            True41False4,
            True42False3,
            True43False2,
            True44False1,
            True45False0,
            True1False45,
            True2False44,
            True3False43,
            True4False42,
            True5False41,
            True6False40,
            True7False39,
            True8False38,
            True9False37,
            True10False36,
            True11False35,
            True12False34,
            True13False33,
            True14False32,
            True15False31,
            True16False30,
            True17False29,
            True18False28,
            True19False27,
            True20False26,
            True21False25,
            True22False24,
            True23False23,
            True24False22,
            True25False21,
            True26False20,
            True27False19,
            True28False18,
            True29False17,
            True30False16,
            True31False15,
            True32False14,
            True33False13,
            True34False12,
            True35False11,
            True36False10,
            True37False9,
            True38False8,
            True39False7,
            True40False6,
            True41False5,
            True42False4,
            True43False3,
            True44False2,
            True45False1,
            True46False0,
            True1False46,
            True2False45,
            True3False44,
            True4False43,
            True5False42,
            True6False41,
            True7False40,
            True8False39,
            True9False38,
            True10False37,
            True11False36,
            True12False35,
            True13False34,
            True14False33,
            True15False32,
            True16False31,
            True17False30,
            True18False29,
            True19False28,
            True20False27,
            True21False26,
            True22False25,
            True23False24,
            True24False23,
            True25False22,
            True26False21,
            True27False20,
            True28False19,
            True29False18,
            True30False17,
            True31False16,
            True32False15,
            True33False14,
            True34False13,
            True35False12,
            True36False11,
            True37False10,
            True38False9,
            True39False8,
            True40False7,
            True41False6,
            True42False5,
            True43False4,
            True44False3,
            True45False2,
            True46False1,
            True47False0,
            True1False47,
            True2False46,
            True3False45,
            True4False44,
            True5False43,
            True6False42,
            True7False41,
            True8False40,
            True9False39,
            True10False38,
            True11False37,
            True12False36,
            True13False35,
            True14False34,
            True15False33,
            True16False32,
            True17False31,
            True18False30,
            True19False29,
            True20False28,
            True21False27,
            True22False26,
            True23False25,
            True24False24,
            True25False23,
            True26False22,
            True27False21,
            True28False20,
            True29False19,
            True30False18,
            True31False17,
            True32False16,
            True33False15,
            True34False14,
            True35False13,
            True36False12,
            True37False11,
            True38False10,
            True39False9,
            True40False8,
            True41False7,
            True42False6,
            True43False5,
            True44False4,
            True45False3,
            True46False2,
            True47False1,
            True48False0,
            True1False48,
            True2False47,
            True3False46,
            True4False45,
            True5False44,
            True6False43,
            True7False42,
            True8False41,
            True9False40,
            True10False39,
            True11False38,
            True12False37,
            True13False36,
            True14False35,
            True15False34,
            True16False33,
            True17False32,
            True18False31,
            True19False30,
            True20False29,
            True21False28,
            True22False27,
            True23False26,
            True24False25,
            True25False24,
            True26False23,
            True27False22,
            True28False21,
            True29False20,
            True30False19,
            True31False18,
            True32False17,
            True33False16,
            True34False15,
            True35False14,
            True36False13,
            True37False12,
            True38False11,
            True39False10,
            True40False9,
            True41False8,
            True42False7,
            True43False6,
            True44False5,
            True45False4,
            True46False3,
            True47False2,
            True48False1,
            True49False0,
            True0False24,
            True1False24,
            True1False23,
            True2False23,
            True2False22,
            True3False22,
            True3False21,
            True4False21,
            True4False20,
            True5False20,
            True5False19,
            True6False19,
            True6False18,
            True7False18,
            True7False17,
            True8False17,
            True8False16,
            True9False16,
            True9False15,
            True10False15,
            True10False14,
            True11False14,
            True11False13,
            True12False13,
            True12False12,
            True13False12,
            True13False11,
            True14False11,
            True14False10,
            True15False10,
            True15False9,
            True16False9,
            True16False8,
            True17False8,
            True17False7,
            True18False7,
            True18False6,
            True19False6,
            True19False5,
            True20False5,
            True20False4,
            True21False4,
            True21False3,
            True22False3,
            True22False2,
            True23False2,
            True23False1,
            True24False1,
            True24False0,
            True25False0,
        ];
        OUTCOMES[(self as usize) + (bit as usize) * 1275]
    }
}
// Count of variants: 1275
