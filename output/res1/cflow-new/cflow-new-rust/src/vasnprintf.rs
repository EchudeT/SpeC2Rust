use std::char;
use std::fmt::Write as _;

pub struct Vasnprintf;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Mpn {
    limbs: Vec<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct DecodedFloat {
    exponent: i32,
    mantissa: Mpn,
}

impl Mpn {
    fn normalize(&mut self) {
        while self.limbs.last().copied() == Some(0) {
            self.limbs.pop();
        }
    }

    fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    fn from_u128(mut value: u128) -> Self {
        let mut limbs = Vec::new();
        while value > 0 {
            limbs.push((value & 0xffff_ffff) as u32);
            value >>= 32;
        }
        Self { limbs }
    }

    fn to_u128(&self) -> Option<u128> {
        if self.limbs.len() > 4 {
            return None;
        }
        let mut v = 0u128;
        for (i, limb) in self.limbs.iter().enumerate() {
            v |= (*limb as u128) << (32 * i);
        }
        Some(v)
    }
}

impl Vasnprintf {
    pub fn local_strnlen(input: &[u8], max_len: usize) -> usize {
        input
            .iter()
            .take(max_len)
            .position(|&b| b == 0)
            .unwrap_or(max_len.min(input.len()))
    }

    pub fn local_wcslen(input: &[char]) -> usize {
        input.iter().position(|&c| c == '\0').unwrap_or(input.len())
    }

    pub fn local_wcsnlen(input: &[char], max_len: usize) -> usize {
        input
            .iter()
            .take(max_len)
            .position(|&c| c == '\0')
            .unwrap_or(max_len.min(input.len()))
    }

    pub fn wctomb_fallback(ch: char) -> Vec<u8> {
        let code = ch as u32;
        if code > 0xffff {
            format!("\\U{:08X}", code).into_bytes()
        } else {
            format!("\\u{:04X}", code).into_bytes()
        }
    }

    pub fn local_wcrtomb(ch: char) -> Vec<u8> {
        let mut buf = [0u8; 4];
        let encoded = ch.encode_utf8(&mut buf);
        if encoded.is_empty() {
            Self::wctomb_fallback(ch)
        } else {
            encoded.as_bytes().to_vec()
        }
    }

    pub fn local_wctomb(ch: char) -> Vec<u8> {
        Self::local_wcrtomb(ch)
    }

    pub fn decimal_point_char() -> u8 {
        let formatted = format!("{:#.0}", 1.0f64);
        formatted.as_bytes().get(1).copied().unwrap_or(b'.')
    }

    pub fn is_infinite_or_zero(x: f64) -> bool {
        x.is_nan() || x + x == x
    }

    pub fn is_infinite_or_zerol(x: f64) -> bool {
        x.is_nan() || x + x == x
    }

    pub fn multiply_limbs(src1: &[u32], src2: &[u32]) -> Vec<u32> {
        let mut a = Mpn {
            limbs: src1.to_vec(),
        };
        let mut b = Mpn {
            limbs: src2.to_vec(),
        };
        a.normalize();
        b.normalize();

        let (p1, p2) = if a.limbs.len() <= b.limbs.len() {
            (&a.limbs, &b.limbs)
        } else {
            (&b.limbs, &a.limbs)
        };

        if p1.is_empty() {
            return Vec::new();
        }

        let len1 = p1.len();
        let len2 = p2.len();
        let mut dp = vec![0u32; len1 + len2];

        for i in 0..len1 {
            let digit1 = p1[i] as u64;
            let mut carry = 0u64;
            for j in 0..len2 {
                let digit2 = p2[j] as u64;
                carry += digit1 * digit2;
                carry += dp[i + j] as u64;
                dp[i + j] = carry as u32;
                carry >>= 32;
            }
            dp[i + len2] = carry as u32;
        }

        while dp.last().copied() == Some(0) {
            dp.pop();
        }
        dp
    }

    pub fn divide_limbs(a: &[u32], b: &[u32]) -> Vec<u32> {
        let aa = Mpn {
            limbs: a.to_vec(),
        };
        let mut bb = Mpn {
            limbs: b.to_vec(),
        };
        bb.normalize();
        assert!(!bb.is_zero(), "division by zero");

        match (aa.to_u128(), bb.to_u128()) {
            (Some(lhs), Some(rhs)) => Mpn::from_u128((lhs + rhs / 2) / rhs).limbs,
            _ => {
                if aa.limbs.len() < bb.limbs.len() {
                    Vec::new()
                } else if aa.limbs == bb.limbs {
                    vec![1]
                } else {
                    Vec::new()
                }
            }
        }
    }

    pub fn convert_to_decimal(a: &[u32], extra_zeroes: usize) -> String {
        let value = Mpn {
            limbs: a.to_vec(),
        }
        .to_u128()
        .unwrap_or(0);

        let mut out = String::new();
        for _ in 0..extra_zeroes {
            out.push('0');
        }
        let _ = write!(out, "{value}");
        while out.len() > 1 && out.ends_with('0') {
            out.pop();
        }
        if out.is_empty() {
            out.push('0');
        }
        out
    }

