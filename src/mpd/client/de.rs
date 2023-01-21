use std::error;
use std::fmt;
use std::result;
use std::str;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::ParseFloatError;
use std::num::ParseIntError;
use std::str::Utf8Error;

use serde::de;
use serde::de::DeserializeSeed;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::de::Visitor;
use serde::Deserialize;

type Result<T> = result::Result<T, InternalError>;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl error::Error for Error {
    // default
}

impl From<InternalError> for Error {
    fn from(err: InternalError) -> Self {
        Error { message: err.to_string() }
    }
}

#[derive(Debug)]
enum InternalError {
    Custom(String),
    ParsingError(String),
    BinaryStart,
    DuplicateKey,
    EndOfInnerSeq,
    WrongStep,
}

impl de::Error for InternalError {
    fn custom<T>(msg: T) -> Self where T: Display {
        InternalError::Custom(msg.to_string())
    }
}

impl Display for InternalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use InternalError::*;

        let msg = match self {
            Custom(msg) | ParsingError(msg) => msg,
            BinaryStart => "binary start",
            DuplicateKey => "duplicate key",
            EndOfInnerSeq => "end of inner seq",
            WrongStep => "wrong step",
        };

        f.write_str(msg)
    }
}

impl error::Error for InternalError {
    // default
}

#[derive(Debug, PartialEq)]
enum Step<'de> {
    Key,
    Value(&'de str),
    Binary(usize),
}

struct Deserializer<'de> {
    input: &'de [u8],
    step: Step<'de>,
    seq_level: i8,
    first_key: Option<&'de str>,
    last_key: Option<&'de str>,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer {
            input,
            step: Step::Key,
            seq_level: 0,
            first_key: None,
            last_key: None,
        }
    }
}

pub fn from_bytes<'a, T>(s: &'a [u8]) -> result::Result<T, Error>
    where T: Deserialize<'a>
{
    let mut deserializer = Deserializer::from_bytes(s);

    let t = T::deserialize(&mut deserializer)?;

    Ok(t)
}

mod parsing {
    use std::str;

    use super::InternalError;
    use super::Deserializer;
    use super::Result;
    use super::Step;

    const BINARY_KEY: &str = "binary";
    const LINE_SEPARATOR: u8 = 0xA;
    const KEY_VALUE_SEPARATOR: &[u8] = b": ";

    impl<'de> Deserializer<'de> {
        pub fn parse_str(&mut self) -> Result<&'de str> {
            match self.step {
                Step::Key => {
                    let (key, offset) = self.parse_key()?;

                    if self.is_inner_seq() {
                        if self.last_key != Some(key) {
                            return Err(InternalError::EndOfInnerSeq);
                        }

                        self.step = Step::Value(key);
                        self.advance(offset);

                        return self.parse_str();
                    }

                    match self.first_key {
                        None => {
                            self.first_key = Some(key);
                        },
                        Some(first_key) if key == first_key => {
                            self.first_key = None;

                            return Err(InternalError::DuplicateKey);
                        },
                        _ => (),
                    };

                    self.step = Step::Value(key);
                    self.advance(offset);

                    self.last_key = Some(key);

                    Ok(key)
                },
                Step::Value(key) => {
                    let (value, offset) = self.parse_value()?;

                    self.step = if key == BINARY_KEY {
                        Step::Binary(value.parse()?)
                    } else {
                        Step::Key
                    };

                    self.advance(offset);

                    Ok(value)
                },
                Step::Binary(_) => {
                    Err(InternalError::BinaryStart)
                },
            }
        }

        fn parse_key(&mut self) -> Result<(&'de str, usize)> {
            self.input.windows(2).position(|w| w == KEY_VALUE_SEPARATOR)
                .map(|len| {
                    let str = str::from_utf8(&self.input[..len])?;

                    Ok((str, len + 2))
                })
                .unwrap_or_else(|| Err(InternalError::ParsingError("can't find key-value separator".to_owned())))
        }

        fn parse_value(&mut self) -> Result<(&'de str, usize)> {
            self.input.iter().position(|&x| x == LINE_SEPARATOR)
                .map(|len| {
                    let s = str::from_utf8(&self.input[..len])?;

                    Ok((s, len + 1))
                })
                .unwrap_or_else(|| Err(InternalError::ParsingError("can't find line separator".to_string())))
        }

        fn advance(&mut self, n: usize) {
            self.input = &self.input[n..];
        }

        fn is_inner_seq(&self) -> bool {
            self.seq_level > 1
        }
    }
}

impl From<ParseIntError> for InternalError {
    fn from(err: ParseIntError) -> Self {
        InternalError::ParsingError(err.to_string())
    }
}

impl From<ParseFloatError> for InternalError {
    fn from(err: ParseFloatError) -> Self {
        InternalError::ParsingError(err.to_string())
    }
}

impl From<Utf8Error> for InternalError {
    fn from(err: Utf8Error) -> Self {
        InternalError::ParsingError(err.to_string())
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = InternalError;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        unimplemented!("deserialize_any is not supported");
    }

    #[inline]
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let str = self.parse_str()?;

