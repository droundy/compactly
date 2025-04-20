//! Generated with `src/v1/bit-context.sh`
use super::arith::Probability;

impl BitContext {
    pub const CONFIDENT: Self = True0False4;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitContext {
    #[default]
    True0False0, // 0.5
    True0False1,   // 0.66796875
    True1False0,   // 0.33203125
    True0False2,   // 0.75
    True1False1,   // 0.5
    True2False0,   // 0.25
    True0False3,   // 0.80078125
    True1False2,   // 0.6015625
    True2False1,   // 0.3984375
    True3False0,   // 0.19921875
    True0False4,   // 0.8359375
    True1False3,   // 0.66796875
    True2False2,   // 0.5
    True3False1,   // 0.33203125
    True4False0,   // 0.1640625
    True0False5,   // 0.859375
    True1False4,   // 0.71484375
    True2False3,   // 0.5703125
    True3False2,   // 0.4296875
    True4False1,   // 0.28515625
    True5False0,   // 0.140625
    True0False6,   // 0.875
    True1False5,   // 0.75
    True2False4,   // 0.625
    True3False3,   // 0.5
    True4False2,   // 0.375
    True5False1,   // 0.25
    True6False0,   // 0.125
    True0False7,   // 0.890625
    True1False6,   // 0.77734375
    True2False5,   // 0.66796875
    True3False4,   // 0.5546875
    True4False3,   // 0.4453125
    True5False2,   // 0.33203125
    True6False1,   // 0.22265625
    True7False0,   // 0.109375
    True0False8,   // 0.90234375
    True1False7,   // 0.80078125
    True2False6,   // 0.69921875
    True3False5,   // 0.6015625
    True4False4,   // 0.5
    True5False3,   // 0.3984375
    True6False2,   // 0.30078125
    True7False1,   // 0.19921875
    True8False0,   // 0.09765625
    True0False9,   // 0.91015625
    True1False8,   // 0.81640625
    True2False7,   // 0.7265625
    True3False6,   // 0.63671875
    True4False5,   // 0.546875
    True5False4,   // 0.453125
    True6False3,   // 0.36328125
    True7False2,   // 0.2734375
    True8False1,   // 0.18359375
    True9False0,   // 0.08984375
    True0False10,  // 0.91796875
    True1False9,   // 0.83203125
    True2False8,   // 0.75
    True3False7,   // 0.66796875
    True4False6,   // 0.58203125
    True5False5,   // 0.5
    True6False4,   // 0.41796875
    True7False3,   // 0.33203125
    True8False2,   // 0.25
    True9False1,   // 0.16796875
    True10False0,  // 0.08203125
    True0False11,  // 0.92578125
    True1False10,  // 0.84765625
    True2False9,   // 0.76953125
    True3False8,   // 0.69140625
    True4False7,   // 0.6171875
    True5False6,   // 0.5390625
    True6False5,   // 0.4609375
    True7False4,   // 0.3828125
    True8False3,   // 0.30859375
    True9False2,   // 0.23046875
    True10False1,  // 0.15234375
    True11False0,  // 0.07421875
    True0False12,  // 0.9296875
    True1False11,  // 0.85546875
    True2False10,  // 0.78515625
    True3False9,   // 0.71484375
    True4False8,   // 0.64453125
    True5False7,   // 0.5703125
    True6False6,   // 0.5
    True7False5,   // 0.4296875
    True8False4,   // 0.35546875
    True9False3,   // 0.28515625
    True10False2,  // 0.21484375
    True11False1,  // 0.14453125
    True12False0,  // 0.0703125
    True0False13,  // 0.93359375
    True1False12,  // 0.8671875
    True2False11,  // 0.80078125
    True3False10,  // 0.734375
    True4False9,   // 0.66796875
    True5False8,   // 0.6015625
    True6False7,   // 0.53515625
    True7False6,   // 0.46484375
    True8False5,   // 0.3984375
    True9False4,   // 0.33203125
    True10False3,  // 0.265625
    True11False2,  // 0.19921875
    True12False1,  // 0.1328125
    True13False0,  // 0.06640625
    True0False14,  // 0.9375
    True1False13,  // 0.875
    True2False12,  // 0.8125
    True3False11,  // 0.75
    True4False10,  // 0.6875
    True5False9,   // 0.625
    True6False8,   // 0.5625
    True7False7,   // 0.5
    True8False6,   // 0.4375
    True9False5,   // 0.375
    True10False4,  // 0.3125
    True11False3,  // 0.25
    True12False2,  // 0.1875
    True13False1,  // 0.125
    True14False0,  // 0.0625
    True0False15,  // 0.94140625
    True1False14,  // 0.8828125
    True2False13,  // 0.82421875
    True3False12,  // 0.765625
    True4False11,  // 0.70703125
    True5False10,  // 0.6484375
    True6False9,   // 0.58984375
    True7False8,   // 0.53125
    True8False7,   // 0.46875
    True9False6,   // 0.41015625
    True10False5,  // 0.3515625
    True11False4,  // 0.29296875
    True12False3,  // 0.234375
    True13False2,  // 0.17578125
    True14False1,  // 0.1171875
    True15False0,  // 0.05859375
    True0False16,  // 0.9453125
    True1False15,  // 0.890625
    True2False14,  // 0.83203125
    True3False13,  // 0.77734375
    True4False12,  // 0.72265625
    True5False11,  // 0.66796875
    True6False10,  // 0.609375
    True7False9,   // 0.5546875
    True8False8,   // 0.5
    True9False7,   // 0.4453125
    True10False6,  // 0.390625
    True11False5,  // 0.33203125
    True12False4,  // 0.27734375
    True13False3,  // 0.22265625
    True14False2,  // 0.16796875
    True15False1,  // 0.109375
    True16False0,  // 0.0546875
    True0False17,  // 0.94921875
    True1False16,  // 0.89453125
    True2False15,  // 0.84375
    True3False14,  // 0.7890625
    True4False13,  // 0.73828125
    True5False12,  // 0.68359375
    True6False11,  // 0.6328125
    True7False10,  // 0.578125
    True8False9,   // 0.52734375
    True9False8,   // 0.47265625
    True10False7,  // 0.421875
    True11False6,  // 0.3671875
    True12False5,  // 0.31640625
    True13False4,  // 0.26171875
    True14False3,  // 0.2109375
    True15False2,  // 0.15625
    True16False1,  // 0.10546875
    True17False0,  // 0.05078125
    True0False18,  // 0.953125
    True1False17,  // 0.8984375
    True2False16,  // 0.8515625
    True3False15,  // 0.80078125
    True4False14,  // 0.75
    True5False13,  // 0.69921875
    True6False12,  // 0.6484375
    True7False11,  // 0.6015625
    True8False10,  // 0.55078125
    True9False9,   // 0.5
    True10False8,  // 0.44921875
    True11False7,  // 0.3984375
    True12False6,  // 0.3515625
    True13False5,  // 0.30078125
    True14False4,  // 0.25
    True15False3,  // 0.19921875
    True16False2,  // 0.1484375
    True17False1,  // 0.1015625
    True18False0,  // 0.046875
    True0False19,  // 0.953125
    True1False18,  // 0.90625
    True2False17,  // 0.85546875
    True3False16,  // 0.80859375
    True4False15,  // 0.76171875
    True5False14,  // 0.71484375
    True6False13,  // 0.66796875
    True7False12,  // 0.6171875
    True8False11,  // 0.5703125
    True9False10,  // 0.5234375
    True10False9,  // 0.4765625
    True11False8,  // 0.4296875
    True12False7,  // 0.3828125
    True13False6,  // 0.33203125
    True14False5,  // 0.28515625
    True15False4,  // 0.23828125
    True16False3,  // 0.19140625
    True17False2,  // 0.14453125
    True18False1,  // 0.09375
    True19False0,  // 0.046875
    True0False20,  // 0.95703125
    True1False19,  // 0.91015625
    True2False18,  // 0.86328125
    True3False17,  // 0.81640625
    True4False16,  // 0.7734375
    True5False15,  // 0.7265625
    True6False14,  // 0.68359375
    True7False13,  // 0.63671875
    True8False12,  // 0.58984375
    True9False11,  // 0.546875
    True10False10, // 0.5
    True11False9,  // 0.453125
    True12False8,  // 0.41015625
    True13False7,  // 0.36328125
    True14False6,  // 0.31640625
    True15False5,  // 0.2734375
    True16False4,  // 0.2265625
    True17False3,  // 0.18359375
    True18False2,  // 0.13671875
    True19False1,  // 0.08984375
    True20False0,  // 0.04296875
    True0False21,  // 0.95703125
    True1False20,  // 0.9140625
    True2False19,  // 0.87109375
    True3False18,  // 0.82421875
    True4False17,  // 0.78125
    True5False16,  // 0.73828125
    True6False15,  // 0.6953125
    True7False14,  // 0.65234375
    True8False13,  // 0.609375
    True9False12,  // 0.56640625
    True10False11, // 0.5234375
    True11False10, // 0.4765625
    True12False9,  // 0.43359375
    True13False8,  // 0.390625
    True14False7,  // 0.34765625
    True15False6,  // 0.3046875
    True16False5,  // 0.26171875
    True17False4,  // 0.21875
    True18False3,  // 0.17578125
    True19False2,  // 0.12890625
    True20False1,  // 0.0859375
    True21False0,  // 0.04296875
    True0False22,  // 0.9609375
    True1False21,  // 0.91796875
    True2False20,  // 0.875
    True3False19,  // 0.83203125
    True4False18,  // 0.79296875
    True5False17,  // 0.75
    True6False16,  // 0.70703125
    True7False15,  // 0.66796875
    True8False14,  // 0.625
    True9False13,  // 0.58203125
    True10False12, // 0.54296875
    True11False11, // 0.5
    True12False10, // 0.45703125
    True13False9,  // 0.41796875
    True14False8,  // 0.375
    True15False7,  // 0.33203125
    True16False6,  // 0.29296875
    True17False5,  // 0.25
    True18False4,  // 0.20703125
    True19False3,  // 0.16796875
    True20False2,  // 0.125
    True21False1,  // 0.08203125
    True22False0,  // 0.0390625
    True0False23,  // 0.9609375
    True1False22,  // 0.921875
    True2False21,  // 0.87890625
    True3False20,  // 0.83984375
    True4False19,  // 0.80078125
    True5False18,  // 0.76171875
    True6False17,  // 0.71875
    True7False16,  // 0.6796875
    True8False15,  // 0.640625
    True9False14,  // 0.6015625
    True10False13, // 0.55859375
    True11False12, // 0.51953125
    True12False11, // 0.48046875
    True13False10, // 0.44140625
    True14False9,  // 0.3984375
    True15False8,  // 0.359375
    True16False7,  // 0.3203125
    True17False6,  // 0.28125
    True18False5,  // 0.23828125
    True19False4,  // 0.19921875
    True20False3,  // 0.16015625
    True21False2,  // 0.12109375
    True22False1,  // 0.078125
    True23False0,  // 0.0390625
    True0False24,  // 0.96484375
    True1False23,  // 0.921875
    True2False22,  // 0.8828125
    True3False21,  // 0.84765625
    True4False20,  // 0.80859375
    True5False19,  // 0.76953125
    True6False18,  // 0.73046875
    True7False17,  // 0.69140625
    True8False16,  // 0.65234375
    True9False15,  // 0.6171875
    True10False14, // 0.578125
    True11False13, // 0.5390625
    True12False12, // 0.5
    True13False11, // 0.4609375
    True14False10, // 0.421875
    True15False9,  // 0.3828125
    True16False8,  // 0.34765625
    True17False7,  // 0.30859375
    True18False6,  // 0.26953125
    True19False5,  // 0.23046875
    True20False4,  // 0.19140625
    True21False3,  // 0.15234375
    True22False2,  // 0.1171875
    True23False1,  // 0.078125
    True24False0,  // 0.03515625
    True0False25,  // 0.96484375
    True1False24,  // 0.92578125
    True2False23,  // 0.890625
    True3False22,  // 0.8515625
    True4False21,  // 0.81640625
    True5False20,  // 0.77734375
    True6False19,  // 0.7421875
    True7False18,  // 0.703125
    True8False17,  // 0.66796875
    True9False16,  // 0.62890625
    True10False15, // 0.59375
    True11False14, // 0.5546875
    True12False13, // 0.51953125
    True13False12, // 0.48046875
    True14False11, // 0.4453125
    True15False10, // 0.40625
    True16False9,  // 0.37109375
    True17False8,  // 0.33203125
    True18False7,  // 0.296875
    True19False6,  // 0.2578125
    True20False5,  // 0.22265625
    True21False4,  // 0.18359375
    True22False3,  // 0.1484375
    True23False2,  // 0.109375
    True24False1,  // 0.07421875
    True25False0,  // 0.03515625
    True0False26,  // 0.96484375
    True1False25,  // 0.9296875
    True2False24,  // 0.89453125
    True3False23,  // 0.85546875
    True4False22,  // 0.8203125
    True5False21,  // 0.78515625
    True6False20,  // 0.75
    True7False19,  // 0.71484375
    True8False18,  // 0.6796875
    True9False17,  // 0.64453125
    True10False16, // 0.60546875
    True11False15, // 0.5703125
    True12False14, // 0.53515625
    True13False13, // 0.5
    True14False12, // 0.46484375
    True15False11, // 0.4296875
    True16False10, // 0.39453125
    True17False9,  // 0.35546875
    True18False8,  // 0.3203125
    True19False7,  // 0.28515625
    True20False6,  // 0.25
    True21False5,  // 0.21484375
    True22False4,  // 0.1796875
    True23False3,  // 0.14453125
    True24False2,  // 0.10546875
    True25False1,  // 0.0703125
    True26False0,  // 0.03515625
    True0False27,  // 0.96875
    True1False26,  // 0.9296875
    True2False25,  // 0.8984375
    True3False24,  // 0.86328125
    True4False23,  // 0.828125
    True5False22,  // 0.79296875
    True6False21,  // 0.7578125
    True7False20,  // 0.72265625
    True8False19,  // 0.69140625
    True9False18,  // 0.65625
    True10False17, // 0.62109375
    True11False16, // 0.5859375
    True12False15, // 0.55078125
    True13False14, // 0.515625
    True14False13, // 0.484375
    True15False12, // 0.44921875
    True16False11, // 0.4140625
    True17False10, // 0.37890625
    True18False9,  // 0.34375
    True19False8,  // 0.30859375
    True20False7,  // 0.27734375
    True21False6,  // 0.2421875
    True22False5,  // 0.20703125
    True23False4,  // 0.171875
    True24False3,  // 0.13671875
    True25False2,  // 0.1015625
    True26False1,  // 0.0703125
    True27False0,  // 0.03125
    True0False28,  // 0.96875
    True1False27,  // 0.93359375
    True2False26,  // 0.8984375
    True3False25,  // 0.8671875
    True4False24,  // 0.83203125
    True5False23,  // 0.80078125
    True6False22,  // 0.765625
    True7False21,  // 0.734375
    True8False20,  // 0.69921875
    True9False19,  // 0.66796875
    True10False18, // 0.6328125
    True11False17, // 0.6015625
    True12False16, // 0.56640625
    True13False15, // 0.53515625
    True14False14, // 0.5
    True15False13, // 0.46484375
    True16False12, // 0.43359375
    True17False11, // 0.3984375
    True18False10, // 0.3671875
    True19False9,  // 0.33203125
    True20False8,  // 0.30078125
    True21False7,  // 0.265625
    True22False6,  // 0.234375
    True23False5,  // 0.19921875
    True24False4,  // 0.16796875
    True25False3,  // 0.1328125
    True26False2,  // 0.1015625
    True27False1,  // 0.06640625
    True28False0,  // 0.03125
    True0False29,  // 0.96875
    True1False28,  // 0.93359375
    True2False27,  // 0.90234375
    True3False26,  // 0.87109375
    True4False25,  // 0.83984375
    True5False24,  // 0.8046875
    True6False23,  // 0.7734375
    True7False22,  // 0.7421875
    True8False21,  // 0.7109375
    True9False20,  // 0.67578125
    True10False19, // 0.64453125
    True11False18, // 0.61328125
    True12False17, // 0.58203125
    True13False16, // 0.546875
    True14False15, // 0.515625
    True15False14, // 0.484375
    True16False13, // 0.453125
    True17False12, // 0.41796875
    True18False11, // 0.38671875
    True19False10, // 0.35546875
    True20False9,  // 0.32421875
    True21False8,  // 0.2890625
    True22False7,  // 0.2578125
    True23False6,  // 0.2265625
    True24False5,  // 0.1953125
    True25False4,  // 0.16015625
    True26False3,  // 0.12890625
    True27False2,  // 0.09765625
    True28False1,  // 0.06640625
    True29False0,  // 0.03125
    True0False30,  // 0.96875
    True1False29,  // 0.9375
    True2False28,  // 0.90625
    True3False27,  // 0.875
    True4False26,  // 0.84375
    True5False25,  // 0.8125
    True6False24,  // 0.78125
    True7False23,  // 0.75
    True8False22,  // 0.71875
    True9False21,  // 0.6875
    True10False20, // 0.65625
    True11False19, // 0.625
    True12False18, // 0.59375
    True13False17, // 0.5625
    True14False16, // 0.53125
    True15False15, // 0.5
    True16False14, // 0.46875
    True17False13, // 0.4375
    True18False12, // 0.40625
    True19False11, // 0.375
    True20False10, // 0.34375
    True21False9,  // 0.3125
    True22False8,  // 0.28125
    True23False7,  // 0.25
    True24False6,  // 0.21875
    True25False5,  // 0.1875
    True26False4,  // 0.15625
    True27False3,  // 0.125
    True28False2,  // 0.09375
    True29False1,  // 0.0625
    True30False0,  // 0.03125
    True0False31,  // 0.97265625
    True1False30,  // 0.9375
    True2False29,  // 0.91015625
    True3False28,  // 0.87890625
    True4False27,  // 0.84765625
    True5False26,  // 0.81640625
    True6False25,  // 0.7890625
    True7False24,  // 0.7578125
    True8False23,  // 0.7265625
    True9False22,  // 0.6953125
    True10False21, // 0.66796875
    True11False20, // 0.63671875
    True12False19, // 0.60546875
    True13False18, // 0.57421875
    True14False17, // 0.546875
    True15False16, // 0.515625
    True16False15, // 0.484375
    True17False14, // 0.453125
    True18False13, // 0.42578125
    True19False12, // 0.39453125
    True20False11, // 0.36328125
    True21False10, // 0.33203125
    True22False9,  // 0.3046875
    True23False8,  // 0.2734375
    True24False7,  // 0.2421875
    True25False6,  // 0.2109375
    True26False5,  // 0.18359375
    True27False4,  // 0.15234375
    True28False3,  // 0.12109375
    True29False2,  // 0.08984375
    True30False1,  // 0.0625
    True31False0,  // 0.02734375
    True0False32,  // 0.97265625
    True1False31,  // 0.94140625
    True2False30,  // 0.91015625
    True3False29,  // 0.8828125
    True4False28,  // 0.8515625
    True5False27,  // 0.82421875
    True6False26,  // 0.79296875
    True7False25,  // 0.765625
    True8False24,  // 0.734375
    True9False23,  // 0.70703125
    True10False22, // 0.67578125
    True11False21, // 0.6484375
    True12False20, // 0.6171875
    True13False19, // 0.58984375
    True14False18, // 0.55859375
    True15False17, // 0.53125
    True16False16, // 0.5
    True17False15, // 0.46875
    True18False14, // 0.44140625
    True19False13, // 0.41015625
    True20False12, // 0.3828125
    True21False11, // 0.3515625
    True22False10, // 0.32421875
    True23False9,  // 0.29296875
    True24False8,  // 0.265625
    True25False7,  // 0.234375
    True26False6,  // 0.20703125
    True27False5,  // 0.17578125
    True28False4,  // 0.1484375
    True29False3,  // 0.1171875
    True30False2,  // 0.08984375
    True31False1,  // 0.05859375
    True32False0,  // 0.02734375
    True0False33,  // 0.97265625
    True1False32,  // 0.94140625
    True2False31,  // 0.9140625
    True3False30,  // 0.88671875
    True4False29,  // 0.85546875
    True5False28,  // 0.828125
    True6False27,  // 0.80078125
    True7False26,  // 0.76953125
    True8False25,  // 0.7421875
    True9False24,  // 0.71484375
    True10False23, // 0.6875
    True11False22, // 0.65625
    True12False21, // 0.62890625
    True13False20, // 0.6015625
    True14False19, // 0.5703125
    True15False18, // 0.54296875
    True16False17, // 0.515625
    True17False16, // 0.484375
    True18False15, // 0.45703125
    True19False14, // 0.4296875
    True20False13, // 0.3984375
    True21False12, // 0.37109375
    True22False11, // 0.34375
    True23False10, // 0.3125
    True24False9,  // 0.28515625
    True25False8,  // 0.2578125
    True26False7,  // 0.23046875
    True27False6,  // 0.19921875
    True28False5,  // 0.171875
    True29False4,  // 0.14453125
    True30False3,  // 0.11328125
    True31False2,  // 0.0859375
    True32False1,  // 0.05859375
    True33False0,  // 0.02734375
    True0False34,  // 0.97265625
    True1False33,  // 0.9453125
    True2False32,  // 0.91796875
    True3False31,  // 0.890625
    True4False30,  // 0.859375
    True5False29,  // 0.83203125
    True6False28,  // 0.8046875
    True7False27,  // 0.77734375
    True8False26,  // 0.75
    True9False25,  // 0.72265625
    True10False24, // 0.6953125
    True11False23, // 0.66796875
    True12False22, // 0.640625
    True13False21, // 0.609375
    True14False20, // 0.58203125
    True15False19, // 0.5546875
    True16False18, // 0.52734375
    True17False17, // 0.5
    True18False16, // 0.47265625
    True19False15, // 0.4453125
    True20False14, // 0.41796875
    True21False13, // 0.390625
    True22False12, // 0.359375
    True23False11, // 0.33203125
    True24False10, // 0.3046875
    True25False9,  // 0.27734375
    True26False8,  // 0.25
    True27False7,  // 0.22265625
    True28False6,  // 0.1953125
    True29False5,  // 0.16796875
    True30False4,  // 0.140625
    True31False3,  // 0.109375
    True32False2,  // 0.08203125
    True33False1,  // 0.0546875
    True34False0,  // 0.02734375
    True0False35,  // 0.9765625
    True1False34,  // 0.9453125
    True2False33,  // 0.91796875
    True3False32,  // 0.890625
    True4False31,  // 0.86328125
    True5False30,  // 0.8359375
    True6False29,  // 0.8125
    True7False28,  // 0.78515625
    True8False27,  // 0.7578125
    True9False26,  // 0.73046875
    True10False25, // 0.703125
    True11False24, // 0.67578125
    True12False23, // 0.6484375
    True13False22, // 0.62109375
    True14False21, // 0.59375
    True15False20, // 0.56640625
    True16False19, // 0.5390625
    True17False18, // 0.51171875
    True18False17, // 0.48828125
    True19False16, // 0.4609375
    True20False15, // 0.43359375
    True21False14, // 0.40625
    True22False13, // 0.37890625
    True23False12, // 0.3515625
    True24False11, // 0.32421875
    True25False10, // 0.296875
    True26False9,  // 0.26953125
    True27False8,  // 0.2421875
    True28False7,  // 0.21484375
    True29False6,  // 0.1875
    True30False5,  // 0.1640625
    True31False4,  // 0.13671875
    True32False3,  // 0.109375
    True33False2,  // 0.08203125
    True34False1,  // 0.0546875
    True35False0,  // 0.0234375
    True0False36,  // 0.9765625
    True1False35,  // 0.9453125
    True2False34,  // 0.921875
    True3False33,  // 0.89453125
    True4False32,  // 0.8671875
    True5False31,  // 0.84375
    True6False30,  // 0.81640625
    True7False29,  // 0.7890625
    True8False28,  // 0.76171875
    True9False27,  // 0.73828125
    True10False26, // 0.7109375
    True11False25, // 0.68359375
    True12False24, // 0.65625
    True13False23, // 0.6328125
    True14False22, // 0.60546875
    True15False21, // 0.578125
    True16False20, // 0.55078125
    True17False19, // 0.52734375
    True18False18, // 0.5
    True19False17, // 0.47265625
    True20False16, // 0.44921875
    True21False15, // 0.421875
    True22False14, // 0.39453125
    True23False13, // 0.3671875
    True24False12, // 0.34375
    True25False11, // 0.31640625
    True26False10, // 0.2890625
    True27False9,  // 0.26171875
    True28False8,  // 0.23828125
    True29False7,  // 0.2109375
    True30False6,  // 0.18359375
    True31False5,  // 0.15625
    True32False4,  // 0.1328125
    True33False3,  // 0.10546875
    True34False2,  // 0.078125
    True35False1,  // 0.0546875
    True36False0,  // 0.0234375
    True0False37,  // 0.9765625
    True1False36,  // 0.94921875
    True2False35,  // 0.921875
    True3False34,  // 0.8984375
    True4False33,  // 0.87109375
    True5False32,  // 0.84765625
    True6False31,  // 0.8203125
    True7False30,  // 0.79296875
    True8False29,  // 0.76953125
    True9False28,  // 0.7421875
    True10False27, // 0.71875
    True11False26, // 0.69140625
    True12False25, // 0.66796875
    True13False24, // 0.640625
    True14False23, // 0.6171875
    True15False22, // 0.58984375
    True16False21, // 0.5625
    True17False20, // 0.5390625
    True18False19, // 0.51171875
    True19False18, // 0.48828125
    True20False17, // 0.4609375
    True21False16, // 0.4375
    True22False15, // 0.41015625
    True23False14, // 0.3828125
    True24False13, // 0.359375
    True25False12, // 0.33203125
    True26False11, // 0.30859375
    True27False10, // 0.28125
    True28False9,  // 0.2578125
    True29False8,  // 0.23046875
    True30False7,  // 0.20703125
    True31False6,  // 0.1796875
    True32False5,  // 0.15234375
    True33False4,  // 0.12890625
    True34False3,  // 0.1015625
    True35False2,  // 0.078125
    True36False1,  // 0.05078125
    True37False0,  // 0.0234375
    True0False38,  // 0.9765625
    True1False37,  // 0.94921875
    True2False36,  // 0.92578125
    True3False35,  // 0.8984375
    True4False34,  // 0.875
    True5False33,  // 0.8515625
    True6False32,  // 0.82421875
    True7False31,  // 0.80078125
    True8False30,  // 0.7734375
    True9False29,  // 0.75
    True10False28, // 0.7265625
    True11False27, // 0.69921875
    True12False26, // 0.67578125
    True13False25, // 0.6484375
    True14False24, // 0.625
    True15False23, // 0.6015625
    True16False22, // 0.57421875
    True17False21, // 0.55078125
    True18False20, // 0.5234375
    True19False19, // 0.5
    True20False18, // 0.4765625
    True21False17, // 0.44921875
    True22False16, // 0.42578125
    True23False15, // 0.3984375
    True24False14, // 0.375
    True25False13, // 0.3515625
    True26False12, // 0.32421875
    True27False11, // 0.30078125
    True28False10, // 0.2734375
    True29False9,  // 0.25
    True30False8,  // 0.2265625
    True31False7,  // 0.19921875
    True32False6,  // 0.17578125
    True33False5,  // 0.1484375
    True34False4,  // 0.125
    True35False3,  // 0.1015625
    True36False2,  // 0.07421875
    True37False1,  // 0.05078125
    True38False0,  // 0.0234375
    True0False39,  // 0.9765625
    True1False38,  // 0.94921875
    True2False37,  // 0.92578125
    True3False36,  // 0.90234375
    True4False35,  // 0.87890625
    True5False34,  // 0.85546875
    True6False33,  // 0.828125
    True7False32,  // 0.8046875
    True8False31,  // 0.78125
    True9False30,  // 0.7578125
    True10False29, // 0.73046875
    True11False28, // 0.70703125
    True12False27, // 0.68359375
    True13False26, // 0.66015625
    True14False25, // 0.6328125
    True15False24, // 0.609375
    True16False23, // 0.5859375
    True17False22, // 0.5625
    True18False21, // 0.53515625
    True19False20, // 0.51171875
    True20False19, // 0.48828125
    True21False18, // 0.46484375
    True22False17, // 0.4375
    True23False16, // 0.4140625
    True24False15, // 0.390625
    True25False14, // 0.3671875
    True26False13, // 0.33984375
    True27False12, // 0.31640625
    True28False11, // 0.29296875
    True29False10, // 0.26953125
    True30False9,  // 0.2421875
    True31False8,  // 0.21875
    True32False7,  // 0.1953125
    True33False6,  // 0.171875
    True34False5,  // 0.14453125
    True35False4,  // 0.12109375
    True36False3,  // 0.09765625
    True37False2,  // 0.07421875
    True38False1,  // 0.05078125
    True39False0,  // 0.0234375
    True0False40,  // 0.9765625
    True1False39,  // 0.953125
    True2False38,  // 0.9296875
    True3False37,  // 0.90625
    True4False36,  // 0.8828125
    True5False35,  // 0.85546875
    True6False34,  // 0.83203125
    True7False33,  // 0.80859375
    True8False32,  // 0.78515625
    True9False31,  // 0.76171875
    True10False30, // 0.73828125
    True11False29, // 0.71484375
    True12False28, // 0.69140625
    True13False27, // 0.66796875
    True14False26, // 0.64453125
    True15False25, // 0.6171875
    True16False24, // 0.59375
    True17False23, // 0.5703125
    True18False22, // 0.546875
    True19False21, // 0.5234375
    True20False20, // 0.5
    True21False19, // 0.4765625
    True22False18, // 0.453125
    True23False17, // 0.4296875
    True24False16, // 0.40625
    True25False15, // 0.3828125
    True26False14, // 0.35546875
    True27False13, // 0.33203125
    True28False12, // 0.30859375
    True29False11, // 0.28515625
    True30False10, // 0.26171875
    True31False9,  // 0.23828125
    True32False8,  // 0.21484375
    True33False7,  // 0.19140625
    True34False6,  // 0.16796875
    True35False5,  // 0.14453125
    True36False4,  // 0.1171875
    True37False3,  // 0.09375
    True38False2,  // 0.0703125
    True39False1,  // 0.046875
    True40False0,  // 0.0234375
    True0False41,  // 0.98046875
    True1False40,  // 0.953125
    True2False39,  // 0.9296875
    True3False38,  // 0.90625
    True4False37,  // 0.8828125
    True5False36,  // 0.859375
    True6False35,  // 0.8359375
    True7False34,  // 0.8125
    True8False33,  // 0.7890625
    True9False32,  // 0.765625
    True10False31, // 0.74609375
    True11False30, // 0.72265625
    True12False29, // 0.69921875
    True13False28, // 0.67578125
    True14False27, // 0.65234375
    True15False26, // 0.62890625
    True16False25, // 0.60546875
    True17False24, // 0.58203125
    True18False23, // 0.55859375
    True19False22, // 0.53515625
    True20False21, // 0.51171875
    True21False20, // 0.48828125
    True22False19, // 0.46484375
    True23False18, // 0.44140625
    True24False17, // 0.41796875
    True25False16, // 0.39453125
    True26False15, // 0.37109375
    True27False14, // 0.34765625
    True28False13, // 0.32421875
    True29False12, // 0.30078125
    True30False11, // 0.27734375
    True31False10, // 0.25390625
    True32False9,  // 0.234375
    True33False8,  // 0.2109375
    True34False7,  // 0.1875
    True35False6,  // 0.1640625
    True36False5,  // 0.140625
    True37False4,  // 0.1171875
    True38False3,  // 0.09375
    True39False2,  // 0.0703125
    True40False1,  // 0.046875
    True41False0,  // 0.01953125
    True0False42,  // 0.98046875
    True1False41,  // 0.953125
    True2False40,  // 0.93359375
    True3False39,  // 0.91015625
    True4False38,  // 0.88671875
    True5False37,  // 0.86328125
    True6False36,  // 0.83984375
    True7False35,  // 0.81640625
    True8False34,  // 0.796875
    True9False33,  // 0.7734375
    True10False32, // 0.75
    True11False31, // 0.7265625
    True12False30, // 0.703125
    True13False29, // 0.68359375
    True14False28, // 0.66015625
    True15False27, // 0.63671875
    True16False26, // 0.61328125
    True17False25, // 0.58984375
    True18False24, // 0.56640625
    True19False23, // 0.546875
    True20False22, // 0.5234375
    True21False21, // 0.5
    True22False20, // 0.4765625
    True23False19, // 0.453125
    True24False18, // 0.43359375
    True25False17, // 0.41015625
    True26False16, // 0.38671875
    True27False15, // 0.36328125
    True28False14, // 0.33984375
    True29False13, // 0.31640625
    True30False12, // 0.296875
    True31False11, // 0.2734375
    True32False10, // 0.25
    True33False9,  // 0.2265625
    True34False8,  // 0.203125
    True35False7,  // 0.18359375
    True36False6,  // 0.16015625
    True37False5,  // 0.13671875
    True38False4,  // 0.11328125
    True39False3,  // 0.08984375
    True40False2,  // 0.06640625
    True41False1,  // 0.046875
    True42False0,  // 0.01953125
    True0False43,  // 0.98046875
    True1False42,  // 0.95703125
    True2False41,  // 0.93359375
    True3False40,  // 0.91015625
    True4False39,  // 0.890625
    True5False38,  // 0.8671875
    True6False37,  // 0.84375
    True7False36,  // 0.8203125
    True8False35,  // 0.80078125
    True9False34,  // 0.77734375
    True10False33, // 0.75390625
    True11False32, // 0.734375
    True12False31, // 0.7109375
    True13False30, // 0.6875
    True14False29, // 0.66796875
    True15False28, // 0.64453125
    True16False27, // 0.62109375
    True17False26, // 0.6015625
    True18False25, // 0.578125
    True19False24, // 0.5546875
    True20False23, // 0.53515625
    True21False22, // 0.51171875
    True22False21, // 0.48828125
    True23False20, // 0.46484375
    True24False19, // 0.4453125
    True25False18, // 0.421875
    True26False17, // 0.3984375
    True27False16, // 0.37890625
    True28False15, // 0.35546875
    True29False14, // 0.33203125
    True30False13, // 0.3125
    True31False12, // 0.2890625
    True32False11, // 0.265625
    True33False10, // 0.24609375
    True34False9,  // 0.22265625
    True35False8,  // 0.19921875
    True36False7,  // 0.1796875
    True37False6,  // 0.15625
    True38False5,  // 0.1328125
    True39False4,  // 0.109375
    True40False3,  // 0.08984375
    True41False2,  // 0.06640625
    True42False1,  // 0.04296875
    True43False0,  // 0.01953125
    True0False44,  // 0.98046875
    True1False43,  // 0.95703125
    True2False42,  // 0.93359375
    True3False41,  // 0.9140625
    True4False40,  // 0.890625
    True5False39,  // 0.87109375
    True6False38,  // 0.84765625
    True7False37,  // 0.82421875
    True8False36,  // 0.8046875
    True9False35,  // 0.78125
    True10False34, // 0.76171875
    True11False33, // 0.73828125
    True12False32, // 0.71875
    True13False31, // 0.6953125
    True14False30, // 0.67578125
    True15False29, // 0.65234375
    True16False28, // 0.62890625
    True17False27, // 0.609375
    True18False26, // 0.5859375
    True19False25, // 0.56640625
    True20False24, // 0.54296875
    True21False23, // 0.5234375
    True22False22, // 0.5
    True23False21, // 0.4765625
    True24False20, // 0.45703125
    True25False19, // 0.43359375
    True26False18, // 0.4140625
    True27False17, // 0.390625
    True28False16, // 0.37109375
    True29False15, // 0.34765625
    True30False14, // 0.32421875
    True31False13, // 0.3046875
    True32False12, // 0.28125
    True33False11, // 0.26171875
    True34False10, // 0.23828125
    True35False9,  // 0.21875
    True36False8,  // 0.1953125
    True37False7,  // 0.17578125
    True38False6,  // 0.15234375
    True39False5,  // 0.12890625
    True40False4,  // 0.109375
    True41False3,  // 0.0859375
    True42False2,  // 0.06640625
    True43False1,  // 0.04296875
    True44False0,  // 0.01953125
    True0False45,  // 0.98046875
    True1False44,  // 0.95703125
    True2False43,  // 0.9375
    True3False42,  // 0.9140625
    True4False41,  // 0.89453125
    True5False40,  // 0.87109375
    True6False39,  // 0.8515625
    True7False38,  // 0.828125
    True8False37,  // 0.80859375
    True9False36,  // 0.7890625
    True10False35, // 0.765625
    True11False34, // 0.74609375
    True12False33, // 0.72265625
    True13False32, // 0.703125
    True14False31, // 0.6796875
    True15False30, // 0.66015625
    True16False29, // 0.63671875
    True17False28, // 0.6171875
    True18False27, // 0.59765625
    True19False26, // 0.57421875
    True20False25, // 0.5546875
    True21False24, // 0.53125
    True22False23, // 0.51171875
    True23False22, // 0.48828125
    True24False21, // 0.46875
    True25False20, // 0.4453125
    True26False19, // 0.42578125
    True27False18, // 0.40234375
    True28False17, // 0.3828125
    True29False16, // 0.36328125
    True30False15, // 0.33984375
    True31False14, // 0.3203125
    True32False13, // 0.296875
    True33False12, // 0.27734375
    True34False11, // 0.25390625
    True35False10, // 0.234375
    True36False9,  // 0.2109375
    True37False8,  // 0.19140625
    True38False7,  // 0.171875
    True39False6,  // 0.1484375
    True40False5,  // 0.12890625
    True41False4,  // 0.10546875
    True42False3,  // 0.0859375
    True43False2,  // 0.0625
    True44False1,  // 0.04296875
    True45False0,  // 0.01953125
    True0False46,  // 0.98046875
    True1False45,  // 0.95703125
    True2False44,  // 0.9375
    True3False43,  // 0.91796875
    True4False42,  // 0.89453125
    True5False41,  // 0.875
    True6False40,  // 0.85546875
    True7False39,  // 0.83203125
    True8False38,  // 0.8125
    True9False37,  // 0.79296875
    True10False36, // 0.76953125
    True11False35, // 0.75
    True12False34, // 0.73046875
    True13False33, // 0.70703125
    True14False32, // 0.6875
    True15False31, // 0.66796875
    True16False30, // 0.64453125
    True17False29, // 0.625
    True18False28, // 0.60546875
    True19False27, // 0.58203125
    True20False26, // 0.5625
    True21False25, // 0.54296875
    True22False24, // 0.51953125
    True23False23, // 0.5
    True24False22, // 0.48046875
    True25False21, // 0.45703125
    True26False20, // 0.4375
    True27False19, // 0.41796875
    True28False18, // 0.39453125
    True29False17, // 0.375
    True30False16, // 0.35546875
    True31False15, // 0.33203125
    True32False14, // 0.3125
    True33False13, // 0.29296875
    True34False12, // 0.26953125
    True35False11, // 0.25
    True36False10, // 0.23046875
    True37False9,  // 0.20703125
    True38False8,  // 0.1875
    True39False7,  // 0.16796875
    True40False6,  // 0.14453125
    True41False5,  // 0.125
    True42False4,  // 0.10546875
    True43False3,  // 0.08203125
    True44False2,  // 0.0625
    True45False1,  // 0.04296875
    True46False0,  // 0.01953125
    True0False47,  // 0.98046875
    True1False46,  // 0.9609375
    True2False45,  // 0.9375
    True3False44,  // 0.91796875
    True4False43,  // 0.8984375
    True5False42,  // 0.87890625
    True6False41,  // 0.85546875
    True7False40,  // 0.8359375
    True8False39,  // 0.81640625
    True9False38,  // 0.796875
    True10False37, // 0.77734375
    True11False36, // 0.75390625
    True12False35, // 0.734375
    True13False34, // 0.71484375
    True14False33, // 0.6953125
    True15False32, // 0.671875
    True16False31, // 0.65234375
    True17False30, // 0.6328125
    True18False29, // 0.61328125
    True19False28, // 0.59375
    True20False27, // 0.5703125
    True21False26, // 0.55078125
    True22False25, // 0.53125
    True23False24, // 0.51171875
    True24False23, // 0.48828125
    True25False22, // 0.46875
    True26False21, // 0.44921875
    True27False20, // 0.4296875
    True28False19, // 0.40625
    True29False18, // 0.38671875
    True30False17, // 0.3671875
    True31False16, // 0.34765625
    True32False15, // 0.328125
    True33False14, // 0.3046875
    True34False13, // 0.28515625
    True35False12, // 0.265625
    True36False11, // 0.24609375
    True37False10, // 0.22265625
    True38False9,  // 0.203125
    True39False8,  // 0.18359375
    True40False7,  // 0.1640625
    True41False6,  // 0.14453125
    True42False5,  // 0.12109375
    True43False4,  // 0.1015625
    True44False3,  // 0.08203125
    True45False2,  // 0.0625
    True46False1,  // 0.0390625
    True47False0,  // 0.01953125
    True0False48,  // 0.98046875
    True1False47,  // 0.9609375
    True2False46,  // 0.94140625
    True3False45,  // 0.921875
    True4False44,  // 0.8984375
    True5False43,  // 0.87890625
    True6False42,  // 0.859375
    True7False41,  // 0.83984375
    True8False40,  // 0.8203125
    True9False39,  // 0.80078125
    True10False38, // 0.78125
    True11False37, // 0.76171875
    True12False36, // 0.73828125
    True13False35, // 0.71875
    True14False34, // 0.69921875
    True15False33, // 0.6796875
    True16False32, // 0.66015625
    True17False31, // 0.640625
    True18False30, // 0.62109375
    True19False29, // 0.6015625
    True20False28, // 0.578125
    True21False27, // 0.55859375
    True22False26, // 0.5390625
    True23False25, // 0.51953125
    True24False24, // 0.5
    True25False23, // 0.48046875
    True26False22, // 0.4609375
    True27False21, // 0.44140625
    True28False20, // 0.421875
    True29False19, // 0.3984375
    True30False18, // 0.37890625
    True31False17, // 0.359375
    True32False16, // 0.33984375
    True33False15, // 0.3203125
    True34False14, // 0.30078125
    True35False13, // 0.28125
    True36False12, // 0.26171875
    True37False11, // 0.23828125
    True38False10, // 0.21875
    True39False9,  // 0.19921875
    True40False8,  // 0.1796875
    True41False7,  // 0.16015625
    True42False6,  // 0.140625
    True43False5,  // 0.12109375
    True44False4,  // 0.1015625
    True45False3,  // 0.078125
    True46False2,  // 0.05859375
    True47False1,  // 0.0390625
    True48False0,  // 0.01953125
    True0False49,  // 0.98046875
    True1False48,  // 0.9609375
    True2False47,  // 0.94140625
    True3False46,  // 0.921875
    True4False45,  // 0.90234375
    True5False44,  // 0.8828125
    True6False43,  // 0.86328125
    True7False42,  // 0.84375
    True8False41,  // 0.82421875
    True9False40,  // 0.8046875
    True10False39, // 0.78515625
    True11False38, // 0.765625
    True12False37, // 0.74609375
    True13False36, // 0.7265625
    True14False35, // 0.70703125
    True15False34, // 0.6875
    True16False33, // 0.66796875
    True17False32, // 0.6484375
    True18False31, // 0.62890625
    True19False30, // 0.609375
    True20False29, // 0.58984375
    True21False28, // 0.5703125
    True22False27, // 0.55078125
    True23False26, // 0.53125
    True24False25, // 0.51171875
    True25False24, // 0.48828125
    True26False23, // 0.46875
    True27False22, // 0.44921875
    True28False21, // 0.4296875
    True29False20, // 0.41015625
    True30False19, // 0.390625
    True31False18, // 0.37109375
    True32False17, // 0.3515625
    True33False16, // 0.33203125
    True34False15, // 0.3125
    True35False14, // 0.29296875
    True36False13, // 0.2734375
    True37False12, // 0.25390625
    True38False11, // 0.234375
    True39False10, // 0.21484375
    True40False9,  // 0.1953125
    True41False8,  // 0.17578125
    True42False7,  // 0.15625
    True43False6,  // 0.13671875
    True44False5,  // 0.1171875
    True45False4,  // 0.09765625
    True46False3,  // 0.078125
    True47False2,  // 0.05859375
    True48False1,  // 0.0390625
    True49False0,  // 0.01953125
    True0False50,  // 0.984375
    True1False49,  // 0.9609375
    True2False48,  // 0.94140625
    True3False47,  // 0.921875
    True4False46,  // 0.90234375
    True5False45,  // 0.8828125
    True6False44,  // 0.8671875
    True7False43,  // 0.84765625
    True8False42,  // 0.828125
    True9False41,  // 0.80859375
    True10False40, // 0.7890625
    True11False39, // 0.76953125
    True12False38, // 0.75
    True13False37, // 0.73046875
    True14False36, // 0.7109375
    True15False35, // 0.69140625
    True16False34, // 0.671875
    True17False33, // 0.65234375
    True18False32, // 0.6328125
    True19False31, // 0.6171875
    True20False30, // 0.59765625
    True21False29, // 0.578125
    True22False28, // 0.55859375
    True23False27, // 0.5390625
    True24False26, // 0.51953125
    True25False25, // 0.5
    True26False24, // 0.48046875
    True27False23, // 0.4609375
    True28False22, // 0.44140625
    True29False21, // 0.421875
    True30False20, // 0.40234375
    True31False19, // 0.3828125
    True32False18, // 0.3671875
    True33False17, // 0.34765625
    True34False16, // 0.328125
    True35False15, // 0.30859375
    True36False14, // 0.2890625
    True37False13, // 0.26953125
    True38False12, // 0.25
    True39False11, // 0.23046875
    True40False10, // 0.2109375
    True41False9,  // 0.19140625
    True42False8,  // 0.171875
    True43False7,  // 0.15234375
    True44False6,  // 0.1328125
    True45False5,  // 0.1171875
    True46False4,  // 0.09765625
    True47False3,  // 0.078125
    True48False2,  // 0.05859375
    True49False1,  // 0.0390625
    True50False0,  // 0.015625
    True0False51,  // 0.984375
    True1False50,  // 0.9609375
    True2False49,  // 0.9453125
    True3False48,  // 0.92578125
    True4False47,  // 0.90625
    True5False46,  // 0.88671875
    True6False45,  // 0.8671875
    True7False44,  // 0.84765625
    True8False43,  // 0.83203125
    True9False42,  // 0.8125
    True10False41, // 0.79296875
    True11False40, // 0.7734375
    True12False39, // 0.75390625
    True13False38, // 0.734375
    True14False37, // 0.71875
    True15False36, // 0.69921875
    True16False35, // 0.6796875
    True17False34, // 0.66015625
    True18False33, // 0.640625
    True19False32, // 0.62109375
    True20False31, // 0.60546875
    True21False30, // 0.5859375
    True22False29, // 0.56640625
    True23False28, // 0.546875
    True24False27, // 0.52734375
    True25False26, // 0.5078125
    True26False25, // 0.4921875
    True27False24, // 0.47265625
    True28False23, // 0.453125
    True29False22, // 0.43359375
    True30False21, // 0.4140625
    True31False20, // 0.39453125
    True32False19, // 0.37890625
    True33False18, // 0.359375
    True34False17, // 0.33984375
    True35False16, // 0.3203125
    True36False15, // 0.30078125
    True37False14, // 0.28125
    True38False13, // 0.265625
    True39False12, // 0.24609375
    True40False11, // 0.2265625
    True41False10, // 0.20703125
    True42False9,  // 0.1875
    True43False8,  // 0.16796875
    True44False7,  // 0.15234375
    True45False6,  // 0.1328125
    True46False5,  // 0.11328125
    True47False4,  // 0.09375
    True48False3,  // 0.07421875
    True49False2,  // 0.0546875
    True50False1,  // 0.0390625
    True51False0,  // 0.015625
    True0False52,  // 0.984375
    True1False51,  // 0.9609375
    True2False50,  // 0.9453125
    True3False49,  // 0.92578125
    True4False48,  // 0.90625
    True5False47,  // 0.890625
    True6False46,  // 0.87109375
    True7False45,  // 0.8515625
    True8False44,  // 0.83203125
    True9False43,  // 0.81640625
    True10False42, // 0.796875
    True11False41, // 0.77734375
    True12False40, // 0.7578125
    True13False39, // 0.7421875
    True14False38, // 0.72265625
    True15False37, // 0.703125
    True16False36, // 0.68359375
    True17False35, // 0.66796875
    True18False34, // 0.6484375
    True19False33, // 0.62890625
    True20False32, // 0.609375
    True21False31, // 0.59375
    True22False30, // 0.57421875
    True23False29, // 0.5546875
    True24False28, // 0.53515625
    True25False27, // 0.51953125
    True26False26, // 0.5
    True27False25, // 0.48046875
    True28False24, // 0.46484375
    True29False23, // 0.4453125
    True30False22, // 0.42578125
    True31False21, // 0.40625
    True32False20, // 0.390625
    True33False19, // 0.37109375
    True34False18, // 0.3515625
    True35False17, // 0.33203125
    True36False16, // 0.31640625
    True37False15, // 0.296875
    True38False14, // 0.27734375
    True39False13, // 0.2578125
    True40False12, // 0.2421875
    True41False11, // 0.22265625
    True42False10, // 0.203125
    True43False9,  // 0.18359375
    True44False8,  // 0.16796875
    True45False7,  // 0.1484375
    True46False6,  // 0.12890625
    True47False5,  // 0.109375
    True48False4,  // 0.09375
    True49False3,  // 0.07421875
    True50False2,  // 0.0546875
    True51False1,  // 0.0390625
    True52False0,  // 0.015625
    True0False53,  // 0.984375
    True1False52,  // 0.96484375
    True2False51,  // 0.9453125
    True3False50,  // 0.92578125
    True4False49,  // 0.91015625
    True5False48,  // 0.890625
    True6False47,  // 0.87109375
    True7False46,  // 0.85546875
    True8False45,  // 0.8359375
    True9False44,  // 0.81640625
    True10False43, // 0.80078125
    True11False42, // 0.78125
    True12False41, // 0.76171875
    True13False40, // 0.74609375
    True14False39, // 0.7265625
    True15False38, // 0.7109375
    True16False37, // 0.69140625
    True17False36, // 0.671875
    True18False35, // 0.65625
    True19False34, // 0.63671875
    True20False33, // 0.6171875
    True21False32, // 0.6015625
    True22False31, // 0.58203125
    True23False30, // 0.5625
    True24False29, // 0.546875
    True25False28, // 0.52734375
    True26False27, // 0.5078125
    True27False26, // 0.4921875
    True28False25, // 0.47265625
    True29False24, // 0.453125
    True30False23, // 0.4375
    True31False22, // 0.41796875
    True32False21, // 0.3984375
    True33False20, // 0.3828125
    True34False19, // 0.36328125
    True35False18, // 0.34375
    True36False17, // 0.328125
    True37False16, // 0.30859375
    True38False15, // 0.2890625
    True39False14, // 0.2734375
    True40False13, // 0.25390625
    True41False12, // 0.23828125
    True42False11, // 0.21875
    True43False10, // 0.19921875
    True44False9,  // 0.18359375
    True45False8,  // 0.1640625
    True46False7,  // 0.14453125
    True47False6,  // 0.12890625
    True48False5,  // 0.109375
    True49False4,  // 0.08984375
    True50False3,  // 0.07421875
    True51False2,  // 0.0546875
    True52False1,  // 0.03515625
    True53False0,  // 0.015625
    True0False54,  // 0.984375
    True1False53,  // 0.96484375
    True2False52,  // 0.9453125
    True3False51,  // 0.9296875
    True4False50,  // 0.91015625
    True5False49,  // 0.89453125
    True6False48,  // 0.875
    True7False47,  // 0.85546875
    True8False46,  // 0.83984375
    True9False45,  // 0.8203125
    True10False44, // 0.8046875
    True11False43, // 0.78515625
    True12False42, // 0.76953125
    True13False41, // 0.75
    True14False40, // 0.73046875
    True15False39, // 0.71484375
    True16False38, // 0.6953125
    True17False37, // 0.6796875
    True18False36, // 0.66015625
    True19False35, // 0.64453125
    True20False34, // 0.625
    True21False33, // 0.60546875
    True22False32, // 0.58984375
    True23False31, // 0.5703125
    True24False30, // 0.5546875
    True25False29, // 0.53515625
    True26False28, // 0.51953125
    True27False27, // 0.5
    True28False26, // 0.48046875
    True29False25, // 0.46484375
    True30False24, // 0.4453125
    True31False23, // 0.4296875
    True32False22, // 0.41015625
    True33False21, // 0.39453125
    True34False20, // 0.375
    True35False19, // 0.35546875
    True36False18, // 0.33984375
    True37False17, // 0.3203125
    True38False16, // 0.3046875
    True39False15, // 0.28515625
    True40False14, // 0.26953125
    True41False13, // 0.25
    True42False12, // 0.23046875
    True43False11, // 0.21484375
    True44False10, // 0.1953125
    True45False9,  // 0.1796875
    True46False8,  // 0.16015625
    True47False7,  // 0.14453125
    True48False6,  // 0.125
    True49False5,  // 0.10546875
    True50False4,  // 0.08984375
    True51False3,  // 0.0703125
    True52False2,  // 0.0546875
    True53False1,  // 0.03515625
    True54False0,  // 0.015625
    True0False55,  // 0.984375
    True1False54,  // 0.96484375
    True2False53,  // 0.94921875
    True3False52,  // 0.9296875
    True4False51,  // 0.9140625
    True5False50,  // 0.89453125
    True6False49,  // 0.87890625
    True7False48,  // 0.859375
    True8False47,  // 0.84375
    True9False46,  // 0.82421875
    True10False45, // 0.80859375
    True11False44, // 0.7890625
    True12False43, // 0.7734375
    True13False42, // 0.75390625
    True14False41, // 0.73828125
    True15False40, // 0.71875
    True16False39, // 0.703125
    True17False38, // 0.68359375
    True18False37, // 0.66796875
    True19False36, // 0.6484375
    True20False35, // 0.6328125
    True21False34, // 0.61328125
    True22False33, // 0.59765625
    True23False32, // 0.578125
    True24False31, // 0.5625
    True25False30, // 0.54296875
    True26False29, // 0.52734375
    True27False28, // 0.5078125
    True28False27, // 0.4921875
    True29False26, // 0.47265625
    True30False25, // 0.45703125
    True31False24, // 0.4375
    True32False23, // 0.421875
    True33False22, // 0.40234375
    True34False21, // 0.38671875
    True35False20, // 0.3671875
    True36False19, // 0.3515625
    True37False18, // 0.33203125
    True38False17, // 0.31640625
    True39False16, // 0.296875
    True40False15, // 0.28125
    True41False14, // 0.26171875
    True42False13, // 0.24609375
    True43False12, // 0.2265625
    True44False11, // 0.2109375
    True45False10, // 0.19140625
    True46False9,  // 0.17578125
    True47False8,  // 0.15625
    True48False7,  // 0.140625
    True49False6,  // 0.12109375
    True50False5,  // 0.10546875
    True51False4,  // 0.0859375
    True52False3,  // 0.0703125
    True53False2,  // 0.05078125
    True54False1,  // 0.03515625
    True55False0,  // 0.015625
    True0False56,  // 0.984375
    True1False55,  // 0.96484375
    True2False54,  // 0.94921875
    True3False53,  // 0.9296875
    True4False52,  // 0.9140625
    True5False51,  // 0.8984375
    True6False50,  // 0.87890625
    True7False49,  // 0.86328125
    True8False48,  // 0.84375
    True9False47,  // 0.828125
    True10False46, // 0.80859375
    True11False45, // 0.79296875
    True12False44, // 0.77734375
    True13False43, // 0.7578125
    True14False42, // 0.7421875
    True15False41, // 0.72265625
    True16False40, // 0.70703125
    True17False39, // 0.69140625
    True18False38, // 0.671875
    True19False37, // 0.65625
    True20False36, // 0.63671875
    True21False35, // 0.62109375
    True22False34, // 0.6015625
    True23False33, // 0.5859375
    True24False32, // 0.5703125
    True25False31, // 0.55078125
    True26False30, // 0.53515625
    True27False29, // 0.515625
    True28False28, // 0.5
    True29False27, // 0.484375
    True30False26, // 0.46484375
    True31False25, // 0.44921875
    True32False24, // 0.4296875
    True33False23, // 0.4140625
    True34False22, // 0.3984375
    True35False21, // 0.37890625
    True36False20, // 0.36328125
    True37False19, // 0.34375
    True38False18, // 0.328125
    True39False17, // 0.30859375
    True40False16, // 0.29296875
    True41False15, // 0.27734375
    True42False14, // 0.2578125
    True43False13, // 0.2421875
    True44False12, // 0.22265625
    True45False11, // 0.20703125
    True46False10, // 0.19140625
    True47False9,  // 0.171875
    True48False8,  // 0.15625
    True49False7,  // 0.13671875
    True50False6,  // 0.12109375
    True51False5,  // 0.1015625
    True52False4,  // 0.0859375
    True53False3,  // 0.0703125
    True54False2,  // 0.05078125
    True55False1,  // 0.03515625
    True56False0,  // 0.015625
    True0False57,  // 0.984375
    True1False56,  // 0.96484375
    True2False55,  // 0.94921875
    True3False54,  // 0.93359375
    True4False53,  // 0.9140625
    True5False52,  // 0.8984375
    True6False51,  // 0.8828125
    True7False50,  // 0.86328125
    True8False49,  // 0.84765625
    True9False48,  // 0.83203125
    True10False47, // 0.8125
    True11False46, // 0.796875
    True12False45, // 0.78125
    True13False44, // 0.76171875
    True14False43, // 0.74609375
    True15False42, // 0.73046875
    True16False41, // 0.7109375
    True17False40, // 0.6953125
    True18False39, // 0.6796875
    True19False38, // 0.66015625
    True20False37, // 0.64453125
    True21False36, // 0.62890625
    True22False35, // 0.609375
    True23False34, // 0.59375
    True24False33, // 0.578125
    True25False32, // 0.55859375
    True26False31, // 0.54296875
    True27False30, // 0.52734375
    True28False29, // 0.5078125
    True29False28, // 0.4921875
    True30False27, // 0.47265625
    True31False26, // 0.45703125
    True32False25, // 0.44140625
    True33False24, // 0.421875
    True34False23, // 0.40625
    True35False22, // 0.390625
    True36False21, // 0.37109375
    True37False20, // 0.35546875
    True38False19, // 0.33984375
    True39False18, // 0.3203125
    True40False17, // 0.3046875
    True41False16, // 0.2890625
    True42False15, // 0.26953125
    True43False14, // 0.25390625
    True44False13, // 0.23828125
    True45False12, // 0.21875
    True46False11, // 0.203125
    True47False10, // 0.1875
    True48False9,  // 0.16796875
    True49False8,  // 0.15234375
    True50False7,  // 0.13671875
    True51False6,  // 0.1171875
    True52False5,  // 0.1015625
    True53False4,  // 0.0859375
    True54False3,  // 0.06640625
    True55False2,  // 0.05078125
    True56False1,  // 0.03515625
    True57False0,  // 0.015625
    True0False58,  // 0.984375
    True1False57,  // 0.96484375
    True2False56,  // 0.94921875
    True3False55,  // 0.93359375
    True4False54,  // 0.91796875
    True5False53,  // 0.8984375
    True6False52,  // 0.8828125
    True7False51,  // 0.8671875
    True8False50,  // 0.8515625
    True9False49,  // 0.83203125
    True10False48, // 0.81640625
    True11False47, // 0.80078125
    True12False46, // 0.78515625
    True13False45, // 0.765625
    True14False44, // 0.75
    True15False43, // 0.734375
    True16False42, // 0.71484375
    True17False41, // 0.69921875
    True18False40, // 0.68359375
    True19False39, // 0.66796875
    True20False38, // 0.6484375
    True21False37, // 0.6328125
    True22False36, // 0.6171875
    True23False35, // 0.6015625
    True24False34, // 0.58203125
    True25False33, // 0.56640625
    True26False32, // 0.55078125
    True27False31, // 0.53515625
    True28False30, // 0.515625
    True29False29, // 0.5
    True30False28, // 0.484375
    True31False27, // 0.46484375
    True32False26, // 0.44921875
    True33False25, // 0.43359375
    True34False24, // 0.41796875
    True35False23, // 0.3984375
    True36False22, // 0.3828125
    True37False21, // 0.3671875
    True38False20, // 0.3515625
    True39False19, // 0.33203125
    True40False18, // 0.31640625
    True41False17, // 0.30078125
    True42False16, // 0.28515625
    True43False15, // 0.265625
    True44False14, // 0.25
    True45False13, // 0.234375
    True46False12, // 0.21484375
    True47False11, // 0.19921875
    True48False10, // 0.18359375
    True49False9,  // 0.16796875
    True50False8,  // 0.1484375
    True51False7,  // 0.1328125
    True52False6,  // 0.1171875
    True53False5,  // 0.1015625
    True54False4,  // 0.08203125
    True55False3,  // 0.06640625
    True56False2,  // 0.05078125
    True57False1,  // 0.03515625
    True58False0,  // 0.015625
    True0False59,  // 0.984375
    True1False58,  // 0.96875
    True2False57,  // 0.94921875
    True3False56,  // 0.93359375
    True4False55,  // 0.91796875
    True5False54,  // 0.90234375
    True6False53,  // 0.88671875
    True7False52,  // 0.8671875
    True8False51,  // 0.8515625
    True9False50,  // 0.8359375
    True10False49, // 0.8203125
    True11False48, // 0.8046875
    True12False47, // 0.78515625
    True13False46, // 0.76953125
    True14False45, // 0.75390625
    True15False44, // 0.73828125
    True16False43, // 0.72265625
    True17False42, // 0.703125
    True18False41, // 0.6875
    True19False40, // 0.671875
    True20False39, // 0.65625
    True21False38, // 0.640625
    True22False37, // 0.62109375
    True23False36, // 0.60546875
    True24False35, // 0.58984375
    True25False34, // 0.57421875
    True26False33, // 0.55859375
    True27False32, // 0.5390625
    True28False31, // 0.5234375
    True29False30, // 0.5078125
    True30False29, // 0.4921875
    True31False28, // 0.4765625
    True32False27, // 0.4609375
    True33False26, // 0.44140625
    True34False25, // 0.42578125
    True35False24, // 0.41015625
    True36False23, // 0.39453125
    True37False22, // 0.37890625
    True38False21, // 0.359375
    True39False20, // 0.34375
    True40False19, // 0.328125
    True41False18, // 0.3125
    True42False17, // 0.296875
    True43False16, // 0.27734375
    True44False15, // 0.26171875
    True45False14, // 0.24609375
    True46False13, // 0.23046875
    True47False12, // 0.21484375
    True48False11, // 0.1953125
    True49False10, // 0.1796875
    True50False9,  // 0.1640625
    True51False8,  // 0.1484375
    True52False7,  // 0.1328125
    True53False6,  // 0.11328125
    True54False5,  // 0.09765625
    True55False4,  // 0.08203125
    True56False3,  // 0.06640625
    True57False2,  // 0.05078125
    True58False1,  // 0.03125
    True59False0,  // 0.015625
    True0False60,  // 0.984375
    True1False59,  // 0.96875
    True2False58,  // 0.953125
    True3False57,  // 0.93359375
    True4False56,  // 0.91796875
    True5False55,  // 0.90234375
    True6False54,  // 0.88671875
    True7False53,  // 0.87109375
    True8False52,  // 0.85546875
    True9False51,  // 0.83984375
    True10False50, // 0.82421875
    True11False49, // 0.8046875
    True12False48, // 0.7890625
    True13False47, // 0.7734375
    True14False46, // 0.7578125
    True15False45, // 0.7421875
    True16False44, // 0.7265625
    True17False43, // 0.7109375
    True18False42, // 0.6953125
    True19False41, // 0.67578125
    True20False40, // 0.66015625
    True21False39, // 0.64453125
    True22False38, // 0.62890625
    True23False37, // 0.61328125
    True24False36, // 0.59765625
    True25False35, // 0.58203125
    True26False34, // 0.56640625
    True27False33, // 0.546875
    True28False32, // 0.53125
    True29False31, // 0.515625
    True30False30, // 0.5
    True31False29, // 0.484375
    True32False28, // 0.46875
    True33False27, // 0.453125
    True34False26, // 0.43359375
    True35False25, // 0.41796875
    True36False24, // 0.40234375
    True37False23, // 0.38671875
    True38False22, // 0.37109375
    True39False21, // 0.35546875
    True40False20, // 0.33984375
    True41False19, // 0.32421875
    True42False18, // 0.3046875
    True43False17, // 0.2890625
    True44False16, // 0.2734375
    True45False15, // 0.2578125
    True46False14, // 0.2421875
    True47False13, // 0.2265625
    True48False12, // 0.2109375
    True49False11, // 0.1953125
    True50False10, // 0.17578125
    True51False9,  // 0.16015625
    True52False8,  // 0.14453125
    True53False7,  // 0.12890625
    True54False6,  // 0.11328125
    True55False5,  // 0.09765625
    True56False4,  // 0.08203125
    True57False3,  // 0.06640625
    True58False2,  // 0.046875
    True59False1,  // 0.03125
    True60False0,  // 0.015625
    True0False61,  // 0.984375
    True1False60,  // 0.96875
    True2False59,  // 0.953125
    True3False58,  // 0.9375
    True4False57,  // 0.921875
    True5False56,  // 0.90625
    True6False55,  // 0.890625
    True7False54,  // 0.87109375
    True8False53,  // 0.85546875
    True9False52,  // 0.83984375
    True10False51, // 0.82421875
    True11False50, // 0.80859375
    True12False49, // 0.79296875
    True13False48, // 0.77734375
    True14False47, // 0.76171875
    True15False46, // 0.74609375
    True16False45, // 0.73046875
    True17False44, // 0.71484375
    True18False43, // 0.69921875
    True19False42, // 0.68359375
    True20False41, // 0.66796875
    True21False40, // 0.65234375
    True22False39, // 0.63671875
    True23False38, // 0.6171875
    True24False37, // 0.6015625
    True25False36, // 0.5859375
    True26False35, // 0.5703125
    True27False34, // 0.5546875
    True28False33, // 0.5390625
    True29False32, // 0.5234375
    True30False31, // 0.5078125
    True31False30, // 0.4921875
    True32False29, // 0.4765625
    True33False28, // 0.4609375
    True34False27, // 0.4453125
    True35False26, // 0.4296875
    True36False25, // 0.4140625
    True37False24, // 0.3984375
    True38False23, // 0.3828125
    True39False22, // 0.36328125
    True40False21, // 0.34765625
    True41False20, // 0.33203125
    True42False19, // 0.31640625
    True43False18, // 0.30078125
    True44False17, // 0.28515625
    True45False16, // 0.26953125
    True46False15, // 0.25390625
    True47False14, // 0.23828125
    True48False13, // 0.22265625
    True49False12, // 0.20703125
    True50False11, // 0.19140625
    True51False10, // 0.17578125
    True52False9,  // 0.16015625
    True53False8,  // 0.14453125
    True54False7,  // 0.12890625
    True55False6,  // 0.109375
    True56False5,  // 0.09375
    True57False4,  // 0.078125
    True58False3,  // 0.0625
    True59False2,  // 0.046875
    True60False1,  // 0.03125
    True61False0,  // 0.015625
    True0False62,  // 0.984375
    True1False61,  // 0.96875
    True2False60,  // 0.953125
    True3False59,  // 0.9375
    True4False58,  // 0.921875
    True5False57,  // 0.90625
    True6False56,  // 0.890625
    True7False55,  // 0.875
    True8False54,  // 0.859375
    True9False53,  // 0.84375
    True10False52, // 0.828125
    True11False51, // 0.8125
    True12False50, // 0.796875
    True13False49, // 0.78125
    True14False48, // 0.765625
    True15False47, // 0.75
    True16False46, // 0.734375
    True17False45, // 0.71875
    True18False44, // 0.703125
    True19False43, // 0.6875
    True20False42, // 0.671875
    True21False41, // 0.65625
    True22False40, // 0.640625
    True23False39, // 0.625
    True24False38, // 0.609375
    True25False37, // 0.59375
    True26False36, // 0.578125
    True27False35, // 0.5625
    True28False34, // 0.546875
    True29False33, // 0.53125
    True30False32, // 0.515625
    True31False31, // 0.5
    True32False30, // 0.484375
    True33False29, // 0.46875
    True34False28, // 0.453125
    True35False27, // 0.4375
    True36False26, // 0.421875
    True37False25, // 0.40625
    True38False24, // 0.390625
    True39False23, // 0.375
    True40False22, // 0.359375
    True41False21, // 0.34375
    True42False20, // 0.328125
    True43False19, // 0.3125
    True44False18, // 0.296875
    True45False17, // 0.28125
    True46False16, // 0.265625
    True47False15, // 0.25
    True48False14, // 0.234375
    True49False13, // 0.21875
    True50False12, // 0.203125
    True51False11, // 0.1875
    True52False10, // 0.171875
    True53False9,  // 0.15625
    True54False8,  // 0.140625
    True55False7,  // 0.125
    True56False6,  // 0.109375
    True57False5,  // 0.09375
    True58False4,  // 0.078125
    True59False3,  // 0.0625
    True60False2,  // 0.046875
    True61False1,  // 0.03125
    True62False0,  // 0.015625
    True0False63,  // 0.98828125
    True1False62,  // 0.96875
    True2False61,  // 0.953125
    True3False60,  // 0.9375
    True4False59,  // 0.921875
    True5False58,  // 0.90625
    True6False57,  // 0.890625
    True7False56,  // 0.875
    True8False55,  // 0.86328125
    True9False54,  // 0.84765625
    True10False53, // 0.83203125
    True11False52, // 0.81640625
    True12False51, // 0.80078125
    True13False50, // 0.78515625
    True14False49, // 0.76953125
    True15False48, // 0.75390625
    True16False47, // 0.73828125
    True17False46, // 0.72265625
    True18False45, // 0.70703125
    True19False44, // 0.69140625
    True20False43, // 0.67578125
    True21False42, // 0.66015625
    True22False41, // 0.64453125
    True23False40, // 0.62890625
    True24False39, // 0.6171875
    True25False38, // 0.6015625
    True26False37, // 0.5859375
    True27False36, // 0.5703125
    True28False35, // 0.5546875
    True29False34, // 0.5390625
    True30False33, // 0.5234375
    True31False32, // 0.5078125
    True32False31, // 0.4921875
    True33False30, // 0.4765625
    True34False29, // 0.4609375
    True35False28, // 0.4453125
    True36False27, // 0.4296875
    True37False26, // 0.4140625
    True38False25, // 0.3984375
    True39False24, // 0.3828125
    True40False23, // 0.37109375
    True41False22, // 0.35546875
    True42False21, // 0.33984375
    True43False20, // 0.32421875
    True44False19, // 0.30859375
    True45False18, // 0.29296875
    True46False17, // 0.27734375
    True47False16, // 0.26171875
    True48False15, // 0.24609375
    True49False14, // 0.23046875
    True50False13, // 0.21484375
    True51False12, // 0.19921875
    True52False11, // 0.18359375
    True53False10, // 0.16796875
    True54False9,  // 0.15234375
    True55False8,  // 0.13671875
    True56False7,  // 0.125
    True57False6,  // 0.109375
    True58False5,  // 0.09375
    True59False4,  // 0.078125
    True60False3,  // 0.0625
    True61False2,  // 0.046875
    True62False1,  // 0.03125
    True63False0,  // 0.01171875
    True0False64,  // 0.98828125
    True1False63,  // 0.96875
    True2False62,  // 0.953125
    True3False61,  // 0.9375
    True4False60,  // 0.92578125
    True5False59,  // 0.91015625
    True6False58,  // 0.89453125
    True7False57,  // 0.87890625
    True8False56,  // 0.86328125
    True9False55,  // 0.84765625
    True10False54, // 0.83203125
    True11False53, // 0.81640625
    True12False52, // 0.8046875
    True13False51, // 0.7890625
    True14False50, // 0.7734375
    True15False49, // 0.7578125
    True16False48, // 0.7421875
    True17False47, // 0.7265625
    True18False46, // 0.7109375
    True19False45, // 0.6953125
    True20False44, // 0.68359375
    True21False43, // 0.66796875
    True22False42, // 0.65234375
    True23False41, // 0.63671875
    True24False40, // 0.62109375
    True25False39, // 0.60546875
    True26False38, // 0.58984375
    True27False37, // 0.57421875
    True28False36, // 0.5625
    True29False35, // 0.546875
    True30False34, // 0.53125
    True31False33, // 0.515625
    True32False32, // 0.5
    True33False31, // 0.484375
    True34False30, // 0.46875
    True35False29, // 0.453125
    True36False28, // 0.4375
    True37False27, // 0.42578125
    True38False26, // 0.41015625
    True39False25, // 0.39453125
    True40False24, // 0.37890625
    True41False23, // 0.36328125
    True42False22, // 0.34765625
    True43False21, // 0.33203125
    True44False20, // 0.31640625
    True45False19, // 0.3046875
    True46False18, // 0.2890625
    True47False17, // 0.2734375
    True48False16, // 0.2578125
    True49False15, // 0.2421875
    True50False14, // 0.2265625
    True51False13, // 0.2109375
    True52False12, // 0.1953125
    True53False11, // 0.18359375
    True54False10, // 0.16796875
    True55False9,  // 0.15234375
    True56False8,  // 0.13671875
    True57False7,  // 0.12109375
    True58False6,  // 0.10546875
    True59False5,  // 0.08984375
    True60False4,  // 0.07421875
    True61False3,  // 0.0625
    True62False2,  // 0.046875
    True63False1,  // 0.03125
    True64False0,  // 0.01171875
    True0False65,  // 0.98828125
    True1False64,  // 0.96875
    True2False63,  // 0.95703125
    True3False62,  // 0.94140625
    True4False61,  // 0.92578125
    True5False60,  // 0.91015625
    True6False59,  // 0.89453125
    True7False58,  // 0.87890625
    True8False57,  // 0.8671875
    True9False56,  // 0.8515625
    True10False55, // 0.8359375
    True11False54, // 0.8203125
    True12False53, // 0.8046875
    True13False52, // 0.79296875
    True14False51, // 0.77734375
    True15False50, // 0.76171875
    True16False49, // 0.74609375
    True17False48, // 0.73046875
    True18False47, // 0.71484375
    True19False46, // 0.703125
    True20False45, // 0.6875
    True21False44, // 0.671875
    True22False43, // 0.65625
    True23False42, // 0.640625
    True24False41, // 0.625
    True25False40, // 0.61328125
    True26False39, // 0.59765625
    True27False38, // 0.58203125
    True28False37, // 0.56640625
    True29False36, // 0.55078125
    True30False35, // 0.5390625
    True31False34, // 0.5234375
    True32False33, // 0.5078125
    True33False32, // 0.4921875
    True34False31, // 0.4765625
    True35False30, // 0.4609375
    True36False29, // 0.44921875
    True37False28, // 0.43359375
    True38False27, // 0.41796875
    True39False26, // 0.40234375
    True40False25, // 0.38671875
    True41False24, // 0.375
    True42False23, // 0.359375
    True43False22, // 0.34375
    True44False21, // 0.328125
    True45False20, // 0.3125
    True46False19, // 0.296875
    True47False18, // 0.28515625
    True48False17, // 0.26953125
    True49False16, // 0.25390625
    True50False15, // 0.23828125
    True51False14, // 0.22265625
    True52False13, // 0.20703125
    True53False12, // 0.1953125
    True54False11, // 0.1796875
    True55False10, // 0.1640625
    True56False9,  // 0.1484375
    True57False8,  // 0.1328125
    True58False7,  // 0.12109375
    True59False6,  // 0.10546875
    True60False5,  // 0.08984375
    True61False4,  // 0.07421875
    True62False3,  // 0.05859375
    True63False2,  // 0.04296875
    True64False1,  // 0.03125
    True65False0,  // 0.01171875
    True0False66,  // 0.98828125
    True1False65,  // 0.96875
    True2False64,  // 0.95703125
    True3False63,  // 0.94140625
    True4False62,  // 0.92578125
    True5False61,  // 0.91015625
    True6False60,  // 0.8984375
    True7False59,  // 0.8828125
    True8False58,  // 0.8671875
    True9False57,  // 0.8515625
    True10False56, // 0.83984375
    True11False55, // 0.82421875
    True12False54, // 0.80859375
    True13False53, // 0.79296875
    True14False52, // 0.78125
    True15False51, // 0.765625
    True16False50, // 0.75
    True17False49, // 0.734375
    True18False48, // 0.71875
    True19False47, // 0.70703125
    True20False46, // 0.69140625
    True21False45, // 0.67578125
    True22False44, // 0.66015625
    True23False43, // 0.6484375
    True24False42, // 0.6328125
    True25False41, // 0.6171875
    True26False40, // 0.6015625
    True27False39, // 0.58984375
    True28False38, // 0.57421875
    True29False37, // 0.55859375
    True30False36, // 0.54296875
    True31False35, // 0.53125
    True32False34, // 0.515625
    True33False33, // 0.5
    True34False32, // 0.484375
    True35False31, // 0.46875
    True36False30, // 0.45703125
    True37False29, // 0.44140625
    True38False28, // 0.42578125
    True39False27, // 0.41015625
    True40False26, // 0.3984375
    True41False25, // 0.3828125
    True42False24, // 0.3671875
    True43False23, // 0.3515625
    True44False22, // 0.33984375
    True45False21, // 0.32421875
    True46False20, // 0.30859375
    True47False19, // 0.29296875
    True48False18, // 0.28125
    True49False17, // 0.265625
    True50False16, // 0.25
    True51False15, // 0.234375
    True52False14, // 0.21875
    True53False13, // 0.20703125
    True54False12, // 0.19140625
    True55False11, // 0.17578125
    True56False10, // 0.16015625
    True57False9,  // 0.1484375
    True58False8,  // 0.1328125
    True59False7,  // 0.1171875
    True60False6,  // 0.1015625
    True61False5,  // 0.08984375
    True62False4,  // 0.07421875
    True63False3,  // 0.05859375
    True64False2,  // 0.04296875
    True65False1,  // 0.03125
    True66False0,  // 0.01171875
    True0False67,  // 0.98828125
    True1False66,  // 0.97265625
    True2False65,  // 0.95703125
    True3False64,  // 0.94140625
    True4False63,  // 0.92578125
    True5False62,  // 0.9140625
    True6False61,  // 0.8984375
    True7False60,  // 0.8828125
    True8False59,  // 0.87109375
    True9False58,  // 0.85546875
    True10False57, // 0.83984375
    True11False56, // 0.82421875
    True12False55, // 0.8125
    True13False54, // 0.796875
    True14False53, // 0.78125
    True15False52, // 0.76953125
    True16False51, // 0.75390625
    True17False50, // 0.73828125
    True18False49, // 0.7265625
    True19False48, // 0.7109375
    True20False47, // 0.6953125
    True21False46, // 0.6796875
    True22False45, // 0.66796875
    True23False44, // 0.65234375
    True24False43, // 0.63671875
    True25False42, // 0.625
    True26False41, // 0.609375
    True27False40, // 0.59375
    True28False39, // 0.578125
    True29False38, // 0.56640625
    True30False37, // 0.55078125
    True31False36, // 0.53515625
    True32False35, // 0.5234375
    True33False34, // 0.5078125
    True34False33, // 0.4921875
    True35False32, // 0.4765625
    True36False31, // 0.46484375
    True37False30, // 0.44921875
    True38False29, // 0.43359375
    True39False28, // 0.421875
    True40False27, // 0.40625
    True41False26, // 0.390625
    True42False25, // 0.375
    True43False24, // 0.36328125
    True44False23, // 0.34765625
    True45False22, // 0.33203125
    True46False21, // 0.3203125
    True47False20, // 0.3046875
    True48False19, // 0.2890625
    True49False18, // 0.2734375
    True50False17, // 0.26171875
    True51False16, // 0.24609375
    True52False15, // 0.23046875
    True53False14, // 0.21875
    True54False13, // 0.203125
    True55False12, // 0.1875
    True56False11, // 0.17578125
    True57False10, // 0.16015625
    True58False9,  // 0.14453125
    True59False8,  // 0.12890625
    True60False7,  // 0.1171875
    True61False6,  // 0.1015625
    True62False5,  // 0.0859375
    True63False4,  // 0.07421875
    True64False3,  // 0.05859375
    True65False2,  // 0.04296875
    True66False1,  // 0.02734375
    True67False0,  // 0.01171875
    True0False68,  // 0.98828125
    True1False67,  // 0.97265625
    True2False66,  // 0.95703125
    True3False65,  // 0.94140625
    True4False64,  // 0.9296875
    True5False63,  // 0.9140625
    True6False62,  // 0.8984375
    True7False61,  // 0.88671875
    True8False60,  // 0.87109375
    True9False59,  // 0.85546875
    True10False58, // 0.84375
    True11False57, // 0.828125
    True12False56, // 0.8125
    True13False55, // 0.80078125
    True14False54, // 0.78515625
    True15False53, // 0.76953125
    True16False52, // 0.7578125
    True17False51, // 0.7421875
    True18False50, // 0.73046875
    True19False49, // 0.71484375
    True20False48, // 0.69921875
    True21False47, // 0.6875
    True22False46, // 0.671875
    True23False45, // 0.65625
    True24False44, // 0.64453125
    True25False43, // 0.62890625
    True26False42, // 0.61328125
    True27False41, // 0.6015625
    True28False40, // 0.5859375
    True29False39, // 0.5703125
    True30False38, // 0.55859375
    True31False37, // 0.54296875
    True32False36, // 0.52734375
    True33False35, // 0.515625
    True34False34, // 0.5
    True35False33, // 0.484375
    True36False32, // 0.47265625
    True37False31, // 0.45703125
    True38False30, // 0.44140625
    True39False29, // 0.4296875
    True40False28, // 0.4140625
    True41False27, // 0.3984375
    True42False26, // 0.38671875
    True43False25, // 0.37109375
    True44False24, // 0.35546875
    True45False23, // 0.34375
    True46False22, // 0.328125
    True47False21, // 0.3125
    True48False20, // 0.30078125
    True49False19, // 0.28515625
    True50False18, // 0.26953125
    True51False17, // 0.2578125
    True52False16, // 0.2421875
    True53False15, // 0.23046875
    True54False14, // 0.21484375
    True55False13, // 0.19921875
    True56False12, // 0.1875
    True57False11, // 0.171875
    True58False10, // 0.15625
    True59False9,  // 0.14453125
    True60False8,  // 0.12890625
    True61False7,  // 0.11328125
    True62False6,  // 0.1015625
    True63False5,  // 0.0859375
    True64False4,  // 0.0703125
    True65False3,  // 0.05859375
    True66False2,  // 0.04296875
    True67False1,  // 0.02734375
    True68False0,  // 0.01171875
    True0False69,  // 0.98828125
    True1False68,  // 0.97265625
    True2False67,  // 0.95703125
    True3False66,  // 0.9453125
    True4False65,  // 0.9296875
    True5False64,  // 0.9140625
    True6False63,  // 0.90234375
    True7False62,  // 0.88671875
    True8False61,  // 0.875
    True9False60,  // 0.859375
    True10False59, // 0.84375
    True11False58, // 0.83203125
    True12False57, // 0.81640625
    True13False56, // 0.8046875
    True14False55, // 0.7890625
    True15False54, // 0.7734375
    True16False53, // 0.76171875
    True17False52, // 0.74609375
    True18False51, // 0.73046875
    True19False50, // 0.71875
    True20False49, // 0.703125
    True21False48, // 0.69140625
    True22False47, // 0.67578125
    True23False46, // 0.66015625
    True24False45, // 0.6484375
    True25False44, // 0.6328125
    True26False43, // 0.62109375
    True27False42, // 0.60546875
    True28False41, // 0.58984375
    True29False40, // 0.578125
    True30False39, // 0.5625
    True31False38, // 0.55078125
    True32False37, // 0.53515625
    True33False36, // 0.51953125
    True34False35, // 0.5078125
    True35False34, // 0.4921875
    True36False33, // 0.48046875
    True37False32, // 0.46484375
    True38False31, // 0.44921875
    True39False30, // 0.4375
    True40False29, // 0.421875
    True41False28, // 0.41015625
    True42False27, // 0.39453125
    True43False26, // 0.37890625
    True44False25, // 0.3671875
    True45False24, // 0.3515625
    True46False23, // 0.33984375
    True47False22, // 0.32421875
    True48False21, // 0.30859375
    True49False20, // 0.296875
    True50False19, // 0.28125
    True51False18, // 0.26953125
    True52False17, // 0.25390625
    True53False16, // 0.23828125
    True54False15, // 0.2265625
    True55False14, // 0.2109375
    True56False13, // 0.1953125
    True57False12, // 0.18359375
    True58False11, // 0.16796875
    True59False10, // 0.15625
    True60False9,  // 0.140625
    True61False8,  // 0.125
    True62False7,  // 0.11328125
    True63False6,  // 0.09765625
    True64False5,  // 0.0859375
    True65False4,  // 0.0703125
    True66False3,  // 0.0546875
    True67False2,  // 0.04296875
    True68False1,  // 0.02734375
    True69False0,  // 0.01171875
    True0False70,  // 0.98828125
    True1False69,  // 0.97265625
    True2False68,  // 0.95703125
    True3False67,  // 0.9453125
    True4False66,  // 0.9296875
    True5False65,  // 0.91796875
    True6False64,  // 0.90234375
    True7False63,  // 0.890625
    True8False62,  // 0.875
    True9False61,  // 0.859375
    True10False60, // 0.84765625
    True11False59, // 0.83203125
    True12False58, // 0.8203125
    True13False57, // 0.8046875
    True14False56, // 0.79296875
    True15False55, // 0.77734375
    True16False54, // 0.765625
    True17False53, // 0.75
    True18False52, // 0.734375
    True19False51, // 0.72265625
    True20False50, // 0.70703125
    True21False49, // 0.6953125
    True22False48, // 0.6796875
    True23False47, // 0.66796875
    True24False46, // 0.65234375
    True25False45, // 0.640625
    True26False44, // 0.625
    True27False43, // 0.609375
    True28False42, // 0.59765625
    True29False41, // 0.58203125
    True30False40, // 0.5703125
    True31False39, // 0.5546875
    True32False38, // 0.54296875
    True33False37, // 0.52734375
    True34False36, // 0.515625
    True35False35, // 0.5
    True36False34, // 0.484375
    True37False33, // 0.47265625
    True38False32, // 0.45703125
    True39False31, // 0.4453125
    True40False30, // 0.4296875
    True41False29, // 0.41796875
    True42False28, // 0.40234375
    True43False27, // 0.390625
    True44False26, // 0.375
    True45False25, // 0.359375
    True46False24, // 0.34765625
    True47False23, // 0.33203125
    True48False22, // 0.3203125
    True49False21, // 0.3046875
    True50False20, // 0.29296875
    True51False19, // 0.27734375
    True52False18, // 0.265625
    True53False17, // 0.25
    True54False16, // 0.234375
    True55False15, // 0.22265625
    True56False14, // 0.20703125
    True57False13, // 0.1953125
    True58False12, // 0.1796875
    True59False11, // 0.16796875
    True60False10, // 0.15234375
    True61False9,  // 0.140625
    True62False8,  // 0.125
    True63False7,  // 0.109375
    True64False6,  // 0.09765625
    True65False5,  // 0.08203125
    True66False4,  // 0.0703125
    True67False3,  // 0.0546875
    True68False2,  // 0.04296875
    True69False1,  // 0.02734375
    True70False0,  // 0.01171875
    True0False71,  // 0.98828125
    True1False70,  // 0.97265625
    True2False69,  // 0.95703125
    True3False68,  // 0.9453125
    True4False67,  // 0.9296875
    True5False66,  // 0.91796875
    True6False65,  // 0.90234375
    True7False64,  // 0.890625
    True8False63,  // 0.875
    True9False62,  // 0.86328125
    True10False61, // 0.84765625
    True11False60, // 0.8359375
    True12False59, // 0.8203125
    True13False58, // 0.80859375
    True14False57, // 0.79296875
    True15False56, // 0.78125
    True16False55, // 0.765625
    True17False54, // 0.75390625
    True18False53, // 0.73828125
    True19False52, // 0.7265625
    True20False51, // 0.7109375
    True21False50, // 0.69921875
    True22False49, // 0.68359375
    True23False48, // 0.671875
    True24False47, // 0.65625
    True25False46, // 0.64453125
    True26False45, // 0.62890625
    True27False44, // 0.6171875
    True28False43, // 0.6015625
    True29False42, // 0.58984375
    True30False41, // 0.57421875
    True31False40, // 0.5625
    True32False39, // 0.546875
    True33False38, // 0.53515625
    True34False37, // 0.51953125
    True35False36, // 0.5078125
    True36False35, // 0.4921875
    True37False34, // 0.48046875
    True38False33, // 0.46484375
    True39False32, // 0.453125
    True40False31, // 0.4375
    True41False30, // 0.42578125
    True42False29, // 0.41015625
    True43False28, // 0.3984375
    True44False27, // 0.3828125
    True45False26, // 0.37109375
    True46False25, // 0.35546875
    True47False24, // 0.34375
    True48False23, // 0.328125
    True49False22, // 0.31640625
    True50False21, // 0.30078125
    True51False20, // 0.2890625
    True52False19, // 0.2734375
    True53False18, // 0.26171875
    True54False17, // 0.24609375
    True55False16, // 0.234375
    True56False15, // 0.21875
    True57False14, // 0.20703125
    True58False13, // 0.19140625
    True59False12, // 0.1796875
    True60False11, // 0.1640625
    True61False10, // 0.15234375
    True62False9,  // 0.13671875
    True63False8,  // 0.125
    True64False7,  // 0.109375
    True65False6,  // 0.09765625
    True66False5,  // 0.08203125
    True67False4,  // 0.0703125
    True68False3,  // 0.0546875
    True69False2,  // 0.04296875
    True70False1,  // 0.02734375
    True71False0,  // 0.01171875
    True0False72,  // 0.98828125
    True1False71,  // 0.97265625
    True2False70,  // 0.9609375
    True3False69,  // 0.9453125
    True4False68,  // 0.93359375
    True5False67,  // 0.91796875
    True6False66,  // 0.90625
    True7False65,  // 0.890625
    True8False64,  // 0.87890625
    True9False63,  // 0.86328125
    True10False62, // 0.8515625
    True11False61, // 0.8359375
    True12False60, // 0.82421875
    True13False59, // 0.8125
    True14False58, // 0.796875
    True15False57, // 0.78515625
    True16False56, // 0.76953125
    True17False55, // 0.7578125
    True18False54, // 0.7421875
    True19False53, // 0.73046875
    True20False52, // 0.71484375
    True21False51, // 0.703125
    True22False50, // 0.6875
    True23False49, // 0.67578125
    True24False48, // 0.6640625
    True25False47, // 0.6484375
    True26False46, // 0.63671875
    True27False45, // 0.62109375
    True28False44, // 0.609375
    True29False43, // 0.59375
    True30False42, // 0.58203125
    True31False41, // 0.56640625
    True32False40, // 0.5546875
    True33False39, // 0.5390625
    True34False38, // 0.52734375
    True35False37, // 0.51171875
    True36False36, // 0.5
    True37False35, // 0.48828125
    True38False34, // 0.47265625
    True39False33, // 0.4609375
    True40False32, // 0.4453125
    True41False31, // 0.43359375
    True42False30, // 0.41796875
    True43False29, // 0.40625
    True44False28, // 0.390625
    True45False27, // 0.37890625
    True46False26, // 0.36328125
    True47False25, // 0.3515625
    True48False24, // 0.3359375
    True49False23, // 0.32421875
    True50False22, // 0.3125
    True51False21, // 0.296875
    True52False20, // 0.28515625
    True53False19, // 0.26953125
    True54False18, // 0.2578125
    True55False17, // 0.2421875
    True56False16, // 0.23046875
    True57False15, // 0.21484375
    True58False14, // 0.203125
    True59False13, // 0.1875
    True60False12, // 0.17578125
    True61False11, // 0.1640625
    True62False10, // 0.1484375
    True63False9,  // 0.13671875
    True64False8,  // 0.12109375
    True65False7,  // 0.109375
    True66False6,  // 0.09375
    True67False5,  // 0.08203125
    True68False4,  // 0.06640625
    True69False3,  // 0.0546875
    True70False2,  // 0.0390625
    True71False1,  // 0.02734375
    True72False0,  // 0.01171875
    True0False73,  // 0.98828125
    True1False72,  // 0.97265625
    True2False71,  // 0.9609375
    True3False70,  // 0.9453125
    True4False69,  // 0.93359375
    True5False68,  // 0.921875
    True6False67,  // 0.90625
    True7False66,  // 0.89453125
    True8False65,  // 0.87890625
    True9False64,  // 0.8671875
    True10False63, // 0.8515625
    True11False62, // 0.83984375
    True12False61, // 0.828125
    True13False60, // 0.8125
    True14False59, // 0.80078125
    True15False58, // 0.78515625
    True16False57, // 0.7734375
    True17False56, // 0.76171875
    True18False55, // 0.74609375
    True19False54, // 0.734375
    True20False53, // 0.71875
    True21False52, // 0.70703125
    True22False51, // 0.69140625
    True23False50, // 0.6796875
    True24False49, // 0.66796875
    True25False48, // 0.65234375
    True26False47, // 0.640625
    True27False46, // 0.625
    True28False45, // 0.61328125
    True29False44, // 0.6015625
    True30False43, // 0.5859375
    True31False42, // 0.57421875
    True32False41, // 0.55859375
    True33False40, // 0.546875
    True34False39, // 0.53515625
    True35False38, // 0.51953125
    True36False37, // 0.5078125
    True37False36, // 0.4921875
    True38False35, // 0.48046875
    True39False34, // 0.46484375
    True40False33, // 0.453125
    True41False32, // 0.44140625
    True42False31, // 0.42578125
    True43False30, // 0.4140625
    True44False29, // 0.3984375
    True45False28, // 0.38671875
    True46False27, // 0.375
    True47False26, // 0.359375
    True48False25, // 0.34765625
    True49False24, // 0.33203125
    True50False23, // 0.3203125
    True51False22, // 0.30859375
    True52False21, // 0.29296875
    True53False20, // 0.28125
    True54False19, // 0.265625
    True55False18, // 0.25390625
    True56False17, // 0.23828125
    True57False16, // 0.2265625
    True58False15, // 0.21484375
    True59False14, // 0.19921875
    True60False13, // 0.1875
    True61False12, // 0.171875
    True62False11, // 0.16015625
    True63False10, // 0.1484375
    True64False9,  // 0.1328125
    True65False8,  // 0.12109375
    True66False7,  // 0.10546875
    True67False6,  // 0.09375
    True68False5,  // 0.078125
    True69False4,  // 0.06640625
    True70False3,  // 0.0546875
    True71False2,  // 0.0390625
    True72False1,  // 0.02734375
    True73False0,  // 0.01171875
    True0False74,  // 0.98828125
    True1False73,  // 0.97265625
    True2False72,  // 0.9609375
    True3False71,  // 0.94921875
    True4False70,  // 0.93359375
    True5False69,  // 0.921875
    True6False68,  // 0.90625
    True7False67,  // 0.89453125
    True8False66,  // 0.8828125
    True9False65,  // 0.8671875
    True10False64, // 0.85546875
    True11False63, // 0.84375
    True12False62, // 0.828125
    True13False61, // 0.81640625
    True14False60, // 0.80078125
    True15False59, // 0.7890625
    True16False58, // 0.77734375
    True17False57, // 0.76171875
    True18False56, // 0.75
    True19False55, // 0.73828125
    True20False54, // 0.72265625
    True21False53, // 0.7109375
    True22False52, // 0.69921875
    True23False51, // 0.68359375
    True24False50, // 0.671875
    True25False49, // 0.65625
    True26False48, // 0.64453125
    True27False47, // 0.6328125
    True28False46, // 0.6171875
    True29False45, // 0.60546875
    True30False44, // 0.59375
    True31False43, // 0.578125
    True32False42, // 0.56640625
    True33False41, // 0.55078125
    True34False40, // 0.5390625
    True35False39, // 0.52734375
    True36False38, // 0.51171875
    True37False37, // 0.5
    True38False36, // 0.48828125
    True39False35, // 0.47265625
    True40False34, // 0.4609375
    True41False33, // 0.44921875
    True42False32, // 0.43359375
    True43False31, // 0.421875
    True44False30, // 0.40625
    True45False29, // 0.39453125
    True46False28, // 0.3828125
    True47False27, // 0.3671875
    True48False26, // 0.35546875
    True49False25, // 0.34375
    True50False24, // 0.328125
    True51False23, // 0.31640625
    True52False22, // 0.30078125
    True53False21, // 0.2890625
    True54False20, // 0.27734375
    True55False19, // 0.26171875
    True56False18, // 0.25
    True57False17, // 0.23828125
    True58False16, // 0.22265625
    True59False15, // 0.2109375
    True60False14, // 0.19921875
    True61False13, // 0.18359375
    True62False12, // 0.171875
    True63False11, // 0.15625
    True64False10, // 0.14453125
    True65False9,  // 0.1328125
    True66False8,  // 0.1171875
    True67False7,  // 0.10546875
    True68False6,  // 0.09375
    True69False5,  // 0.078125
    True70False4,  // 0.06640625
    True71False3,  // 0.05078125
    True72False2,  // 0.0390625
    True73False1,  // 0.02734375
    True74False0,  // 0.01171875
    True0False75,  // 0.98828125
    True1False74,  // 0.97265625
    True2False73,  // 0.9609375
    True3False72,  // 0.94921875
    True4False71,  // 0.93359375
    True5False70,  // 0.921875
    True6False69,  // 0.91015625
    True7False68,  // 0.89453125
    True8False67,  // 0.8828125
    True9False66,  // 0.87109375
    True10False65, // 0.85546875
    True11False64, // 0.84375
    True12False63, // 0.83203125
    True13False62, // 0.81640625
    True14False61, // 0.8046875
    True15False60, // 0.79296875
    True16False59, // 0.77734375
    True17False58, // 0.765625
    True18False57, // 0.75390625
    True19False56, // 0.7421875
    True20False55, // 0.7265625
    True21False54, // 0.71484375
    True22False53, // 0.703125
    True23False52, // 0.6875
    True24False51, // 0.67578125
    True25False50, // 0.6640625
    True26False49, // 0.6484375
    True27False48, // 0.63671875
    True28False47, // 0.625
    True29False46, // 0.609375
    True30False45, // 0.59765625
    True31False44, // 0.5859375
    True32False43, // 0.5703125
    True33False42, // 0.55859375
    True34False41, // 0.546875
    True35False40, // 0.53125
    True36False39, // 0.51953125
    True37False38, // 0.5078125
    True38False37, // 0.4921875
    True39False36, // 0.48046875
    True40False35, // 0.46875
    True41False34, // 0.453125
    True42False33, // 0.44140625
    True43False32, // 0.4296875
    True44False31, // 0.4140625
    True45False30, // 0.40234375
    True46False29, // 0.390625
    True47False28, // 0.375
    True48False27, // 0.36328125
    True49False26, // 0.3515625
    True50False25, // 0.3359375
    True51False24, // 0.32421875
    True52False23, // 0.3125
    True53False22, // 0.296875
    True54False21, // 0.28515625
    True55False20, // 0.2734375
    True56False19, // 0.2578125
    True57False18, // 0.24609375
    True58False17, // 0.234375
    True59False16, // 0.22265625
    True60False15, // 0.20703125
    True61False14, // 0.1953125
    True62False13, // 0.18359375
    True63False12, // 0.16796875
    True64False11, // 0.15625
    True65False10, // 0.14453125
    True66False9,  // 0.12890625
    True67False8,  // 0.1171875
    True68False7,  // 0.10546875
    True69False6,  // 0.08984375
    True70False5,  // 0.078125
    True71False4,  // 0.06640625
    True72False3,  // 0.05078125
    True73False2,  // 0.0390625
    True74False1,  // 0.02734375
    True75False0,  // 0.01171875
    True0False76,  // 0.98828125
    True1False75,  // 0.97265625
    True2False74,  // 0.9609375
    True3False73,  // 0.94921875
    True4False72,  // 0.9375
    True5False71,  // 0.921875
    True6False70,  // 0.91015625
    True7False69,  // 0.8984375
    True8False68,  // 0.8828125
    True9False67,  // 0.87109375
    True10False66, // 0.859375
    True11False65, // 0.84765625
    True12False64, // 0.83203125
    True13False63, // 0.8203125
    True14False62, // 0.80859375
    True15False61, // 0.79296875
    True16False60, // 0.78125
    True17False59, // 0.76953125
    True18False58, // 0.7578125
    True19False57, // 0.7421875
    True20False56, // 0.73046875
    True21False55, // 0.71875
    True22False54, // 0.70703125
    True23False53, // 0.69140625
    True24False52, // 0.6796875
    True25False51, // 0.66796875
    True26False50, // 0.65234375
    True27False49, // 0.640625
    True28False48, // 0.62890625
    True29False47, // 0.6171875
    True30False46, // 0.6015625
    True31False45, // 0.58984375
    True32False44, // 0.578125
    True33False43, // 0.5625
    True34False42, // 0.55078125
    True35False41, // 0.5390625
    True36False40, // 0.52734375
    True37False39, // 0.51171875
    True38False38, // 0.5
    True39False37, // 0.48828125
    True40False36, // 0.47265625
    True41False35, // 0.4609375
    True42False34, // 0.44921875
    True43False33, // 0.4375
    True44False32, // 0.421875
    True45False31, // 0.41015625
    True46False30, // 0.3984375
    True47False29, // 0.3828125
    True48False28, // 0.37109375
    True49False27, // 0.359375
    True50False26, // 0.34765625
    True51False25, // 0.33203125
    True52False24, // 0.3203125
    True53False23, // 0.30859375
    True54False22, // 0.29296875
    True55False21, // 0.28125
    True56False20, // 0.26953125
    True57False19, // 0.2578125
    True58False18, // 0.2421875
    True59False17, // 0.23046875
    True60False16, // 0.21875
    True61False15, // 0.20703125
    True62False14, // 0.19140625
    True63False13, // 0.1796875
    True64False12, // 0.16796875
    True65False11, // 0.15234375
    True66False10, // 0.140625
    True67False9,  // 0.12890625
    True68False8,  // 0.1171875
    True69False7,  // 0.1015625
    True70False6,  // 0.08984375
    True71False5,  // 0.078125
    True72False4,  // 0.0625
    True73False3,  // 0.05078125
    True74False2,  // 0.0390625
    True75False1,  // 0.02734375
    True76False0,  // 0.01171875
    True0False77,  // 0.98828125
    True1False76,  // 0.97265625
    True2False75,  // 0.9609375
    True3False74,  // 0.94921875
    True4False73,  // 0.9375
    True5False72,  // 0.92578125
    True6False71,  // 0.91015625
    True7False70,  // 0.8984375
    True8False69,  // 0.88671875
    True9False68,  // 0.875
    True10False67, // 0.859375
    True11False66, // 0.84765625
    True12False65, // 0.8359375
    True13False64, // 0.82421875
    True14False63, // 0.80859375
    True15False62, // 0.796875
    True16False61, // 0.78515625
    True17False60, // 0.7734375
    True18False59, // 0.7578125
    True19False58, // 0.74609375
    True20False57, // 0.734375
    True21False56, // 0.72265625
    True22False55, // 0.70703125
    True23False54, // 0.6953125
    True24False53, // 0.68359375
    True25False52, // 0.671875
    True26False51, // 0.66015625
    True27False50, // 0.64453125
    True28False49, // 0.6328125
    True29False48, // 0.62109375
    True30False47, // 0.609375
    True31False46, // 0.59375
    True32False45, // 0.58203125
    True33False44, // 0.5703125
    True34False43, // 0.55859375
    True35False42, // 0.54296875
    True36False41, // 0.53125
    True37False40, // 0.51953125
    True38False39, // 0.5078125
    True39False38, // 0.4921875
    True40False37, // 0.48046875
    True41False36, // 0.46875
    True42False35, // 0.45703125
    True43False34, // 0.44140625
    True44False33, // 0.4296875
    True45False32, // 0.41796875
    True46False31, // 0.40625
    True47False30, // 0.390625
    True48False29, // 0.37890625
    True49False28, // 0.3671875
    True50False27, // 0.35546875
    True51False26, // 0.33984375
    True52False25, // 0.328125
    True53False24, // 0.31640625
    True54False23, // 0.3046875
    True55False22, // 0.29296875
    True56False21, // 0.27734375
    True57False20, // 0.265625
    True58False19, // 0.25390625
    True59False18, // 0.2421875
    True60False17, // 0.2265625
    True61False16, // 0.21484375
    True62False15, // 0.203125
    True63False14, // 0.19140625
    True64False13, // 0.17578125
    True65False12, // 0.1640625
    True66False11, // 0.15234375
    True67False10, // 0.140625
    True68False9,  // 0.125
    True69False8,  // 0.11328125
    True70False7,  // 0.1015625
    True71False6,  // 0.08984375
    True72False5,  // 0.07421875
    True73False4,  // 0.0625
    True74False3,  // 0.05078125
    True75False2,  // 0.0390625
    True76False1,  // 0.02734375
    True77False0,  // 0.01171875
    True0False78,  // 0.98828125
    True1False77,  // 0.9765625
    True2False76,  // 0.9609375
    True3False75,  // 0.94921875
    True4False74,  // 0.9375
    True5False73,  // 0.92578125
    True6False72,  // 0.9140625
    True7False71,  // 0.8984375
    True8False70,  // 0.88671875
    True9False69,  // 0.875
    True10False68, // 0.86328125
    True11False67, // 0.8515625
    True12False66, // 0.8359375
    True13False65, // 0.82421875
    True14False64, // 0.8125
    True15False63, // 0.80078125
    True16False62, // 0.7890625
    True17False61, // 0.7734375
    True18False60, // 0.76171875
    True19False59, // 0.75
    True20False58, // 0.73828125
    True21False57, // 0.7265625
    True22False56, // 0.7109375
    True23False55, // 0.69921875
    True24False54, // 0.6875
    True25False53, // 0.67578125
    True26False52, // 0.6640625
    True27False51, // 0.6484375
    True28False50, // 0.63671875
    True29False49, // 0.625
    True30False48, // 0.61328125
    True31False47, // 0.6015625
    True32False46, // 0.5859375
    True33False45, // 0.57421875
    True34False44, // 0.5625
    True35False43, // 0.55078125
    True36False42, // 0.5390625
    True37False41, // 0.5234375
    True38False40, // 0.51171875
    True39False39, // 0.5
    True40False38, // 0.48828125
    True41False37, // 0.4765625
    True42False36, // 0.4609375
    True43False35, // 0.44921875
    True44False34, // 0.4375
    True45False33, // 0.42578125
    True46False32, // 0.4140625
    True47False31, // 0.3984375
    True48False30, // 0.38671875
    True49False29, // 0.375
    True50False28, // 0.36328125
    True51False27, // 0.3515625
    True52False26, // 0.3359375
    True53False25, // 0.32421875
    True54False24, // 0.3125
    True55False23, // 0.30078125
    True56False22, // 0.2890625
    True57False21, // 0.2734375
    True58False20, // 0.26171875
    True59False19, // 0.25
    True60False18, // 0.23828125
    True61False17, // 0.2265625
    True62False16, // 0.2109375
    True63False15, // 0.19921875
    True64False14, // 0.1875
    True65False13, // 0.17578125
    True66False12, // 0.1640625
    True67False11, // 0.1484375
    True68False10, // 0.13671875
    True69False9,  // 0.125
    True70False8,  // 0.11328125
    True71False7,  // 0.1015625
    True72False6,  // 0.0859375
    True73False5,  // 0.07421875
    True74False4,  // 0.0625
    True75False3,  // 0.05078125
    True76False2,  // 0.0390625
    True77False1,  // 0.0234375
    True78False0,  // 0.01171875
    True0False79,  // 0.98828125
    True1False78,  // 0.9765625
    True2False77,  // 0.96484375
    True3False76,  // 0.94921875
    True4False75,  // 0.9375
    True5False74,  // 0.92578125
    True6False73,  // 0.9140625
    True7False72,  // 0.90234375
    True8False71,  // 0.890625
    True9False70,  // 0.875
    True10False69, // 0.86328125
    True11False68, // 0.8515625
    True12False67, // 0.83984375
    True13False66, // 0.828125
    True14False65, // 0.81640625
    True15False64, // 0.80078125
    True16False63, // 0.7890625
    True17False62, // 0.77734375
    True18False61, // 0.765625
    True19False60, // 0.75390625
    True20False59, // 0.7421875
    True21False58, // 0.7265625
    True22False57, // 0.71484375
    True23False56, // 0.703125
    True24False55, // 0.69140625
    True25False54, // 0.6796875
    True26False53, // 0.66796875
    True27False52, // 0.65625
    True28False51, // 0.640625
    True29False50, // 0.62890625
    True30False49, // 0.6171875
    True31False48, // 0.60546875
    True32False47, // 0.59375
    True33False46, // 0.58203125
    True34False45, // 0.56640625
    True35False44, // 0.5546875
    True36False43, // 0.54296875
    True37False42, // 0.53125
    True38False41, // 0.51953125
    True39False40, // 0.5078125
    True40False39, // 0.4921875
    True41False38, // 0.48046875
    True42False37, // 0.46875
    True43False36, // 0.45703125
    True44False35, // 0.4453125
    True45False34, // 0.43359375
    True46False33, // 0.41796875
    True47False32, // 0.40625
    True48False31, // 0.39453125
    True49False30, // 0.3828125
    True50False29, // 0.37109375
    True51False28, // 0.359375
    True52False27, // 0.34375
    True53False26, // 0.33203125
    True54False25, // 0.3203125
    True55False24, // 0.30859375
    True56False23, // 0.296875
    True57False22, // 0.28515625
    True58False21, // 0.2734375
    True59False20, // 0.2578125
    True60False19, // 0.24609375
    True61False18, // 0.234375
    True62False17, // 0.22265625
    True63False16, // 0.2109375
    True64False15, // 0.19921875
    True65False14, // 0.18359375
    True66False13, // 0.171875
    True67False12, // 0.16015625
    True68False11, // 0.1484375
    True69False10, // 0.13671875
    True70False9,  // 0.125
    True71False8,  // 0.109375
    True72False7,  // 0.09765625
    True73False6,  // 0.0859375
    True74False5,  // 0.07421875
    True75False4,  // 0.0625
    True76False3,  // 0.05078125
    True77False2,  // 0.03515625
    True78False1,  // 0.0234375
    True79False0,  // 0.01171875
    True0False80,  // 0.98828125
    True1False79,  // 0.9765625
    True2False78,  // 0.96484375
    True3False77,  // 0.953125
    True4False76,  // 0.9375
    True5False75,  // 0.92578125
    True6False74,  // 0.9140625
    True7False73,  // 0.90234375
    True8False72,  // 0.890625
    True9False71,  // 0.87890625
    True10False70, // 0.8671875
    True11False69, // 0.85546875
    True12False68, // 0.83984375
    True13False67, // 0.828125
    True14False66, // 0.81640625
    True15False65, // 0.8046875
    True16False64, // 0.79296875
    True17False63, // 0.78125
    True18False62, // 0.76953125
    True19False61, // 0.7578125
    True20False60, // 0.7421875
    True21False59, // 0.73046875
    True22False58, // 0.71875
    True23False57, // 0.70703125
    True24False56, // 0.6953125
    True25False55, // 0.68359375
    True26False54, // 0.671875
    True27False53, // 0.66015625
    True28False52, // 0.64453125
    True29False51, // 0.6328125
    True30False50, // 0.62109375
    True31False49, // 0.609375
    True32False48, // 0.59765625
    True33False47, // 0.5859375
    True34False46, // 0.57421875
    True35False45, // 0.5625
    True36False44, // 0.546875
    True37False43, // 0.53515625
    True38False42, // 0.5234375
    True39False41, // 0.51171875
    True40False40, // 0.5
    True41False39, // 0.48828125
    True42False38, // 0.4765625
    True43False37, // 0.46484375
    True44False36, // 0.453125
    True45False35, // 0.4375
    True46False34, // 0.42578125
    True47False33, // 0.4140625
    True48False32, // 0.40234375
    True49False31, // 0.390625
    True50False30, // 0.37890625
    True51False29, // 0.3671875
    True52False28, // 0.35546875
    True53False27, // 0.33984375
    True54False26, // 0.328125
    True55False25, // 0.31640625
    True56False24, // 0.3046875
    True57False23, // 0.29296875
    True58False22, // 0.28125
    True59False21, // 0.26953125
    True60False20, // 0.2578125
    True61False19, // 0.2421875
    True62False18, // 0.23046875
    True63False17, // 0.21875
    True64False16, // 0.20703125
    True65False15, // 0.1953125
    True66False14, // 0.18359375
    True67False13, // 0.171875
    True68False12, // 0.16015625
    True69False11, // 0.14453125
    True70False10, // 0.1328125
    True71False9,  // 0.12109375
    True72False8,  // 0.109375
    True73False7,  // 0.09765625
    True74False6,  // 0.0859375
    True75False5,  // 0.07421875
    True76False4,  // 0.0625
    True77False3,  // 0.046875
    True78False2,  // 0.03515625
    True79False1,  // 0.0234375
    True80False0,  // 0.01171875
}
use BitContext::*;

impl BitContext {
    #[inline]
    pub fn probability(self) -> Probability {
        const LOOKUP: [Probability; 3321] = [
            Probability::new(128, 128),
            Probability::new(85, 171),
            Probability::new(171, 85),
            Probability::new(64, 192),
            Probability::new(128, 128),
            Probability::new(192, 64),
            Probability::new(51, 205),
            Probability::new(102, 154),
            Probability::new(154, 102),
            Probability::new(205, 51),
            Probability::new(42, 214),
            Probability::new(85, 171),
            Probability::new(128, 128),
            Probability::new(171, 85),
            Probability::new(214, 42),
            Probability::new(36, 220),
            Probability::new(73, 183),
            Probability::new(110, 146),
            Probability::new(146, 110),
            Probability::new(183, 73),
            Probability::new(220, 36),
            Probability::new(32, 224),
            Probability::new(64, 192),
            Probability::new(96, 160),
            Probability::new(128, 128),
            Probability::new(160, 96),
            Probability::new(192, 64),
            Probability::new(224, 32),
            Probability::new(28, 228),
            Probability::new(57, 199),
            Probability::new(85, 171),
            Probability::new(114, 142),
            Probability::new(142, 114),
            Probability::new(171, 85),
            Probability::new(199, 57),
            Probability::new(228, 28),
            Probability::new(25, 231),
            Probability::new(51, 205),
            Probability::new(77, 179),
            Probability::new(102, 154),
            Probability::new(128, 128),
            Probability::new(154, 102),
            Probability::new(179, 77),
            Probability::new(205, 51),
            Probability::new(231, 25),
            Probability::new(23, 233),
            Probability::new(47, 209),
            Probability::new(70, 186),
            Probability::new(93, 163),
            Probability::new(116, 140),
            Probability::new(140, 116),
            Probability::new(163, 93),
            Probability::new(186, 70),
            Probability::new(209, 47),
            Probability::new(233, 23),
            Probability::new(21, 235),
            Probability::new(43, 213),
            Probability::new(64, 192),
            Probability::new(85, 171),
            Probability::new(107, 149),
            Probability::new(128, 128),
            Probability::new(149, 107),
            Probability::new(171, 85),
            Probability::new(192, 64),
            Probability::new(213, 43),
            Probability::new(235, 21),
            Probability::new(19, 237),
            Probability::new(39, 217),
            Probability::new(59, 197),
            Probability::new(79, 177),
            Probability::new(98, 158),
            Probability::new(118, 138),
            Probability::new(138, 118),
            Probability::new(158, 98),
            Probability::new(177, 79),
            Probability::new(197, 59),
            Probability::new(217, 39),
            Probability::new(237, 19),
            Probability::new(18, 238),
            Probability::new(37, 219),
            Probability::new(55, 201),
            Probability::new(73, 183),
            Probability::new(91, 165),
            Probability::new(110, 146),
            Probability::new(128, 128),
            Probability::new(146, 110),
            Probability::new(165, 91),
            Probability::new(183, 73),
            Probability::new(201, 55),
            Probability::new(219, 37),
            Probability::new(238, 18),
            Probability::new(17, 239),
            Probability::new(34, 222),
            Probability::new(51, 205),
            Probability::new(68, 188),
            Probability::new(85, 171),
            Probability::new(102, 154),
            Probability::new(119, 137),
            Probability::new(137, 119),
            Probability::new(154, 102),
            Probability::new(171, 85),
            Probability::new(188, 68),
            Probability::new(205, 51),
            Probability::new(222, 34),
            Probability::new(239, 17),
            Probability::new(16, 240),
            Probability::new(32, 224),
            Probability::new(48, 208),
            Probability::new(64, 192),
            Probability::new(80, 176),
            Probability::new(96, 160),
            Probability::new(112, 144),
            Probability::new(128, 128),
            Probability::new(144, 112),
            Probability::new(160, 96),
            Probability::new(176, 80),
            Probability::new(192, 64),
            Probability::new(208, 48),
            Probability::new(224, 32),
            Probability::new(240, 16),
            Probability::new(15, 241),
            Probability::new(30, 226),
            Probability::new(45, 211),
            Probability::new(60, 196),
            Probability::new(75, 181),
            Probability::new(90, 166),
            Probability::new(105, 151),
            Probability::new(120, 136),
            Probability::new(136, 120),
            Probability::new(151, 105),
            Probability::new(166, 90),
            Probability::new(181, 75),
            Probability::new(196, 60),
            Probability::new(211, 45),
            Probability::new(226, 30),
            Probability::new(241, 15),
            Probability::new(14, 242),
            Probability::new(28, 228),
            Probability::new(43, 213),
            Probability::new(57, 199),
            Probability::new(71, 185),
            Probability::new(85, 171),
            Probability::new(100, 156),
            Probability::new(114, 142),
            Probability::new(128, 128),
            Probability::new(142, 114),
            Probability::new(156, 100),
            Probability::new(171, 85),
            Probability::new(185, 71),
            Probability::new(199, 57),
            Probability::new(213, 43),
            Probability::new(228, 28),
            Probability::new(242, 14),
            Probability::new(13, 243),
            Probability::new(27, 229),
            Probability::new(40, 216),
            Probability::new(54, 202),
            Probability::new(67, 189),
            Probability::new(81, 175),
            Probability::new(94, 162),
            Probability::new(108, 148),
            Probability::new(121, 135),
            Probability::new(135, 121),
            Probability::new(148, 108),
            Probability::new(162, 94),
            Probability::new(175, 81),
            Probability::new(189, 67),
            Probability::new(202, 54),
            Probability::new(216, 40),
            Probability::new(229, 27),
            Probability::new(243, 13),
            Probability::new(12, 244),
            Probability::new(26, 230),
            Probability::new(38, 218),
            Probability::new(51, 205),
            Probability::new(64, 192),
            Probability::new(77, 179),
            Probability::new(90, 166),
            Probability::new(102, 154),
            Probability::new(115, 141),
            Probability::new(128, 128),
            Probability::new(141, 115),
            Probability::new(154, 102),
            Probability::new(166, 90),
            Probability::new(179, 77),
            Probability::new(192, 64),
            Probability::new(205, 51),
            Probability::new(218, 38),
            Probability::new(230, 26),
            Probability::new(244, 12),
            Probability::new(12, 244),
            Probability::new(24, 232),
            Probability::new(37, 219),
            Probability::new(49, 207),
            Probability::new(61, 195),
            Probability::new(73, 183),
            Probability::new(85, 171),
            Probability::new(98, 158),
            Probability::new(110, 146),
            Probability::new(122, 134),
            Probability::new(134, 122),
            Probability::new(146, 110),
            Probability::new(158, 98),
            Probability::new(171, 85),
            Probability::new(183, 73),
            Probability::new(195, 61),
            Probability::new(207, 49),
            Probability::new(219, 37),
            Probability::new(232, 24),
            Probability::new(244, 12),
            Probability::new(11, 245),
            Probability::new(23, 233),
            Probability::new(35, 221),
            Probability::new(47, 209),
            Probability::new(58, 198),
            Probability::new(70, 186),
            Probability::new(81, 175),
            Probability::new(93, 163),
            Probability::new(105, 151),
            Probability::new(116, 140),
            Probability::new(128, 128),
            Probability::new(140, 116),
            Probability::new(151, 105),
            Probability::new(163, 93),
            Probability::new(175, 81),
            Probability::new(186, 70),
            Probability::new(198, 58),
            Probability::new(209, 47),
            Probability::new(221, 35),
            Probability::new(233, 23),
            Probability::new(245, 11),
            Probability::new(11, 245),
            Probability::new(22, 234),
            Probability::new(33, 223),
            Probability::new(45, 211),
            Probability::new(56, 200),
            Probability::new(67, 189),
            Probability::new(78, 178),
            Probability::new(89, 167),
            Probability::new(100, 156),
            Probability::new(111, 145),
            Probability::new(122, 134),
            Probability::new(134, 122),
            Probability::new(145, 111),
            Probability::new(156, 100),
            Probability::new(167, 89),
            Probability::new(178, 78),
            Probability::new(189, 67),
            Probability::new(200, 56),
            Probability::new(211, 45),
            Probability::new(223, 33),
            Probability::new(234, 22),
            Probability::new(245, 11),
            Probability::new(10, 246),
            Probability::new(21, 235),
            Probability::new(32, 224),
            Probability::new(43, 213),
            Probability::new(53, 203),
            Probability::new(64, 192),
            Probability::new(75, 181),
            Probability::new(85, 171),
            Probability::new(96, 160),
            Probability::new(107, 149),
            Probability::new(117, 139),
            Probability::new(128, 128),
            Probability::new(139, 117),
            Probability::new(149, 107),
            Probability::new(160, 96),
            Probability::new(171, 85),
            Probability::new(181, 75),
            Probability::new(192, 64),
            Probability::new(203, 53),
            Probability::new(213, 43),
            Probability::new(224, 32),
            Probability::new(235, 21),
            Probability::new(246, 10),
            Probability::new(10, 246),
            Probability::new(20, 236),
            Probability::new(31, 225),
            Probability::new(41, 215),
            Probability::new(51, 205),
            Probability::new(61, 195),
            Probability::new(72, 184),
            Probability::new(82, 174),
            Probability::new(92, 164),
            Probability::new(102, 154),
            Probability::new(113, 143),
            Probability::new(123, 133),
            Probability::new(133, 123),
            Probability::new(143, 113),
            Probability::new(154, 102),
            Probability::new(164, 92),
            Probability::new(174, 82),
            Probability::new(184, 72),
            Probability::new(195, 61),
            Probability::new(205, 51),
            Probability::new(215, 41),
            Probability::new(225, 31),
            Probability::new(236, 20),
            Probability::new(246, 10),
            Probability::new(9, 247),
            Probability::new(20, 236),
            Probability::new(30, 226),
            Probability::new(39, 217),
            Probability::new(49, 207),
            Probability::new(59, 197),
            Probability::new(69, 187),
            Probability::new(79, 177),
            Probability::new(89, 167),
            Probability::new(98, 158),
            Probability::new(108, 148),
            Probability::new(118, 138),
            Probability::new(128, 128),
            Probability::new(138, 118),
            Probability::new(148, 108),
            Probability::new(158, 98),
            Probability::new(167, 89),
            Probability::new(177, 79),
            Probability::new(187, 69),
            Probability::new(197, 59),
            Probability::new(207, 49),
            Probability::new(217, 39),
            Probability::new(226, 30),
            Probability::new(236, 20),
            Probability::new(247, 9),
            Probability::new(9, 247),
            Probability::new(19, 237),
            Probability::new(28, 228),
            Probability::new(38, 218),
            Probability::new(47, 209),
            Probability::new(57, 199),
            Probability::new(66, 190),
            Probability::new(76, 180),
            Probability::new(85, 171),
            Probability::new(95, 161),
            Probability::new(104, 152),
            Probability::new(114, 142),
            Probability::new(123, 133),
            Probability::new(133, 123),
            Probability::new(142, 114),
            Probability::new(152, 104),
            Probability::new(161, 95),
            Probability::new(171, 85),
            Probability::new(180, 76),
            Probability::new(190, 66),
            Probability::new(199, 57),
            Probability::new(209, 47),
            Probability::new(218, 38),
            Probability::new(228, 28),
            Probability::new(237, 19),
            Probability::new(247, 9),
            Probability::new(9, 247),
            Probability::new(18, 238),
            Probability::new(27, 229),
            Probability::new(37, 219),
            Probability::new(46, 210),
            Probability::new(55, 201),
            Probability::new(64, 192),
            Probability::new(73, 183),
            Probability::new(82, 174),
            Probability::new(91, 165),
            Probability::new(101, 155),
            Probability::new(110, 146),
            Probability::new(119, 137),
            Probability::new(128, 128),
            Probability::new(137, 119),
            Probability::new(146, 110),
            Probability::new(155, 101),
            Probability::new(165, 91),
            Probability::new(174, 82),
            Probability::new(183, 73),
            Probability::new(192, 64),
            Probability::new(201, 55),
            Probability::new(210, 46),
            Probability::new(219, 37),
            Probability::new(229, 27),
            Probability::new(238, 18),
            Probability::new(247, 9),
            Probability::new(8, 248),
            Probability::new(18, 238),
            Probability::new(26, 230),
            Probability::new(35, 221),
            Probability::new(44, 212),
            Probability::new(53, 203),
            Probability::new(62, 194),
            Probability::new(71, 185),
            Probability::new(79, 177),
            Probability::new(88, 168),
            Probability::new(97, 159),
            Probability::new(106, 150),
            Probability::new(115, 141),
            Probability::new(124, 132),
            Probability::new(132, 124),
            Probability::new(141, 115),
            Probability::new(150, 106),
            Probability::new(159, 97),
            Probability::new(168, 88),
            Probability::new(177, 79),
            Probability::new(185, 71),
            Probability::new(194, 62),
            Probability::new(203, 53),
            Probability::new(212, 44),
            Probability::new(221, 35),
            Probability::new(230, 26),
            Probability::new(238, 18),
            Probability::new(248, 8),
            Probability::new(8, 248),
            Probability::new(17, 239),
            Probability::new(26, 230),
            Probability::new(34, 222),
            Probability::new(43, 213),
            Probability::new(51, 205),
            Probability::new(60, 196),
            Probability::new(68, 188),
            Probability::new(77, 179),
            Probability::new(85, 171),
            Probability::new(94, 162),
            Probability::new(102, 154),
            Probability::new(111, 145),
            Probability::new(119, 137),
            Probability::new(128, 128),
            Probability::new(137, 119),
            Probability::new(145, 111),
            Probability::new(154, 102),
            Probability::new(162, 94),
            Probability::new(171, 85),
            Probability::new(179, 77),
            Probability::new(188, 68),
            Probability::new(196, 60),
            Probability::new(205, 51),
            Probability::new(213, 43),
            Probability::new(222, 34),
            Probability::new(230, 26),
            Probability::new(239, 17),
            Probability::new(248, 8),
            Probability::new(8, 248),
            Probability::new(17, 239),
            Probability::new(25, 231),
            Probability::new(33, 223),
            Probability::new(41, 215),
            Probability::new(50, 206),
            Probability::new(58, 198),
            Probability::new(66, 190),
            Probability::new(74, 182),
            Probability::new(83, 173),
            Probability::new(91, 165),
            Probability::new(99, 157),
            Probability::new(107, 149),
            Probability::new(116, 140),
            Probability::new(124, 132),
            Probability::new(132, 124),
            Probability::new(140, 116),
            Probability::new(149, 107),
            Probability::new(157, 99),
            Probability::new(165, 91),
            Probability::new(173, 83),
            Probability::new(182, 74),
            Probability::new(190, 66),
            Probability::new(198, 58),
            Probability::new(206, 50),
            Probability::new(215, 41),
            Probability::new(223, 33),
            Probability::new(231, 25),
            Probability::new(239, 17),
            Probability::new(248, 8),
            Probability::new(8, 248),
            Probability::new(16, 240),
            Probability::new(24, 232),
            Probability::new(32, 224),
            Probability::new(40, 216),
            Probability::new(48, 208),
            Probability::new(56, 200),
            Probability::new(64, 192),
            Probability::new(72, 184),
            Probability::new(80, 176),
            Probability::new(88, 168),
            Probability::new(96, 160),
            Probability::new(104, 152),
            Probability::new(112, 144),
            Probability::new(120, 136),
            Probability::new(128, 128),
            Probability::new(136, 120),
            Probability::new(144, 112),
            Probability::new(152, 104),
            Probability::new(160, 96),
            Probability::new(168, 88),
            Probability::new(176, 80),
            Probability::new(184, 72),
            Probability::new(192, 64),
            Probability::new(200, 56),
            Probability::new(208, 48),
            Probability::new(216, 40),
            Probability::new(224, 32),
            Probability::new(232, 24),
            Probability::new(240, 16),
            Probability::new(248, 8),
            Probability::new(7, 249),
            Probability::new(16, 240),
            Probability::new(23, 233),
            Probability::new(31, 225),
            Probability::new(39, 217),
            Probability::new(47, 209),
            Probability::new(54, 202),
            Probability::new(62, 194),
            Probability::new(70, 186),
            Probability::new(78, 178),
            Probability::new(85, 171),
            Probability::new(93, 163),
            Probability::new(101, 155),
            Probability::new(109, 147),
            Probability::new(116, 140),
            Probability::new(124, 132),
            Probability::new(132, 124),
            Probability::new(140, 116),
            Probability::new(147, 109),
            Probability::new(155, 101),
            Probability::new(163, 93),
            Probability::new(171, 85),
            Probability::new(178, 78),
            Probability::new(186, 70),
            Probability::new(194, 62),
            Probability::new(202, 54),
            Probability::new(209, 47),
            Probability::new(217, 39),
            Probability::new(225, 31),
            Probability::new(233, 23),
            Probability::new(240, 16),
            Probability::new(249, 7),
            Probability::new(7, 249),
            Probability::new(15, 241),
            Probability::new(23, 233),
            Probability::new(30, 226),
            Probability::new(38, 218),
            Probability::new(45, 211),
            Probability::new(53, 203),
            Probability::new(60, 196),
            Probability::new(68, 188),
            Probability::new(75, 181),
            Probability::new(83, 173),
            Probability::new(90, 166),
            Probability::new(98, 158),
            Probability::new(105, 151),
            Probability::new(113, 143),
            Probability::new(120, 136),
            Probability::new(128, 128),
            Probability::new(136, 120),
            Probability::new(143, 113),
            Probability::new(151, 105),
            Probability::new(158, 98),
            Probability::new(166, 90),
            Probability::new(173, 83),
            Probability::new(181, 75),
            Probability::new(188, 68),
            Probability::new(196, 60),
            Probability::new(203, 53),
            Probability::new(211, 45),
            Probability::new(218, 38),
            Probability::new(226, 30),
            Probability::new(233, 23),
            Probability::new(241, 15),
            Probability::new(249, 7),
            Probability::new(7, 249),
            Probability::new(15, 241),
            Probability::new(22, 234),
            Probability::new(29, 227),
            Probability::new(37, 219),
            Probability::new(44, 212),
            Probability::new(51, 205),
            Probability::new(59, 197),
            Probability::new(66, 190),
            Probability::new(73, 183),
            Probability::new(80, 176),
            Probability::new(88, 168),
            Probability::new(95, 161),
            Probability::new(102, 154),
            Probability::new(110, 146),
            Probability::new(117, 139),
            Probability::new(124, 132),
            Probability::new(132, 124),
            Probability::new(139, 117),
            Probability::new(146, 110),
            Probability::new(154, 102),
            Probability::new(161, 95),
            Probability::new(168, 88),
            Probability::new(176, 80),
            Probability::new(183, 73),
            Probability::new(190, 66),
            Probability::new(197, 59),
            Probability::new(205, 51),
            Probability::new(212, 44),
            Probability::new(219, 37),
            Probability::new(227, 29),
            Probability::new(234, 22),
            Probability::new(241, 15),
            Probability::new(249, 7),
            Probability::new(7, 249),
            Probability::new(14, 242),
            Probability::new(21, 235),
            Probability::new(28, 228),
            Probability::new(36, 220),
            Probability::new(43, 213),
            Probability::new(50, 206),
            Probability::new(57, 199),
            Probability::new(64, 192),
            Probability::new(71, 185),
            Probability::new(78, 178),
            Probability::new(85, 171),
            Probability::new(92, 164),
            Probability::new(100, 156),
            Probability::new(107, 149),
            Probability::new(114, 142),
            Probability::new(121, 135),
            Probability::new(128, 128),
            Probability::new(135, 121),
            Probability::new(142, 114),
            Probability::new(149, 107),
            Probability::new(156, 100),
            Probability::new(164, 92),
            Probability::new(171, 85),
            Probability::new(178, 78),
            Probability::new(185, 71),
            Probability::new(192, 64),
            Probability::new(199, 57),
            Probability::new(206, 50),
            Probability::new(213, 43),
            Probability::new(220, 36),
            Probability::new(228, 28),
            Probability::new(235, 21),
            Probability::new(242, 14),
            Probability::new(249, 7),
            Probability::new(6, 250),
            Probability::new(14, 242),
            Probability::new(21, 235),
            Probability::new(28, 228),
            Probability::new(35, 221),
            Probability::new(42, 214),
            Probability::new(48, 208),
            Probability::new(55, 201),
            Probability::new(62, 194),
            Probability::new(69, 187),
            Probability::new(76, 180),
            Probability::new(83, 173),
            Probability::new(90, 166),
            Probability::new(97, 159),
            Probability::new(104, 152),
            Probability::new(111, 145),
            Probability::new(118, 138),
            Probability::new(125, 131),
            Probability::new(131, 125),
            Probability::new(138, 118),
            Probability::new(145, 111),
            Probability::new(152, 104),
            Probability::new(159, 97),
            Probability::new(166, 90),
            Probability::new(173, 83),
            Probability::new(180, 76),
            Probability::new(187, 69),
            Probability::new(194, 62),
            Probability::new(201, 55),
            Probability::new(208, 48),
            Probability::new(214, 42),
            Probability::new(221, 35),
            Probability::new(228, 28),
            Probability::new(235, 21),
            Probability::new(242, 14),
            Probability::new(250, 6),
            Probability::new(6, 250),
            Probability::new(14, 242),
            Probability::new(20, 236),
            Probability::new(27, 229),
            Probability::new(34, 222),
            Probability::new(40, 216),
            Probability::new(47, 209),
            Probability::new(54, 202),
            Probability::new(61, 195),
            Probability::new(67, 189),
            Probability::new(74, 182),
            Probability::new(81, 175),
            Probability::new(88, 168),
            Probability::new(94, 162),
            Probability::new(101, 155),
            Probability::new(108, 148),
            Probability::new(115, 141),
            Probability::new(121, 135),
            Probability::new(128, 128),
            Probability::new(135, 121),
            Probability::new(141, 115),
            Probability::new(148, 108),
            Probability::new(155, 101),
            Probability::new(162, 94),
            Probability::new(168, 88),
            Probability::new(175, 81),
            Probability::new(182, 74),
            Probability::new(189, 67),
            Probability::new(195, 61),
            Probability::new(202, 54),
            Probability::new(209, 47),
            Probability::new(216, 40),
            Probability::new(222, 34),
            Probability::new(229, 27),
            Probability::new(236, 20),
            Probability::new(242, 14),
            Probability::new(250, 6),
            Probability::new(6, 250),
            Probability::new(13, 243),
            Probability::new(20, 236),
            Probability::new(26, 230),
            Probability::new(33, 223),
            Probability::new(39, 217),
            Probability::new(46, 210),
            Probability::new(53, 203),
            Probability::new(59, 197),
            Probability::new(66, 190),
            Probability::new(72, 184),
            Probability::new(79, 177),
            Probability::new(85, 171),
            Probability::new(92, 164),
            Probability::new(98, 158),
            Probability::new(105, 151),
            Probability::new(112, 144),
            Probability::new(118, 138),
            Probability::new(125, 131),
            Probability::new(131, 125),
            Probability::new(138, 118),
            Probability::new(144, 112),
            Probability::new(151, 105),
            Probability::new(158, 98),
            Probability::new(164, 92),
            Probability::new(171, 85),
            Probability::new(177, 79),
            Probability::new(184, 72),
            Probability::new(190, 66),
            Probability::new(197, 59),
            Probability::new(203, 53),
            Probability::new(210, 46),
            Probability::new(217, 39),
            Probability::new(223, 33),
            Probability::new(230, 26),
            Probability::new(236, 20),
            Probability::new(243, 13),
            Probability::new(250, 6),
            Probability::new(6, 250),
            Probability::new(13, 243),
            Probability::new(19, 237),
            Probability::new(26, 230),
            Probability::new(32, 224),
            Probability::new(38, 218),
            Probability::new(45, 211),
            Probability::new(51, 205),
            Probability::new(58, 198),
            Probability::new(64, 192),
            Probability::new(70, 186),
            Probability::new(77, 179),
            Probability::new(83, 173),
            Probability::new(90, 166),
            Probability::new(96, 160),
            Probability::new(102, 154),
            Probability::new(109, 147),
            Probability::new(115, 141),
            Probability::new(122, 134),
            Probability::new(128, 128),
            Probability::new(134, 122),
            Probability::new(141, 115),
            Probability::new(147, 109),
            Probability::new(154, 102),
            Probability::new(160, 96),
            Probability::new(166, 90),
            Probability::new(173, 83),
            Probability::new(179, 77),
            Probability::new(186, 70),
            Probability::new(192, 64),
            Probability::new(198, 58),
            Probability::new(205, 51),
            Probability::new(211, 45),
            Probability::new(218, 38),
            Probability::new(224, 32),
            Probability::new(230, 26),
            Probability::new(237, 19),
            Probability::new(243, 13),
            Probability::new(250, 6),
            Probability::new(6, 250),
            Probability::new(13, 243),
            Probability::new(19, 237),
            Probability::new(25, 231),
            Probability::new(31, 225),
            Probability::new(37, 219),
            Probability::new(44, 212),
            Probability::new(50, 206),
            Probability::new(56, 200),
            Probability::new(62, 194),
            Probability::new(69, 187),
            Probability::new(75, 181),
            Probability::new(81, 175),
            Probability::new(87, 169),
            Probability::new(94, 162),
            Probability::new(100, 156),
            Probability::new(106, 150),
            Probability::new(112, 144),
            Probability::new(119, 137),
            Probability::new(125, 131),
            Probability::new(131, 125),
            Probability::new(137, 119),
            Probability::new(144, 112),
            Probability::new(150, 106),
            Probability::new(156, 100),
            Probability::new(162, 94),
            Probability::new(169, 87),
            Probability::new(175, 81),
            Probability::new(181, 75),
            Probability::new(187, 69),
            Probability::new(194, 62),
            Probability::new(200, 56),
            Probability::new(206, 50),
            Probability::new(212, 44),
            Probability::new(219, 37),
            Probability::new(225, 31),
            Probability::new(231, 25),
            Probability::new(237, 19),
            Probability::new(243, 13),
            Probability::new(250, 6),
            Probability::new(6, 250),
            Probability::new(12, 244),
            Probability::new(18, 238),
            Probability::new(24, 232),
            Probability::new(30, 226),
            Probability::new(37, 219),
            Probability::new(43, 213),
            Probability::new(49, 207),
            Probability::new(55, 201),
            Probability::new(61, 195),
            Probability::new(67, 189),
            Probability::new(73, 183),
            Probability::new(79, 177),
            Probability::new(85, 171),
            Probability::new(91, 165),
            Probability::new(98, 158),
            Probability::new(104, 152),
            Probability::new(110, 146),
            Probability::new(116, 140),
            Probability::new(122, 134),
            Probability::new(128, 128),
            Probability::new(134, 122),
            Probability::new(140, 116),
            Probability::new(146, 110),
            Probability::new(152, 104),
            Probability::new(158, 98),
            Probability::new(165, 91),
            Probability::new(171, 85),
            Probability::new(177, 79),
            Probability::new(183, 73),
            Probability::new(189, 67),
            Probability::new(195, 61),
            Probability::new(201, 55),
            Probability::new(207, 49),
            Probability::new(213, 43),
            Probability::new(219, 37),
            Probability::new(226, 30),
            Probability::new(232, 24),
            Probability::new(238, 18),
            Probability::new(244, 12),
            Probability::new(250, 6),
            Probability::new(5, 251),
            Probability::new(12, 244),
            Probability::new(18, 238),
            Probability::new(24, 232),
            Probability::new(30, 226),
            Probability::new(36, 220),
            Probability::new(42, 214),
            Probability::new(48, 208),
            Probability::new(54, 202),
            Probability::new(60, 196),
            Probability::new(65, 191),
            Probability::new(71, 185),
            Probability::new(77, 179),
            Probability::new(83, 173),
            Probability::new(89, 167),
            Probability::new(95, 161),
            Probability::new(101, 155),
            Probability::new(107, 149),
            Probability::new(113, 143),
            Probability::new(119, 137),
            Probability::new(125, 131),
            Probability::new(131, 125),
            Probability::new(137, 119),
            Probability::new(143, 113),
            Probability::new(149, 107),
            Probability::new(155, 101),
            Probability::new(161, 95),
            Probability::new(167, 89),
            Probability::new(173, 83),
            Probability::new(179, 77),
            Probability::new(185, 71),
            Probability::new(191, 65),
            Probability::new(196, 60),
            Probability::new(202, 54),
            Probability::new(208, 48),
            Probability::new(214, 42),
            Probability::new(220, 36),
            Probability::new(226, 30),
            Probability::new(232, 24),
            Probability::new(238, 18),
            Probability::new(244, 12),
            Probability::new(251, 5),
            Probability::new(5, 251),
            Probability::new(12, 244),
            Probability::new(17, 239),
            Probability::new(23, 233),
            Probability::new(29, 227),
            Probability::new(35, 221),
            Probability::new(41, 215),
            Probability::new(47, 209),
            Probability::new(52, 204),
            Probability::new(58, 198),
            Probability::new(64, 192),
            Probability::new(70, 186),
            Probability::new(76, 180),
            Probability::new(81, 175),
            Probability::new(87, 169),
            Probability::new(93, 163),
            Probability::new(99, 157),
            Probability::new(105, 151),
            Probability::new(111, 145),
            Probability::new(116, 140),
            Probability::new(122, 134),
            Probability::new(128, 128),
            Probability::new(134, 122),
            Probability::new(140, 116),
            Probability::new(145, 111),
            Probability::new(151, 105),
            Probability::new(157, 99),
            Probability::new(163, 93),
            Probability::new(169, 87),
            Probability::new(175, 81),
            Probability::new(180, 76),
            Probability::new(186, 70),
            Probability::new(192, 64),
            Probability::new(198, 58),
            Probability::new(204, 52),
            Probability::new(209, 47),
            Probability::new(215, 41),
            Probability::new(221, 35),
            Probability::new(227, 29),
            Probability::new(233, 23),
            Probability::new(239, 17),
            Probability::new(244, 12),
            Probability::new(251, 5),
            Probability::new(5, 251),
            Probability::new(11, 245),
            Probability::new(17, 239),
            Probability::new(23, 233),
            Probability::new(28, 228),
            Probability::new(34, 222),
            Probability::new(40, 216),
            Probability::new(46, 210),
            Probability::new(51, 205),
            Probability::new(57, 199),
            Probability::new(63, 193),
            Probability::new(68, 188),
            Probability::new(74, 182),
            Probability::new(80, 176),
            Probability::new(85, 171),
            Probability::new(91, 165),
            Probability::new(97, 159),
            Probability::new(102, 154),
            Probability::new(108, 148),
            Probability::new(114, 142),
            Probability::new(119, 137),
            Probability::new(125, 131),
            Probability::new(131, 125),
            Probability::new(137, 119),
            Probability::new(142, 114),
            Probability::new(148, 108),
            Probability::new(154, 102),
            Probability::new(159, 97),
            Probability::new(165, 91),
            Probability::new(171, 85),
            Probability::new(176, 80),
            Probability::new(182, 74),
            Probability::new(188, 68),
            Probability::new(193, 63),
            Probability::new(199, 57),
            Probability::new(205, 51),
            Probability::new(210, 46),
            Probability::new(216, 40),
            Probability::new(222, 34),
            Probability::new(228, 28),
            Probability::new(233, 23),
            Probability::new(239, 17),
            Probability::new(245, 11),
            Probability::new(251, 5),
            Probability::new(5, 251),
            Probability::new(11, 245),
            Probability::new(17, 239),
            Probability::new(22, 234),
            Probability::new(28, 228),
            Probability::new(33, 223),
            Probability::new(39, 217),
            Probability::new(45, 211),
            Probability::new(50, 206),
            Probability::new(56, 200),
            Probability::new(61, 195),
            Probability::new(67, 189),
            Probability::new(72, 184),
            Probability::new(78, 178),
            Probability::new(83, 173),
            Probability::new(89, 167),
            Probability::new(95, 161),
            Probability::new(100, 156),
            Probability::new(106, 150),
            Probability::new(111, 145),
            Probability::new(117, 139),
            Probability::new(122, 134),
            Probability::new(128, 128),
            Probability::new(134, 122),
            Probability::new(139, 117),
            Probability::new(145, 111),
            Probability::new(150, 106),
            Probability::new(156, 100),
            Probability::new(161, 95),
            Probability::new(167, 89),
            Probability::new(173, 83),
            Probability::new(178, 78),
            Probability::new(184, 72),
            Probability::new(189, 67),
            Probability::new(195, 61),
            Probability::new(200, 56),
            Probability::new(206, 50),
            Probability::new(211, 45),
            Probability::new(217, 39),
            Probability::new(223, 33),
            Probability::new(228, 28),
            Probability::new(234, 22),
            Probability::new(239, 17),
            Probability::new(245, 11),
            Probability::new(251, 5),
            Probability::new(5, 251),
            Probability::new(11, 245),
            Probability::new(16, 240),
            Probability::new(22, 234),
            Probability::new(27, 229),
            Probability::new(33, 223),
            Probability::new(38, 218),
            Probability::new(44, 212),
            Probability::new(49, 207),
            Probability::new(54, 202),
            Probability::new(60, 196),
            Probability::new(65, 191),
            Probability::new(71, 185),
            Probability::new(76, 180),
            Probability::new(82, 174),
            Probability::new(87, 169),
            Probability::new(93, 163),
            Probability::new(98, 158),
            Probability::new(103, 153),
            Probability::new(109, 147),
            Probability::new(114, 142),
            Probability::new(120, 136),
            Probability::new(125, 131),
            Probability::new(131, 125),
            Probability::new(136, 120),
            Probability::new(142, 114),
            Probability::new(147, 109),
            Probability::new(153, 103),
            Probability::new(158, 98),
            Probability::new(163, 93),
            Probability::new(169, 87),
            Probability::new(174, 82),
            Probability::new(180, 76),
            Probability::new(185, 71),
            Probability::new(191, 65),
            Probability::new(196, 60),
            Probability::new(202, 54),
            Probability::new(207, 49),
            Probability::new(212, 44),
            Probability::new(218, 38),
            Probability::new(223, 33),
            Probability::new(229, 27),
            Probability::new(234, 22),
            Probability::new(240, 16),
            Probability::new(245, 11),
            Probability::new(251, 5),
            Probability::new(5, 251),
            Probability::new(11, 245),
            Probability::new(16, 240),
            Probability::new(21, 235),
            Probability::new(27, 229),
            Probability::new(32, 224),
            Probability::new(37, 219),
            Probability::new(43, 213),
            Probability::new(48, 208),
            Probability::new(53, 203),
            Probability::new(59, 197),
            Probability::new(64, 192),
            Probability::new(69, 187),
            Probability::new(75, 181),
            Probability::new(80, 176),
            Probability::new(85, 171),
            Probability::new(91, 165),
            Probability::new(96, 160),
            Probability::new(101, 155),
            Probability::new(107, 149),
            Probability::new(112, 144),
            Probability::new(117, 139),
            Probability::new(123, 133),
            Probability::new(128, 128),
            Probability::new(133, 123),
            Probability::new(139, 117),
            Probability::new(144, 112),
            Probability::new(149, 107),
            Probability::new(155, 101),
            Probability::new(160, 96),
            Probability::new(165, 91),
            Probability::new(171, 85),
            Probability::new(176, 80),
            Probability::new(181, 75),
            Probability::new(187, 69),
            Probability::new(192, 64),
            Probability::new(197, 59),
            Probability::new(203, 53),
            Probability::new(208, 48),
            Probability::new(213, 43),
            Probability::new(219, 37),
            Probability::new(224, 32),
            Probability::new(229, 27),
            Probability::new(235, 21),
            Probability::new(240, 16),
            Probability::new(245, 11),
            Probability::new(251, 5),
            Probability::new(5, 251),
            Probability::new(10, 246),
            Probability::new(16, 240),
            Probability::new(21, 235),
            Probability::new(26, 230),
            Probability::new(31, 225),
            Probability::new(37, 219),
            Probability::new(42, 214),
            Probability::new(47, 209),
            Probability::new(52, 204),
            Probability::new(57, 199),
            Probability::new(63, 193),
            Probability::new(68, 188),
            Probability::new(73, 183),
            Probability::new(78, 178),
            Probability::new(84, 172),
            Probability::new(89, 167),
            Probability::new(94, 162),
            Probability::new(99, 157),
            Probability::new(104, 152),
            Probability::new(110, 146),
            Probability::new(115, 141),
            Probability::new(120, 136),
            Probability::new(125, 131),
            Probability::new(131, 125),
            Probability::new(136, 120),
            Probability::new(141, 115),
            Probability::new(146, 110),
            Probability::new(152, 104),
            Probability::new(157, 99),
            Probability::new(162, 94),
            Probability::new(167, 89),
            Probability::new(172, 84),
            Probability::new(178, 78),
            Probability::new(183, 73),
            Probability::new(188, 68),
            Probability::new(193, 63),
            Probability::new(199, 57),
            Probability::new(204, 52),
            Probability::new(209, 47),
            Probability::new(214, 42),
            Probability::new(219, 37),
            Probability::new(225, 31),
            Probability::new(230, 26),
            Probability::new(235, 21),
            Probability::new(240, 16),
            Probability::new(246, 10),
            Probability::new(251, 5),
            Probability::new(5, 251),
            Probability::new(10, 246),
            Probability::new(15, 241),
            Probability::new(20, 236),
            Probability::new(26, 230),
            Probability::new(31, 225),
            Probability::new(36, 220),
            Probability::new(41, 215),
            Probability::new(46, 210),
            Probability::new(51, 205),
            Probability::new(56, 200),
            Probability::new(61, 195),
            Probability::new(67, 189),
            Probability::new(72, 184),
            Probability::new(77, 179),
            Probability::new(82, 174),
            Probability::new(87, 169),
            Probability::new(92, 164),
            Probability::new(97, 159),
            Probability::new(102, 154),
            Probability::new(108, 148),
            Probability::new(113, 143),
            Probability::new(118, 138),
            Probability::new(123, 133),
            Probability::new(128, 128),
            Probability::new(133, 123),
            Probability::new(138, 118),
            Probability::new(143, 113),
            Probability::new(148, 108),
            Probability::new(154, 102),
            Probability::new(159, 97),
            Probability::new(164, 92),
            Probability::new(169, 87),
            Probability::new(174, 82),
            Probability::new(179, 77),
            Probability::new(184, 72),
            Probability::new(189, 67),
            Probability::new(195, 61),
            Probability::new(200, 56),
            Probability::new(205, 51),
            Probability::new(210, 46),
            Probability::new(215, 41),
            Probability::new(220, 36),
            Probability::new(225, 31),
            Probability::new(230, 26),
            Probability::new(236, 20),
            Probability::new(241, 15),
            Probability::new(246, 10),
            Probability::new(251, 5),
            Probability::new(5, 251),
            Probability::new(10, 246),
            Probability::new(15, 241),
            Probability::new(20, 236),
            Probability::new(25, 231),
            Probability::new(30, 226),
            Probability::new(35, 221),
            Probability::new(40, 216),
            Probability::new(45, 211),
            Probability::new(50, 206),
            Probability::new(55, 201),
            Probability::new(60, 196),
            Probability::new(65, 191),
            Probability::new(70, 186),
            Probability::new(75, 181),
            Probability::new(80, 176),
            Probability::new(85, 171),
            Probability::new(90, 166),
            Probability::new(95, 161),
            Probability::new(100, 156),
            Probability::new(105, 151),
            Probability::new(110, 146),
            Probability::new(115, 141),
            Probability::new(120, 136),
            Probability::new(125, 131),
            Probability::new(131, 125),
            Probability::new(136, 120),
            Probability::new(141, 115),
            Probability::new(146, 110),
            Probability::new(151, 105),
            Probability::new(156, 100),
            Probability::new(161, 95),
            Probability::new(166, 90),
            Probability::new(171, 85),
            Probability::new(176, 80),
            Probability::new(181, 75),
            Probability::new(186, 70),
            Probability::new(191, 65),
            Probability::new(196, 60),
            Probability::new(201, 55),
            Probability::new(206, 50),
            Probability::new(211, 45),
            Probability::new(216, 40),
            Probability::new(221, 35),
            Probability::new(226, 30),
            Probability::new(231, 25),
            Probability::new(236, 20),
            Probability::new(241, 15),
            Probability::new(246, 10),
            Probability::new(251, 5),
            Probability::new(4, 252),
            Probability::new(10, 246),
            Probability::new(15, 241),
            Probability::new(20, 236),
            Probability::new(25, 231),
            Probability::new(30, 226),
            Probability::new(34, 222),
            Probability::new(39, 217),
            Probability::new(44, 212),
            Probability::new(49, 207),
            Probability::new(54, 202),
            Probability::new(59, 197),
            Probability::new(64, 192),
            Probability::new(69, 187),
            Probability::new(74, 182),
            Probability::new(79, 177),
            Probability::new(84, 172),
            Probability::new(89, 167),
            Probability::new(94, 162),
            Probability::new(98, 158),
            Probability::new(103, 153),
            Probability::new(108, 148),
            Probability::new(113, 143),
            Probability::new(118, 138),
            Probability::new(123, 133),
            Probability::new(128, 128),
            Probability::new(133, 123),
            Probability::new(138, 118),
            Probability::new(143, 113),
            Probability::new(148, 108),
            Probability::new(153, 103),
            Probability::new(158, 98),
            Probability::new(162, 94),
            Probability::new(167, 89),
            Probability::new(172, 84),
            Probability::new(177, 79),
            Probability::new(182, 74),
            Probability::new(187, 69),
            Probability::new(192, 64),
            Probability::new(197, 59),
            Probability::new(202, 54),
            Probability::new(207, 49),
            Probability::new(212, 44),
            Probability::new(217, 39),
            Probability::new(222, 34),
            Probability::new(226, 30),
            Probability::new(231, 25),
            Probability::new(236, 20),
            Probability::new(241, 15),
            Probability::new(246, 10),
            Probability::new(252, 4),
            Probability::new(4, 252),
            Probability::new(10, 246),
            Probability::new(14, 242),
            Probability::new(19, 237),
            Probability::new(24, 232),
            Probability::new(29, 227),
            Probability::new(34, 222),
            Probability::new(39, 217),
            Probability::new(43, 213),
            Probability::new(48, 208),
            Probability::new(53, 203),
            Probability::new(58, 198),
            Probability::new(63, 193),
            Probability::new(68, 188),
            Probability::new(72, 184),
            Probability::new(77, 179),
            Probability::new(82, 174),
            Probability::new(87, 169),
            Probability::new(92, 164),
            Probability::new(97, 159),
            Probability::new(101, 155),
            Probability::new(106, 150),
            Probability::new(111, 145),
            Probability::new(116, 140),
            Probability::new(121, 135),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(135, 121),
            Probability::new(140, 116),
            Probability::new(145, 111),
            Probability::new(150, 106),
            Probability::new(155, 101),
            Probability::new(159, 97),
            Probability::new(164, 92),
            Probability::new(169, 87),
            Probability::new(174, 82),
            Probability::new(179, 77),
            Probability::new(184, 72),
            Probability::new(188, 68),
            Probability::new(193, 63),
            Probability::new(198, 58),
            Probability::new(203, 53),
            Probability::new(208, 48),
            Probability::new(213, 43),
            Probability::new(217, 39),
            Probability::new(222, 34),
            Probability::new(227, 29),
            Probability::new(232, 24),
            Probability::new(237, 19),
            Probability::new(242, 14),
            Probability::new(246, 10),
            Probability::new(252, 4),
            Probability::new(4, 252),
            Probability::new(10, 246),
            Probability::new(14, 242),
            Probability::new(19, 237),
            Probability::new(24, 232),
            Probability::new(28, 228),
            Probability::new(33, 223),
            Probability::new(38, 218),
            Probability::new(43, 213),
            Probability::new(47, 209),
            Probability::new(52, 204),
            Probability::new(57, 199),
            Probability::new(62, 194),
            Probability::new(66, 190),
            Probability::new(71, 185),
            Probability::new(76, 180),
            Probability::new(81, 175),
            Probability::new(85, 171),
            Probability::new(90, 166),
            Probability::new(95, 161),
            Probability::new(100, 156),
            Probability::new(104, 152),
            Probability::new(109, 147),
            Probability::new(114, 142),
            Probability::new(119, 137),
            Probability::new(123, 133),
            Probability::new(128, 128),
            Probability::new(133, 123),
            Probability::new(137, 119),
            Probability::new(142, 114),
            Probability::new(147, 109),
            Probability::new(152, 104),
            Probability::new(156, 100),
            Probability::new(161, 95),
            Probability::new(166, 90),
            Probability::new(171, 85),
            Probability::new(175, 81),
            Probability::new(180, 76),
            Probability::new(185, 71),
            Probability::new(190, 66),
            Probability::new(194, 62),
            Probability::new(199, 57),
            Probability::new(204, 52),
            Probability::new(209, 47),
            Probability::new(213, 43),
            Probability::new(218, 38),
            Probability::new(223, 33),
            Probability::new(228, 28),
            Probability::new(232, 24),
            Probability::new(237, 19),
            Probability::new(242, 14),
            Probability::new(246, 10),
            Probability::new(252, 4),
            Probability::new(4, 252),
            Probability::new(9, 247),
            Probability::new(14, 242),
            Probability::new(19, 237),
            Probability::new(23, 233),
            Probability::new(28, 228),
            Probability::new(33, 223),
            Probability::new(37, 219),
            Probability::new(42, 214),
            Probability::new(47, 209),
            Probability::new(51, 205),
            Probability::new(56, 200),
            Probability::new(61, 195),
            Probability::new(65, 191),
            Probability::new(70, 186),
            Probability::new(74, 182),
            Probability::new(79, 177),
            Probability::new(84, 172),
            Probability::new(88, 168),
            Probability::new(93, 163),
            Probability::new(98, 158),
            Probability::new(102, 154),
            Probability::new(107, 149),
            Probability::new(112, 144),
            Probability::new(116, 140),
            Probability::new(121, 135),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(135, 121),
            Probability::new(140, 116),
            Probability::new(144, 112),
            Probability::new(149, 107),
            Probability::new(154, 102),
            Probability::new(158, 98),
            Probability::new(163, 93),
            Probability::new(168, 88),
            Probability::new(172, 84),
            Probability::new(177, 79),
            Probability::new(182, 74),
            Probability::new(186, 70),
            Probability::new(191, 65),
            Probability::new(195, 61),
            Probability::new(200, 56),
            Probability::new(205, 51),
            Probability::new(209, 47),
            Probability::new(214, 42),
            Probability::new(219, 37),
            Probability::new(223, 33),
            Probability::new(228, 28),
            Probability::new(233, 23),
            Probability::new(237, 19),
            Probability::new(242, 14),
            Probability::new(247, 9),
            Probability::new(252, 4),
            Probability::new(4, 252),
            Probability::new(9, 247),
            Probability::new(14, 242),
            Probability::new(18, 238),
            Probability::new(23, 233),
            Probability::new(27, 229),
            Probability::new(32, 224),
            Probability::new(37, 219),
            Probability::new(41, 215),
            Probability::new(46, 210),
            Probability::new(50, 206),
            Probability::new(55, 201),
            Probability::new(59, 197),
            Probability::new(64, 192),
            Probability::new(69, 187),
            Probability::new(73, 183),
            Probability::new(78, 178),
            Probability::new(82, 174),
            Probability::new(87, 169),
            Probability::new(91, 165),
            Probability::new(96, 160),
            Probability::new(101, 155),
            Probability::new(105, 151),
            Probability::new(110, 146),
            Probability::new(114, 142),
            Probability::new(119, 137),
            Probability::new(123, 133),
            Probability::new(128, 128),
            Probability::new(133, 123),
            Probability::new(137, 119),
            Probability::new(142, 114),
            Probability::new(146, 110),
            Probability::new(151, 105),
            Probability::new(155, 101),
            Probability::new(160, 96),
            Probability::new(165, 91),
            Probability::new(169, 87),
            Probability::new(174, 82),
            Probability::new(178, 78),
            Probability::new(183, 73),
            Probability::new(187, 69),
            Probability::new(192, 64),
            Probability::new(197, 59),
            Probability::new(201, 55),
            Probability::new(206, 50),
            Probability::new(210, 46),
            Probability::new(215, 41),
            Probability::new(219, 37),
            Probability::new(224, 32),
            Probability::new(229, 27),
            Probability::new(233, 23),
            Probability::new(238, 18),
            Probability::new(242, 14),
            Probability::new(247, 9),
            Probability::new(252, 4),
            Probability::new(4, 252),
            Probability::new(9, 247),
            Probability::new(13, 243),
            Probability::new(18, 238),
            Probability::new(22, 234),
            Probability::new(27, 229),
            Probability::new(31, 225),
            Probability::new(36, 220),
            Probability::new(40, 216),
            Probability::new(45, 211),
            Probability::new(49, 207),
            Probability::new(54, 202),
            Probability::new(58, 198),
            Probability::new(63, 193),
            Probability::new(67, 189),
            Probability::new(72, 184),
            Probability::new(76, 180),
            Probability::new(81, 175),
            Probability::new(85, 171),
            Probability::new(90, 166),
            Probability::new(94, 162),
            Probability::new(99, 157),
            Probability::new(103, 153),
            Probability::new(108, 148),
            Probability::new(112, 144),
            Probability::new(117, 139),
            Probability::new(121, 135),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(135, 121),
            Probability::new(139, 117),
            Probability::new(144, 112),
            Probability::new(148, 108),
            Probability::new(153, 103),
            Probability::new(157, 99),
            Probability::new(162, 94),
            Probability::new(166, 90),
            Probability::new(171, 85),
            Probability::new(175, 81),
            Probability::new(180, 76),
            Probability::new(184, 72),
            Probability::new(189, 67),
            Probability::new(193, 63),
            Probability::new(198, 58),
            Probability::new(202, 54),
            Probability::new(207, 49),
            Probability::new(211, 45),
            Probability::new(216, 40),
            Probability::new(220, 36),
            Probability::new(225, 31),
            Probability::new(229, 27),
            Probability::new(234, 22),
            Probability::new(238, 18),
            Probability::new(243, 13),
            Probability::new(247, 9),
            Probability::new(252, 4),
            Probability::new(4, 252),
            Probability::new(9, 247),
            Probability::new(13, 243),
            Probability::new(18, 238),
            Probability::new(22, 234),
            Probability::new(26, 230),
            Probability::new(31, 225),
            Probability::new(35, 221),
            Probability::new(40, 216),
            Probability::new(44, 212),
            Probability::new(49, 207),
            Probability::new(53, 203),
            Probability::new(57, 199),
            Probability::new(62, 194),
            Probability::new(66, 190),
            Probability::new(71, 185),
            Probability::new(75, 181),
            Probability::new(79, 177),
            Probability::new(84, 172),
            Probability::new(88, 168),
            Probability::new(93, 163),
            Probability::new(97, 159),
            Probability::new(102, 154),
            Probability::new(106, 150),
            Probability::new(110, 146),
            Probability::new(115, 141),
            Probability::new(119, 137),
            Probability::new(124, 132),
            Probability::new(128, 128),
            Probability::new(132, 124),
            Probability::new(137, 119),
            Probability::new(141, 115),
            Probability::new(146, 110),
            Probability::new(150, 106),
            Probability::new(154, 102),
            Probability::new(159, 97),
            Probability::new(163, 93),
            Probability::new(168, 88),
            Probability::new(172, 84),
            Probability::new(177, 79),
            Probability::new(181, 75),
            Probability::new(185, 71),
            Probability::new(190, 66),
            Probability::new(194, 62),
            Probability::new(199, 57),
            Probability::new(203, 53),
            Probability::new(207, 49),
            Probability::new(212, 44),
            Probability::new(216, 40),
            Probability::new(221, 35),
            Probability::new(225, 31),
            Probability::new(230, 26),
            Probability::new(234, 22),
            Probability::new(238, 18),
            Probability::new(243, 13),
            Probability::new(247, 9),
            Probability::new(252, 4),
            Probability::new(4, 252),
            Probability::new(9, 247),
            Probability::new(13, 243),
            Probability::new(17, 239),
            Probability::new(22, 234),
            Probability::new(26, 230),
            Probability::new(30, 226),
            Probability::new(35, 221),
            Probability::new(39, 217),
            Probability::new(43, 213),
            Probability::new(48, 208),
            Probability::new(52, 204),
            Probability::new(56, 200),
            Probability::new(61, 195),
            Probability::new(65, 191),
            Probability::new(69, 187),
            Probability::new(74, 182),
            Probability::new(78, 178),
            Probability::new(82, 174),
            Probability::new(87, 169),
            Probability::new(91, 165),
            Probability::new(95, 161),
            Probability::new(100, 156),
            Probability::new(104, 152),
            Probability::new(108, 148),
            Probability::new(113, 143),
            Probability::new(117, 139),
            Probability::new(121, 135),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(135, 121),
            Probability::new(139, 117),
            Probability::new(143, 113),
            Probability::new(148, 108),
            Probability::new(152, 104),
            Probability::new(156, 100),
            Probability::new(161, 95),
            Probability::new(165, 91),
            Probability::new(169, 87),
            Probability::new(174, 82),
            Probability::new(178, 78),
            Probability::new(182, 74),
            Probability::new(187, 69),
            Probability::new(191, 65),
            Probability::new(195, 61),
            Probability::new(200, 56),
            Probability::new(204, 52),
            Probability::new(208, 48),
            Probability::new(213, 43),
            Probability::new(217, 39),
            Probability::new(221, 35),
            Probability::new(226, 30),
            Probability::new(230, 26),
            Probability::new(234, 22),
            Probability::new(239, 17),
            Probability::new(243, 13),
            Probability::new(247, 9),
            Probability::new(252, 4),
            Probability::new(4, 252),
            Probability::new(9, 247),
            Probability::new(13, 243),
            Probability::new(17, 239),
            Probability::new(21, 235),
            Probability::new(26, 230),
            Probability::new(30, 226),
            Probability::new(34, 222),
            Probability::new(38, 218),
            Probability::new(43, 213),
            Probability::new(47, 209),
            Probability::new(51, 205),
            Probability::new(55, 201),
            Probability::new(60, 196),
            Probability::new(64, 192),
            Probability::new(68, 188),
            Probability::new(73, 183),
            Probability::new(77, 179),
            Probability::new(81, 175),
            Probability::new(85, 171),
            Probability::new(90, 166),
            Probability::new(94, 162),
            Probability::new(98, 158),
            Probability::new(102, 154),
            Probability::new(107, 149),
            Probability::new(111, 145),
            Probability::new(115, 141),
            Probability::new(119, 137),
            Probability::new(124, 132),
            Probability::new(128, 128),
            Probability::new(132, 124),
            Probability::new(137, 119),
            Probability::new(141, 115),
            Probability::new(145, 111),
            Probability::new(149, 107),
            Probability::new(154, 102),
            Probability::new(158, 98),
            Probability::new(162, 94),
            Probability::new(166, 90),
            Probability::new(171, 85),
            Probability::new(175, 81),
            Probability::new(179, 77),
            Probability::new(183, 73),
            Probability::new(188, 68),
            Probability::new(192, 64),
            Probability::new(196, 60),
            Probability::new(201, 55),
            Probability::new(205, 51),
            Probability::new(209, 47),
            Probability::new(213, 43),
            Probability::new(218, 38),
            Probability::new(222, 34),
            Probability::new(226, 30),
            Probability::new(230, 26),
            Probability::new(235, 21),
            Probability::new(239, 17),
            Probability::new(243, 13),
            Probability::new(247, 9),
            Probability::new(252, 4),
            Probability::new(4, 252),
            Probability::new(8, 248),
            Probability::new(13, 243),
            Probability::new(17, 239),
            Probability::new(21, 235),
            Probability::new(25, 231),
            Probability::new(29, 227),
            Probability::new(34, 222),
            Probability::new(38, 218),
            Probability::new(42, 214),
            Probability::new(46, 210),
            Probability::new(50, 206),
            Probability::new(55, 201),
            Probability::new(59, 197),
            Probability::new(63, 193),
            Probability::new(67, 189),
            Probability::new(71, 185),
            Probability::new(76, 180),
            Probability::new(80, 176),
            Probability::new(84, 172),
            Probability::new(88, 168),
            Probability::new(92, 164),
            Probability::new(97, 159),
            Probability::new(101, 155),
            Probability::new(105, 151),
            Probability::new(109, 147),
            Probability::new(113, 143),
            Probability::new(118, 138),
            Probability::new(122, 134),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(134, 122),
            Probability::new(138, 118),
            Probability::new(143, 113),
            Probability::new(147, 109),
            Probability::new(151, 105),
            Probability::new(155, 101),
            Probability::new(159, 97),
            Probability::new(164, 92),
            Probability::new(168, 88),
            Probability::new(172, 84),
            Probability::new(176, 80),
            Probability::new(180, 76),
            Probability::new(185, 71),
            Probability::new(189, 67),
            Probability::new(193, 63),
            Probability::new(197, 59),
            Probability::new(201, 55),
            Probability::new(206, 50),
            Probability::new(210, 46),
            Probability::new(214, 42),
            Probability::new(218, 38),
            Probability::new(222, 34),
            Probability::new(227, 29),
            Probability::new(231, 25),
            Probability::new(235, 21),
            Probability::new(239, 17),
            Probability::new(243, 13),
            Probability::new(248, 8),
            Probability::new(252, 4),
            Probability::new(4, 252),
            Probability::new(8, 248),
            Probability::new(12, 244),
            Probability::new(17, 239),
            Probability::new(21, 235),
            Probability::new(25, 231),
            Probability::new(29, 227),
            Probability::new(33, 223),
            Probability::new(37, 219),
            Probability::new(41, 215),
            Probability::new(45, 211),
            Probability::new(50, 206),
            Probability::new(54, 202),
            Probability::new(58, 198),
            Probability::new(62, 194),
            Probability::new(66, 190),
            Probability::new(70, 186),
            Probability::new(74, 182),
            Probability::new(78, 178),
            Probability::new(83, 173),
            Probability::new(87, 169),
            Probability::new(91, 165),
            Probability::new(95, 161),
            Probability::new(99, 157),
            Probability::new(103, 153),
            Probability::new(107, 149),
            Probability::new(111, 145),
            Probability::new(116, 140),
            Probability::new(120, 136),
            Probability::new(124, 132),
            Probability::new(128, 128),
            Probability::new(132, 124),
            Probability::new(136, 120),
            Probability::new(140, 116),
            Probability::new(145, 111),
            Probability::new(149, 107),
            Probability::new(153, 103),
            Probability::new(157, 99),
            Probability::new(161, 95),
            Probability::new(165, 91),
            Probability::new(169, 87),
            Probability::new(173, 83),
            Probability::new(178, 78),
            Probability::new(182, 74),
            Probability::new(186, 70),
            Probability::new(190, 66),
            Probability::new(194, 62),
            Probability::new(198, 58),
            Probability::new(202, 54),
            Probability::new(206, 50),
            Probability::new(211, 45),
            Probability::new(215, 41),
            Probability::new(219, 37),
            Probability::new(223, 33),
            Probability::new(227, 29),
            Probability::new(231, 25),
            Probability::new(235, 21),
            Probability::new(239, 17),
            Probability::new(244, 12),
            Probability::new(248, 8),
            Probability::new(252, 4),
            Probability::new(4, 252),
            Probability::new(8, 248),
            Probability::new(12, 244),
            Probability::new(16, 240),
            Probability::new(20, 236),
            Probability::new(24, 232),
            Probability::new(28, 228),
            Probability::new(33, 223),
            Probability::new(37, 219),
            Probability::new(41, 215),
            Probability::new(45, 211),
            Probability::new(49, 207),
            Probability::new(53, 203),
            Probability::new(57, 199),
            Probability::new(61, 195),
            Probability::new(65, 191),
            Probability::new(69, 187),
            Probability::new(73, 183),
            Probability::new(77, 179),
            Probability::new(81, 175),
            Probability::new(85, 171),
            Probability::new(89, 167),
            Probability::new(93, 163),
            Probability::new(98, 158),
            Probability::new(102, 154),
            Probability::new(106, 150),
            Probability::new(110, 146),
            Probability::new(114, 142),
            Probability::new(118, 138),
            Probability::new(122, 134),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(134, 122),
            Probability::new(138, 118),
            Probability::new(142, 114),
            Probability::new(146, 110),
            Probability::new(150, 106),
            Probability::new(154, 102),
            Probability::new(158, 98),
            Probability::new(163, 93),
            Probability::new(167, 89),
            Probability::new(171, 85),
            Probability::new(175, 81),
            Probability::new(179, 77),
            Probability::new(183, 73),
            Probability::new(187, 69),
            Probability::new(191, 65),
            Probability::new(195, 61),
            Probability::new(199, 57),
            Probability::new(203, 53),
            Probability::new(207, 49),
            Probability::new(211, 45),
            Probability::new(215, 41),
            Probability::new(219, 37),
            Probability::new(223, 33),
            Probability::new(228, 28),
            Probability::new(232, 24),
            Probability::new(236, 20),
            Probability::new(240, 16),
            Probability::new(244, 12),
            Probability::new(248, 8),
            Probability::new(252, 4),
            Probability::new(4, 252),
            Probability::new(8, 248),
            Probability::new(12, 244),
            Probability::new(16, 240),
            Probability::new(20, 236),
            Probability::new(24, 232),
            Probability::new(28, 228),
            Probability::new(32, 224),
            Probability::new(36, 220),
            Probability::new(40, 216),
            Probability::new(44, 212),
            Probability::new(48, 208),
            Probability::new(52, 204),
            Probability::new(56, 200),
            Probability::new(60, 196),
            Probability::new(64, 192),
            Probability::new(68, 188),
            Probability::new(72, 184),
            Probability::new(76, 180),
            Probability::new(80, 176),
            Probability::new(84, 172),
            Probability::new(88, 168),
            Probability::new(92, 164),
            Probability::new(96, 160),
            Probability::new(100, 156),
            Probability::new(104, 152),
            Probability::new(108, 148),
            Probability::new(112, 144),
            Probability::new(116, 140),
            Probability::new(120, 136),
            Probability::new(124, 132),
            Probability::new(128, 128),
            Probability::new(132, 124),
            Probability::new(136, 120),
            Probability::new(140, 116),
            Probability::new(144, 112),
            Probability::new(148, 108),
            Probability::new(152, 104),
            Probability::new(156, 100),
            Probability::new(160, 96),
            Probability::new(164, 92),
            Probability::new(168, 88),
            Probability::new(172, 84),
            Probability::new(176, 80),
            Probability::new(180, 76),
            Probability::new(184, 72),
            Probability::new(188, 68),
            Probability::new(192, 64),
            Probability::new(196, 60),
            Probability::new(200, 56),
            Probability::new(204, 52),
            Probability::new(208, 48),
            Probability::new(212, 44),
            Probability::new(216, 40),
            Probability::new(220, 36),
            Probability::new(224, 32),
            Probability::new(228, 28),
            Probability::new(232, 24),
            Probability::new(236, 20),
            Probability::new(240, 16),
            Probability::new(244, 12),
            Probability::new(248, 8),
            Probability::new(252, 4),
            Probability::new(3, 253),
            Probability::new(8, 248),
            Probability::new(12, 244),
            Probability::new(16, 240),
            Probability::new(20, 236),
            Probability::new(24, 232),
            Probability::new(28, 228),
            Probability::new(32, 224),
            Probability::new(35, 221),
            Probability::new(39, 217),
            Probability::new(43, 213),
            Probability::new(47, 209),
            Probability::new(51, 205),
            Probability::new(55, 201),
            Probability::new(59, 197),
            Probability::new(63, 193),
            Probability::new(67, 189),
            Probability::new(71, 185),
            Probability::new(75, 181),
            Probability::new(79, 177),
            Probability::new(83, 173),
            Probability::new(87, 169),
            Probability::new(91, 165),
            Probability::new(95, 161),
            Probability::new(98, 158),
            Probability::new(102, 154),
            Probability::new(106, 150),
            Probability::new(110, 146),
            Probability::new(114, 142),
            Probability::new(118, 138),
            Probability::new(122, 134),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(134, 122),
            Probability::new(138, 118),
            Probability::new(142, 114),
            Probability::new(146, 110),
            Probability::new(150, 106),
            Probability::new(154, 102),
            Probability::new(158, 98),
            Probability::new(161, 95),
            Probability::new(165, 91),
            Probability::new(169, 87),
            Probability::new(173, 83),
            Probability::new(177, 79),
            Probability::new(181, 75),
            Probability::new(185, 71),
            Probability::new(189, 67),
            Probability::new(193, 63),
            Probability::new(197, 59),
            Probability::new(201, 55),
            Probability::new(205, 51),
            Probability::new(209, 47),
            Probability::new(213, 43),
            Probability::new(217, 39),
            Probability::new(221, 35),
            Probability::new(224, 32),
            Probability::new(228, 28),
            Probability::new(232, 24),
            Probability::new(236, 20),
            Probability::new(240, 16),
            Probability::new(244, 12),
            Probability::new(248, 8),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(8, 248),
            Probability::new(12, 244),
            Probability::new(16, 240),
            Probability::new(19, 237),
            Probability::new(23, 233),
            Probability::new(27, 229),
            Probability::new(31, 225),
            Probability::new(35, 221),
            Probability::new(39, 217),
            Probability::new(43, 213),
            Probability::new(47, 209),
            Probability::new(50, 206),
            Probability::new(54, 202),
            Probability::new(58, 198),
            Probability::new(62, 194),
            Probability::new(66, 190),
            Probability::new(70, 186),
            Probability::new(74, 182),
            Probability::new(78, 178),
            Probability::new(81, 175),
            Probability::new(85, 171),
            Probability::new(89, 167),
            Probability::new(93, 163),
            Probability::new(97, 159),
            Probability::new(101, 155),
            Probability::new(105, 151),
            Probability::new(109, 147),
            Probability::new(112, 144),
            Probability::new(116, 140),
            Probability::new(120, 136),
            Probability::new(124, 132),
            Probability::new(128, 128),
            Probability::new(132, 124),
            Probability::new(136, 120),
            Probability::new(140, 116),
            Probability::new(144, 112),
            Probability::new(147, 109),
            Probability::new(151, 105),
            Probability::new(155, 101),
            Probability::new(159, 97),
            Probability::new(163, 93),
            Probability::new(167, 89),
            Probability::new(171, 85),
            Probability::new(175, 81),
            Probability::new(178, 78),
            Probability::new(182, 74),
            Probability::new(186, 70),
            Probability::new(190, 66),
            Probability::new(194, 62),
            Probability::new(198, 58),
            Probability::new(202, 54),
            Probability::new(206, 50),
            Probability::new(209, 47),
            Probability::new(213, 43),
            Probability::new(217, 39),
            Probability::new(221, 35),
            Probability::new(225, 31),
            Probability::new(229, 27),
            Probability::new(233, 23),
            Probability::new(237, 19),
            Probability::new(240, 16),
            Probability::new(244, 12),
            Probability::new(248, 8),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(8, 248),
            Probability::new(11, 245),
            Probability::new(15, 241),
            Probability::new(19, 237),
            Probability::new(23, 233),
            Probability::new(27, 229),
            Probability::new(31, 225),
            Probability::new(34, 222),
            Probability::new(38, 218),
            Probability::new(42, 214),
            Probability::new(46, 210),
            Probability::new(50, 206),
            Probability::new(53, 203),
            Probability::new(57, 199),
            Probability::new(61, 195),
            Probability::new(65, 191),
            Probability::new(69, 187),
            Probability::new(73, 183),
            Probability::new(76, 180),
            Probability::new(80, 176),
            Probability::new(84, 172),
            Probability::new(88, 168),
            Probability::new(92, 164),
            Probability::new(96, 160),
            Probability::new(99, 157),
            Probability::new(103, 153),
            Probability::new(107, 149),
            Probability::new(111, 145),
            Probability::new(115, 141),
            Probability::new(118, 138),
            Probability::new(122, 134),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(134, 122),
            Probability::new(138, 118),
            Probability::new(141, 115),
            Probability::new(145, 111),
            Probability::new(149, 107),
            Probability::new(153, 103),
            Probability::new(157, 99),
            Probability::new(160, 96),
            Probability::new(164, 92),
            Probability::new(168, 88),
            Probability::new(172, 84),
            Probability::new(176, 80),
            Probability::new(180, 76),
            Probability::new(183, 73),
            Probability::new(187, 69),
            Probability::new(191, 65),
            Probability::new(195, 61),
            Probability::new(199, 57),
            Probability::new(203, 53),
            Probability::new(206, 50),
            Probability::new(210, 46),
            Probability::new(214, 42),
            Probability::new(218, 38),
            Probability::new(222, 34),
            Probability::new(225, 31),
            Probability::new(229, 27),
            Probability::new(233, 23),
            Probability::new(237, 19),
            Probability::new(241, 15),
            Probability::new(245, 11),
            Probability::new(248, 8),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(8, 248),
            Probability::new(11, 245),
            Probability::new(15, 241),
            Probability::new(19, 237),
            Probability::new(23, 233),
            Probability::new(26, 230),
            Probability::new(30, 226),
            Probability::new(34, 222),
            Probability::new(38, 218),
            Probability::new(41, 215),
            Probability::new(45, 211),
            Probability::new(49, 207),
            Probability::new(53, 203),
            Probability::new(56, 200),
            Probability::new(60, 196),
            Probability::new(64, 192),
            Probability::new(68, 188),
            Probability::new(72, 184),
            Probability::new(75, 181),
            Probability::new(79, 177),
            Probability::new(83, 173),
            Probability::new(87, 169),
            Probability::new(90, 166),
            Probability::new(94, 162),
            Probability::new(98, 158),
            Probability::new(102, 154),
            Probability::new(105, 151),
            Probability::new(109, 147),
            Probability::new(113, 143),
            Probability::new(117, 139),
            Probability::new(120, 136),
            Probability::new(124, 132),
            Probability::new(128, 128),
            Probability::new(132, 124),
            Probability::new(136, 120),
            Probability::new(139, 117),
            Probability::new(143, 113),
            Probability::new(147, 109),
            Probability::new(151, 105),
            Probability::new(154, 102),
            Probability::new(158, 98),
            Probability::new(162, 94),
            Probability::new(166, 90),
            Probability::new(169, 87),
            Probability::new(173, 83),
            Probability::new(177, 79),
            Probability::new(181, 75),
            Probability::new(184, 72),
            Probability::new(188, 68),
            Probability::new(192, 64),
            Probability::new(196, 60),
            Probability::new(200, 56),
            Probability::new(203, 53),
            Probability::new(207, 49),
            Probability::new(211, 45),
            Probability::new(215, 41),
            Probability::new(218, 38),
            Probability::new(222, 34),
            Probability::new(226, 30),
            Probability::new(230, 26),
            Probability::new(233, 23),
            Probability::new(237, 19),
            Probability::new(241, 15),
            Probability::new(245, 11),
            Probability::new(248, 8),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(7, 249),
            Probability::new(11, 245),
            Probability::new(15, 241),
            Probability::new(19, 237),
            Probability::new(22, 234),
            Probability::new(26, 230),
            Probability::new(30, 226),
            Probability::new(33, 223),
            Probability::new(37, 219),
            Probability::new(41, 215),
            Probability::new(45, 211),
            Probability::new(48, 208),
            Probability::new(52, 204),
            Probability::new(56, 200),
            Probability::new(59, 197),
            Probability::new(63, 193),
            Probability::new(67, 189),
            Probability::new(70, 186),
            Probability::new(74, 182),
            Probability::new(78, 178),
            Probability::new(82, 174),
            Probability::new(85, 171),
            Probability::new(89, 167),
            Probability::new(93, 163),
            Probability::new(96, 160),
            Probability::new(100, 156),
            Probability::new(104, 152),
            Probability::new(108, 148),
            Probability::new(111, 145),
            Probability::new(115, 141),
            Probability::new(119, 137),
            Probability::new(122, 134),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(134, 122),
            Probability::new(137, 119),
            Probability::new(141, 115),
            Probability::new(145, 111),
            Probability::new(148, 108),
            Probability::new(152, 104),
            Probability::new(156, 100),
            Probability::new(160, 96),
            Probability::new(163, 93),
            Probability::new(167, 89),
            Probability::new(171, 85),
            Probability::new(174, 82),
            Probability::new(178, 78),
            Probability::new(182, 74),
            Probability::new(186, 70),
            Probability::new(189, 67),
            Probability::new(193, 63),
            Probability::new(197, 59),
            Probability::new(200, 56),
            Probability::new(204, 52),
            Probability::new(208, 48),
            Probability::new(211, 45),
            Probability::new(215, 41),
            Probability::new(219, 37),
            Probability::new(223, 33),
            Probability::new(226, 30),
            Probability::new(230, 26),
            Probability::new(234, 22),
            Probability::new(237, 19),
            Probability::new(241, 15),
            Probability::new(245, 11),
            Probability::new(249, 7),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(7, 249),
            Probability::new(11, 245),
            Probability::new(15, 241),
            Probability::new(18, 238),
            Probability::new(22, 234),
            Probability::new(26, 230),
            Probability::new(29, 227),
            Probability::new(33, 223),
            Probability::new(37, 219),
            Probability::new(40, 216),
            Probability::new(44, 212),
            Probability::new(48, 208),
            Probability::new(51, 205),
            Probability::new(55, 201),
            Probability::new(59, 197),
            Probability::new(62, 194),
            Probability::new(66, 190),
            Probability::new(69, 187),
            Probability::new(73, 183),
            Probability::new(77, 179),
            Probability::new(80, 176),
            Probability::new(84, 172),
            Probability::new(88, 168),
            Probability::new(91, 165),
            Probability::new(95, 161),
            Probability::new(99, 157),
            Probability::new(102, 154),
            Probability::new(106, 150),
            Probability::new(110, 146),
            Probability::new(113, 143),
            Probability::new(117, 139),
            Probability::new(121, 135),
            Probability::new(124, 132),
            Probability::new(128, 128),
            Probability::new(132, 124),
            Probability::new(135, 121),
            Probability::new(139, 117),
            Probability::new(143, 113),
            Probability::new(146, 110),
            Probability::new(150, 106),
            Probability::new(154, 102),
            Probability::new(157, 99),
            Probability::new(161, 95),
            Probability::new(165, 91),
            Probability::new(168, 88),
            Probability::new(172, 84),
            Probability::new(176, 80),
            Probability::new(179, 77),
            Probability::new(183, 73),
            Probability::new(187, 69),
            Probability::new(190, 66),
            Probability::new(194, 62),
            Probability::new(197, 59),
            Probability::new(201, 55),
            Probability::new(205, 51),
            Probability::new(208, 48),
            Probability::new(212, 44),
            Probability::new(216, 40),
            Probability::new(219, 37),
            Probability::new(223, 33),
            Probability::new(227, 29),
            Probability::new(230, 26),
            Probability::new(234, 22),
            Probability::new(238, 18),
            Probability::new(241, 15),
            Probability::new(245, 11),
            Probability::new(249, 7),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(7, 249),
            Probability::new(11, 245),
            Probability::new(14, 242),
            Probability::new(18, 238),
            Probability::new(22, 234),
            Probability::new(25, 231),
            Probability::new(29, 227),
            Probability::new(32, 224),
            Probability::new(36, 220),
            Probability::new(40, 216),
            Probability::new(43, 213),
            Probability::new(47, 209),
            Probability::new(50, 206),
            Probability::new(54, 202),
            Probability::new(58, 198),
            Probability::new(61, 195),
            Probability::new(65, 191),
            Probability::new(69, 187),
            Probability::new(72, 184),
            Probability::new(76, 180),
            Probability::new(79, 177),
            Probability::new(83, 173),
            Probability::new(87, 169),
            Probability::new(90, 166),
            Probability::new(94, 162),
            Probability::new(97, 159),
            Probability::new(101, 155),
            Probability::new(105, 151),
            Probability::new(108, 148),
            Probability::new(112, 144),
            Probability::new(115, 141),
            Probability::new(119, 137),
            Probability::new(123, 133),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(133, 123),
            Probability::new(137, 119),
            Probability::new(141, 115),
            Probability::new(144, 112),
            Probability::new(148, 108),
            Probability::new(151, 105),
            Probability::new(155, 101),
            Probability::new(159, 97),
            Probability::new(162, 94),
            Probability::new(166, 90),
            Probability::new(169, 87),
            Probability::new(173, 83),
            Probability::new(177, 79),
            Probability::new(180, 76),
            Probability::new(184, 72),
            Probability::new(187, 69),
            Probability::new(191, 65),
            Probability::new(195, 61),
            Probability::new(198, 58),
            Probability::new(202, 54),
            Probability::new(206, 50),
            Probability::new(209, 47),
            Probability::new(213, 43),
            Probability::new(216, 40),
            Probability::new(220, 36),
            Probability::new(224, 32),
            Probability::new(227, 29),
            Probability::new(231, 25),
            Probability::new(234, 22),
            Probability::new(238, 18),
            Probability::new(242, 14),
            Probability::new(245, 11),
            Probability::new(249, 7),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(7, 249),
            Probability::new(11, 245),
            Probability::new(14, 242),
            Probability::new(18, 238),
            Probability::new(21, 235),
            Probability::new(25, 231),
            Probability::new(28, 228),
            Probability::new(32, 224),
            Probability::new(36, 220),
            Probability::new(39, 217),
            Probability::new(43, 213),
            Probability::new(46, 210),
            Probability::new(50, 206),
            Probability::new(53, 203),
            Probability::new(57, 199),
            Probability::new(60, 196),
            Probability::new(64, 192),
            Probability::new(68, 188),
            Probability::new(71, 185),
            Probability::new(75, 181),
            Probability::new(78, 178),
            Probability::new(82, 174),
            Probability::new(85, 171),
            Probability::new(89, 167),
            Probability::new(92, 164),
            Probability::new(96, 160),
            Probability::new(100, 156),
            Probability::new(103, 153),
            Probability::new(107, 149),
            Probability::new(110, 146),
            Probability::new(114, 142),
            Probability::new(117, 139),
            Probability::new(121, 135),
            Probability::new(124, 132),
            Probability::new(128, 128),
            Probability::new(132, 124),
            Probability::new(135, 121),
            Probability::new(139, 117),
            Probability::new(142, 114),
            Probability::new(146, 110),
            Probability::new(149, 107),
            Probability::new(153, 103),
            Probability::new(156, 100),
            Probability::new(160, 96),
            Probability::new(164, 92),
            Probability::new(167, 89),
            Probability::new(171, 85),
            Probability::new(174, 82),
            Probability::new(178, 78),
            Probability::new(181, 75),
            Probability::new(185, 71),
            Probability::new(188, 68),
            Probability::new(192, 64),
            Probability::new(196, 60),
            Probability::new(199, 57),
            Probability::new(203, 53),
            Probability::new(206, 50),
            Probability::new(210, 46),
            Probability::new(213, 43),
            Probability::new(217, 39),
            Probability::new(220, 36),
            Probability::new(224, 32),
            Probability::new(228, 28),
            Probability::new(231, 25),
            Probability::new(235, 21),
            Probability::new(238, 18),
            Probability::new(242, 14),
            Probability::new(245, 11),
            Probability::new(249, 7),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(7, 249),
            Probability::new(11, 245),
            Probability::new(14, 242),
            Probability::new(18, 238),
            Probability::new(21, 235),
            Probability::new(25, 231),
            Probability::new(28, 228),
            Probability::new(32, 224),
            Probability::new(35, 221),
            Probability::new(39, 217),
            Probability::new(42, 214),
            Probability::new(46, 210),
            Probability::new(49, 207),
            Probability::new(53, 203),
            Probability::new(56, 200),
            Probability::new(60, 196),
            Probability::new(63, 193),
            Probability::new(67, 189),
            Probability::new(70, 186),
            Probability::new(74, 182),
            Probability::new(77, 179),
            Probability::new(81, 175),
            Probability::new(84, 172),
            Probability::new(88, 168),
            Probability::new(91, 165),
            Probability::new(95, 161),
            Probability::new(98, 158),
            Probability::new(102, 154),
            Probability::new(105, 151),
            Probability::new(109, 147),
            Probability::new(112, 144),
            Probability::new(116, 140),
            Probability::new(119, 137),
            Probability::new(123, 133),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(133, 123),
            Probability::new(137, 119),
            Probability::new(140, 116),
            Probability::new(144, 112),
            Probability::new(147, 109),
            Probability::new(151, 105),
            Probability::new(154, 102),
            Probability::new(158, 98),
            Probability::new(161, 95),
            Probability::new(165, 91),
            Probability::new(168, 88),
            Probability::new(172, 84),
            Probability::new(175, 81),
            Probability::new(179, 77),
            Probability::new(182, 74),
            Probability::new(186, 70),
            Probability::new(189, 67),
            Probability::new(193, 63),
            Probability::new(196, 60),
            Probability::new(200, 56),
            Probability::new(203, 53),
            Probability::new(207, 49),
            Probability::new(210, 46),
            Probability::new(214, 42),
            Probability::new(217, 39),
            Probability::new(221, 35),
            Probability::new(224, 32),
            Probability::new(228, 28),
            Probability::new(231, 25),
            Probability::new(235, 21),
            Probability::new(238, 18),
            Probability::new(242, 14),
            Probability::new(245, 11),
            Probability::new(249, 7),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(7, 249),
            Probability::new(10, 246),
            Probability::new(14, 242),
            Probability::new(17, 239),
            Probability::new(21, 235),
            Probability::new(24, 232),
            Probability::new(28, 228),
            Probability::new(31, 225),
            Probability::new(35, 221),
            Probability::new(38, 218),
            Probability::new(42, 214),
            Probability::new(45, 211),
            Probability::new(48, 208),
            Probability::new(52, 204),
            Probability::new(55, 201),
            Probability::new(59, 197),
            Probability::new(62, 194),
            Probability::new(66, 190),
            Probability::new(69, 187),
            Probability::new(73, 183),
            Probability::new(76, 180),
            Probability::new(80, 176),
            Probability::new(83, 173),
            Probability::new(86, 170),
            Probability::new(90, 166),
            Probability::new(93, 163),
            Probability::new(97, 159),
            Probability::new(100, 156),
            Probability::new(104, 152),
            Probability::new(107, 149),
            Probability::new(111, 145),
            Probability::new(114, 142),
            Probability::new(118, 138),
            Probability::new(121, 135),
            Probability::new(125, 131),
            Probability::new(128, 128),
            Probability::new(131, 125),
            Probability::new(135, 121),
            Probability::new(138, 118),
            Probability::new(142, 114),
            Probability::new(145, 111),
            Probability::new(149, 107),
            Probability::new(152, 104),
            Probability::new(156, 100),
            Probability::new(159, 97),
            Probability::new(163, 93),
            Probability::new(166, 90),
            Probability::new(170, 86),
            Probability::new(173, 83),
            Probability::new(176, 80),
            Probability::new(180, 76),
            Probability::new(183, 73),
            Probability::new(187, 69),
            Probability::new(190, 66),
            Probability::new(194, 62),
            Probability::new(197, 59),
            Probability::new(201, 55),
            Probability::new(204, 52),
            Probability::new(208, 48),
            Probability::new(211, 45),
            Probability::new(214, 42),
            Probability::new(218, 38),
            Probability::new(221, 35),
            Probability::new(225, 31),
            Probability::new(228, 28),
            Probability::new(232, 24),
            Probability::new(235, 21),
            Probability::new(239, 17),
            Probability::new(242, 14),
            Probability::new(246, 10),
            Probability::new(249, 7),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(7, 249),
            Probability::new(10, 246),
            Probability::new(14, 242),
            Probability::new(17, 239),
            Probability::new(20, 236),
            Probability::new(24, 232),
            Probability::new(27, 229),
            Probability::new(31, 225),
            Probability::new(34, 222),
            Probability::new(38, 218),
            Probability::new(41, 215),
            Probability::new(44, 212),
            Probability::new(48, 208),
            Probability::new(51, 205),
            Probability::new(55, 201),
            Probability::new(58, 198),
            Probability::new(61, 195),
            Probability::new(65, 191),
            Probability::new(68, 188),
            Probability::new(72, 184),
            Probability::new(75, 181),
            Probability::new(79, 177),
            Probability::new(82, 174),
            Probability::new(85, 171),
            Probability::new(89, 167),
            Probability::new(92, 164),
            Probability::new(96, 160),
            Probability::new(99, 157),
            Probability::new(102, 154),
            Probability::new(106, 150),
            Probability::new(109, 147),
            Probability::new(113, 143),
            Probability::new(116, 140),
            Probability::new(119, 137),
            Probability::new(123, 133),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(133, 123),
            Probability::new(137, 119),
            Probability::new(140, 116),
            Probability::new(143, 113),
            Probability::new(147, 109),
            Probability::new(150, 106),
            Probability::new(154, 102),
            Probability::new(157, 99),
            Probability::new(160, 96),
            Probability::new(164, 92),
            Probability::new(167, 89),
            Probability::new(171, 85),
            Probability::new(174, 82),
            Probability::new(177, 79),
            Probability::new(181, 75),
            Probability::new(184, 72),
            Probability::new(188, 68),
            Probability::new(191, 65),
            Probability::new(195, 61),
            Probability::new(198, 58),
            Probability::new(201, 55),
            Probability::new(205, 51),
            Probability::new(208, 48),
            Probability::new(212, 44),
            Probability::new(215, 41),
            Probability::new(218, 38),
            Probability::new(222, 34),
            Probability::new(225, 31),
            Probability::new(229, 27),
            Probability::new(232, 24),
            Probability::new(236, 20),
            Probability::new(239, 17),
            Probability::new(242, 14),
            Probability::new(246, 10),
            Probability::new(249, 7),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(7, 249),
            Probability::new(10, 246),
            Probability::new(13, 243),
            Probability::new(17, 239),
            Probability::new(20, 236),
            Probability::new(24, 232),
            Probability::new(27, 229),
            Probability::new(30, 226),
            Probability::new(34, 222),
            Probability::new(37, 219),
            Probability::new(40, 216),
            Probability::new(44, 212),
            Probability::new(47, 209),
            Probability::new(51, 205),
            Probability::new(54, 202),
            Probability::new(57, 199),
            Probability::new(61, 195),
            Probability::new(64, 192),
            Probability::new(67, 189),
            Probability::new(71, 185),
            Probability::new(74, 182),
            Probability::new(77, 179),
            Probability::new(81, 175),
            Probability::new(84, 172),
            Probability::new(88, 168),
            Probability::new(91, 165),
            Probability::new(94, 162),
            Probability::new(98, 158),
            Probability::new(101, 155),
            Probability::new(104, 152),
            Probability::new(108, 148),
            Probability::new(111, 145),
            Probability::new(115, 141),
            Probability::new(118, 138),
            Probability::new(121, 135),
            Probability::new(125, 131),
            Probability::new(128, 128),
            Probability::new(131, 125),
            Probability::new(135, 121),
            Probability::new(138, 118),
            Probability::new(141, 115),
            Probability::new(145, 111),
            Probability::new(148, 108),
            Probability::new(152, 104),
            Probability::new(155, 101),
            Probability::new(158, 98),
            Probability::new(162, 94),
            Probability::new(165, 91),
            Probability::new(168, 88),
            Probability::new(172, 84),
            Probability::new(175, 81),
            Probability::new(179, 77),
            Probability::new(182, 74),
            Probability::new(185, 71),
            Probability::new(189, 67),
            Probability::new(192, 64),
            Probability::new(195, 61),
            Probability::new(199, 57),
            Probability::new(202, 54),
            Probability::new(205, 51),
            Probability::new(209, 47),
            Probability::new(212, 44),
            Probability::new(216, 40),
            Probability::new(219, 37),
            Probability::new(222, 34),
            Probability::new(226, 30),
            Probability::new(229, 27),
            Probability::new(232, 24),
            Probability::new(236, 20),
            Probability::new(239, 17),
            Probability::new(243, 13),
            Probability::new(246, 10),
            Probability::new(249, 7),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(7, 249),
            Probability::new(10, 246),
            Probability::new(13, 243),
            Probability::new(17, 239),
            Probability::new(20, 236),
            Probability::new(23, 233),
            Probability::new(27, 229),
            Probability::new(30, 226),
            Probability::new(33, 223),
            Probability::new(37, 219),
            Probability::new(40, 216),
            Probability::new(43, 213),
            Probability::new(47, 209),
            Probability::new(50, 206),
            Probability::new(53, 203),
            Probability::new(57, 199),
            Probability::new(60, 196),
            Probability::new(63, 193),
            Probability::new(66, 190),
            Probability::new(70, 186),
            Probability::new(73, 183),
            Probability::new(76, 180),
            Probability::new(80, 176),
            Probability::new(83, 173),
            Probability::new(86, 170),
            Probability::new(90, 166),
            Probability::new(93, 163),
            Probability::new(96, 160),
            Probability::new(100, 156),
            Probability::new(103, 153),
            Probability::new(106, 150),
            Probability::new(110, 146),
            Probability::new(113, 143),
            Probability::new(116, 140),
            Probability::new(120, 136),
            Probability::new(123, 133),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(133, 123),
            Probability::new(136, 120),
            Probability::new(140, 116),
            Probability::new(143, 113),
            Probability::new(146, 110),
            Probability::new(150, 106),
            Probability::new(153, 103),
            Probability::new(156, 100),
            Probability::new(160, 96),
            Probability::new(163, 93),
            Probability::new(166, 90),
            Probability::new(170, 86),
            Probability::new(173, 83),
            Probability::new(176, 80),
            Probability::new(180, 76),
            Probability::new(183, 73),
            Probability::new(186, 70),
            Probability::new(190, 66),
            Probability::new(193, 63),
            Probability::new(196, 60),
            Probability::new(199, 57),
            Probability::new(203, 53),
            Probability::new(206, 50),
            Probability::new(209, 47),
            Probability::new(213, 43),
            Probability::new(216, 40),
            Probability::new(219, 37),
            Probability::new(223, 33),
            Probability::new(226, 30),
            Probability::new(229, 27),
            Probability::new(233, 23),
            Probability::new(236, 20),
            Probability::new(239, 17),
            Probability::new(243, 13),
            Probability::new(246, 10),
            Probability::new(249, 7),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(7, 249),
            Probability::new(10, 246),
            Probability::new(13, 243),
            Probability::new(16, 240),
            Probability::new(20, 236),
            Probability::new(23, 233),
            Probability::new(26, 230),
            Probability::new(30, 226),
            Probability::new(33, 223),
            Probability::new(36, 220),
            Probability::new(39, 217),
            Probability::new(43, 213),
            Probability::new(46, 210),
            Probability::new(49, 207),
            Probability::new(53, 203),
            Probability::new(56, 200),
            Probability::new(59, 197),
            Probability::new(62, 194),
            Probability::new(66, 190),
            Probability::new(69, 187),
            Probability::new(72, 184),
            Probability::new(75, 181),
            Probability::new(79, 177),
            Probability::new(82, 174),
            Probability::new(85, 171),
            Probability::new(89, 167),
            Probability::new(92, 164),
            Probability::new(95, 161),
            Probability::new(98, 158),
            Probability::new(102, 154),
            Probability::new(105, 151),
            Probability::new(108, 148),
            Probability::new(112, 144),
            Probability::new(115, 141),
            Probability::new(118, 138),
            Probability::new(121, 135),
            Probability::new(125, 131),
            Probability::new(128, 128),
            Probability::new(131, 125),
            Probability::new(135, 121),
            Probability::new(138, 118),
            Probability::new(141, 115),
            Probability::new(144, 112),
            Probability::new(148, 108),
            Probability::new(151, 105),
            Probability::new(154, 102),
            Probability::new(158, 98),
            Probability::new(161, 95),
            Probability::new(164, 92),
            Probability::new(167, 89),
            Probability::new(171, 85),
            Probability::new(174, 82),
            Probability::new(177, 79),
            Probability::new(181, 75),
            Probability::new(184, 72),
            Probability::new(187, 69),
            Probability::new(190, 66),
            Probability::new(194, 62),
            Probability::new(197, 59),
            Probability::new(200, 56),
            Probability::new(203, 53),
            Probability::new(207, 49),
            Probability::new(210, 46),
            Probability::new(213, 43),
            Probability::new(217, 39),
            Probability::new(220, 36),
            Probability::new(223, 33),
            Probability::new(226, 30),
            Probability::new(230, 26),
            Probability::new(233, 23),
            Probability::new(236, 20),
            Probability::new(240, 16),
            Probability::new(243, 13),
            Probability::new(246, 10),
            Probability::new(249, 7),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(7, 249),
            Probability::new(10, 246),
            Probability::new(13, 243),
            Probability::new(16, 240),
            Probability::new(19, 237),
            Probability::new(23, 233),
            Probability::new(26, 230),
            Probability::new(29, 227),
            Probability::new(32, 224),
            Probability::new(36, 220),
            Probability::new(39, 217),
            Probability::new(42, 214),
            Probability::new(45, 211),
            Probability::new(49, 207),
            Probability::new(52, 204),
            Probability::new(55, 201),
            Probability::new(58, 198),
            Probability::new(62, 194),
            Probability::new(65, 191),
            Probability::new(68, 188),
            Probability::new(71, 185),
            Probability::new(75, 181),
            Probability::new(78, 178),
            Probability::new(81, 175),
            Probability::new(84, 172),
            Probability::new(87, 169),
            Probability::new(91, 165),
            Probability::new(94, 162),
            Probability::new(97, 159),
            Probability::new(100, 156),
            Probability::new(104, 152),
            Probability::new(107, 149),
            Probability::new(110, 146),
            Probability::new(113, 143),
            Probability::new(117, 139),
            Probability::new(120, 136),
            Probability::new(123, 133),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(133, 123),
            Probability::new(136, 120),
            Probability::new(139, 117),
            Probability::new(143, 113),
            Probability::new(146, 110),
            Probability::new(149, 107),
            Probability::new(152, 104),
            Probability::new(156, 100),
            Probability::new(159, 97),
            Probability::new(162, 94),
            Probability::new(165, 91),
            Probability::new(169, 87),
            Probability::new(172, 84),
            Probability::new(175, 81),
            Probability::new(178, 78),
            Probability::new(181, 75),
            Probability::new(185, 71),
            Probability::new(188, 68),
            Probability::new(191, 65),
            Probability::new(194, 62),
            Probability::new(198, 58),
            Probability::new(201, 55),
            Probability::new(204, 52),
            Probability::new(207, 49),
            Probability::new(211, 45),
            Probability::new(214, 42),
            Probability::new(217, 39),
            Probability::new(220, 36),
            Probability::new(224, 32),
            Probability::new(227, 29),
            Probability::new(230, 26),
            Probability::new(233, 23),
            Probability::new(237, 19),
            Probability::new(240, 16),
            Probability::new(243, 13),
            Probability::new(246, 10),
            Probability::new(249, 7),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(6, 250),
            Probability::new(10, 246),
            Probability::new(13, 243),
            Probability::new(16, 240),
            Probability::new(19, 237),
            Probability::new(22, 234),
            Probability::new(26, 230),
            Probability::new(29, 227),
            Probability::new(32, 224),
            Probability::new(35, 221),
            Probability::new(38, 218),
            Probability::new(42, 214),
            Probability::new(45, 211),
            Probability::new(48, 208),
            Probability::new(51, 205),
            Probability::new(54, 202),
            Probability::new(58, 198),
            Probability::new(61, 195),
            Probability::new(64, 192),
            Probability::new(67, 189),
            Probability::new(70, 186),
            Probability::new(74, 182),
            Probability::new(77, 179),
            Probability::new(80, 176),
            Probability::new(83, 173),
            Probability::new(86, 170),
            Probability::new(90, 166),
            Probability::new(93, 163),
            Probability::new(96, 160),
            Probability::new(99, 157),
            Probability::new(102, 154),
            Probability::new(106, 150),
            Probability::new(109, 147),
            Probability::new(112, 144),
            Probability::new(115, 141),
            Probability::new(118, 138),
            Probability::new(122, 134),
            Probability::new(125, 131),
            Probability::new(128, 128),
            Probability::new(131, 125),
            Probability::new(134, 122),
            Probability::new(138, 118),
            Probability::new(141, 115),
            Probability::new(144, 112),
            Probability::new(147, 109),
            Probability::new(150, 106),
            Probability::new(154, 102),
            Probability::new(157, 99),
            Probability::new(160, 96),
            Probability::new(163, 93),
            Probability::new(166, 90),
            Probability::new(170, 86),
            Probability::new(173, 83),
            Probability::new(176, 80),
            Probability::new(179, 77),
            Probability::new(182, 74),
            Probability::new(186, 70),
            Probability::new(189, 67),
            Probability::new(192, 64),
            Probability::new(195, 61),
            Probability::new(198, 58),
            Probability::new(202, 54),
            Probability::new(205, 51),
            Probability::new(208, 48),
            Probability::new(211, 45),
            Probability::new(214, 42),
            Probability::new(218, 38),
            Probability::new(221, 35),
            Probability::new(224, 32),
            Probability::new(227, 29),
            Probability::new(230, 26),
            Probability::new(234, 22),
            Probability::new(237, 19),
            Probability::new(240, 16),
            Probability::new(243, 13),
            Probability::new(246, 10),
            Probability::new(250, 6),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(6, 250),
            Probability::new(9, 247),
            Probability::new(13, 243),
            Probability::new(16, 240),
            Probability::new(19, 237),
            Probability::new(22, 234),
            Probability::new(25, 231),
            Probability::new(28, 228),
            Probability::new(32, 224),
            Probability::new(35, 221),
            Probability::new(38, 218),
            Probability::new(41, 215),
            Probability::new(44, 212),
            Probability::new(47, 209),
            Probability::new(51, 205),
            Probability::new(54, 202),
            Probability::new(57, 199),
            Probability::new(60, 196),
            Probability::new(63, 193),
            Probability::new(66, 190),
            Probability::new(70, 186),
            Probability::new(73, 183),
            Probability::new(76, 180),
            Probability::new(79, 177),
            Probability::new(82, 174),
            Probability::new(85, 171),
            Probability::new(88, 168),
            Probability::new(92, 164),
            Probability::new(95, 161),
            Probability::new(98, 158),
            Probability::new(101, 155),
            Probability::new(104, 152),
            Probability::new(107, 149),
            Probability::new(111, 145),
            Probability::new(114, 142),
            Probability::new(117, 139),
            Probability::new(120, 136),
            Probability::new(123, 133),
            Probability::new(126, 130),
            Probability::new(130, 126),
            Probability::new(133, 123),
            Probability::new(136, 120),
            Probability::new(139, 117),
            Probability::new(142, 114),
            Probability::new(145, 111),
            Probability::new(149, 107),
            Probability::new(152, 104),
            Probability::new(155, 101),
            Probability::new(158, 98),
            Probability::new(161, 95),
            Probability::new(164, 92),
            Probability::new(168, 88),
            Probability::new(171, 85),
            Probability::new(174, 82),
            Probability::new(177, 79),
            Probability::new(180, 76),
            Probability::new(183, 73),
            Probability::new(186, 70),
            Probability::new(190, 66),
            Probability::new(193, 63),
            Probability::new(196, 60),
            Probability::new(199, 57),
            Probability::new(202, 54),
            Probability::new(205, 51),
            Probability::new(209, 47),
            Probability::new(212, 44),
            Probability::new(215, 41),
            Probability::new(218, 38),
            Probability::new(221, 35),
            Probability::new(224, 32),
            Probability::new(228, 28),
            Probability::new(231, 25),
            Probability::new(234, 22),
            Probability::new(237, 19),
            Probability::new(240, 16),
            Probability::new(243, 13),
            Probability::new(247, 9),
            Probability::new(250, 6),
            Probability::new(253, 3),
            Probability::new(3, 253),
            Probability::new(6, 250),
            Probability::new(9, 247),
            Probability::new(12, 244),
            Probability::new(16, 240),
            Probability::new(19, 237),
            Probability::new(22, 234),
            Probability::new(25, 231),
            Probability::new(28, 228),
            Probability::new(31, 225),
            Probability::new(34, 222),
            Probability::new(37, 219),
            Probability::new(41, 215),
            Probability::new(44, 212),
            Probability::new(47, 209),
            Probability::new(50, 206),
            Probability::new(53, 203),
            Probability::new(56, 200),
            Probability::new(59, 197),
            Probability::new(62, 194),
            Probability::new(66, 190),
            Probability::new(69, 187),
            Probability::new(72, 184),
            Probability::new(75, 181),
            Probability::new(78, 178),
            Probability::new(81, 175),
            Probability::new(84, 172),
            Probability::new(87, 169),
            Probability::new(91, 165),
            Probability::new(94, 162),
            Probability::new(97, 159),
            Probability::new(100, 156),
            Probability::new(103, 153),
            Probability::new(106, 150),
            Probability::new(109, 147),
            Probability::new(112, 144),
            Probability::new(116, 140),
            Probability::new(119, 137),
            Probability::new(122, 134),
            Probability::new(125, 131),
            Probability::new(128, 128),
            Probability::new(131, 125),
            Probability::new(134, 122),
            Probability::new(137, 119),
            Probability::new(140, 116),
            Probability::new(144, 112),
            Probability::new(147, 109),
            Probability::new(150, 106),
            Probability::new(153, 103),
            Probability::new(156, 100),
            Probability::new(159, 97),
            Probability::new(162, 94),
            Probability::new(165, 91),
            Probability::new(169, 87),
            Probability::new(172, 84),
            Probability::new(175, 81),
            Probability::new(178, 78),
            Probability::new(181, 75),
            Probability::new(184, 72),
            Probability::new(187, 69),
            Probability::new(190, 66),
            Probability::new(194, 62),
            Probability::new(197, 59),
            Probability::new(200, 56),
            Probability::new(203, 53),
            Probability::new(206, 50),
            Probability::new(209, 47),
            Probability::new(212, 44),
            Probability::new(215, 41),
            Probability::new(219, 37),
            Probability::new(222, 34),
            Probability::new(225, 31),
            Probability::new(228, 28),
            Probability::new(231, 25),
            Probability::new(234, 22),
            Probability::new(237, 19),
            Probability::new(240, 16),
            Probability::new(244, 12),
            Probability::new(247, 9),
            Probability::new(250, 6),
            Probability::new(253, 3),
        ];
        LOOKUP[self as usize]
    }

