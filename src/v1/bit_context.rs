//! Generated with `src/v1/bit-context.sh`
use super::arith::Probability;

impl BitContext {
    pub const CONFIDENT: Self = True0False4;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitContext {
    #[default]
    True0False0, // 0.5
    True1False0,   // 0.33203125
    True2False0,   // 0.25
    True3False0,   // 0.19921875
    True4False0,   // 0.1640625
    True5False0,   // 0.140625
    True6False0,   // 0.125
    True7False0,   // 0.109375
    True8False0,   // 0.09765625
    True9False0,   // 0.08984375
    True10False0,  // 0.08203125
    True11False0,  // 0.07421875
    True12False0,  // 0.0703125
    True13False0,  // 0.06640625
    True14False0,  // 0.0625
    True15False0,  // 0.05859375
    True16False0,  // 0.0546875
    True17False0,  // 0.05078125
    True18False0,  // 0.046875
    True19False0,  // 0.046875
    True20False0,  // 0.04296875
    True21False0,  // 0.04296875
    True22False0,  // 0.0390625
    True23False0,  // 0.0390625
    True24False0,  // 0.03515625
    True25False0,  // 0.03515625
    True26False0,  // 0.03515625
    True27False0,  // 0.03125
    True28False0,  // 0.03125
    True29False0,  // 0.03125
    True30False0,  // 0.03125
    True31False0,  // 0.02734375
    True32False0,  // 0.02734375
    True33False0,  // 0.02734375
    True34False0,  // 0.02734375
    True35False0,  // 0.0234375
    True36False0,  // 0.0234375
    True37False0,  // 0.0234375
    True38False0,  // 0.0234375
    True39False0,  // 0.0234375
    True40False0,  // 0.0234375
    True41False0,  // 0.01953125
    True42False0,  // 0.01953125
    True43False0,  // 0.01953125
    True44False0,  // 0.01953125
    True45False0,  // 0.01953125
    True46False0,  // 0.01953125
    True47False0,  // 0.01953125
    True48False0,  // 0.01953125
    True49False0,  // 0.01953125
    True50False0,  // 0.015625
    True51False0,  // 0.015625
    True52False0,  // 0.015625
    True53False0,  // 0.015625
    True54False0,  // 0.015625
    True55False0,  // 0.015625
    True56False0,  // 0.015625
    True57False0,  // 0.015625
    True58False0,  // 0.015625
    True59False0,  // 0.015625
    True60False0,  // 0.015625
    True61False0,  // 0.015625
    True62False0,  // 0.015625
    True63False0,  // 0.01171875
    True64False0,  // 0.01171875
    True65False0,  // 0.01171875
    True66False0,  // 0.01171875
    True67False0,  // 0.01171875
    True68False0,  // 0.01171875
    True69False0,  // 0.01171875
    True70False0,  // 0.01171875
    True71False0,  // 0.01171875
    True72False0,  // 0.01171875
    True73False0,  // 0.01171875
    True74False0,  // 0.01171875
    True75False0,  // 0.01171875
    True76False0,  // 0.01171875
    True77False0,  // 0.01171875
    True78False0,  // 0.01171875
    True79False0,  // 0.01171875
    True80False0,  // 0.01171875
    True81False0,  // 0.01171875
    True82False0,  // 0.01171875
    True83False0,  // 0.01171875
    True84False0,  // 0.01171875
    True85False0,  // 0.01171875
    True86False0,  // 0.0078125
    True87False0,  // 0.0078125
    True88False0,  // 0.0078125
    True89False0,  // 0.0078125
    True90False0,  // 0.0078125
    True91False0,  // 0.0078125
    True92False0,  // 0.0078125
    True93False0,  // 0.0078125
    True94False0,  // 0.0078125
    True95False0,  // 0.0078125
    True96False0,  // 0.0078125
    True97False0,  // 0.0078125
    True98False0,  // 0.0078125
    True99False0,  // 0.0078125
    True100False0, // 0.0078125
    True101False0, // 0.0078125
    True102False0, // 0.0078125
    True103False0, // 0.0078125
    True104False0, // 0.0078125
    True105False0, // 0.0078125
    True106False0, // 0.0078125
    True107False0, // 0.0078125
    True108False0, // 0.0078125
    True109False0, // 0.0078125
    True110False0, // 0.0078125
    True111False0, // 0.0078125
    True112False0, // 0.0078125
    True113False0, // 0.0078125
    True114False0, // 0.0078125
    True115False0, // 0.0078125
    True116False0, // 0.0078125
    True117False0, // 0.0078125
    True118False0, // 0.0078125
    True119False0, // 0.0078125
    True120False0, // 0.0078125
    True121False0, // 0.0078125
    True122False0, // 0.0078125
    True123False0, // 0.0078125
    True124False0, // 0.0078125
    True125False0, // 0.0078125
    True126False0, // 0.0078125
    True127False0, // 0.0078125
    True128False0, // 0.0078125
    True129False0, // 0.0078125
    True130False0, // 0.0078125
    True131False0, // 0.0078125
    True132False0, // 0.0078125
    True0False1,   // 0.66796875
    True1False1,   // 0.5
    True2False1,   // 0.3984375
    True3False1,   // 0.33203125
    True4False1,   // 0.28515625
    True5False1,   // 0.25
    True6False1,   // 0.22265625
    True7False1,   // 0.19921875
    True8False1,   // 0.18359375
    True9False1,   // 0.16796875
    True10False1,  // 0.15234375
    True11False1,  // 0.14453125
    True12False1,  // 0.1328125
    True13False1,  // 0.125
    True14False1,  // 0.1171875
    True15False1,  // 0.109375
    True16False1,  // 0.10546875
    True17False1,  // 0.1015625
    True18False1,  // 0.09375
    True19False1,  // 0.08984375
    True20False1,  // 0.0859375
    True21False1,  // 0.08203125
    True22False1,  // 0.078125
    True23False1,  // 0.078125
    True24False1,  // 0.07421875
    True25False1,  // 0.0703125
    True26False1,  // 0.0703125
    True27False1,  // 0.06640625
    True28False1,  // 0.06640625
    True29False1,  // 0.0625
    True30False1,  // 0.0625
    True31False1,  // 0.05859375
    True32False1,  // 0.05859375
    True33False1,  // 0.0546875
    True34False1,  // 0.0546875
    True35False1,  // 0.0546875
    True36False1,  // 0.05078125
    True37False1,  // 0.05078125
    True38False1,  // 0.05078125
    True39False1,  // 0.046875
    True40False1,  // 0.046875
    True41False1,  // 0.046875
    True42False1,  // 0.04296875
    True43False1,  // 0.04296875
    True44False1,  // 0.04296875
    True45False1,  // 0.04296875
    True46False1,  // 0.0390625
    True47False1,  // 0.0390625
    True48False1,  // 0.0390625
    True49False1,  // 0.0390625
    True50False1,  // 0.0390625
    True51False1,  // 0.0390625
    True52False1,  // 0.03515625
    True53False1,  // 0.03515625
    True54False1,  // 0.03515625
    True55False1,  // 0.03515625
    True56False1,  // 0.03515625
    True57False1,  // 0.03515625
    True58False1,  // 0.03125
    True59False1,  // 0.03125
    True60False1,  // 0.03125
    True61False1,  // 0.03125
    True62False1,  // 0.03125
    True63False1,  // 0.03125
    True64False1,  // 0.03125
    True65False1,  // 0.03125
    True0False2,   // 0.75
    True1False2,   // 0.6015625
    True2False2,   // 0.5
    True3False2,   // 0.4296875
    True4False2,   // 0.375
    True5False2,   // 0.33203125
    True6False2,   // 0.30078125
    True7False2,   // 0.2734375
    True8False2,   // 0.25
    True9False2,   // 0.23046875
    True10False2,  // 0.21484375
    True11False2,  // 0.19921875
    True12False2,  // 0.1875
    True13False2,  // 0.17578125
    True14False2,  // 0.16796875
    True15False2,  // 0.15625
    True16False2,  // 0.1484375
    True17False2,  // 0.14453125
    True18False2,  // 0.13671875
    True19False2,  // 0.12890625
    True20False2,  // 0.125
    True21False2,  // 0.12109375
    True22False2,  // 0.1171875
    True23False2,  // 0.109375
    True24False2,  // 0.10546875
    True25False2,  // 0.1015625
    True26False2,  // 0.1015625
    True27False2,  // 0.09765625
    True28False2,  // 0.09375
    True29False2,  // 0.08984375
    True30False2,  // 0.08984375
    True31False2,  // 0.0859375
    True32False2,  // 0.08203125
    True33False2,  // 0.08203125
    True34False2,  // 0.078125
    True35False2,  // 0.078125
    True36False2,  // 0.07421875
    True37False2,  // 0.07421875
    True38False2,  // 0.0703125
    True39False2,  // 0.0703125
    True40False2,  // 0.06640625
    True41False2,  // 0.06640625
    True42False2,  // 0.06640625
    True43False2,  // 0.0625
    True0False3,   // 0.80078125
    True1False3,   // 0.66796875
    True2False3,   // 0.5703125
    True3False3,   // 0.5
    True4False3,   // 0.4453125
    True5False3,   // 0.3984375
    True6False3,   // 0.36328125
    True7False3,   // 0.33203125
    True8False3,   // 0.30859375
    True9False3,   // 0.28515625
    True10False3,  // 0.265625
    True11False3,  // 0.25
    True12False3,  // 0.234375
    True13False3,  // 0.22265625
    True14False3,  // 0.2109375
    True15False3,  // 0.19921875
    True16False3,  // 0.19140625
    True17False3,  // 0.18359375
    True18False3,  // 0.17578125
    True19False3,  // 0.16796875
    True20False3,  // 0.16015625
    True21False3,  // 0.15234375
    True22False3,  // 0.1484375
    True23False3,  // 0.14453125
    True24False3,  // 0.13671875
    True25False3,  // 0.1328125
    True26False3,  // 0.12890625
    True27False3,  // 0.125
    True28False3,  // 0.12109375
    True29False3,  // 0.1171875
    True30False3,  // 0.11328125
    True31False3,  // 0.109375
    True32False3,  // 0.109375
    True0False4,   // 0.8359375
    True1False4,   // 0.71484375
    True2False4,   // 0.625
    True3False4,   // 0.5546875
    True4False4,   // 0.5
    True5False4,   // 0.453125
    True6False4,   // 0.41796875
    True7False4,   // 0.3828125
    True8False4,   // 0.35546875
    True9False4,   // 0.33203125
    True10False4,  // 0.3125
    True11False4,  // 0.29296875
    True12False4,  // 0.27734375
    True13False4,  // 0.26171875
    True14False4,  // 0.25
    True15False4,  // 0.23828125
    True16False4,  // 0.2265625
    True17False4,  // 0.21875
    True18False4,  // 0.20703125
    True19False4,  // 0.19921875
    True20False4,  // 0.19140625
    True21False4,  // 0.18359375
    True22False4,  // 0.1796875
    True23False4,  // 0.171875
    True24False4,  // 0.16796875
    True25False4,  // 0.16015625
    True0False5,   // 0.859375
    True1False5,   // 0.75
    True2False5,   // 0.66796875
    True3False5,   // 0.6015625
    True4False5,   // 0.546875
    True5False5,   // 0.5
    True6False5,   // 0.4609375
    True7False5,   // 0.4296875
    True8False5,   // 0.3984375
    True9False5,   // 0.375
    True10False5,  // 0.3515625
    True11False5,  // 0.33203125
    True12False5,  // 0.31640625
    True13False5,  // 0.30078125
    True14False5,  // 0.28515625
    True15False5,  // 0.2734375
    True16False5,  // 0.26171875
    True17False5,  // 0.25
    True18False5,  // 0.23828125
    True19False5,  // 0.23046875
    True20False5,  // 0.22265625
    True21False5,  // 0.21484375
    True0False6,   // 0.875
    True1False6,   // 0.77734375
    True2False6,   // 0.69921875
    True3False6,   // 0.63671875
    True4False6,   // 0.58203125
    True5False6,   // 0.5390625
    True6False6,   // 0.5
    True7False6,   // 0.46484375
    True8False6,   // 0.4375
    True9False6,   // 0.41015625
    True10False6,  // 0.390625
    True11False6,  // 0.3671875
    True12False6,  // 0.3515625
    True13False6,  // 0.33203125
    True14False6,  // 0.31640625
    True15False6,  // 0.3046875
    True16False6,  // 0.29296875
    True17False6,  // 0.28125
    True18False6,  // 0.26953125
    True0False7,   // 0.890625
    True1False7,   // 0.80078125
    True2False7,   // 0.7265625
    True3False7,   // 0.66796875
    True4False7,   // 0.6171875
    True5False7,   // 0.5703125
    True6False7,   // 0.53515625
    True7False7,   // 0.5
    True8False7,   // 0.46875
    True9False7,   // 0.4453125
    True10False7,  // 0.421875
    True11False7,  // 0.3984375
    True12False7,  // 0.3828125
    True13False7,  // 0.36328125
    True14False7,  // 0.34765625
    True15False7,  // 0.33203125
    True0False8,   // 0.90234375
    True1False8,   // 0.81640625
    True2False8,   // 0.75
    True3False8,   // 0.69140625
    True4False8,   // 0.64453125
    True5False8,   // 0.6015625
    True6False8,   // 0.5625
    True7False8,   // 0.53125
    True8False8,   // 0.5
    True9False8,   // 0.47265625
    True10False8,  // 0.44921875
    True11False8,  // 0.4296875
    True12False8,  // 0.41015625
    True13False8,  // 0.390625
    True0False9,   // 0.91015625
    True1False9,   // 0.83203125
    True2False9,   // 0.76953125
    True3False9,   // 0.71484375
    True4False9,   // 0.66796875
    True5False9,   // 0.625
    True6False9,   // 0.58984375
    True7False9,   // 0.5546875
    True8False9,   // 0.52734375
    True9False9,   // 0.5
    True10False9,  // 0.4765625
    True11False9,  // 0.453125
    True12False9,  // 0.43359375
    True0False10,  // 0.91796875
    True1False10,  // 0.84765625
    True2False10,  // 0.78515625
    True3False10,  // 0.734375
    True4False10,  // 0.6875
    True5False10,  // 0.6484375
    True6False10,  // 0.609375
    True7False10,  // 0.578125
    True8False10,  // 0.55078125
    True9False10,  // 0.5234375
    True10False10, // 0.5
    True11False10, // 0.4765625
    True0False11,  // 0.92578125
    True1False11,  // 0.85546875
    True2False11,  // 0.80078125
    True3False11,  // 0.75
    True4False11,  // 0.70703125
    True5False11,  // 0.66796875
    True6False11,  // 0.6328125
    True7False11,  // 0.6015625
    True8False11,  // 0.5703125
    True9False11,  // 0.546875
    True10False11, // 0.5234375
    True0False12,  // 0.9296875
    True1False12,  // 0.8671875
    True2False12,  // 0.8125
    True3False12,  // 0.765625
    True4False12,  // 0.72265625
    True5False12,  // 0.68359375
    True6False12,  // 0.6484375
    True7False12,  // 0.6171875
    True8False12,  // 0.58984375
    True9False12,  // 0.56640625
    True0False13,  // 0.93359375
    True1False13,  // 0.875
    True2False13,  // 0.82421875
    True3False13,  // 0.77734375
    True4False13,  // 0.73828125
    True5False13,  // 0.69921875
    True6False13,  // 0.66796875
    True7False13,  // 0.63671875
    True8False13,  // 0.609375
    True0False14,  // 0.9375
    True1False14,  // 0.8828125
    True2False14,  // 0.83203125
    True3False14,  // 0.7890625
    True4False14,  // 0.75
    True5False14,  // 0.71484375
    True6False14,  // 0.68359375
    True7False14,  // 0.65234375
    True0False15,  // 0.94140625
    True1False15,  // 0.890625
    True2False15,  // 0.84375
    True3False15,  // 0.80078125
    True4False15,  // 0.76171875
    True5False15,  // 0.7265625
    True6False15,  // 0.6953125
    True7False15,  // 0.66796875
    True0False16,  // 0.9453125
    True1False16,  // 0.89453125
    True2False16,  // 0.8515625
    True3False16,  // 0.80859375
    True4False16,  // 0.7734375
    True5False16,  // 0.73828125
    True6False16,  // 0.70703125
    True0False17,  // 0.94921875
    True1False17,  // 0.8984375
    True2False17,  // 0.85546875
    True3False17,  // 0.81640625
    True4False17,  // 0.78125
    True5False17,  // 0.75
    True6False17,  // 0.71875
    True0False18,  // 0.953125
    True1False18,  // 0.90625
    True2False18,  // 0.86328125
    True3False18,  // 0.82421875
    True4False18,  // 0.79296875
    True5False18,  // 0.76171875
    True6False18,  // 0.73046875
    True0False19,  // 0.953125
    True1False19,  // 0.91015625
    True2False19,  // 0.87109375
    True3False19,  // 0.83203125
    True4False19,  // 0.80078125
    True5False19,  // 0.76953125
    True0False20,  // 0.95703125
    True1False20,  // 0.9140625
    True2False20,  // 0.875
    True3False20,  // 0.83984375
    True4False20,  // 0.80859375
    True5False20,  // 0.77734375
    True0False21,  // 0.95703125
    True1False21,  // 0.91796875
    True2False21,  // 0.87890625
    True3False21,  // 0.84765625
    True4False21,  // 0.81640625
    True5False21,  // 0.78515625
    True0False22,  // 0.9609375
    True1False22,  // 0.921875
    True2False22,  // 0.8828125
    True3False22,  // 0.8515625
    True4False22,  // 0.8203125
    True0False23,  // 0.9609375
    True1False23,  // 0.921875
    True2False23,  // 0.890625
    True3False23,  // 0.85546875
    True4False23,  // 0.828125
    True0False24,  // 0.96484375
    True1False24,  // 0.92578125
    True2False24,  // 0.89453125
    True3False24,  // 0.86328125
    True4False24,  // 0.83203125
    True0False25,  // 0.96484375
    True1False25,  // 0.9296875
    True2False25,  // 0.8984375
    True3False25,  // 0.8671875
    True4False25,  // 0.83984375
    True0False26,  // 0.96484375
    True1False26,  // 0.9296875
    True2False26,  // 0.8984375
    True3False26,  // 0.87109375
    True0False27,  // 0.96875
    True1False27,  // 0.93359375
    True2False27,  // 0.90234375
    True3False27,  // 0.875
    True0False28,  // 0.96875
    True1False28,  // 0.93359375
    True2False28,  // 0.90625
    True3False28,  // 0.87890625
    True0False29,  // 0.96875
    True1False29,  // 0.9375
    True2False29,  // 0.91015625
    True3False29,  // 0.8828125
    True0False30,  // 0.96875
    True1False30,  // 0.9375
    True2False30,  // 0.91015625
    True3False30,  // 0.88671875
    True0False31,  // 0.97265625
    True1False31,  // 0.94140625
    True2False31,  // 0.9140625
    True3False31,  // 0.890625
    True0False32,  // 0.97265625
    True1False32,  // 0.94140625
    True2False32,  // 0.91796875
    True3False32,  // 0.890625
    True0False33,  // 0.97265625
    True1False33,  // 0.9453125
    True2False33,  // 0.91796875
    True0False34,  // 0.97265625
    True1False34,  // 0.9453125
    True2False34,  // 0.921875
    True0False35,  // 0.9765625
    True1False35,  // 0.9453125
    True2False35,  // 0.921875
    True0False36,  // 0.9765625
    True1False36,  // 0.94921875
    True2False36,  // 0.92578125
    True0False37,  // 0.9765625
    True1False37,  // 0.94921875
    True2False37,  // 0.92578125
    True0False38,  // 0.9765625
    True1False38,  // 0.94921875
    True2False38,  // 0.9296875
    True0False39,  // 0.9765625
    True1False39,  // 0.953125
    True2False39,  // 0.9296875
    True0False40,  // 0.9765625
    True1False40,  // 0.953125
    True2False40,  // 0.93359375
    True0False41,  // 0.98046875
    True1False41,  // 0.953125
    True2False41,  // 0.93359375
    True0False42,  // 0.98046875
    True1False42,  // 0.95703125
    True2False42,  // 0.93359375
    True0False43,  // 0.98046875
    True1False43,  // 0.95703125
    True2False43,  // 0.9375
    True0False44,  // 0.98046875
    True1False44,  // 0.95703125
    True0False45,  // 0.98046875
    True1False45,  // 0.95703125
    True0False46,  // 0.98046875
    True1False46,  // 0.9609375
    True0False47,  // 0.98046875
    True1False47,  // 0.9609375
    True0False48,  // 0.98046875
    True1False48,  // 0.9609375
    True0False49,  // 0.98046875
    True1False49,  // 0.9609375
    True0False50,  // 0.984375
    True1False50,  // 0.9609375
    True0False51,  // 0.984375
    True1False51,  // 0.9609375
    True0False52,  // 0.984375
    True1False52,  // 0.96484375
    True0False53,  // 0.984375
    True1False53,  // 0.96484375
    True0False54,  // 0.984375
    True1False54,  // 0.96484375
    True0False55,  // 0.984375
    True1False55,  // 0.96484375
    True0False56,  // 0.984375
    True1False56,  // 0.96484375
    True0False57,  // 0.984375
    True1False57,  // 0.96484375
    True0False58,  // 0.984375
    True1False58,  // 0.96875
    True0False59,  // 0.984375
    True1False59,  // 0.96875
    True0False60,  // 0.984375
    True1False60,  // 0.96875
    True0False61,  // 0.984375
    True1False61,  // 0.96875
    True0False62,  // 0.984375
    True1False62,  // 0.96875
    True0False63,  // 0.98828125
    True1False63,  // 0.96875
    True0False64,  // 0.98828125
    True1False64,  // 0.96875
    True0False65,  // 0.98828125
    True1False65,  // 0.96875
    True0False66,  // 0.98828125
    True0False67,  // 0.98828125
    True0False68,  // 0.98828125
    True0False69,  // 0.98828125
    True0False70,  // 0.98828125
    True0False71,  // 0.98828125
    True0False72,  // 0.98828125
    True0False73,  // 0.98828125
    True0False74,  // 0.98828125
    True0False75,  // 0.98828125
    True0False76,  // 0.98828125
    True0False77,  // 0.98828125
    True0False78,  // 0.98828125
    True0False79,  // 0.98828125
    True0False80,  // 0.98828125
    True0False81,  // 0.98828125
    True0False82,  // 0.98828125
    True0False83,  // 0.98828125
    True0False84,  // 0.98828125
    True0False85,  // 0.98828125
    True0False86,  // 0.9921875
    True0False87,  // 0.9921875
    True0False88,  // 0.9921875
    True0False89,  // 0.9921875
    True0False90,  // 0.9921875
    True0False91,  // 0.9921875
    True0False92,  // 0.9921875
    True0False93,  // 0.9921875
    True0False94,  // 0.9921875
    True0False95,  // 0.9921875
    True0False96,  // 0.9921875
    True0False97,  // 0.9921875
    True0False98,  // 0.9921875
    True0False99,  // 0.9921875
    True0False100, // 0.9921875
    True0False101, // 0.9921875
    True0False102, // 0.9921875
    True0False103, // 0.9921875
    True0False104, // 0.9921875
    True0False105, // 0.9921875
    True0False106, // 0.9921875
    True0False107, // 0.9921875
    True0False108, // 0.9921875
    True0False109, // 0.9921875
    True0False110, // 0.9921875
    True0False111, // 0.9921875
    True0False112, // 0.9921875
    True0False113, // 0.9921875
    True0False114, // 0.9921875
    True0False115, // 0.9921875
    True0False116, // 0.9921875
    True0False117, // 0.9921875
    True0False118, // 0.9921875
    True0False119, // 0.9921875
    True0False120, // 0.9921875
    True0False121, // 0.9921875
    True0False122, // 0.9921875
    True0False123, // 0.9921875
    True0False124, // 0.9921875
    True0False125, // 0.9921875
    True0False126, // 0.9921875
    True0False127, // 0.9921875
    True0False128, // 0.9921875
    True0False129, // 0.9921875
    True0False130, // 0.9921875
    True0False131, // 0.9921875
    True0False132, // 0.9921875
}
use BitContext::*;

impl BitContext {
    #[inline]
    pub fn probability(self) -> Probability {
        const LOOKUP: [Probability; 675] = [
            Probability::new(128, 128),
            Probability::new(171, 85),
            Probability::new(192, 64),
            Probability::new(205, 51),
            Probability::new(214, 42),
            Probability::new(220, 36),
            Probability::new(224, 32),
            Probability::new(228, 28),
            Probability::new(231, 25),
            Probability::new(233, 23),
            Probability::new(235, 21),
            Probability::new(237, 19),
            Probability::new(238, 18),
            Probability::new(239, 17),
            Probability::new(240, 16),
            Probability::new(241, 15),
            Probability::new(242, 14),
            Probability::new(243, 13),
            Probability::new(244, 12),
            Probability::new(244, 12),
            Probability::new(245, 11),
            Probability::new(245, 11),
            Probability::new(246, 10),
            Probability::new(246, 10),
            Probability::new(247, 9),
            Probability::new(247, 9),
            Probability::new(247, 9),
            Probability::new(248, 8),
            Probability::new(248, 8),
            Probability::new(248, 8),
            Probability::new(248, 8),
            Probability::new(249, 7),
            Probability::new(249, 7),
            Probability::new(249, 7),
            Probability::new(249, 7),
            Probability::new(250, 6),
            Probability::new(250, 6),
            Probability::new(250, 6),
            Probability::new(250, 6),
            Probability::new(250, 6),
            Probability::new(250, 6),
            Probability::new(251, 5),
            Probability::new(251, 5),
            Probability::new(251, 5),
            Probability::new(251, 5),
            Probability::new(251, 5),
            Probability::new(251, 5),
            Probability::new(251, 5),
            Probability::new(251, 5),
            Probability::new(251, 5),
            Probability::new(252, 4),
            Probability::new(252, 4),
            Probability::new(252, 4),
            Probability::new(252, 4),
            Probability::new(252, 4),
            Probability::new(252, 4),
            Probability::new(252, 4),
            Probability::new(252, 4),
            Probability::new(252, 4),
            Probability::new(252, 4),
            Probability::new(252, 4),
            Probability::new(252, 4),
            Probability::new(252, 4),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(253, 3),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(254, 2),
            Probability::new(85, 171),
            Probability::new(128, 128),
            Probability::new(154, 102),
            Probability::new(171, 85),
            Probability::new(183, 73),
            Probability::new(192, 64),
            Probability::new(199, 57),
            Probability::new(205, 51),
            Probability::new(209, 47),
            Probability::new(213, 43),
            Probability::new(217, 39),
            Probability::new(219, 37),
            Probability::new(222, 34),
            Probability::new(224, 32),
            Probability::new(226, 30),
            Probability::new(228, 28),
            Probability::new(229, 27),
            Probability::new(230, 26),
            Probability::new(232, 24),
            Probability::new(233, 23),
            Probability::new(234, 22),
            Probability::new(235, 21),
            Probability::new(236, 20),
            Probability::new(236, 20),
            Probability::new(237, 19),
            Probability::new(238, 18),
            Probability::new(238, 18),
            Probability::new(239, 17),
            Probability::new(239, 17),
            Probability::new(240, 16),
            Probability::new(240, 16),
            Probability::new(241, 15),
            Probability::new(241, 15),
            Probability::new(242, 14),
            Probability::new(242, 14),
            Probability::new(242, 14),
            Probability::new(243, 13),
            Probability::new(243, 13),
            Probability::new(243, 13),
            Probability::new(244, 12),
            Probability::new(244, 12),
            Probability::new(244, 12),
            Probability::new(245, 11),
            Probability::new(245, 11),
            Probability::new(245, 11),
            Probability::new(245, 11),
            Probability::new(246, 10),
            Probability::new(246, 10),
            Probability::new(246, 10),
            Probability::new(246, 10),
            Probability::new(246, 10),
            Probability::new(246, 10),
            Probability::new(247, 9),
            Probability::new(247, 9),
            Probability::new(247, 9),
            Probability::new(247, 9),
            Probability::new(247, 9),
            Probability::new(247, 9),
            Probability::new(248, 8),
            Probability::new(248, 8),
            Probability::new(248, 8),
            Probability::new(248, 8),
            Probability::new(248, 8),
            Probability::new(248, 8),
            Probability::new(248, 8),
            Probability::new(248, 8),
            Probability::new(64, 192),
            Probability::new(102, 154),
            Probability::new(128, 128),
            Probability::new(146, 110),
            Probability::new(160, 96),
            Probability::new(171, 85),
            Probability::new(179, 77),
            Probability::new(186, 70),
            Probability::new(192, 64),
            Probability::new(197, 59),
            Probability::new(201, 55),
            Probability::new(205, 51),
            Probability::new(208, 48),
            Probability::new(211, 45),
            Probability::new(213, 43),
            Probability::new(216, 40),
            Probability::new(218, 38),
            Probability::new(219, 37),
            Probability::new(221, 35),
            Probability::new(223, 33),
            Probability::new(224, 32),
            Probability::new(225, 31),
            Probability::new(226, 30),
            Probability::new(228, 28),
            Probability::new(229, 27),
            Probability::new(230, 26),
            Probability::new(230, 26),
            Probability::new(231, 25),
            Probability::new(232, 24),
            Probability::new(233, 23),
            Probability::new(233, 23),
            Probability::new(234, 22),
            Probability::new(235, 21),
            Probability::new(235, 21),
            Probability::new(236, 20),
            Probability::new(236, 20),
            Probability::new(237, 19),
            Probability::new(237, 19),
            Probability::new(238, 18),
            Probability::new(238, 18),
            Probability::new(239, 17),
            Probability::new(239, 17),
            Probability::new(239, 17),
            Probability::new(240, 16),
            Probability::new(51, 205),
            Probability::new(85, 171),
            Probability::new(110, 146),
            Probability::new(128, 128),
            Probability::new(142, 114),
            Probability::new(154, 102),
            Probability::new(163, 93),
            Probability::new(171, 85),
            Probability::new(177, 79),
            Probability::new(183, 73),
            Probability::new(188, 68),
            Probability::new(192, 64),
            Probability::new(196, 60),
            Probability::new(199, 57),
            Probability::new(202, 54),
            Probability::new(205, 51),
            Probability::new(207, 49),
            Probability::new(209, 47),
            Probability::new(211, 45),
            Probability::new(213, 43),
            Probability::new(215, 41),
            Probability::new(217, 39),
            Probability::new(218, 38),
            Probability::new(219, 37),
            Probability::new(221, 35),
            Probability::new(222, 34),
            Probability::new(223, 33),
            Probability::new(224, 32),
            Probability::new(225, 31),
            Probability::new(226, 30),
            Probability::new(227, 29),
            Probability::new(228, 28),
            Probability::new(228, 28),
            Probability::new(42, 214),
            Probability::new(73, 183),
            Probability::new(96, 160),
            Probability::new(114, 142),
            Probability::new(128, 128),
            Probability::new(140, 116),
            Probability::new(149, 107),
            Probability::new(158, 98),
            Probability::new(165, 91),
            Probability::new(171, 85),
            Probability::new(176, 80),
            Probability::new(181, 75),
            Probability::new(185, 71),
            Probability::new(189, 67),
            Probability::new(192, 64),
            Probability::new(195, 61),
            Probability::new(198, 58),
            Probability::new(200, 56),
            Probability::new(203, 53),
            Probability::new(205, 51),
            Probability::new(207, 49),
            Probability::new(209, 47),
            Probability::new(210, 46),
            Probability::new(212, 44),
            Probability::new(213, 43),
            Probability::new(215, 41),
            Probability::new(36, 220),
            Probability::new(64, 192),
            Probability::new(85, 171),
            Probability::new(102, 154),
            Probability::new(116, 140),
            Probability::new(128, 128),
            Probability::new(138, 118),
            Probability::new(146, 110),
            Probability::new(154, 102),
            Probability::new(160, 96),
            Probability::new(166, 90),
            Probability::new(171, 85),
            Probability::new(175, 81),
            Probability::new(179, 77),
            Probability::new(183, 73),
            Probability::new(186, 70),
            Probability::new(189, 67),
            Probability::new(192, 64),
            Probability::new(195, 61),
            Probability::new(197, 59),
            Probability::new(199, 57),
            Probability::new(201, 55),
            Probability::new(32, 224),
            Probability::new(57, 199),
            Probability::new(77, 179),
            Probability::new(93, 163),
            Probability::new(107, 149),
            Probability::new(118, 138),
            Probability::new(128, 128),
            Probability::new(137, 119),
            Probability::new(144, 112),
            Probability::new(151, 105),
            Probability::new(156, 100),
            Probability::new(162, 94),
            Probability::new(166, 90),
            Probability::new(171, 85),
            Probability::new(175, 81),
            Probability::new(178, 78),
            Probability::new(181, 75),
            Probability::new(184, 72),
            Probability::new(187, 69),
            Probability::new(28, 228),
            Probability::new(51, 205),
            Probability::new(70, 186),
            Probability::new(85, 171),
            Probability::new(98, 158),
            Probability::new(110, 146),
            Probability::new(119, 137),
            Probability::new(128, 128),
            Probability::new(136, 120),
            Probability::new(142, 114),
            Probability::new(148, 108),
            Probability::new(154, 102),
            Probability::new(158, 98),
            Probability::new(163, 93),
            Probability::new(167, 89),
            Probability::new(171, 85),
            Probability::new(25, 231),
            Probability::new(47, 209),
            Probability::new(64, 192),
            Probability::new(79, 177),
            Probability::new(91, 165),
            Probability::new(102, 154),
            Probability::new(112, 144),
            Probability::new(120, 136),
            Probability::new(128, 128),
            Probability::new(135, 121),
            Probability::new(141, 115),
            Probability::new(146, 110),
            Probability::new(151, 105),
            Probability::new(156, 100),
            Probability::new(23, 233),
            Probability::new(43, 213),
            Probability::new(59, 197),
            Probability::new(73, 183),
            Probability::new(85, 171),
            Probability::new(96, 160),
            Probability::new(105, 151),
            Probability::new(114, 142),
            Probability::new(121, 135),
            Probability::new(128, 128),
            Probability::new(134, 122),
            Probability::new(140, 116),
            Probability::new(145, 111),
            Probability::new(21, 235),
            Probability::new(39, 217),
            Probability::new(55, 201),
            Probability::new(68, 188),
            Probability::new(80, 176),
            Probability::new(90, 166),
            Probability::new(100, 156),
            Probability::new(108, 148),
            Probability::new(115, 141),
            Probability::new(122, 134),
            Probability::new(128, 128),
            Probability::new(134, 122),
            Probability::new(19, 237),
            Probability::new(37, 219),
            Probability::new(51, 205),
            Probability::new(64, 192),
            Probability::new(75, 181),
            Probability::new(85, 171),
            Probability::new(94, 162),
            Probability::new(102, 154),
            Probability::new(110, 146),
            Probability::new(116, 140),
            Probability::new(122, 134),
            Probability::new(18, 238),
            Probability::new(34, 222),
            Probability::new(48, 208),
            Probability::new(60, 196),
            Probability::new(71, 185),
            Probability::new(81, 175),
            Probability::new(90, 166),
            Probability::new(98, 158),
            Probability::new(105, 151),
            Probability::new(111, 145),
            Probability::new(17, 239),
            Probability::new(32, 224),
            Probability::new(45, 211),
            Probability::new(57, 199),
            Probability::new(67, 189),
            Probability::new(77, 179),
            Probability::new(85, 171),
            Probability::new(93, 163),
            Probability::new(100, 156),
            Probability::new(16, 240),
            Probability::new(30, 226),
            Probability::new(43, 213),
            Probability::new(54, 202),
            Probability::new(64, 192),
            Probability::new(73, 183),
            Probability::new(81, 175),
            Probability::new(89, 167),
            Probability::new(15, 241),
            Probability::new(28, 228),
            Probability::new(40, 216),
            Probability::new(51, 205),
            Probability::new(61, 195),
            Probability::new(70, 186),
            Probability::new(78, 178),
            Probability::new(85, 171),
            Probability::new(14, 242),
            Probability::new(27, 229),
            Probability::new(38, 218),
            Probability::new(49, 207),
            Probability::new(58, 198),
            Probability::new(67, 189),
            Probability::new(75, 181),
            Probability::new(13, 243),
            Probability::new(26, 230),
            Probability::new(37, 219),
            Probability::new(47, 209),
            Probability::new(56, 200),
            Probability::new(64, 192),
            Probability::new(72, 184),
            Probability::new(12, 244),
            Probability::new(24, 232),
            Probability::new(35, 221),
            Probability::new(45, 211),
            Probability::new(53, 203),
            Probability::new(61, 195),
            Probability::new(69, 187),
            Probability::new(12, 244),
            Probability::new(23, 233),
            Probability::new(33, 223),
            Probability::new(43, 213),
            Probability::new(51, 205),
            Probability::new(59, 197),
            Probability::new(11, 245),
            Probability::new(22, 234),
            Probability::new(32, 224),
            Probability::new(41, 215),
            Probability::new(49, 207),
            Probability::new(57, 199),
            Probability::new(11, 245),
            Probability::new(21, 235),
            Probability::new(31, 225),
            Probability::new(39, 217),
            Probability::new(47, 209),
            Probability::new(55, 201),
            Probability::new(10, 246),
            Probability::new(20, 236),
            Probability::new(30, 226),
            Probability::new(38, 218),
            Probability::new(46, 210),
            Probability::new(10, 246),
            Probability::new(20, 236),
            Probability::new(28, 228),
            Probability::new(37, 219),
            Probability::new(44, 212),
            Probability::new(9, 247),
            Probability::new(19, 237),
            Probability::new(27, 229),
            Probability::new(35, 221),
            Probability::new(43, 213),
            Probability::new(9, 247),
            Probability::new(18, 238),
            Probability::new(26, 230),
            Probability::new(34, 222),
            Probability::new(41, 215),
            Probability::new(9, 247),
            Probability::new(18, 238),
            Probability::new(26, 230),
            Probability::new(33, 223),
            Probability::new(8, 248),
            Probability::new(17, 239),
            Probability::new(25, 231),
            Probability::new(32, 224),
            Probability::new(8, 248),
            Probability::new(17, 239),
            Probability::new(24, 232),
            Probability::new(31, 225),
            Probability::new(8, 248),
            Probability::new(16, 240),
            Probability::new(23, 233),
            Probability::new(30, 226),
            Probability::new(8, 248),
            Probability::new(16, 240),
            Probability::new(23, 233),
            Probability::new(29, 227),
            Probability::new(7, 249),
            Probability::new(15, 241),
            Probability::new(22, 234),
            Probability::new(28, 228),
            Probability::new(7, 249),
            Probability::new(15, 241),
            Probability::new(21, 235),
            Probability::new(28, 228),
            Probability::new(7, 249),
            Probability::new(14, 242),
            Probability::new(21, 235),
            Probability::new(7, 249),
            Probability::new(14, 242),
            Probability::new(20, 236),
            Probability::new(6, 250),
            Probability::new(14, 242),
            Probability::new(20, 236),
            Probability::new(6, 250),
            Probability::new(13, 243),
            Probability::new(19, 237),
            Probability::new(6, 250),
            Probability::new(13, 243),
            Probability::new(19, 237),
            Probability::new(6, 250),
            Probability::new(13, 243),
            Probability::new(18, 238),
            Probability::new(6, 250),
            Probability::new(12, 244),
            Probability::new(18, 238),
            Probability::new(6, 250),
            Probability::new(12, 244),
            Probability::new(17, 239),
            Probability::new(5, 251),
            Probability::new(12, 244),
            Probability::new(17, 239),
            Probability::new(5, 251),
            Probability::new(11, 245),
            Probability::new(17, 239),
            Probability::new(5, 251),
            Probability::new(11, 245),
            Probability::new(16, 240),
            Probability::new(5, 251),
            Probability::new(11, 245),
            Probability::new(5, 251),
            Probability::new(11, 245),
            Probability::new(5, 251),
            Probability::new(10, 246),
            Probability::new(5, 251),
            Probability::new(10, 246),
            Probability::new(5, 251),
            Probability::new(10, 246),
            Probability::new(5, 251),
            Probability::new(10, 246),
            Probability::new(4, 252),
            Probability::new(10, 246),
            Probability::new(4, 252),
            Probability::new(10, 246),
            Probability::new(4, 252),
            Probability::new(9, 247),
            Probability::new(4, 252),
            Probability::new(9, 247),
            Probability::new(4, 252),
            Probability::new(9, 247),
            Probability::new(4, 252),
            Probability::new(9, 247),
            Probability::new(4, 252),
            Probability::new(9, 247),
            Probability::new(4, 252),
            Probability::new(9, 247),
            Probability::new(4, 252),
            Probability::new(8, 248),
            Probability::new(4, 252),
            Probability::new(8, 248),
            Probability::new(4, 252),
            Probability::new(8, 248),
            Probability::new(4, 252),
            Probability::new(8, 248),
            Probability::new(4, 252),
            Probability::new(8, 248),
            Probability::new(3, 253),
            Probability::new(8, 248),
            Probability::new(3, 253),
            Probability::new(8, 248),
            Probability::new(3, 253),
            Probability::new(8, 248),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(3, 253),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
            Probability::new(2, 254),
        ];
        LOOKUP[self as usize]
    }

