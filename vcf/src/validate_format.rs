use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

enum NumberField {
    Number(u32),
    A,   // The field has one value per alternate allele
    R,   // The field has one value for each possible allele
    G,   // The field has one value for each possible genotype
    Dot, // The number of possible values varies, is unknown or unbounded
}

// use static dispatch for Info field parser
enum DataType {
    Integer(NumberField),
    Float(NumberField),
    Flag,
    Character(NumberField),
    String(NumberField),
}

struct InfoFormat {
    fieldtype: DataType,
    description: String,
    source: Option<String>,
    version: Option<String>,
}

fn parse_number_field(number: Option<&&str>) -> Result<NumberField, Box<dyn std::error::Error>> {
    match number {
        Some(&"A") => Ok(NumberField::A),
        Some(&"R") => Ok(NumberField::R),
        Some(&"G") => Ok(NumberField::G),
        Some(&".") => Ok(NumberField::Dot),
        Some(n) => {
            let num = u32::from_str(n)?;
            Ok(NumberField::Number(num))
        }
        None => Err("Number field not found".into()),
    }
}

/// - [X] Check has all and only required keys.
/// - [ ] Check that values are of the required types.
fn is_valid_format(input: HashMap<&str, &str>) -> bool {
    return has_required_keys(&input)
        & has_valid_type_value(&input)
        & has_valid_number_value(&input);
}

fn has_required_keys(input: &HashMap<&str, &str>) -> bool {
    let keys: HashSet<&str> = input.keys().copied().collect();
    let required_keys = HashSet::from(["ID", "Number", "Type", "Description"]);
    return required_keys == keys;
}

fn parse_type_value(
    info_map: &HashMap<&str, &str>,
) -> Result<DataType, Box<dyn std::error::Error>> {
    match info_map.get("Type") {
        Some(&"Integer") => Ok(DataType::Integer(parse_number_field(
            info_map.get("Number"),
        )?)),
        Some(&"Float") => Ok(DataType::Float(parse_number_field(info_map.get("Number"))?)),
        Some(&"Flag") => Ok(DataType::Flag),
        Some(&"Character") => Ok(DataType::Character(parse_number_field(
            info_map.get("Number"),
        )?)),
        Some(&"String") => Ok(DataType::String(parse_number_field(
            info_map.get("Number"),
        )?)),
        _ => Err("Invalid data type".into()),
    }
}

fn has_valid_type_value(input: &HashMap<&str, &str>) -> bool {
    parse_type_value(input).is_ok()
}

fn has_valid_number_value(input: &HashMap<&str, &str>) -> bool {
    parse_number_field(input.get("Number")).is_ok()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn returns_true_when_all_fields_present_and_value_types_correct() {
        let valid_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "3"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(valid_input);

        assert_eq!(result, true);
    }

    #[test]
    fn returns_false_if_missing_id() {
        let missing_id_input = HashMap::from([
            ("Number", "3"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(missing_id_input);

        assert_eq!(result, false);
    }

    #[test]
    fn returns_false_if_missing_number() {
        let missing_number_input = HashMap::from([
            ("ID", "ID123"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(missing_number_input);

        assert_eq!(result, false);
    }

    #[test]
    fn returns_false_if_missing_type() {
        let missing_type_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "3"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(missing_type_input);

        assert_eq!(result, false);
    }

    #[test]
    fn returns_false_if_missing_description() {
        let missing_description_input =
            HashMap::from([("ID", "ID123"), ("Number", "3"), ("Type", "String")]);

        let result = is_valid_format(missing_description_input);

        assert_eq!(result, false);
    }

    #[test]
    fn returns_false_if_extra_key_included() {
        let extra_key_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "3"),
            ("Type", "String"),
            ("Description", "This is a thing"),
            ("BadKey", "This shouldn't be here"),
        ]);

        let result = is_valid_format(extra_key_input);

        assert_eq!(result, false);
    }

    #[test]
    fn returns_true_if_type_is_Integer() {
        let integer_value_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "3"),
            ("Type", "Integer"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(integer_value_input);

        assert_eq!(result, true);
    }

    #[test]
    fn returns_true_if_type_is_Float() {
        let float_value_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "3"),
            ("Type", "Float"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(float_value_input);

        assert_eq!(result, true);
    }

    #[test]
    fn returns_true_if_type_is_Character() {
        let character_value_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "3"),
            ("Type", "Character"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(character_value_input);

        assert_eq!(result, true);
    }

    #[test]
    fn returns_true_if_type_is_String() {
        let string_value_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "3"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(string_value_input);

        assert_eq!(result, true);
    }

    #[test]
    fn returns_false_if_type_is_not_valid() {
        let string_value_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "3"),
            ("Type", "Number"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(string_value_input);

        assert_eq!(result, false);
    }

    #[test]
    fn returns_true_if_number_is_single_figure_positive_integer() {
        let valid_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "1"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(valid_input);

        assert_eq!(result, true);
    }

    #[test]
    fn returns_true_if_number_is_multiple_figure_positive_integer() {
        let valid_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "33"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(valid_input);

        assert_eq!(result, true);
    }

    #[test]
    fn returns_true_if_number_is_zero() {
        let valid_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "0"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(valid_input);

        assert_eq!(result, true);
    }

    #[test]
    fn returns_false_if_number_is_negative() {
        let valid_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "-3"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(valid_input);

        assert_eq!(result, false);
    }

    #[test]
    fn returns_false_if_number_has_fractional_part() {
        let valid_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "3.0"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(valid_input);

        assert_eq!(result, false);
    }

    #[test]
    fn returns_true_if_number_is_A() {
        let valid_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "A"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(valid_input);

        assert_eq!(result, true);
    }

    #[test]
    fn returns_true_if_number_is_R() {
        let valid_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "R"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(valid_input);

        assert_eq!(result, true);
    }

    #[test]
    fn returns_true_is_number_is_G() {
        let valid_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "G"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(valid_input);

        assert_eq!(result, true);
    }

    #[test]
    fn returns_true_if_number_is_dot() {
        let valid_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "."),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(valid_input);

        assert_eq!(result, true);
    }

    #[test]
    fn returns_false_if_number_is_not_special_character() {
        let valid_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "B"),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(valid_input);

        assert_eq!(result, false);
    }

    #[test]
    fn returns_false_if_number_is_empty() {
        let valid_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", ""),
            ("Type", "String"),
            ("Description", "This is a thing"),
        ]);

        let result = is_valid_format(valid_input);

        assert_eq!(result, false);
    }
}
