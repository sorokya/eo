use crate::data::EOInt;

mod action;
pub use action::Action;

mod family;
pub use family::Family;

/// flattens a packet [Family] and [Action] into a single [EOInt]
///
/// This is useful for identifying pairs of packet [Family]s, and [Action]s.
/// It is assumed there are a max of 255 actions per packet family.
///
/// # Example
/// ```
/// use eo::net::*;
///
/// assert_eq!(packet_id_hash(Family::Connection, Action::Request), 257)
/// ```
pub fn packet_id_hash(family: Family, action: Action) -> EOInt {
    ((family as EOInt) << 8) | action as EOInt
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn packet_id_hash() {
        assert_eq!(super::packet_id_hash(Family::Init, Action::Init), 65535);
        assert_eq!(
            super::packet_id_hash(Family::Connection, Action::Request),
            257
        );
        assert_eq!(super::packet_id_hash(Family::Welcome, Action::Reply), 1283);
    }
}