    #[inline]
    pub fn adapt(self, bit: bool) -> Self {
        const OUTCOMES: [BitContext; 2 * 675] = [
            True0False1,
            True1False1,
            True2False1,
            True3False1,
            True4False1,
            True5False1,
            True6False1,
            True7False1,
            True8False1,
            True9False1,
            True10False1,
            True11False1,
            True12False1,
            True13False1,
            True14False1,
            True15False1,
            True16False1,
            True17False1,
            True18False1,
            True19False1,
            True20False1,
            True21False1,
            True22False1,
            True23False1,
            True24False1,
            True25False1,
            True26False1,
            True27False1,
            True28False1,
            True29False1,
            True30False1,
            True31False1,
            True32False1,
            True33False1,
            True34False1,
            True35False1,
            True36False1,
            True37False1,
            True38False1,
            True39False1,
            True40False1,
            True41False1,
            True42False1,
            True43False1,
            True44False1,
            True45False1,
            True46False1,
            True47False1,
            True48False1,
            True49False1,
            True50False1,
            True51False1,
            True52False1,
            True53False1,
            True54False1,
            True55False1,
            True56False1,
            True57False1,
            True58False1,
            True59False1,
            True60False1,
            True61False1,
            True62False1,
            True63False1,
            True64False1,
            True65False1,
            True33False0,
            True33False0,
            True34False0,
            True34False0,
            True35False0,
            True35False0,
            True36False0,
            True36False0,
            True37False0,
            True37False0,
            True38False0,
            True38False0,
            True39False0,
            True39False0,
            True40False0,
            True40False0,
            True41False0,
            True41False0,
            True42False0,
            True42False0,
            True43False0,
            True43False0,
            True44False0,
            True44False0,
            True45False0,
            True45False0,
            True46False0,
            True46False0,
            True47False0,
            True47False0,
            True48False0,
            True48False0,
            True49False0,
            True49False0,
            True50False0,
            True50False0,
            True51False0,
            True51False0,
            True52False0,
            True52False0,
            True53False0,
            True53False0,
            True54False0,
            True54False0,
            True55False0,
            True55False0,
            True56False0,
            True56False0,
            True57False0,
            True57False0,
            True58False0,
            True58False0,
            True59False0,
            True59False0,
            True60False0,
            True60False0,
            True61False0,
            True61False0,
            True62False0,
            True62False0,
            True63False0,
            True63False0,
            True64False0,
            True64False0,
            True65False0,
            True65False0,
            True66False0,
            True0False2,
            True1False2,
            True2False2,
            True3False2,
            True4False2,
            True5False2,
            True6False2,
            True7False2,
            True8False2,
            True9False2,
            True10False2,
            True11False2,
            True12False2,
            True13False2,
            True14False2,
            True15False2,
            True16False2,
            True17False2,
            True18False2,
            True19False2,
            True20False2,
            True21False2,
            True22False2,
            True23False2,
            True24False2,
            True25False2,
            True26False2,
            True27False2,
            True28False2,
            True29False2,
            True30False2,
            True31False2,
            True32False2,
            True33False2,
            True34False2,
            True35False2,
            True36False2,
            True37False2,
            True38False2,
            True39False2,
            True40False2,
            True41False2,
            True42False2,
            True43False2,
            True22False1,
            True22False1,
            True23False1,
            True23False1,
            True24False1,
            True24False1,
            True25False1,
            True25False1,
            True26False1,
            True26False1,
            True27False1,
            True27False1,
            True28False1,
            True28False1,
            True29False1,
            True29False1,
            True30False1,
            True30False1,
            True31False1,
            True31False1,
            True32False1,
            True32False1,
            True0False3,
            True1False3,
            True2False3,
            True3False3,
            True4False3,
            True5False3,
            True6False3,
            True7False3,
            True8False3,
            True9False3,
            True10False3,
            True11False3,
            True12False3,
            True13False3,
            True14False3,
            True15False3,
            True16False3,
            True17False3,
            True18False3,
            True19False3,
            True20False3,
            True21False3,
            True22False3,
            True23False3,
            True24False3,
            True25False3,
            True26False3,
            True27False3,
            True28False3,
            True29False3,
            True30False3,
            True31False3,
            True32False3,
            True16False1,
            True17False1,
            True17False1,
            True18False1,
            True18False1,
            True19False1,
            True19False1,
            True20False1,
            True20False1,
            True21False1,
            True21False1,
            True0False4,
            True1False4,
            True2False4,
            True3False4,
            True4False4,
            True5False4,
            True6False4,
            True7False4,
            True8False4,
            True9False4,
            True10False4,
            True11False4,
            True12False4,
            True13False4,
            True14False4,
            True15False4,
            True16False4,
            True17False4,
            True18False4,
            True19False4,
            True20False4,
            True21False4,
            True22False4,
            True23False4,
            True24False4,
            True25False4,
            True13False2,
            True13False2,
            True14False2,
            True14False2,
            True15False2,
            True15False2,
            True16False2,
            True0False5,
            True1False5,
            True2False5,
            True3False5,
            True4False5,
            True5False5,
            True6False5,
            True7False5,
            True8False5,
            True9False5,
            True10False5,
            True11False5,
            True12False5,
            True13False5,
            True14False5,
            True15False5,
            True16False5,
            True17False5,
            True18False5,
            True19False5,
            True20False5,
            True21False5,
            True11False2,
            True11False2,
            True12False2,
            True12False2,
            True0False6,
            True1False6,
            True2False6,
            True3False6,
            True4False6,
            True5False6,
            True6False6,
            True7False6,
            True8False6,
            True9False6,
            True10False6,
            True11False6,
            True12False6,
            True13False6,
            True14False6,
            True15False6,
            True16False6,
            True17False6,
            True18False6,
            True9False3,
            True10False3,
            True10False3,
            True0False7,
            True1False7,
            True2False7,
            True3False7,
            True4False7,
            True5False7,
            True6False7,
            True7False7,
            True8False7,
            True9False7,
            True10False7,
            True11False7,
            True12False7,
            True13False7,
            True14False7,
            True15False7,
            True8False3,
            True8False3,
            True9False3,
            True0False8,
            True1False8,
            True2False8,
            True3False8,
            True4False8,
            True5False8,
            True6False8,
            True7False8,
            True8False8,
            True9False8,
            True10False8,
            True11False8,
            True12False8,
            True13False8,
            True7False4,
            True7False4,
            True0False9,
            True1False9,
            True2False9,
            True3False9,
            True4False9,
            True5False9,
            True6False9,
            True7False9,
            True8False9,
            True9False9,
            True10False9,
            True11False9,
            True12False9,
            True6False4,
            True0False10,
            True1False10,
            True2False10,
            True3False10,
            True4False10,
            True5False10,
            True6False10,
            True7False10,
            True8False10,
            True9False10,
            True10False10,
            True11False10,
            True6False5,
            True0False11,
            True1False11,
            True2False11,
            True3False11,
            True4False11,
            True5False11,
            True6False11,
            True7False11,
            True8False11,
            True9False11,
            True10False11,
            True5False5,
            True0False12,
            True1False12,
            True2False12,
            True3False12,
            True4False12,
            True5False12,
            True6False12,
            True7False12,
            True8False12,
            True9False12,
            True5False6,
            True0False13,
            True1False13,
            True2False13,
            True3False13,
            True4False13,
            True5False13,
            True6False13,
            True7False13,
            True8False13,
            True4False6,
            True0False14,
            True1False14,
            True2False14,
            True3False14,
            True4False14,
            True5False14,
            True6False14,
            True7False14,
            True4False7,
            True0False15,
            True1False15,
            True2False15,
            True3False15,
            True4False15,
            True5False15,
            True6False15,
            True7False15,
            True0False16,
            True1False16,
            True2False16,
            True3False16,
            True4False16,
            True5False16,
            True6False16,
            True3False8,
            True0False17,
            True1False17,
            True2False17,
            True3False17,
            True4False17,
            True5False17,
            True6False17,
            True0False18,
            True1False18,
            True2False18,
            True3False18,
            True4False18,
            True5False18,
            True6False18,
            True0False19,
            True1False19,
            True2False19,
            True3False19,
            True4False19,
            True5False19,
            True3False9,
            True0False20,
            True1False20,
            True2False20,
            True3False20,
            True4False20,
            True5False20,
            True0False21,
            True1False21,
            True2False21,
            True3False21,
            True4False21,
            True5False21,
            True0False22,
            True1False22,
            True2False22,
            True3False22,
            True4False22,
            True2False11,
            True0False23,
            True1False23,
            True2False23,
            True3False23,
            True4False23,
            True0False24,
            True1False24,
            True2False24,
            True3False24,
            True4False24,
            True0False25,
            True1False25,
            True2False25,
            True3False25,
            True4False25,
            True0False26,
            True1False26,
            True2False26,
            True3False26,
            True2False13,
            True0False27,
            True1False27,
            True2False27,
            True3False27,
            True0False28,
            True1False28,
            True2False28,
            True3False28,
            True0False29,
            True1False29,
            True2False29,
            True3False29,
            True0False30,
            True1False30,
            True2False30,
            True3False30,
            True0False31,
            True1False31,
            True2False31,
            True3False31,
            True0False32,
            True1False32,
            True2False32,
            True3False32,
            True0False33,
            True1False33,
            True2False33,
            True1False16,
            True0False34,
            True1False34,
            True2False34,
            True0False35,
            True1False35,
            True2False35,
            True0False36,
            True1False36,
            True2False36,
            True0False37,
            True1False37,
            True2False37,
            True0False38,
            True1False38,
            True2False38,
            True0False39,
            True1False39,
            True2False39,
            True0False40,
            True1False40,
            True2False40,
            True0False41,
            True1False41,
            True2False41,
            True0False42,
            True1False42,
            True2False42,
            True0False43,
            True1False43,
            True2False43,
            True0False44,
            True1False44,
            True1False22,
            True0False45,
            True1False45,
            True0False46,
            True1False46,
            True0False47,
            True1False47,
            True0False48,
            True1False48,
            True0False49,
            True1False49,
            True0False50,
            True1False50,
            True0False51,
            True1False51,
            True0False52,
            True1False52,
            True0False53,
            True1False53,
            True0False54,
            True1False54,
            True0False55,
            True1False55,
            True0False56,
            True1False56,
            True0False57,
            True1False57,
            True0False58,
            True1False58,
            True0False59,
            True1False59,
            True0False60,
            True1False60,
            True0False61,
            True1False61,
            True0False62,
            True1False62,
            True0False63,
            True1False63,
            True0False64,
            True1False64,
            True0False65,
            True1False65,
            True0False66,
            True0False33,
            True0False67,
            True0False68,
            True0False69,
            True0False70,
            True0False71,
            True0False72,
            True0False73,
            True0False74,
            True0False75,
            True0False76,
            True0False77,
            True0False78,
            True0False79,
            True0False80,
            True0False81,
            True0False82,
            True0False83,
            True0False84,
            True0False85,
            True0False86,
            True0False87,
            True0False88,
            True0False89,
            True0False90,
            True0False91,
            True0False92,
            True0False93,
            True0False94,
            True0False95,
            True0False96,
            True0False97,
            True0False98,
            True0False99,
            True0False100,
            True0False101,
            True0False102,
            True0False103,
            True0False104,
            True0False105,
            True0False106,
            True0False107,
            True0False108,
            True0False109,
            True0False110,
            True0False111,
            True0False112,
            True0False113,
            True0False114,
            True0False115,
            True0False116,
            True0False117,
            True0False118,
            True0False119,
            True0False120,
            True0False121,
            True0False122,
            True0False123,
            True0False124,
            True0False125,
            True0False126,
            True0False127,
            True0False128,
            True0False129,
            True0False130,
            True0False131,
            True0False132,
            True0False132,
            True1False0,
            True2False0,
            True3False0,
            True4False0,
            True5False0,
            True6False0,
            True7False0,
            True8False0,
            True9False0,
            True10False0,
            True11False0,
            True12False0,
            True13False0,
            True14False0,
            True15False0,
            True16False0,
            True17False0,
            True18False0,
            True19False0,
            True20False0,
            True21False0,
            True22False0,
            True23False0,
            True24False0,
            True25False0,
            True26False0,
            True27False0,
            True28False0,
            True29False0,
            True30False0,
            True31False0,
            True32False0,
            True33False0,
            True34False0,
            True35False0,
            True36False0,
            True37False0,
            True38False0,
            True39False0,
            True40False0,
            True41False0,
            True42False0,
            True43False0,
            True44False0,
            True45False0,
            True46False0,
            True47False0,
            True48False0,
            True49False0,
            True50False0,
            True51False0,
            True52False0,
            True53False0,
            True54False0,
            True55False0,
            True56False0,
            True57False0,
            True58False0,
            True59False0,
            True60False0,
            True61False0,
            True62False0,
            True63False0,
            True64False0,
            True65False0,
            True66False0,
            True67False0,
            True68False0,
            True69False0,
            True70False0,
            True71False0,
            True72False0,
            True73False0,
            True74False0,
            True75False0,
            True76False0,
            True77False0,
            True78False0,
            True79False0,
            True80False0,
            True81False0,
            True82False0,
            True83False0,
            True84False0,
            True85False0,
            True86False0,
            True87False0,
            True88False0,
            True89False0,
            True90False0,
            True91False0,
            True92False0,
            True93False0,
            True94False0,
            True95False0,
            True96False0,
            True97False0,
            True98False0,
            True99False0,
            True100False0,
            True101False0,
            True102False0,
            True103False0,
            True104False0,
            True105False0,
            True106False0,
            True107False0,
            True108False0,
            True109False0,
            True110False0,
            True111False0,
            True112False0,
            True113False0,
            True114False0,
            True115False0,
            True116False0,
            True117False0,
            True118False0,
            True119False0,
            True120False0,
            True121False0,
            True122False0,
            True123False0,
            True124False0,
            True125False0,
            True126False0,
            True127False0,
            True128False0,
            True129False0,
            True130False0,
            True131False0,
            True132False0,
            True132False0,
            True1False1,
            True2False1,
            True3False1,
            True4False1,
            True5False1,
            True6False1,
            True7False1,
            True8False1,
            True9False1,
            True10False1,
            True11False1,
            True12False1,
            True13False1,
            True14False1,
            True15False1,
            True16False1,
            True17False1,
            True18False1,
            True19False1,
            True20False1,
            True21False1,
            True22False1,
            True23False1,
            True24False1,
            True25False1,
            True26False1,
            True27False1,
            True28False1,
            True29False1,
            True30False1,
            True31False1,
            True32False1,
            True33False1,
            True34False1,
            True35False1,
            True36False1,
            True37False1,
            True38False1,
            True39False1,
            True40False1,
            True41False1,
            True42False1,
            True43False1,
            True44False1,
            True45False1,
            True46False1,
            True47False1,
            True48False1,
            True49False1,
            True50False1,
            True51False1,
            True52False1,
            True53False1,
            True54False1,
            True55False1,
            True56False1,
            True57False1,
            True58False1,
            True59False1,
            True60False1,
            True61False1,
            True62False1,
            True63False1,
            True64False1,
            True65False1,
            True33False0,
            True1False2,
            True2False2,
            True3False2,
            True4False2,
            True5False2,
            True6False2,
            True7False2,
            True8False2,
            True9False2,
            True10False2,
            True11False2,
            True12False2,
            True13False2,
            True14False2,
            True15False2,
            True16False2,
            True17False2,
            True18False2,
            True19False2,
            True20False2,
            True21False2,
            True22False2,
            True23False2,
            True24False2,
            True25False2,
            True26False2,
            True27False2,
            True28False2,
            True29False2,
            True30False2,
            True31False2,
            True32False2,
            True33False2,
            True34False2,
            True35False2,
            True36False2,
            True37False2,
            True38False2,
            True39False2,
            True40False2,
            True41False2,
            True42False2,
            True43False2,
            True22False1,
            True1False3,
            True2False3,
            True3False3,
            True4False3,
            True5False3,
            True6False3,
            True7False3,
            True8False3,
            True9False3,
            True10False3,
            True11False3,
            True12False3,
            True13False3,
            True14False3,
            True15False3,
            True16False3,
            True17False3,
            True18False3,
            True19False3,
            True20False3,
            True21False3,
            True22False3,
            True23False3,
            True24False3,
            True25False3,
            True26False3,
            True27False3,
            True28False3,
            True29False3,
            True30False3,
            True31False3,
            True32False3,
            True16False1,
            True1False4,
            True2False4,
            True3False4,
            True4False4,
            True5False4,
            True6False4,
            True7False4,
            True8False4,
            True9False4,
            True10False4,
            True11False4,
            True12False4,
            True13False4,
            True14False4,
            True15False4,
            True16False4,
            True17False4,
            True18False4,
            True19False4,
            True20False4,
            True21False4,
            True22False4,
            True23False4,
            True24False4,
            True25False4,
            True13False2,
            True1False5,
            True2False5,
            True3False5,
            True4False5,
            True5False5,
            True6False5,
            True7False5,
            True8False5,
            True9False5,
            True10False5,
            True11False5,
            True12False5,
            True13False5,
            True14False5,
            True15False5,
            True16False5,
            True17False5,
            True18False5,
            True19False5,
            True20False5,
            True21False5,
            True11False2,
            True1False6,
            True2False6,
            True3False6,
            True4False6,
            True5False6,
            True6False6,
            True7False6,
            True8False6,
            True9False6,
            True10False6,
            True11False6,
            True12False6,
            True13False6,
            True14False6,
            True15False6,
            True16False6,
            True17False6,
            True18False6,
            True9False3,
            True1False7,
            True2False7,
            True3False7,
            True4False7,
            True5False7,
            True6False7,
            True7False7,
            True8False7,
            True9False7,
            True10False7,
            True11False7,
            True12False7,
            True13False7,
            True14False7,
            True15False7,
            True8False3,
            True1False8,
            True2False8,
            True3False8,
            True4False8,
            True5False8,
            True6False8,
            True7False8,
            True8False8,
            True9False8,
            True10False8,
            True11False8,
            True12False8,
            True13False8,
            True7False4,
            True1False9,
            True2False9,
            True3False9,
            True4False9,
            True5False9,
            True6False9,
            True7False9,
            True8False9,
            True9False9,
            True10False9,
            True11False9,
            True12False9,
            True6False4,
            True1False10,
            True2False10,
            True3False10,
            True4False10,
            True5False10,
            True6False10,
            True7False10,
            True8False10,
            True9False10,
            True10False10,
            True11False10,
            True6False5,
            True1False11,
            True2False11,
            True3False11,
            True4False11,
            True5False11,
            True6False11,
            True7False11,
            True8False11,
            True9False11,
            True10False11,
            True5False5,
            True1False12,
            True2False12,
            True3False12,
            True4False12,
            True5False12,
            True6False12,
            True7False12,
            True8False12,
            True9False12,
            True5False6,
            True1False13,
            True2False13,
            True3False13,
            True4False13,
            True5False13,
            True6False13,
            True7False13,
            True8False13,
            True4False6,
            True1False14,
            True2False14,
            True3False14,
            True4False14,
            True5False14,
            True6False14,
            True7False14,
            True4False7,
            True1False15,
            True2False15,
            True3False15,
            True4False15,
            True5False15,
            True6False15,
            True7False15,
            True4False7,
            True1False16,
            True2False16,
            True3False16,
            True4False16,
            True5False16,
            True6False16,
            True3False8,
            True1False17,
            True2False17,
            True3False17,
            True4False17,
            True5False17,
            True6False17,
            True3False8,
            True1False18,
            True2False18,
            True3False18,
            True4False18,
            True5False18,
            True6False18,
            True3False9,
            True1False19,
            True2False19,
            True3False19,
            True4False19,
            True5False19,
            True3False9,
            True1False20,
            True2False20,
            True3False20,
            True4False20,
            True5False20,
            True3False10,
            True1False21,
            True2False21,
            True3False21,
            True4False21,
            True5False21,
            True3False10,
            True1False22,
            True2False22,
            True3False22,
            True4False22,
            True2False11,
            True1False23,
            True2False23,
            True3False23,
            True4False23,
            True2False11,
            True1False24,
            True2False24,
            True3False24,
            True4False24,
            True2False12,
            True1False25,
            True2False25,
            True3False25,
            True4False25,
            True2False12,
            True1False26,
            True2False26,
            True3False26,
            True2False13,
            True1False27,
            True2False27,
            True3False27,
            True2False13,
            True1False28,
            True2False28,
            True3False28,
            True2False14,
            True1False29,
            True2False29,
            True3False29,
            True2False14,
            True1False30,
            True2False30,
            True3False30,
            True2False15,
            True1False31,
            True2False31,
            True3False31,
            True2False15,
            True1False32,
            True2False32,
            True3False32,
            True2False16,
            True1False33,
            True2False33,
            True1False16,
            True1False34,
            True2False34,
            True1False17,
            True1False35,
            True2False35,
            True1False17,
            True1False36,
            True2False36,
            True1False18,
            True1False37,
            True2False37,
            True1False18,
            True1False38,
            True2False38,
            True1False19,
            True1False39,
            True2False39,
            True1False19,
            True1False40,
            True2False40,
            True1False20,
            True1False41,
            True2False41,
            True1False20,
            True1False42,
            True2False42,
            True1False21,
            True1False43,
            True2False43,
            True1False21,
            True1False44,
            True1False22,
            True1False45,
            True1False22,
            True1False46,
            True1False23,
            True1False47,
            True1False23,
            True1False48,
            True1False24,
            True1False49,
            True1False24,
            True1False50,
            True1False25,
            True1False51,
            True1False25,
            True1False52,
            True1False26,
            True1False53,
            True1False26,
            True1False54,
            True1False27,
            True1False55,
            True1False27,
            True1False56,
            True1False28,
            True1False57,
            True1False28,
            True1False58,
            True1False29,
            True1False59,
            True1False29,
            True1False60,
            True1False30,
            True1False61,
            True1False30,
            True1False62,
            True1False31,
            True1False63,
            True1False31,
            True1False64,
            True1False32,
            True1False65,
            True1False32,
            True0False33,
            True0False33,
            True0False34,
            True0False34,
            True0False35,
            True0False35,
            True0False36,
            True0False36,
            True0False37,
            True0False37,
            True0False38,
            True0False38,
            True0False39,
            True0False39,
            True0False40,
            True0False40,
            True0False41,
            True0False41,
            True0False42,
            True0False42,
            True0False43,
            True0False43,
            True0False44,
            True0False44,
            True0False45,
            True0False45,
            True0False46,
            True0False46,
            True0False47,
            True0False47,
            True0False48,
            True0False48,
            True0False49,
            True0False49,
            True0False50,
            True0False50,
            True0False51,
            True0False51,
            True0False52,
            True0False52,
            True0False53,
            True0False53,
            True0False54,
            True0False54,
            True0False55,
            True0False55,
            True0False56,
            True0False56,
            True0False57,
            True0False57,
            True0False58,
            True0False58,
            True0False59,
            True0False59,
            True0False60,
            True0False60,
            True0False61,
            True0False61,
            True0False62,
            True0False62,
            True0False63,
            True0False63,
            True0False64,
            True0False64,
            True0False65,
            True0False65,
            True0False66,
        ];
        OUTCOMES[(self as usize) + (bit as usize) * 675]
    }
    #[inline]
    pub fn millibits_required(&mut self, bit: bool) -> u32 {
        const LOOKUP: [u32; 1350] = [
            1000, // for false
            1590, // for false
            2000, // for false
            2327, // for false
            2607, // for false
            2830, // for false
            3000, // for false
            3192, // for false
            3356, // for false
            3476, // for false
            3607, // for false
            3752, // for false
            3830, // for false
            3912, // for false
            4000, // for false
            4093, // for false
            4192, // for false
            4299, // for false
            4415, // for false
            4415, // for false
            4540, // for false
            4540, // for false
            4678, // for false
            4678, // for false
            4830, // for false
            4830, // for false
            4830, // for false
            5000, // for false
            5000, // for false
            5000, // for false
            5000, // for false
            5192, // for false
            5192, // for false
            5192, // for false
            5192, // for false
            5415, // for false
            5415, // for false
            5415, // for false
            5415, // for false
            5415, // for false
            5415, // for false
            5678, // for false
            5678, // for false
            5678, // for false
            5678, // for false
            5678, // for false
            5678, // for false
            5678, // for false
            5678, // for false
            5678, // for false
            6000, // for false
            6000, // for false
            6000, // for false
            6000, // for false
            6000, // for false
            6000, // for false
            6000, // for false
            6000, // for false
            6000, // for false
            6000, // for false
            6000, // for false
            6000, // for false
            6000, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            6415, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            7000, // for false
            582,  // for false
            1000, // for false
            1327, // for false
            1590, // for false
            1810, // for false
            2000, // for false
            2167, // for false
            2327, // for false
            2445, // for false
            2573, // for false
            2714, // for false
            2790, // for false
            2912, // for false
            3000, // for false
            3093, // for false
            3192, // for false
            3245, // for false
            3299, // for false
            3415, // for false
            3476, // for false
            3540, // for false
            3607, // for false
            3678, // for false
            3678, // for false
            3752, // for false
            3830, // for false
            3830, // for false
            3912, // for false
            3912, // for false
            4000, // for false
            4000, // for false
            4093, // for false
            4093, // for false
            4192, // for false
            4192, // for false
            4192, // for false
            4299, // for false
            4299, // for false
            4299, // for false
            4415, // for false
            4415, // for false
            4415, // for false
            4540, // for false
            4540, // for false
            4540, // for false
            4540, // for false
            4678, // for false
            4678, // for false
            4678, // for false
            4678, // for false
            4678, // for false
            4678, // for false
            4830, // for false
            4830, // for false
            4830, // for false
            4830, // for false
            4830, // for false
            4830, // for false
            5000, // for false
            5000, // for false
            5000, // for false
            5000, // for false
            5000, // for false
            5000, // for false
            5000, // for false
            5000, // for false
            415,  // for false
            733,  // for false
            1000, // for false
            1218, // for false
            1415, // for false
            1590, // for false
            1733, // for false
            1870, // for false
            2000, // for false
            2117, // for false
            2218, // for false
            2327, // for false
            2415, // for false
            2508, // for false
            2573, // for false
            2678, // for false
            2752, // for false
            2790, // for false
            2870, // for false
            2955, // for false
            3000, // for false
            3045, // for false
            3093, // for false
            3192, // for false
            3245, // for false
            3299, // for false
            3299, // for false
            3356, // for false
            3415, // for false
            3476, // for false
            3476, // for false
            3540, // for false
            3607, // for false
            3607, // for false
            3678, // for false
            3678, // for false
            3752, // for false
            3752, // for false
            3830, // for false
            3830, // for false
            3912, // for false
            3912, // for false
            3912, // for false
            4000, // for false
            320,  // for false
            582,  // for false
            810,  // for false
            1000, // for false
            1167, // for false
            1327, // for false
            1460, // for false
            1590, // for false
            1696, // for false
            1810, // for false
            1912, // for false
            2000, // for false
            2093, // for false
            2167, // for false
            2245, // for false
            2327, // for false
            2385, // for false
            2445, // for false
            2508, // for false
            2573, // for false
            2642, // for false
            2714, // for false
            2752, // for false
            2790, // for false
            2870, // for false
            2912, // for false
            2955, // for false
            3000, // for false
            3045, // for false
            3093, // for false
            3142, // for false
            3192, // for false
            3192, // for false
            258,  // for false
            484,  // for false
            678,  // for false
            850,  // for false
            1000, // for false
            1142, // for false
            1258, // for false
            1385, // for false
            1492, // for false
            1590, // for false
            1678, // for false
            1771, // for false
            1850, // for false
            1933, // for false
            2000, // for false
            2069, // for false
            2142, // for false
            2192, // for false
            2272, // for false
            2327, // for false
            2385, // for false
            2445, // for false
            2476, // for false
            2540, // for false
            2573, // for false
            2642, // for false
            218,  // for false
            415,  // for false
            582,  // for false
            733,  // for false
            870,  // for false
            1000, // for false
            1117, // for false
            1218, // for false
            1327, // for false
            1415, // for false
            1508, // for false
            1590, // for false
            1660, // for false
            1733, // for false
            1810, // for false
            1870, // for false
            1933, // for false
            2000, // for false
            2069, // for false
            2117, // for false
            2167, // for false
            2218, // for false
            192,  // for false
            363,  // for false
            516,  // for false
            651,  // for false
            780,  // for false
            891,  // for false
            1000, // for false
            1105, // for false
            1192, // for false
            1285, // for false
            1356, // for false
            1445, // for false
            1508, // for false
            1590, // for false
            1660, // for false
            1714, // for false
            1771, // for false
            1830, // for false
            1891, // for false
            167,  // for false
            320,  // for false
            460,  // for false
            582,  // for false
            696,  // for false
            810,  // for false
            901,  // for false
            1000, // for false
            1093, // for false
            1167, // for false
            1245, // for false
            1327, // for false
            1385, // for false
            1460, // for false
            1524, // for false
            1590, // for false
            148,  // for false
            292,  // for false
            415,  // for false
            532,  // for false
            633,  // for false
            733,  // for false
            830,  // for false
            912,  // for false
            1000, // for false
            1081, // for false
            1154, // for false
            1218, // for false
            1285, // for false
            1356, // for false
            135,  // for false
            265,  // for false
            377,  // for false
            484,  // for false
            582,  // for false
            678,  // for false
            761,  // for false
            850,  // for false
            923,  // for false
            1000, // for false
            1069, // for false
            1142, // for false
            1205, // for false
            123,  // for false
            238,  // for false
            348,  // for false
            445,  // for false
            540,  // for false
            624,  // for false
            714,  // for false
            790,  // for false
            860,  // for false
            933,  // for false
            1000, // for false
            1069, // for false
            111,  // for false
            225,  // for false
            320,  // for false
            415,  // for false
            500,  // for false
            582,  // for false
            660,  // for false
            733,  // for false
            810,  // for false
            870,  // for false
            933,  // for false
            105,  // for false
            205,  // for false
            299,  // for false
            385,  // for false
            468,  // for false
            548,  // for false
            624,  // for false
            696,  // for false
            761,  // for false
            820,  // for false
            99,   // for false
            192,  // for false
            278,  // for false
            363,  // for false
            437,  // for false
            516,  // for false
            582,  // for false
            651,  // for false
            714,  // for false
            93,   // for false
            179,  // for false
            265,  // for false
            341,  // for false
            415,  // for false
            484,  // for false
            548,  // for false
            616,  // for false
            87,   // for false
            167,  // for false
            245,  // for false
            320,  // for false
            392,  // for false
            460,  // for false
            524,  // for false
            582,  // for false
            81,   // for false
            160,  // for false
            231,  // for false
            306,  // for false
            370,  // for false
            437,  // for false
            500,  // for false
            75,   // for false
            154,  // for false
            225,  // for false
            292,  // for false
            356,  // for false
            415,  // for false
            476,  // for false
            69,   // for false
            142,  // for false
            212,  // for false
            278,  // for false
            334,  // for false
            392,  // for false
            453,  // for false
            69,   // for false
            135,  // for false
            199,  // for false
            265,  // for false
            320,  // for false
            377,  // for false
            63,   // for false
            129,  // for false
            192,  // for false
            251,  // for false
            306,  // for false
            363,  // for false
            63,   // for false
            123,  // for false
            186,  // for false
            238,  // for false
            292,  // for false
            348,  // for false
            57,   // for false
            117,  // for false
            179,  // for false
            231,  // for false
            285,  // for false
            57,   // for false
            117,  // for false
            167,  // for false
            225,  // for false
            272,  // for false
            51,   // for false
            111,  // for false
            160,  // for false
            212,  // for false
            265,  // for false
            51,   // for false
            105,  // for false
            154,  // for false
            205,  // for false
            251,  // for false
            51,   // for false
            105,  // for false
            154,  // for false
            199,  // for false
            45,   // for false
            99,   // for false
            148,  // for false
            192,  // for false
            45,   // for false
            99,   // for false
            142,  // for false
            186,  // for false
            45,   // for false
            93,   // for false
            135,  // for false
            179,  // for false
            45,   // for false
            93,   // for false
            135,  // for false
            173,  // for false
            39,   // for false
            87,   // for false
            129,  // for false
            167,  // for false
            39,   // for false
            87,   // for false
            123,  // for false
            167,  // for false
            39,   // for false
            81,   // for false
            123,  // for false
            39,   // for false
            81,   // for false
            117,  // for false
            34,   // for false
            81,   // for false
            117,  // for false
            34,   // for false
            75,   // for false
            111,  // for false
            34,   // for false
            75,   // for false
            111,  // for false
            34,   // for false
            75,   // for false
            105,  // for false
            34,   // for false
            69,   // for false
            105,  // for false
            34,   // for false
            69,   // for false
            99,   // for false
            28,   // for false
            69,   // for false
            99,   // for false
            28,   // for false
            63,   // for false
            99,   // for false
            28,   // for false
            63,   // for false
            93,   // for false
            28,   // for false
            63,   // for false
            28,   // for false
            63,   // for false
            28,   // for false
            57,   // for false
            28,   // for false
            57,   // for false
            28,   // for false
            57,   // for false
            28,   // for false
            57,   // for false
            22,   // for false
            57,   // for false
            22,   // for false
            57,   // for false
            22,   // for false
            51,   // for false
            22,   // for false
            51,   // for false
            22,   // for false
            51,   // for false
            22,   // for false
            51,   // for false
            22,   // for false
            51,   // for false
            22,   // for false
            51,   // for false
            22,   // for false
            45,   // for false
            22,   // for false
            45,   // for false
            22,   // for false
            45,   // for false
            22,   // for false
            45,   // for false
            22,   // for false
            45,   // for false
            17,   // for false
            45,   // for false
            17,   // for false
            45,   // for false
            17,   // for false
            45,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            17,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            11,   // for false
            1000, // for true
            582,  // for true
            415,  // for true
            320,  // for true
            258,  // for true
            218,  // for true
            192,  // for true
            167,  // for true
            148,  // for true
            135,  // for true
            123,  // for true
            111,  // for true
            105,  // for true
            99,   // for true
            93,   // for true
            87,   // for true
            81,   // for true
            75,   // for true
            69,   // for true
            69,   // for true
            63,   // for true
            63,   // for true
            57,   // for true
            57,   // for true
            51,   // for true
            51,   // for true
            51,   // for true
            45,   // for true
            45,   // for true
            45,   // for true
            45,   // for true
            39,   // for true
            39,   // for true
            39,   // for true
            39,   // for true
            34,   // for true
            34,   // for true
            34,   // for true
            34,   // for true
            34,   // for true
            34,   // for true
            28,   // for true
            28,   // for true
            28,   // for true
            28,   // for true
            28,   // for true
            28,   // for true
            28,   // for true
            28,   // for true
            28,   // for true
            22,   // for true
            22,   // for true
            22,   // for true
            22,   // for true
            22,   // for true
            22,   // for true
            22,   // for true
            22,   // for true
            22,   // for true
            22,   // for true
            22,   // for true
            22,   // for true
            22,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            17,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            11,   // for true
            1590, // for true
            1000, // for true
            733,  // for true
            582,  // for true
            484,  // for true
            415,  // for true
            363,  // for true
            320,  // for true
            292,  // for true
            265,  // for true
            238,  // for true
            225,  // for true
            205,  // for true
            192,  // for true
            179,  // for true
            167,  // for true
            160,  // for true
            154,  // for true
            142,  // for true
            135,  // for true
            129,  // for true
            123,  // for true
            117,  // for true
            117,  // for true
            111,  // for true
            105,  // for true
            105,  // for true
            99,   // for true
            99,   // for true
            93,   // for true
            93,   // for true
            87,   // for true
            87,   // for true
            81,   // for true
            81,   // for true
            81,   // for true
            75,   // for true
            75,   // for true
            75,   // for true
            69,   // for true
            69,   // for true
            69,   // for true
            63,   // for true
            63,   // for true
            63,   // for true
            63,   // for true
            57,   // for true
            57,   // for true
            57,   // for true
            57,   // for true
            57,   // for true
            57,   // for true
            51,   // for true
            51,   // for true
            51,   // for true
            51,   // for true
            51,   // for true
            51,   // for true
            45,   // for true
            45,   // for true
            45,   // for true
            45,   // for true
            45,   // for true
            45,   // for true
            45,   // for true
            45,   // for true
            2000, // for true
            1327, // for true
            1000, // for true
            810,  // for true
            678,  // for true
            582,  // for true
            516,  // for true
            460,  // for true
            415,  // for true
            377,  // for true
            348,  // for true
            320,  // for true
            299,  // for true
            278,  // for true
            265,  // for true
            245,  // for true
            231,  // for true
            225,  // for true
            212,  // for true
            199,  // for true
            192,  // for true
            186,  // for true
            179,  // for true
            167,  // for true
            160,  // for true
            154,  // for true
            154,  // for true
            148,  // for true
            142,  // for true
            135,  // for true
            135,  // for true
            129,  // for true
            123,  // for true
            123,  // for true
            117,  // for true
            117,  // for true
            111,  // for true
            111,  // for true
            105,  // for true
            105,  // for true
            99,   // for true
            99,   // for true
            99,   // for true
            93,   // for true
            2327, // for true
            1590, // for true
            1218, // for true
            1000, // for true
            850,  // for true
            733,  // for true
            651,  // for true
            582,  // for true
            532,  // for true
            484,  // for true
            445,  // for true
            415,  // for true
            385,  // for true
            363,  // for true
            341,  // for true
            320,  // for true
            306,  // for true
            292,  // for true
            278,  // for true
            265,  // for true
            251,  // for true
            238,  // for true
            231,  // for true
            225,  // for true
            212,  // for true
            205,  // for true
            199,  // for true
            192,  // for true
            186,  // for true
            179,  // for true
            173,  // for true
            167,  // for true
            167,  // for true
            2607, // for true
            1810, // for true
            1415, // for true
            1167, // for true
            1000, // for true
            870,  // for true
            780,  // for true
            696,  // for true
            633,  // for true
            582,  // for true
            540,  // for true
            500,  // for true
            468,  // for true
            437,  // for true
            415,  // for true
            392,  // for true
            370,  // for true
            356,  // for true
            334,  // for true
            320,  // for true
            306,  // for true
            292,  // for true
            285,  // for true
            272,  // for true
            265,  // for true
            251,  // for true
            2830, // for true
            2000, // for true
            1590, // for true
            1327, // for true
            1142, // for true
            1000, // for true
            891,  // for true
            810,  // for true
            733,  // for true
            678,  // for true
            624,  // for true
            582,  // for true
            548,  // for true
            516,  // for true
            484,  // for true
            460,  // for true
            437,  // for true
            415,  // for true
            392,  // for true
            377,  // for true
            363,  // for true
            348,  // for true
            3000, // for true
            2167, // for true
            1733, // for true
            1460, // for true
            1258, // for true
            1117, // for true
            1000, // for true
            901,  // for true
            830,  // for true
            761,  // for true
            714,  // for true
            660,  // for true
            624,  // for true
            582,  // for true
            548,  // for true
            524,  // for true
            500,  // for true
            476,  // for true
            453,  // for true
            3192, // for true
            2327, // for true
            1870, // for true
            1590, // for true
            1385, // for true
            1218, // for true
            1105, // for true
            1000, // for true
            912,  // for true
            850,  // for true
            790,  // for true
            733,  // for true
            696,  // for true
            651,  // for true
            616,  // for true
            582,  // for true
            3356, // for true
            2445, // for true
            2000, // for true
            1696, // for true
            1492, // for true
            1327, // for true
            1192, // for true
            1093, // for true
            1000, // for true
            923,  // for true
            860,  // for true
            810,  // for true
            761,  // for true
            714,  // for true
            3476, // for true
            2573, // for true
            2117, // for true
            1810, // for true
            1590, // for true
            1415, // for true
            1285, // for true
            1167, // for true
            1081, // for true
            1000, // for true
            933,  // for true
            870,  // for true
            820,  // for true
            3607, // for true
            2714, // for true
            2218, // for true
            1912, // for true
            1678, // for true
            1508, // for true
            1356, // for true
            1245, // for true
            1154, // for true
            1069, // for true
            1000, // for true
            933,  // for true
            3752, // for true
            2790, // for true
            2327, // for true
            2000, // for true
            1771, // for true
            1590, // for true
            1445, // for true
            1327, // for true
            1218, // for true
            1142, // for true
            1069, // for true
            3830, // for true
            2912, // for true
            2415, // for true
            2093, // for true
            1850, // for true
            1660, // for true
            1508, // for true
            1385, // for true
            1285, // for true
            1205, // for true
            3912, // for true
            3000, // for true
            2508, // for true
            2167, // for true
            1933, // for true
            1733, // for true
            1590, // for true
            1460, // for true
            1356, // for true
            4000, // for true
            3093, // for true
            2573, // for true
            2245, // for true
            2000, // for true
            1810, // for true
            1660, // for true
            1524, // for true
            4093, // for true
            3192, // for true
            2678, // for true
            2327, // for true
            2069, // for true
            1870, // for true
            1714, // for true
            1590, // for true
            4192, // for true
            3245, // for true
            2752, // for true
            2385, // for true
            2142, // for true
            1933, // for true
            1771, // for true
            4299, // for true
            3299, // for true
            2790, // for true
            2445, // for true
            2192, // for true
            2000, // for true
            1830, // for true
            4415, // for true
            3415, // for true
            2870, // for true
            2508, // for true
            2272, // for true
            2069, // for true
            1891, // for true
            4415, // for true
            3476, // for true
            2955, // for true
            2573, // for true
            2327, // for true
            2117, // for true
            4540, // for true
            3540, // for true
            3000, // for true
            2642, // for true
            2385, // for true
            2167, // for true
            4540, // for true
            3607, // for true
            3045, // for true
            2714, // for true
            2445, // for true
            2218, // for true
            4678, // for true
            3678, // for true
            3093, // for true
            2752, // for true
            2476, // for true
            4678, // for true
            3678, // for true
            3192, // for true
            2790, // for true
            2540, // for true
            4830, // for true
            3752, // for true
            3245, // for true
            2870, // for true
            2573, // for true
            4830, // for true
            3830, // for true
            3299, // for true
            2912, // for true
            2642, // for true
            4830, // for true
            3830, // for true
            3299, // for true
            2955, // for true
            5000, // for true
            3912, // for true
            3356, // for true
            3000, // for true
            5000, // for true
            3912, // for true
            3415, // for true
            3045, // for true
            5000, // for true
            4000, // for true
            3476, // for true
            3093, // for true
            5000, // for true
            4000, // for true
            3476, // for true
            3142, // for true
            5192, // for true
            4093, // for true
            3540, // for true
            3192, // for true
            5192, // for true
            4093, // for true
            3607, // for true
            3192, // for true
            5192, // for true
            4192, // for true
            3607, // for true
            5192, // for true
            4192, // for true
            3678, // for true
            5415, // for true
            4192, // for true
            3678, // for true
            5415, // for true
            4299, // for true
            3752, // for true
            5415, // for true
            4299, // for true
            3752, // for true
            5415, // for true
            4299, // for true
            3830, // for true
            5415, // for true
            4415, // for true
            3830, // for true
            5415, // for true
            4415, // for true
            3912, // for true
            5678, // for true
            4415, // for true
            3912, // for true
            5678, // for true
            4540, // for true
            3912, // for true
            5678, // for true
            4540, // for true
            4000, // for true
            5678, // for true
            4540, // for true
            5678, // for true
            4540, // for true
            5678, // for true
            4678, // for true
            5678, // for true
            4678, // for true
            5678, // for true
            4678, // for true
            5678, // for true
            4678, // for true
            6000, // for true
            4678, // for true
            6000, // for true
            4678, // for true
            6000, // for true
            4830, // for true
            6000, // for true
            4830, // for true
            6000, // for true
            4830, // for true
            6000, // for true
            4830, // for true
            6000, // for true
            4830, // for true
            6000, // for true
            4830, // for true
            6000, // for true
            5000, // for true
            6000, // for true
            5000, // for true
            6000, // for true
            5000, // for true
            6000, // for true
            5000, // for true
            6000, // for true
            5000, // for true
            6415, // for true
            5000, // for true
            6415, // for true
            5000, // for true
            6415, // for true
            5000, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            6415, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
            7000, // for true
        ];
        let out = LOOKUP[*self as usize * (1 + bit as usize)];
        *self = self.adapt(bit);
        out
    }
}
// Count of variants: 675