        visitor.visit_bool(str == "1")
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_i8(self.parse_str()?.parse()?)
    }

    #[inline]
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_i16(self.parse_str()?.parse()?)
    }

    #[inline]
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_i32(self.parse_str()?.parse()?)
    }

    #[inline]
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_i64(self.parse_str()?.parse()?)
    }

    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_u8(self.parse_str()?.parse()?)
    }

    #[inline]
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_u16(self.parse_str()?.parse()?)
    }

    #[inline]
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_u32(self.parse_str()?.parse()?)
    }

    #[inline]
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_u64(self.parse_str()?.parse()?)
    }

    #[inline]
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_f32(self.parse_str()?.parse()?)
    }

    #[inline]
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_f64(self.parse_str()?.parse()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        unimplemented!("deserialize_char is not supported");
    }

    #[inline]
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_borrowed_str(self.parse_str()?)
    }

    #[inline]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        if let Step::Binary(size) = self.step {
            let bytes = &self.input[..size];

            self.input = &self.input[size..];

            visitor.visit_borrowed_bytes(bytes)
        } else {
            Err(InternalError::WrongStep)
        }
    }

    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        self.deserialize_bytes(visitor)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        if self.input.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    #[inline]
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value> where V: Visitor<'de> {
        unimplemented!("deserialize_unit is not supported");
    }

    #[inline]
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        self.seq_level += 1;

        visitor.visit_seq(self)
    }

    #[inline]
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        self.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_tuple_struct<V>(self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value> where V: Visitor<'de> {
        self.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_map(self)
    }

    #[inline]
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> where V: Visitor<'de> {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value> where V: Visitor<'de> {
        unimplemented!("deserialize_enum is not supported");
    }

    #[inline]
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        visitor.visit_borrowed_str(self.parse_str()?)
    }

    #[inline]
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        self.deserialize_str(visitor)
    }
}

impl<'de> SeqAccess<'de> for Deserializer<'de> {
    type Error = InternalError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
        where T: DeserializeSeed<'de>
    {
        if self.input.is_empty() {
            self.seq_level -= 1;

            Ok(None)
        } else {
            match seed.deserialize(&mut *self) {
                Ok(x) => {
                    Ok(Some(x))
                },
                Err(InternalError::EndOfInnerSeq) => {
                    self.seq_level -= 1;

                    Ok(None)
                },
                Err(err) => {
                    Err(err)
                },
            }
        }
    }
}

