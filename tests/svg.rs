mod tests {
    use twemoji_assets::svg::codes::*;
    use twemoji_assets::svg::SvgTwemojiAsset;
    use twemoji_assets::*;

    #[test]
    fn macro_test_emoji() {
        assert_eq!(svg_twemoji_asset!("🐦"), &U_1F426);
        assert_eq!(svg_twemoji_asset!("🇩🇪"), &U_1F1E9_1F1EA);
        assert_eq!(svg_twemoji_asset!("🐕‍🦺"), &U_1F415_200D_1F9BA);
        assert_eq!(svg_twemoji_asset!("🚴‍♀️"), &U_1F6B4_200D_2640_FE0F);
        assert_eq!(svg_twemoji_asset!("🧙🏾‍♂️"), &U_1F9D9_1F3FE_200D_2642_FE0F);
        assert_eq!(
            svg_twemoji_asset!("🧑🏻‍❤️‍💋‍🧑🏾"),
            &U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE
        );
    }

    #[test]
    fn macro_test_name() {
        assert_eq!(svg_twemoji_asset_from_name!("bird"), &U_1F426);
        assert_eq!(svg_twemoji_asset_from_name!("germany"), &U_1F1E9_1F1EA);
        assert_eq!(
            svg_twemoji_asset_from_name!("service_dog"),
            &U_1F415_200D_1F9BA
        );
        assert_eq!(
            svg_twemoji_asset_from_name!("woman_biking"),
            &U_1F6B4_200D_2640_FE0F
        );
    }

    #[test]
    fn from_emoji() {
        assert!(SvgTwemojiAsset::from_emoji("").is_none());
        assert!(SvgTwemojiAsset::from_emoji("not an emoji").is_none());

        assert_eq!(SvgTwemojiAsset::from_emoji("🐦"), Some(&U_1F426));
        assert_eq!(SvgTwemojiAsset::from_emoji("🇩🇪"), Some(&U_1F1E9_1F1EA));
        assert_eq!(SvgTwemojiAsset::from_emoji("🐕‍🦺"), Some(&U_1F415_200D_1F9BA));
        assert_eq!(
            SvgTwemojiAsset::from_emoji("🚴‍♀️"),
            Some(&U_1F6B4_200D_2640_FE0F)
        );
        assert_eq!(
            SvgTwemojiAsset::from_emoji("🧙🏾‍♂️"),
            Some(&U_1F9D9_1F3FE_200D_2642_FE0F)
        );
        assert_eq!(
            SvgTwemojiAsset::from_emoji("🧑🏻‍❤️‍💋‍🧑🏾"),
            Some(&U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE)
        );
    }

    #[test]
    fn from_name() {
        assert!(SvgTwemojiAsset::from_name("").is_none());
        assert!(SvgTwemojiAsset::from_name("not an emoji").is_none());

        assert_eq!(SvgTwemojiAsset::from_name("bird"), Some(&U_1F426));
        assert_eq!(SvgTwemojiAsset::from_name("germany"), Some(&U_1F1E9_1F1EA));
        assert_eq!(
            SvgTwemojiAsset::from_name("service_dog"),
            Some(&U_1F415_200D_1F9BA)
        );
        assert_eq!(
            SvgTwemojiAsset::from_name("woman_biking"),
            Some(&U_1F6B4_200D_2640_FE0F)
        );
    }
}
