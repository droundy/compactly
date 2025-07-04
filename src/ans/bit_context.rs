//! Generated with `src/ans/generate_bit_context.rs`
use super::ans::Probability;

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
            True0False1,   // from True0False0 with false
            True1False1,   // from True1False0 with false
            True2False1,   // from True2False0 with false
            True3False1,   // from True3False0 with false
            True4False1,   // from True4False0 with false
            True5False1,   // from True5False0 with false
            True6False1,   // from True6False0 with false
            True7False1,   // from True7False0 with false
            True8False1,   // from True8False0 with false
            True9False1,   // from True9False0 with false
            True10False1,  // from True10False0 with false
            True11False1,  // from True11False0 with false
            True12False1,  // from True12False0 with false
            True13False1,  // from True13False0 with false
            True14False1,  // from True14False0 with false
            True15False1,  // from True15False0 with false
            True16False1,  // from True16False0 with false
            True17False1,  // from True17False0 with false
            True18False1,  // from True18False0 with false
            True19False1,  // from True19False0 with false
            True20False1,  // from True20False0 with false
            True21False1,  // from True21False0 with false
            True22False1,  // from True22False0 with false
            True23False1,  // from True23False0 with false
            True24False1,  // from True24False0 with false
            True25False1,  // from True25False0 with false
            True26False1,  // from True26False0 with false
            True27False1,  // from True27False0 with false
            True28False1,  // from True28False0 with false
            True29False1,  // from True29False0 with false
            True30False1,  // from True30False0 with false
            True31False1,  // from True31False0 with false
            True32False1,  // from True32False0 with false
            True33False1,  // from True33False0 with false
            True34False1,  // from True34False0 with false
            True35False1,  // from True35False0 with false
            True36False1,  // from True36False0 with false
            True37False1,  // from True37False0 with false
            True38False1,  // from True38False0 with false
            True39False1,  // from True39False0 with false
            True40False1,  // from True40False0 with false
            True41False1,  // from True41False0 with false
            True42False1,  // from True42False0 with false
            True43False1,  // from True43False0 with false
            True44False1,  // from True44False0 with false
            True45False1,  // from True45False0 with false
            True46False1,  // from True46False0 with false
            True47False1,  // from True47False0 with false
            True48False1,  // from True48False0 with false
            True49False1,  // from True49False0 with false
            True50False1,  // from True50False0 with false
            True51False1,  // from True51False0 with false
            True52False1,  // from True52False0 with false
            True53False1,  // from True53False0 with false
            True54False1,  // from True54False0 with false
            True55False1,  // from True55False0 with false
            True56False1,  // from True56False0 with false
            True57False1,  // from True57False0 with false
            True58False1,  // from True58False0 with false
            True59False1,  // from True59False0 with false
            True60False1,  // from True60False0 with false
            True61False1,  // from True61False0 with false
            True62False1,  // from True62False0 with false
            True63False1,  // from True63False0 with false
            True64False1,  // from True64False0 with false
            True65False1,  // from True65False0 with false
            True33False0,  // from True66False0 with false
            True33False0,  // from True67False0 with false
            True34False0,  // from True68False0 with false
            True34False0,  // from True69False0 with false
            True35False0,  // from True70False0 with false
            True35False0,  // from True71False0 with false
            True36False0,  // from True72False0 with false
            True36False0,  // from True73False0 with false
            True37False0,  // from True74False0 with false
            True37False0,  // from True75False0 with false
            True38False0,  // from True76False0 with false
            True38False0,  // from True77False0 with false
            True39False0,  // from True78False0 with false
            True39False0,  // from True79False0 with false
            True40False0,  // from True80False0 with false
            True40False0,  // from True81False0 with false
            True41False0,  // from True82False0 with false
            True41False0,  // from True83False0 with false
            True42False0,  // from True84False0 with false
            True42False0,  // from True85False0 with false
            True43False0,  // from True86False0 with false
            True43False0,  // from True87False0 with false
            True44False0,  // from True88False0 with false
            True44False0,  // from True89False0 with false
            True45False0,  // from True90False0 with false
            True45False0,  // from True91False0 with false
            True46False0,  // from True92False0 with false
            True46False0,  // from True93False0 with false
            True47False0,  // from True94False0 with false
            True47False0,  // from True95False0 with false
            True48False0,  // from True96False0 with false
            True48False0,  // from True97False0 with false
            True49False0,  // from True98False0 with false
            True49False0,  // from True99False0 with false
            True50False0,  // from True100False0 with false
            True50False0,  // from True101False0 with false
            True51False0,  // from True102False0 with false
            True51False0,  // from True103False0 with false
            True52False0,  // from True104False0 with false
            True52False0,  // from True105False0 with false
            True53False0,  // from True106False0 with false
            True53False0,  // from True107False0 with false
            True54False0,  // from True108False0 with false
            True54False0,  // from True109False0 with false
            True55False0,  // from True110False0 with false
            True55False0,  // from True111False0 with false
            True56False0,  // from True112False0 with false
            True56False0,  // from True113False0 with false
            True57False0,  // from True114False0 with false
            True57False0,  // from True115False0 with false
            True58False0,  // from True116False0 with false
            True58False0,  // from True117False0 with false
            True59False0,  // from True118False0 with false
            True59False0,  // from True119False0 with false
            True60False0,  // from True120False0 with false
            True60False0,  // from True121False0 with false
            True61False0,  // from True122False0 with false
            True61False0,  // from True123False0 with false
            True62False0,  // from True124False0 with false
            True62False0,  // from True125False0 with false
            True63False0,  // from True126False0 with false
            True63False0,  // from True127False0 with false
            True64False0,  // from True128False0 with false
            True64False0,  // from True129False0 with false
            True65False0,  // from True130False0 with false
            True65False0,  // from True131False0 with false
            True66False0,  // from True132False0 with false
            True0False2,   // from True0False1 with false
            True1False2,   // from True1False1 with false
            True2False2,   // from True2False1 with false
            True3False2,   // from True3False1 with false
            True4False2,   // from True4False1 with false
            True5False2,   // from True5False1 with false
            True6False2,   // from True6False1 with false
            True7False2,   // from True7False1 with false
            True8False2,   // from True8False1 with false
            True9False2,   // from True9False1 with false
            True10False2,  // from True10False1 with false
            True11False2,  // from True11False1 with false
            True12False2,  // from True12False1 with false
            True13False2,  // from True13False1 with false
            True14False2,  // from True14False1 with false
            True15False2,  // from True15False1 with false
            True16False2,  // from True16False1 with false
            True17False2,  // from True17False1 with false
            True18False2,  // from True18False1 with false
            True19False2,  // from True19False1 with false
            True20False2,  // from True20False1 with false
            True21False2,  // from True21False1 with false
            True22False2,  // from True22False1 with false
            True23False2,  // from True23False1 with false
            True24False2,  // from True24False1 with false
            True25False2,  // from True25False1 with false
            True26False2,  // from True26False1 with false
            True27False2,  // from True27False1 with false
            True28False2,  // from True28False1 with false
            True29False2,  // from True29False1 with false
            True30False2,  // from True30False1 with false
            True31False2,  // from True31False1 with false
            True32False2,  // from True32False1 with false
            True33False2,  // from True33False1 with false
            True34False2,  // from True34False1 with false
            True35False2,  // from True35False1 with false
            True36False2,  // from True36False1 with false
            True37False2,  // from True37False1 with false
            True38False2,  // from True38False1 with false
            True39False2,  // from True39False1 with false
            True40False2,  // from True40False1 with false
            True41False2,  // from True41False1 with false
            True42False2,  // from True42False1 with false
            True43False2,  // from True43False1 with false
            True22False1,  // from True44False1 with false
            True22False1,  // from True45False1 with false
            True23False1,  // from True46False1 with false
            True23False1,  // from True47False1 with false
            True24False1,  // from True48False1 with false
            True24False1,  // from True49False1 with false
            True25False1,  // from True50False1 with false
            True25False1,  // from True51False1 with false
            True26False1,  // from True52False1 with false
            True26False1,  // from True53False1 with false
            True27False1,  // from True54False1 with false
            True27False1,  // from True55False1 with false
            True28False1,  // from True56False1 with false
            True28False1,  // from True57False1 with false
            True29False1,  // from True58False1 with false
            True29False1,  // from True59False1 with false
            True30False1,  // from True60False1 with false
            True30False1,  // from True61False1 with false
            True31False1,  // from True62False1 with false
            True31False1,  // from True63False1 with false
            True32False1,  // from True64False1 with false
            True32False1,  // from True65False1 with false
            True0False3,   // from True0False2 with false
            True1False3,   // from True1False2 with false
            True2False3,   // from True2False2 with false
            True3False3,   // from True3False2 with false
            True4False3,   // from True4False2 with false
            True5False3,   // from True5False2 with false
            True6False3,   // from True6False2 with false
            True7False3,   // from True7False2 with false
            True8False3,   // from True8False2 with false
            True9False3,   // from True9False2 with false
            True10False3,  // from True10False2 with false
            True11False3,  // from True11False2 with false
            True12False3,  // from True12False2 with false
            True13False3,  // from True13False2 with false
            True14False3,  // from True14False2 with false
            True15False3,  // from True15False2 with false
            True16False3,  // from True16False2 with false
            True17False3,  // from True17False2 with false
            True18False3,  // from True18False2 with false
            True19False3,  // from True19False2 with false
            True20False3,  // from True20False2 with false
            True21False3,  // from True21False2 with false
            True22False3,  // from True22False2 with false
            True23False3,  // from True23False2 with false
            True24False3,  // from True24False2 with false
            True25False3,  // from True25False2 with false
            True26False3,  // from True26False2 with false
            True27False3,  // from True27False2 with false
            True28False3,  // from True28False2 with false
            True29False3,  // from True29False2 with false
            True30False3,  // from True30False2 with false
            True31False3,  // from True31False2 with false
            True32False3,  // from True32False2 with false
            True16False1,  // from True33False2 with false
            True17False1,  // from True34False2 with false
            True17False1,  // from True35False2 with false
            True18False1,  // from True36False2 with false
            True18False1,  // from True37False2 with false
            True19False1,  // from True38False2 with false
            True19False1,  // from True39False2 with false
            True20False1,  // from True40False2 with false
            True20False1,  // from True41False2 with false
            True21False1,  // from True42False2 with false
            True21False1,  // from True43False2 with false
            True0False4,   // from True0False3 with false
            True1False4,   // from True1False3 with false
            True2False4,   // from True2False3 with false
            True3False4,   // from True3False3 with false
            True4False4,   // from True4False3 with false
            True5False4,   // from True5False3 with false
            True6False4,   // from True6False3 with false
            True7False4,   // from True7False3 with false
            True8False4,   // from True8False3 with false
            True9False4,   // from True9False3 with false
            True10False4,  // from True10False3 with false
            True11False4,  // from True11False3 with false
            True12False4,  // from True12False3 with false
            True13False4,  // from True13False3 with false
            True14False4,  // from True14False3 with false
            True15False4,  // from True15False3 with false
            True16False4,  // from True16False3 with false
            True17False4,  // from True17False3 with false
            True18False4,  // from True18False3 with false
            True19False4,  // from True19False3 with false
            True20False4,  // from True20False3 with false
            True21False4,  // from True21False3 with false
            True22False4,  // from True22False3 with false
            True23False4,  // from True23False3 with false
            True24False4,  // from True24False3 with false
            True25False4,  // from True25False3 with false
            True13False2,  // from True26False3 with false
            True13False2,  // from True27False3 with false
            True14False2,  // from True28False3 with false
            True14False2,  // from True29False3 with false
            True15False2,  // from True30False3 with false
            True15False2,  // from True31False3 with false
            True16False2,  // from True32False3 with false
            True0False5,   // from True0False4 with false
            True1False5,   // from True1False4 with false
            True2False5,   // from True2False4 with false
            True3False5,   // from True3False4 with false
            True4False5,   // from True4False4 with false
            True5False5,   // from True5False4 with false
            True6False5,   // from True6False4 with false
            True7False5,   // from True7False4 with false
            True8False5,   // from True8False4 with false
            True9False5,   // from True9False4 with false
            True10False5,  // from True10False4 with false
            True11False5,  // from True11False4 with false
            True12False5,  // from True12False4 with false
            True13False5,  // from True13False4 with false
            True14False5,  // from True14False4 with false
            True15False5,  // from True15False4 with false
            True16False5,  // from True16False4 with false
            True17False5,  // from True17False4 with false
            True18False5,  // from True18False4 with false
            True19False5,  // from True19False4 with false
            True20False5,  // from True20False4 with false
            True21False5,  // from True21False4 with false
            True11False2,  // from True22False4 with false
            True11False2,  // from True23False4 with false
            True12False2,  // from True24False4 with false
            True12False2,  // from True25False4 with false
            True0False6,   // from True0False5 with false
            True1False6,   // from True1False5 with false
            True2False6,   // from True2False5 with false
            True3False6,   // from True3False5 with false
            True4False6,   // from True4False5 with false
            True5False6,   // from True5False5 with false
            True6False6,   // from True6False5 with false
            True7False6,   // from True7False5 with false
            True8False6,   // from True8False5 with false
            True9False6,   // from True9False5 with false
            True10False6,  // from True10False5 with false
            True11False6,  // from True11False5 with false
            True12False6,  // from True12False5 with false
            True13False6,  // from True13False5 with false
            True14False6,  // from True14False5 with false
            True15False6,  // from True15False5 with false
            True16False6,  // from True16False5 with false
            True17False6,  // from True17False5 with false
            True18False6,  // from True18False5 with false
            True9False3,   // from True19False5 with false
            True10False3,  // from True20False5 with false
            True10False3,  // from True21False5 with false
            True0False7,   // from True0False6 with false
            True1False7,   // from True1False6 with false
            True2False7,   // from True2False6 with false
            True3False7,   // from True3False6 with false
            True4False7,   // from True4False6 with false
            True5False7,   // from True5False6 with false
            True6False7,   // from True6False6 with false
            True7False7,   // from True7False6 with false
            True8False7,   // from True8False6 with false
            True9False7,   // from True9False6 with false
            True10False7,  // from True10False6 with false
            True11False7,  // from True11False6 with false
            True12False7,  // from True12False6 with false
            True13False7,  // from True13False6 with false
            True14False7,  // from True14False6 with false
            True15False7,  // from True15False6 with false
            True8False3,   // from True16False6 with false
            True8False3,   // from True17False6 with false
            True9False3,   // from True18False6 with false
            True0False8,   // from True0False7 with false
            True1False8,   // from True1False7 with false
            True2False8,   // from True2False7 with false
            True3False8,   // from True3False7 with false
            True4False8,   // from True4False7 with false
            True5False8,   // from True5False7 with false
            True6False8,   // from True6False7 with false
            True7False8,   // from True7False7 with false
            True8False8,   // from True8False7 with false
            True9False8,   // from True9False7 with false
            True10False8,  // from True10False7 with false
            True11False8,  // from True11False7 with false
            True12False8,  // from True12False7 with false
            True13False8,  // from True13False7 with false
            True7False4,   // from True14False7 with false
            True7False4,   // from True15False7 with false
            True0False9,   // from True0False8 with false
            True1False9,   // from True1False8 with false
            True2False9,   // from True2False8 with false
            True3False9,   // from True3False8 with false
            True4False9,   // from True4False8 with false
            True5False9,   // from True5False8 with false
            True6False9,   // from True6False8 with false
            True7False9,   // from True7False8 with false
            True8False9,   // from True8False8 with false
            True9False9,   // from True9False8 with false
            True10False9,  // from True10False8 with false
            True11False9,  // from True11False8 with false
            True12False9,  // from True12False8 with false
            True6False4,   // from True13False8 with false
            True0False10,  // from True0False9 with false
            True1False10,  // from True1False9 with false
            True2False10,  // from True2False9 with false
            True3False10,  // from True3False9 with false
            True4False10,  // from True4False9 with false
            True5False10,  // from True5False9 with false
            True6False10,  // from True6False9 with false
            True7False10,  // from True7False9 with false
            True8False10,  // from True8False9 with false
            True9False10,  // from True9False9 with false
            True10False10, // from True10False9 with false
            True11False10, // from True11False9 with false
            True6False5,   // from True12False9 with false
            True0False11,  // from True0False10 with false
            True1False11,  // from True1False10 with false
            True2False11,  // from True2False10 with false
            True3False11,  // from True3False10 with false
            True4False11,  // from True4False10 with false
            True5False11,  // from True5False10 with false
            True6False11,  // from True6False10 with false
            True7False11,  // from True7False10 with false
            True8False11,  // from True8False10 with false
            True9False11,  // from True9False10 with false
            True10False11, // from True10False10 with false
            True5False5,   // from True11False10 with false
            True0False12,  // from True0False11 with false
            True1False12,  // from True1False11 with false
            True2False12,  // from True2False11 with false
            True3False12,  // from True3False11 with false
            True4False12,  // from True4False11 with false
            True5False12,  // from True5False11 with false
            True6False12,  // from True6False11 with false
            True7False12,  // from True7False11 with false
            True8False12,  // from True8False11 with false
            True9False12,  // from True9False11 with false
            True5False6,   // from True10False11 with false
            True0False13,  // from True0False12 with false
            True1False13,  // from True1False12 with false
            True2False13,  // from True2False12 with false
            True3False13,  // from True3False12 with false
            True4False13,  // from True4False12 with false
            True5False13,  // from True5False12 with false
            True6False13,  // from True6False12 with false
            True7False13,  // from True7False12 with false
            True8False13,  // from True8False12 with false
            True4False6,   // from True9False12 with false
            True0False14,  // from True0False13 with false
            True1False14,  // from True1False13 with false
            True2False14,  // from True2False13 with false
            True3False14,  // from True3False13 with false
            True4False14,  // from True4False13 with false
            True5False14,  // from True5False13 with false
            True6False14,  // from True6False13 with false
            True7False14,  // from True7False13 with false
            True4False7,   // from True8False13 with false
            True0False15,  // from True0False14 with false
            True1False15,  // from True1False14 with false
            True2False15,  // from True2False14 with false
            True3False15,  // from True3False14 with false
            True4False15,  // from True4False14 with false
            True5False15,  // from True5False14 with false
            True6False15,  // from True6False14 with false
            True7False15,  // from True7False14 with false
            True0False16,  // from True0False15 with false
            True1False16,  // from True1False15 with false
            True2False16,  // from True2False15 with false
            True3False16,  // from True3False15 with false
            True4False16,  // from True4False15 with false
            True5False16,  // from True5False15 with false
            True6False16,  // from True6False15 with false
            True3False8,   // from True7False15 with false
            True0False17,  // from True0False16 with false
            True1False17,  // from True1False16 with false
            True2False17,  // from True2False16 with false
            True3False17,  // from True3False16 with false
            True4False17,  // from True4False16 with false
            True5False17,  // from True5False16 with false
            True6False17,  // from True6False16 with false
            True0False18,  // from True0False17 with false
            True1False18,  // from True1False17 with false
            True2False18,  // from True2False17 with false
            True3False18,  // from True3False17 with false
            True4False18,  // from True4False17 with false
            True5False18,  // from True5False17 with false
            True6False18,  // from True6False17 with false
            True0False19,  // from True0False18 with false
            True1False19,  // from True1False18 with false
            True2False19,  // from True2False18 with false
            True3False19,  // from True3False18 with false
            True4False19,  // from True4False18 with false
            True5False19,  // from True5False18 with false
            True3False9,   // from True6False18 with false
            True0False20,  // from True0False19 with false
            True1False20,  // from True1False19 with false
            True2False20,  // from True2False19 with false
            True3False20,  // from True3False19 with false
            True4False20,  // from True4False19 with false
            True5False20,  // from True5False19 with false
            True0False21,  // from True0False20 with false
            True1False21,  // from True1False20 with false
            True2False21,  // from True2False20 with false
            True3False21,  // from True3False20 with false
            True4False21,  // from True4False20 with false
            True5False21,  // from True5False20 with false
            True0False22,  // from True0False21 with false
            True1False22,  // from True1False21 with false
            True2False22,  // from True2False21 with false
            True3False22,  // from True3False21 with false
            True4False22,  // from True4False21 with false
            True2False11,  // from True5False21 with false
            True0False23,  // from True0False22 with false
            True1False23,  // from True1False22 with false
            True2False23,  // from True2False22 with false
            True3False23,  // from True3False22 with false
            True4False23,  // from True4False22 with false
            True0False24,  // from True0False23 with false
            True1False24,  // from True1False23 with false
            True2False24,  // from True2False23 with false
            True3False24,  // from True3False23 with false
            True4False24,  // from True4False23 with false
            True0False25,  // from True0False24 with false
            True1False25,  // from True1False24 with false
            True2False25,  // from True2False24 with false
            True3False25,  // from True3False24 with false
            True4False25,  // from True4False24 with false
            True0False26,  // from True0False25 with false
            True1False26,  // from True1False25 with false
            True2False26,  // from True2False25 with false
            True3False26,  // from True3False25 with false
            True2False13,  // from True4False25 with false
            True0False27,  // from True0False26 with false
            True1False27,  // from True1False26 with false
            True2False27,  // from True2False26 with false
            True3False27,  // from True3False26 with false
            True0False28,  // from True0False27 with false
            True1False28,  // from True1False27 with false
            True2False28,  // from True2False27 with false
            True3False28,  // from True3False27 with false
            True0False29,  // from True0False28 with false
            True1False29,  // from True1False28 with false
            True2False29,  // from True2False28 with false
            True3False29,  // from True3False28 with false
            True0False30,  // from True0False29 with false
            True1False30,  // from True1False29 with false
            True2False30,  // from True2False29 with false
            True3False30,  // from True3False29 with false
            True0False31,  // from True0False30 with false
            True1False31,  // from True1False30 with false
            True2False31,  // from True2False30 with false
            True3False31,  // from True3False30 with false
            True0False32,  // from True0False31 with false
            True1False32,  // from True1False31 with false
            True2False32,  // from True2False31 with false
            True3False32,  // from True3False31 with false
            True0False33,  // from True0False32 with false
            True1False33,  // from True1False32 with false
            True2False33,  // from True2False32 with false
            True1False16,  // from True3False32 with false
            True0False34,  // from True0False33 with false
            True1False34,  // from True1False33 with false
            True2False34,  // from True2False33 with false
            True0False35,  // from True0False34 with false
            True1False35,  // from True1False34 with false
            True2False35,  // from True2False34 with false
            True0False36,  // from True0False35 with false
            True1False36,  // from True1False35 with false
            True2False36,  // from True2False35 with false
            True0False37,  // from True0False36 with false
            True1False37,  // from True1False36 with false
            True2False37,  // from True2False36 with false
            True0False38,  // from True0False37 with false
            True1False38,  // from True1False37 with false
            True2False38,  // from True2False37 with false
            True0False39,  // from True0False38 with false
            True1False39,  // from True1False38 with false
            True2False39,  // from True2False38 with false
            True0False40,  // from True0False39 with false
            True1False40,  // from True1False39 with false
            True2False40,  // from True2False39 with false
            True0False41,  // from True0False40 with false
            True1False41,  // from True1False40 with false
            True2False41,  // from True2False40 with false
            True0False42,  // from True0False41 with false
            True1False42,  // from True1False41 with false
            True2False42,  // from True2False41 with false
            True0False43,  // from True0False42 with false
            True1False43,  // from True1False42 with false
            True2False43,  // from True2False42 with false
            True0False44,  // from True0False43 with false
            True1False44,  // from True1False43 with false
            True1False22,  // from True2False43 with false
            True0False45,  // from True0False44 with false
            True1False45,  // from True1False44 with false
            True0False46,  // from True0False45 with false
            True1False46,  // from True1False45 with false
            True0False47,  // from True0False46 with false
            True1False47,  // from True1False46 with false
            True0False48,  // from True0False47 with false
            True1False48,  // from True1False47 with false
            True0False49,  // from True0False48 with false
            True1False49,  // from True1False48 with false
            True0False50,  // from True0False49 with false
            True1False50,  // from True1False49 with false
            True0False51,  // from True0False50 with false
            True1False51,  // from True1False50 with false
            True0False52,  // from True0False51 with false
            True1False52,  // from True1False51 with false
            True0False53,  // from True0False52 with false
            True1False53,  // from True1False52 with false
            True0False54,  // from True0False53 with false
            True1False54,  // from True1False53 with false
            True0False55,  // from True0False54 with false
            True1False55,  // from True1False54 with false
            True0False56,  // from True0False55 with false
            True1False56,  // from True1False55 with false
            True0False57,  // from True0False56 with false
            True1False57,  // from True1False56 with false
            True0False58,  // from True0False57 with false
            True1False58,  // from True1False57 with false
            True0False59,  // from True0False58 with false
            True1False59,  // from True1False58 with false
            True0False60,  // from True0False59 with false
            True1False60,  // from True1False59 with false
            True0False61,  // from True0False60 with false
            True1False61,  // from True1False60 with false
            True0False62,  // from True0False61 with false
            True1False62,  // from True1False61 with false
            True0False63,  // from True0False62 with false
            True1False63,  // from True1False62 with false
            True0False64,  // from True0False63 with false
            True1False64,  // from True1False63 with false
            True0False65,  // from True0False64 with false
            True1False65,  // from True1False64 with false
            True0False66,  // from True0False65 with false
            True0False33,  // from True1False65 with false
            True0False67,  // from True0False66 with false
            True0False68,  // from True0False67 with false
            True0False69,  // from True0False68 with false
            True0False70,  // from True0False69 with false
            True0False71,  // from True0False70 with false
            True0False72,  // from True0False71 with false
            True0False73,  // from True0False72 with false
            True0False74,  // from True0False73 with false
            True0False75,  // from True0False74 with false
            True0False76,  // from True0False75 with false
            True0False77,  // from True0False76 with false
            True0False78,  // from True0False77 with false
            True0False79,  // from True0False78 with false
            True0False80,  // from True0False79 with false
            True0False81,  // from True0False80 with false
            True0False82,  // from True0False81 with false
            True0False83,  // from True0False82 with false
            True0False84,  // from True0False83 with false
            True0False85,  // from True0False84 with false
            True0False86,  // from True0False85 with false
            True0False87,  // from True0False86 with false
            True0False88,  // from True0False87 with false
            True0False89,  // from True0False88 with false
            True0False90,  // from True0False89 with false
            True0False91,  // from True0False90 with false
            True0False92,  // from True0False91 with false
            True0False93,  // from True0False92 with false
            True0False94,  // from True0False93 with false
            True0False95,  // from True0False94 with false
            True0False96,  // from True0False95 with false
            True0False97,  // from True0False96 with false
            True0False98,  // from True0False97 with false
            True0False99,  // from True0False98 with false
            True0False100, // from True0False99 with false
            True0False101, // from True0False100 with false
            True0False102, // from True0False101 with false
            True0False103, // from True0False102 with false
            True0False104, // from True0False103 with false
            True0False105, // from True0False104 with false
            True0False106, // from True0False105 with false
            True0False107, // from True0False106 with false
            True0False108, // from True0False107 with false
            True0False109, // from True0False108 with false
            True0False110, // from True0False109 with false
            True0False111, // from True0False110 with false
            True0False112, // from True0False111 with false
            True0False113, // from True0False112 with false
            True0False114, // from True0False113 with false
            True0False115, // from True0False114 with false
            True0False116, // from True0False115 with false
            True0False117, // from True0False116 with false
            True0False118, // from True0False117 with false
            True0False119, // from True0False118 with false
            True0False120, // from True0False119 with false
            True0False121, // from True0False120 with false
            True0False122, // from True0False121 with false
            True0False123, // from True0False122 with false
            True0False124, // from True0False123 with false
            True0False125, // from True0False124 with false
            True0False126, // from True0False125 with false
            True0False127, // from True0False126 with false
            True0False128, // from True0False127 with false
            True0False129, // from True0False128 with false
            True0False130, // from True0False129 with false
            True0False131, // from True0False130 with false
            True0False132, // from True0False131 with false
            True0False132, // from True0False132 with false
            True1False0,   // from True0False0 with true
            True2False0,   // from True1False0 with true
            True3False0,   // from True2False0 with true
            True4False0,   // from True3False0 with true
            True5False0,   // from True4False0 with true
            True6False0,   // from True5False0 with true
            True7False0,   // from True6False0 with true
            True8False0,   // from True7False0 with true
            True9False0,   // from True8False0 with true
            True10False0,  // from True9False0 with true
            True11False0,  // from True10False0 with true
            True12False0,  // from True11False0 with true
            True13False0,  // from True12False0 with true
            True14False0,  // from True13False0 with true
            True15False0,  // from True14False0 with true
            True16False0,  // from True15False0 with true
            True17False0,  // from True16False0 with true
            True18False0,  // from True17False0 with true
            True19False0,  // from True18False0 with true
            True20False0,  // from True19False0 with true
            True21False0,  // from True20False0 with true
            True22False0,  // from True21False0 with true
            True23False0,  // from True22False0 with true
            True24False0,  // from True23False0 with true
            True25False0,  // from True24False0 with true
            True26False0,  // from True25False0 with true
            True27False0,  // from True26False0 with true
            True28False0,  // from True27False0 with true
            True29False0,  // from True28False0 with true
            True30False0,  // from True29False0 with true
            True31False0,  // from True30False0 with true
            True32False0,  // from True31False0 with true
            True33False0,  // from True32False0 with true
            True34False0,  // from True33False0 with true
            True35False0,  // from True34False0 with true
            True36False0,  // from True35False0 with true
            True37False0,  // from True36False0 with true
            True38False0,  // from True37False0 with true
            True39False0,  // from True38False0 with true
            True40False0,  // from True39False0 with true
            True41False0,  // from True40False0 with true
            True42False0,  // from True41False0 with true
            True43False0,  // from True42False0 with true
            True44False0,  // from True43False0 with true
            True45False0,  // from True44False0 with true
            True46False0,  // from True45False0 with true
            True47False0,  // from True46False0 with true
            True48False0,  // from True47False0 with true
            True49False0,  // from True48False0 with true
            True50False0,  // from True49False0 with true
            True51False0,  // from True50False0 with true
            True52False0,  // from True51False0 with true
            True53False0,  // from True52False0 with true
            True54False0,  // from True53False0 with true
            True55False0,  // from True54False0 with true
            True56False0,  // from True55False0 with true
            True57False0,  // from True56False0 with true
            True58False0,  // from True57False0 with true
            True59False0,  // from True58False0 with true
            True60False0,  // from True59False0 with true
            True61False0,  // from True60False0 with true
            True62False0,  // from True61False0 with true
            True63False0,  // from True62False0 with true
            True64False0,  // from True63False0 with true
            True65False0,  // from True64False0 with true
            True66False0,  // from True65False0 with true
            True67False0,  // from True66False0 with true
            True68False0,  // from True67False0 with true
            True69False0,  // from True68False0 with true
            True70False0,  // from True69False0 with true
            True71False0,  // from True70False0 with true
            True72False0,  // from True71False0 with true
            True73False0,  // from True72False0 with true
            True74False0,  // from True73False0 with true
            True75False0,  // from True74False0 with true
            True76False0,  // from True75False0 with true
            True77False0,  // from True76False0 with true
            True78False0,  // from True77False0 with true
            True79False0,  // from True78False0 with true
            True80False0,  // from True79False0 with true
            True81False0,  // from True80False0 with true
            True82False0,  // from True81False0 with true
            True83False0,  // from True82False0 with true
            True84False0,  // from True83False0 with true
            True85False0,  // from True84False0 with true
            True86False0,  // from True85False0 with true
            True87False0,  // from True86False0 with true
            True88False0,  // from True87False0 with true
            True89False0,  // from True88False0 with true
            True90False0,  // from True89False0 with true
            True91False0,  // from True90False0 with true
            True92False0,  // from True91False0 with true
            True93False0,  // from True92False0 with true
            True94False0,  // from True93False0 with true
            True95False0,  // from True94False0 with true
            True96False0,  // from True95False0 with true
            True97False0,  // from True96False0 with true
            True98False0,  // from True97False0 with true
            True99False0,  // from True98False0 with true
            True100False0, // from True99False0 with true
            True101False0, // from True100False0 with true
            True102False0, // from True101False0 with true
            True103False0, // from True102False0 with true
            True104False0, // from True103False0 with true
            True105False0, // from True104False0 with true
            True106False0, // from True105False0 with true
            True107False0, // from True106False0 with true
            True108False0, // from True107False0 with true
            True109False0, // from True108False0 with true
            True110False0, // from True109False0 with true
            True111False0, // from True110False0 with true
            True112False0, // from True111False0 with true
            True113False0, // from True112False0 with true
            True114False0, // from True113False0 with true
            True115False0, // from True114False0 with true
            True116False0, // from True115False0 with true
            True117False0, // from True116False0 with true
            True118False0, // from True117False0 with true
            True119False0, // from True118False0 with true
            True120False0, // from True119False0 with true
            True121False0, // from True120False0 with true
            True122False0, // from True121False0 with true
            True123False0, // from True122False0 with true
            True124False0, // from True123False0 with true
            True125False0, // from True124False0 with true
            True126False0, // from True125False0 with true
            True127False0, // from True126False0 with true
            True128False0, // from True127False0 with true
            True129False0, // from True128False0 with true
            True130False0, // from True129False0 with true
            True131False0, // from True130False0 with true
            True132False0, // from True131False0 with true
            True132False0, // from True132False0 with true
            True1False1,   // from True0False1 with true
            True2False1,   // from True1False1 with true
            True3False1,   // from True2False1 with true
            True4False1,   // from True3False1 with true
            True5False1,   // from True4False1 with true
            True6False1,   // from True5False1 with true
            True7False1,   // from True6False1 with true
            True8False1,   // from True7False1 with true
            True9False1,   // from True8False1 with true
            True10False1,  // from True9False1 with true
            True11False1,  // from True10False1 with true
            True12False1,  // from True11False1 with true
            True13False1,  // from True12False1 with true
            True14False1,  // from True13False1 with true
            True15False1,  // from True14False1 with true
            True16False1,  // from True15False1 with true
            True17False1,  // from True16False1 with true
            True18False1,  // from True17False1 with true
            True19False1,  // from True18False1 with true
            True20False1,  // from True19False1 with true
            True21False1,  // from True20False1 with true
            True22False1,  // from True21False1 with true
            True23False1,  // from True22False1 with true
            True24False1,  // from True23False1 with true
            True25False1,  // from True24False1 with true
            True26False1,  // from True25False1 with true
            True27False1,  // from True26False1 with true
            True28False1,  // from True27False1 with true
            True29False1,  // from True28False1 with true
            True30False1,  // from True29False1 with true
            True31False1,  // from True30False1 with true
            True32False1,  // from True31False1 with true
            True33False1,  // from True32False1 with true
            True34False1,  // from True33False1 with true
            True35False1,  // from True34False1 with true
            True36False1,  // from True35False1 with true
            True37False1,  // from True36False1 with true
            True38False1,  // from True37False1 with true
            True39False1,  // from True38False1 with true
            True40False1,  // from True39False1 with true
            True41False1,  // from True40False1 with true
            True42False1,  // from True41False1 with true
            True43False1,  // from True42False1 with true
            True44False1,  // from True43False1 with true
            True45False1,  // from True44False1 with true
            True46False1,  // from True45False1 with true
            True47False1,  // from True46False1 with true
            True48False1,  // from True47False1 with true
            True49False1,  // from True48False1 with true
            True50False1,  // from True49False1 with true
            True51False1,  // from True50False1 with true
            True52False1,  // from True51False1 with true
            True53False1,  // from True52False1 with true
            True54False1,  // from True53False1 with true
            True55False1,  // from True54False1 with true
            True56False1,  // from True55False1 with true
            True57False1,  // from True56False1 with true
            True58False1,  // from True57False1 with true
            True59False1,  // from True58False1 with true
            True60False1,  // from True59False1 with true
            True61False1,  // from True60False1 with true
            True62False1,  // from True61False1 with true
            True63False1,  // from True62False1 with true
            True64False1,  // from True63False1 with true
            True65False1,  // from True64False1 with true
            True33False0,  // from True65False1 with true
            True1False2,   // from True0False2 with true
            True2False2,   // from True1False2 with true
            True3False2,   // from True2False2 with true
            True4False2,   // from True3False2 with true
            True5False2,   // from True4False2 with true
            True6False2,   // from True5False2 with true
            True7False2,   // from True6False2 with true
            True8False2,   // from True7False2 with true
            True9False2,   // from True8False2 with true
            True10False2,  // from True9False2 with true
            True11False2,  // from True10False2 with true
            True12False2,  // from True11False2 with true
            True13False2,  // from True12False2 with true
            True14False2,  // from True13False2 with true
            True15False2,  // from True14False2 with true
            True16False2,  // from True15False2 with true
            True17False2,  // from True16False2 with true
            True18False2,  // from True17False2 with true
            True19False2,  // from True18False2 with true
            True20False2,  // from True19False2 with true
            True21False2,  // from True20False2 with true
            True22False2,  // from True21False2 with true
            True23False2,  // from True22False2 with true
            True24False2,  // from True23False2 with true
            True25False2,  // from True24False2 with true
            True26False2,  // from True25False2 with true
            True27False2,  // from True26False2 with true
            True28False2,  // from True27False2 with true
            True29False2,  // from True28False2 with true
            True30False2,  // from True29False2 with true
            True31False2,  // from True30False2 with true
            True32False2,  // from True31False2 with true
            True33False2,  // from True32False2 with true
            True34False2,  // from True33False2 with true
            True35False2,  // from True34False2 with true
            True36False2,  // from True35False2 with true
            True37False2,  // from True36False2 with true
            True38False2,  // from True37False2 with true
            True39False2,  // from True38False2 with true
            True40False2,  // from True39False2 with true
            True41False2,  // from True40False2 with true
            True42False2,  // from True41False2 with true
            True43False2,  // from True42False2 with true
            True22False1,  // from True43False2 with true
            True1False3,   // from True0False3 with true
            True2False3,   // from True1False3 with true
            True3False3,   // from True2False3 with true
            True4False3,   // from True3False3 with true
            True5False3,   // from True4False3 with true
            True6False3,   // from True5False3 with true
            True7False3,   // from True6False3 with true
            True8False3,   // from True7False3 with true
            True9False3,   // from True8False3 with true
            True10False3,  // from True9False3 with true
            True11False3,  // from True10False3 with true
            True12False3,  // from True11False3 with true
            True13False3,  // from True12False3 with true
            True14False3,  // from True13False3 with true
            True15False3,  // from True14False3 with true
            True16False3,  // from True15False3 with true
            True17False3,  // from True16False3 with true
            True18False3,  // from True17False3 with true
            True19False3,  // from True18False3 with true
            True20False3,  // from True19False3 with true
            True21False3,  // from True20False3 with true
            True22False3,  // from True21False3 with true
            True23False3,  // from True22False3 with true
            True24False3,  // from True23False3 with true
            True25False3,  // from True24False3 with true
            True26False3,  // from True25False3 with true
            True27False3,  // from True26False3 with true
            True28False3,  // from True27False3 with true
            True29False3,  // from True28False3 with true
            True30False3,  // from True29False3 with true
            True31False3,  // from True30False3 with true
            True32False3,  // from True31False3 with true
            True16False1,  // from True32False3 with true
            True1False4,   // from True0False4 with true
            True2False4,   // from True1False4 with true
            True3False4,   // from True2False4 with true
            True4False4,   // from True3False4 with true
            True5False4,   // from True4False4 with true
            True6False4,   // from True5False4 with true
            True7False4,   // from True6False4 with true
            True8False4,   // from True7False4 with true
            True9False4,   // from True8False4 with true
            True10False4,  // from True9False4 with true
            True11False4,  // from True10False4 with true
            True12False4,  // from True11False4 with true
            True13False4,  // from True12False4 with true
            True14False4,  // from True13False4 with true
            True15False4,  // from True14False4 with true
            True16False4,  // from True15False4 with true
            True17False4,  // from True16False4 with true
            True18False4,  // from True17False4 with true
            True19False4,  // from True18False4 with true
            True20False4,  // from True19False4 with true
            True21False4,  // from True20False4 with true
            True22False4,  // from True21False4 with true
            True23False4,  // from True22False4 with true
            True24False4,  // from True23False4 with true
            True25False4,  // from True24False4 with true
            True13False2,  // from True25False4 with true
            True1False5,   // from True0False5 with true
            True2False5,   // from True1False5 with true
            True3False5,   // from True2False5 with true
            True4False5,   // from True3False5 with true
            True5False5,   // from True4False5 with true
            True6False5,   // from True5False5 with true
            True7False5,   // from True6False5 with true
            True8False5,   // from True7False5 with true
            True9False5,   // from True8False5 with true
            True10False5,  // from True9False5 with true
            True11False5,  // from True10False5 with true
            True12False5,  // from True11False5 with true
            True13False5,  // from True12False5 with true
            True14False5,  // from True13False5 with true
            True15False5,  // from True14False5 with true
            True16False5,  // from True15False5 with true
            True17False5,  // from True16False5 with true
            True18False5,  // from True17False5 with true
            True19False5,  // from True18False5 with true
            True20False5,  // from True19False5 with true
            True21False5,  // from True20False5 with true
            True11False2,  // from True21False5 with true
            True1False6,   // from True0False6 with true
            True2False6,   // from True1False6 with true
            True3False6,   // from True2False6 with true
            True4False6,   // from True3False6 with true
            True5False6,   // from True4False6 with true
            True6False6,   // from True5False6 with true
            True7False6,   // from True6False6 with true
            True8False6,   // from True7False6 with true
            True9False6,   // from True8False6 with true
            True10False6,  // from True9False6 with true
            True11False6,  // from True10False6 with true
            True12False6,  // from True11False6 with true
            True13False6,  // from True12False6 with true
            True14False6,  // from True13False6 with true
            True15False6,  // from True14False6 with true
            True16False6,  // from True15False6 with true
            True17False6,  // from True16False6 with true
            True18False6,  // from True17False6 with true
            True9False3,   // from True18False6 with true
            True1False7,   // from True0False7 with true
            True2False7,   // from True1False7 with true
            True3False7,   // from True2False7 with true
            True4False7,   // from True3False7 with true
            True5False7,   // from True4False7 with true
            True6False7,   // from True5False7 with true
            True7False7,   // from True6False7 with true
            True8False7,   // from True7False7 with true
            True9False7,   // from True8False7 with true
            True10False7,  // from True9False7 with true
            True11False7,  // from True10False7 with true
            True12False7,  // from True11False7 with true
            True13False7,  // from True12False7 with true
            True14False7,  // from True13False7 with true
            True15False7,  // from True14False7 with true
            True8False3,   // from True15False7 with true
            True1False8,   // from True0False8 with true
            True2False8,   // from True1False8 with true
            True3False8,   // from True2False8 with true
            True4False8,   // from True3False8 with true
            True5False8,   // from True4False8 with true
            True6False8,   // from True5False8 with true
            True7False8,   // from True6False8 with true
            True8False8,   // from True7False8 with true
            True9False8,   // from True8False8 with true
            True10False8,  // from True9False8 with true
            True11False8,  // from True10False8 with true
            True12False8,  // from True11False8 with true
            True13False8,  // from True12False8 with true
            True7False4,   // from True13False8 with true
            True1False9,   // from True0False9 with true
            True2False9,   // from True1False9 with true
            True3False9,   // from True2False9 with true
            True4False9,   // from True3False9 with true
            True5False9,   // from True4False9 with true
            True6False9,   // from True5False9 with true
            True7False9,   // from True6False9 with true
            True8False9,   // from True7False9 with true
            True9False9,   // from True8False9 with true
            True10False9,  // from True9False9 with true
            True11False9,  // from True10False9 with true
            True12False9,  // from True11False9 with true
            True6False4,   // from True12False9 with true
            True1False10,  // from True0False10 with true
            True2False10,  // from True1False10 with true
            True3False10,  // from True2False10 with true
            True4False10,  // from True3False10 with true
            True5False10,  // from True4False10 with true
            True6False10,  // from True5False10 with true
            True7False10,  // from True6False10 with true
            True8False10,  // from True7False10 with true
            True9False10,  // from True8False10 with true
            True10False10, // from True9False10 with true
            True11False10, // from True10False10 with true
            True6False5,   // from True11False10 with true
            True1False11,  // from True0False11 with true
            True2False11,  // from True1False11 with true
            True3False11,  // from True2False11 with true
            True4False11,  // from True3False11 with true
            True5False11,  // from True4False11 with true
            True6False11,  // from True5False11 with true
            True7False11,  // from True6False11 with true
            True8False11,  // from True7False11 with true
            True9False11,  // from True8False11 with true
            True10False11, // from True9False11 with true
            True5False5,   // from True10False11 with true
            True1False12,  // from True0False12 with true
            True2False12,  // from True1False12 with true
            True3False12,  // from True2False12 with true
            True4False12,  // from True3False12 with true
            True5False12,  // from True4False12 with true
            True6False12,  // from True5False12 with true
            True7False12,  // from True6False12 with true
            True8False12,  // from True7False12 with true
            True9False12,  // from True8False12 with true
            True5False6,   // from True9False12 with true
            True1False13,  // from True0False13 with true
            True2False13,  // from True1False13 with true
            True3False13,  // from True2False13 with true
            True4False13,  // from True3False13 with true
            True5False13,  // from True4False13 with true
            True6False13,  // from True5False13 with true
            True7False13,  // from True6False13 with true
            True8False13,  // from True7False13 with true
            True4False6,   // from True8False13 with true
            True1False14,  // from True0False14 with true
            True2False14,  // from True1False14 with true
            True3False14,  // from True2False14 with true
            True4False14,  // from True3False14 with true
            True5False14,  // from True4False14 with true
            True6False14,  // from True5False14 with true
            True7False14,  // from True6False14 with true
            True4False7,   // from True7False14 with true
            True1False15,  // from True0False15 with true
            True2False15,  // from True1False15 with true
            True3False15,  // from True2False15 with true
            True4False15,  // from True3False15 with true
            True5False15,  // from True4False15 with true
            True6False15,  // from True5False15 with true
            True7False15,  // from True6False15 with true
            True4False7,   // from True7False15 with true
            True1False16,  // from True0False16 with true
            True2False16,  // from True1False16 with true
            True3False16,  // from True2False16 with true
            True4False16,  // from True3False16 with true
            True5False16,  // from True4False16 with true
            True6False16,  // from True5False16 with true
            True3False8,   // from True6False16 with true
            True1False17,  // from True0False17 with true
            True2False17,  // from True1False17 with true
            True3False17,  // from True2False17 with true
            True4False17,  // from True3False17 with true
            True5False17,  // from True4False17 with true
            True6False17,  // from True5False17 with true
            True3False8,   // from True6False17 with true
            True1False18,  // from True0False18 with true
            True2False18,  // from True1False18 with true
            True3False18,  // from True2False18 with true
            True4False18,  // from True3False18 with true
            True5False18,  // from True4False18 with true
            True6False18,  // from True5False18 with true
            True3False9,   // from True6False18 with true
            True1False19,  // from True0False19 with true
            True2False19,  // from True1False19 with true
            True3False19,  // from True2False19 with true
            True4False19,  // from True3False19 with true
            True5False19,  // from True4False19 with true
            True3False9,   // from True5False19 with true
            True1False20,  // from True0False20 with true
            True2False20,  // from True1False20 with true
            True3False20,  // from True2False20 with true
            True4False20,  // from True3False20 with true
            True5False20,  // from True4False20 with true
            True3False10,  // from True5False20 with true
            True1False21,  // from True0False21 with true
            True2False21,  // from True1False21 with true
            True3False21,  // from True2False21 with true
            True4False21,  // from True3False21 with true
            True5False21,  // from True4False21 with true
            True3False10,  // from True5False21 with true
            True1False22,  // from True0False22 with true
            True2False22,  // from True1False22 with true
            True3False22,  // from True2False22 with true
            True4False22,  // from True3False22 with true
            True2False11,  // from True4False22 with true
            True1False23,  // from True0False23 with true
            True2False23,  // from True1False23 with true
            True3False23,  // from True2False23 with true
            True4False23,  // from True3False23 with true
            True2False11,  // from True4False23 with true
            True1False24,  // from True0False24 with true
            True2False24,  // from True1False24 with true
            True3False24,  // from True2False24 with true
            True4False24,  // from True3False24 with true
            True2False12,  // from True4False24 with true
            True1False25,  // from True0False25 with true
            True2False25,  // from True1False25 with true
            True3False25,  // from True2False25 with true
            True4False25,  // from True3False25 with true
            True2False12,  // from True4False25 with true
            True1False26,  // from True0False26 with true
            True2False26,  // from True1False26 with true
            True3False26,  // from True2False26 with true
            True2False13,  // from True3False26 with true
            True1False27,  // from True0False27 with true
            True2False27,  // from True1False27 with true
            True3False27,  // from True2False27 with true
            True2False13,  // from True3False27 with true
            True1False28,  // from True0False28 with true
            True2False28,  // from True1False28 with true
            True3False28,  // from True2False28 with true
            True2False14,  // from True3False28 with true
            True1False29,  // from True0False29 with true
            True2False29,  // from True1False29 with true
            True3False29,  // from True2False29 with true
            True2False14,  // from True3False29 with true
            True1False30,  // from True0False30 with true
            True2False30,  // from True1False30 with true
            True3False30,  // from True2False30 with true
            True2False15,  // from True3False30 with true
            True1False31,  // from True0False31 with true
            True2False31,  // from True1False31 with true
            True3False31,  // from True2False31 with true
            True2False15,  // from True3False31 with true
            True1False32,  // from True0False32 with true
            True2False32,  // from True1False32 with true
            True3False32,  // from True2False32 with true
            True2False16,  // from True3False32 with true
            True1False33,  // from True0False33 with true
            True2False33,  // from True1False33 with true
            True1False16,  // from True2False33 with true
            True1False34,  // from True0False34 with true
            True2False34,  // from True1False34 with true
            True1False17,  // from True2False34 with true
            True1False35,  // from True0False35 with true
            True2False35,  // from True1False35 with true
            True1False17,  // from True2False35 with true
            True1False36,  // from True0False36 with true
            True2False36,  // from True1False36 with true
            True1False18,  // from True2False36 with true
            True1False37,  // from True0False37 with true
            True2False37,  // from True1False37 with true
            True1False18,  // from True2False37 with true
            True1False38,  // from True0False38 with true
            True2False38,  // from True1False38 with true
            True1False19,  // from True2False38 with true
            True1False39,  // from True0False39 with true
            True2False39,  // from True1False39 with true
            True1False19,  // from True2False39 with true
            True1False40,  // from True0False40 with true
            True2False40,  // from True1False40 with true
            True1False20,  // from True2False40 with true
            True1False41,  // from True0False41 with true
            True2False41,  // from True1False41 with true
            True1False20,  // from True2False41 with true
            True1False42,  // from True0False42 with true
            True2False42,  // from True1False42 with true
            True1False21,  // from True2False42 with true
            True1False43,  // from True0False43 with true
            True2False43,  // from True1False43 with true
            True1False21,  // from True2False43 with true
            True1False44,  // from True0False44 with true
            True1False22,  // from True1False44 with true
            True1False45,  // from True0False45 with true
            True1False22,  // from True1False45 with true
            True1False46,  // from True0False46 with true
            True1False23,  // from True1False46 with true
            True1False47,  // from True0False47 with true
            True1False23,  // from True1False47 with true
            True1False48,  // from True0False48 with true
            True1False24,  // from True1False48 with true
            True1False49,  // from True0False49 with true
            True1False24,  // from True1False49 with true
            True1False50,  // from True0False50 with true
            True1False25,  // from True1False50 with true
            True1False51,  // from True0False51 with true
            True1False25,  // from True1False51 with true
            True1False52,  // from True0False52 with true
            True1False26,  // from True1False52 with true
            True1False53,  // from True0False53 with true
            True1False26,  // from True1False53 with true
            True1False54,  // from True0False54 with true
            True1False27,  // from True1False54 with true
            True1False55,  // from True0False55 with true
            True1False27,  // from True1False55 with true
            True1False56,  // from True0False56 with true
            True1False28,  // from True1False56 with true
            True1False57,  // from True0False57 with true
            True1False28,  // from True1False57 with true
            True1False58,  // from True0False58 with true
            True1False29,  // from True1False58 with true
            True1False59,  // from True0False59 with true
            True1False29,  // from True1False59 with true
            True1False60,  // from True0False60 with true
            True1False30,  // from True1False60 with true
            True1False61,  // from True0False61 with true
            True1False30,  // from True1False61 with true
            True1False62,  // from True0False62 with true
            True1False31,  // from True1False62 with true
            True1False63,  // from True0False63 with true
            True1False31,  // from True1False63 with true
            True1False64,  // from True0False64 with true
            True1False32,  // from True1False64 with true
            True1False65,  // from True0False65 with true
            True1False32,  // from True1False65 with true
            True0False33,  // from True0False66 with true
            True0False33,  // from True0False67 with true
            True0False34,  // from True0False68 with true
            True0False34,  // from True0False69 with true
            True0False35,  // from True0False70 with true
            True0False35,  // from True0False71 with true
            True0False36,  // from True0False72 with true
            True0False36,  // from True0False73 with true
            True0False37,  // from True0False74 with true
            True0False37,  // from True0False75 with true
            True0False38,  // from True0False76 with true
            True0False38,  // from True0False77 with true
            True0False39,  // from True0False78 with true
            True0False39,  // from True0False79 with true
            True0False40,  // from True0False80 with true
            True0False40,  // from True0False81 with true
            True0False41,  // from True0False82 with true
            True0False41,  // from True0False83 with true
            True0False42,  // from True0False84 with true
            True0False42,  // from True0False85 with true
            True0False43,  // from True0False86 with true
            True0False43,  // from True0False87 with true
            True0False44,  // from True0False88 with true
            True0False44,  // from True0False89 with true
            True0False45,  // from True0False90 with true
            True0False45,  // from True0False91 with true
            True0False46,  // from True0False92 with true
            True0False46,  // from True0False93 with true
            True0False47,  // from True0False94 with true
            True0False47,  // from True0False95 with true
            True0False48,  // from True0False96 with true
            True0False48,  // from True0False97 with true
            True0False49,  // from True0False98 with true
            True0False49,  // from True0False99 with true
            True0False50,  // from True0False100 with true
            True0False50,  // from True0False101 with true
            True0False51,  // from True0False102 with true
            True0False51,  // from True0False103 with true
            True0False52,  // from True0False104 with true
            True0False52,  // from True0False105 with true
            True0False53,  // from True0False106 with true
            True0False53,  // from True0False107 with true
            True0False54,  // from True0False108 with true
            True0False54,  // from True0False109 with true
            True0False55,  // from True0False110 with true
            True0False55,  // from True0False111 with true
            True0False56,  // from True0False112 with true
            True0False56,  // from True0False113 with true
            True0False57,  // from True0False114 with true
            True0False57,  // from True0False115 with true
            True0False58,  // from True0False116 with true
            True0False58,  // from True0False117 with true
            True0False59,  // from True0False118 with true
            True0False59,  // from True0False119 with true
            True0False60,  // from True0False120 with true
            True0False60,  // from True0False121 with true
            True0False61,  // from True0False122 with true
            True0False61,  // from True0False123 with true
            True0False62,  // from True0False124 with true
            True0False62,  // from True0False125 with true
            True0False63,  // from True0False126 with true
            True0False63,  // from True0False127 with true
            True0False64,  // from True0False128 with true
            True0False64,  // from True0False129 with true
            True0False65,  // from True0False130 with true
            True0False65,  // from True0False131 with true
            True0False66,  // from True0False132 with true
        ];
        OUTCOMES[(self as usize) + (bit as usize) * 675]
    }
    #[inline]
    pub fn millibits_required(&mut self, bit: bool) -> u32 {
        const LOOKUP: [u32; 1350] = [
            1000, // True0False0 for false
            1590, // True1False0 for false
            2000, // True2False0 for false
            2327, // True3False0 for false
            2607, // True4False0 for false
            2830, // True5False0 for false
            3000, // True6False0 for false
            3192, // True7False0 for false
            3356, // True8False0 for false
            3476, // True9False0 for false
            3607, // True10False0 for false
            3752, // True11False0 for false
            3830, // True12False0 for false
            3912, // True13False0 for false
            4000, // True14False0 for false
            4093, // True15False0 for false
            4192, // True16False0 for false
            4299, // True17False0 for false
            4415, // True18False0 for false
            4415, // True19False0 for false
            4540, // True20False0 for false
            4540, // True21False0 for false
            4678, // True22False0 for false
            4678, // True23False0 for false
            4830, // True24False0 for false
            4830, // True25False0 for false
            4830, // True26False0 for false
            5000, // True27False0 for false
            5000, // True28False0 for false
            5000, // True29False0 for false
            5000, // True30False0 for false
            5192, // True31False0 for false
            5192, // True32False0 for false
            5192, // True33False0 for false
            5192, // True34False0 for false
            5415, // True35False0 for false
            5415, // True36False0 for false
            5415, // True37False0 for false
            5415, // True38False0 for false
            5415, // True39False0 for false
            5415, // True40False0 for false
            5678, // True41False0 for false
            5678, // True42False0 for false
            5678, // True43False0 for false
            5678, // True44False0 for false
            5678, // True45False0 for false
            5678, // True46False0 for false
            5678, // True47False0 for false
            5678, // True48False0 for false
            5678, // True49False0 for false
            6000, // True50False0 for false
            6000, // True51False0 for false
            6000, // True52False0 for false
            6000, // True53False0 for false
            6000, // True54False0 for false
            6000, // True55False0 for false
            6000, // True56False0 for false
            6000, // True57False0 for false
            6000, // True58False0 for false
            6000, // True59False0 for false
            6000, // True60False0 for false
            6000, // True61False0 for false
            6000, // True62False0 for false
            6415, // True63False0 for false
            6415, // True64False0 for false
            6415, // True65False0 for false
            6415, // True66False0 for false
            6415, // True67False0 for false
            6415, // True68False0 for false
            6415, // True69False0 for false
            6415, // True70False0 for false
            6415, // True71False0 for false
            6415, // True72False0 for false
            6415, // True73False0 for false
            6415, // True74False0 for false
            6415, // True75False0 for false
            6415, // True76False0 for false
            6415, // True77False0 for false
            6415, // True78False0 for false
            6415, // True79False0 for false
            6415, // True80False0 for false
            6415, // True81False0 for false
            6415, // True82False0 for false
            6415, // True83False0 for false
            6415, // True84False0 for false
            6415, // True85False0 for false
            7000, // True86False0 for false
            7000, // True87False0 for false
            7000, // True88False0 for false
            7000, // True89False0 for false
            7000, // True90False0 for false
            7000, // True91False0 for false
            7000, // True92False0 for false
            7000, // True93False0 for false
            7000, // True94False0 for false
            7000, // True95False0 for false
            7000, // True96False0 for false
            7000, // True97False0 for false
            7000, // True98False0 for false
            7000, // True99False0 for false
            7000, // True100False0 for false
            7000, // True101False0 for false
            7000, // True102False0 for false
            7000, // True103False0 for false
            7000, // True104False0 for false
            7000, // True105False0 for false
            7000, // True106False0 for false
            7000, // True107False0 for false
            7000, // True108False0 for false
            7000, // True109False0 for false
            7000, // True110False0 for false
            7000, // True111False0 for false
            7000, // True112False0 for false
            7000, // True113False0 for false
            7000, // True114False0 for false
            7000, // True115False0 for false
            7000, // True116False0 for false
            7000, // True117False0 for false
            7000, // True118False0 for false
            7000, // True119False0 for false
            7000, // True120False0 for false
            7000, // True121False0 for false
            7000, // True122False0 for false
            7000, // True123False0 for false
            7000, // True124False0 for false
            7000, // True125False0 for false
            7000, // True126False0 for false
            7000, // True127False0 for false
            7000, // True128False0 for false
            7000, // True129False0 for false
            7000, // True130False0 for false
            7000, // True131False0 for false
            7000, // True132False0 for false
            582,  // True0False1 for false
            1000, // True1False1 for false
            1327, // True2False1 for false
            1590, // True3False1 for false
            1810, // True4False1 for false
            2000, // True5False1 for false
            2167, // True6False1 for false
            2327, // True7False1 for false
            2445, // True8False1 for false
            2573, // True9False1 for false
            2714, // True10False1 for false
            2790, // True11False1 for false
            2912, // True12False1 for false
            3000, // True13False1 for false
            3093, // True14False1 for false
            3192, // True15False1 for false
            3245, // True16False1 for false
            3299, // True17False1 for false
            3415, // True18False1 for false
            3476, // True19False1 for false
            3540, // True20False1 for false
            3607, // True21False1 for false
            3678, // True22False1 for false
            3678, // True23False1 for false
            3752, // True24False1 for false
            3830, // True25False1 for false
            3830, // True26False1 for false
            3912, // True27False1 for false
            3912, // True28False1 for false
            4000, // True29False1 for false
            4000, // True30False1 for false
            4093, // True31False1 for false
            4093, // True32False1 for false
            4192, // True33False1 for false
            4192, // True34False1 for false
            4192, // True35False1 for false
            4299, // True36False1 for false
            4299, // True37False1 for false
            4299, // True38False1 for false
            4415, // True39False1 for false
            4415, // True40False1 for false
            4415, // True41False1 for false
            4540, // True42False1 for false
            4540, // True43False1 for false
            4540, // True44False1 for false
            4540, // True45False1 for false
            4678, // True46False1 for false
            4678, // True47False1 for false
            4678, // True48False1 for false
            4678, // True49False1 for false
            4678, // True50False1 for false
            4678, // True51False1 for false
            4830, // True52False1 for false
            4830, // True53False1 for false
            4830, // True54False1 for false
            4830, // True55False1 for false
            4830, // True56False1 for false
            4830, // True57False1 for false
            5000, // True58False1 for false
            5000, // True59False1 for false
            5000, // True60False1 for false
            5000, // True61False1 for false
            5000, // True62False1 for false
            5000, // True63False1 for false
            5000, // True64False1 for false
            5000, // True65False1 for false
            415,  // True0False2 for false
            733,  // True1False2 for false
            1000, // True2False2 for false
            1218, // True3False2 for false
            1415, // True4False2 for false
            1590, // True5False2 for false
            1733, // True6False2 for false
            1870, // True7False2 for false
            2000, // True8False2 for false
            2117, // True9False2 for false
            2218, // True10False2 for false
            2327, // True11False2 for false
            2415, // True12False2 for false
            2508, // True13False2 for false
            2573, // True14False2 for false
            2678, // True15False2 for false
            2752, // True16False2 for false
            2790, // True17False2 for false
            2870, // True18False2 for false
            2955, // True19False2 for false
            3000, // True20False2 for false
            3045, // True21False2 for false
            3093, // True22False2 for false
            3192, // True23False2 for false
            3245, // True24False2 for false
            3299, // True25False2 for false
            3299, // True26False2 for false
            3356, // True27False2 for false
            3415, // True28False2 for false
            3476, // True29False2 for false
            3476, // True30False2 for false
            3540, // True31False2 for false
            3607, // True32False2 for false
            3607, // True33False2 for false
            3678, // True34False2 for false
            3678, // True35False2 for false
            3752, // True36False2 for false
            3752, // True37False2 for false
            3830, // True38False2 for false
            3830, // True39False2 for false
            3912, // True40False2 for false
            3912, // True41False2 for false
            3912, // True42False2 for false
            4000, // True43False2 for false
            320,  // True0False3 for false
            582,  // True1False3 for false
            810,  // True2False3 for false
            1000, // True3False3 for false
            1167, // True4False3 for false
            1327, // True5False3 for false
            1460, // True6False3 for false
            1590, // True7False3 for false
            1696, // True8False3 for false
            1810, // True9False3 for false
            1912, // True10False3 for false
            2000, // True11False3 for false
            2093, // True12False3 for false
            2167, // True13False3 for false
            2245, // True14False3 for false
            2327, // True15False3 for false
            2385, // True16False3 for false
            2445, // True17False3 for false
            2508, // True18False3 for false
            2573, // True19False3 for false
            2642, // True20False3 for false
            2714, // True21False3 for false
            2752, // True22False3 for false
            2790, // True23False3 for false
            2870, // True24False3 for false
            2912, // True25False3 for false
            2955, // True26False3 for false
            3000, // True27False3 for false
            3045, // True28False3 for false
            3093, // True29False3 for false
            3142, // True30False3 for false
            3192, // True31False3 for false
            3192, // True32False3 for false
            258,  // True0False4 for false
            484,  // True1False4 for false
            678,  // True2False4 for false
            850,  // True3False4 for false
            1000, // True4False4 for false
            1142, // True5False4 for false
            1258, // True6False4 for false
            1385, // True7False4 for false
            1492, // True8False4 for false
            1590, // True9False4 for false
            1678, // True10False4 for false
            1771, // True11False4 for false
            1850, // True12False4 for false
            1933, // True13False4 for false
            2000, // True14False4 for false
            2069, // True15False4 for false
            2142, // True16False4 for false
            2192, // True17False4 for false
            2272, // True18False4 for false
            2327, // True19False4 for false
            2385, // True20False4 for false
            2445, // True21False4 for false
            2476, // True22False4 for false
            2540, // True23False4 for false
            2573, // True24False4 for false
            2642, // True25False4 for false
            218,  // True0False5 for false
            415,  // True1False5 for false
            582,  // True2False5 for false
            733,  // True3False5 for false
            870,  // True4False5 for false
            1000, // True5False5 for false
            1117, // True6False5 for false
            1218, // True7False5 for false
            1327, // True8False5 for false
            1415, // True9False5 for false
            1508, // True10False5 for false
            1590, // True11False5 for false
            1660, // True12False5 for false
            1733, // True13False5 for false
            1810, // True14False5 for false
            1870, // True15False5 for false
            1933, // True16False5 for false
            2000, // True17False5 for false
            2069, // True18False5 for false
            2117, // True19False5 for false
            2167, // True20False5 for false
            2218, // True21False5 for false
            192,  // True0False6 for false
            363,  // True1False6 for false
            516,  // True2False6 for false
            651,  // True3False6 for false
            780,  // True4False6 for false
            891,  // True5False6 for false
            1000, // True6False6 for false
            1105, // True7False6 for false
            1192, // True8False6 for false
            1285, // True9False6 for false
            1356, // True10False6 for false
            1445, // True11False6 for false
            1508, // True12False6 for false
            1590, // True13False6 for false
            1660, // True14False6 for false
            1714, // True15False6 for false
            1771, // True16False6 for false
            1830, // True17False6 for false
            1891, // True18False6 for false
            167,  // True0False7 for false
            320,  // True1False7 for false
            460,  // True2False7 for false
            582,  // True3False7 for false
            696,  // True4False7 for false
            810,  // True5False7 for false
            901,  // True6False7 for false
            1000, // True7False7 for false
            1093, // True8False7 for false
            1167, // True9False7 for false
            1245, // True10False7 for false
            1327, // True11False7 for false
            1385, // True12False7 for false
            1460, // True13False7 for false
            1524, // True14False7 for false
            1590, // True15False7 for false
            148,  // True0False8 for false
            292,  // True1False8 for false
            415,  // True2False8 for false
            532,  // True3False8 for false
            633,  // True4False8 for false
            733,  // True5False8 for false
            830,  // True6False8 for false
            912,  // True7False8 for false
            1000, // True8False8 for false
            1081, // True9False8 for false
            1154, // True10False8 for false
            1218, // True11False8 for false
            1285, // True12False8 for false
            1356, // True13False8 for false
            135,  // True0False9 for false
            265,  // True1False9 for false
            377,  // True2False9 for false
            484,  // True3False9 for false
            582,  // True4False9 for false
            678,  // True5False9 for false
            761,  // True6False9 for false
            850,  // True7False9 for false
            923,  // True8False9 for false
            1000, // True9False9 for false
            1069, // True10False9 for false
            1142, // True11False9 for false
            1205, // True12False9 for false
            123,  // True0False10 for false
            238,  // True1False10 for false
            348,  // True2False10 for false
            445,  // True3False10 for false
            540,  // True4False10 for false
            624,  // True5False10 for false
            714,  // True6False10 for false
            790,  // True7False10 for false
            860,  // True8False10 for false
            933,  // True9False10 for false
            1000, // True10False10 for false
            1069, // True11False10 for false
            111,  // True0False11 for false
            225,  // True1False11 for false
            320,  // True2False11 for false
            415,  // True3False11 for false
            500,  // True4False11 for false
            582,  // True5False11 for false
            660,  // True6False11 for false
            733,  // True7False11 for false
            810,  // True8False11 for false
            870,  // True9False11 for false
            933,  // True10False11 for false
            105,  // True0False12 for false
            205,  // True1False12 for false
            299,  // True2False12 for false
            385,  // True3False12 for false
            468,  // True4False12 for false
            548,  // True5False12 for false
            624,  // True6False12 for false
            696,  // True7False12 for false
            761,  // True8False12 for false
            820,  // True9False12 for false
            99,   // True0False13 for false
            192,  // True1False13 for false
            278,  // True2False13 for false
            363,  // True3False13 for false
            437,  // True4False13 for false
            516,  // True5False13 for false
            582,  // True6False13 for false
            651,  // True7False13 for false
            714,  // True8False13 for false
            93,   // True0False14 for false
            179,  // True1False14 for false
            265,  // True2False14 for false
            341,  // True3False14 for false
            415,  // True4False14 for false
            484,  // True5False14 for false
            548,  // True6False14 for false
            616,  // True7False14 for false
            87,   // True0False15 for false
            167,  // True1False15 for false
            245,  // True2False15 for false
            320,  // True3False15 for false
            392,  // True4False15 for false
            460,  // True5False15 for false
            524,  // True6False15 for false
            582,  // True7False15 for false
            81,   // True0False16 for false
            160,  // True1False16 for false
            231,  // True2False16 for false
            306,  // True3False16 for false
            370,  // True4False16 for false
            437,  // True5False16 for false
            500,  // True6False16 for false
            75,   // True0False17 for false
            154,  // True1False17 for false
            225,  // True2False17 for false
            292,  // True3False17 for false
            356,  // True4False17 for false
            415,  // True5False17 for false
            476,  // True6False17 for false
            69,   // True0False18 for false
            142,  // True1False18 for false
            212,  // True2False18 for false
            278,  // True3False18 for false
            334,  // True4False18 for false
            392,  // True5False18 for false
            453,  // True6False18 for false
            69,   // True0False19 for false
            135,  // True1False19 for false
            199,  // True2False19 for false
            265,  // True3False19 for false
            320,  // True4False19 for false
            377,  // True5False19 for false
            63,   // True0False20 for false
            129,  // True1False20 for false
            192,  // True2False20 for false
            251,  // True3False20 for false
            306,  // True4False20 for false
            363,  // True5False20 for false
            63,   // True0False21 for false
            123,  // True1False21 for false
            186,  // True2False21 for false
            238,  // True3False21 for false
            292,  // True4False21 for false
            348,  // True5False21 for false
            57,   // True0False22 for false
            117,  // True1False22 for false
            179,  // True2False22 for false
            231,  // True3False22 for false
            285,  // True4False22 for false
            57,   // True0False23 for false
            117,  // True1False23 for false
            167,  // True2False23 for false
            225,  // True3False23 for false
            272,  // True4False23 for false
            51,   // True0False24 for false
            111,  // True1False24 for false
            160,  // True2False24 for false
            212,  // True3False24 for false
            265,  // True4False24 for false
            51,   // True0False25 for false
            105,  // True1False25 for false
            154,  // True2False25 for false
            205,  // True3False25 for false
            251,  // True4False25 for false
            51,   // True0False26 for false
            105,  // True1False26 for false
            154,  // True2False26 for false
            199,  // True3False26 for false
            45,   // True0False27 for false
            99,   // True1False27 for false
            148,  // True2False27 for false
            192,  // True3False27 for false
            45,   // True0False28 for false
            99,   // True1False28 for false
            142,  // True2False28 for false
            186,  // True3False28 for false
            45,   // True0False29 for false
            93,   // True1False29 for false
            135,  // True2False29 for false
            179,  // True3False29 for false
            45,   // True0False30 for false
            93,   // True1False30 for false
            135,  // True2False30 for false
            173,  // True3False30 for false
            39,   // True0False31 for false
            87,   // True1False31 for false
            129,  // True2False31 for false
            167,  // True3False31 for false
            39,   // True0False32 for false
            87,   // True1False32 for false
            123,  // True2False32 for false
            167,  // True3False32 for false
            39,   // True0False33 for false
            81,   // True1False33 for false
            123,  // True2False33 for false
            39,   // True0False34 for false
            81,   // True1False34 for false
            117,  // True2False34 for false
            34,   // True0False35 for false
            81,   // True1False35 for false
            117,  // True2False35 for false
            34,   // True0False36 for false
            75,   // True1False36 for false
            111,  // True2False36 for false
            34,   // True0False37 for false
            75,   // True1False37 for false
            111,  // True2False37 for false
            34,   // True0False38 for false
            75,   // True1False38 for false
            105,  // True2False38 for false
            34,   // True0False39 for false
            69,   // True1False39 for false
            105,  // True2False39 for false
            34,   // True0False40 for false
            69,   // True1False40 for false
            99,   // True2False40 for false
            28,   // True0False41 for false
            69,   // True1False41 for false
            99,   // True2False41 for false
            28,   // True0False42 for false
            63,   // True1False42 for false
            99,   // True2False42 for false
            28,   // True0False43 for false
            63,   // True1False43 for false
            93,   // True2False43 for false
            28,   // True0False44 for false
            63,   // True1False44 for false
            28,   // True0False45 for false
            63,   // True1False45 for false
            28,   // True0False46 for false
            57,   // True1False46 for false
            28,   // True0False47 for false
            57,   // True1False47 for false
            28,   // True0False48 for false
            57,   // True1False48 for false
            28,   // True0False49 for false
            57,   // True1False49 for false
            22,   // True0False50 for false
            57,   // True1False50 for false
            22,   // True0False51 for false
            57,   // True1False51 for false
            22,   // True0False52 for false
            51,   // True1False52 for false
            22,   // True0False53 for false
            51,   // True1False53 for false
            22,   // True0False54 for false
            51,   // True1False54 for false
            22,   // True0False55 for false
            51,   // True1False55 for false
            22,   // True0False56 for false
            51,   // True1False56 for false
            22,   // True0False57 for false
            51,   // True1False57 for false
            22,   // True0False58 for false
            45,   // True1False58 for false
            22,   // True0False59 for false
            45,   // True1False59 for false
            22,   // True0False60 for false
            45,   // True1False60 for false
            22,   // True0False61 for false
            45,   // True1False61 for false
            22,   // True0False62 for false
            45,   // True1False62 for false
            17,   // True0False63 for false
            45,   // True1False63 for false
            17,   // True0False64 for false
            45,   // True1False64 for false
            17,   // True0False65 for false
            45,   // True1False65 for false
            17,   // True0False66 for false
            17,   // True0False67 for false
            17,   // True0False68 for false
            17,   // True0False69 for false
            17,   // True0False70 for false
            17,   // True0False71 for false
            17,   // True0False72 for false
            17,   // True0False73 for false
            17,   // True0False74 for false
            17,   // True0False75 for false
            17,   // True0False76 for false
            17,   // True0False77 for false
            17,   // True0False78 for false
            17,   // True0False79 for false
            17,   // True0False80 for false
            17,   // True0False81 for false
            17,   // True0False82 for false
            17,   // True0False83 for false
            17,   // True0False84 for false
            17,   // True0False85 for false
            11,   // True0False86 for false
            11,   // True0False87 for false
            11,   // True0False88 for false
            11,   // True0False89 for false
            11,   // True0False90 for false
            11,   // True0False91 for false
            11,   // True0False92 for false
            11,   // True0False93 for false
            11,   // True0False94 for false
            11,   // True0False95 for false
            11,   // True0False96 for false
            11,   // True0False97 for false
            11,   // True0False98 for false
            11,   // True0False99 for false
            11,   // True0False100 for false
            11,   // True0False101 for false
            11,   // True0False102 for false
            11,   // True0False103 for false
            11,   // True0False104 for false
            11,   // True0False105 for false
            11,   // True0False106 for false
            11,   // True0False107 for false
            11,   // True0False108 for false
            11,   // True0False109 for false
            11,   // True0False110 for false
            11,   // True0False111 for false
            11,   // True0False112 for false
            11,   // True0False113 for false
            11,   // True0False114 for false
            11,   // True0False115 for false
            11,   // True0False116 for false
            11,   // True0False117 for false
            11,   // True0False118 for false
            11,   // True0False119 for false
            11,   // True0False120 for false
            11,   // True0False121 for false
            11,   // True0False122 for false
            11,   // True0False123 for false
            11,   // True0False124 for false
            11,   // True0False125 for false
            11,   // True0False126 for false
            11,   // True0False127 for false
            11,   // True0False128 for false
            11,   // True0False129 for false
            11,   // True0False130 for false
            11,   // True0False131 for false
            11,   // True0False132 for false
            1000, // True0False0 for true
            582,  // True1False0 for true
            415,  // True2False0 for true
            320,  // True3False0 for true
            258,  // True4False0 for true
            218,  // True5False0 for true
            192,  // True6False0 for true
            167,  // True7False0 for true
            148,  // True8False0 for true
            135,  // True9False0 for true
            123,  // True10False0 for true
            111,  // True11False0 for true
            105,  // True12False0 for true
            99,   // True13False0 for true
            93,   // True14False0 for true
            87,   // True15False0 for true
            81,   // True16False0 for true
            75,   // True17False0 for true
            69,   // True18False0 for true
            69,   // True19False0 for true
            63,   // True20False0 for true
            63,   // True21False0 for true
            57,   // True22False0 for true
            57,   // True23False0 for true
            51,   // True24False0 for true
            51,   // True25False0 for true
            51,   // True26False0 for true
            45,   // True27False0 for true
            45,   // True28False0 for true
            45,   // True29False0 for true
            45,   // True30False0 for true
            39,   // True31False0 for true
            39,   // True32False0 for true
            39,   // True33False0 for true
            39,   // True34False0 for true
            34,   // True35False0 for true
            34,   // True36False0 for true
            34,   // True37False0 for true
            34,   // True38False0 for true
            34,   // True39False0 for true
            34,   // True40False0 for true
            28,   // True41False0 for true
            28,   // True42False0 for true
            28,   // True43False0 for true
            28,   // True44False0 for true
            28,   // True45False0 for true
            28,   // True46False0 for true
            28,   // True47False0 for true
            28,   // True48False0 for true
            28,   // True49False0 for true
            22,   // True50False0 for true
            22,   // True51False0 for true
            22,   // True52False0 for true
            22,   // True53False0 for true
            22,   // True54False0 for true
            22,   // True55False0 for true
            22,   // True56False0 for true
            22,   // True57False0 for true
            22,   // True58False0 for true
            22,   // True59False0 for true
            22,   // True60False0 for true
            22,   // True61False0 for true
            22,   // True62False0 for true
            17,   // True63False0 for true
            17,   // True64False0 for true
            17,   // True65False0 for true
            17,   // True66False0 for true
            17,   // True67False0 for true
            17,   // True68False0 for true
            17,   // True69False0 for true
            17,   // True70False0 for true
            17,   // True71False0 for true
            17,   // True72False0 for true
            17,   // True73False0 for true
            17,   // True74False0 for true
            17,   // True75False0 for true
            17,   // True76False0 for true
            17,   // True77False0 for true
            17,   // True78False0 for true
            17,   // True79False0 for true
            17,   // True80False0 for true
            17,   // True81False0 for true
            17,   // True82False0 for true
            17,   // True83False0 for true
            17,   // True84False0 for true
            17,   // True85False0 for true
            11,   // True86False0 for true
            11,   // True87False0 for true
            11,   // True88False0 for true
            11,   // True89False0 for true
            11,   // True90False0 for true
            11,   // True91False0 for true
            11,   // True92False0 for true
            11,   // True93False0 for true
            11,   // True94False0 for true
            11,   // True95False0 for true
            11,   // True96False0 for true
            11,   // True97False0 for true
            11,   // True98False0 for true
            11,   // True99False0 for true
            11,   // True100False0 for true
            11,   // True101False0 for true
            11,   // True102False0 for true
            11,   // True103False0 for true
            11,   // True104False0 for true
            11,   // True105False0 for true
            11,   // True106False0 for true
            11,   // True107False0 for true
            11,   // True108False0 for true
            11,   // True109False0 for true
            11,   // True110False0 for true
            11,   // True111False0 for true
            11,   // True112False0 for true
            11,   // True113False0 for true
            11,   // True114False0 for true
            11,   // True115False0 for true
            11,   // True116False0 for true
            11,   // True117False0 for true
            11,   // True118False0 for true
            11,   // True119False0 for true
            11,   // True120False0 for true
            11,   // True121False0 for true
            11,   // True122False0 for true
            11,   // True123False0 for true
            11,   // True124False0 for true
            11,   // True125False0 for true
            11,   // True126False0 for true
            11,   // True127False0 for true
            11,   // True128False0 for true
            11,   // True129False0 for true
            11,   // True130False0 for true
            11,   // True131False0 for true
            11,   // True132False0 for true
            1590, // True0False1 for true
            1000, // True1False1 for true
            733,  // True2False1 for true
            582,  // True3False1 for true
            484,  // True4False1 for true
            415,  // True5False1 for true
            363,  // True6False1 for true
            320,  // True7False1 for true
            292,  // True8False1 for true
            265,  // True9False1 for true
            238,  // True10False1 for true
            225,  // True11False1 for true
            205,  // True12False1 for true
            192,  // True13False1 for true
            179,  // True14False1 for true
            167,  // True15False1 for true
            160,  // True16False1 for true
            154,  // True17False1 for true
            142,  // True18False1 for true
            135,  // True19False1 for true
            129,  // True20False1 for true
            123,  // True21False1 for true
            117,  // True22False1 for true
            117,  // True23False1 for true
            111,  // True24False1 for true
            105,  // True25False1 for true
            105,  // True26False1 for true
            99,   // True27False1 for true
            99,   // True28False1 for true
            93,   // True29False1 for true
            93,   // True30False1 for true
            87,   // True31False1 for true
            87,   // True32False1 for true
            81,   // True33False1 for true
            81,   // True34False1 for true
            81,   // True35False1 for true
            75,   // True36False1 for true
            75,   // True37False1 for true
            75,   // True38False1 for true
            69,   // True39False1 for true
            69,   // True40False1 for true
            69,   // True41False1 for true
            63,   // True42False1 for true
            63,   // True43False1 for true
            63,   // True44False1 for true
            63,   // True45False1 for true
            57,   // True46False1 for true
            57,   // True47False1 for true
            57,   // True48False1 for true
            57,   // True49False1 for true
            57,   // True50False1 for true
            57,   // True51False1 for true
            51,   // True52False1 for true
            51,   // True53False1 for true
            51,   // True54False1 for true
            51,   // True55False1 for true
            51,   // True56False1 for true
            51,   // True57False1 for true
            45,   // True58False1 for true
            45,   // True59False1 for true
            45,   // True60False1 for true
            45,   // True61False1 for true
            45,   // True62False1 for true
            45,   // True63False1 for true
            45,   // True64False1 for true
            45,   // True65False1 for true
            2000, // True0False2 for true
            1327, // True1False2 for true
            1000, // True2False2 for true
            810,  // True3False2 for true
            678,  // True4False2 for true
            582,  // True5False2 for true
            516,  // True6False2 for true
            460,  // True7False2 for true
            415,  // True8False2 for true
            377,  // True9False2 for true
            348,  // True10False2 for true
            320,  // True11False2 for true
            299,  // True12False2 for true
            278,  // True13False2 for true
            265,  // True14False2 for true
            245,  // True15False2 for true
            231,  // True16False2 for true
            225,  // True17False2 for true
            212,  // True18False2 for true
            199,  // True19False2 for true
            192,  // True20False2 for true
            186,  // True21False2 for true
            179,  // True22False2 for true
            167,  // True23False2 for true
            160,  // True24False2 for true
            154,  // True25False2 for true
            154,  // True26False2 for true
            148,  // True27False2 for true
            142,  // True28False2 for true
            135,  // True29False2 for true
            135,  // True30False2 for true
            129,  // True31False2 for true
            123,  // True32False2 for true
            123,  // True33False2 for true
            117,  // True34False2 for true
            117,  // True35False2 for true
            111,  // True36False2 for true
            111,  // True37False2 for true
            105,  // True38False2 for true
            105,  // True39False2 for true
            99,   // True40False2 for true
            99,   // True41False2 for true
            99,   // True42False2 for true
            93,   // True43False2 for true
            2327, // True0False3 for true
            1590, // True1False3 for true
            1218, // True2False3 for true
            1000, // True3False3 for true
            850,  // True4False3 for true
            733,  // True5False3 for true
            651,  // True6False3 for true
            582,  // True7False3 for true
            532,  // True8False3 for true
            484,  // True9False3 for true
            445,  // True10False3 for true
            415,  // True11False3 for true
            385,  // True12False3 for true
            363,  // True13False3 for true
            341,  // True14False3 for true
            320,  // True15False3 for true
            306,  // True16False3 for true
            292,  // True17False3 for true
            278,  // True18False3 for true
            265,  // True19False3 for true
            251,  // True20False3 for true
            238,  // True21False3 for true
            231,  // True22False3 for true
            225,  // True23False3 for true
            212,  // True24False3 for true
            205,  // True25False3 for true
            199,  // True26False3 for true
            192,  // True27False3 for true
            186,  // True28False3 for true
            179,  // True29False3 for true
            173,  // True30False3 for true
            167,  // True31False3 for true
            167,  // True32False3 for true
            2607, // True0False4 for true
            1810, // True1False4 for true
            1415, // True2False4 for true
            1167, // True3False4 for true
            1000, // True4False4 for true
            870,  // True5False4 for true
            780,  // True6False4 for true
            696,  // True7False4 for true
            633,  // True8False4 for true
            582,  // True9False4 for true
            540,  // True10False4 for true
            500,  // True11False4 for true
            468,  // True12False4 for true
            437,  // True13False4 for true
            415,  // True14False4 for true
            392,  // True15False4 for true
            370,  // True16False4 for true
            356,  // True17False4 for true
            334,  // True18False4 for true
            320,  // True19False4 for true
            306,  // True20False4 for true
            292,  // True21False4 for true
            285,  // True22False4 for true
            272,  // True23False4 for true
            265,  // True24False4 for true
            251,  // True25False4 for true
            2830, // True0False5 for true
            2000, // True1False5 for true
            1590, // True2False5 for true
            1327, // True3False5 for true
            1142, // True4False5 for true
            1000, // True5False5 for true
            891,  // True6False5 for true
            810,  // True7False5 for true
            733,  // True8False5 for true
            678,  // True9False5 for true
            624,  // True10False5 for true
            582,  // True11False5 for true
            548,  // True12False5 for true
            516,  // True13False5 for true
            484,  // True14False5 for true
            460,  // True15False5 for true
            437,  // True16False5 for true
            415,  // True17False5 for true
            392,  // True18False5 for true
            377,  // True19False5 for true
            363,  // True20False5 for true
            348,  // True21False5 for true
            3000, // True0False6 for true
            2167, // True1False6 for true
            1733, // True2False6 for true
            1460, // True3False6 for true
            1258, // True4False6 for true
            1117, // True5False6 for true
            1000, // True6False6 for true
            901,  // True7False6 for true
            830,  // True8False6 for true
            761,  // True9False6 for true
            714,  // True10False6 for true
            660,  // True11False6 for true
            624,  // True12False6 for true
            582,  // True13False6 for true
            548,  // True14False6 for true
            524,  // True15False6 for true
            500,  // True16False6 for true
            476,  // True17False6 for true
            453,  // True18False6 for true
            3192, // True0False7 for true
            2327, // True1False7 for true
            1870, // True2False7 for true
            1590, // True3False7 for true
            1385, // True4False7 for true
            1218, // True5False7 for true
            1105, // True6False7 for true
            1000, // True7False7 for true
            912,  // True8False7 for true
            850,  // True9False7 for true
            790,  // True10False7 for true
            733,  // True11False7 for true
            696,  // True12False7 for true
            651,  // True13False7 for true
            616,  // True14False7 for true
            582,  // True15False7 for true
            3356, // True0False8 for true
            2445, // True1False8 for true
            2000, // True2False8 for true
            1696, // True3False8 for true
            1492, // True4False8 for true
            1327, // True5False8 for true
            1192, // True6False8 for true
            1093, // True7False8 for true
            1000, // True8False8 for true
            923,  // True9False8 for true
            860,  // True10False8 for true
            810,  // True11False8 for true
            761,  // True12False8 for true
            714,  // True13False8 for true
            3476, // True0False9 for true
            2573, // True1False9 for true
            2117, // True2False9 for true
            1810, // True3False9 for true
            1590, // True4False9 for true
            1415, // True5False9 for true
            1285, // True6False9 for true
            1167, // True7False9 for true
            1081, // True8False9 for true
            1000, // True9False9 for true
            933,  // True10False9 for true
            870,  // True11False9 for true
            820,  // True12False9 for true
            3607, // True0False10 for true
            2714, // True1False10 for true
            2218, // True2False10 for true
            1912, // True3False10 for true
            1678, // True4False10 for true
            1508, // True5False10 for true
            1356, // True6False10 for true
            1245, // True7False10 for true
            1154, // True8False10 for true
            1069, // True9False10 for true
            1000, // True10False10 for true
            933,  // True11False10 for true
            3752, // True0False11 for true
            2790, // True1False11 for true
            2327, // True2False11 for true
            2000, // True3False11 for true
            1771, // True4False11 for true
            1590, // True5False11 for true
            1445, // True6False11 for true
            1327, // True7False11 for true
            1218, // True8False11 for true
            1142, // True9False11 for true
            1069, // True10False11 for true
            3830, // True0False12 for true
            2912, // True1False12 for true
            2415, // True2False12 for true
            2093, // True3False12 for true
            1850, // True4False12 for true
            1660, // True5False12 for true
            1508, // True6False12 for true
            1385, // True7False12 for true
            1285, // True8False12 for true
            1205, // True9False12 for true
            3912, // True0False13 for true
            3000, // True1False13 for true
            2508, // True2False13 for true
            2167, // True3False13 for true
            1933, // True4False13 for true
            1733, // True5False13 for true
            1590, // True6False13 for true
            1460, // True7False13 for true
            1356, // True8False13 for true
            4000, // True0False14 for true
            3093, // True1False14 for true
            2573, // True2False14 for true
            2245, // True3False14 for true
            2000, // True4False14 for true
            1810, // True5False14 for true
            1660, // True6False14 for true
            1524, // True7False14 for true
            4093, // True0False15 for true
            3192, // True1False15 for true
            2678, // True2False15 for true
            2327, // True3False15 for true
            2069, // True4False15 for true
            1870, // True5False15 for true
            1714, // True6False15 for true
            1590, // True7False15 for true
            4192, // True0False16 for true
            3245, // True1False16 for true
            2752, // True2False16 for true
            2385, // True3False16 for true
            2142, // True4False16 for true
            1933, // True5False16 for true
            1771, // True6False16 for true
            4299, // True0False17 for true
            3299, // True1False17 for true
            2790, // True2False17 for true
            2445, // True3False17 for true
            2192, // True4False17 for true
            2000, // True5False17 for true
            1830, // True6False17 for true
            4415, // True0False18 for true
            3415, // True1False18 for true
            2870, // True2False18 for true
            2508, // True3False18 for true
            2272, // True4False18 for true
            2069, // True5False18 for true
            1891, // True6False18 for true
            4415, // True0False19 for true
            3476, // True1False19 for true
            2955, // True2False19 for true
            2573, // True3False19 for true
            2327, // True4False19 for true
            2117, // True5False19 for true
            4540, // True0False20 for true
            3540, // True1False20 for true
            3000, // True2False20 for true
            2642, // True3False20 for true
            2385, // True4False20 for true
            2167, // True5False20 for true
            4540, // True0False21 for true
            3607, // True1False21 for true
            3045, // True2False21 for true
            2714, // True3False21 for true
            2445, // True4False21 for true
            2218, // True5False21 for true
            4678, // True0False22 for true
            3678, // True1False22 for true
            3093, // True2False22 for true
            2752, // True3False22 for true
            2476, // True4False22 for true
            4678, // True0False23 for true
            3678, // True1False23 for true
            3192, // True2False23 for true
            2790, // True3False23 for true
            2540, // True4False23 for true
            4830, // True0False24 for true
            3752, // True1False24 for true
            3245, // True2False24 for true
            2870, // True3False24 for true
            2573, // True4False24 for true
            4830, // True0False25 for true
            3830, // True1False25 for true
            3299, // True2False25 for true
            2912, // True3False25 for true
            2642, // True4False25 for true
            4830, // True0False26 for true
            3830, // True1False26 for true
            3299, // True2False26 for true
            2955, // True3False26 for true
            5000, // True0False27 for true
            3912, // True1False27 for true
            3356, // True2False27 for true
            3000, // True3False27 for true
            5000, // True0False28 for true
            3912, // True1False28 for true
            3415, // True2False28 for true
            3045, // True3False28 for true
            5000, // True0False29 for true
            4000, // True1False29 for true
            3476, // True2False29 for true
            3093, // True3False29 for true
            5000, // True0False30 for true
            4000, // True1False30 for true
            3476, // True2False30 for true
            3142, // True3False30 for true
            5192, // True0False31 for true
            4093, // True1False31 for true
            3540, // True2False31 for true
            3192, // True3False31 for true
            5192, // True0False32 for true
            4093, // True1False32 for true
            3607, // True2False32 for true
            3192, // True3False32 for true
            5192, // True0False33 for true
            4192, // True1False33 for true
            3607, // True2False33 for true
            5192, // True0False34 for true
            4192, // True1False34 for true
            3678, // True2False34 for true
            5415, // True0False35 for true
            4192, // True1False35 for true
            3678, // True2False35 for true
            5415, // True0False36 for true
            4299, // True1False36 for true
            3752, // True2False36 for true
            5415, // True0False37 for true
            4299, // True1False37 for true
            3752, // True2False37 for true
            5415, // True0False38 for true
            4299, // True1False38 for true
            3830, // True2False38 for true
            5415, // True0False39 for true
            4415, // True1False39 for true
            3830, // True2False39 for true
            5415, // True0False40 for true
            4415, // True1False40 for true
            3912, // True2False40 for true
            5678, // True0False41 for true
            4415, // True1False41 for true
            3912, // True2False41 for true
            5678, // True0False42 for true
            4540, // True1False42 for true
            3912, // True2False42 for true
            5678, // True0False43 for true
            4540, // True1False43 for true
            4000, // True2False43 for true
            5678, // True0False44 for true
            4540, // True1False44 for true
            5678, // True0False45 for true
            4540, // True1False45 for true
            5678, // True0False46 for true
            4678, // True1False46 for true
            5678, // True0False47 for true
            4678, // True1False47 for true
            5678, // True0False48 for true
            4678, // True1False48 for true
            5678, // True0False49 for true
            4678, // True1False49 for true
            6000, // True0False50 for true
            4678, // True1False50 for true
            6000, // True0False51 for true
            4678, // True1False51 for true
            6000, // True0False52 for true
            4830, // True1False52 for true
            6000, // True0False53 for true
            4830, // True1False53 for true
            6000, // True0False54 for true
            4830, // True1False54 for true
            6000, // True0False55 for true
            4830, // True1False55 for true
            6000, // True0False56 for true
            4830, // True1False56 for true
            6000, // True0False57 for true
            4830, // True1False57 for true
            6000, // True0False58 for true
            5000, // True1False58 for true
            6000, // True0False59 for true
            5000, // True1False59 for true
            6000, // True0False60 for true
            5000, // True1False60 for true
            6000, // True0False61 for true
            5000, // True1False61 for true
            6000, // True0False62 for true
            5000, // True1False62 for true
            6415, // True0False63 for true
            5000, // True1False63 for true
            6415, // True0False64 for true
            5000, // True1False64 for true
            6415, // True0False65 for true
            5000, // True1False65 for true
            6415, // True0False66 for true
            6415, // True0False67 for true
            6415, // True0False68 for true
            6415, // True0False69 for true
            6415, // True0False70 for true
            6415, // True0False71 for true
            6415, // True0False72 for true
            6415, // True0False73 for true
            6415, // True0False74 for true
            6415, // True0False75 for true
            6415, // True0False76 for true
            6415, // True0False77 for true
            6415, // True0False78 for true
            6415, // True0False79 for true
            6415, // True0False80 for true
            6415, // True0False81 for true
            6415, // True0False82 for true
            6415, // True0False83 for true
            6415, // True0False84 for true
            6415, // True0False85 for true
            7000, // True0False86 for true
            7000, // True0False87 for true
            7000, // True0False88 for true
            7000, // True0False89 for true
            7000, // True0False90 for true
            7000, // True0False91 for true
            7000, // True0False92 for true
            7000, // True0False93 for true
            7000, // True0False94 for true
            7000, // True0False95 for true
            7000, // True0False96 for true
            7000, // True0False97 for true
            7000, // True0False98 for true
            7000, // True0False99 for true
            7000, // True0False100 for true
            7000, // True0False101 for true
            7000, // True0False102 for true
            7000, // True0False103 for true
            7000, // True0False104 for true
            7000, // True0False105 for true
            7000, // True0False106 for true
            7000, // True0False107 for true
            7000, // True0False108 for true
            7000, // True0False109 for true
            7000, // True0False110 for true
            7000, // True0False111 for true
            7000, // True0False112 for true
            7000, // True0False113 for true
            7000, // True0False114 for true
            7000, // True0False115 for true
            7000, // True0False116 for true
            7000, // True0False117 for true
            7000, // True0False118 for true
            7000, // True0False119 for true
            7000, // True0False120 for true
            7000, // True0False121 for true
            7000, // True0False122 for true
            7000, // True0False123 for true
            7000, // True0False124 for true
            7000, // True0False125 for true
            7000, // True0False126 for true
            7000, // True0False127 for true
            7000, // True0False128 for true
            7000, // True0False129 for true
            7000, // True0False130 for true
            7000, // True0False131 for true
            7000, // True0False132 for true
        ];
        let out = LOOKUP[(*self as usize) + (bit as usize) * 675];
        *self = self.adapt(bit);
        out
    }
}
// Count of variants: 675
