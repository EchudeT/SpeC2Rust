# spec.md

## Title

Functional Specification for Rust Port of `module_gnu_vasnprintf.c_53`

## Status

Draft

## Scope

This specification covers the floating-point decoding and decimal-magnitude support logic evidenced in `gnu/vasnprintf.c` for:

- decoding `double` and `long double` values into an internal multiprecision-oriented representation,
- computing the base-10 magnitude floor for `double` and `long double`,
- detecting borderline digit sequences relevant to formatting decisions.

The Rust rewrite on branch `059-module_gnu_vasnprintf.c_53-rust-port` must preserve the observable behavior of this module within the formatting pipeline of the project.

## References

### Source Files

- `gnu/vasnprintf.c`

### Traced Functions

- `decode_long_double`
- `decode_double`
- `floorlog10l`
- `floorlog10`
- `is_borderline`

### Traced Data Types

- anonymous struct at `gnu/vasnprintf.c:426-430`

---

## 1. Feature Specification

### 1.1 Overview

This module provides numeric support used by formatted output logic when handling floating-point values. Its role is to transform binary floating-point inputs into intermediate data suitable for decimal formatting decisions and to supply helper classification used around decimal-digit boundaries.

The Rust version must implement the same functional boundaries evidenced by the C module:

1. **Floating-point decomposition for formatting**
   - Accept a `double` or `long double` input.
   - Derive an internal representation consisting of:
     - a returned storage object representing the decoded significand payload,
     - an exponent output,
     - a multiprecision-oriented output descriptor.
   - Support use by later formatting stages that need exact or controlled conversion behavior rather than naive decimal rendering.

2. **Base-10 magnitude estimation**
   - Compute the floor of the decimal logarithm for positive floating-point values of both supported precisions.
   - Produce values suitable for formatting decisions such as digit count, scaling, or exponent-style output selection.

3. **Borderline decimal-digit detection**
   - Inspect a digit sequence and a precision value.
   - Identify whether the sequence is on a formatting boundary that requires special handling.

### 1.2 Functional Boundary

The Rust port must remain limited to the evidenced behavior of this module:

- It is a support component for floating-point formatting.
- It operates on `double`-precision and `long double`-precision inputs.
- It provides decimal-order and digit-boundary helpers.
- It does not define a broader public formatting API beyond what is implied by these functions.
- It must not introduce new formatting modes or numeric classes not evidenced by the source analysis.

### 1.3 Inputs and Outputs

#### Floating-point decode operations

For both supported floating-point widths, the module must:

- accept a finite floating-point value as input,
- produce:
  - an exponent through an output parameter equivalent,
  - a multiprecision descriptor through an output parameter equivalent,
  - a returned object representing the decoded numeric payload.

The exact Rust type design may differ from C, but all information carried by the C behavior must remain available to downstream formatting logic.

#### Decimal logarithm floor operations

For `double` and `long double`, the module must:

- accept a floating-point input,
- return an integer representing `floor(log10(x))` behavior as used by the formatter.

#### Borderline detection

The module must:

- accept a decimal digit string and a precision,
- return a boolean-equivalent result indicating whether the digits are on a formatting boundary.

---

## 2. User Scenarios & Testing

### 2.1 Scenario: Formatting pipeline decodes a `double`

A formatting path receives a `double` value that must be rendered accurately. Before digits are emitted, the formatter calls the decode logic to obtain an internal significand/exponent form.

The Rust version must support:

- ordinary positive values,
- ordinary negative values if sign is handled externally and magnitude is decoded here,
- normalized values,
- subnormal values if accepted by the original formatting path,
- values whose binary representation requires exact downstream rounding decisions.

**Testing implications**
- Verify that decoding a representative set of `double` values yields internally consistent exponent and payload data sufficient for decimal formatting.
- Verify stable behavior across powers of two, powers of ten, and non-terminating decimal cases.

### 2.2 Scenario: Formatting pipeline decodes a `long double`

A formatting path receives a `long double` value and requires the same style of internal decomposition for high-precision formatting support.

The Rust version must support:

- values that fit the platform `long double` semantics represented by the original module,
- values near decimal exponent boundaries,
- values that need correct handling beyond plain `double` precision.

**Testing implications**
- Verify that representative `long double` values can be decoded into exponent plus multiprecision-oriented payload.
- Verify that the resulting internal form is suitable for the same downstream formatting decisions expected by the C module.

### 2.3 Scenario: Formatter chooses decimal scale

Before deciding digit placement or exponent notation, the formatter needs the decimal order of magnitude of a positive floating-point input.

The Rust version must support:

- computing decimal magnitude for positive `double`,
- computing decimal magnitude for positive `long double`,
- correct results around powers of ten and immediately adjacent values.

**Testing implications**
- Test exact powers of ten where representable.
- Test values just below and just above decimal boundaries.
- Test very small and very large positive values within supported finite ranges.

### 2.4 Scenario: Formatter checks a borderline digit sequence

A decimal digit buffer has been generated and the formatter must determine whether the digits lie on a boundary that affects rounding or final representation choice.

The Rust version must support:

- evaluating a digit sequence with a supplied precision,
- returning the same borderline/non-borderline decision as the C module.

**Testing implications**
- Test digit sequences that are clearly non-boundary cases.
- Test sequences at boundary patterns relevant to rounding-sensitive formatting.
- Test multiple precision values against the same digit prefix where the decision may differ.

