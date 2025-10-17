use specify_test::character_model::calculate_saving_throw_bonus;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saving_throw_proficient() {
        // Strength 14 (+2 modifier), level 5 (+3 proficiency), proficient
        let bonus = calculate_saving_throw_bonus(14, 5, true);
        assert_eq!(bonus, 5); // +2 modifier + 3 proficiency = +5
    }

    #[test]
    fn test_saving_throw_not_proficient() {
        // Wisdom 16 (+3 modifier), level 5, not proficient
        let bonus = calculate_saving_throw_bonus(16, 5, false);
        assert_eq!(bonus, 3); // +3 modifier only
    }

    #[test]
    fn test_saving_throw_negative_modifier() {
        // Strength 8 (-1 modifier), level 1 (+2 proficiency), proficient
        let bonus = calculate_saving_throw_bonus(8, 1, true);
        assert_eq!(bonus, 1); // -1 modifier + 2 proficiency = +1
    }

    #[test]
    fn test_saving_throw_negative_modifier_not_proficient() {
        // Charisma 6 (-2 modifier), level 1, not proficient
        let bonus = calculate_saving_throw_bonus(6, 1, false);
        assert_eq!(bonus, -2); // -2 modifier only
    }

    #[test]
    fn test_proficiency_bonus_progression() {
        // Test proficiency bonus at different levels
        assert_eq!(calculate_saving_throw_bonus(10, 1, true), 2); // Level 1-4: +2
        assert_eq!(calculate_saving_throw_bonus(10, 5, true), 3); // Level 5-8: +3
        assert_eq!(calculate_saving_throw_bonus(10, 9, true), 4); // Level 9-12: +4
        assert_eq!(calculate_saving_throw_bonus(10, 13, true), 5); // Level 13-16: +5
        assert_eq!(calculate_saving_throw_bonus(10, 17, true), 6); // Level 17-20: +6
    }
}