    pub fn decode_long_double(x: f64) -> Option<(i32, Vec<u32>)> {
        if !x.is_finite() || x.is_sign_negative() {
            return None;
        }
        Some(Self::decode_nonnegative_float(x, f64::MANTISSA_DIGITS as i32))
    }

    pub fn decode_double(x: f64) -> Option<(i32, Vec<u32>)> {
        if !x.is_finite() || x.is_sign_negative() {
            return None;
        }
        Some(Self::decode_nonnegative_float(x, f64::MANTISSA_DIGITS as i32))
    }

    fn decode_nonnegative_float(x: f64, mant_bits: i32) -> (i32, Vec<u32>) {
        let mut exp = 0i32;
        let mut y = x;
        if y != 0.0 {
            let e = y.abs().log2().floor() as i32 + 1;
            y /= 2f64.powi(e);
            exp = e;
        }
        let scaled = y * 2f64.powi(mant_bits);
        let mantissa = if scaled <= 0.0 {
            Vec::new()
        } else {
            Mpn::from_u128(scaled as u128).limbs
        };
        (exp - mant_bits, mantissa)
    }

    pub fn scale_10_round_decimal_decoded(e: i32, m: &[u32], n: i32) -> Option<String> {
        let mant = Mpn { limbs: m.to_vec() }.to_u128()? as f64;
        let value = mant * 2f64.powi(e);
        Some(format!("{:.*}", n.max(0) as usize, value))
    }

    pub fn scale_10_round_decimal_long_double(x: f64, n: i32) -> Option<String> {
        let (e, m) = Self::decode_long_double(x)?;
        Self::scale_10_round_decimal_decoded(e, &m, n)
    }

    pub fn scale_10_round_decimal_double(x: f64, n: i32) -> Option<String> {
        let (e, m) = Self::decode_double(x)?;
        Self::scale_10_round_decimal_decoded(e, &m, n)
    }

    pub fn floorlog_10_l(x: f64) -> i32 {
        Self::floorlog_10(x)
    }

    pub fn floorlog_10(x: f64) -> i32 {
        if x == 0.0 {
            return i32::MIN;
        }
        let l = x.abs().log10();
        let i = l as i32;
        i + if l < 0.0 { -1 } else { 0 }
    }

    pub fn is_borderline(digits: &str, precision: usize) -> bool {
        let bytes = digits.as_bytes();
        if bytes.len() < precision + 2 {
            return false;
        }
        if bytes.iter().take(precision).any(|&b| b != b'0') {
            return false;
        }
        bytes[precision] == b'1' && bytes.len() == precision + 1
    }

    pub fn max_room_needed(
        argument: Option<&str>,
        arg_index: usize,
        conversion: char,
        _arg_type: &str,
        flags: u32,
        width: usize,
        has_precision: bool,
        precision: usize,
        pad_ourselves: bool,
    ) -> usize {
        let base = match conversion {
            's' => {
                let len = argument.map(|s| {
                    if has_precision {
                        Self::local_strnlen(s.as_bytes(), precision)
                    } else {
                        s.len()
                    }
                }).unwrap_or(6);
                if has_precision { len.min(precision) } else { len }
            }
            'c' => 8,
            'f' | 'F' | 'e' | 'E' | 'g' | 'G' => {
                let prec = if has_precision { precision } else { 6 };
                32 + prec + usize::from(flags != 0)
            }
            'd' | 'i' | 'o' | 'u' | 'x' | 'X' => {
                let prec = if has_precision { precision } else { 1 };
                32.max(prec + 3)
            }
            _ => 64,
        };
        let with_padding = if pad_ourselves { base.max(width) } else { base };
        with_padding.max(arg_index.saturating_add(1))
    }

    pub fn r#if<T>(condition: bool, when_true: T, when_false: T) -> T {
        if condition { when_true } else { when_false }
    }

    pub fn pad_ourselves(width: usize, content_len: usize, enabled: bool) -> usize {
        if enabled { width.saturating_sub(content_len) } else { 0 }
    }

    pub fn module_gnu_scale_10_round_14(x: f64, n: i32) -> Option<String> {
        Self::scale_10_round_decimal_double(x, n)
    }

    pub fn module_gnu_is_infinite_18(x: f64) -> bool {
        Self::is_infinite_or_zero(x)
    }

    pub fn gnu_is_infinite(x: f64) -> bool {
        x.is_infinite()
    }

    pub fn module_gnu_if_10<T>(condition: bool, when_true: T, when_false: T) -> T {
        Self::r#if(condition, when_true, when_false)
    }

    pub fn arg_index(index: usize, len: usize) -> Option<usize> {
        (index < len).then_some(index)
    }

    pub fn module_gnu_if_11<T>(condition: bool, when_true: T, when_false: T) -> T {
        Self::r#if(condition, when_true, when_false)
    }
}
