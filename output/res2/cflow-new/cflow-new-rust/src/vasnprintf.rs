use crate::printf_args::{PrintfArg, PrintfArgs};
use crate::printf_parse::{PrintfArgKind, PrintfDirective};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ModuleGnuScale10Round14 {
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ModuleGnuIsInfinite18 {
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ModuleGnuIf10 {
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ModuleGnuIf11 {
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CLongdouble {
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecimalMpn {
    pub limbs: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecimalDecode {
    pub exponent: i32,
    pub mantissa: DecimalMpn,
}

pub struct Vasnprintf;

impl Vasnprintf {
    pub fn local_strnlen(string: &str, maxlen: usize) -> usize {
        let bytes = string.as_bytes();
        match bytes.iter().take(maxlen).position(|b| *b == 0) {
            Some(index) => index,
            None => bytes.len().min(maxlen),
        }
    }

    pub fn local_wcslen(s: &[char]) -> usize {
        s.iter().position(|ch| *ch == '\0').unwrap_or(s.len())
    }

    pub fn local_wcsnlen(s: &[char], maxlen: usize) -> usize {
        let limit = s.len().min(maxlen);
        s.iter().take(limit).position(|ch| *ch == '\0').unwrap_or(limit)
    }

    pub fn wctomb_fallback(wc: char) -> String {
        let value = wc as u32;
        if value > 0xffff {
            format!("\\U{value:08X}")
        } else {
            format!("\\u{value:04X}")
        }
    }

    pub fn local_wcrtomb(wc: char) -> String {
        let mut buf = [0u8; 4];
        let encoded = wc.encode_utf8(&mut buf);
        if encoded.is_empty() {
            Self::wctomb_fallback(wc)
        } else {
            encoded.to_string()
        }
    }

    pub fn local_wctomb(wc: char) -> String {
        Self::local_wcrtomb(wc)
    }

    pub fn decimal_point_char() -> char {
        '.'
    }

    pub fn is_infinite_or_zero(x: f64) -> bool {
        x.is_nan() || x + x == x
    }

    pub fn is_infinite_or_zerol(x: f64) -> bool {
        x.is_nan() || x + x == x
    }

    pub fn multiply_decimal_mpn(src1: &DecimalMpn, src2: &DecimalMpn) -> Option<DecimalMpn> {
        let (p1, p2) = if src1.limbs.len() <= src2.limbs.len() {
            (&src1.limbs, &src2.limbs)
        } else {
            (&src2.limbs, &src1.limbs)
        };

        if p1.is_empty() {
            return Some(DecimalMpn { limbs: Vec::new() });
        }

        let len1 = p1.len();
        let len2 = p2.len();
        let mut dp = vec![0u32; len1 + len2];

        for i in 0..len1 {
            let digit1 = u64::from(p1[i]);
            let mut carry: u64 = 0;
            for j in 0..len2 {
                let digit2 = u64::from(p2[j]);
                carry += digit1 * digit2;
                carry += u64::from(dp[i + j]);
                dp[i + j] = carry as u32;
                carry >>= 32;
            }
            dp[i + len2] = carry as u32;
        }

        while dp.last().copied() == Some(0) {
            dp.pop();
        }

        Some(DecimalMpn { limbs: dp })
    }

    pub fn divide_decimal_mpn(a: &DecimalMpn, b: &DecimalMpn) -> Option<DecimalMpn> {
        let av = Self::mpn_to_u128(a)?;
        let bv = Self::mpn_to_u128(b)?;
        if bv == 0 {
            return None;
        }
        Some(Self::u128_to_mpn(av / bv))
    }

    pub fn convert_to_decimal(a: &DecimalMpn, extra_zeroes: usize) -> String {
        let mut s = Self::mpn_to_u128(a).unwrap_or(0).to_string();
        s.extend(std::iter::repeat_n('0', extra_zeroes));
        s
    }

    pub fn decode_long_double(x: f64) -> Option<DecimalDecode> {
        Self::decode_double(x)
    }

    pub fn decode_double(x: f64) -> Option<DecimalDecode> {
        if !x.is_finite() {
            return None;
        }

        let bits = x.abs().to_bits();
        let exp_bits = ((bits >> 52) & 0x7ff) as i32;
        let frac_bits = bits & ((1u64 << 52) - 1);

        if exp_bits == 0 {
            if frac_bits == 0 {
                return Some(DecimalDecode {
                    exponent: 0,
                    mantissa: DecimalMpn { limbs: Vec::new() },
                });
            }
            Some(DecimalDecode {
                exponent: -1074,
                mantissa: Self::u128_to_mpn(frac_bits as u128),
            })
        } else {
            let mantissa = frac_bits | (1u64 << 52);
            Some(DecimalDecode {
                exponent: exp_bits - 1075,
                mantissa: Self::u128_to_mpn(mantissa as u128),
            })
        }
    }

    pub fn scale_10_round_decimal_decoded(decoded: &DecimalDecode, n: i32) -> Option<String> {
        if n < 0 {
            return None;
        }
        let value = Self::mpn_to_u128(&decoded.mantissa).unwrap_or(0);
        let digits = value.to_string();
        let precision = n as usize;
        if digits.len() <= precision {
            return Some(digits);
        }
        Some(digits[..precision].to_string())
    }

    pub fn scale_10_round_decimal_long_double(x: f64, n: i32) -> Option<String> {
        let decoded = Self::decode_long_double(x)?;
        Self::scale_10_round_decimal_decoded(&decoded, n)
    }

    pub fn scale_10_round_decimal_double(x: f64, n: i32) -> Option<String> {
        let decoded = Self::decode_double(x)?;
        Self::scale_10_round_decimal_decoded(&decoded, n)
    }

    pub fn floorlog_10_l(x: f64) -> i32 {
        Self::floorlog_10(x)
    }

    pub fn floorlog_10(x: f64) -> i32 {
        if !x.is_finite() || x == 0.0 {
            0
        } else {
            x.abs().log10().floor() as i32
        }
    }

    pub fn is_borderline(digits: &str, precision: usize) -> bool {
        let bytes = digits.as_bytes();
        if bytes.len() != precision + 1 {
            return false;
        }
        if bytes.iter().take(precision).any(|b| *b != b'0') {
            return false;
        }
        bytes[precision] == b'1'
    }

    pub fn max_room_needed(
        arguments: &PrintfArgs,
        arg_index: usize,
        directive: &PrintfDirective,
        kind: Option<PrintfArgKind>,
        pad_ourselves: bool,
    ) -> usize {
        let width = directive.width.unwrap_or(0);
        let precision = directive.precision.unwrap_or(0);
        let has_precision = directive.precision.is_some();
        let arg = arguments.get(arg_index);

        let room = match directive.conversion {
            'd' | 'i' => {
                let digits = match arg {
                    Some(PrintfArg::Signed(v)) => v.unsigned_abs().to_string().len(),
                    Some(PrintfArg::Unsigned(v)) => v.to_string().len(),
                    _ => 1,
                };
                let sign = usize::from(matches!(arg, Some(PrintfArg::Signed(v)) if *v < 0))
                    + usize::from(directive.flags.contains('+') || directive.flags.contains(' '));
                sign + digits.max(if has_precision { precision } else { 1 })
            }
            'o' => {
                let digits = match arg {
                    Some(PrintfArg::Unsigned(v)) => format!("{v:o}").len(),
                    _ => 1,
                };
                digits.max(if has_precision { precision } else { 1 })
                    + usize::from(directive.flags.contains('#'))
            }
            'u' => {
                let digits = match arg {
                    Some(PrintfArg::Unsigned(v)) => v.to_string().len(),
                    _ => 1,
                };
                digits.max(if has_precision { precision } else { 1 })
            }
            'x' | 'X' => {
                let digits = match arg {
                    Some(PrintfArg::Unsigned(v)) if directive.conversion == 'x' => {
                        format!("{v:x}").len()
                    }
                    Some(PrintfArg::Unsigned(v)) => format!("{v:X}").len(),
                    _ => 1,
                };
                digits.max(if has_precision { precision } else { 1 })
                    + if directive.flags.contains('#') { 2 } else { 0 }
            }
            'p' => match arg {
                Some(PrintfArg::Pointer(v)) => format!("0x{v:x}").len(),
                _ => 2,
            },
            'c' => match kind.or(directive.argument_kind.clone()) {
                Some(PrintfArgKind::Char) | None => 1,
                _ => 1,
            },
            's' => match arg {
                Some(PrintfArg::Str(s)) => {
                    if has_precision {
                        Self::local_strnlen(s, precision)
                    } else {
                        s.len()
                    }
                }
                _ => 0,
            },
            'f' | 'F' => {
                let p = if has_precision { precision } else { 6 };
                let lead = match arg {
                    Some(PrintfArg::Float(v)) => {
                        if v.is_finite() && *v != 0.0 {
                            (Self::floorlog_10(v.abs()) + 1).max(1) as usize
                        } else {
                            1
                        }
                    }
                    _ => 1,
                };
                lead + p + usize::from(p > 0 || directive.flags.contains('#')) + 1
            }
            'e' | 'E' => {
                let p = if has_precision { precision } else { 6 };
                1 + usize::from(p > 0 || directive.flags.contains('#')) + p + 4
            }
            'g' | 'G' => {
                let p = if has_precision { precision.max(1) } else { 6 };
                p + 7
            }
            'a' | 'A' => {
                let p = if has_precision { precision } else { 13 };
                p + 8
            }
            'n' | '%' => 0,
            _ => 1,
        };

        if pad_ourselves { room.max(width) } else { room }
    }

    pub fn r#if(condition: bool, if_true: usize, if_false: usize) -> usize {
        if condition { if_true } else { if_false }
    }

    pub fn c_longdouble() -> CLongdouble {
        CLongdouble { enabled: true }
    }

    pub fn has_precision(directive: &PrintfDirective) -> bool {
        directive.precision.is_some()
    }

    pub fn pad_ourselves(directive: &PrintfDirective) -> bool {
        !matches!(directive.conversion, 'n' | '%')
    }

    pub fn module_gnu_scale_10_round_14() -> ModuleGnuScale10Round14 {
        ModuleGnuScale10Round14 { enabled: true }
    }

    pub fn module_gnu_is_infinite_18() -> ModuleGnuIsInfinite18 {
        ModuleGnuIsInfinite18 { enabled: true }
    }

    pub fn module_gnu_if_10() -> ModuleGnuIf10 {
        ModuleGnuIf10 { enabled: true }
    }

    pub fn module_gnu_if_11() -> ModuleGnuIf11 {
        ModuleGnuIf11 { enabled: true }
    }

    fn mpn_to_u128(value: &DecimalMpn) -> Option<u128> {
        let mut result = 0u128;
        for (index, limb) in value.limbs.iter().enumerate() {
            let shift = index.checked_mul(32)?;
            if shift >= 128 {
                return None;
            }
            result = result.checked_add((u128::from(*limb)) << shift)?;
        }
        Some(result)
    }

    fn u128_to_mpn(mut value: u128) -> DecimalMpn {
        let mut limbs = Vec::new();
        while value != 0 {
            limbs.push((value & 0xffff_ffff) as u32);
            value >>= 32;
        }
        DecimalMpn { limbs }
    }
}
