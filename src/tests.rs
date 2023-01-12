use crate::directory::get_config_file_location;

#[test]

fn test_get_config_file() {
    let result = get_config_file_location();

    assert_eq!(result, "/Users/matt/.config/envctl/config.json");
}
