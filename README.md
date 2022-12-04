## Roadmap

### TODO:
* Rename inner generated `validate` and `sanitize` methods into something more unique. Otherwise it may conflict because of `use super::*;`
* Rename default inner modules into something less scary
* Finish implementation of FromStr wtih validation for integers and floats. Ensure there is a test coverage.
* Support serde
  * Serialize
  * Deserialize
* Support Arbitrary
* Support decimals libraries:
  * https://crates.io/crates/rust_decimal
* Regex
  * See https://github.com/CryptArchy/regex_generate to impl support with arbitrary
* Support time libraries (e.g. chrono, time)
* Impl  "did you mean" hints:
  * E.g. unknown validation rule `min`. Did you mean `min_len`?
* Finalize syntax!
* Setup CI
* Address TODOs
* Refactor parsers
* Number sanitizers:
  * Replace `clamp(a, b)` with something like `min = a, max = b`
* String sanitizers:
  * capitalize
  * truncate
  * Remove extra spaces
* Extra validations for floats:
  * `is_number` / `is_finite` (aka not NaN, and not `Inifinity`)
  * This should allow to derive Eq and Ord
* Generate documentation automatically.
* Intercept derive of DerefMut, AsMut, BorrowMut and print an explaining error message
* Rename nutype_derive to nutype_macros or something else?
* Rename nutype_test_suite to `test_suite ` ?


### Done
* Custom sanitizers for strings
* Custom validators for strings
* Custom sanitizers for numbers
* Custom validators for numbers
* Setup compiletests
* Use `new`, instead of `from` and `try_from`
* Respect visibility
* Respect documentation
* Implement std::error::Error for errors
* Support derive for String
* Support derive of From and TryFrom for String types
* Add UI tests to detect conflicts:
  * derive(TryFrom) without validations
  * derive(From) with validations
* Support derive Hash for String
* Impl FromStr for String types
* Support derive of Borrow<str> and Borrow<String> for String types
* Refactor numbers and split into groups: integer and float.
* Support derive for integers
* Support derive for floats
* Support derive of Into trait for String
* Support derive of Into trait for integers
* Support derive of Into trait for floats
* Refactor: extract common generator functions
* Cleanup tests: split number tests into integer and float
* Use absolute path to `Result` in the generated code



## Similar crates

* bounded_integer
* semval
* refinement
