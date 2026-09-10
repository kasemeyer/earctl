//! Human-readable names for the raw gesture codes.
//!
//! Sources: ear-web's per-model gesture UIs (radiance-project/ear-web) and
//! live captures from Nothing Ear (3) hardware. The gesture_type codes are
//! shared across tap- and pinch-based models, so the names here stay
//! neutral ("double" rather than "double pinch").

/// device byte in a gesture slot; matches the battery device ids.
pub fn device_name(device: u8) -> Option<&'static str> {
    match device {
        0x02 => Some("left"),
        0x03 => Some("right"),
        0x04 => Some("case"),
        _ => None,
    }
}

pub fn device_code(name: &str) -> Option<u8> {
    match name {
        "left" => Some(0x02),
        "right" => Some(0x03),
        "case" => Some(0x04),
        _ => None,
    }
}

pub fn gesture_type_name(gesture_type: u8) -> Option<&'static str> {
    match gesture_type {
        2 => Some("double"),
        3 => Some("triple"),
        7 => Some("hold"),
        9 => Some("double_hold"),
        _ => None,
    }
}

pub fn gesture_type_code(name: &str) -> Option<u8> {
    match name {
        "double" => Some(2),
        "triple" => Some(3),
        "hold" => Some(7),
        "double_hold" => Some(9),
        _ => None,
    }
}

/// Actions 10/20/21/22 are all "noise control"; the suffix says which modes
/// the gesture cycles through (anc / transparency / off).
pub fn action_name(action: u8) -> Option<&'static str> {
    match action {
        1 => Some("none"),
        8 => Some("skip_back"),
        9 => Some("skip_forward"),
        10 => Some("noise_control_all"),
        11 => Some("voice_assistant"),
        18 => Some("volume_up"),
        19 => Some("volume_down"),
        20 => Some("noise_control_anc_off"),
        21 => Some("noise_control_transparency_off"),
        22 => Some("noise_control_anc_transparency"),
        _ => None,
    }
}

pub fn action_code(name: &str) -> Option<u8> {
    match name {
        "none" => Some(1),
        "skip_back" => Some(8),
        "skip_forward" => Some(9),
        "noise_control_all" => Some(10),
        "voice_assistant" => Some(11),
        "volume_up" => Some(18),
        "volume_down" => Some(19),
        "noise_control_anc_off" => Some(20),
        "noise_control_transparency_off" => Some(21),
        "noise_control_anc_transparency" => Some(22),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip() {
        for code in [1u8, 8, 9, 10, 11, 18, 19, 20, 21, 22] {
            let name = action_name(code).expect("known action");
            assert_eq!(action_code(name), Some(code));
        }
        for code in [2u8, 3, 7, 9] {
            let name = gesture_type_name(code).expect("known gesture type");
            assert_eq!(gesture_type_code(name), Some(code));
        }
        for code in [0x02u8, 0x03, 0x04] {
            let name = device_name(code).expect("known device");
            assert_eq!(device_code(name), Some(code));
        }
    }

    #[test]
    fn unknown_codes_are_none() {
        assert_eq!(action_name(99), None);
        assert_eq!(gesture_type_name(0), None);
        assert_eq!(device_name(0x05), None);
    }
}
