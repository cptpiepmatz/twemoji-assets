mod tests {
    use twemoji_assets::*;
    use twemoji_assets::svg::codes::*;

    #[test]
    fn macro_test_emoji() {
        assert_eq!(svg_twemoji_asset!("🐦"), &U_1F426);
        assert_eq!(svg_twemoji_asset!("🇩🇪"), &U_1F1E9_1F1EA);
        assert_eq!(svg_twemoji_asset!("🐕‍🦺"), &U_1F415_200D_1F9BA);
        assert_eq!(svg_twemoji_asset!("🚴‍♀️"), &U_1F6B4_200D_2640_FE0F);
        assert_eq!(svg_twemoji_asset!("🧙🏾‍♂️"), &U_1F9D9_1F3FE_200D_2642_FE0F);
        assert_eq!(svg_twemoji_asset!("🧑🏻‍❤️‍💋‍🧑🏾"), &U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE);
    }

    #[test]
    fn macro_test_name() {
        assert_eq!(svg_twemoji_asset_from_name!("bird"), &U_1F426);
        assert_eq!(svg_twemoji_asset_from_name!("germany"), &U_1F1E9_1F1EA);
        assert_eq!(svg_twemoji_asset_from_name!("service_dog"), &U_1F415_200D_1F9BA);
        assert_eq!(svg_twemoji_asset_from_name!("woman_biking"), &U_1F6B4_200D_2640_FE0F);
    }

}