    #[inline]
    pub fn adapt(self, bit: bool) -> Self {
        const OUTCOMES: [BitContext; 2 * 3321] = [
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
            True0False50,
            True1False49,
            True2False48,
            True3False47,
            True4False46,
            True5False45,
            True6False44,
            True7False43,
            True8False42,
            True9False41,
            True10False40,
            True11False39,
            True12False38,
            True13False37,
            True14False36,
            True15False35,
            True16False34,
            True17False33,
            True18False32,
            True19False31,
            True20False30,
            True21False29,
            True22False28,
            True23False27,
            True24False26,
            True25False25,
            True26False24,
            True27False23,
            True28False22,
            True29False21,
            True30False20,
            True31False19,
            True32False18,
            True33False17,
            True34False16,
            True35False15,
            True36False14,
            True37False13,
            True38False12,
            True39False11,
            True40False10,
            True41False9,
            True42False8,
            True43False7,
            True44False6,
            True45False5,
            True46False4,
            True47False3,
            True48False2,
            True49False1,
            True0False51,
            True1False50,
            True2False49,
            True3False48,
            True4False47,
            True5False46,
            True6False45,
            True7False44,
            True8False43,
            True9False42,
            True10False41,
            True11False40,
            True12False39,
            True13False38,
            True14False37,
            True15False36,
            True16False35,
            True17False34,
            True18False33,
            True19False32,
            True20False31,
            True21False30,
            True22False29,
            True23False28,
            True24False27,
            True25False26,
            True26False25,
            True27False24,
            True28False23,
            True29False22,
            True30False21,
            True31False20,
            True32False19,
            True33False18,
            True34False17,
            True35False16,
            True36False15,
            True37False14,
            True38False13,
            True39False12,
            True40False11,
            True41False10,
            True42False9,
            True43False8,
            True44False7,
            True45False6,
            True46False5,
            True47False4,
            True48False3,
            True49False2,
            True50False1,
            True0False52,
            True1False51,
            True2False50,
            True3False49,
            True4False48,
            True5False47,
            True6False46,
            True7False45,
            True8False44,
            True9False43,
            True10False42,
            True11False41,
            True12False40,
            True13False39,
            True14False38,
            True15False37,
            True16False36,
            True17False35,
            True18False34,
            True19False33,
            True20False32,
            True21False31,
            True22False30,
            True23False29,
            True24False28,
            True25False27,
            True26False26,
            True27False25,
            True28False24,
            True29False23,
            True30False22,
            True31False21,
            True32False20,
            True33False19,
            True34False18,
            True35False17,
            True36False16,
            True37False15,
            True38False14,
            True39False13,
            True40False12,
            True41False11,
            True42False10,
            True43False9,
            True44False8,
            True45False7,
            True46False6,
            True47False5,
            True48False4,
            True49False3,
            True50False2,
            True51False1,
            True0False53,
            True1False52,
            True2False51,
            True3False50,
            True4False49,
            True5False48,
            True6False47,
            True7False46,
            True8False45,
            True9False44,
            True10False43,
            True11False42,
            True12False41,
            True13False40,
            True14False39,
            True15False38,
            True16False37,
            True17False36,
            True18False35,
            True19False34,
            True20False33,
            True21False32,
            True22False31,
            True23False30,
            True24False29,
            True25False28,
            True26False27,
            True27False26,
            True28False25,
            True29False24,
            True30False23,
            True31False22,
            True32False21,
            True33False20,
            True34False19,
            True35False18,
            True36False17,
            True37False16,
            True38False15,
            True39False14,
            True40False13,
            True41False12,
            True42False11,
            True43False10,
            True44False9,
            True45False8,
            True46False7,
            True47False6,
            True48False5,
            True49False4,
            True50False3,
            True51False2,
            True52False1,
            True0False54,
            True1False53,
            True2False52,
            True3False51,
            True4False50,
            True5False49,
            True6False48,
            True7False47,
            True8False46,
            True9False45,
            True10False44,
            True11False43,
            True12False42,
            True13False41,
            True14False40,
            True15False39,
            True16False38,
            True17False37,
            True18False36,
            True19False35,
            True20False34,
            True21False33,
            True22False32,
            True23False31,
            True24False30,
            True25False29,
            True26False28,
            True27False27,
            True28False26,
            True29False25,
            True30False24,
            True31False23,
            True32False22,
            True33False21,
            True34False20,
            True35False19,
            True36False18,
            True37False17,
            True38False16,
            True39False15,
            True40False14,
            True41False13,
            True42False12,
            True43False11,
            True44False10,
            True45False9,
            True46False8,
            True47False7,
            True48False6,
            True49False5,
            True50False4,
            True51False3,
            True52False2,
            True53False1,
            True0False55,
            True1False54,
            True2False53,
            True3False52,
            True4False51,
            True5False50,
            True6False49,
            True7False48,
            True8False47,
            True9False46,
            True10False45,
            True11False44,
            True12False43,
            True13False42,
            True14False41,
            True15False40,
            True16False39,
            True17False38,
            True18False37,
            True19False36,
            True20False35,
            True21False34,
            True22False33,
            True23False32,
            True24False31,
            True25False30,
            True26False29,
            True27False28,
            True28False27,
            True29False26,
            True30False25,
            True31False24,
            True32False23,
            True33False22,
            True34False21,
            True35False20,
            True36False19,
            True37False18,
            True38False17,
            True39False16,
            True40False15,
            True41False14,
            True42False13,
            True43False12,
            True44False11,
            True45False10,
            True46False9,
            True47False8,
            True48False7,
            True49False6,
            True50False5,
            True51False4,
            True52False3,
            True53False2,
            True54False1,
            True0False56,
            True1False55,
            True2False54,
            True3False53,
            True4False52,
            True5False51,
            True6False50,
            True7False49,
            True8False48,
            True9False47,
            True10False46,
            True11False45,
            True12False44,
            True13False43,
            True14False42,
            True15False41,
            True16False40,
            True17False39,
            True18False38,
            True19False37,
            True20False36,
            True21False35,
            True22False34,
            True23False33,
            True24False32,
            True25False31,
            True26False30,
            True27False29,
            True28False28,
            True29False27,
            True30False26,
            True31False25,
            True32False24,
            True33False23,
            True34False22,
            True35False21,
            True36False20,
            True37False19,
            True38False18,
            True39False17,
            True40False16,
            True41False15,
            True42False14,
            True43False13,
            True44False12,
            True45False11,
            True46False10,
            True47False9,
            True48False8,
            True49False7,
            True50False6,
            True51False5,
            True52False4,
            True53False3,
            True54False2,
            True55False1,
            True0False57,
            True1False56,
            True2False55,
            True3False54,
            True4False53,
            True5False52,
            True6False51,
            True7False50,
            True8False49,
            True9False48,
            True10False47,
            True11False46,
            True12False45,
            True13False44,
            True14False43,
            True15False42,
            True16False41,
            True17False40,
            True18False39,
            True19False38,
            True20False37,
            True21False36,
            True22False35,
            True23False34,
            True24False33,
            True25False32,
            True26False31,
            True27False30,
            True28False29,
            True29False28,
            True30False27,
            True31False26,
            True32False25,
            True33False24,
            True34False23,
            True35False22,
            True36False21,
            True37False20,
            True38False19,
            True39False18,
            True40False17,
            True41False16,
            True42False15,
            True43False14,
            True44False13,
            True45False12,
            True46False11,
            True47False10,
            True48False9,
            True49False8,
            True50False7,
            True51False6,
            True52False5,
            True53False4,
            True54False3,
            True55False2,
            True56False1,
            True0False58,
            True1False57,
            True2False56,
            True3False55,
            True4False54,
            True5False53,
            True6False52,
            True7False51,
            True8False50,
            True9False49,
            True10False48,
            True11False47,
            True12False46,
            True13False45,
            True14False44,
            True15False43,
            True16False42,
            True17False41,
            True18False40,
            True19False39,
            True20False38,
            True21False37,
            True22False36,
            True23False35,
            True24False34,
            True25False33,
            True26False32,
            True27False31,
            True28False30,
            True29False29,
            True30False28,
            True31False27,
            True32False26,
            True33False25,
            True34False24,
            True35False23,
            True36False22,
            True37False21,
            True38False20,
            True39False19,
            True40False18,
            True41False17,
            True42False16,
            True43False15,
            True44False14,
            True45False13,
            True46False12,
            True47False11,
            True48False10,
            True49False9,
            True50False8,
            True51False7,
            True52False6,
            True53False5,
            True54False4,
            True55False3,
            True56False2,
            True57False1,
            True0False59,
            True1False58,
            True2False57,
            True3False56,
            True4False55,
            True5False54,
            True6False53,
            True7False52,
            True8False51,
            True9False50,
            True10False49,
            True11False48,
            True12False47,
            True13False46,
            True14False45,
            True15False44,
            True16False43,
            True17False42,
            True18False41,
            True19False40,
            True20False39,
            True21False38,
            True22False37,
            True23False36,
            True24False35,
            True25False34,
            True26False33,
            True27False32,
            True28False31,
            True29False30,
            True30False29,
            True31False28,
            True32False27,
            True33False26,
            True34False25,
            True35False24,
            True36False23,
            True37False22,
            True38False21,
            True39False20,
            True40False19,
            True41False18,
            True42False17,
            True43False16,
            True44False15,
            True45False14,
            True46False13,
            True47False12,
            True48False11,
            True49False10,
            True50False9,
            True51False8,
            True52False7,
            True53False6,
            True54False5,
            True55False4,
            True56False3,
            True57False2,
            True58False1,
            True0False60,
            True1False59,
            True2False58,
            True3False57,
            True4False56,
            True5False55,
            True6False54,
            True7False53,
            True8False52,
            True9False51,
            True10False50,
            True11False49,
            True12False48,
            True13False47,
            True14False46,
            True15False45,
            True16False44,
            True17False43,
            True18False42,
            True19False41,
            True20False40,
            True21False39,
            True22False38,
            True23False37,
            True24False36,
            True25False35,
            True26False34,
            True27False33,
            True28False32,
            True29False31,
            True30False30,
            True31False29,
            True32False28,
            True33False27,
            True34False26,
            True35False25,
            True36False24,
            True37False23,
            True38False22,
            True39False21,
            True40False20,
            True41False19,
            True42False18,
            True43False17,
            True44False16,
            True45False15,
            True46False14,
            True47False13,
            True48False12,
            True49False11,
            True50False10,
            True51False9,
            True52False8,
            True53False7,
            True54False6,
            True55False5,
            True56False4,
            True57False3,
            True58False2,
            True59False1,
            True0False61,
            True1False60,
            True2False59,
            True3False58,
            True4False57,
            True5False56,
            True6False55,
            True7False54,
            True8False53,
            True9False52,
            True10False51,
            True11False50,
            True12False49,
            True13False48,
            True14False47,
            True15False46,
            True16False45,
            True17False44,
            True18False43,
            True19False42,
            True20False41,
            True21False40,
            True22False39,
            True23False38,
            True24False37,
            True25False36,
            True26False35,
            True27False34,
            True28False33,
            True29False32,
            True30False31,
            True31False30,
            True32False29,
            True33False28,
            True34False27,
            True35False26,
            True36False25,
            True37False24,
            True38False23,
            True39False22,
            True40False21,
            True41False20,
            True42False19,
            True43False18,
            True44False17,
            True45False16,
            True46False15,
            True47False14,
            True48False13,
            True49False12,
            True50False11,
            True51False10,
            True52False9,
            True53False8,
            True54False7,
            True55False6,
            True56False5,
            True57False4,
            True58False3,
            True59False2,
            True60False1,
            True0False62,
            True1False61,
            True2False60,
            True3False59,
            True4False58,
            True5False57,
            True6False56,
            True7False55,
            True8False54,
            True9False53,
            True10False52,
            True11False51,
            True12False50,
            True13False49,
            True14False48,
            True15False47,
            True16False46,
            True17False45,
            True18False44,
            True19False43,
            True20False42,
            True21False41,
            True22False40,
            True23False39,
            True24False38,
            True25False37,
            True26False36,
            True27False35,
            True28False34,
            True29False33,
            True30False32,
            True31False31,
            True32False30,
            True33False29,
            True34False28,
            True35False27,
            True36False26,
            True37False25,
            True38False24,
            True39False23,
            True40False22,
            True41False21,
            True42False20,
            True43False19,
            True44False18,
            True45False17,
            True46False16,
            True47False15,
            True48False14,
            True49False13,
            True50False12,
            True51False11,
            True52False10,
            True53False9,
            True54False8,
            True55False7,
            True56False6,
            True57False5,
            True58False4,
            True59False3,
            True60False2,
            True61False1,
            True0False63,
            True1False62,
            True2False61,
            True3False60,
            True4False59,
            True5False58,
            True6False57,
            True7False56,
            True8False55,
            True9False54,
            True10False53,
            True11False52,
            True12False51,
            True13False50,
            True14False49,
            True15False48,
            True16False47,
            True17False46,
            True18False45,
            True19False44,
            True20False43,
            True21False42,
            True22False41,
            True23False40,
            True24False39,
            True25False38,
            True26False37,
            True27False36,
            True28False35,
            True29False34,
            True30False33,
            True31False32,
            True32False31,
            True33False30,
            True34False29,
            True35False28,
            True36False27,
            True37False26,
            True38False25,
            True39False24,
            True40False23,
            True41False22,
            True42False21,
            True43False20,
            True44False19,
            True45False18,
            True46False17,
            True47False16,
            True48False15,
            True49False14,
            True50False13,
            True51False12,
            True52False11,
            True53False10,
            True54False9,
            True55False8,
            True56False7,
            True57False6,
            True58False5,
            True59False4,
            True60False3,
            True61False2,
            True62False1,
            True0False64,
            True1False63,
            True2False62,
            True3False61,
            True4False60,
            True5False59,
            True6False58,
            True7False57,
            True8False56,
            True9False55,
            True10False54,
            True11False53,
            True12False52,
            True13False51,
            True14False50,
            True15False49,
            True16False48,
            True17False47,
            True18False46,
            True19False45,
            True20False44,
            True21False43,
            True22False42,
            True23False41,
            True24False40,
            True25False39,
            True26False38,
            True27False37,
            True28False36,
            True29False35,
            True30False34,
            True31False33,
            True32False32,
            True33False31,
            True34False30,
            True35False29,
            True36False28,
            True37False27,
            True38False26,
            True39False25,
            True40False24,
            True41False23,
            True42False22,
            True43False21,
            True44False20,
            True45False19,
            True46False18,
            True47False17,
            True48False16,
            True49False15,
            True50False14,
            True51False13,
            True52False12,
            True53False11,
            True54False10,
            True55False9,
            True56False8,
            True57False7,
            True58False6,
            True59False5,
            True60False4,
            True61False3,
            True62False2,
            True63False1,
            True0False65,
            True1False64,
            True2False63,
            True3False62,
            True4False61,
            True5False60,
            True6False59,
            True7False58,
            True8False57,
            True9False56,
            True10False55,
            True11False54,
            True12False53,
            True13False52,
            True14False51,
            True15False50,
            True16False49,
            True17False48,
            True18False47,
            True19False46,
            True20False45,
            True21False44,
            True22False43,
            True23False42,
            True24False41,
            True25False40,
            True26False39,
            True27False38,
            True28False37,
            True29False36,
            True30False35,
            True31False34,
            True32False33,
            True33False32,
            True34False31,
            True35False30,
            True36False29,
            True37False28,
            True38False27,
            True39False26,
            True40False25,
            True41False24,
            True42False23,
            True43False22,
            True44False21,
            True45False20,
            True46False19,
            True47False18,
            True48False17,
            True49False16,
            True50False15,
            True51False14,
            True52False13,
            True53False12,
            True54False11,
            True55False10,
            True56False9,
            True57False8,
            True58False7,
            True59False6,
            True60False5,
            True61False4,
            True62False3,
            True63False2,
            True64False1,
            True0False66,
            True1False65,
            True2False64,
            True3False63,
            True4False62,
            True5False61,
            True6False60,
            True7False59,
            True8False58,
            True9False57,
            True10False56,
            True11False55,
            True12False54,
            True13False53,
            True14False52,
            True15False51,
            True16False50,
            True17False49,
            True18False48,
            True19False47,
            True20False46,
            True21False45,
            True22False44,
            True23False43,
            True24False42,
            True25False41,
            True26False40,
            True27False39,
            True28False38,
            True29False37,
            True30False36,
            True31False35,
            True32False34,
            True33False33,
            True34False32,
            True35False31,
            True36False30,
            True37False29,
            True38False28,
            True39False27,
            True40False26,
            True41False25,
            True42False24,
            True43False23,
            True44False22,
            True45False21,
            True46False20,
            True47False19,
            True48False18,
            True49False17,
            True50False16,
            True51False15,
            True52False14,
            True53False13,
            True54False12,
            True55False11,
            True56False10,
            True57False9,
            True58False8,
            True59False7,
            True60False6,
            True61False5,
            True62False4,
            True63False3,
            True64False2,
            True65False1,
            True0False67,
            True1False66,
            True2False65,
            True3False64,
            True4False63,
            True5False62,
            True6False61,
            True7False60,
            True8False59,
            True9False58,
            True10False57,
            True11False56,
            True12False55,
            True13False54,
            True14False53,
            True15False52,
            True16False51,
            True17False50,
            True18False49,
            True19False48,
            True20False47,
            True21False46,
            True22False45,
            True23False44,
            True24False43,
            True25False42,
            True26False41,
            True27False40,
            True28False39,
            True29False38,
            True30False37,
            True31False36,
            True32False35,
            True33False34,
            True34False33,
            True35False32,
            True36False31,
            True37False30,
            True38False29,
            True39False28,
            True40False27,
            True41False26,
            True42False25,
            True43False24,
            True44False23,
            True45False22,
            True46False21,
            True47False20,
            True48False19,
            True49False18,
            True50False17,
            True51False16,
            True52False15,
            True53False14,
            True54False13,
            True55False12,
            True56False11,
            True57False10,
            True58False9,
            True59False8,
            True60False7,
            True61False6,
            True62False5,
            True63False4,
            True64False3,
            True65False2,
            True66False1,
            True0False68,
            True1False67,
            True2False66,
            True3False65,
            True4False64,
            True5False63,
            True6False62,
            True7False61,
            True8False60,
            True9False59,
            True10False58,
            True11False57,
            True12False56,
            True13False55,
            True14False54,
            True15False53,
            True16False52,
            True17False51,
            True18False50,
            True19False49,
            True20False48,
            True21False47,
            True22False46,
            True23False45,
            True24False44,
            True25False43,
            True26False42,
            True27False41,
            True28False40,
            True29False39,
            True30False38,
            True31False37,
            True32False36,
            True33False35,
            True34False34,
            True35False33,
            True36False32,
            True37False31,
            True38False30,
            True39False29,
            True40False28,
            True41False27,
            True42False26,
            True43False25,
            True44False24,
            True45False23,
            True46False22,
            True47False21,
            True48False20,
            True49False19,
            True50False18,
            True51False17,
            True52False16,
            True53False15,
            True54False14,
            True55False13,
            True56False12,
            True57False11,
            True58False10,
            True59False9,
            True60False8,
            True61False7,
            True62False6,
            True63False5,
            True64False4,
            True65False3,
            True66False2,
            True67False1,
            True0False69,
            True1False68,
            True2False67,
            True3False66,
            True4False65,
            True5False64,
            True6False63,
            True7False62,
            True8False61,
            True9False60,
            True10False59,
            True11False58,
            True12False57,
            True13False56,
            True14False55,
            True15False54,
            True16False53,
            True17False52,
            True18False51,
            True19False50,
            True20False49,
            True21False48,
            True22False47,
            True23False46,
            True24False45,
            True25False44,
            True26False43,
            True27False42,
            True28False41,
            True29False40,
            True30False39,
            True31False38,
            True32False37,
            True33False36,
            True34False35,
            True35False34,
            True36False33,
            True37False32,
            True38False31,
            True39False30,
            True40False29,
            True41False28,
            True42False27,
            True43False26,
            True44False25,
            True45False24,
            True46False23,
            True47False22,
            True48False21,
            True49False20,
            True50False19,
            True51False18,
            True52False17,
            True53False16,
            True54False15,
            True55False14,
            True56False13,
            True57False12,
            True58False11,
            True59False10,
            True60False9,
            True61False8,
            True62False7,
            True63False6,
            True64False5,
            True65False4,
            True66False3,
            True67False2,
            True68False1,
            True0False70,
            True1False69,
            True2False68,
            True3False67,
            True4False66,
            True5False65,
            True6False64,
            True7False63,
            True8False62,
            True9False61,
            True10False60,
            True11False59,
            True12False58,
            True13False57,
            True14False56,
            True15False55,
            True16False54,
            True17False53,
            True18False52,
            True19False51,
            True20False50,
            True21False49,
            True22False48,
            True23False47,
            True24False46,
            True25False45,
            True26False44,
            True27False43,
            True28False42,
            True29False41,
            True30False40,
            True31False39,
            True32False38,
            True33False37,
            True34False36,
            True35False35,
            True36False34,
            True37False33,
            True38False32,
            True39False31,
            True40False30,
            True41False29,
            True42False28,
            True43False27,
            True44False26,
            True45False25,
            True46False24,
            True47False23,
            True48False22,
            True49False21,
            True50False20,
            True51False19,
            True52False18,
            True53False17,
            True54False16,
            True55False15,
            True56False14,
            True57False13,
            True58False12,
            True59False11,
            True60False10,
            True61False9,
            True62False8,
            True63False7,
            True64False6,
            True65False5,
            True66False4,
            True67False3,
            True68False2,
            True69False1,
            True0False71,
            True1False70,
            True2False69,
            True3False68,
            True4False67,
            True5False66,
            True6False65,
            True7False64,
            True8False63,
            True9False62,
            True10False61,
            True11False60,
            True12False59,
            True13False58,
            True14False57,
            True15False56,
            True16False55,
            True17False54,
            True18False53,
            True19False52,
            True20False51,
            True21False50,
            True22False49,
            True23False48,
            True24False47,
            True25False46,
            True26False45,
            True27False44,
            True28False43,
            True29False42,
            True30False41,
            True31False40,
            True32False39,
            True33False38,
            True34False37,
            True35False36,
            True36False35,
            True37False34,
            True38False33,
            True39False32,
            True40False31,
            True41False30,
            True42False29,
            True43False28,
            True44False27,
            True45False26,
            True46False25,
            True47False24,
            True48False23,
            True49False22,
            True50False21,
            True51False20,
            True52False19,
            True53False18,
            True54False17,
            True55False16,
            True56False15,
            True57False14,
            True58False13,
            True59False12,
            True60False11,
            True61False10,
            True62False9,
            True63False8,
            True64False7,
            True65False6,
            True66False5,
            True67False4,
            True68False3,
            True69False2,
            True70False1,
            True0False72,
            True1False71,
            True2False70,
            True3False69,
            True4False68,
            True5False67,
            True6False66,
            True7False65,
            True8False64,
            True9False63,
            True10False62,
            True11False61,
            True12False60,
            True13False59,
            True14False58,
            True15False57,
            True16False56,
            True17False55,
            True18False54,
            True19False53,
            True20False52,
            True21False51,
            True22False50,
            True23False49,
            True24False48,
            True25False47,
            True26False46,
            True27False45,
            True28False44,
            True29False43,
            True30False42,
            True31False41,
            True32False40,
            True33False39,
            True34False38,
            True35False37,
            True36False36,
            True37False35,
            True38False34,
            True39False33,
            True40False32,
            True41False31,
            True42False30,
            True43False29,
            True44False28,
            True45False27,
            True46False26,
            True47False25,
            True48False24,
            True49False23,
            True50False22,
            True51False21,
            True52False20,
            True53False19,
            True54False18,
            True55False17,
            True56False16,
            True57False15,
            True58False14,
            True59False13,
            True60False12,
            True61False11,
            True62False10,
            True63False9,
            True64False8,
            True65False7,
            True66False6,
            True67False5,
            True68False4,
            True69False3,
            True70False2,
            True71False1,
            True0False73,
            True1False72,
            True2False71,
            True3False70,
            True4False69,
            True5False68,
            True6False67,
            True7False66,
            True8False65,
            True9False64,
            True10False63,
            True11False62,
            True12False61,
            True13False60,
            True14False59,
            True15False58,
            True16False57,
            True17False56,
            True18False55,
            True19False54,
            True20False53,
            True21False52,
            True22False51,
            True23False50,
            True24False49,
            True25False48,
            True26False47,
            True27False46,
            True28False45,
            True29False44,
            True30False43,
            True31False42,
            True32False41,
            True33False40,
            True34False39,
            True35False38,
            True36False37,
            True37False36,
            True38False35,
            True39False34,
            True40False33,
            True41False32,
            True42False31,
            True43False30,
            True44False29,
            True45False28,
            True46False27,
            True47False26,
            True48False25,
            True49False24,
            True50False23,
            True51False22,
            True52False21,
            True53False20,
            True54False19,
            True55False18,
            True56False17,
            True57False16,
            True58False15,
            True59False14,
            True60False13,
            True61False12,
            True62False11,
            True63False10,
            True64False9,
            True65False8,
            True66False7,
            True67False6,
            True68False5,
            True69False4,
            True70False3,
            True71False2,
            True72False1,
            True0False74,
            True1False73,
            True2False72,
            True3False71,
            True4False70,
            True5False69,
            True6False68,
            True7False67,
            True8False66,
            True9False65,
            True10False64,
            True11False63,
            True12False62,
            True13False61,
            True14False60,
            True15False59,
            True16False58,
            True17False57,
            True18False56,
            True19False55,
            True20False54,
            True21False53,
            True22False52,
            True23False51,
            True24False50,
            True25False49,
            True26False48,
            True27False47,
            True28False46,
            True29False45,
            True30False44,
            True31False43,
            True32False42,
            True33False41,
            True34False40,
            True35False39,
            True36False38,
            True37False37,
            True38False36,
            True39False35,
            True40False34,
            True41False33,
            True42False32,
            True43False31,
            True44False30,
            True45False29,
            True46False28,
            True47False27,
            True48False26,
            True49False25,
            True50False24,
            True51False23,
            True52False22,
            True53False21,
            True54False20,
            True55False19,
            True56False18,
            True57False17,
            True58False16,
            True59False15,
            True60False14,
            True61False13,
            True62False12,
            True63False11,
            True64False10,
            True65False9,
            True66False8,
            True67False7,
            True68False6,
            True69False5,
            True70False4,
            True71False3,
            True72False2,
            True73False1,
            True0False75,
            True1False74,
            True2False73,
            True3False72,
            True4False71,
            True5False70,
            True6False69,
            True7False68,
            True8False67,
            True9False66,
            True10False65,
            True11False64,
            True12False63,
            True13False62,
            True14False61,
            True15False60,
            True16False59,
            True17False58,
            True18False57,
            True19False56,
            True20False55,
            True21False54,
            True22False53,
            True23False52,
            True24False51,
            True25False50,
            True26False49,
            True27False48,
            True28False47,
            True29False46,
            True30False45,
            True31False44,
            True32False43,
            True33False42,
            True34False41,
            True35False40,
            True36False39,
            True37False38,
            True38False37,
            True39False36,
            True40False35,
            True41False34,
            True42False33,
            True43False32,
            True44False31,
            True45False30,
            True46False29,
            True47False28,
            True48False27,
            True49False26,
            True50False25,
            True51False24,
            True52False23,
            True53False22,
            True54False21,
            True55False20,
            True56False19,
            True57False18,
            True58False17,
            True59False16,
            True60False15,
            True61False14,
            True62False13,
            True63False12,
            True64False11,
            True65False10,
            True66False9,
            True67False8,
            True68False7,
            True69False6,
            True70False5,
            True71False4,
            True72False3,
            True73False2,
            True74False1,
            True0False76,
            True1False75,
            True2False74,
            True3False73,
            True4False72,
            True5False71,
            True6False70,
            True7False69,
            True8False68,
            True9False67,
            True10False66,
            True11False65,
            True12False64,
            True13False63,
            True14False62,
            True15False61,
            True16False60,
            True17False59,
            True18False58,
            True19False57,
            True20False56,
            True21False55,
            True22False54,
            True23False53,
            True24False52,
            True25False51,
            True26False50,
            True27False49,
            True28False48,
            True29False47,
            True30False46,
            True31False45,
            True32False44,
            True33False43,
            True34False42,
            True35False41,
            True36False40,
            True37False39,
            True38False38,
            True39False37,
            True40False36,
            True41False35,
            True42False34,
            True43False33,
            True44False32,
            True45False31,
            True46False30,
            True47False29,
            True48False28,
            True49False27,
            True50False26,
            True51False25,
            True52False24,
            True53False23,
            True54False22,
            True55False21,
            True56False20,
            True57False19,
            True58False18,
            True59False17,
            True60False16,
            True61False15,
            True62False14,
            True63False13,
            True64False12,
            True65False11,
            True66False10,
            True67False9,
            True68False8,
            True69False7,
            True70False6,
            True71False5,
            True72False4,
            True73False3,
            True74False2,
            True75False1,
            True0False77,
            True1False76,
            True2False75,
            True3False74,
            True4False73,
            True5False72,
            True6False71,
            True7False70,
            True8False69,
            True9False68,
            True10False67,
            True11False66,
            True12False65,
            True13False64,
            True14False63,
            True15False62,
            True16False61,
            True17False60,
            True18False59,
            True19False58,
            True20False57,
            True21False56,
            True22False55,
            True23False54,
            True24False53,
            True25False52,
            True26False51,
            True27False50,
            True28False49,
            True29False48,
            True30False47,
            True31False46,
            True32False45,
            True33False44,
            True34False43,
            True35False42,
            True36False41,
            True37False40,
            True38False39,
            True39False38,
            True40False37,
            True41False36,
            True42False35,
            True43False34,
            True44False33,
            True45False32,
            True46False31,
            True47False30,
            True48False29,
            True49False28,
            True50False27,
            True51False26,
            True52False25,
            True53False24,
            True54False23,
            True55False22,
            True56False21,
            True57False20,
            True58False19,
            True59False18,
            True60False17,
            True61False16,
            True62False15,
            True63False14,
            True64False13,
            True65False12,
            True66False11,
            True67False10,
            True68False9,
            True69False8,
            True70False7,
            True71False6,
            True72False5,
            True73False4,
            True74False3,
            True75False2,
            True76False1,
            True0False78,
            True1False77,
            True2False76,
            True3False75,
            True4False74,
            True5False73,
            True6False72,
            True7False71,
            True8False70,
            True9False69,
            True10False68,
            True11False67,
            True12False66,
            True13False65,
            True14False64,
            True15False63,
            True16False62,
            True17False61,
            True18False60,
            True19False59,
            True20False58,
            True21False57,
            True22False56,
            True23False55,
            True24False54,
            True25False53,
            True26False52,
            True27False51,
            True28False50,
            True29False49,
            True30False48,
            True31False47,
            True32False46,
            True33False45,
            True34False44,
            True35False43,
            True36False42,
            True37False41,
            True38False40,
            True39False39,
            True40False38,
            True41False37,
            True42False36,
            True43False35,
            True44False34,
            True45False33,
            True46False32,
            True47False31,
            True48False30,
            True49False29,
            True50False28,
            True51False27,
            True52False26,
            True53False25,
            True54False24,
            True55False23,
            True56False22,
            True57False21,
            True58False20,
            True59False19,
            True60False18,
            True61False17,
            True62False16,
            True63False15,
            True64False14,
            True65False13,
            True66False12,
            True67False11,
            True68False10,
            True69False9,
            True70False8,
            True71False7,
            True72False6,
            True73False5,
            True74False4,
            True75False3,
            True76False2,
            True77False1,
            True0False79,
            True1False78,
            True2False77,
            True3False76,
            True4False75,
            True5False74,
            True6False73,
            True7False72,
            True8False71,
            True9False70,
            True10False69,
            True11False68,
            True12False67,
            True13False66,
            True14False65,
            True15False64,
            True16False63,
            True17False62,
            True18False61,
            True19False60,
            True20False59,
            True21False58,
            True22False57,
            True23False56,
            True24False55,
            True25False54,
            True26False53,
            True27False52,
            True28False51,
            True29False50,
            True30False49,
            True31False48,
            True32False47,
            True33False46,
            True34False45,
            True35False44,
            True36False43,
            True37False42,
            True38False41,
            True39False40,
            True40False39,
            True41False38,
            True42False37,
            True43False36,
            True44False35,
            True45False34,
            True46False33,
            True47False32,
            True48False31,
            True49False30,
            True50False29,
            True51False28,
            True52False27,
            True53False26,
            True54False25,
            True55False24,
            True56False23,
            True57False22,
            True58False21,
            True59False20,
            True60False19,
            True61False18,
            True62False17,
            True63False16,
            True64False15,
            True65False14,
            True66False13,
            True67False12,
            True68False11,
            True69False10,
            True70False9,
            True71False8,
            True72False7,
            True73False6,
            True74False5,
            True75False4,
            True76False3,
            True77False2,
            True78False1,
            True0False80,
            True1False79,
            True2False78,
            True3False77,
            True4False76,
            True5False75,
            True6False74,
            True7False73,
            True8False72,
            True9False71,
            True10False70,
            True11False69,
            True12False68,
            True13False67,
            True14False66,
            True15False65,
            True16False64,
            True17False63,
            True18False62,
            True19False61,
            True20False60,
            True21False59,
            True22False58,
            True23False57,
            True24False56,
            True25False55,
            True26False54,
            True27False53,
            True28False52,
            True29False51,
            True30False50,
            True31False49,
            True32False48,
            True33False47,
            True34False46,
            True35False45,
            True36False44,
            True37False43,
            True38False42,
            True39False41,
            True40False40,
            True41False39,
            True42False38,
            True43False37,
            True44False36,
            True45False35,
            True46False34,
            True47False33,
            True48False32,
            True49False31,
            True50False30,
            True51False29,
            True52False28,
            True53False27,
            True54False26,
            True55False25,
            True56False24,
            True57False23,
            True58False22,
            True59False21,
            True60False20,
            True61False19,
            True62False18,
            True63False17,
            True64False16,
            True65False15,
            True66False14,
            True67False13,
            True68False12,
            True69False11,
            True70False10,
            True71False9,
            True72False8,
            True73False7,
            True74False6,
            True75False5,
            True76False4,
            True77False3,
            True78False2,
            True79False1,
            True0False80,
            True0False40,
            True1False39,
            True1False39,
            True2False38,
            True2False38,
            True3False37,
            True3False37,
            True4False36,
            True4False36,
            True5False35,
            True5False35,
            True6False34,
            True6False34,
            True7False33,
            True7False33,
            True8False32,
            True8False32,
            True9False31,
            True9False31,
            True10False30,
            True10False30,
            True11False29,
            True11False29,
            True12False28,
            True12False28,
            True13False27,
            True13False27,
            True14False26,
            True14False26,
            True15False25,
            True15False25,
            True16False24,
            True16False24,
            True17False23,
            True17False23,
            True18False22,
            True18False22,
            True19False21,
            True19False21,
            True20False20,
            True20False20,
            True21False19,
            True21False19,
            True22False18,
            True22False18,
            True23False17,
            True23False17,
            True24False16,
            True24False16,
            True25False15,
            True25False15,
            True26False14,
            True26False14,
            True27False13,
            True27False13,
            True28False12,
            True28False12,
            True29False11,
            True29False11,
            True30False10,
            True30False10,
            True31False9,
            True31False9,
            True32False8,
            True32False8,
            True33False7,
            True33False7,
            True34False6,
            True34False6,
            True35False5,
            True35False5,
            True36False4,
            True36False4,
            True37False3,
            True37False3,
            True38False2,
            True38False2,
            True39False1,
            True39False1,
            True40False0,
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
            True1False49,
            True2False48,
            True3False47,
            True4False46,
            True5False45,
            True6False44,
            True7False43,
            True8False42,
            True9False41,
            True10False40,
            True11False39,
            True12False38,
            True13False37,
            True14False36,
            True15False35,
            True16False34,
            True17False33,
            True18False32,
            True19False31,
            True20False30,
            True21False29,
            True22False28,
            True23False27,
            True24False26,
            True25False25,
            True26False24,
            True27False23,
            True28False22,
            True29False21,
            True30False20,
            True31False19,
            True32False18,
            True33False17,
            True34False16,
            True35False15,
            True36False14,
            True37False13,
            True38False12,
            True39False11,
            True40False10,
            True41False9,
            True42False8,
            True43False7,
            True44False6,
            True45False5,
            True46False4,
            True47False3,
            True48False2,
            True49False1,
            True50False0,
            True1False50,
            True2False49,
            True3False48,
            True4False47,
            True5False46,
            True6False45,
            True7False44,
            True8False43,
            True9False42,
            True10False41,
            True11False40,
            True12False39,
            True13False38,
            True14False37,
            True15False36,
            True16False35,
            True17False34,
            True18False33,
            True19False32,
            True20False31,
            True21False30,
            True22False29,
            True23False28,
            True24False27,
            True25False26,
            True26False25,
            True27False24,
            True28False23,
            True29False22,
            True30False21,
            True31False20,
            True32False19,
            True33False18,
            True34False17,
            True35False16,
            True36False15,
            True37False14,
            True38False13,
            True39False12,
            True40False11,
            True41False10,
            True42False9,
            True43False8,
            True44False7,
            True45False6,
            True46False5,
            True47False4,
            True48False3,
            True49False2,
            True50False1,
            True51False0,
            True1False51,
            True2False50,
            True3False49,
            True4False48,
            True5False47,
            True6False46,
            True7False45,
            True8False44,
            True9False43,
            True10False42,
            True11False41,
            True12False40,
            True13False39,
            True14False38,
            True15False37,
            True16False36,
            True17False35,
            True18False34,
            True19False33,
            True20False32,
            True21False31,
            True22False30,
            True23False29,
            True24False28,
            True25False27,
            True26False26,
            True27False25,
            True28False24,
            True29False23,
            True30False22,
            True31False21,
            True32False20,
            True33False19,
            True34False18,
            True35False17,
            True36False16,
            True37False15,
            True38False14,
            True39False13,
            True40False12,
            True41False11,
            True42False10,
            True43False9,
            True44False8,
            True45False7,
            True46False6,
            True47False5,
            True48False4,
            True49False3,
            True50False2,
            True51False1,
            True52False0,
            True1False52,
            True2False51,
            True3False50,
            True4False49,
            True5False48,
            True6False47,
            True7False46,
            True8False45,
            True9False44,
            True10False43,
            True11False42,
            True12False41,
            True13False40,
            True14False39,
            True15False38,
            True16False37,
            True17False36,
            True18False35,
            True19False34,
            True20False33,
            True21False32,
            True22False31,
            True23False30,
            True24False29,
            True25False28,
            True26False27,
            True27False26,
            True28False25,
            True29False24,
            True30False23,
            True31False22,
            True32False21,
            True33False20,
            True34False19,
            True35False18,
            True36False17,
            True37False16,
            True38False15,
            True39False14,
            True40False13,
            True41False12,
            True42False11,
            True43False10,
            True44False9,
            True45False8,
            True46False7,
            True47False6,
            True48False5,
            True49False4,
            True50False3,
            True51False2,
            True52False1,
            True53False0,
            True1False53,
            True2False52,
            True3False51,
            True4False50,
            True5False49,
            True6False48,
            True7False47,
            True8False46,
            True9False45,
            True10False44,
            True11False43,
            True12False42,
            True13False41,
            True14False40,
            True15False39,
            True16False38,
            True17False37,
            True18False36,
            True19False35,
            True20False34,
            True21False33,
            True22False32,
            True23False31,
            True24False30,
            True25False29,
            True26False28,
            True27False27,
            True28False26,
            True29False25,
            True30False24,
            True31False23,
            True32False22,
            True33False21,
            True34False20,
            True35False19,
            True36False18,
            True37False17,
            True38False16,
            True39False15,
            True40False14,
            True41False13,
            True42False12,
            True43False11,
            True44False10,
            True45False9,
            True46False8,
            True47False7,
            True48False6,
            True49False5,
            True50False4,
            True51False3,
            True52False2,
            True53False1,
            True54False0,
            True1False54,
            True2False53,
            True3False52,
            True4False51,
            True5False50,
            True6False49,
            True7False48,
            True8False47,
            True9False46,
            True10False45,
            True11False44,
            True12False43,
            True13False42,
            True14False41,
            True15False40,
            True16False39,
            True17False38,
            True18False37,
            True19False36,
            True20False35,
            True21False34,
            True22False33,
            True23False32,
            True24False31,
            True25False30,
            True26False29,
            True27False28,
            True28False27,
            True29False26,
            True30False25,
            True31False24,
            True32False23,
            True33False22,
            True34False21,
            True35False20,
            True36False19,
            True37False18,
            True38False17,
            True39False16,
            True40False15,
            True41False14,
            True42False13,
            True43False12,
            True44False11,
            True45False10,
            True46False9,
            True47False8,
            True48False7,
            True49False6,
            True50False5,
            True51False4,
            True52False3,
            True53False2,
            True54False1,
            True55False0,
            True1False55,
            True2False54,
            True3False53,
            True4False52,
            True5False51,
            True6False50,
            True7False49,
            True8False48,
            True9False47,
            True10False46,
            True11False45,
            True12False44,
            True13False43,
            True14False42,
            True15False41,
            True16False40,
            True17False39,
            True18False38,
            True19False37,
            True20False36,
            True21False35,
            True22False34,
            True23False33,
            True24False32,
            True25False31,
            True26False30,
            True27False29,
            True28False28,
            True29False27,
            True30False26,
            True31False25,
            True32False24,
            True33False23,
            True34False22,
            True35False21,
            True36False20,
            True37False19,
            True38False18,
            True39False17,
            True40False16,
            True41False15,
            True42False14,
            True43False13,
            True44False12,
            True45False11,
            True46False10,
            True47False9,
            True48False8,
            True49False7,
            True50False6,
            True51False5,
            True52False4,
            True53False3,
            True54False2,
            True55False1,
            True56False0,
            True1False56,
            True2False55,
            True3False54,
            True4False53,
            True5False52,
            True6False51,
            True7False50,
            True8False49,
            True9False48,
            True10False47,
            True11False46,
            True12False45,
            True13False44,
            True14False43,
            True15False42,
            True16False41,
            True17False40,
            True18False39,
            True19False38,
            True20False37,
            True21False36,
            True22False35,
            True23False34,
            True24False33,
            True25False32,
            True26False31,
            True27False30,
            True28False29,
            True29False28,
            True30False27,
            True31False26,
            True32False25,
            True33False24,
            True34False23,
            True35False22,
            True36False21,
            True37False20,
            True38False19,
            True39False18,
            True40False17,
            True41False16,
            True42False15,
            True43False14,
            True44False13,
            True45False12,
            True46False11,
            True47False10,
            True48False9,
            True49False8,
            True50False7,
            True51False6,
            True52False5,
            True53False4,
            True54False3,
            True55False2,
            True56False1,
            True57False0,
            True1False57,
            True2False56,
            True3False55,
            True4False54,
            True5False53,
            True6False52,
            True7False51,
            True8False50,
            True9False49,
            True10False48,
            True11False47,
            True12False46,
            True13False45,
            True14False44,
            True15False43,
            True16False42,
            True17False41,
            True18False40,
            True19False39,
            True20False38,
            True21False37,
            True22False36,
            True23False35,
            True24False34,
            True25False33,
            True26False32,
            True27False31,
            True28False30,
            True29False29,
            True30False28,
            True31False27,
            True32False26,
            True33False25,
            True34False24,
            True35False23,
            True36False22,
            True37False21,
            True38False20,
            True39False19,
            True40False18,
            True41False17,
            True42False16,
            True43False15,
            True44False14,
            True45False13,
            True46False12,
            True47False11,
            True48False10,
            True49False9,
            True50False8,
            True51False7,
            True52False6,
            True53False5,
            True54False4,
            True55False3,
            True56False2,
            True57False1,
            True58False0,
            True1False58,
            True2False57,
            True3False56,
            True4False55,
            True5False54,
            True6False53,
            True7False52,
            True8False51,
            True9False50,
            True10False49,
            True11False48,
            True12False47,
            True13False46,
            True14False45,
            True15False44,
            True16False43,
            True17False42,
            True18False41,
            True19False40,
            True20False39,
            True21False38,
            True22False37,
            True23False36,
            True24False35,
            True25False34,
            True26False33,
            True27False32,
            True28False31,
            True29False30,
            True30False29,
            True31False28,
            True32False27,
            True33False26,
            True34False25,
            True35False24,
            True36False23,
            True37False22,
            True38False21,
            True39False20,
            True40False19,
            True41False18,
            True42False17,
            True43False16,
            True44False15,
            True45False14,
            True46False13,
            True47False12,
            True48False11,
            True49False10,
            True50False9,
            True51False8,
            True52False7,
            True53False6,
            True54False5,
            True55False4,
            True56False3,
            True57False2,
            True58False1,
            True59False0,
            True1False59,
            True2False58,
            True3False57,
            True4False56,
            True5False55,
            True6False54,
            True7False53,
            True8False52,
            True9False51,
            True10False50,
            True11False49,
            True12False48,
            True13False47,
            True14False46,
            True15False45,
            True16False44,
            True17False43,
            True18False42,
            True19False41,
            True20False40,
            True21False39,
            True22False38,
            True23False37,
            True24False36,
            True25False35,
            True26False34,
            True27False33,
            True28False32,
            True29False31,
            True30False30,
            True31False29,
            True32False28,
            True33False27,
            True34False26,
            True35False25,
            True36False24,
            True37False23,
            True38False22,
            True39False21,
            True40False20,
            True41False19,
            True42False18,
            True43False17,
            True44False16,
            True45False15,
            True46False14,
            True47False13,
            True48False12,
            True49False11,
            True50False10,
            True51False9,
            True52False8,
            True53False7,
            True54False6,
            True55False5,
            True56False4,
            True57False3,
            True58False2,
            True59False1,
            True60False0,
            True1False60,
            True2False59,
            True3False58,
            True4False57,
            True5False56,
            True6False55,
            True7False54,
            True8False53,
            True9False52,
            True10False51,
            True11False50,
            True12False49,
            True13False48,
            True14False47,
            True15False46,
            True16False45,
            True17False44,
            True18False43,
            True19False42,
            True20False41,
            True21False40,
            True22False39,
            True23False38,
            True24False37,
            True25False36,
            True26False35,
            True27False34,
            True28False33,
            True29False32,
            True30False31,
            True31False30,
            True32False29,
            True33False28,
            True34False27,
            True35False26,
            True36False25,
            True37False24,
            True38False23,
            True39False22,
            True40False21,
            True41False20,
            True42False19,
            True43False18,
            True44False17,
            True45False16,
            True46False15,
            True47False14,
            True48False13,
            True49False12,
            True50False11,
            True51False10,
            True52False9,
            True53False8,
            True54False7,
            True55False6,
            True56False5,
            True57False4,
            True58False3,
            True59False2,
            True60False1,
            True61False0,
            True1False61,
            True2False60,
            True3False59,
            True4False58,
            True5False57,
            True6False56,
            True7False55,
            True8False54,
            True9False53,
            True10False52,
            True11False51,
            True12False50,
            True13False49,
            True14False48,
            True15False47,
            True16False46,
            True17False45,
            True18False44,
            True19False43,
            True20False42,
            True21False41,
            True22False40,
            True23False39,
            True24False38,
            True25False37,
            True26False36,
            True27False35,
            True28False34,
            True29False33,
            True30False32,
            True31False31,
            True32False30,
            True33False29,
            True34False28,
            True35False27,
            True36False26,
            True37False25,
            True38False24,
            True39False23,
            True40False22,
            True41False21,
            True42False20,
            True43False19,
            True44False18,
            True45False17,
            True46False16,
            True47False15,
            True48False14,
            True49False13,
            True50False12,
            True51False11,
            True52False10,
            True53False9,
            True54False8,
            True55False7,
            True56False6,
            True57False5,
            True58False4,
            True59False3,
            True60False2,
            True61False1,
            True62False0,
            True1False62,
            True2False61,
            True3False60,
            True4False59,
            True5False58,
            True6False57,
            True7False56,
            True8False55,
            True9False54,
            True10False53,
            True11False52,
            True12False51,
            True13False50,
            True14False49,
            True15False48,
            True16False47,
            True17False46,
            True18False45,
            True19False44,
            True20False43,
            True21False42,
            True22False41,
            True23False40,
            True24False39,
            True25False38,
            True26False37,
            True27False36,
            True28False35,
            True29False34,
            True30False33,
            True31False32,
            True32False31,
            True33False30,
            True34False29,
            True35False28,
            True36False27,
            True37False26,
            True38False25,
            True39False24,
            True40False23,
            True41False22,
            True42False21,
            True43False20,
            True44False19,
            True45False18,
            True46False17,
            True47False16,
            True48False15,
            True49False14,
            True50False13,
            True51False12,
            True52False11,
            True53False10,
            True54False9,
            True55False8,
            True56False7,
            True57False6,
            True58False5,
            True59False4,
            True60False3,
            True61False2,
            True62False1,
            True63False0,
            True1False63,
            True2False62,
            True3False61,
            True4False60,
            True5False59,
            True6False58,
            True7False57,
            True8False56,
            True9False55,
            True10False54,
            True11False53,
            True12False52,
            True13False51,
            True14False50,
            True15False49,
            True16False48,
            True17False47,
            True18False46,
            True19False45,
            True20False44,
            True21False43,
            True22False42,
            True23False41,
            True24False40,
            True25False39,
            True26False38,
            True27False37,
            True28False36,
            True29False35,
            True30False34,
            True31False33,
            True32False32,
            True33False31,
            True34False30,
            True35False29,
            True36False28,
            True37False27,
            True38False26,
            True39False25,
            True40False24,
            True41False23,
            True42False22,
            True43False21,
            True44False20,
            True45False19,
            True46False18,
            True47False17,
            True48False16,
            True49False15,
            True50False14,
            True51False13,
            True52False12,
            True53False11,
            True54False10,
            True55False9,
            True56False8,
            True57False7,
            True58False6,
            True59False5,
            True60False4,
            True61False3,
            True62False2,
            True63False1,
            True64False0,
            True1False64,
            True2False63,
            True3False62,
            True4False61,
            True5False60,
            True6False59,
            True7False58,
            True8False57,
            True9False56,
            True10False55,
            True11False54,
            True12False53,
            True13False52,
            True14False51,
            True15False50,
            True16False49,
            True17False48,
            True18False47,
            True19False46,
            True20False45,
            True21False44,
            True22False43,
            True23False42,
            True24False41,
            True25False40,
            True26False39,
            True27False38,
            True28False37,
            True29False36,
            True30False35,
            True31False34,
            True32False33,
            True33False32,
            True34False31,
            True35False30,
            True36False29,
            True37False28,
            True38False27,
            True39False26,
            True40False25,
            True41False24,
            True42False23,
            True43False22,
            True44False21,
            True45False20,
            True46False19,
            True47False18,
            True48False17,
            True49False16,
            True50False15,
            True51False14,
            True52False13,
            True53False12,
            True54False11,
            True55False10,
            True56False9,
            True57False8,
            True58False7,
            True59False6,
            True60False5,
            True61False4,
            True62False3,
            True63False2,
            True64False1,
            True65False0,
            True1False65,
            True2False64,
            True3False63,
            True4False62,
            True5False61,
            True6False60,
            True7False59,
            True8False58,
            True9False57,
            True10False56,
            True11False55,
            True12False54,
            True13False53,
            True14False52,
            True15False51,
            True16False50,
            True17False49,
            True18False48,
            True19False47,
            True20False46,
            True21False45,
            True22False44,
            True23False43,
            True24False42,
            True25False41,
            True26False40,
            True27False39,
            True28False38,
            True29False37,
            True30False36,
            True31False35,
            True32False34,
            True33False33,
            True34False32,
            True35False31,
            True36False30,
            True37False29,
            True38False28,
            True39False27,
            True40False26,
            True41False25,
            True42False24,
            True43False23,
            True44False22,
            True45False21,
            True46False20,
            True47False19,
            True48False18,
            True49False17,
            True50False16,
            True51False15,
            True52False14,
            True53False13,
            True54False12,
            True55False11,
            True56False10,
            True57False9,
            True58False8,
            True59False7,
            True60False6,
            True61False5,
            True62False4,
            True63False3,
            True64False2,
            True65False1,
            True66False0,
            True1False66,
            True2False65,
            True3False64,
            True4False63,
            True5False62,
            True6False61,
            True7False60,
            True8False59,
            True9False58,
            True10False57,
            True11False56,
            True12False55,
            True13False54,
            True14False53,
            True15False52,
            True16False51,
            True17False50,
            True18False49,
            True19False48,
            True20False47,
            True21False46,
            True22False45,
            True23False44,
            True24False43,
            True25False42,
            True26False41,
            True27False40,
            True28False39,
            True29False38,
            True30False37,
            True31False36,
            True32False35,
            True33False34,
            True34False33,
            True35False32,
            True36False31,
            True37False30,
            True38False29,
            True39False28,
            True40False27,
            True41False26,
            True42False25,
            True43False24,
            True44False23,
            True45False22,
            True46False21,
            True47False20,
            True48False19,
            True49False18,
            True50False17,
            True51False16,
            True52False15,
            True53False14,
            True54False13,
            True55False12,
            True56False11,
            True57False10,
            True58False9,
            True59False8,
            True60False7,
            True61False6,
            True62False5,
            True63False4,
            True64False3,
            True65False2,
            True66False1,
            True67False0,
            True1False67,
            True2False66,
            True3False65,
            True4False64,
            True5False63,
            True6False62,
            True7False61,
            True8False60,
            True9False59,
            True10False58,
            True11False57,
            True12False56,
            True13False55,
            True14False54,
            True15False53,
            True16False52,
            True17False51,
            True18False50,
            True19False49,
            True20False48,
            True21False47,
            True22False46,
            True23False45,
            True24False44,
            True25False43,
            True26False42,
            True27False41,
            True28False40,
            True29False39,
            True30False38,
            True31False37,
            True32False36,
            True33False35,
            True34False34,
            True35False33,
            True36False32,
            True37False31,
            True38False30,
            True39False29,
            True40False28,
            True41False27,
            True42False26,
            True43False25,
            True44False24,
            True45False23,
            True46False22,
            True47False21,
            True48False20,
            True49False19,
            True50False18,
            True51False17,
            True52False16,
            True53False15,
            True54False14,
            True55False13,
            True56False12,
            True57False11,
            True58False10,
            True59False9,
            True60False8,
            True61False7,
            True62False6,
            True63False5,
            True64False4,
            True65False3,
            True66False2,
            True67False1,
            True68False0,
            True1False68,
            True2False67,
            True3False66,
            True4False65,
            True5False64,
            True6False63,
            True7False62,
            True8False61,
            True9False60,
            True10False59,
            True11False58,
            True12False57,
            True13False56,
            True14False55,
            True15False54,
            True16False53,
            True17False52,
            True18False51,
            True19False50,
            True20False49,
            True21False48,
            True22False47,
            True23False46,
            True24False45,
            True25False44,
            True26False43,
            True27False42,
            True28False41,
            True29False40,
            True30False39,
            True31False38,
            True32False37,
            True33False36,
            True34False35,
            True35False34,
            True36False33,
            True37False32,
            True38False31,
            True39False30,
            True40False29,
            True41False28,
            True42False27,
            True43False26,
            True44False25,
            True45False24,
            True46False23,
            True47False22,
            True48False21,
            True49False20,
            True50False19,
            True51False18,
            True52False17,
            True53False16,
            True54False15,
            True55False14,
            True56False13,
            True57False12,
            True58False11,
            True59False10,
            True60False9,
            True61False8,
            True62False7,
            True63False6,
            True64False5,
            True65False4,
            True66False3,
            True67False2,
            True68False1,
            True69False0,
            True1False69,
            True2False68,
            True3False67,
            True4False66,
            True5False65,
            True6False64,
            True7False63,
            True8False62,
            True9False61,
            True10False60,
            True11False59,
            True12False58,
            True13False57,
            True14False56,
            True15False55,
            True16False54,
            True17False53,
            True18False52,
            True19False51,
            True20False50,
            True21False49,
            True22False48,
            True23False47,
            True24False46,
            True25False45,
            True26False44,
            True27False43,
            True28False42,
            True29False41,
            True30False40,
            True31False39,
            True32False38,
            True33False37,
            True34False36,
            True35False35,
            True36False34,
            True37False33,
            True38False32,
            True39False31,
            True40False30,
            True41False29,
            True42False28,
            True43False27,
            True44False26,
            True45False25,
            True46False24,
            True47False23,
            True48False22,
            True49False21,
            True50False20,
            True51False19,
            True52False18,
            True53False17,
            True54False16,
            True55False15,
            True56False14,
            True57False13,
            True58False12,
            True59False11,
            True60False10,
            True61False9,
            True62False8,
            True63False7,
            True64False6,
            True65False5,
            True66False4,
            True67False3,
            True68False2,
            True69False1,
            True70False0,
            True1False70,
            True2False69,
            True3False68,
            True4False67,
            True5False66,
            True6False65,
            True7False64,
            True8False63,
            True9False62,
            True10False61,
            True11False60,
            True12False59,
            True13False58,
            True14False57,
            True15False56,
            True16False55,
            True17False54,
            True18False53,
            True19False52,
            True20False51,
            True21False50,
            True22False49,
            True23False48,
            True24False47,
            True25False46,
            True26False45,
            True27False44,
            True28False43,
            True29False42,
            True30False41,
            True31False40,
            True32False39,
            True33False38,
            True34False37,
            True35False36,
            True36False35,
            True37False34,
            True38False33,
            True39False32,
            True40False31,
            True41False30,
            True42False29,
            True43False28,
            True44False27,
            True45False26,
            True46False25,
            True47False24,
            True48False23,
            True49False22,
            True50False21,
            True51False20,
            True52False19,
            True53False18,
            True54False17,
            True55False16,
            True56False15,
            True57False14,
            True58False13,
            True59False12,
            True60False11,
            True61False10,
            True62False9,
            True63False8,
            True64False7,
            True65False6,
            True66False5,
            True67False4,
            True68False3,
            True69False2,
            True70False1,
            True71False0,
            True1False71,
            True2False70,
            True3False69,
            True4False68,
            True5False67,
            True6False66,
            True7False65,
            True8False64,
            True9False63,
            True10False62,
            True11False61,
            True12False60,
            True13False59,
            True14False58,
            True15False57,
            True16False56,
            True17False55,
            True18False54,
            True19False53,
            True20False52,
            True21False51,
            True22False50,
            True23False49,
            True24False48,
            True25False47,
            True26False46,
            True27False45,
            True28False44,
            True29False43,
            True30False42,
            True31False41,
            True32False40,
            True33False39,
            True34False38,
            True35False37,
            True36False36,
            True37False35,
            True38False34,
            True39False33,
            True40False32,
            True41False31,
            True42False30,
            True43False29,
            True44False28,
            True45False27,
            True46False26,
            True47False25,
            True48False24,
            True49False23,
            True50False22,
            True51False21,
            True52False20,
            True53False19,
            True54False18,
            True55False17,
            True56False16,
            True57False15,
            True58False14,
            True59False13,
            True60False12,
            True61False11,
            True62False10,
            True63False9,
            True64False8,
            True65False7,
            True66False6,
            True67False5,
            True68False4,
            True69False3,
            True70False2,
            True71False1,
            True72False0,
            True1False72,
            True2False71,
            True3False70,
            True4False69,
            True5False68,
            True6False67,
            True7False66,
            True8False65,
            True9False64,
            True10False63,
            True11False62,
            True12False61,
            True13False60,
            True14False59,
            True15False58,
            True16False57,
            True17False56,
            True18False55,
            True19False54,
            True20False53,
            True21False52,
            True22False51,
            True23False50,
            True24False49,
            True25False48,
            True26False47,
            True27False46,
            True28False45,
            True29False44,
            True30False43,
            True31False42,
            True32False41,
            True33False40,
            True34False39,
            True35False38,
            True36False37,
            True37False36,
            True38False35,
            True39False34,
            True40False33,
            True41False32,
            True42False31,
            True43False30,
            True44False29,
            True45False28,
            True46False27,
            True47False26,
            True48False25,
            True49False24,
            True50False23,
            True51False22,
            True52False21,
            True53False20,
            True54False19,
            True55False18,
            True56False17,
            True57False16,
            True58False15,
            True59False14,
            True60False13,
            True61False12,
            True62False11,
            True63False10,
            True64False9,
            True65False8,
            True66False7,
            True67False6,
            True68False5,
            True69False4,
            True70False3,
            True71False2,
            True72False1,
            True73False0,
            True1False73,
            True2False72,
            True3False71,
            True4False70,
            True5False69,
            True6False68,
            True7False67,
            True8False66,
            True9False65,
            True10False64,
            True11False63,
            True12False62,
            True13False61,
            True14False60,
            True15False59,
            True16False58,
            True17False57,
            True18False56,
            True19False55,
            True20False54,
            True21False53,
            True22False52,
            True23False51,
            True24False50,
            True25False49,
            True26False48,
            True27False47,
            True28False46,
            True29False45,
            True30False44,
            True31False43,
            True32False42,
            True33False41,
            True34False40,
            True35False39,
            True36False38,
            True37False37,
            True38False36,
            True39False35,
            True40False34,
            True41False33,
            True42False32,
            True43False31,
            True44False30,
            True45False29,
            True46False28,
            True47False27,
            True48False26,
            True49False25,
            True50False24,
            True51False23,
            True52False22,
            True53False21,
            True54False20,
            True55False19,
            True56False18,
            True57False17,
            True58False16,
            True59False15,
            True60False14,
            True61False13,
            True62False12,
            True63False11,
            True64False10,
            True65False9,
            True66False8,
            True67False7,
            True68False6,
            True69False5,
            True70False4,
            True71False3,
            True72False2,
            True73False1,
            True74False0,
            True1False74,
            True2False73,
            True3False72,
            True4False71,
            True5False70,
            True6False69,
            True7False68,
            True8False67,
            True9False66,
            True10False65,
            True11False64,
            True12False63,
            True13False62,
            True14False61,
            True15False60,
            True16False59,
            True17False58,
            True18False57,
            True19False56,
            True20False55,
            True21False54,
            True22False53,
            True23False52,
            True24False51,
            True25False50,
            True26False49,
            True27False48,
            True28False47,
            True29False46,
            True30False45,
            True31False44,
            True32False43,
            True33False42,
            True34False41,
            True35False40,
            True36False39,
            True37False38,
            True38False37,
            True39False36,
            True40False35,
            True41False34,
            True42False33,
            True43False32,
            True44False31,
            True45False30,
            True46False29,
            True47False28,
            True48False27,
            True49False26,
            True50False25,
            True51False24,
            True52False23,
            True53False22,
            True54False21,
            True55False20,
            True56False19,
            True57False18,
            True58False17,
            True59False16,
            True60False15,
            True61False14,
            True62False13,
            True63False12,
            True64False11,
            True65False10,
            True66False9,
            True67False8,
            True68False7,
            True69False6,
            True70False5,
            True71False4,
            True72False3,
            True73False2,
            True74False1,
            True75False0,
            True1False75,
            True2False74,
            True3False73,
            True4False72,
            True5False71,
            True6False70,
            True7False69,
            True8False68,
            True9False67,
            True10False66,
            True11False65,
            True12False64,
            True13False63,
            True14False62,
            True15False61,
            True16False60,
            True17False59,
            True18False58,
            True19False57,
            True20False56,
            True21False55,
            True22False54,
            True23False53,
            True24False52,
            True25False51,
            True26False50,
            True27False49,
            True28False48,
            True29False47,
            True30False46,
            True31False45,
            True32False44,
            True33False43,
            True34False42,
            True35False41,
            True36False40,
            True37False39,
            True38False38,
            True39False37,
            True40False36,
            True41False35,
            True42False34,
            True43False33,
            True44False32,
            True45False31,
            True46False30,
            True47False29,
            True48False28,
            True49False27,
            True50False26,
            True51False25,
            True52False24,
            True53False23,
            True54False22,
            True55False21,
            True56False20,
            True57False19,
            True58False18,
            True59False17,
            True60False16,
            True61False15,
            True62False14,
            True63False13,
            True64False12,
            True65False11,
            True66False10,
            True67False9,
            True68False8,
            True69False7,
            True70False6,
            True71False5,
            True72False4,
            True73False3,
            True74False2,
            True75False1,
            True76False0,
            True1False76,
            True2False75,
            True3False74,
            True4False73,
            True5False72,
            True6False71,
            True7False70,
            True8False69,
            True9False68,
            True10False67,
            True11False66,
            True12False65,
            True13False64,
            True14False63,
            True15False62,
            True16False61,
            True17False60,
            True18False59,
            True19False58,
            True20False57,
            True21False56,
            True22False55,
            True23False54,
            True24False53,
            True25False52,
            True26False51,
            True27False50,
            True28False49,
            True29False48,
            True30False47,
            True31False46,
            True32False45,
            True33False44,
            True34False43,
            True35False42,
            True36False41,
            True37False40,
            True38False39,
            True39False38,
            True40False37,
            True41False36,
            True42False35,
            True43False34,
            True44False33,
            True45False32,
            True46False31,
            True47False30,
            True48False29,
            True49False28,
            True50False27,
            True51False26,
            True52False25,
            True53False24,
            True54False23,
            True55False22,
            True56False21,
            True57False20,
            True58False19,
            True59False18,
            True60False17,
            True61False16,
            True62False15,
            True63False14,
            True64False13,
            True65False12,
            True66False11,
            True67False10,
            True68False9,
            True69False8,
            True70False7,
            True71False6,
            True72False5,
            True73False4,
            True74False3,
            True75False2,
            True76False1,
            True77False0,
            True1False77,
            True2False76,
            True3False75,
            True4False74,
            True5False73,
            True6False72,
            True7False71,
            True8False70,
            True9False69,
            True10False68,
            True11False67,
            True12False66,
            True13False65,
            True14False64,
            True15False63,
            True16False62,
            True17False61,
            True18False60,
            True19False59,
            True20False58,
            True21False57,
            True22False56,
            True23False55,
            True24False54,
            True25False53,
            True26False52,
            True27False51,
            True28False50,
            True29False49,
            True30False48,
            True31False47,
            True32False46,
            True33False45,
            True34False44,
            True35False43,
            True36False42,
            True37False41,
            True38False40,
            True39False39,
            True40False38,
            True41False37,
            True42False36,
            True43False35,
            True44False34,
            True45False33,
            True46False32,
            True47False31,
            True48False30,
            True49False29,
            True50False28,
            True51False27,
            True52False26,
            True53False25,
            True54False24,
            True55False23,
            True56False22,
            True57False21,
            True58False20,
            True59False19,
            True60False18,
            True61False17,
            True62False16,
            True63False15,
            True64False14,
            True65False13,
            True66False12,
            True67False11,
            True68False10,
            True69False9,
            True70False8,
            True71False7,
            True72False6,
            True73False5,
            True74False4,
            True75False3,
            True76False2,
            True77False1,
            True78False0,
            True1False78,
            True2False77,
            True3False76,
            True4False75,
            True5False74,
            True6False73,
            True7False72,
            True8False71,
            True9False70,
            True10False69,
            True11False68,
            True12False67,
            True13False66,
            True14False65,
            True15False64,
            True16False63,
            True17False62,
            True18False61,
            True19False60,
            True20False59,
            True21False58,
            True22False57,
            True23False56,
            True24False55,
            True25False54,
            True26False53,
            True27False52,
            True28False51,
            True29False50,
            True30False49,
            True31False48,
            True32False47,
            True33False46,
            True34False45,
            True35False44,
            True36False43,
            True37False42,
            True38False41,
            True39False40,
            True40False39,
            True41False38,
            True42False37,
            True43False36,
            True44False35,
            True45False34,
            True46False33,
            True47False32,
            True48False31,
            True49False30,
            True50False29,
            True51False28,
            True52False27,
            True53False26,
            True54False25,
            True55False24,
            True56False23,
            True57False22,
            True58False21,
            True59False20,
            True60False19,
            True61False18,
            True62False17,
            True63False16,
            True64False15,
            True65False14,
            True66False13,
            True67False12,
            True68False11,
            True69False10,
            True70False9,
            True71False8,
            True72False7,
            True73False6,
            True74False5,
            True75False4,
            True76False3,
            True77False2,
            True78False1,
            True79False0,
            True1False79,
            True2False78,
            True3False77,
            True4False76,
            True5False75,
            True6False74,
            True7False73,
            True8False72,
            True9False71,
            True10False70,
            True11False69,
            True12False68,
            True13False67,
            True14False66,
            True15False65,
            True16False64,
            True17False63,
            True18False62,
            True19False61,
            True20False60,
            True21False59,
            True22False58,
            True23False57,
            True24False56,
            True25False55,
            True26False54,
            True27False53,
            True28False52,
            True29False51,
            True30False50,
            True31False49,
            True32False48,
            True33False47,
            True34False46,
            True35False45,
            True36False44,
            True37False43,
            True38False42,
            True39False41,
            True40False40,
            True41False39,
            True42False38,
            True43False37,
            True44False36,
            True45False35,
            True46False34,
            True47False33,
            True48False32,
            True49False31,
            True50False30,
            True51False29,
            True52False28,
            True53False27,
            True54False26,
            True55False25,
            True56False24,
            True57False23,
            True58False22,
            True59False21,
            True60False20,
            True61False19,
            True62False18,
            True63False17,
            True64False16,
            True65False15,
            True66False14,
            True67False13,
            True68False12,
            True69False11,
            True70False10,
            True71False9,
            True72False8,
            True73False7,
            True74False6,
            True75False5,
            True76False4,
            True77False3,
            True78False2,
            True79False1,
            True80False0,
            True0False40,
            True1False39,
            True1False39,
            True2False38,
            True2False38,
            True3False37,
            True3False37,
            True4False36,
            True4False36,
            True5False35,
            True5False35,
            True6False34,
            True6False34,
            True7False33,
            True7False33,
            True8False32,
            True8False32,
            True9False31,
            True9False31,
            True10False30,
            True10False30,
            True11False29,
            True11False29,
            True12False28,
            True12False28,
            True13False27,
            True13False27,
            True14False26,
            True14False26,
            True15False25,
            True15False25,
            True16False24,
            True16False24,
            True17False23,
            True17False23,
            True18False22,
            True18False22,
            True19False21,
            True19False21,
            True20False20,
            True20False20,
            True21False19,
            True21False19,
            True22False18,
            True22False18,
            True23False17,
            True23False17,
            True24False16,
            True24False16,
            True25False15,
            True25False15,
            True26False14,
            True26False14,
            True27False13,
            True27False13,
            True28False12,
            True28False12,
            True29False11,
            True29False11,
            True30False10,
            True30False10,
            True31False9,
            True31False9,
            True32False8,
            True32False8,
            True33False7,
            True33False7,
            True34False6,
            True34False6,
            True35False5,
            True35False5,
            True36False4,
            True36False4,
            True37False3,
            True37False3,
            True38False2,
            True38False2,
            True39False1,
            True39False1,
            True40False0,
            True80False0,
        ];
        OUTCOMES[(self as usize) + (bit as usize) * 3321]
    }
}
// Count of variants: 3321