---

## 3. Requirements

### 3.1 Functional Requirements

#### FR-1 Floating-point decode for `double`
The module shall decode `double` input into an internal representation that includes:
- a returned payload object,
- an output exponent,
- an output multiprecision descriptor.

**Traceability:** `decode_double` in `gnu/vasnprintf.c`

#### FR-2 Floating-point decode for `long double`
The module shall decode `long double` input into an internal representation that includes:
- a returned payload object,
- an output exponent,
- an output multiprecision descriptor.

**Traceability:** `decode_long_double` in `gnu/vasnprintf.c`

#### FR-3 Decimal magnitude floor for `double`
The module shall compute the floor of the base-10 logarithm for `double` input in the manner required by floating-point formatting decisions.

**Traceability:** `floorlog10` in `gnu/vasnprintf.c`

#### FR-4 Decimal magnitude floor for `long double`
The module shall compute the floor of the base-10 logarithm for `long double` input in the manner required by floating-point formatting decisions.

**Traceability:** `floorlog10l` in `gnu/vasnprintf.c`

#### FR-5 Borderline digit classification
The module shall classify whether a decimal digit sequence at a given precision is a borderline case for formatting-sensitive handling.

**Traceability:** `is_borderline` in `gnu/vasnprintf.c`

#### FR-6 Formatter-oriented interoperability
The outputs of the decode and helper operations shall be usable together by the surrounding formatted-output logic to support decimal rendering decisions.

**Traceability:** combined role of `decode_double`, `decode_long_double`, `floorlog10`, `floorlog10l`, and `is_borderline` in `gnu/vasnprintf.c`

### 3.2 Key Entities

#### KE-1 Decoded floating-point payload
A decoded floating-point payload is the returned object from the decode operations. It represents the extracted numeric content needed by later formatting logic.

**Traceability:** return values of `decode_double` and `decode_long_double`

#### KE-2 Decimal exponent output
An integer exponent output accompanies decoding and represents exponent information derived from the source floating-point value.

**Traceability:** `int *ep` parameter of `decode_double` and `decode_long_double`

#### KE-3 Multiprecision descriptor
A multiprecision-oriented descriptor accompanies decoding and carries metadata needed for subsequent decimal conversion or formatting steps.

**Traceability:** `mpn_t *mp` parameter of `decode_double` and `decode_long_double`

#### KE-4 Borderline digit sequence
A digit sequence plus precision value forms the input to borderline classification and represents candidate formatted digits under evaluation.

**Traceability:** `const char *digits, size_t precision` in `is_borderline`

#### KE-5 Local supporting struct
The anonymous struct defined in `gnu/vasnprintf.c:426-430` is a local supporting data structure participating in this module’s internal state handling. The Rust port must preserve any behaviorally relevant information carried through this structure where required by the traced functions.

**Traceability:** anonymous struct at `gnu/vasnprintf.c:426-430`

### 3.3 Relationships

- A floating-point input is transformed into a decoded payload plus exponent and multiprecision descriptor.
- Decimal magnitude helpers operate on floating-point inputs to guide placement and scaling decisions in formatting.
- Borderline classification operates on generated decimal digits and precision to determine whether the formatted result lies on a sensitive boundary.
- These entities are consumed by the same overall formatted-output workflow in `gnu/vasnprintf.c`.

---

## 4. Success Criteria

### 4.1 Behavioral Equivalence

#### SC-1 Decode equivalence for `double`
For a conformance set of representative finite `double` inputs, the Rust port shall produce decoding results that drive the same formatting-relevant behavior as the C module.

**Traceability:** `decode_double`

#### SC-2 Decode equivalence for `long double`
For a conformance set of representative finite `long double` inputs, the Rust port shall produce decoding results that drive the same formatting-relevant behavior as the C module.

**Traceability:** `decode_long_double`

#### SC-3 Decimal magnitude correctness for `double`
For representative positive finite `double` inputs, including values around powers of ten, the Rust port shall return the same decimal magnitude floor as the C module.

**Traceability:** `floorlog10`

#### SC-4 Decimal magnitude correctness for `long double`
For representative positive finite `long double` inputs, including values around powers of ten, the Rust port shall return the same decimal magnitude floor as the C module.

**Traceability:** `floorlog10l`

#### SC-5 Borderline classification equivalence
For representative digit strings and precision values, especially boundary-sensitive cases, the Rust port shall return the same borderline classification as the C module.

**Traceability:** `is_borderline`

### 4.2 Integration Readiness

#### SC-6 Formatter compatibility
When used by the surrounding floating-point formatting logic, the Rust module shall preserve the same decision-relevant outcomes for:
- decimal scaling,
- boundary detection,
- decode-driven formatting steps.

**Traceability:** combined behavior of all traced functions in `gnu/vasnprintf.c`

### 4.3 Test Completion

#### SC-7 Required test coverage areas
The Rust port shall include tests covering at minimum:
- representative normalized and subnormal `double` decode cases,
- representative `long double` decode cases,
- decimal magnitude calculations near powers of ten for both precisions,
- borderline and non-borderline digit-sequence cases.

**Traceability:** `decode_double`, `decode_long_double`, `floorlog10`, `floorlog10l`, `is_borderline`