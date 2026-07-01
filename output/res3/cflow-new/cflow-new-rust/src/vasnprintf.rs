use std::cmp::max;

pub struct Vasnprintf;

#[derive(Clone, Debug, PartialEq)]
pub enum VasnprintfArgument {
    Signed(i128),
    Unsigned(u128),
    Float(f64),
    Char(char),
    Str(String),
    Wide(Vec<char>),
    Bytes(Vec<u8>),
    Bool(bool),
    None,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VasnprintfArgType {
    Int,
    UInt,
    Float,
    Char,
    WideChar,
    Str,
    WideStr,
    Pointer,
    Count,
    Bool,
    Other,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VasnprintfDecoded {
    pub exponent2: i32,
    pub mantissa: Vec<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VasnprintfEstimate {
    pub conversion: char,
    pub arg_type: VasnprintfArgType,
    pub flags: i32,
    pub width: usize,
    pub has_precision: bool,
    pub precision: usize,
    pub pad_ourselves: bool,
}

impl Vasnprintf {
    pub fn local_strnlen(string: &str, maxlen: usize) -> usize {
        let bytes = string.as_bytes();
        match bytes.iter().take(maxlen).position(|&b| b == 0) {
            Some(pos) => pos,
            None => bytes.len().min(maxlen),
        }
    }

    pub fn local_wcslen(s: &[char]) -> usize {
        s.iter().position(|&ch| ch == '\0').unwrap_or(s.len())
    }

    pub fn local_wcsnlen(s: &[char], maxlen: usize) -> usize {
        s.iter()
            .take(maxlen)
            .position(|&ch| ch == '\0')
            .unwrap_or(s.len().min(maxlen))
    }

    pub fn wctomb_fallback(wc: char) -> String {
        let value = wc as u32;
        if value > 0xFFFF {
            format!("\\U{:08X}", value)
        } else {
            format!("\\u{:04X}", value)
        }
    }

    pub fn local_wcrtomb(wc: char) -> String {
        let mut buf = [0_u8; 4];
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
        let rendered = format!("{:#.0}", 1.0_f64);
        rendered.chars().nth(1).unwrap_or('.')
    }

    pub fn is_infinite_or_zero(x: f64) -> bool {
        x.is_nan() || x + x == x
    }

    pub fn is_infinite_or_zerol(x: f64) -> bool {
        x.is_nan() || x + x == x
    }

    pub fn multiply_limbs(src1: &[u32], src2: &[u32]) -> Vec<u32> {
        if src1.is_empty() || src2.is_empty() {
            return Vec::new();
        }

        let (shorter, longer) = if src1.len() <= src2.len() {
            (src1, src2)
        } else {
            (src2, src1)
        };

        let mut out = vec![0_u32; shorter.len() + longer.len()];
        for (i, &a) in shorter.iter().enumerate() {
            let mut carry: u64 = 0;
            for (j, &b) in longer.iter().enumerate() {
                let acc = carry + (a as u64) * (b as u64) + (out[i + j] as u64);
                out[i + j] = acc as u32;
                carry = acc >> 32;
            }
            out[i + longer.len()] = carry as u32;
        }
        while out.last() == Some(&0) {
            out.pop();
        }
        out
    }

    pub fn divide_limbs(a: &[u32], b: &[u32]) -> Vec<u32> {
        if b.is_empty() {
            return Vec::new();
        }
        if a.is_empty() {
            return Vec::new();
        }

        let av = Self::limbs_to_u128(a);
        let bv = Self::limbs_to_u128(b);
        if bv == 0 {
            return Vec::new();
        }
        Self::u128_to_limbs(Self::round_divide_u128(av, bv))
    }

    pub fn convert_to_decimal(mut a: Vec<u32>, extra_zeroes: usize) -> String {
        if a.is_empty() {
            let mut out = String::from("0");
            out.extend(std::iter::repeat_n('0', extra_zeroes));
            return out;
        }

        let mut parts = Vec::new();
        while !a.is_empty() {
            let (q, r) = Self::div_rem_small(&a, 1_000_000_000);
            parts.push(r);
            a = q;
        }

        let mut out = String::new();
        out.extend(std::iter::repeat_n('0', extra_zeroes));
        for (idx, part) in parts.iter().rev().enumerate() {
            if idx == 0 {
                out.push_str(&part.to_string());
            } else {
                out.push_str(&format!("{part:09}"));
            }
        }

        let trimmed = out.trim_start_matches('0');
        if trimmed.is_empty() {
            "0".to_string()
        } else {
            trimmed.to_string()
        }
    }

    pub fn decode_long_double(x: f64) -> Option<VasnprintfDecoded> {
        Self::decode_float_like(x, f64::MANTISSA_DIGITS as i32)
    }

    pub fn decode_double(x: f64) -> Option<VasnprintfDecoded> {
        Self::decode_float_like(x, f64::MANTISSA_DIGITS as i32)
    }

    pub fn scale_10_round_decimal_decoded(
        exponent2: i32,
        mantissa: &[u32],
        n: i32,
    ) -> Option<String> {
        let mut s = exponent2 + n;
        let mut extra_zeroes = 0usize;
        let mut n_mut = n;

        if s > 0 && n_mut > 0 {
            extra_zeroes = (s.min(n_mut)) as usize;
            s -= extra_zeroes as i32;
            n_mut -= extra_zeroes as i32;
        }

        let x = (Self::limbs_to_u128(mantissa) as f64) * 2f64.powi(exponent2);
        if !x.is_finite() {
            return None;
        }

        let y = (x * 10f64.powi(n_mut)).round();
        if !y.is_finite() || y < 0.0 {
            return None;
        }

        let z = y as u128;
        Some(Self::convert_to_decimal(
            Self::u128_to_limbs(z),
            extra_zeroes,
        ))
    }

    pub fn scale_10_round_decimal_long_double(x: f64, n: i32) -> Option<String> {
        let decoded = Self::decode_long_double(x)?;
        Self::scale_10_round_decimal_decoded(decoded.exponent2, &decoded.mantissa, n)
    }

    pub fn scale_10_round_decimal_double(x: f64, n: i32) -> Option<String> {
        let decoded = Self::decode_double(x)?;
        Self::scale_10_round_decimal_decoded(decoded.exponent2, &decoded.mantissa, n)
    }

    pub fn floorlog_10_l(x: f64) -> i32 {
        Self::floorlog_impl(x)
    }

    pub fn floorlog_10(x: f64) -> i32 {
        Self::floorlog_impl(x)
    }

    pub fn is_borderline(digits: &str, precision: usize) -> bool {
        let bytes = digits.as_bytes();
        if bytes.len() < precision + 2 {
            return false;
        }
        if bytes.iter().take(precision).any(|&b| b != b'0') {
            return false;
        }
        bytes.get(precision) == Some(&b'1') && bytes.get(precision + 1).copied() == Some(0).or_else(|| Some(b'\0')).is_some() && bytes.len() == precision + 1
    }

    pub fn max_room_needed(
        arguments: &[VasnprintfArgument],
        arg_index: usize,
        conversion: char,
        arg_type: VasnprintfArgType,
        flags: i32,
        width: usize,
        has_precision: bool,
        precision: usize,
        pad_ourselves: bool,
    ) -> usize {
        let argument = arguments.get(arg_index);
        let base = match (conversion, &arg_type, argument) {
            ('c', VasnprintfArgType::WideChar, Some(VasnprintfArgument::Char(c))) => {
                Self::local_wctomb(*c).len()
            }
            ('c', _, _) => 1,
            ('s', VasnprintfArgType::WideStr, Some(VasnprintfArgument::Wide(s))) => {
                let len = if has_precision {
                    Self::local_wcsnlen(s, precision)
                } else {
                    Self::local_wcslen(s)
                };
                s.iter()
                    .take(len)
                    .map(|c| Self::local_wctomb(*c).len())
                    .sum()
            }
            ('s', _, Some(VasnprintfArgument::Str(s))) => {
                if has_precision {
                    Self::local_strnlen(s, precision)
                } else {
                    s.len()
                }
            }
            ('d' | 'i', _, Some(VasnprintfArgument::Signed(v))) => {
                let digits = v.unsigned_abs().to_string().len();
                digits + usize::from(*v < 0 || (flags & 0x01) != 0 || (flags & 0x02) != 0)
            }
            ('u' | 'o' | 'x' | 'X', _, Some(VasnprintfArgument::Unsigned(v))) => match conversion {
                'u' => v.to_string().len(),
                'o' => format!("{v:o}").len(),
                'x' => format!("{v:x}").len(),
                'X' => format!("{v:X}").len(),
                _ => 0,
            },
            ('f' | 'F' | 'e' | 'E' | 'g' | 'G', _, Some(VasnprintfArgument::Float(v))) => {
                let prec = if has_precision { precision } else { 6 };
                Self::float_room(*v, conversion, prec, flags)
            }
            ('p', _, _) => 2 + std::mem::size_of::<usize>() * 2,
            _ => {
                let _ = flags;
                32
            }
        };

        if pad_ourselves {
            max(base, width)
        } else {
            base
        }
    }

    pub fn r#if(
        arguments: &[VasnprintfArgument],
        arg_index: usize,
        conversion: char,
        arg_type: VasnprintfArgType,
        flags: i32,
        width: usize,
        precision: Option<usize>,
    ) -> usize {
        Self::max_room_needed(
            arguments,
            arg_index,
            conversion,
            arg_type,
            flags,
            width,
            precision.is_some(),
            precision.unwrap_or(0),
            Self::pad_ourselves(flags, conversion),
        )
    }

    pub fn pad_ourselves(flags: i32, conversion: char) -> bool {
        let zero_pad = (flags & 0x08) != 0;
        !matches!(conversion, 'n') && !zero_pad
    }

    pub fn has_precision(precision: Option<usize>) -> bool {
        precision.is_some()
    }

    pub fn module_gnu_scale_10_round_14(x: f64, n: i32) -> Option<String> {
        Self::scale_10_round_decimal_double(x, n)
    }

    pub fn is_infinite(x: f64) -> bool {
        x.is_infinite() || x.is_nan()
    }

    pub fn module_gnu_if_10(
        arguments: &[VasnprintfArgument],
        estimate: &VasnprintfEstimate,
        arg_index: usize,
    ) -> usize {
        Self::max_room_needed(
            arguments,
            arg_index,
            estimate.conversion,
            estimate.arg_type.clone(),
            estimate.flags,
            estimate.width,
            estimate.has_precision,
            estimate.precision,
            estimate.pad_ourselves,
        )
    }

    fn decode_float_like(x: f64, mantissa_bits: i32) -> Option<VasnprintfDecoded> {
        if !x.is_finite() || x < 0.0 {
            return None;
        }
        if x == 0.0 {
            return Some(VasnprintfDecoded {
                exponent2: -mantissa_bits,
                mantissa: Vec::new(),
            });
        }
        let (mantissa, exponent, sign) = Self::integer_decode(x);
        if sign < 0 {
            return None;
        }
        Some(VasnprintfDecoded {
            exponent2: exponent + mantissa_bits,
            mantissa: Self::u128_to_limbs(mantissa),
        })
    }

    fn integer_decode(x: f64) -> (u128, i32, i8) {
        let bits = x.to_bits();
        let sign = if bits >> 63 == 0 { 1 } else { -1 };
        let exponent = ((bits >> 52) & 0x7ff) as i32;
        let mantissa = bits & 0x000f_ffff_ffff_ffff;

        if exponent == 0 {
            (mantissa as u128, -1074, sign)
        } else {
            (((mantissa | (1_u64 << 52)) as u128), exponent - 1075, sign)
        }
    }

    fn floorlog_impl(x: f64) -> i32 {
        if x == 0.0 {
            return i32::MIN;
        }
        if !x.is_finite() || x < 0.0 {
            return i32::MIN;
        }
        let l = x.log10().floor();
        if l < i32::MIN as f64 {
            i32::MIN
        } else if l > i32::MAX as f64 {
            i32::MAX
        } else {
            l as i32
        }
    }

    fn float_room(x: f64, conversion: char, precision: usize, flags: i32) -> usize {
        if Self::is_infinite(x) {
            return 3 + usize::from(x.is_sign_negative() || (flags & 0x01) != 0 || (flags & 0x02) != 0);
        }

        match conversion {
            'f' | 'F' => {
                let whole = if x == 0.0 {
                    1
                } else {
                    (x.abs().log10().floor().max(0.0) as usize) + 1
                };
                whole + usize::from(precision > 0 || (flags & 0x04) != 0) + precision + 2
            }
            'e' | 'E' => 1 + usize::from(precision > 0 || (flags & 0x04) != 0) + precision + 5,
            'g' | 'G' => max(precision, 1) + 8,
            _ => 32,
        }
    }

    fn limbs_to_u128(limbs: &[u32]) -> u128 {
        limbs
            .iter()
            .enumerate()
            .fold(0_u128, |acc, (i, &limb)| acc | ((limb as u128) << (32 * i)))
    }

    fn u128_to_limbs(mut value: u128) -> Vec<u32> {
        let mut out = Vec::new();
        while value > 0 {
            out.push((value & 0xffff_ffff) as u32);
            value >>= 32;
        }
        out
    }

    fn round_divide_u128(a: u128, b: u128) -> u128 {
        let q = a / b;
        let r = a % b;
        let twice_r = r.saturating_mul(2);
        if twice_r > b || (twice_r == b && (q & 1) == 1) {
            q + 1
        } else {
            q
        }
    }

    fn div_rem_small(a: &[u32], div: u32) -> (Vec<u32>, u32) {
        let mut q = vec![0_u32; a.len()];
        let mut rem: u64 = 0;
        for (idx, &limb) in a.iter().enumerate().rev() {
            let cur = (rem << 32) | limb as u64;
            q[idx] = (cur / div as u64) as u32;
            rem = cur % div as u64;
        }
        while q.last() == Some(&0) {
            q.pop();
        }
        (q, rem as u32)
    }
}
