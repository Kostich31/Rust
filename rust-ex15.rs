#[derive(Debug, PartialEq, Eq)]
enum WireType {
    Varint,
    Len,
}

#[derive(Debug)]
enum FieldValue<'a> {
    Varint(u64),
    Len(&'a [u8]),
}

#[derive(Debug)]
struct Field<'a> {
    field_num: u64,
    value: FieldValue<'a>,
}

trait ProtoMessage<'a>: Default {
    fn add_field(&mut self, field: Field<'a>);
}

impl From<u64> for WireType {
    fn from(value: u64) -> Self {
        match value {
            0 => WireType::Varint,
            2 => WireType::Len,
            _ => panic!("Invalid wire type: {value}"),
        }
    }
}

impl<'a> FieldValue<'a> {
    fn as_str(&self) -> &'a str {
        match self {
            FieldValue::Len(data) => std::str::from_utf8(data).expect("Invalid string"),
            _ => panic!("Expected Len field for string"),
        }
    }

    fn as_bytes(&self) -> &'a [u8] {
        match self {
            FieldValue::Len(data) => data,
            _ => panic!("Expected Len field for bytes"),
        }
    }

    fn as_u64(&self) -> u64 {
        match self {
            FieldValue::Varint(value) => *value,
            _ => panic!("Expected Varint field for u64"),
        }
    }
}

fn parse_varint(data: &[u8]) -> (u64, &[u8]) {
    for i in 0..7 {
        let Some(b) = data.get(i) else {
            panic!("Not enough bytes for varint");
        };
        if b & 0x80 == 0 {
            let mut value = 0u64;
            for b in data[..=i].iter().rev() {
                value = (value << 7) | (b & 0x7f) as u64;
            }
            return (value, &data[i + 1..]);
        }
    }
    panic!("Слишком много байтов для varint");
}

fn unpack_tag(tag: u64) -> (u64, WireType) {
    let field_num = tag >> 3;
    let wire_type = WireType::from(tag & 0x7);
    (field_num, wire_type)
}

fn parse_field(data: &[u8]) -> (Field, &[u8]) {
    let (tag, remainder) = parse_varint(data);
    let (field_num, wire_type) = unpack_tag(tag);
    
    let (value, new_remainder) = match wire_type {
        WireType::Varint => {
            let (val, rem) = parse_varint(remainder);
            (FieldValue::Varint(val), rem)
        }
        WireType::Len => {
            let (len, rem) = parse_varint(remainder);
            let end = len as usize;
            if end > rem.len() {
                panic!("Invalid length for Len type");
            }
            let data = &rem[..end];
            (FieldValue::Len(data), &rem[end..])
        }
    };
    
    (Field { field_num, value }, new_remainder)
}

fn parse_message<'a, T: ProtoMessage<'a>>(data: &'a [u8]) -> T {
    let mut result = T::default();
    let mut remaining = data;
    while !remaining.is_empty() {
        let (field, rem) = parse_field(remaining);
        result.add_field(field);
        remaining = rem;
    }
    result
}

#[derive(Debug, Default, PartialEq)]
struct PhoneNumber<'a> {
    number: &'a str,
    type_: &'a str,
}

impl<'a> ProtoMessage<'a> for PhoneNumber<'a> {
    fn add_field(&mut self, field: Field<'a>) {
        match field.field_num {
            1 => self.number = field.value.as_str(),
            2 => self.type_ = field.value.as_str(),
            _ => panic!("Invalid field number for PhoneNumber"),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
struct Person<'a> {
    name: &'a str,
    id: u64,
    phone: Vec<PhoneNumber<'a>>,
}

impl<'a> ProtoMessage<'a> for Person<'a> {
    fn add_field(&mut self, field: Field<'a>) {
        match field.field_num {
            1 => self.name = field.value.as_str(),
            2 => self.id = field.value.as_u64(),
            3 => {
                let phone = parse_message::<PhoneNumber>(field.value.as_bytes());
                self.phone.push(phone);
            }
            _ => panic!("Invalid field number for Person"),
        }
    }
}

fn main() {
    let person: Person = parse_message(&[0x10, 0x2a]);
    assert_eq!(person, Person { name: "", id: 42, phone: vec![] });

    let person: Person = parse_message(&[
        0x0a, 0x0e, 0x62, 0x65, 0x61, 0x75, 0x74, 0x69, 0x66, 0x75, 0x6c, 0x20,
        0x6e, 0x61, 0x6d, 0x65,
    ]);
    assert_eq!(person, Person { name: "beautiful name", id: 0, phone: vec![] });

    let person: Person = parse_message(&[0x0a, 0x04, 0x45, 0x76, 0x61, 0x6e, 0x10, 0x16]);
    assert_eq!(person, Person { name: "Evan", id: 22, phone: vec![] });

    let person: Person = parse_message(&[
        0x0a, 0x00, 0x10, 0x00, 0x1a, 0x16, 0x0a, 0x0e, 0x2b, 0x31, 0x32, 0x33,
        0x34, 0x2d, 0x37, 0x37, 0x37, 0x2d, 0x39, 0x30, 0x39, 0x30, 0x12, 0x04,
        0x68, 0x6f, 0x6d, 0x65,
    ]);
    assert_eq!(
        person,
        Person {
            name: "",
            id: 0,
            phone: vec![PhoneNumber { number: "+1234-777-9090", type_: "home" },],
        }
    );

    let person: Person = parse_message(&[
        0x0a, 0x07, 0x6d, 0x61, 0x78, 0x77, 0x65, 0x6c, 0x6c, 0x10, 0x2a, 0x1a,
        0x16, 0x0a, 0x0e, 0x2b, 0x31, 0x32, 0x30, 0x32, 0x2d, 0x35, 0x35, 0x35,
        0x2d, 0x31, 0x32, 0x31, 0x32, 0x12, 0x04, 0x68, 0x6f, 0x6d, 0x65, 0x1a,
        0x18, 0x0a, 0x0e, 0x2b, 0x31, 0x38, 0x30, 0x30, 0x2d, 0x38, 0x36, 0x37,
        0x2d, 0x35, 0x33, 0x30, 0x38, 0x12, 0x06, 0x6d, 0x6f, 0x62, 0x69, 0x6c,
        0x65,
    ]);
    assert_eq!(
        person,
        Person {
            name: "maxwell",
            id: 42,
            phone: vec![
                PhoneNumber { number: "+1202-555-1212", type_: "home" },
                PhoneNumber { number: "+1800-867-5308", type_: "mobile" },
            ]
        }
    );
}
