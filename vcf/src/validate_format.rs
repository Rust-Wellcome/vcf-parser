use std::collections::HashMap;
use std::collections::HashSet;

/// - [X] Check has all and only required keys.
/// - [ ] Check that values are of the required types.
fn is_valid_format(input: HashMap<&str, &str>) -> bool {
        let keys: HashSet<&str> = input.keys().copied().collect();
        let required_keys = HashSet::from(["ID", "Number", "Type", "Description"]);
        if !(required_keys == keys) {return false};
        if !([&"Integer", &"Float", &"Character", &"String"].map(|s| Some(s)).contains(&input.get("Type"))) {return false};
        return true;
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
        let missing_description_input = HashMap::from([
            ("ID", "ID123"),
            ("Number", "3"),
            ("Type", "String"),
        ]);

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
}