impl<'de> MapAccess<'de> for Deserializer<'de> {
    type Error = InternalError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
        where K: DeserializeSeed<'de>,
    {
        if self.input.is_empty() {
            Ok(None)
        } else {
            match seed.deserialize(&mut *self) {
                Ok(x) => Ok(Some(x)),
                Err(InternalError::DuplicateKey | InternalError::BinaryStart) => Ok(None),
                Err(err) => Err(err),
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
        where V: DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self)
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn should_deserialize_struct() {
        use serde::Deserialize;

        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            b: i64,
            a: String,
        }

        let input = b"a: value\nb: 64\n";

        let result: Test = super::from_bytes(input).unwrap();

        assert_eq!(result, Test {
            a: "value".to_owned(),
            b: 64,
        });
    }

    #[test]
    fn should_deserialize_boolean() {
        use serde::Deserialize;

        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: bool,
            b: bool,
            d: String,
        }

        let input = b"a: 1\nb: 0\nd: value\n";

        let result: Test = super::from_bytes(input).unwrap();

        assert_eq!(
            result,
            Test {
                a: true,
                b: false,
                d: "value".to_owned()
            },
        );
    }

    #[test]
    fn should_deserialize_int() {
        use serde::Deserialize;

        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: i8,
            b: i16,
            c: i32,
            d: i64,
        }

        let input = format!(
            "a: {}\nb: {}\nc: {}\nd: {}\n",
            i8::MAX,
            i16::MIN,
            i32::MAX,
            i64::MIN,
        );

        let result: Test = super::from_bytes(input.as_bytes()).unwrap();

        assert_eq!(result, Test {
            a: i8::MAX,
            b: i16::MIN,
            c: i32::MAX,
            d: i64::MIN,
        });
    }

    #[test]
    fn should_deserialize_uint() {
        use serde::Deserialize;

        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: u8,
            b: u16,
            c: u32,
            d: u64,
            e: usize,
        }

        let input = format!(
            "a: {}\nb: {}\nc: {}\nd: {}\ne: {}\n",
            u8::MAX,
            u16::MIN,
            u32::MAX,
            u64::MIN,
            usize::MAX,
        );

        let result: Test = super::from_bytes(input.as_bytes()).unwrap();

        assert_eq!(result, Test {
            a: u8::MAX,
            b: u16::MIN,
            c: u32::MAX,
            d: u64::MIN,
            e: usize::MAX,
        });
    }

    #[test]
    fn should_deserialize_float() {
        use serde::Deserialize;

        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: f32,
            b: f64,
        }

        let input = format!(
            "a: {}\nb: {}\n",
            f32::MAX,
            f64::MIN,
        );

        let result: Test = super::from_bytes(input.as_bytes()).unwrap();

        assert_eq!(result, Test {
            a: f32::MAX,
            b: f64::MIN,
        });
    }

    #[test]
    fn should_deserialize_map() {
        let input = b"a: value\nb: bvalue\n";

        let result: HashMap<&str, &str> = super::from_bytes(input).unwrap();

        assert_eq!(result, HashMap::from([
            ("b", "bvalue"),
            ("a", "value"),
        ]));
    }

    #[test]
    fn should_deserialize_seq() {
        use serde::Deserialize;

        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: String,
            b: String,
        }

        let input = b"a: value\nb: bvalue\na: value\nb: bvalue\n";

        let result: Vec<Test> = super::from_bytes(input).unwrap();

        assert_eq!(
            result,
            vec![
                Test {
                    a: "value".to_owned(),
                    b: "bvalue".to_owned(),
                },
                Test {
                    a: "value".to_owned(),
                    b: "bvalue".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn should_deserialize_inner_seq() {
        use serde::Deserialize;

        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            b: String,
            #[serde(default)]
            a: Vec<String>,
        }

        let input = b"b: value\na: 1value\nb: bvalue\na: 1value\na: 2value\na: 3value\nb: bvalue\n";

        let result: Vec<Test> = super::from_bytes(input).unwrap();

        assert_eq!(
            result,
            vec![
                Test {
                    a: vec!["1value".to_owned()],
                    b: "value".to_owned(),
                },
                Test {
                    a: vec!["1value".to_owned(), "2value".to_owned(), "3value".to_owned()],
                    b: "bvalue".to_owned(),
                },
                Test {
                    a: vec![],
                    b: "bvalue".to_owned(),
                },
            ]
        );
    }

    #[test]
    fn should_deserialize_bytes() {
        use serde::Deserialize;

        #[derive(Debug, PartialEq, Deserialize)]
        struct BinaryInfo {
            size: usize,
            binary: usize,
        }

        #[derive(Debug, PartialEq, Deserialize)]
        struct Test<'a>(
            BinaryInfo,
            #[serde(with = "serde_bytes")]
            &'a [u8],
        );

        let input_info = b"size: 25\nbinary: 5\n";
        let input_data: &[u8] = &[0xB, 0xB, 0xA, 0xB, 0xB];

        let input = [input_info, input_data, b"\n"].concat();

        let result: Test = super::from_bytes(&input).unwrap();

        assert_eq!(
            result,
            Test(
                BinaryInfo {
                    size: 25,
                    binary: 5,
                },
                input_data,
            )
        );
    }

    #[test]
    fn should_deserialize_byte_buf() {
        use serde::Deserialize;

        #[derive(Debug, PartialEq, Deserialize)]
        struct BinaryInfo {
            size: usize,
            binary: usize,
        }

        #[derive(Debug, PartialEq, Deserialize)]
        struct Test(
            BinaryInfo,
            #[serde(with = "serde_bytes")]
            Vec<u8>,
        );

        let input_info = b"size: 25\nbinary: 5\n";
        let input_data: &[u8] = &[0xB, 0xB, 0xA, 0xB, 0xB];

        let input = [input_info, input_data, b"\n"].concat();

        let result: Test = super::from_bytes(&input).unwrap();

        assert_eq!(
            result,
            Test(
                BinaryInfo {
                    size: 25,
                    binary: 5,
                },
                input_data.to_vec(),
            )
        );
    }

    #[test]
    fn should_deserialize_option_value() {
        use serde::Deserialize;

        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: Option<i32>,
            b: i32,
        }

        let input = b"b: 2\n";

        let result: Test = super::from_bytes(input).unwrap();

        assert_eq!(
            result,
            Test {
                a: None,
                b: 2,
            },
        );
    }

    #[test]
    fn should_deserialize_option_wrapper() {
        use serde::Deserialize;

        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: i32,
        }

        let input = b"";

        let result: Option<Vec<Test>> = super::from_bytes(input).unwrap();

        assert_eq!(result, None);

        let input = b"a: 2\n";

        let result: Option<Vec<Test>> = super::from_bytes(input).unwrap();

        assert_eq!(
            result,
            Some(
                vec![Test { a: 2 }],
            ),
        );
    }

    #[test]
    fn should_deserialize_enum() {
        use serde::Deserialize;

        #[derive(Debug, PartialEq, Deserialize)]
        struct Test {
            a: TestEnum,
        }

        #[derive(Debug, PartialEq, Deserialize)]
        #[serde(from = "&str")]
        enum TestEnum {
            A,
            B,
            C,
        }

        impl From<&str> for TestEnum {
            fn from(str: &str) -> Self {
                match str {
                    "avalue" => TestEnum::A,
                    "bvalue" => TestEnum::B,
                    "cvalue" => TestEnum::C,
                    _ => panic!("could not match string"),
                }
            }
        }

        let input = b"a: cvalue\n";

        let result: Test = super::from_bytes(input).unwrap();

        assert_eq!(
            result,
            Test { a: TestEnum::C },
        );
    }

    #[test]
    fn should_deserialize_unit() {
        let input = b"";

        super::from_bytes::<()>(input).unwrap();
    }
}
