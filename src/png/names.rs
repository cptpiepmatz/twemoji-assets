// @generated

//! PNG Twemoji assets with human-readable names.
//!
//! This module provides a subset of the Twemoji assets, which are provided by Twitter in the form
//! of PNG, with human-readable names.
//! The names are sourced from [Emojibase](https://github.com/milesj/emojibase).
//! The full set of Twemoji assets can be found in the [`codes`](super::codes) module.
//!
//! # Naming Convention
//!
//! The names provided by Emojibase may contain characters that are not valid as Rust identifiers.
//! To handle this, the names are sanitized using the `sanitize_ident` function in `gen.rs`.
//! This function performs the following transformations:
//!
//! - Replaces the "-" character with "_".
//! - Removes any "+" characters.
//! - If the first character is not an alphabetic character, a "X_" prefix is added.
//!
//! Here is the implementation of the `sanitize_ident` function:
//!
//! ```
//! fn sanitize_ident(ident: &str) -> String {
//!     let as_ident = ident.to_uppercase().replace('-', "_").replace('+', "");
//!     match as_ident.chars().next() {
//!         Some('A'..='Z') => as_ident,
//!         _ => format!("X_{as_ident}"),
//!     }
//! }
//! ```

use super::{PngTwemojiAsset, png_name, png_match_name};
use super::codes::*;

png_name!(MAHJONG, "🀄", "mahjong red dragon", U_1F004, "1f004.png");
png_name!(BLACK_JOKER, "🃏", "joker", U_1F0CF, "1f0cf.png");
png_name!(A, "🅰", "A button (blood type)", U_1F170, "1f170.png");
png_name!(A_BLOOD, "🅰", "A button (blood type)", U_1F170, "1f170.png");
png_name!(B, "🅱", "B button (blood type)", U_1F171, "1f171.png");
png_name!(B_BLOOD, "🅱", "B button (blood type)", U_1F171, "1f171.png");
png_name!(O, "🅾", "O button (blood type)", U_1F17E, "1f17e.png");
png_name!(O_BLOOD, "🅾", "O button (blood type)", U_1F17E, "1f17e.png");
png_name!(PARKING, "🅿", "P button", U_1F17F, "1f17f.png");
png_name!(AB, "🆎", "AB button (blood type)", U_1F18E, "1f18e.png");
png_name!(AB_BLOOD, "🆎", "AB button (blood type)", U_1F18E, "1f18e.png");
png_name!(CL, "🆑", "CL button", U_1F191, "1f191.png");
png_name!(COOL, "🆒", "COOL button", U_1F192, "1f192.png");
png_name!(FREE, "🆓", "FREE button", U_1F193, "1f193.png");
png_name!(ID, "🆔", "ID button", U_1F194, "1f194.png");
png_name!(NEW, "🆕", "NEW button", U_1F195, "1f195.png");
png_name!(NG, "🆖", "NG button", U_1F196, "1f196.png");
png_name!(OK, "🆗", "OK button", U_1F197, "1f197.png");
png_name!(SOS, "🆘", "SOS button", U_1F198, "1f198.png");
png_name!(UP2, "🆙", "UP! button", U_1F199, "1f199.png");
png_name!(VS, "🆚", "VS button", U_1F19A, "1f19a.png");
png_name!(ASCENSION_ISLAND, "🇦🇨", "flag: Ascension Island", U_1F1E6_1F1E8, "1f1e6-1f1e8.png");
png_name!(FLAG_AC, "🇦🇨", "flag: Ascension Island", U_1F1E6_1F1E8, "1f1e6-1f1e8.png");
png_name!(ANDORRA, "🇦🇩", "flag: Andorra", U_1F1E6_1F1E9, "1f1e6-1f1e9.png");
png_name!(FLAG_AD, "🇦🇩", "flag: Andorra", U_1F1E6_1F1E9, "1f1e6-1f1e9.png");
png_name!(FLAG_AE, "🇦🇪", "flag: United Arab Emirates", U_1F1E6_1F1EA, "1f1e6-1f1ea.png");
png_name!(UNITED_ARAB_EMIRATES, "🇦🇪", "flag: United Arab Emirates", U_1F1E6_1F1EA, "1f1e6-1f1ea.png");
png_name!(AFGHANISTAN, "🇦🇫", "flag: Afghanistan", U_1F1E6_1F1EB, "1f1e6-1f1eb.png");
png_name!(FLAG_AF, "🇦🇫", "flag: Afghanistan", U_1F1E6_1F1EB, "1f1e6-1f1eb.png");
png_name!(ANTIGUA_BARBUDA, "🇦🇬", "flag: Antigua & Barbuda", U_1F1E6_1F1EC, "1f1e6-1f1ec.png");
png_name!(FLAG_AG, "🇦🇬", "flag: Antigua & Barbuda", U_1F1E6_1F1EC, "1f1e6-1f1ec.png");
png_name!(ANGUILLA, "🇦🇮", "flag: Anguilla", U_1F1E6_1F1EE, "1f1e6-1f1ee.png");
png_name!(FLAG_AI, "🇦🇮", "flag: Anguilla", U_1F1E6_1F1EE, "1f1e6-1f1ee.png");
png_name!(ALBANIA, "🇦🇱", "flag: Albania", U_1F1E6_1F1F1, "1f1e6-1f1f1.png");
png_name!(FLAG_AL, "🇦🇱", "flag: Albania", U_1F1E6_1F1F1, "1f1e6-1f1f1.png");
png_name!(ARMENIA, "🇦🇲", "flag: Armenia", U_1F1E6_1F1F2, "1f1e6-1f1f2.png");
png_name!(FLAG_AM, "🇦🇲", "flag: Armenia", U_1F1E6_1F1F2, "1f1e6-1f1f2.png");
png_name!(ANGOLA, "🇦🇴", "flag: Angola", U_1F1E6_1F1F4, "1f1e6-1f1f4.png");
png_name!(FLAG_AO, "🇦🇴", "flag: Angola", U_1F1E6_1F1F4, "1f1e6-1f1f4.png");
png_name!(ANTARCTICA, "🇦🇶", "flag: Antarctica", U_1F1E6_1F1F6, "1f1e6-1f1f6.png");
png_name!(FLAG_AQ, "🇦🇶", "flag: Antarctica", U_1F1E6_1F1F6, "1f1e6-1f1f6.png");
png_name!(ARGENTINA, "🇦🇷", "flag: Argentina", U_1F1E6_1F1F7, "1f1e6-1f1f7.png");
png_name!(FLAG_AR, "🇦🇷", "flag: Argentina", U_1F1E6_1F1F7, "1f1e6-1f1f7.png");
png_name!(AMERICAN_SAMOA, "🇦🇸", "flag: American Samoa", U_1F1E6_1F1F8, "1f1e6-1f1f8.png");
png_name!(FLAG_AS, "🇦🇸", "flag: American Samoa", U_1F1E6_1F1F8, "1f1e6-1f1f8.png");
png_name!(AUSTRIA, "🇦🇹", "flag: Austria", U_1F1E6_1F1F9, "1f1e6-1f1f9.png");
png_name!(FLAG_AT, "🇦🇹", "flag: Austria", U_1F1E6_1F1F9, "1f1e6-1f1f9.png");
png_name!(AUSTRALIA, "🇦🇺", "flag: Australia", U_1F1E6_1F1FA, "1f1e6-1f1fa.png");
png_name!(FLAG_AU, "🇦🇺", "flag: Australia", U_1F1E6_1F1FA, "1f1e6-1f1fa.png");
png_name!(ARUBA, "🇦🇼", "flag: Aruba", U_1F1E6_1F1FC, "1f1e6-1f1fc.png");
png_name!(FLAG_AW, "🇦🇼", "flag: Aruba", U_1F1E6_1F1FC, "1f1e6-1f1fc.png");
png_name!(ALAND_ISLANDS, "🇦🇽", "flag: Åland Islands", U_1F1E6_1F1FD, "1f1e6-1f1fd.png");
png_name!(FLAG_AX, "🇦🇽", "flag: Åland Islands", U_1F1E6_1F1FD, "1f1e6-1f1fd.png");
png_name!(AZERBAIJAN, "🇦🇿", "flag: Azerbaijan", U_1F1E6_1F1FF, "1f1e6-1f1ff.png");
png_name!(FLAG_AZ, "🇦🇿", "flag: Azerbaijan", U_1F1E6_1F1FF, "1f1e6-1f1ff.png");
png_name!(REGIONAL_INDICATOR_A, "🇦", "regional indicator A", U_1F1E6, "1f1e6.png");
png_name!(BOSNIA_HERZEGOVINA, "🇧🇦", "flag: Bosnia & Herzegovina", U_1F1E7_1F1E6, "1f1e7-1f1e6.png");
png_name!(FLAG_BA, "🇧🇦", "flag: Bosnia & Herzegovina", U_1F1E7_1F1E6, "1f1e7-1f1e6.png");
png_name!(BARBADOS, "🇧🇧", "flag: Barbados", U_1F1E7_1F1E7, "1f1e7-1f1e7.png");
png_name!(FLAG_BB, "🇧🇧", "flag: Barbados", U_1F1E7_1F1E7, "1f1e7-1f1e7.png");
png_name!(BANGLADESH, "🇧🇩", "flag: Bangladesh", U_1F1E7_1F1E9, "1f1e7-1f1e9.png");
png_name!(FLAG_BD, "🇧🇩", "flag: Bangladesh", U_1F1E7_1F1E9, "1f1e7-1f1e9.png");
png_name!(BELGIUM, "🇧🇪", "flag: Belgium", U_1F1E7_1F1EA, "1f1e7-1f1ea.png");
png_name!(FLAG_BE, "🇧🇪", "flag: Belgium", U_1F1E7_1F1EA, "1f1e7-1f1ea.png");
png_name!(BURKINA_FASO, "🇧🇫", "flag: Burkina Faso", U_1F1E7_1F1EB, "1f1e7-1f1eb.png");
png_name!(FLAG_BF, "🇧🇫", "flag: Burkina Faso", U_1F1E7_1F1EB, "1f1e7-1f1eb.png");
png_name!(BULGARIA, "🇧🇬", "flag: Bulgaria", U_1F1E7_1F1EC, "1f1e7-1f1ec.png");
png_name!(FLAG_BG, "🇧🇬", "flag: Bulgaria", U_1F1E7_1F1EC, "1f1e7-1f1ec.png");
png_name!(BAHRAIN, "🇧🇭", "flag: Bahrain", U_1F1E7_1F1ED, "1f1e7-1f1ed.png");
png_name!(FLAG_BH, "🇧🇭", "flag: Bahrain", U_1F1E7_1F1ED, "1f1e7-1f1ed.png");
png_name!(BURUNDI, "🇧🇮", "flag: Burundi", U_1F1E7_1F1EE, "1f1e7-1f1ee.png");
png_name!(FLAG_BI, "🇧🇮", "flag: Burundi", U_1F1E7_1F1EE, "1f1e7-1f1ee.png");
png_name!(BENIN, "🇧🇯", "flag: Benin", U_1F1E7_1F1EF, "1f1e7-1f1ef.png");
png_name!(FLAG_BJ, "🇧🇯", "flag: Benin", U_1F1E7_1F1EF, "1f1e7-1f1ef.png");
png_name!(FLAG_BL, "🇧🇱", "flag: St. Barthélemy", U_1F1E7_1F1F1, "1f1e7-1f1f1.png");
png_name!(ST_BARTHELEMY, "🇧🇱", "flag: St. Barthélemy", U_1F1E7_1F1F1, "1f1e7-1f1f1.png");
png_name!(BERMUDA, "🇧🇲", "flag: Bermuda", U_1F1E7_1F1F2, "1f1e7-1f1f2.png");
png_name!(FLAG_BM, "🇧🇲", "flag: Bermuda", U_1F1E7_1F1F2, "1f1e7-1f1f2.png");
png_name!(BRUNEI, "🇧🇳", "flag: Brunei", U_1F1E7_1F1F3, "1f1e7-1f1f3.png");
png_name!(FLAG_BN, "🇧🇳", "flag: Brunei", U_1F1E7_1F1F3, "1f1e7-1f1f3.png");
png_name!(BOLIVIA, "🇧🇴", "flag: Bolivia", U_1F1E7_1F1F4, "1f1e7-1f1f4.png");
png_name!(FLAG_BO, "🇧🇴", "flag: Bolivia", U_1F1E7_1F1F4, "1f1e7-1f1f4.png");
png_name!(CARIBBEAN_NETHERLANDS, "🇧🇶", "flag: Caribbean Netherlands", U_1F1E7_1F1F6, "1f1e7-1f1f6.png");
png_name!(FLAG_BQ, "🇧🇶", "flag: Caribbean Netherlands", U_1F1E7_1F1F6, "1f1e7-1f1f6.png");
png_name!(BRAZIL, "🇧🇷", "flag: Brazil", U_1F1E7_1F1F7, "1f1e7-1f1f7.png");
png_name!(FLAG_BR, "🇧🇷", "flag: Brazil", U_1F1E7_1F1F7, "1f1e7-1f1f7.png");
png_name!(BAHAMAS, "🇧🇸", "flag: Bahamas", U_1F1E7_1F1F8, "1f1e7-1f1f8.png");
png_name!(FLAG_BS, "🇧🇸", "flag: Bahamas", U_1F1E7_1F1F8, "1f1e7-1f1f8.png");
png_name!(BHUTAN, "🇧🇹", "flag: Bhutan", U_1F1E7_1F1F9, "1f1e7-1f1f9.png");
png_name!(FLAG_BT, "🇧🇹", "flag: Bhutan", U_1F1E7_1F1F9, "1f1e7-1f1f9.png");
png_name!(BOUVET_ISLAND, "🇧🇻", "flag: Bouvet Island", U_1F1E7_1F1FB, "1f1e7-1f1fb.png");
png_name!(FLAG_BV, "🇧🇻", "flag: Bouvet Island", U_1F1E7_1F1FB, "1f1e7-1f1fb.png");
png_name!(BOTSWANA, "🇧🇼", "flag: Botswana", U_1F1E7_1F1FC, "1f1e7-1f1fc.png");
png_name!(FLAG_BW, "🇧🇼", "flag: Botswana", U_1F1E7_1F1FC, "1f1e7-1f1fc.png");
png_name!(BELARUS, "🇧🇾", "flag: Belarus", U_1F1E7_1F1FE, "1f1e7-1f1fe.png");
png_name!(FLAG_BY, "🇧🇾", "flag: Belarus", U_1F1E7_1F1FE, "1f1e7-1f1fe.png");
png_name!(BELIZE, "🇧🇿", "flag: Belize", U_1F1E7_1F1FF, "1f1e7-1f1ff.png");
png_name!(FLAG_BZ, "🇧🇿", "flag: Belize", U_1F1E7_1F1FF, "1f1e7-1f1ff.png");
png_name!(REGIONAL_INDICATOR_B, "🇧", "regional indicator B", U_1F1E7, "1f1e7.png");
png_name!(CANADA, "🇨🇦", "flag: Canada", U_1F1E8_1F1E6, "1f1e8-1f1e6.png");
png_name!(FLAG_CA, "🇨🇦", "flag: Canada", U_1F1E8_1F1E6, "1f1e8-1f1e6.png");
png_name!(COCOS_ISLANDS, "🇨🇨", "flag: Cocos (Keeling) Islands", U_1F1E8_1F1E8, "1f1e8-1f1e8.png");
png_name!(FLAG_CC, "🇨🇨", "flag: Cocos (Keeling) Islands", U_1F1E8_1F1E8, "1f1e8-1f1e8.png");
png_name!(CONGO_KINSHASA, "🇨🇩", "flag: Congo - Kinshasa", U_1F1E8_1F1E9, "1f1e8-1f1e9.png");
png_name!(FLAG_CD, "🇨🇩", "flag: Congo - Kinshasa", U_1F1E8_1F1E9, "1f1e8-1f1e9.png");
png_name!(CENTRAL_AFRICAN_REPUBLIC, "🇨🇫", "flag: Central African Republic", U_1F1E8_1F1EB, "1f1e8-1f1eb.png");
png_name!(FLAG_CF, "🇨🇫", "flag: Central African Republic", U_1F1E8_1F1EB, "1f1e8-1f1eb.png");
png_name!(CONGO_BRAZZAVILLE, "🇨🇬", "flag: Congo - Brazzaville", U_1F1E8_1F1EC, "1f1e8-1f1ec.png");
png_name!(FLAG_CG, "🇨🇬", "flag: Congo - Brazzaville", U_1F1E8_1F1EC, "1f1e8-1f1ec.png");
png_name!(FLAG_CH, "🇨🇭", "flag: Switzerland", U_1F1E8_1F1ED, "1f1e8-1f1ed.png");
png_name!(SWITZERLAND, "🇨🇭", "flag: Switzerland", U_1F1E8_1F1ED, "1f1e8-1f1ed.png");
png_name!(COTE_DIVOIRE, "🇨🇮", "flag: Côte d’Ivoire", U_1F1E8_1F1EE, "1f1e8-1f1ee.png");
png_name!(FLAG_CI, "🇨🇮", "flag: Côte d’Ivoire", U_1F1E8_1F1EE, "1f1e8-1f1ee.png");
png_name!(COOK_ISLANDS, "🇨🇰", "flag: Cook Islands", U_1F1E8_1F1F0, "1f1e8-1f1f0.png");
png_name!(FLAG_CK, "🇨🇰", "flag: Cook Islands", U_1F1E8_1F1F0, "1f1e8-1f1f0.png");
png_name!(CHILE, "🇨🇱", "flag: Chile", U_1F1E8_1F1F1, "1f1e8-1f1f1.png");
png_name!(FLAG_CL, "🇨🇱", "flag: Chile", U_1F1E8_1F1F1, "1f1e8-1f1f1.png");
png_name!(CAMEROON, "🇨🇲", "flag: Cameroon", U_1F1E8_1F1F2, "1f1e8-1f1f2.png");
png_name!(FLAG_CM, "🇨🇲", "flag: Cameroon", U_1F1E8_1F1F2, "1f1e8-1f1f2.png");
png_name!(CHINA, "🇨🇳", "flag: China", U_1F1E8_1F1F3, "1f1e8-1f1f3.png");
png_name!(FLAG_CN, "🇨🇳", "flag: China", U_1F1E8_1F1F3, "1f1e8-1f1f3.png");
png_name!(COLOMBIA, "🇨🇴", "flag: Colombia", U_1F1E8_1F1F4, "1f1e8-1f1f4.png");
png_name!(FLAG_CO, "🇨🇴", "flag: Colombia", U_1F1E8_1F1F4, "1f1e8-1f1f4.png");
png_name!(CLIPPERTON_ISLAND, "🇨🇵", "flag: Clipperton Island", U_1F1E8_1F1F5, "1f1e8-1f1f5.png");
png_name!(FLAG_CP, "🇨🇵", "flag: Clipperton Island", U_1F1E8_1F1F5, "1f1e8-1f1f5.png");
png_name!(COSTA_RICA, "🇨🇷", "flag: Costa Rica", U_1F1E8_1F1F7, "1f1e8-1f1f7.png");
png_name!(FLAG_CR, "🇨🇷", "flag: Costa Rica", U_1F1E8_1F1F7, "1f1e8-1f1f7.png");
png_name!(CUBA, "🇨🇺", "flag: Cuba", U_1F1E8_1F1FA, "1f1e8-1f1fa.png");
png_name!(FLAG_CU, "🇨🇺", "flag: Cuba", U_1F1E8_1F1FA, "1f1e8-1f1fa.png");
png_name!(CAPE_VERDE, "🇨🇻", "flag: Cape Verde", U_1F1E8_1F1FB, "1f1e8-1f1fb.png");
png_name!(FLAG_CV, "🇨🇻", "flag: Cape Verde", U_1F1E8_1F1FB, "1f1e8-1f1fb.png");
png_name!(CURACAO, "🇨🇼", "flag: Curaçao", U_1F1E8_1F1FC, "1f1e8-1f1fc.png");
png_name!(FLAG_CW, "🇨🇼", "flag: Curaçao", U_1F1E8_1F1FC, "1f1e8-1f1fc.png");
png_name!(CHRISTMAS_ISLAND, "🇨🇽", "flag: Christmas Island", U_1F1E8_1F1FD, "1f1e8-1f1fd.png");
png_name!(FLAG_CX, "🇨🇽", "flag: Christmas Island", U_1F1E8_1F1FD, "1f1e8-1f1fd.png");
png_name!(CYPRUS, "🇨🇾", "flag: Cyprus", U_1F1E8_1F1FE, "1f1e8-1f1fe.png");
png_name!(FLAG_CY, "🇨🇾", "flag: Cyprus", U_1F1E8_1F1FE, "1f1e8-1f1fe.png");
png_name!(CZECH_REPUBLIC, "🇨🇿", "flag: Czechia", U_1F1E8_1F1FF, "1f1e8-1f1ff.png");
png_name!(CZECHIA, "🇨🇿", "flag: Czechia", U_1F1E8_1F1FF, "1f1e8-1f1ff.png");
png_name!(FLAG_CZ, "🇨🇿", "flag: Czechia", U_1F1E8_1F1FF, "1f1e8-1f1ff.png");
png_name!(REGIONAL_INDICATOR_C, "🇨", "regional indicator C", U_1F1E8, "1f1e8.png");
png_name!(FLAG_DE, "🇩🇪", "flag: Germany", U_1F1E9_1F1EA, "1f1e9-1f1ea.png");
png_name!(GERMANY, "🇩🇪", "flag: Germany", U_1F1E9_1F1EA, "1f1e9-1f1ea.png");
png_name!(DIEGO_GARCIA, "🇩🇬", "flag: Diego Garcia", U_1F1E9_1F1EC, "1f1e9-1f1ec.png");
png_name!(FLAG_DG, "🇩🇬", "flag: Diego Garcia", U_1F1E9_1F1EC, "1f1e9-1f1ec.png");
png_name!(DJIBOUTI, "🇩🇯", "flag: Djibouti", U_1F1E9_1F1EF, "1f1e9-1f1ef.png");
png_name!(FLAG_DJ, "🇩🇯", "flag: Djibouti", U_1F1E9_1F1EF, "1f1e9-1f1ef.png");
png_name!(DENMARK, "🇩🇰", "flag: Denmark", U_1F1E9_1F1F0, "1f1e9-1f1f0.png");
png_name!(FLAG_DK, "🇩🇰", "flag: Denmark", U_1F1E9_1F1F0, "1f1e9-1f1f0.png");
png_name!(DOMINICA, "🇩🇲", "flag: Dominica", U_1F1E9_1F1F2, "1f1e9-1f1f2.png");
png_name!(FLAG_DM, "🇩🇲", "flag: Dominica", U_1F1E9_1F1F2, "1f1e9-1f1f2.png");
png_name!(DOMINICAN_REPUBLIC, "🇩🇴", "flag: Dominican Republic", U_1F1E9_1F1F4, "1f1e9-1f1f4.png");
png_name!(FLAG_DO, "🇩🇴", "flag: Dominican Republic", U_1F1E9_1F1F4, "1f1e9-1f1f4.png");
png_name!(ALGERIA, "🇩🇿", "flag: Algeria", U_1F1E9_1F1FF, "1f1e9-1f1ff.png");
png_name!(FLAG_DZ, "🇩🇿", "flag: Algeria", U_1F1E9_1F1FF, "1f1e9-1f1ff.png");
png_name!(REGIONAL_INDICATOR_D, "🇩", "regional indicator D", U_1F1E9, "1f1e9.png");
png_name!(CEUTA_MELILLA, "🇪🇦", "flag: Ceuta & Melilla", U_1F1EA_1F1E6, "1f1ea-1f1e6.png");
png_name!(FLAG_EA, "🇪🇦", "flag: Ceuta & Melilla", U_1F1EA_1F1E6, "1f1ea-1f1e6.png");
png_name!(ECUADOR, "🇪🇨", "flag: Ecuador", U_1F1EA_1F1E8, "1f1ea-1f1e8.png");
png_name!(FLAG_EC, "🇪🇨", "flag: Ecuador", U_1F1EA_1F1E8, "1f1ea-1f1e8.png");
png_name!(ESTONIA, "🇪🇪", "flag: Estonia", U_1F1EA_1F1EA, "1f1ea-1f1ea.png");
png_name!(FLAG_EE, "🇪🇪", "flag: Estonia", U_1F1EA_1F1EA, "1f1ea-1f1ea.png");
png_name!(EGYPT, "🇪🇬", "flag: Egypt", U_1F1EA_1F1EC, "1f1ea-1f1ec.png");
png_name!(FLAG_EG, "🇪🇬", "flag: Egypt", U_1F1EA_1F1EC, "1f1ea-1f1ec.png");
png_name!(FLAG_EH, "🇪🇭", "flag: Western Sahara", U_1F1EA_1F1ED, "1f1ea-1f1ed.png");
png_name!(WESTERN_SAHARA, "🇪🇭", "flag: Western Sahara", U_1F1EA_1F1ED, "1f1ea-1f1ed.png");
png_name!(ERITREA, "🇪🇷", "flag: Eritrea", U_1F1EA_1F1F7, "1f1ea-1f1f7.png");
png_name!(FLAG_ER, "🇪🇷", "flag: Eritrea", U_1F1EA_1F1F7, "1f1ea-1f1f7.png");
png_name!(FLAG_ES, "🇪🇸", "flag: Spain", U_1F1EA_1F1F8, "1f1ea-1f1f8.png");
png_name!(SPAIN, "🇪🇸", "flag: Spain", U_1F1EA_1F1F8, "1f1ea-1f1f8.png");
png_name!(ETHIOPIA, "🇪🇹", "flag: Ethiopia", U_1F1EA_1F1F9, "1f1ea-1f1f9.png");
png_name!(FLAG_ET, "🇪🇹", "flag: Ethiopia", U_1F1EA_1F1F9, "1f1ea-1f1f9.png");
png_name!(EUROPEAN_UNION, "🇪🇺", "flag: European Union", U_1F1EA_1F1FA, "1f1ea-1f1fa.png");
png_name!(FLAG_EU, "🇪🇺", "flag: European Union", U_1F1EA_1F1FA, "1f1ea-1f1fa.png");
png_name!(REGIONAL_INDICATOR_E, "🇪", "regional indicator E", U_1F1EA, "1f1ea.png");
png_name!(FINLAND, "🇫🇮", "flag: Finland", U_1F1EB_1F1EE, "1f1eb-1f1ee.png");
png_name!(FLAG_FI, "🇫🇮", "flag: Finland", U_1F1EB_1F1EE, "1f1eb-1f1ee.png");
png_name!(FIJI, "🇫🇯", "flag: Fiji", U_1F1EB_1F1EF, "1f1eb-1f1ef.png");
png_name!(FLAG_FJ, "🇫🇯", "flag: Fiji", U_1F1EB_1F1EF, "1f1eb-1f1ef.png");
png_name!(FALKLAND_ISLANDS, "🇫🇰", "flag: Falkland Islands", U_1F1EB_1F1F0, "1f1eb-1f1f0.png");
png_name!(FLAG_FK, "🇫🇰", "flag: Falkland Islands", U_1F1EB_1F1F0, "1f1eb-1f1f0.png");
png_name!(FLAG_FM, "🇫🇲", "flag: Micronesia", U_1F1EB_1F1F2, "1f1eb-1f1f2.png");
png_name!(MICRONESIA, "🇫🇲", "flag: Micronesia", U_1F1EB_1F1F2, "1f1eb-1f1f2.png");
png_name!(FAROE_ISLANDS, "🇫🇴", "flag: Faroe Islands", U_1F1EB_1F1F4, "1f1eb-1f1f4.png");
png_name!(FLAG_FO, "🇫🇴", "flag: Faroe Islands", U_1F1EB_1F1F4, "1f1eb-1f1f4.png");
png_name!(FLAG_FR, "🇫🇷", "flag: France", U_1F1EB_1F1F7, "1f1eb-1f1f7.png");
png_name!(FRANCE, "🇫🇷", "flag: France", U_1F1EB_1F1F7, "1f1eb-1f1f7.png");
png_name!(REGIONAL_INDICATOR_F, "🇫", "regional indicator F", U_1F1EB, "1f1eb.png");
png_name!(FLAG_GA, "🇬🇦", "flag: Gabon", U_1F1EC_1F1E6, "1f1ec-1f1e6.png");
png_name!(GABON, "🇬🇦", "flag: Gabon", U_1F1EC_1F1E6, "1f1ec-1f1e6.png");
png_name!(FLAG_GB, "🇬🇧", "flag: United Kingdom", U_1F1EC_1F1E7, "1f1ec-1f1e7.png");
png_name!(UK, "🇬🇧", "flag: United Kingdom", U_1F1EC_1F1E7, "1f1ec-1f1e7.png");
png_name!(UNITED_KINGDOM, "🇬🇧", "flag: United Kingdom", U_1F1EC_1F1E7, "1f1ec-1f1e7.png");
png_name!(FLAG_GD, "🇬🇩", "flag: Grenada", U_1F1EC_1F1E9, "1f1ec-1f1e9.png");
png_name!(GRENADA, "🇬🇩", "flag: Grenada", U_1F1EC_1F1E9, "1f1ec-1f1e9.png");
png_name!(FLAG_GE, "🇬🇪", "flag: Georgia", U_1F1EC_1F1EA, "1f1ec-1f1ea.png");
png_name!(GEORGIA, "🇬🇪", "flag: Georgia", U_1F1EC_1F1EA, "1f1ec-1f1ea.png");
png_name!(FLAG_GF, "🇬🇫", "flag: French Guiana", U_1F1EC_1F1EB, "1f1ec-1f1eb.png");
png_name!(FRENCH_GUIANA, "🇬🇫", "flag: French Guiana", U_1F1EC_1F1EB, "1f1ec-1f1eb.png");
png_name!(FLAG_GG, "🇬🇬", "flag: Guernsey", U_1F1EC_1F1EC, "1f1ec-1f1ec.png");
png_name!(GUERNSEY, "🇬🇬", "flag: Guernsey", U_1F1EC_1F1EC, "1f1ec-1f1ec.png");
png_name!(FLAG_GH, "🇬🇭", "flag: Ghana", U_1F1EC_1F1ED, "1f1ec-1f1ed.png");
png_name!(GHANA, "🇬🇭", "flag: Ghana", U_1F1EC_1F1ED, "1f1ec-1f1ed.png");
png_name!(FLAG_GI, "🇬🇮", "flag: Gibraltar", U_1F1EC_1F1EE, "1f1ec-1f1ee.png");
png_name!(GIBRALTAR, "🇬🇮", "flag: Gibraltar", U_1F1EC_1F1EE, "1f1ec-1f1ee.png");
png_name!(FLAG_GL, "🇬🇱", "flag: Greenland", U_1F1EC_1F1F1, "1f1ec-1f1f1.png");
png_name!(GREENLAND, "🇬🇱", "flag: Greenland", U_1F1EC_1F1F1, "1f1ec-1f1f1.png");
png_name!(FLAG_GM, "🇬🇲", "flag: Gambia", U_1F1EC_1F1F2, "1f1ec-1f1f2.png");
png_name!(GAMBIA, "🇬🇲", "flag: Gambia", U_1F1EC_1F1F2, "1f1ec-1f1f2.png");
png_name!(FLAG_GN, "🇬🇳", "flag: Guinea", U_1F1EC_1F1F3, "1f1ec-1f1f3.png");
png_name!(GUINEA, "🇬🇳", "flag: Guinea", U_1F1EC_1F1F3, "1f1ec-1f1f3.png");
png_name!(FLAG_GP, "🇬🇵", "flag: Guadeloupe", U_1F1EC_1F1F5, "1f1ec-1f1f5.png");
png_name!(GUADELOUPE, "🇬🇵", "flag: Guadeloupe", U_1F1EC_1F1F5, "1f1ec-1f1f5.png");
png_name!(EQUATORIAL_GUINEA, "🇬🇶", "flag: Equatorial Guinea", U_1F1EC_1F1F6, "1f1ec-1f1f6.png");
png_name!(FLAG_GQ, "🇬🇶", "flag: Equatorial Guinea", U_1F1EC_1F1F6, "1f1ec-1f1f6.png");
png_name!(FLAG_GR, "🇬🇷", "flag: Greece", U_1F1EC_1F1F7, "1f1ec-1f1f7.png");
png_name!(GREECE, "🇬🇷", "flag: Greece", U_1F1EC_1F1F7, "1f1ec-1f1f7.png");
png_name!(FLAG_GS, "🇬🇸", "flag: South Georgia & South Sandwich Islands", U_1F1EC_1F1F8, "1f1ec-1f1f8.png");
png_name!(SOUTH_GEORGIA_SOUTH_SANDWICH_ISLANDS, "🇬🇸", "flag: South Georgia & South Sandwich Islands", U_1F1EC_1F1F8, "1f1ec-1f1f8.png");
png_name!(FLAG_GT, "🇬🇹", "flag: Guatemala", U_1F1EC_1F1F9, "1f1ec-1f1f9.png");
png_name!(GUATEMALA, "🇬🇹", "flag: Guatemala", U_1F1EC_1F1F9, "1f1ec-1f1f9.png");
png_name!(FLAG_GU, "🇬🇺", "flag: Guam", U_1F1EC_1F1FA, "1f1ec-1f1fa.png");
png_name!(GUAM, "🇬🇺", "flag: Guam", U_1F1EC_1F1FA, "1f1ec-1f1fa.png");
png_name!(FLAG_GW, "🇬🇼", "flag: Guinea-Bissau", U_1F1EC_1F1FC, "1f1ec-1f1fc.png");
png_name!(GUINEA_BISSAU, "🇬🇼", "flag: Guinea-Bissau", U_1F1EC_1F1FC, "1f1ec-1f1fc.png");
png_name!(FLAG_GY, "🇬🇾", "flag: Guyana", U_1F1EC_1F1FE, "1f1ec-1f1fe.png");
png_name!(GUYANA, "🇬🇾", "flag: Guyana", U_1F1EC_1F1FE, "1f1ec-1f1fe.png");
png_name!(REGIONAL_INDICATOR_G, "🇬", "regional indicator G", U_1F1EC, "1f1ec.png");
png_name!(FLAG_HK, "🇭🇰", "flag: Hong Kong SAR China", U_1F1ED_1F1F0, "1f1ed-1f1f0.png");
png_name!(HONG_KONG, "🇭🇰", "flag: Hong Kong SAR China", U_1F1ED_1F1F0, "1f1ed-1f1f0.png");
png_name!(FLAG_HM, "🇭🇲", "flag: Heard & McDonald Islands", U_1F1ED_1F1F2, "1f1ed-1f1f2.png");
png_name!(HEARD_MCDONALD_ISLANDS, "🇭🇲", "flag: Heard & McDonald Islands", U_1F1ED_1F1F2, "1f1ed-1f1f2.png");
png_name!(FLAG_HN, "🇭🇳", "flag: Honduras", U_1F1ED_1F1F3, "1f1ed-1f1f3.png");
png_name!(HONDURAS, "🇭🇳", "flag: Honduras", U_1F1ED_1F1F3, "1f1ed-1f1f3.png");
png_name!(CROATIA, "🇭🇷", "flag: Croatia", U_1F1ED_1F1F7, "1f1ed-1f1f7.png");
png_name!(FLAG_HR, "🇭🇷", "flag: Croatia", U_1F1ED_1F1F7, "1f1ed-1f1f7.png");
png_name!(FLAG_HT, "🇭🇹", "flag: Haiti", U_1F1ED_1F1F9, "1f1ed-1f1f9.png");
png_name!(HAITI, "🇭🇹", "flag: Haiti", U_1F1ED_1F1F9, "1f1ed-1f1f9.png");
png_name!(FLAG_HU, "🇭🇺", "flag: Hungary", U_1F1ED_1F1FA, "1f1ed-1f1fa.png");
png_name!(HUNGARY, "🇭🇺", "flag: Hungary", U_1F1ED_1F1FA, "1f1ed-1f1fa.png");
png_name!(REGIONAL_INDICATOR_H, "🇭", "regional indicator H", U_1F1ED, "1f1ed.png");
png_name!(CANARY_ISLANDS, "🇮🇨", "flag: Canary Islands", U_1F1EE_1F1E8, "1f1ee-1f1e8.png");
png_name!(FLAG_IC, "🇮🇨", "flag: Canary Islands", U_1F1EE_1F1E8, "1f1ee-1f1e8.png");
png_name!(FLAG_ID, "🇮🇩", "flag: Indonesia", U_1F1EE_1F1E9, "1f1ee-1f1e9.png");
png_name!(INDONESIA, "🇮🇩", "flag: Indonesia", U_1F1EE_1F1E9, "1f1ee-1f1e9.png");
png_name!(FLAG_IE, "🇮🇪", "flag: Ireland", U_1F1EE_1F1EA, "1f1ee-1f1ea.png");
png_name!(IRELAND, "🇮🇪", "flag: Ireland", U_1F1EE_1F1EA, "1f1ee-1f1ea.png");
png_name!(FLAG_IL, "🇮🇱", "flag: Israel", U_1F1EE_1F1F1, "1f1ee-1f1f1.png");
png_name!(ISRAEL, "🇮🇱", "flag: Israel", U_1F1EE_1F1F1, "1f1ee-1f1f1.png");
png_name!(FLAG_IM, "🇮🇲", "flag: Isle of Man", U_1F1EE_1F1F2, "1f1ee-1f1f2.png");
png_name!(ISLE_OF_MAN, "🇮🇲", "flag: Isle of Man", U_1F1EE_1F1F2, "1f1ee-1f1f2.png");
png_name!(FLAG_IN, "🇮🇳", "flag: India", U_1F1EE_1F1F3, "1f1ee-1f1f3.png");
png_name!(INDIA, "🇮🇳", "flag: India", U_1F1EE_1F1F3, "1f1ee-1f1f3.png");
png_name!(BRITISH_INDIAN_OCEAN_TERRITORY, "🇮🇴", "flag: British Indian Ocean Territory", U_1F1EE_1F1F4, "1f1ee-1f1f4.png");
png_name!(FLAG_IO, "🇮🇴", "flag: British Indian Ocean Territory", U_1F1EE_1F1F4, "1f1ee-1f1f4.png");
png_name!(FLAG_IQ, "🇮🇶", "flag: Iraq", U_1F1EE_1F1F6, "1f1ee-1f1f6.png");
png_name!(IRAQ, "🇮🇶", "flag: Iraq", U_1F1EE_1F1F6, "1f1ee-1f1f6.png");
png_name!(FLAG_IR, "🇮🇷", "flag: Iran", U_1F1EE_1F1F7, "1f1ee-1f1f7.png");
png_name!(IRAN, "🇮🇷", "flag: Iran", U_1F1EE_1F1F7, "1f1ee-1f1f7.png");
png_name!(FLAG_IS, "🇮🇸", "flag: Iceland", U_1F1EE_1F1F8, "1f1ee-1f1f8.png");
png_name!(ICELAND, "🇮🇸", "flag: Iceland", U_1F1EE_1F1F8, "1f1ee-1f1f8.png");
png_name!(FLAG_IT, "🇮🇹", "flag: Italy", U_1F1EE_1F1F9, "1f1ee-1f1f9.png");
png_name!(ITALY, "🇮🇹", "flag: Italy", U_1F1EE_1F1F9, "1f1ee-1f1f9.png");
png_name!(REGIONAL_INDICATOR_I, "🇮", "regional indicator I", U_1F1EE, "1f1ee.png");
png_name!(FLAG_JE, "🇯🇪", "flag: Jersey", U_1F1EF_1F1EA, "1f1ef-1f1ea.png");
png_name!(JERSEY, "🇯🇪", "flag: Jersey", U_1F1EF_1F1EA, "1f1ef-1f1ea.png");
png_name!(FLAG_JM, "🇯🇲", "flag: Jamaica", U_1F1EF_1F1F2, "1f1ef-1f1f2.png");
png_name!(JAMAICA, "🇯🇲", "flag: Jamaica", U_1F1EF_1F1F2, "1f1ef-1f1f2.png");
png_name!(FLAG_JO, "🇯🇴", "flag: Jordan", U_1F1EF_1F1F4, "1f1ef-1f1f4.png");
png_name!(JORDAN, "🇯🇴", "flag: Jordan", U_1F1EF_1F1F4, "1f1ef-1f1f4.png");
png_name!(FLAG_JP, "🇯🇵", "flag: Japan", U_1F1EF_1F1F5, "1f1ef-1f1f5.png");
png_name!(JAPAN, "🇯🇵", "flag: Japan", U_1F1EF_1F1F5, "1f1ef-1f1f5.png");
png_name!(REGIONAL_INDICATOR_J, "🇯", "regional indicator J", U_1F1EF, "1f1ef.png");
png_name!(FLAG_KE, "🇰🇪", "flag: Kenya", U_1F1F0_1F1EA, "1f1f0-1f1ea.png");
png_name!(KENYA, "🇰🇪", "flag: Kenya", U_1F1F0_1F1EA, "1f1f0-1f1ea.png");
png_name!(FLAG_KG, "🇰🇬", "flag: Kyrgyzstan", U_1F1F0_1F1EC, "1f1f0-1f1ec.png");
png_name!(KYRGYZSTAN, "🇰🇬", "flag: Kyrgyzstan", U_1F1F0_1F1EC, "1f1f0-1f1ec.png");
png_name!(CAMBODIA, "🇰🇭", "flag: Cambodia", U_1F1F0_1F1ED, "1f1f0-1f1ed.png");
png_name!(FLAG_KH, "🇰🇭", "flag: Cambodia", U_1F1F0_1F1ED, "1f1f0-1f1ed.png");
png_name!(FLAG_KI, "🇰🇮", "flag: Kiribati", U_1F1F0_1F1EE, "1f1f0-1f1ee.png");
png_name!(KIRIBATI, "🇰🇮", "flag: Kiribati", U_1F1F0_1F1EE, "1f1f0-1f1ee.png");
png_name!(COMOROS, "🇰🇲", "flag: Comoros", U_1F1F0_1F1F2, "1f1f0-1f1f2.png");
png_name!(FLAG_KM, "🇰🇲", "flag: Comoros", U_1F1F0_1F1F2, "1f1f0-1f1f2.png");
png_name!(FLAG_KN, "🇰🇳", "flag: St. Kitts & Nevis", U_1F1F0_1F1F3, "1f1f0-1f1f3.png");
png_name!(ST_KITTS_NEVIS, "🇰🇳", "flag: St. Kitts & Nevis", U_1F1F0_1F1F3, "1f1f0-1f1f3.png");
png_name!(FLAG_KP, "🇰🇵", "flag: North Korea", U_1F1F0_1F1F5, "1f1f0-1f1f5.png");
png_name!(NORTH_KOREA, "🇰🇵", "flag: North Korea", U_1F1F0_1F1F5, "1f1f0-1f1f5.png");
png_name!(FLAG_KR, "🇰🇷", "flag: South Korea", U_1F1F0_1F1F7, "1f1f0-1f1f7.png");
png_name!(SOUTH_KOREA, "🇰🇷", "flag: South Korea", U_1F1F0_1F1F7, "1f1f0-1f1f7.png");
png_name!(FLAG_KW, "🇰🇼", "flag: Kuwait", U_1F1F0_1F1FC, "1f1f0-1f1fc.png");
png_name!(KUWAIT, "🇰🇼", "flag: Kuwait", U_1F1F0_1F1FC, "1f1f0-1f1fc.png");
png_name!(CAYMAN_ISLANDS, "🇰🇾", "flag: Cayman Islands", U_1F1F0_1F1FE, "1f1f0-1f1fe.png");
png_name!(FLAG_KY, "🇰🇾", "flag: Cayman Islands", U_1F1F0_1F1FE, "1f1f0-1f1fe.png");
png_name!(FLAG_KZ, "🇰🇿", "flag: Kazakhstan", U_1F1F0_1F1FF, "1f1f0-1f1ff.png");
png_name!(KAZAKHSTAN, "🇰🇿", "flag: Kazakhstan", U_1F1F0_1F1FF, "1f1f0-1f1ff.png");
png_name!(REGIONAL_INDICATOR_K, "🇰", "regional indicator K", U_1F1F0, "1f1f0.png");
png_name!(FLAG_LA, "🇱🇦", "flag: Laos", U_1F1F1_1F1E6, "1f1f1-1f1e6.png");
png_name!(LAOS, "🇱🇦", "flag: Laos", U_1F1F1_1F1E6, "1f1f1-1f1e6.png");
png_name!(FLAG_LB, "🇱🇧", "flag: Lebanon", U_1F1F1_1F1E7, "1f1f1-1f1e7.png");
png_name!(LEBANON, "🇱🇧", "flag: Lebanon", U_1F1F1_1F1E7, "1f1f1-1f1e7.png");
png_name!(FLAG_LC, "🇱🇨", "flag: St. Lucia", U_1F1F1_1F1E8, "1f1f1-1f1e8.png");
png_name!(ST_LUCIA, "🇱🇨", "flag: St. Lucia", U_1F1F1_1F1E8, "1f1f1-1f1e8.png");
png_name!(FLAG_LI, "🇱🇮", "flag: Liechtenstein", U_1F1F1_1F1EE, "1f1f1-1f1ee.png");
png_name!(LIECHTENSTEIN, "🇱🇮", "flag: Liechtenstein", U_1F1F1_1F1EE, "1f1f1-1f1ee.png");
png_name!(FLAG_LK, "🇱🇰", "flag: Sri Lanka", U_1F1F1_1F1F0, "1f1f1-1f1f0.png");
png_name!(SRI_LANKA, "🇱🇰", "flag: Sri Lanka", U_1F1F1_1F1F0, "1f1f1-1f1f0.png");
png_name!(FLAG_LR, "🇱🇷", "flag: Liberia", U_1F1F1_1F1F7, "1f1f1-1f1f7.png");
png_name!(LIBERIA, "🇱🇷", "flag: Liberia", U_1F1F1_1F1F7, "1f1f1-1f1f7.png");
png_name!(FLAG_LS, "🇱🇸", "flag: Lesotho", U_1F1F1_1F1F8, "1f1f1-1f1f8.png");
png_name!(LESOTHO, "🇱🇸", "flag: Lesotho", U_1F1F1_1F1F8, "1f1f1-1f1f8.png");
png_name!(FLAG_LT, "🇱🇹", "flag: Lithuania", U_1F1F1_1F1F9, "1f1f1-1f1f9.png");
png_name!(LITHUANIA, "🇱🇹", "flag: Lithuania", U_1F1F1_1F1F9, "1f1f1-1f1f9.png");
png_name!(FLAG_LU, "🇱🇺", "flag: Luxembourg", U_1F1F1_1F1FA, "1f1f1-1f1fa.png");
png_name!(LUXEMBOURG, "🇱🇺", "flag: Luxembourg", U_1F1F1_1F1FA, "1f1f1-1f1fa.png");
png_name!(FLAG_LV, "🇱🇻", "flag: Latvia", U_1F1F1_1F1FB, "1f1f1-1f1fb.png");
png_name!(LATVIA, "🇱🇻", "flag: Latvia", U_1F1F1_1F1FB, "1f1f1-1f1fb.png");
png_name!(FLAG_LY, "🇱🇾", "flag: Libya", U_1F1F1_1F1FE, "1f1f1-1f1fe.png");
png_name!(LIBYA, "🇱🇾", "flag: Libya", U_1F1F1_1F1FE, "1f1f1-1f1fe.png");
png_name!(REGIONAL_INDICATOR_L, "🇱", "regional indicator L", U_1F1F1, "1f1f1.png");
png_name!(FLAG_MA, "🇲🇦", "flag: Morocco", U_1F1F2_1F1E6, "1f1f2-1f1e6.png");
png_name!(MOROCCO, "🇲🇦", "flag: Morocco", U_1F1F2_1F1E6, "1f1f2-1f1e6.png");
png_name!(FLAG_MC, "🇲🇨", "flag: Monaco", U_1F1F2_1F1E8, "1f1f2-1f1e8.png");
png_name!(MONACO, "🇲🇨", "flag: Monaco", U_1F1F2_1F1E8, "1f1f2-1f1e8.png");
png_name!(FLAG_MD, "🇲🇩", "flag: Moldova", U_1F1F2_1F1E9, "1f1f2-1f1e9.png");
png_name!(MOLDOVA, "🇲🇩", "flag: Moldova", U_1F1F2_1F1E9, "1f1f2-1f1e9.png");
png_name!(FLAG_ME, "🇲🇪", "flag: Montenegro", U_1F1F2_1F1EA, "1f1f2-1f1ea.png");
png_name!(MONTENEGRO, "🇲🇪", "flag: Montenegro", U_1F1F2_1F1EA, "1f1f2-1f1ea.png");
png_name!(FLAG_MF, "🇲🇫", "flag: St. Martin", U_1F1F2_1F1EB, "1f1f2-1f1eb.png");
png_name!(ST_MARTIN, "🇲🇫", "flag: St. Martin", U_1F1F2_1F1EB, "1f1f2-1f1eb.png");
png_name!(FLAG_MG, "🇲🇬", "flag: Madagascar", U_1F1F2_1F1EC, "1f1f2-1f1ec.png");
png_name!(MADAGASCAR, "🇲🇬", "flag: Madagascar", U_1F1F2_1F1EC, "1f1f2-1f1ec.png");
png_name!(FLAG_MH, "🇲🇭", "flag: Marshall Islands", U_1F1F2_1F1ED, "1f1f2-1f1ed.png");
png_name!(MARSHALL_ISLANDS, "🇲🇭", "flag: Marshall Islands", U_1F1F2_1F1ED, "1f1f2-1f1ed.png");
png_name!(FLAG_MK, "🇲🇰", "flag: North Macedonia", U_1F1F2_1F1F0, "1f1f2-1f1f0.png");
png_name!(MACEDONIA, "🇲🇰", "flag: North Macedonia", U_1F1F2_1F1F0, "1f1f2-1f1f0.png");
png_name!(FLAG_ML, "🇲🇱", "flag: Mali", U_1F1F2_1F1F1, "1f1f2-1f1f1.png");
png_name!(MALI, "🇲🇱", "flag: Mali", U_1F1F2_1F1F1, "1f1f2-1f1f1.png");
png_name!(BURMA, "🇲🇲", "flag: Myanmar (Burma)", U_1F1F2_1F1F2, "1f1f2-1f1f2.png");
png_name!(FLAG_MM, "🇲🇲", "flag: Myanmar (Burma)", U_1F1F2_1F1F2, "1f1f2-1f1f2.png");
png_name!(MYANMAR, "🇲🇲", "flag: Myanmar (Burma)", U_1F1F2_1F1F2, "1f1f2-1f1f2.png");
png_name!(FLAG_MN, "🇲🇳", "flag: Mongolia", U_1F1F2_1F1F3, "1f1f2-1f1f3.png");
png_name!(MONGOLIA, "🇲🇳", "flag: Mongolia", U_1F1F2_1F1F3, "1f1f2-1f1f3.png");
png_name!(FLAG_MO, "🇲🇴", "flag: Macao SAR China", U_1F1F2_1F1F4, "1f1f2-1f1f4.png");
png_name!(MACAO, "🇲🇴", "flag: Macao SAR China", U_1F1F2_1F1F4, "1f1f2-1f1f4.png");
png_name!(MACAU, "🇲🇴", "flag: Macao SAR China", U_1F1F2_1F1F4, "1f1f2-1f1f4.png");
png_name!(FLAG_MP, "🇲🇵", "flag: Northern Mariana Islands", U_1F1F2_1F1F5, "1f1f2-1f1f5.png");
png_name!(NORTHERN_MARIANA_ISLANDS, "🇲🇵", "flag: Northern Mariana Islands", U_1F1F2_1F1F5, "1f1f2-1f1f5.png");
png_name!(FLAG_MQ, "🇲🇶", "flag: Martinique", U_1F1F2_1F1F6, "1f1f2-1f1f6.png");
png_name!(MARTINIQUE, "🇲🇶", "flag: Martinique", U_1F1F2_1F1F6, "1f1f2-1f1f6.png");
png_name!(FLAG_MR, "🇲🇷", "flag: Mauritania", U_1F1F2_1F1F7, "1f1f2-1f1f7.png");
png_name!(MAURITANIA, "🇲🇷", "flag: Mauritania", U_1F1F2_1F1F7, "1f1f2-1f1f7.png");
png_name!(FLAG_MS, "🇲🇸", "flag: Montserrat", U_1F1F2_1F1F8, "1f1f2-1f1f8.png");
png_name!(MONTSERRAT, "🇲🇸", "flag: Montserrat", U_1F1F2_1F1F8, "1f1f2-1f1f8.png");
png_name!(FLAG_MT, "🇲🇹", "flag: Malta", U_1F1F2_1F1F9, "1f1f2-1f1f9.png");
png_name!(MALTA, "🇲🇹", "flag: Malta", U_1F1F2_1F1F9, "1f1f2-1f1f9.png");
png_name!(FLAG_MU, "🇲🇺", "flag: Mauritius", U_1F1F2_1F1FA, "1f1f2-1f1fa.png");
png_name!(MAURITIUS, "🇲🇺", "flag: Mauritius", U_1F1F2_1F1FA, "1f1f2-1f1fa.png");
png_name!(FLAG_MV, "🇲🇻", "flag: Maldives", U_1F1F2_1F1FB, "1f1f2-1f1fb.png");
png_name!(MALDIVES, "🇲🇻", "flag: Maldives", U_1F1F2_1F1FB, "1f1f2-1f1fb.png");
png_name!(FLAG_MW, "🇲🇼", "flag: Malawi", U_1F1F2_1F1FC, "1f1f2-1f1fc.png");
png_name!(MALAWI, "🇲🇼", "flag: Malawi", U_1F1F2_1F1FC, "1f1f2-1f1fc.png");
png_name!(FLAG_MX, "🇲🇽", "flag: Mexico", U_1F1F2_1F1FD, "1f1f2-1f1fd.png");
png_name!(MEXICO, "🇲🇽", "flag: Mexico", U_1F1F2_1F1FD, "1f1f2-1f1fd.png");
png_name!(FLAG_MY, "🇲🇾", "flag: Malaysia", U_1F1F2_1F1FE, "1f1f2-1f1fe.png");
png_name!(MALAYSIA, "🇲🇾", "flag: Malaysia", U_1F1F2_1F1FE, "1f1f2-1f1fe.png");
png_name!(FLAG_MZ, "🇲🇿", "flag: Mozambique", U_1F1F2_1F1FF, "1f1f2-1f1ff.png");
png_name!(MOZAMBIQUE, "🇲🇿", "flag: Mozambique", U_1F1F2_1F1FF, "1f1f2-1f1ff.png");
png_name!(REGIONAL_INDICATOR_M, "🇲", "regional indicator M", U_1F1F2, "1f1f2.png");
png_name!(FLAG_NA, "🇳🇦", "flag: Namibia", U_1F1F3_1F1E6, "1f1f3-1f1e6.png");
png_name!(NAMIBIA, "🇳🇦", "flag: Namibia", U_1F1F3_1F1E6, "1f1f3-1f1e6.png");
png_name!(FLAG_NC, "🇳🇨", "flag: New Caledonia", U_1F1F3_1F1E8, "1f1f3-1f1e8.png");
png_name!(NEW_CALEDONIA, "🇳🇨", "flag: New Caledonia", U_1F1F3_1F1E8, "1f1f3-1f1e8.png");
png_name!(FLAG_NE, "🇳🇪", "flag: Niger", U_1F1F3_1F1EA, "1f1f3-1f1ea.png");
png_name!(NIGER, "🇳🇪", "flag: Niger", U_1F1F3_1F1EA, "1f1f3-1f1ea.png");
png_name!(FLAG_NF, "🇳🇫", "flag: Norfolk Island", U_1F1F3_1F1EB, "1f1f3-1f1eb.png");
png_name!(NORFOLK_ISLAND, "🇳🇫", "flag: Norfolk Island", U_1F1F3_1F1EB, "1f1f3-1f1eb.png");
png_name!(FLAG_NG, "🇳🇬", "flag: Nigeria", U_1F1F3_1F1EC, "1f1f3-1f1ec.png");
png_name!(NIGERIA, "🇳🇬", "flag: Nigeria", U_1F1F3_1F1EC, "1f1f3-1f1ec.png");
png_name!(FLAG_NI, "🇳🇮", "flag: Nicaragua", U_1F1F3_1F1EE, "1f1f3-1f1ee.png");
png_name!(NICARAGUA, "🇳🇮", "flag: Nicaragua", U_1F1F3_1F1EE, "1f1f3-1f1ee.png");
png_name!(FLAG_NL, "🇳🇱", "flag: Netherlands", U_1F1F3_1F1F1, "1f1f3-1f1f1.png");
png_name!(NETHERLANDS, "🇳🇱", "flag: Netherlands", U_1F1F3_1F1F1, "1f1f3-1f1f1.png");
png_name!(FLAG_NO, "🇳🇴", "flag: Norway", U_1F1F3_1F1F4, "1f1f3-1f1f4.png");
png_name!(NORWAY, "🇳🇴", "flag: Norway", U_1F1F3_1F1F4, "1f1f3-1f1f4.png");
png_name!(FLAG_NP, "🇳🇵", "flag: Nepal", U_1F1F3_1F1F5, "1f1f3-1f1f5.png");
png_name!(NEPAL, "🇳🇵", "flag: Nepal", U_1F1F3_1F1F5, "1f1f3-1f1f5.png");
png_name!(FLAG_NR, "🇳🇷", "flag: Nauru", U_1F1F3_1F1F7, "1f1f3-1f1f7.png");
png_name!(NAURU, "🇳🇷", "flag: Nauru", U_1F1F3_1F1F7, "1f1f3-1f1f7.png");
png_name!(FLAG_NU, "🇳🇺", "flag: Niue", U_1F1F3_1F1FA, "1f1f3-1f1fa.png");
png_name!(NIUE, "🇳🇺", "flag: Niue", U_1F1F3_1F1FA, "1f1f3-1f1fa.png");
png_name!(FLAG_NZ, "🇳🇿", "flag: New Zealand", U_1F1F3_1F1FF, "1f1f3-1f1ff.png");
png_name!(NEW_ZEALAND, "🇳🇿", "flag: New Zealand", U_1F1F3_1F1FF, "1f1f3-1f1ff.png");
png_name!(REGIONAL_INDICATOR_N, "🇳", "regional indicator N", U_1F1F3, "1f1f3.png");
png_name!(FLAG_OM, "🇴🇲", "flag: Oman", U_1F1F4_1F1F2, "1f1f4-1f1f2.png");
png_name!(OMAN, "🇴🇲", "flag: Oman", U_1F1F4_1F1F2, "1f1f4-1f1f2.png");
png_name!(REGIONAL_INDICATOR_O, "🇴", "regional indicator O", U_1F1F4, "1f1f4.png");
png_name!(FLAG_PA, "🇵🇦", "flag: Panama", U_1F1F5_1F1E6, "1f1f5-1f1e6.png");
png_name!(PANAMA, "🇵🇦", "flag: Panama", U_1F1F5_1F1E6, "1f1f5-1f1e6.png");
png_name!(FLAG_PE, "🇵🇪", "flag: Peru", U_1F1F5_1F1EA, "1f1f5-1f1ea.png");
png_name!(PERU, "🇵🇪", "flag: Peru", U_1F1F5_1F1EA, "1f1f5-1f1ea.png");
png_name!(FLAG_PF, "🇵🇫", "flag: French Polynesia", U_1F1F5_1F1EB, "1f1f5-1f1eb.png");
png_name!(FRENCH_POLYNESIA, "🇵🇫", "flag: French Polynesia", U_1F1F5_1F1EB, "1f1f5-1f1eb.png");
png_name!(FLAG_PG, "🇵🇬", "flag: Papua New Guinea", U_1F1F5_1F1EC, "1f1f5-1f1ec.png");
png_name!(PAPUA_NEW_GUINEA, "🇵🇬", "flag: Papua New Guinea", U_1F1F5_1F1EC, "1f1f5-1f1ec.png");
png_name!(FLAG_PH, "🇵🇭", "flag: Philippines", U_1F1F5_1F1ED, "1f1f5-1f1ed.png");
png_name!(PHILIPPINES, "🇵🇭", "flag: Philippines", U_1F1F5_1F1ED, "1f1f5-1f1ed.png");
png_name!(FLAG_PK, "🇵🇰", "flag: Pakistan", U_1F1F5_1F1F0, "1f1f5-1f1f0.png");
png_name!(PAKISTAN, "🇵🇰", "flag: Pakistan", U_1F1F5_1F1F0, "1f1f5-1f1f0.png");
png_name!(FLAG_PL, "🇵🇱", "flag: Poland", U_1F1F5_1F1F1, "1f1f5-1f1f1.png");
png_name!(POLAND, "🇵🇱", "flag: Poland", U_1F1F5_1F1F1, "1f1f5-1f1f1.png");
png_name!(FLAG_PM, "🇵🇲", "flag: St. Pierre & Miquelon", U_1F1F5_1F1F2, "1f1f5-1f1f2.png");
png_name!(ST_PIERRE_MIQUELON, "🇵🇲", "flag: St. Pierre & Miquelon", U_1F1F5_1F1F2, "1f1f5-1f1f2.png");
png_name!(FLAG_PN, "🇵🇳", "flag: Pitcairn Islands", U_1F1F5_1F1F3, "1f1f5-1f1f3.png");
png_name!(PITCAIRN_ISLANDS, "🇵🇳", "flag: Pitcairn Islands", U_1F1F5_1F1F3, "1f1f5-1f1f3.png");
png_name!(FLAG_PR, "🇵🇷", "flag: Puerto Rico", U_1F1F5_1F1F7, "1f1f5-1f1f7.png");
png_name!(PUERTO_RICO, "🇵🇷", "flag: Puerto Rico", U_1F1F5_1F1F7, "1f1f5-1f1f7.png");
png_name!(FLAG_PS, "🇵🇸", "flag: Palestinian Territories", U_1F1F5_1F1F8, "1f1f5-1f1f8.png");
png_name!(PALESTINIAN_TERRITORIES, "🇵🇸", "flag: Palestinian Territories", U_1F1F5_1F1F8, "1f1f5-1f1f8.png");
png_name!(FLAG_PT, "🇵🇹", "flag: Portugal", U_1F1F5_1F1F9, "1f1f5-1f1f9.png");
png_name!(PORTUGAL, "🇵🇹", "flag: Portugal", U_1F1F5_1F1F9, "1f1f5-1f1f9.png");
png_name!(FLAG_PW, "🇵🇼", "flag: Palau", U_1F1F5_1F1FC, "1f1f5-1f1fc.png");
png_name!(PALAU, "🇵🇼", "flag: Palau", U_1F1F5_1F1FC, "1f1f5-1f1fc.png");
png_name!(FLAG_PY, "🇵🇾", "flag: Paraguay", U_1F1F5_1F1FE, "1f1f5-1f1fe.png");
png_name!(PARAGUAY, "🇵🇾", "flag: Paraguay", U_1F1F5_1F1FE, "1f1f5-1f1fe.png");
png_name!(REGIONAL_INDICATOR_P, "🇵", "regional indicator P", U_1F1F5, "1f1f5.png");
png_name!(FLAG_QA, "🇶🇦", "flag: Qatar", U_1F1F6_1F1E6, "1f1f6-1f1e6.png");
png_name!(QATAR, "🇶🇦", "flag: Qatar", U_1F1F6_1F1E6, "1f1f6-1f1e6.png");
png_name!(REGIONAL_INDICATOR_Q, "🇶", "regional indicator Q", U_1F1F6, "1f1f6.png");
png_name!(FLAG_RE, "🇷🇪", "flag: Réunion", U_1F1F7_1F1EA, "1f1f7-1f1ea.png");
png_name!(REUNION, "🇷🇪", "flag: Réunion", U_1F1F7_1F1EA, "1f1f7-1f1ea.png");
png_name!(FLAG_RO, "🇷🇴", "flag: Romania", U_1F1F7_1F1F4, "1f1f7-1f1f4.png");
png_name!(ROMANIA, "🇷🇴", "flag: Romania", U_1F1F7_1F1F4, "1f1f7-1f1f4.png");
png_name!(FLAG_RS, "🇷🇸", "flag: Serbia", U_1F1F7_1F1F8, "1f1f7-1f1f8.png");
png_name!(SERBIA, "🇷🇸", "flag: Serbia", U_1F1F7_1F1F8, "1f1f7-1f1f8.png");
png_name!(FLAG_RU, "🇷🇺", "flag: Russia", U_1F1F7_1F1FA, "1f1f7-1f1fa.png");
png_name!(RUSSIA, "🇷🇺", "flag: Russia", U_1F1F7_1F1FA, "1f1f7-1f1fa.png");
png_name!(FLAG_RW, "🇷🇼", "flag: Rwanda", U_1F1F7_1F1FC, "1f1f7-1f1fc.png");
png_name!(RWANDA, "🇷🇼", "flag: Rwanda", U_1F1F7_1F1FC, "1f1f7-1f1fc.png");
png_name!(REGIONAL_INDICATOR_R, "🇷", "regional indicator R", U_1F1F7, "1f1f7.png");
png_name!(FLAG_SA, "🇸🇦", "flag: Saudi Arabia", U_1F1F8_1F1E6, "1f1f8-1f1e6.png");
png_name!(SAUDI_ARABIA, "🇸🇦", "flag: Saudi Arabia", U_1F1F8_1F1E6, "1f1f8-1f1e6.png");
png_name!(FLAG_SB, "🇸🇧", "flag: Solomon Islands", U_1F1F8_1F1E7, "1f1f8-1f1e7.png");
png_name!(SOLOMON_ISLANDS, "🇸🇧", "flag: Solomon Islands", U_1F1F8_1F1E7, "1f1f8-1f1e7.png");
png_name!(FLAG_SC, "🇸🇨", "flag: Seychelles", U_1F1F8_1F1E8, "1f1f8-1f1e8.png");
png_name!(SEYCHELLES, "🇸🇨", "flag: Seychelles", U_1F1F8_1F1E8, "1f1f8-1f1e8.png");
png_name!(FLAG_SD, "🇸🇩", "flag: Sudan", U_1F1F8_1F1E9, "1f1f8-1f1e9.png");
png_name!(SUDAN, "🇸🇩", "flag: Sudan", U_1F1F8_1F1E9, "1f1f8-1f1e9.png");
png_name!(FLAG_SE, "🇸🇪", "flag: Sweden", U_1F1F8_1F1EA, "1f1f8-1f1ea.png");
png_name!(SWEDEN, "🇸🇪", "flag: Sweden", U_1F1F8_1F1EA, "1f1f8-1f1ea.png");
png_name!(FLAG_SG, "🇸🇬", "flag: Singapore", U_1F1F8_1F1EC, "1f1f8-1f1ec.png");
png_name!(SINGAPORE, "🇸🇬", "flag: Singapore", U_1F1F8_1F1EC, "1f1f8-1f1ec.png");
png_name!(FLAG_SH, "🇸🇭", "flag: St. Helena", U_1F1F8_1F1ED, "1f1f8-1f1ed.png");
png_name!(ST_HELENA, "🇸🇭", "flag: St. Helena", U_1F1F8_1F1ED, "1f1f8-1f1ed.png");
png_name!(FLAG_SI, "🇸🇮", "flag: Slovenia", U_1F1F8_1F1EE, "1f1f8-1f1ee.png");
png_name!(SLOVENIA, "🇸🇮", "flag: Slovenia", U_1F1F8_1F1EE, "1f1f8-1f1ee.png");
png_name!(FLAG_SJ, "🇸🇯", "flag: Svalbard & Jan Mayen", U_1F1F8_1F1EF, "1f1f8-1f1ef.png");
png_name!(SVALBARD_JAN_MAYEN, "🇸🇯", "flag: Svalbard & Jan Mayen", U_1F1F8_1F1EF, "1f1f8-1f1ef.png");
png_name!(FLAG_SK, "🇸🇰", "flag: Slovakia", U_1F1F8_1F1F0, "1f1f8-1f1f0.png");
png_name!(SLOVAKIA, "🇸🇰", "flag: Slovakia", U_1F1F8_1F1F0, "1f1f8-1f1f0.png");
png_name!(FLAG_SL, "🇸🇱", "flag: Sierra Leone", U_1F1F8_1F1F1, "1f1f8-1f1f1.png");
png_name!(SIERRA_LEONE, "🇸🇱", "flag: Sierra Leone", U_1F1F8_1F1F1, "1f1f8-1f1f1.png");
png_name!(FLAG_SM, "🇸🇲", "flag: San Marino", U_1F1F8_1F1F2, "1f1f8-1f1f2.png");
png_name!(SAN_MARINO, "🇸🇲", "flag: San Marino", U_1F1F8_1F1F2, "1f1f8-1f1f2.png");
png_name!(FLAG_SN, "🇸🇳", "flag: Senegal", U_1F1F8_1F1F3, "1f1f8-1f1f3.png");
png_name!(SENEGAL, "🇸🇳", "flag: Senegal", U_1F1F8_1F1F3, "1f1f8-1f1f3.png");
png_name!(FLAG_SO, "🇸🇴", "flag: Somalia", U_1F1F8_1F1F4, "1f1f8-1f1f4.png");
png_name!(SOMALIA, "🇸🇴", "flag: Somalia", U_1F1F8_1F1F4, "1f1f8-1f1f4.png");
png_name!(FLAG_SR, "🇸🇷", "flag: Suriname", U_1F1F8_1F1F7, "1f1f8-1f1f7.png");
png_name!(SURINAME, "🇸🇷", "flag: Suriname", U_1F1F8_1F1F7, "1f1f8-1f1f7.png");
png_name!(FLAG_SS, "🇸🇸", "flag: South Sudan", U_1F1F8_1F1F8, "1f1f8-1f1f8.png");
png_name!(SOUTH_SUDAN, "🇸🇸", "flag: South Sudan", U_1F1F8_1F1F8, "1f1f8-1f1f8.png");
png_name!(FLAG_ST, "🇸🇹", "flag: São Tomé & Príncipe", U_1F1F8_1F1F9, "1f1f8-1f1f9.png");
png_name!(SAO_TOME_PRINCIPE, "🇸🇹", "flag: São Tomé & Príncipe", U_1F1F8_1F1F9, "1f1f8-1f1f9.png");
png_name!(EL_SALVADOR, "🇸🇻", "flag: El Salvador", U_1F1F8_1F1FB, "1f1f8-1f1fb.png");
png_name!(FLAG_SV, "🇸🇻", "flag: El Salvador", U_1F1F8_1F1FB, "1f1f8-1f1fb.png");
png_name!(FLAG_SX, "🇸🇽", "flag: Sint Maarten", U_1F1F8_1F1FD, "1f1f8-1f1fd.png");
png_name!(SINT_MAARTEN, "🇸🇽", "flag: Sint Maarten", U_1F1F8_1F1FD, "1f1f8-1f1fd.png");
png_name!(FLAG_SY, "🇸🇾", "flag: Syria", U_1F1F8_1F1FE, "1f1f8-1f1fe.png");
png_name!(SYRIA, "🇸🇾", "flag: Syria", U_1F1F8_1F1FE, "1f1f8-1f1fe.png");
png_name!(ESWATINI, "🇸🇿", "flag: Eswatini", U_1F1F8_1F1FF, "1f1f8-1f1ff.png");
png_name!(FLAG_SZ, "🇸🇿", "flag: Eswatini", U_1F1F8_1F1FF, "1f1f8-1f1ff.png");
png_name!(SWAZILAND, "🇸🇿", "flag: Eswatini", U_1F1F8_1F1FF, "1f1f8-1f1ff.png");
png_name!(REGIONAL_INDICATOR_S, "🇸", "regional indicator S", U_1F1F8, "1f1f8.png");
png_name!(FLAG_TA, "🇹🇦", "flag: Tristan da Cunha", U_1F1F9_1F1E6, "1f1f9-1f1e6.png");
png_name!(TRISTAN_DA_CUNHA, "🇹🇦", "flag: Tristan da Cunha", U_1F1F9_1F1E6, "1f1f9-1f1e6.png");
png_name!(FLAG_TC, "🇹🇨", "flag: Turks & Caicos Islands", U_1F1F9_1F1E8, "1f1f9-1f1e8.png");
png_name!(TURKS_CAICOS_ISLANDS, "🇹🇨", "flag: Turks & Caicos Islands", U_1F1F9_1F1E8, "1f1f9-1f1e8.png");
png_name!(CHAD, "🇹🇩", "flag: Chad", U_1F1F9_1F1E9, "1f1f9-1f1e9.png");
png_name!(FLAG_TD, "🇹🇩", "flag: Chad", U_1F1F9_1F1E9, "1f1f9-1f1e9.png");
png_name!(FLAG_TF, "🇹🇫", "flag: French Southern Territories", U_1F1F9_1F1EB, "1f1f9-1f1eb.png");
png_name!(FRENCH_SOUTHERN_TERRITORIES, "🇹🇫", "flag: French Southern Territories", U_1F1F9_1F1EB, "1f1f9-1f1eb.png");
png_name!(FLAG_TG, "🇹🇬", "flag: Togo", U_1F1F9_1F1EC, "1f1f9-1f1ec.png");
png_name!(TOGO, "🇹🇬", "flag: Togo", U_1F1F9_1F1EC, "1f1f9-1f1ec.png");
png_name!(FLAG_TH, "🇹🇭", "flag: Thailand", U_1F1F9_1F1ED, "1f1f9-1f1ed.png");
png_name!(THAILAND, "🇹🇭", "flag: Thailand", U_1F1F9_1F1ED, "1f1f9-1f1ed.png");
png_name!(FLAG_TJ, "🇹🇯", "flag: Tajikistan", U_1F1F9_1F1EF, "1f1f9-1f1ef.png");
png_name!(TAJIKISTAN, "🇹🇯", "flag: Tajikistan", U_1F1F9_1F1EF, "1f1f9-1f1ef.png");
png_name!(FLAG_TK, "🇹🇰", "flag: Tokelau", U_1F1F9_1F1F0, "1f1f9-1f1f0.png");
png_name!(TOKELAU, "🇹🇰", "flag: Tokelau", U_1F1F9_1F1F0, "1f1f9-1f1f0.png");
png_name!(FLAG_TL, "🇹🇱", "flag: Timor-Leste", U_1F1F9_1F1F1, "1f1f9-1f1f1.png");
png_name!(TIMOR_LESTE, "🇹🇱", "flag: Timor-Leste", U_1F1F9_1F1F1, "1f1f9-1f1f1.png");
png_name!(FLAG_TM, "🇹🇲", "flag: Turkmenistan", U_1F1F9_1F1F2, "1f1f9-1f1f2.png");
png_name!(TURKMENISTAN, "🇹🇲", "flag: Turkmenistan", U_1F1F9_1F1F2, "1f1f9-1f1f2.png");
png_name!(FLAG_TN, "🇹🇳", "flag: Tunisia", U_1F1F9_1F1F3, "1f1f9-1f1f3.png");
png_name!(TUNISIA, "🇹🇳", "flag: Tunisia", U_1F1F9_1F1F3, "1f1f9-1f1f3.png");
png_name!(FLAG_TO, "🇹🇴", "flag: Tonga", U_1F1F9_1F1F4, "1f1f9-1f1f4.png");
png_name!(TONGA, "🇹🇴", "flag: Tonga", U_1F1F9_1F1F4, "1f1f9-1f1f4.png");
png_name!(FLAG_TR, "🇹🇷", "flag: Türkiye", U_1F1F9_1F1F7, "1f1f9-1f1f7.png");
png_name!(TURKEY_TR, "🇹🇷", "flag: Türkiye", U_1F1F9_1F1F7, "1f1f9-1f1f7.png");
png_name!(FLAG_TT, "🇹🇹", "flag: Trinidad & Tobago", U_1F1F9_1F1F9, "1f1f9-1f1f9.png");
png_name!(TRINIDAD_TOBAGO, "🇹🇹", "flag: Trinidad & Tobago", U_1F1F9_1F1F9, "1f1f9-1f1f9.png");
png_name!(FLAG_TV, "🇹🇻", "flag: Tuvalu", U_1F1F9_1F1FB, "1f1f9-1f1fb.png");
png_name!(TUVALU, "🇹🇻", "flag: Tuvalu", U_1F1F9_1F1FB, "1f1f9-1f1fb.png");
png_name!(FLAG_TW, "🇹🇼", "flag: Taiwan", U_1F1F9_1F1FC, "1f1f9-1f1fc.png");
png_name!(TAIWAN, "🇹🇼", "flag: Taiwan", U_1F1F9_1F1FC, "1f1f9-1f1fc.png");
png_name!(FLAG_TZ, "🇹🇿", "flag: Tanzania", U_1F1F9_1F1FF, "1f1f9-1f1ff.png");
png_name!(TANZANIA, "🇹🇿", "flag: Tanzania", U_1F1F9_1F1FF, "1f1f9-1f1ff.png");
png_name!(REGIONAL_INDICATOR_T, "🇹", "regional indicator T", U_1F1F9, "1f1f9.png");
png_name!(FLAG_UA, "🇺🇦", "flag: Ukraine", U_1F1FA_1F1E6, "1f1fa-1f1e6.png");
png_name!(UKRAINE, "🇺🇦", "flag: Ukraine", U_1F1FA_1F1E6, "1f1fa-1f1e6.png");
png_name!(FLAG_UG, "🇺🇬", "flag: Uganda", U_1F1FA_1F1EC, "1f1fa-1f1ec.png");
png_name!(UGANDA, "🇺🇬", "flag: Uganda", U_1F1FA_1F1EC, "1f1fa-1f1ec.png");
png_name!(FLAG_UM, "🇺🇲", "flag: U.S. Outlying Islands", U_1F1FA_1F1F2, "1f1fa-1f1f2.png");
png_name!(US_OUTLYING_ISLANDS, "🇺🇲", "flag: U.S. Outlying Islands", U_1F1FA_1F1F2, "1f1fa-1f1f2.png");
png_name!(FLAG_UN, "🇺🇳", "flag: United Nations", U_1F1FA_1F1F3, "1f1fa-1f1f3.png");
png_name!(UN, "🇺🇳", "flag: United Nations", U_1F1FA_1F1F3, "1f1fa-1f1f3.png");
png_name!(UNITED_NATIONS, "🇺🇳", "flag: United Nations", U_1F1FA_1F1F3, "1f1fa-1f1f3.png");
png_name!(FLAG_US, "🇺🇸", "flag: United States", U_1F1FA_1F1F8, "1f1fa-1f1f8.png");
png_name!(UNITED_STATES, "🇺🇸", "flag: United States", U_1F1FA_1F1F8, "1f1fa-1f1f8.png");
png_name!(USA, "🇺🇸", "flag: United States", U_1F1FA_1F1F8, "1f1fa-1f1f8.png");
png_name!(FLAG_UY, "🇺🇾", "flag: Uruguay", U_1F1FA_1F1FE, "1f1fa-1f1fe.png");
png_name!(URUGUAY, "🇺🇾", "flag: Uruguay", U_1F1FA_1F1FE, "1f1fa-1f1fe.png");
png_name!(FLAG_UZ, "🇺🇿", "flag: Uzbekistan", U_1F1FA_1F1FF, "1f1fa-1f1ff.png");
png_name!(UZBEKISTAN, "🇺🇿", "flag: Uzbekistan", U_1F1FA_1F1FF, "1f1fa-1f1ff.png");
png_name!(REGIONAL_INDICATOR_U, "🇺", "regional indicator U", U_1F1FA, "1f1fa.png");
png_name!(FLAG_VA, "🇻🇦", "flag: Vatican City", U_1F1FB_1F1E6, "1f1fb-1f1e6.png");
png_name!(VATICAN_CITY, "🇻🇦", "flag: Vatican City", U_1F1FB_1F1E6, "1f1fb-1f1e6.png");
png_name!(FLAG_VC, "🇻🇨", "flag: St. Vincent & Grenadines", U_1F1FB_1F1E8, "1f1fb-1f1e8.png");
png_name!(ST_VINCENT_GRENADINES, "🇻🇨", "flag: St. Vincent & Grenadines", U_1F1FB_1F1E8, "1f1fb-1f1e8.png");
png_name!(FLAG_VE, "🇻🇪", "flag: Venezuela", U_1F1FB_1F1EA, "1f1fb-1f1ea.png");
png_name!(VENEZUELA, "🇻🇪", "flag: Venezuela", U_1F1FB_1F1EA, "1f1fb-1f1ea.png");
png_name!(BRITISH_VIRGIN_ISLANDS, "🇻🇬", "flag: British Virgin Islands", U_1F1FB_1F1EC, "1f1fb-1f1ec.png");
png_name!(FLAG_VG, "🇻🇬", "flag: British Virgin Islands", U_1F1FB_1F1EC, "1f1fb-1f1ec.png");
png_name!(FLAG_VI, "🇻🇮", "flag: U.S. Virgin Islands", U_1F1FB_1F1EE, "1f1fb-1f1ee.png");
png_name!(US_VIRGIN_ISLANDS, "🇻🇮", "flag: U.S. Virgin Islands", U_1F1FB_1F1EE, "1f1fb-1f1ee.png");
png_name!(FLAG_VN, "🇻🇳", "flag: Vietnam", U_1F1FB_1F1F3, "1f1fb-1f1f3.png");
png_name!(VIETNAM, "🇻🇳", "flag: Vietnam", U_1F1FB_1F1F3, "1f1fb-1f1f3.png");
png_name!(FLAG_VU, "🇻🇺", "flag: Vanuatu", U_1F1FB_1F1FA, "1f1fb-1f1fa.png");
png_name!(VANUATU, "🇻🇺", "flag: Vanuatu", U_1F1FB_1F1FA, "1f1fb-1f1fa.png");
png_name!(REGIONAL_INDICATOR_V, "🇻", "regional indicator V", U_1F1FB, "1f1fb.png");
png_name!(FLAG_WF, "🇼🇫", "flag: Wallis & Futuna", U_1F1FC_1F1EB, "1f1fc-1f1eb.png");
png_name!(WALLIS_FUTUNA, "🇼🇫", "flag: Wallis & Futuna", U_1F1FC_1F1EB, "1f1fc-1f1eb.png");
png_name!(FLAG_WS, "🇼🇸", "flag: Samoa", U_1F1FC_1F1F8, "1f1fc-1f1f8.png");
png_name!(SAMOA, "🇼🇸", "flag: Samoa", U_1F1FC_1F1F8, "1f1fc-1f1f8.png");
png_name!(REGIONAL_INDICATOR_W, "🇼", "regional indicator W", U_1F1FC, "1f1fc.png");
png_name!(FLAG_XK, "🇽🇰", "flag: Kosovo", U_1F1FD_1F1F0, "1f1fd-1f1f0.png");
png_name!(KOSOVO, "🇽🇰", "flag: Kosovo", U_1F1FD_1F1F0, "1f1fd-1f1f0.png");
png_name!(REGIONAL_INDICATOR_X, "🇽", "regional indicator X", U_1F1FD, "1f1fd.png");
png_name!(FLAG_YE, "🇾🇪", "flag: Yemen", U_1F1FE_1F1EA, "1f1fe-1f1ea.png");
png_name!(YEMEN, "🇾🇪", "flag: Yemen", U_1F1FE_1F1EA, "1f1fe-1f1ea.png");
png_name!(FLAG_YT, "🇾🇹", "flag: Mayotte", U_1F1FE_1F1F9, "1f1fe-1f1f9.png");
png_name!(MAYOTTE, "🇾🇹", "flag: Mayotte", U_1F1FE_1F1F9, "1f1fe-1f1f9.png");
png_name!(REGIONAL_INDICATOR_Y, "🇾", "regional indicator Y", U_1F1FE, "1f1fe.png");
png_name!(FLAG_ZA, "🇿🇦", "flag: South Africa", U_1F1FF_1F1E6, "1f1ff-1f1e6.png");
png_name!(SOUTH_AFRICA, "🇿🇦", "flag: South Africa", U_1F1FF_1F1E6, "1f1ff-1f1e6.png");
png_name!(FLAG_ZM, "🇿🇲", "flag: Zambia", U_1F1FF_1F1F2, "1f1ff-1f1f2.png");
png_name!(ZAMBIA, "🇿🇲", "flag: Zambia", U_1F1FF_1F1F2, "1f1ff-1f1f2.png");
png_name!(FLAG_ZW, "🇿🇼", "flag: Zimbabwe", U_1F1FF_1F1FC, "1f1ff-1f1fc.png");
png_name!(ZIMBABWE, "🇿🇼", "flag: Zimbabwe", U_1F1FF_1F1FC, "1f1ff-1f1fc.png");
png_name!(REGIONAL_INDICATOR_Z, "🇿", "regional indicator Z", U_1F1FF, "1f1ff.png");
png_name!(JA_HERE, "🈁", "Japanese “here” button", U_1F201, "1f201.png");
png_name!(KOKO, "🈁", "Japanese “here” button", U_1F201, "1f201.png");
png_name!(JA_SERVICE_CHARGE, "🈂", "Japanese “service charge” button", U_1F202, "1f202.png");
png_name!(JA_FREE_OF_CHARGE, "🈚", "Japanese “free of charge” button", U_1F21A, "1f21a.png");
png_name!(JA_RESERVED, "🈯", "Japanese “reserved” button", U_1F22F, "1f22f.png");
png_name!(JA_PROHIBITED, "🈲", "Japanese “prohibited” button", U_1F232, "1f232.png");
png_name!(JA_VACANCY, "🈳", "Japanese “vacancy” button", U_1F233, "1f233.png");
png_name!(JA_PASSING_GRADE, "🈴", "Japanese “passing grade” button", U_1F234, "1f234.png");
png_name!(JA_NO_VACANCY, "🈵", "Japanese “no vacancy” button", U_1F235, "1f235.png");
png_name!(JA_NOT_FREE_OF_CARGE, "🈶", "Japanese “not free of charge” button", U_1F236, "1f236.png");
png_name!(JA_MONTHLY_AMOUNT, "🈷", "Japanese “monthly amount” button", U_1F237, "1f237.png");
png_name!(JA_APPLICATION, "🈸", "Japanese “application” button", U_1F238, "1f238.png");
png_name!(JA_DISCOUNT, "🈹", "Japanese “discount” button", U_1F239, "1f239.png");
png_name!(JA_OPEN_FOR_BUSINESS, "🈺", "Japanese “open for business” button", U_1F23A, "1f23a.png");
png_name!(IDEOGRAPH_ADVANTAGE, "🉐", "Japanese “bargain” button", U_1F250, "1f250.png");
png_name!(JA_BARGAIN, "🉐", "Japanese “bargain” button", U_1F250, "1f250.png");
png_name!(ACCEPT, "🉑", "Japanese “acceptable” button", U_1F251, "1f251.png");
png_name!(JA_ACCEPTABLE, "🉑", "Japanese “acceptable” button", U_1F251, "1f251.png");
png_name!(CYCLONE, "🌀", "cyclone", U_1F300, "1f300.png");
png_name!(FOGGY, "🌁", "foggy", U_1F301, "1f301.png");
png_name!(CLOSED_UMBRELLA, "🌂", "closed umbrella", U_1F302, "1f302.png");
png_name!(NIGHT_WITH_STARS, "🌃", "night with stars", U_1F303, "1f303.png");
png_name!(SUNRISE_OVER_MOUNTAINS, "🌄", "sunrise over mountains", U_1F304, "1f304.png");
png_name!(SUNRISE, "🌅", "sunrise", U_1F305, "1f305.png");
png_name!(CITY_DUSK, "🌆", "cityscape at dusk", U_1F306, "1f306.png");
png_name!(CITY_SUNRISE, "🌇", "sunset", U_1F307, "1f307.png");
png_name!(CITY_SUNSET, "🌇", "sunset", U_1F307, "1f307.png");
png_name!(RAINBOW, "🌈", "rainbow", U_1F308, "1f308.png");
png_name!(BRIDGE_AT_NIGHT, "🌉", "bridge at night", U_1F309, "1f309.png");
png_name!(OCEAN, "🌊", "water wave", U_1F30A, "1f30a.png");
png_name!(WATER_WAVE, "🌊", "water wave", U_1F30A, "1f30a.png");
png_name!(VOLCANO, "🌋", "volcano", U_1F30B, "1f30b.png");
png_name!(MILKY_WAY, "🌌", "milky way", U_1F30C, "1f30c.png");
png_name!(EARTH_AFRICA, "🌍", "globe showing Europe-Africa", U_1F30D, "1f30d.png");
png_name!(EARTH_EUROPE, "🌍", "globe showing Europe-Africa", U_1F30D, "1f30d.png");
png_name!(EARTH_AMERICAS, "🌎", "globe showing Americas", U_1F30E, "1f30e.png");
png_name!(EARTH_ASIA, "🌏", "globe showing Asia-Australia", U_1F30F, "1f30f.png");
png_name!(GLOBE_WITH_MERIDIANS, "🌐", "globe with meridians", U_1F310, "1f310.png");
png_name!(NEW_MOON, "🌑", "new moon", U_1F311, "1f311.png");
png_name!(WAXING_CRESCENT_MOON, "🌒", "waxing crescent moon", U_1F312, "1f312.png");
png_name!(FIRST_QUARTER_MOON, "🌓", "first quarter moon", U_1F313, "1f313.png");
png_name!(WAXING_GIBBOUS_MOON, "🌔", "waxing gibbous moon", U_1F314, "1f314.png");
png_name!(FULL_MOON, "🌕", "full moon", U_1F315, "1f315.png");
png_name!(WANING_GIBBOUS_MOON, "🌖", "waning gibbous moon", U_1F316, "1f316.png");
png_name!(LAST_QUARTER_MOON, "🌗", "last quarter moon", U_1F317, "1f317.png");
png_name!(WANING_CRESCENT_MOON, "🌘", "waning crescent moon", U_1F318, "1f318.png");
png_name!(CRESCENT_MOON, "🌙", "crescent moon", U_1F319, "1f319.png");
png_name!(NEW_MOON_WITH_FACE, "🌚", "new moon face", U_1F31A, "1f31a.png");
png_name!(FIRST_QUARTER_MOON_WITH_FACE, "🌛", "first quarter moon face", U_1F31B, "1f31b.png");
png_name!(LAST_QUARTER_MOON_WITH_FACE, "🌜", "last quarter moon face", U_1F31C, "1f31c.png");
png_name!(FULL_MOON_WITH_FACE, "🌝", "full moon face", U_1F31D, "1f31d.png");
png_name!(SUN_WITH_FACE, "🌞", "sun with face", U_1F31E, "1f31e.png");
png_name!(GLOWING_STAR, "🌟", "glowing star", U_1F31F, "1f31f.png");
png_name!(STAR2, "🌟", "glowing star", U_1F31F, "1f31f.png");
png_name!(SHOOTING_STAR, "🌠", "shooting star", U_1F320, "1f320.png");
png_name!(STARS, "🌠", "shooting star", U_1F320, "1f320.png");
png_name!(THERMOMETER, "🌡", "thermometer", U_1F321, "1f321.png");
png_name!(SUN_BEHIND_SMALL_CLOUD, "🌤", "sun behind small cloud", U_1F324, "1f324.png");
png_name!(SUNNY, "🌤", "sun behind small cloud", U_1F324, "1f324.png");
png_name!(CLOUDY, "🌥", "sun behind large cloud", U_1F325, "1f325.png");
png_name!(SUN_BEHIND_LARGE_CLOUD, "🌥", "sun behind large cloud", U_1F325, "1f325.png");
png_name!(SUN_AND_RAIN, "🌦", "sun behind rain cloud", U_1F326, "1f326.png");
png_name!(SUN_BEHIND_RAIN_CLOUD, "🌦", "sun behind rain cloud", U_1F326, "1f326.png");
png_name!(CLOUD_WITH_RAIN, "🌧", "cloud with rain", U_1F327, "1f327.png");
png_name!(RAINY, "🌧", "cloud with rain", U_1F327, "1f327.png");
png_name!(CLOUD_WITH_SNOW, "🌨", "cloud with snow", U_1F328, "1f328.png");
png_name!(SNOWY, "🌨", "cloud with snow", U_1F328, "1f328.png");
png_name!(CLOUD_WITH_LIGHTNING, "🌩", "cloud with lightning", U_1F329, "1f329.png");
png_name!(LIGHTNING, "🌩", "cloud with lightning", U_1F329, "1f329.png");
png_name!(TORNADO, "🌪", "tornado", U_1F32A, "1f32a.png");
png_name!(FOG, "🌫", "fog", U_1F32B, "1f32b.png");
png_name!(WIND_BLOWING_FACE, "🌬", "wind face", U_1F32C, "1f32c.png");
png_name!(HOTDOG, "🌭", "hot dog", U_1F32D, "1f32d.png");
png_name!(TACO, "🌮", "taco", U_1F32E, "1f32e.png");
png_name!(BURRITO, "🌯", "burrito", U_1F32F, "1f32f.png");
png_name!(CHESTNUT, "🌰", "chestnut", U_1F330, "1f330.png");
png_name!(SEEDLING, "🌱", "seedling", U_1F331, "1f331.png");
png_name!(EVERGREEN_TREE, "🌲", "evergreen tree", U_1F332, "1f332.png");
png_name!(DECIDUOUS_TREE, "🌳", "deciduous tree", U_1F333, "1f333.png");
png_name!(PALM_TREE, "🌴", "palm tree", U_1F334, "1f334.png");
png_name!(CACTUS, "🌵", "cactus", U_1F335, "1f335.png");
png_name!(HOT_PEPPER, "🌶", "hot pepper", U_1F336, "1f336.png");
png_name!(TULIP, "🌷", "tulip", U_1F337, "1f337.png");
png_name!(CHERRY_BLOSSOM, "🌸", "cherry blossom", U_1F338, "1f338.png");
png_name!(ROSE, "🌹", "rose", U_1F339, "1f339.png");
png_name!(HIBISCUS, "🌺", "hibiscus", U_1F33A, "1f33a.png");
png_name!(SUNFLOWER, "🌻", "sunflower", U_1F33B, "1f33b.png");
png_name!(BLOSSOM, "🌼", "blossom", U_1F33C, "1f33c.png");
png_name!(CORN, "🌽", "ear of corn", U_1F33D, "1f33d.png");
png_name!(EAR_OF_CORN, "🌽", "ear of corn", U_1F33D, "1f33d.png");
png_name!(EAR_OF_RICE, "🌾", "sheaf of rice", U_1F33E, "1f33e.png");
png_name!(SHEAF_OF_RICE, "🌾", "sheaf of rice", U_1F33E, "1f33e.png");
png_name!(HERB, "🌿", "herb", U_1F33F, "1f33f.png");
png_name!(FOUR_LEAF_CLOVER, "🍀", "four leaf clover", U_1F340, "1f340.png");
png_name!(MAPLE_LEAF, "🍁", "maple leaf", U_1F341, "1f341.png");
png_name!(FALLEN_LEAF, "🍂", "fallen leaf", U_1F342, "1f342.png");
png_name!(LEAVES, "🍃", "leaf fluttering in wind", U_1F343, "1f343.png");
png_name!(MUSHROOM, "🍄", "mushroom", U_1F344, "1f344.png");
png_name!(TOMATO, "🍅", "tomato", U_1F345, "1f345.png");
png_name!(EGGPLANT, "🍆", "eggplant", U_1F346, "1f346.png");
png_name!(GRAPES, "🍇", "grapes", U_1F347, "1f347.png");
png_name!(MELON, "🍈", "melon", U_1F348, "1f348.png");
png_name!(WATERMELON, "🍉", "watermelon", U_1F349, "1f349.png");
png_name!(ORANGE, "🍊", "tangerine", U_1F34A, "1f34a.png");
png_name!(TANGERINE, "🍊", "tangerine", U_1F34A, "1f34a.png");
png_name!(LEMON, "🍋", "lemon", U_1F34B, "1f34b.png");
png_name!(BANANA, "🍌", "banana", U_1F34C, "1f34c.png");
png_name!(PINEAPPLE, "🍍", "pineapple", U_1F34D, "1f34d.png");
png_name!(APPLE, "🍎", "red apple", U_1F34E, "1f34e.png");
png_name!(RED_APPLE, "🍎", "red apple", U_1F34E, "1f34e.png");
png_name!(GREEN_APPLE, "🍏", "green apple", U_1F34F, "1f34f.png");
png_name!(PEAR, "🍐", "pear", U_1F350, "1f350.png");
png_name!(PEACH, "🍑", "peach", U_1F351, "1f351.png");
png_name!(CHERRIES, "🍒", "cherries", U_1F352, "1f352.png");
png_name!(STRAWBERRY, "🍓", "strawberry", U_1F353, "1f353.png");
png_name!(HAMBURGER, "🍔", "hamburger", U_1F354, "1f354.png");
png_name!(PIZZA, "🍕", "pizza", U_1F355, "1f355.png");
png_name!(MEAT_ON_BONE, "🍖", "meat on bone", U_1F356, "1f356.png");
png_name!(POULTRY_LEG, "🍗", "poultry leg", U_1F357, "1f357.png");
png_name!(RICE_CRACKER, "🍘", "rice cracker", U_1F358, "1f358.png");
png_name!(RICE_BALL, "🍙", "rice ball", U_1F359, "1f359.png");
png_name!(COOKED_RICE, "🍚", "cooked rice", U_1F35A, "1f35a.png");
png_name!(RICE, "🍚", "cooked rice", U_1F35A, "1f35a.png");
png_name!(CURRY, "🍛", "curry rice", U_1F35B, "1f35b.png");
png_name!(CURRY_RICE, "🍛", "curry rice", U_1F35B, "1f35b.png");
png_name!(RAMEN, "🍜", "steaming bowl", U_1F35C, "1f35c.png");
png_name!(STEAMING_BOWL, "🍜", "steaming bowl", U_1F35C, "1f35c.png");
png_name!(SPAGHETTI, "🍝", "spaghetti", U_1F35D, "1f35d.png");
png_name!(BREAD, "🍞", "bread", U_1F35E, "1f35e.png");
png_name!(FRENCH_FRIES, "🍟", "french fries", U_1F35F, "1f35f.png");
png_name!(FRIES, "🍟", "french fries", U_1F35F, "1f35f.png");
png_name!(SWEET_POTATO, "🍠", "roasted sweet potato", U_1F360, "1f360.png");
png_name!(DANGO, "🍡", "dango", U_1F361, "1f361.png");
png_name!(ODEN, "🍢", "oden", U_1F362, "1f362.png");
png_name!(SUSHI, "🍣", "sushi", U_1F363, "1f363.png");
png_name!(FRIED_SHRIMP, "🍤", "fried shrimp", U_1F364, "1f364.png");
png_name!(FISH_CAKE, "🍥", "fish cake with swirl", U_1F365, "1f365.png");
png_name!(ICECREAM, "🍦", "soft ice cream", U_1F366, "1f366.png");
png_name!(SOFT_SERVE, "🍦", "soft ice cream", U_1F366, "1f366.png");
png_name!(SHAVED_ICE, "🍧", "shaved ice", U_1F367, "1f367.png");
png_name!(ICE_CREAM, "🍨", "ice cream", U_1F368, "1f368.png");
png_name!(DOUGHNUT, "🍩", "doughnut", U_1F369, "1f369.png");
png_name!(COOKIE, "🍪", "cookie", U_1F36A, "1f36a.png");
png_name!(CHOCOLATE_BAR, "🍫", "chocolate bar", U_1F36B, "1f36b.png");
png_name!(CANDY, "🍬", "candy", U_1F36C, "1f36c.png");
png_name!(LOLLIPOP, "🍭", "lollipop", U_1F36D, "1f36d.png");
png_name!(CUSTARD, "🍮", "custard", U_1F36E, "1f36e.png");
png_name!(HONEY_POT, "🍯", "honey pot", U_1F36F, "1f36f.png");
png_name!(CAKE, "🍰", "shortcake", U_1F370, "1f370.png");
png_name!(SHORTCAKE, "🍰", "shortcake", U_1F370, "1f370.png");
png_name!(BENTO, "🍱", "bento box", U_1F371, "1f371.png");
png_name!(BENTO_BOX, "🍱", "bento box", U_1F371, "1f371.png");
png_name!(POT_OF_FOOD, "🍲", "pot of food", U_1F372, "1f372.png");
png_name!(STEW, "🍲", "pot of food", U_1F372, "1f372.png");
png_name!(COOKING, "🍳", "cooking", U_1F373, "1f373.png");
png_name!(FRIED_EGG, "🍳", "cooking", U_1F373, "1f373.png");
png_name!(FORK_AND_KNIFE, "🍴", "fork and knife", U_1F374, "1f374.png");
png_name!(TEA, "🍵", "teacup without handle", U_1F375, "1f375.png");
png_name!(SAKE, "🍶", "sake", U_1F376, "1f376.png");
png_name!(WINE_GLASS, "🍷", "wine glass", U_1F377, "1f377.png");
png_name!(COCKTAIL, "🍸", "cocktail glass", U_1F378, "1f378.png");
png_name!(TROPICAL_DRINK, "🍹", "tropical drink", U_1F379, "1f379.png");
png_name!(BEER, "🍺", "beer mug", U_1F37A, "1f37a.png");
png_name!(BEERS, "🍻", "clinking beer mugs", U_1F37B, "1f37b.png");
png_name!(BABY_BOTTLE, "🍼", "baby bottle", U_1F37C, "1f37c.png");
png_name!(FORK_KNIFE_PLATE, "🍽", "fork and knife with plate", U_1F37D, "1f37d.png");
png_name!(CHAMPAGNE, "🍾", "bottle with popping cork", U_1F37E, "1f37e.png");
png_name!(POPCORN, "🍿", "popcorn", U_1F37F, "1f37f.png");
png_name!(RIBBON, "🎀", "ribbon", U_1F380, "1f380.png");
png_name!(GIFT, "🎁", "wrapped gift", U_1F381, "1f381.png");
png_name!(BIRTHDAY, "🎂", "birthday cake", U_1F382, "1f382.png");
png_name!(BIRTHDAY_CAKE, "🎂", "birthday cake", U_1F382, "1f382.png");
png_name!(JACK_O_LANTERN, "🎃", "jack-o-lantern", U_1F383, "1f383.png");
png_name!(CHRISTMAS_TREE, "🎄", "Christmas tree", U_1F384, "1f384.png");
png_name!(SANTA_TONE1, "🎅🏻", "", U_1F385_1F3FB, "1f385-1f3fb.png");
png_name!(SANTA_TONE2, "🎅🏼", "", U_1F385_1F3FC, "1f385-1f3fc.png");
png_name!(SANTA_TONE3, "🎅🏽", "", U_1F385_1F3FD, "1f385-1f3fd.png");
png_name!(SANTA_TONE4, "🎅🏾", "", U_1F385_1F3FE, "1f385-1f3fe.png");
png_name!(SANTA_TONE5, "🎅🏿", "", U_1F385_1F3FF, "1f385-1f3ff.png");
png_name!(SANTA, "🎅", "Santa Claus", U_1F385, "1f385.png");
png_name!(FIREWORKS, "🎆", "fireworks", U_1F386, "1f386.png");
png_name!(SPARKLER, "🎇", "sparkler", U_1F387, "1f387.png");
png_name!(BALLOON, "🎈", "balloon", U_1F388, "1f388.png");
png_name!(PARTY, "🎉", "party popper", U_1F389, "1f389.png");
png_name!(PARTY_POPPER, "🎉", "party popper", U_1F389, "1f389.png");
png_name!(TADA, "🎉", "party popper", U_1F389, "1f389.png");
png_name!(CONFETTI_BALL, "🎊", "confetti ball", U_1F38A, "1f38a.png");
png_name!(TANABATA_TREE, "🎋", "tanabata tree", U_1F38B, "1f38b.png");
png_name!(CROSSED_FLAGS, "🎌", "crossed flags", U_1F38C, "1f38c.png");
png_name!(BAMBOO, "🎍", "pine decoration", U_1F38D, "1f38d.png");
png_name!(DOLLS, "🎎", "Japanese dolls", U_1F38E, "1f38e.png");
png_name!(CARP_STREAMER, "🎏", "carp streamer", U_1F38F, "1f38f.png");
png_name!(FLAGS, "🎏", "carp streamer", U_1F38F, "1f38f.png");
png_name!(WIND_CHIME, "🎐", "wind chime", U_1F390, "1f390.png");
png_name!(MOON_CEREMONY, "🎑", "moon viewing ceremony", U_1F391, "1f391.png");
png_name!(RICE_SCENE, "🎑", "moon viewing ceremony", U_1F391, "1f391.png");
png_name!(BACKPACK, "🎒", "backpack", U_1F392, "1f392.png");
png_name!(SCHOOL_SATCHEL, "🎒", "backpack", U_1F392, "1f392.png");
png_name!(GRADUATION_CAP, "🎓", "graduation cap", U_1F393, "1f393.png");
png_name!(MORTAR_BOARD, "🎓", "graduation cap", U_1F393, "1f393.png");
png_name!(MILITARY_MEDAL, "🎖", "military medal", U_1F396, "1f396.png");
png_name!(REMINDER_RIBBON, "🎗", "reminder ribbon", U_1F397, "1f397.png");
png_name!(STUDIO_MICROPHONE, "🎙", "studio microphone", U_1F399, "1f399.png");
png_name!(LEVEL_SLIDER, "🎚", "level slider", U_1F39A, "1f39a.png");
png_name!(CONTROL_KNOBS, "🎛", "control knobs", U_1F39B, "1f39b.png");
png_name!(FILM_FRAMES, "🎞", "film frames", U_1F39E, "1f39e.png");
png_name!(ADMISSION_TICKETS, "🎟", "admission tickets", U_1F39F, "1f39f.png");
png_name!(TICKETS, "🎟", "admission tickets", U_1F39F, "1f39f.png");
png_name!(CAROUSEL_HORSE, "🎠", "carousel horse", U_1F3A0, "1f3a0.png");
png_name!(FERRIS_WHEEL, "🎡", "ferris wheel", U_1F3A1, "1f3a1.png");
png_name!(ROLLER_COASTER, "🎢", "roller coaster", U_1F3A2, "1f3a2.png");
png_name!(FISHING_POLE, "🎣", "fishing pole", U_1F3A3, "1f3a3.png");
png_name!(FISHING_POLE_AND_FISH, "🎣", "fishing pole", U_1F3A3, "1f3a3.png");
png_name!(MICROPHONE, "🎤", "microphone", U_1F3A4, "1f3a4.png");
png_name!(MOVIE_CAMERA, "🎥", "movie camera", U_1F3A5, "1f3a5.png");
png_name!(CINEMA, "🎦", "cinema", U_1F3A6, "1f3a6.png");
png_name!(HEADPHONES, "🎧", "headphone", U_1F3A7, "1f3a7.png");
png_name!(ART, "🎨", "artist palette", U_1F3A8, "1f3a8.png");
png_name!(PALETTE, "🎨", "artist palette", U_1F3A8, "1f3a8.png");
png_name!(TOP_HAT, "🎩", "top hat", U_1F3A9, "1f3a9.png");
png_name!(TOPHAT, "🎩", "top hat", U_1F3A9, "1f3a9.png");
png_name!(CIRCUS_TENT, "🎪", "circus tent", U_1F3AA, "1f3aa.png");
png_name!(TICKET, "🎫", "ticket", U_1F3AB, "1f3ab.png");
png_name!(CLAPPER, "🎬", "clapper board", U_1F3AC, "1f3ac.png");
png_name!(PERFORMING_ARTS, "🎭", "performing arts", U_1F3AD, "1f3ad.png");
png_name!(CONTROLLER, "🎮", "video game", U_1F3AE, "1f3ae.png");
png_name!(VIDEO_GAME, "🎮", "video game", U_1F3AE, "1f3ae.png");
png_name!(BULLSEYE, "🎯", "bullseye", U_1F3AF, "1f3af.png");
png_name!(DART, "🎯", "bullseye", U_1F3AF, "1f3af.png");
png_name!(DIRECT_HIT, "🎯", "bullseye", U_1F3AF, "1f3af.png");
png_name!(SLOT_MACHINE, "🎰", "slot machine", U_1F3B0, "1f3b0.png");
png_name!(X_8BALL, "🎱", "pool 8 ball", U_1F3B1, "1f3b1.png");
png_name!(BILLIARDS, "🎱", "pool 8 ball", U_1F3B1, "1f3b1.png");
png_name!(GAME_DIE, "🎲", "game die", U_1F3B2, "1f3b2.png");
png_name!(BOWLING, "🎳", "bowling", U_1F3B3, "1f3b3.png");
png_name!(FLOWER_PLAYING_CARDS, "🎴", "flower playing cards", U_1F3B4, "1f3b4.png");
png_name!(MUSICAL_NOTE, "🎵", "musical note", U_1F3B5, "1f3b5.png");
png_name!(MUSICAL_NOTES, "🎶", "musical notes", U_1F3B6, "1f3b6.png");
png_name!(NOTES, "🎶", "musical notes", U_1F3B6, "1f3b6.png");
png_name!(SAXOPHONE, "🎷", "saxophone", U_1F3B7, "1f3b7.png");
png_name!(GUITAR, "🎸", "guitar", U_1F3B8, "1f3b8.png");
png_name!(MUSICAL_KEYBOARD, "🎹", "musical keyboard", U_1F3B9, "1f3b9.png");
png_name!(TRUMPET, "🎺", "trumpet", U_1F3BA, "1f3ba.png");
png_name!(VIOLIN, "🎻", "violin", U_1F3BB, "1f3bb.png");
png_name!(MUSICAL_SCORE, "🎼", "musical score", U_1F3BC, "1f3bc.png");
png_name!(RUNNING_SHIRT, "🎽", "running shirt", U_1F3BD, "1f3bd.png");
png_name!(RUNNING_SHIRT_WITH_SASH, "🎽", "running shirt", U_1F3BD, "1f3bd.png");
png_name!(TENNIS, "🎾", "tennis", U_1F3BE, "1f3be.png");
png_name!(SKI, "🎿", "skis", U_1F3BF, "1f3bf.png");
png_name!(BASKETBALL, "🏀", "basketball", U_1F3C0, "1f3c0.png");
png_name!(CHECKERED_FLAG, "🏁", "chequered flag", U_1F3C1, "1f3c1.png");
png_name!(PERSON_SNOWBOARDING_TONE1, "🏂🏻", "", U_1F3C2_1F3FB, "1f3c2-1f3fb.png");
png_name!(SNOWBOARDER_TONE1, "🏂🏻", "", U_1F3C2_1F3FB, "1f3c2-1f3fb.png");
png_name!(SNOWBOARDING_TONE1, "🏂🏻", "", U_1F3C2_1F3FB, "1f3c2-1f3fb.png");
png_name!(PERSON_SNOWBOARDING_TONE2, "🏂🏼", "", U_1F3C2_1F3FC, "1f3c2-1f3fc.png");
png_name!(SNOWBOARDER_TONE2, "🏂🏼", "", U_1F3C2_1F3FC, "1f3c2-1f3fc.png");
png_name!(SNOWBOARDING_TONE2, "🏂🏼", "", U_1F3C2_1F3FC, "1f3c2-1f3fc.png");
png_name!(PERSON_SNOWBOARDING_TONE3, "🏂🏽", "", U_1F3C2_1F3FD, "1f3c2-1f3fd.png");
png_name!(SNOWBOARDER_TONE3, "🏂🏽", "", U_1F3C2_1F3FD, "1f3c2-1f3fd.png");
png_name!(SNOWBOARDING_TONE3, "🏂🏽", "", U_1F3C2_1F3FD, "1f3c2-1f3fd.png");
png_name!(PERSON_SNOWBOARDING_TONE4, "🏂🏾", "", U_1F3C2_1F3FE, "1f3c2-1f3fe.png");
png_name!(SNOWBOARDER_TONE4, "🏂🏾", "", U_1F3C2_1F3FE, "1f3c2-1f3fe.png");
png_name!(SNOWBOARDING_TONE4, "🏂🏾", "", U_1F3C2_1F3FE, "1f3c2-1f3fe.png");
png_name!(PERSON_SNOWBOARDING_TONE5, "🏂🏿", "", U_1F3C2_1F3FF, "1f3c2-1f3ff.png");
png_name!(SNOWBOARDER_TONE5, "🏂🏿", "", U_1F3C2_1F3FF, "1f3c2-1f3ff.png");
png_name!(SNOWBOARDING_TONE5, "🏂🏿", "", U_1F3C2_1F3FF, "1f3c2-1f3ff.png");
png_name!(PERSON_SNOWBOARDING, "🏂", "snowboarder", U_1F3C2, "1f3c2.png");
png_name!(SNOWBOARDER, "🏂", "snowboarder", U_1F3C2, "1f3c2.png");
png_name!(SNOWBOARDING, "🏂", "snowboarder", U_1F3C2, "1f3c2.png");
png_name!(WOMAN_RUNNING_TONE1, "🏃🏻‍♀️", "", U_1F3C3_1F3FB_200D_2640_FE0F, "1f3c3-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_RUNNING_TONE1, "🏃🏻‍♂️", "", U_1F3C3_1F3FB_200D_2642_FE0F, "1f3c3-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_RUNNING_TONE1, "🏃🏻", "", U_1F3C3_1F3FB, "1f3c3-1f3fb.png");
png_name!(RUNNING_TONE1, "🏃🏻", "", U_1F3C3_1F3FB, "1f3c3-1f3fb.png");
png_name!(WOMAN_RUNNING_TONE2, "🏃🏼‍♀️", "", U_1F3C3_1F3FC_200D_2640_FE0F, "1f3c3-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_RUNNING_TONE2, "🏃🏼‍♂️", "", U_1F3C3_1F3FC_200D_2642_FE0F, "1f3c3-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_RUNNING_TONE2, "🏃🏼", "", U_1F3C3_1F3FC, "1f3c3-1f3fc.png");
png_name!(RUNNING_TONE2, "🏃🏼", "", U_1F3C3_1F3FC, "1f3c3-1f3fc.png");
png_name!(WOMAN_RUNNING_TONE3, "🏃🏽‍♀️", "", U_1F3C3_1F3FD_200D_2640_FE0F, "1f3c3-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_RUNNING_TONE3, "🏃🏽‍♂️", "", U_1F3C3_1F3FD_200D_2642_FE0F, "1f3c3-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_RUNNING_TONE3, "🏃🏽", "", U_1F3C3_1F3FD, "1f3c3-1f3fd.png");
png_name!(RUNNING_TONE3, "🏃🏽", "", U_1F3C3_1F3FD, "1f3c3-1f3fd.png");
png_name!(WOMAN_RUNNING_TONE4, "🏃🏾‍♀️", "", U_1F3C3_1F3FE_200D_2640_FE0F, "1f3c3-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_RUNNING_TONE4, "🏃🏾‍♂️", "", U_1F3C3_1F3FE_200D_2642_FE0F, "1f3c3-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_RUNNING_TONE4, "🏃🏾", "", U_1F3C3_1F3FE, "1f3c3-1f3fe.png");
png_name!(RUNNING_TONE4, "🏃🏾", "", U_1F3C3_1F3FE, "1f3c3-1f3fe.png");
png_name!(WOMAN_RUNNING_TONE5, "🏃🏿‍♀️", "", U_1F3C3_1F3FF_200D_2640_FE0F, "1f3c3-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_RUNNING_TONE5, "🏃🏿‍♂️", "", U_1F3C3_1F3FF_200D_2642_FE0F, "1f3c3-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_RUNNING_TONE5, "🏃🏿", "", U_1F3C3_1F3FF, "1f3c3-1f3ff.png");
png_name!(RUNNING_TONE5, "🏃🏿", "", U_1F3C3_1F3FF, "1f3c3-1f3ff.png");
png_name!(WOMAN_RUNNING, "🏃‍♀️", "woman running", U_1F3C3_200D_2640_FE0F, "1f3c3-200d-2640-fe0f.png");
png_name!(MAN_RUNNING, "🏃‍♂️", "man running", U_1F3C3_200D_2642_FE0F, "1f3c3-200d-2642-fe0f.png");
png_name!(PERSON_RUNNING, "🏃", "person running", U_1F3C3, "1f3c3.png");
png_name!(RUNNING, "🏃", "person running", U_1F3C3, "1f3c3.png");
png_name!(WOMAN_SURFING_TONE1, "🏄🏻‍♀️", "", U_1F3C4_1F3FB_200D_2640_FE0F, "1f3c4-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_SURFING_TONE1, "🏄🏻‍♂️", "", U_1F3C4_1F3FB_200D_2642_FE0F, "1f3c4-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_SURFING_TONE1, "🏄🏻", "", U_1F3C4_1F3FB, "1f3c4-1f3fb.png");
png_name!(SURFER_TONE1, "🏄🏻", "", U_1F3C4_1F3FB, "1f3c4-1f3fb.png");
png_name!(SURFING_TONE1, "🏄🏻", "", U_1F3C4_1F3FB, "1f3c4-1f3fb.png");
png_name!(WOMAN_SURFING_TONE2, "🏄🏼‍♀️", "", U_1F3C4_1F3FC_200D_2640_FE0F, "1f3c4-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_SURFING_TONE2, "🏄🏼‍♂️", "", U_1F3C4_1F3FC_200D_2642_FE0F, "1f3c4-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_SURFING_TONE2, "🏄🏼", "", U_1F3C4_1F3FC, "1f3c4-1f3fc.png");
png_name!(SURFER_TONE2, "🏄🏼", "", U_1F3C4_1F3FC, "1f3c4-1f3fc.png");
png_name!(SURFING_TONE2, "🏄🏼", "", U_1F3C4_1F3FC, "1f3c4-1f3fc.png");
png_name!(WOMAN_SURFING_TONE3, "🏄🏽‍♀️", "", U_1F3C4_1F3FD_200D_2640_FE0F, "1f3c4-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_SURFING_TONE3, "🏄🏽‍♂️", "", U_1F3C4_1F3FD_200D_2642_FE0F, "1f3c4-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_SURFING_TONE3, "🏄🏽", "", U_1F3C4_1F3FD, "1f3c4-1f3fd.png");
png_name!(SURFER_TONE3, "🏄🏽", "", U_1F3C4_1F3FD, "1f3c4-1f3fd.png");
png_name!(SURFING_TONE3, "🏄🏽", "", U_1F3C4_1F3FD, "1f3c4-1f3fd.png");
png_name!(WOMAN_SURFING_TONE4, "🏄🏾‍♀️", "", U_1F3C4_1F3FE_200D_2640_FE0F, "1f3c4-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_SURFING_TONE4, "🏄🏾‍♂️", "", U_1F3C4_1F3FE_200D_2642_FE0F, "1f3c4-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_SURFING_TONE4, "🏄🏾", "", U_1F3C4_1F3FE, "1f3c4-1f3fe.png");
png_name!(SURFER_TONE4, "🏄🏾", "", U_1F3C4_1F3FE, "1f3c4-1f3fe.png");
png_name!(SURFING_TONE4, "🏄🏾", "", U_1F3C4_1F3FE, "1f3c4-1f3fe.png");
png_name!(WOMAN_SURFING_TONE5, "🏄🏿‍♀️", "", U_1F3C4_1F3FF_200D_2640_FE0F, "1f3c4-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_SURFING_TONE5, "🏄🏿‍♂️", "", U_1F3C4_1F3FF_200D_2642_FE0F, "1f3c4-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_SURFING_TONE5, "🏄🏿", "", U_1F3C4_1F3FF, "1f3c4-1f3ff.png");
png_name!(SURFER_TONE5, "🏄🏿", "", U_1F3C4_1F3FF, "1f3c4-1f3ff.png");
png_name!(SURFING_TONE5, "🏄🏿", "", U_1F3C4_1F3FF, "1f3c4-1f3ff.png");
png_name!(WOMAN_SURFING, "🏄‍♀️", "woman surfing", U_1F3C4_200D_2640_FE0F, "1f3c4-200d-2640-fe0f.png");
png_name!(MAN_SURFING, "🏄‍♂️", "man surfing", U_1F3C4_200D_2642_FE0F, "1f3c4-200d-2642-fe0f.png");
png_name!(PERSON_SURFING, "🏄", "person surfing", U_1F3C4, "1f3c4.png");
png_name!(SURFER, "🏄", "person surfing", U_1F3C4, "1f3c4.png");
png_name!(SURFING, "🏄", "person surfing", U_1F3C4, "1f3c4.png");
png_name!(SPORTS_MEDAL, "🏅", "sports medal", U_1F3C5, "1f3c5.png");
png_name!(TROPHY, "🏆", "trophy", U_1F3C6, "1f3c6.png");
png_name!(HORSE_RACING_TONE1, "🏇🏻", "", U_1F3C7_1F3FB, "1f3c7-1f3fb.png");
png_name!(HORSE_RACING_TONE2, "🏇🏼", "", U_1F3C7_1F3FC, "1f3c7-1f3fc.png");
png_name!(HORSE_RACING_TONE3, "🏇🏽", "", U_1F3C7_1F3FD, "1f3c7-1f3fd.png");
png_name!(HORSE_RACING_TONE4, "🏇🏾", "", U_1F3C7_1F3FE, "1f3c7-1f3fe.png");
png_name!(HORSE_RACING_TONE5, "🏇🏿", "", U_1F3C7_1F3FF, "1f3c7-1f3ff.png");
png_name!(HORSE_RACING, "🏇", "horse racing", U_1F3C7, "1f3c7.png");
png_name!(FOOTBALL, "🏈", "american football", U_1F3C8, "1f3c8.png");
png_name!(RUGBY_FOOTBALL, "🏉", "rugby football", U_1F3C9, "1f3c9.png");
png_name!(WOMAN_SWIMMING_TONE1, "🏊🏻‍♀️", "", U_1F3CA_1F3FB_200D_2640_FE0F, "1f3ca-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_SWIMMING_TONE1, "🏊🏻‍♂️", "", U_1F3CA_1F3FB_200D_2642_FE0F, "1f3ca-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_SWIMMING_TONE1, "🏊🏻", "", U_1F3CA_1F3FB, "1f3ca-1f3fb.png");
png_name!(SWIMMER_TONE1, "🏊🏻", "", U_1F3CA_1F3FB, "1f3ca-1f3fb.png");
png_name!(SWIMMING_TONE1, "🏊🏻", "", U_1F3CA_1F3FB, "1f3ca-1f3fb.png");
png_name!(WOMAN_SWIMMING_TONE2, "🏊🏼‍♀️", "", U_1F3CA_1F3FC_200D_2640_FE0F, "1f3ca-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_SWIMMING_TONE2, "🏊🏼‍♂️", "", U_1F3CA_1F3FC_200D_2642_FE0F, "1f3ca-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_SWIMMING_TONE2, "🏊🏼", "", U_1F3CA_1F3FC, "1f3ca-1f3fc.png");
png_name!(SWIMMER_TONE2, "🏊🏼", "", U_1F3CA_1F3FC, "1f3ca-1f3fc.png");
png_name!(SWIMMING_TONE2, "🏊🏼", "", U_1F3CA_1F3FC, "1f3ca-1f3fc.png");
png_name!(WOMAN_SWIMMING_TONE3, "🏊🏽‍♀️", "", U_1F3CA_1F3FD_200D_2640_FE0F, "1f3ca-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_SWIMMING_TONE3, "🏊🏽‍♂️", "", U_1F3CA_1F3FD_200D_2642_FE0F, "1f3ca-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_SWIMMING_TONE3, "🏊🏽", "", U_1F3CA_1F3FD, "1f3ca-1f3fd.png");
png_name!(SWIMMER_TONE3, "🏊🏽", "", U_1F3CA_1F3FD, "1f3ca-1f3fd.png");
png_name!(SWIMMING_TONE3, "🏊🏽", "", U_1F3CA_1F3FD, "1f3ca-1f3fd.png");
png_name!(WOMAN_SWIMMING_TONE4, "🏊🏾‍♀️", "", U_1F3CA_1F3FE_200D_2640_FE0F, "1f3ca-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_SWIMMING_TONE4, "🏊🏾‍♂️", "", U_1F3CA_1F3FE_200D_2642_FE0F, "1f3ca-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_SWIMMING_TONE4, "🏊🏾", "", U_1F3CA_1F3FE, "1f3ca-1f3fe.png");
png_name!(SWIMMER_TONE4, "🏊🏾", "", U_1F3CA_1F3FE, "1f3ca-1f3fe.png");
png_name!(SWIMMING_TONE4, "🏊🏾", "", U_1F3CA_1F3FE, "1f3ca-1f3fe.png");
png_name!(WOMAN_SWIMMING_TONE5, "🏊🏿‍♀️", "", U_1F3CA_1F3FF_200D_2640_FE0F, "1f3ca-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_SWIMMING_TONE5, "🏊🏿‍♂️", "", U_1F3CA_1F3FF_200D_2642_FE0F, "1f3ca-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_SWIMMING_TONE5, "🏊🏿", "", U_1F3CA_1F3FF, "1f3ca-1f3ff.png");
png_name!(SWIMMER_TONE5, "🏊🏿", "", U_1F3CA_1F3FF, "1f3ca-1f3ff.png");
png_name!(SWIMMING_TONE5, "🏊🏿", "", U_1F3CA_1F3FF, "1f3ca-1f3ff.png");
png_name!(WOMAN_SWIMMING, "🏊‍♀️", "woman swimming", U_1F3CA_200D_2640_FE0F, "1f3ca-200d-2640-fe0f.png");
png_name!(MAN_SWIMMING, "🏊‍♂️", "man swimming", U_1F3CA_200D_2642_FE0F, "1f3ca-200d-2642-fe0f.png");
png_name!(PERSON_SWIMMING, "🏊", "person swimming", U_1F3CA, "1f3ca.png");
png_name!(SWIMMER, "🏊", "person swimming", U_1F3CA, "1f3ca.png");
png_name!(SWIMMING, "🏊", "person swimming", U_1F3CA, "1f3ca.png");
png_name!(WOMAN_LIFTING_WEIGHTS_TONE1, "🏋🏻‍♀️", "", U_1F3CB_1F3FB_200D_2640_FE0F, "1f3cb-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_LIFTING_WEIGHTS_TONE1, "🏋🏻‍♂️", "", U_1F3CB_1F3FB_200D_2642_FE0F, "1f3cb-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_LIFTING_WEIGHTS_TONE1, "🏋🏻", "", U_1F3CB_1F3FB, "1f3cb-1f3fb.png");
png_name!(WEIGHT_LIFTER_TONE1, "🏋🏻", "", U_1F3CB_1F3FB, "1f3cb-1f3fb.png");
png_name!(WEIGHT_LIFTING_TONE1, "🏋🏻", "", U_1F3CB_1F3FB, "1f3cb-1f3fb.png");
png_name!(WOMAN_LIFTING_WEIGHTS_TONE2, "🏋🏼‍♀️", "", U_1F3CB_1F3FC_200D_2640_FE0F, "1f3cb-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_LIFTING_WEIGHTS_TONE2, "🏋🏼‍♂️", "", U_1F3CB_1F3FC_200D_2642_FE0F, "1f3cb-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_LIFTING_WEIGHTS_TONE2, "🏋🏼", "", U_1F3CB_1F3FC, "1f3cb-1f3fc.png");
png_name!(WEIGHT_LIFTER_TONE2, "🏋🏼", "", U_1F3CB_1F3FC, "1f3cb-1f3fc.png");
png_name!(WEIGHT_LIFTING_TONE2, "🏋🏼", "", U_1F3CB_1F3FC, "1f3cb-1f3fc.png");
png_name!(WOMAN_LIFTING_WEIGHTS_TONE3, "🏋🏽‍♀️", "", U_1F3CB_1F3FD_200D_2640_FE0F, "1f3cb-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_LIFTING_WEIGHTS_TONE3, "🏋🏽‍♂️", "", U_1F3CB_1F3FD_200D_2642_FE0F, "1f3cb-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_LIFTING_WEIGHTS_TONE3, "🏋🏽", "", U_1F3CB_1F3FD, "1f3cb-1f3fd.png");
png_name!(WEIGHT_LIFTER_TONE3, "🏋🏽", "", U_1F3CB_1F3FD, "1f3cb-1f3fd.png");
png_name!(WEIGHT_LIFTING_TONE3, "🏋🏽", "", U_1F3CB_1F3FD, "1f3cb-1f3fd.png");
png_name!(WOMAN_LIFTING_WEIGHTS_TONE4, "🏋🏾‍♀️", "", U_1F3CB_1F3FE_200D_2640_FE0F, "1f3cb-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_LIFTING_WEIGHTS_TONE4, "🏋🏾‍♂️", "", U_1F3CB_1F3FE_200D_2642_FE0F, "1f3cb-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_LIFTING_WEIGHTS_TONE4, "🏋🏾", "", U_1F3CB_1F3FE, "1f3cb-1f3fe.png");
png_name!(WEIGHT_LIFTER_TONE4, "🏋🏾", "", U_1F3CB_1F3FE, "1f3cb-1f3fe.png");
png_name!(WEIGHT_LIFTING_TONE4, "🏋🏾", "", U_1F3CB_1F3FE, "1f3cb-1f3fe.png");
png_name!(WOMAN_LIFTING_WEIGHTS_TONE5, "🏋🏿‍♀️", "", U_1F3CB_1F3FF_200D_2640_FE0F, "1f3cb-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_LIFTING_WEIGHTS_TONE5, "🏋🏿‍♂️", "", U_1F3CB_1F3FF_200D_2642_FE0F, "1f3cb-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_LIFTING_WEIGHTS_TONE5, "🏋🏿", "", U_1F3CB_1F3FF, "1f3cb-1f3ff.png");
png_name!(WEIGHT_LIFTER_TONE5, "🏋🏿", "", U_1F3CB_1F3FF, "1f3cb-1f3ff.png");
png_name!(WEIGHT_LIFTING_TONE5, "🏋🏿", "", U_1F3CB_1F3FF, "1f3cb-1f3ff.png");
png_name!(WOMAN_LIFTING_WEIGHTS, "🏋️‍♀️", "woman lifting weights", U_1F3CB_FE0F_200D_2640_FE0F, "1f3cb-fe0f-200d-2640-fe0f.png");
png_name!(MAN_LIFTING_WEIGHTS, "🏋️‍♂️", "man lifting weights", U_1F3CB_FE0F_200D_2642_FE0F, "1f3cb-fe0f-200d-2642-fe0f.png");
png_name!(PERSON_LIFTING_WEIGHTS, "🏋", "person lifting weights", U_1F3CB, "1f3cb.png");
png_name!(WEIGHT_LIFTER, "🏋", "person lifting weights", U_1F3CB, "1f3cb.png");
png_name!(WEIGHT_LIFTING, "🏋", "person lifting weights", U_1F3CB, "1f3cb.png");
png_name!(WOMAN_GOLFING_TONE1, "🏌🏻‍♀️", "", U_1F3CC_1F3FB_200D_2640_FE0F, "1f3cc-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_GOLFING_TONE1, "🏌🏻‍♂️", "", U_1F3CC_1F3FB_200D_2642_FE0F, "1f3cc-1f3fb-200d-2642-fe0f.png");
png_name!(GOLFER_TONE1, "🏌🏻", "", U_1F3CC_1F3FB, "1f3cc-1f3fb.png");
png_name!(GOLFING_TONE1, "🏌🏻", "", U_1F3CC_1F3FB, "1f3cc-1f3fb.png");
png_name!(PERSON_GOLFING_TONE1, "🏌🏻", "", U_1F3CC_1F3FB, "1f3cc-1f3fb.png");
png_name!(WOMAN_GOLFING_TONE2, "🏌🏼‍♀️", "", U_1F3CC_1F3FC_200D_2640_FE0F, "1f3cc-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_GOLFING_TONE2, "🏌🏼‍♂️", "", U_1F3CC_1F3FC_200D_2642_FE0F, "1f3cc-1f3fc-200d-2642-fe0f.png");
png_name!(GOLFER_TONE2, "🏌🏼", "", U_1F3CC_1F3FC, "1f3cc-1f3fc.png");
png_name!(GOLFING_TONE2, "🏌🏼", "", U_1F3CC_1F3FC, "1f3cc-1f3fc.png");
png_name!(PERSON_GOLFING_TONE2, "🏌🏼", "", U_1F3CC_1F3FC, "1f3cc-1f3fc.png");
png_name!(WOMAN_GOLFING_TONE3, "🏌🏽‍♀️", "", U_1F3CC_1F3FD_200D_2640_FE0F, "1f3cc-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_GOLFING_TONE3, "🏌🏽‍♂️", "", U_1F3CC_1F3FD_200D_2642_FE0F, "1f3cc-1f3fd-200d-2642-fe0f.png");
png_name!(GOLFER_TONE3, "🏌🏽", "", U_1F3CC_1F3FD, "1f3cc-1f3fd.png");
png_name!(GOLFING_TONE3, "🏌🏽", "", U_1F3CC_1F3FD, "1f3cc-1f3fd.png");
png_name!(PERSON_GOLFING_TONE3, "🏌🏽", "", U_1F3CC_1F3FD, "1f3cc-1f3fd.png");
png_name!(WOMAN_GOLFING_TONE4, "🏌🏾‍♀️", "", U_1F3CC_1F3FE_200D_2640_FE0F, "1f3cc-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_GOLFING_TONE4, "🏌🏾‍♂️", "", U_1F3CC_1F3FE_200D_2642_FE0F, "1f3cc-1f3fe-200d-2642-fe0f.png");
png_name!(GOLFER_TONE4, "🏌🏾", "", U_1F3CC_1F3FE, "1f3cc-1f3fe.png");
png_name!(GOLFING_TONE4, "🏌🏾", "", U_1F3CC_1F3FE, "1f3cc-1f3fe.png");
png_name!(PERSON_GOLFING_TONE4, "🏌🏾", "", U_1F3CC_1F3FE, "1f3cc-1f3fe.png");
png_name!(WOMAN_GOLFING_TONE5, "🏌🏿‍♀️", "", U_1F3CC_1F3FF_200D_2640_FE0F, "1f3cc-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_GOLFING_TONE5, "🏌🏿‍♂️", "", U_1F3CC_1F3FF_200D_2642_FE0F, "1f3cc-1f3ff-200d-2642-fe0f.png");
png_name!(GOLFER_TONE5, "🏌🏿", "", U_1F3CC_1F3FF, "1f3cc-1f3ff.png");
png_name!(GOLFING_TONE5, "🏌🏿", "", U_1F3CC_1F3FF, "1f3cc-1f3ff.png");
png_name!(PERSON_GOLFING_TONE5, "🏌🏿", "", U_1F3CC_1F3FF, "1f3cc-1f3ff.png");
png_name!(WOMAN_GOLFING, "🏌️‍♀️", "woman golfing", U_1F3CC_FE0F_200D_2640_FE0F, "1f3cc-fe0f-200d-2640-fe0f.png");
png_name!(MAN_GOLFING, "🏌️‍♂️", "man golfing", U_1F3CC_FE0F_200D_2642_FE0F, "1f3cc-fe0f-200d-2642-fe0f.png");
png_name!(GOLFER, "🏌", "person golfing", U_1F3CC, "1f3cc.png");
png_name!(GOLFING, "🏌", "person golfing", U_1F3CC, "1f3cc.png");
png_name!(PERSON_GOLFING, "🏌", "person golfing", U_1F3CC, "1f3cc.png");
png_name!(MOTORCYCLE, "🏍", "motorcycle", U_1F3CD, "1f3cd.png");
png_name!(RACING_CAR, "🏎", "racing car", U_1F3CE, "1f3ce.png");
png_name!(CRICKET_GAME, "🏏", "cricket game", U_1F3CF, "1f3cf.png");
png_name!(VOLLEYBALL, "🏐", "volleyball", U_1F3D0, "1f3d0.png");
png_name!(FIELD_HOCKEY, "🏑", "field hockey", U_1F3D1, "1f3d1.png");
png_name!(HOCKEY, "🏒", "ice hockey", U_1F3D2, "1f3d2.png");
png_name!(PING_PONG, "🏓", "ping pong", U_1F3D3, "1f3d3.png");
png_name!(MOUNTAIN_SNOW, "🏔", "snow-capped mountain", U_1F3D4, "1f3d4.png");
png_name!(CAMPING, "🏕", "camping", U_1F3D5, "1f3d5.png");
png_name!(BEACH, "🏖", "beach with umbrella", U_1F3D6, "1f3d6.png");
png_name!(BEACH_WITH_UMBRELLA, "🏖", "beach with umbrella", U_1F3D6, "1f3d6.png");
png_name!(BUILDING_CONSTRUCTION, "🏗", "building construction", U_1F3D7, "1f3d7.png");
png_name!(CONSTRUCTION_SITE, "🏗", "building construction", U_1F3D7, "1f3d7.png");
png_name!(HOMES, "🏘", "houses", U_1F3D8, "1f3d8.png");
png_name!(HOUSES, "🏘", "houses", U_1F3D8, "1f3d8.png");
png_name!(CITYSCAPE, "🏙", "cityscape", U_1F3D9, "1f3d9.png");
png_name!(DERELICT_HOUSE, "🏚", "derelict house", U_1F3DA, "1f3da.png");
png_name!(HOUSE_ABANDONED, "🏚", "derelict house", U_1F3DA, "1f3da.png");
png_name!(CLASSICAL_BUILDING, "🏛", "classical building", U_1F3DB, "1f3db.png");
png_name!(DESERT, "🏜", "desert", U_1F3DC, "1f3dc.png");
png_name!(DESERT_ISLAND, "🏝", "desert island", U_1F3DD, "1f3dd.png");
png_name!(ISLAND, "🏝", "desert island", U_1F3DD, "1f3dd.png");
png_name!(NATIONAL_PARK, "🏞", "national park", U_1F3DE, "1f3de.png");
png_name!(STADIUM, "🏟", "stadium", U_1F3DF, "1f3df.png");
png_name!(HOUSE, "🏠", "house", U_1F3E0, "1f3e0.png");
png_name!(HOUSE_WITH_GARDEN, "🏡", "house with garden", U_1F3E1, "1f3e1.png");
png_name!(OFFICE, "🏢", "office building", U_1F3E2, "1f3e2.png");
png_name!(POST_OFFICE, "🏣", "Japanese post office", U_1F3E3, "1f3e3.png");
png_name!(EUROPEAN_POST_OFFICE, "🏤", "post office", U_1F3E4, "1f3e4.png");
png_name!(HOSPITAL, "🏥", "hospital", U_1F3E5, "1f3e5.png");
png_name!(BANK, "🏦", "bank", U_1F3E6, "1f3e6.png");
png_name!(ATM, "🏧", "ATM sign", U_1F3E7, "1f3e7.png");
png_name!(HOTEL, "🏨", "hotel", U_1F3E8, "1f3e8.png");
png_name!(LOVE_HOTEL, "🏩", "love hotel", U_1F3E9, "1f3e9.png");
png_name!(CONVENIENCE_STORE, "🏪", "convenience store", U_1F3EA, "1f3ea.png");
png_name!(SCHOOL, "🏫", "school", U_1F3EB, "1f3eb.png");
png_name!(DEPARTMENT_STORE, "🏬", "department store", U_1F3EC, "1f3ec.png");
png_name!(FACTORY, "🏭", "factory", U_1F3ED, "1f3ed.png");
png_name!(IZAKAYA_LANTERN, "🏮", "red paper lantern", U_1F3EE, "1f3ee.png");
png_name!(RED_PAPER_LANTERN, "🏮", "red paper lantern", U_1F3EE, "1f3ee.png");
png_name!(JAPANESE_CASTLE, "🏯", "Japanese castle", U_1F3EF, "1f3ef.png");
png_name!(CASTLE, "🏰", "castle", U_1F3F0, "1f3f0.png");
png_name!(EUROPEAN_CASTLE, "🏰", "castle", U_1F3F0, "1f3f0.png");
png_name!(RAINBOW_FLAG, "🏳️‍🌈", "rainbow flag", U_1F3F3_FE0F_200D_1F308, "1f3f3-fe0f-200d-1f308.png");
png_name!(TRANSGENDER_FLAG, "🏳️‍⚧️", "transgender flag", U_1F3F3_FE0F_200D_26A7_FE0F, "1f3f3-fe0f-200d-26a7-fe0f.png");
png_name!(WHITE_FLAG, "🏳", "white flag", U_1F3F3, "1f3f3.png");
png_name!(JOLLY_ROGER, "🏴‍☠️", "pirate flag", U_1F3F4_200D_2620_FE0F, "1f3f4-200d-2620-fe0f.png");
png_name!(PIRATE_FLAG, "🏴‍☠️", "pirate flag", U_1F3F4_200D_2620_FE0F, "1f3f4-200d-2620-fe0f.png");
png_name!(ENGLAND, "🏴󠁧󠁢󠁥󠁮󠁧󠁿", "flag: England", U_1F3F4_E0067_E0062_E0065_E006E_E0067_E007F, "1f3f4-e0067-e0062-e0065-e006e-e0067-e007f.png");
png_name!(FLAG_GBENG, "🏴󠁧󠁢󠁥󠁮󠁧󠁿", "flag: England", U_1F3F4_E0067_E0062_E0065_E006E_E0067_E007F, "1f3f4-e0067-e0062-e0065-e006e-e0067-e007f.png");
png_name!(FLAG_GBSCT, "🏴󠁧󠁢󠁳󠁣󠁴󠁿", "flag: Scotland", U_1F3F4_E0067_E0062_E0073_E0063_E0074_E007F, "1f3f4-e0067-e0062-e0073-e0063-e0074-e007f.png");
png_name!(SCOTLAND, "🏴󠁧󠁢󠁳󠁣󠁴󠁿", "flag: Scotland", U_1F3F4_E0067_E0062_E0073_E0063_E0074_E007F, "1f3f4-e0067-e0062-e0073-e0063-e0074-e007f.png");
png_name!(FLAG_GBWLS, "🏴󠁧󠁢󠁷󠁬󠁳󠁿", "flag: Wales", U_1F3F4_E0067_E0062_E0077_E006C_E0073_E007F, "1f3f4-e0067-e0062-e0077-e006c-e0073-e007f.png");
png_name!(WALES, "🏴󠁧󠁢󠁷󠁬󠁳󠁿", "flag: Wales", U_1F3F4_E0067_E0062_E0077_E006C_E0073_E007F, "1f3f4-e0067-e0062-e0077-e006c-e0073-e007f.png");
png_name!(BLACK_FLAG, "🏴", "black flag", U_1F3F4, "1f3f4.png");
png_name!(ROSETTE, "🏵", "rosette", U_1F3F5, "1f3f5.png");
png_name!(LABEL, "🏷", "label", U_1F3F7, "1f3f7.png");
png_name!(BADMINTON, "🏸", "badminton", U_1F3F8, "1f3f8.png");
png_name!(BOW_AND_ARROW, "🏹", "bow and arrow", U_1F3F9, "1f3f9.png");
png_name!(AMPHORA, "🏺", "amphora", U_1F3FA, "1f3fa.png");
png_name!(TONE1, "🏻", "light skin tone", U_1F3FB, "1f3fb.png");
png_name!(TONE_LIGHT, "🏻", "light skin tone", U_1F3FB, "1f3fb.png");
png_name!(TONE2, "🏼", "medium-light skin tone", U_1F3FC, "1f3fc.png");
png_name!(TONE_MEDIUM_LIGHT, "🏼", "medium-light skin tone", U_1F3FC, "1f3fc.png");
png_name!(TONE3, "🏽", "medium skin tone", U_1F3FD, "1f3fd.png");
png_name!(TONE_MEDIUM, "🏽", "medium skin tone", U_1F3FD, "1f3fd.png");
png_name!(TONE4, "🏾", "medium-dark skin tone", U_1F3FE, "1f3fe.png");
png_name!(TONE_MEDIUM_DARK, "🏾", "medium-dark skin tone", U_1F3FE, "1f3fe.png");
png_name!(TONE5, "🏿", "dark skin tone", U_1F3FF, "1f3ff.png");
png_name!(TONE_DARK, "🏿", "dark skin tone", U_1F3FF, "1f3ff.png");
png_name!(RAT, "🐀", "rat", U_1F400, "1f400.png");
png_name!(MOUSE, "🐁", "mouse", U_1F401, "1f401.png");
png_name!(OX, "🐂", "ox", U_1F402, "1f402.png");
png_name!(WATER_BUFFALO, "🐃", "water buffalo", U_1F403, "1f403.png");
png_name!(COW, "🐄", "cow", U_1F404, "1f404.png");
png_name!(TIGER, "🐅", "tiger", U_1F405, "1f405.png");
png_name!(LEOPARD, "🐆", "leopard", U_1F406, "1f406.png");
png_name!(RABBIT, "🐇", "rabbit", U_1F407, "1f407.png");
png_name!(BLACK_CAT, "🐈‍⬛", "black cat", U_1F408_200D_2B1B, "1f408-200d-2b1b.png");
png_name!(CAT, "🐈", "cat", U_1F408, "1f408.png");
png_name!(DRAGON, "🐉", "dragon", U_1F409, "1f409.png");
png_name!(CROCODILE, "🐊", "crocodile", U_1F40A, "1f40a.png");
png_name!(WHALE, "🐋", "whale", U_1F40B, "1f40b.png");
png_name!(SNAIL, "🐌", "snail", U_1F40C, "1f40c.png");
png_name!(SNAKE, "🐍", "snake", U_1F40D, "1f40d.png");
png_name!(HORSE, "🐎", "horse", U_1F40E, "1f40e.png");
png_name!(RACEHORSE, "🐎", "horse", U_1F40E, "1f40e.png");
png_name!(RAM, "🐏", "ram", U_1F40F, "1f40f.png");
png_name!(GOAT, "🐐", "goat", U_1F410, "1f410.png");
png_name!(EWE, "🐑", "ewe", U_1F411, "1f411.png");
png_name!(SHEEP, "🐑", "ewe", U_1F411, "1f411.png");
png_name!(MONKEY, "🐒", "monkey", U_1F412, "1f412.png");
png_name!(ROOSTER, "🐓", "rooster", U_1F413, "1f413.png");
png_name!(CHICKEN, "🐔", "chicken", U_1F414, "1f414.png");
png_name!(CHICKEN_FACE, "🐔", "chicken", U_1F414, "1f414.png");
png_name!(SERVICE_DOG, "🐕‍🦺", "service dog", U_1F415_200D_1F9BA, "1f415-200d-1f9ba.png");
png_name!(DOG, "🐕", "dog", U_1F415, "1f415.png");
png_name!(PIG, "🐖", "pig", U_1F416, "1f416.png");
png_name!(BOAR, "🐗", "boar", U_1F417, "1f417.png");
png_name!(ELEPHANT, "🐘", "elephant", U_1F418, "1f418.png");
png_name!(OCTOPUS, "🐙", "octopus", U_1F419, "1f419.png");
png_name!(SHELL, "🐚", "spiral shell", U_1F41A, "1f41a.png");
png_name!(BUG, "🐛", "bug", U_1F41B, "1f41b.png");
png_name!(ANT, "🐜", "ant", U_1F41C, "1f41c.png");
png_name!(BEE, "🐝", "honeybee", U_1F41D, "1f41d.png");
png_name!(LADY_BEETLE, "🐞", "lady beetle", U_1F41E, "1f41e.png");
png_name!(FISH, "🐟", "fish", U_1F41F, "1f41f.png");
png_name!(TROPICAL_FISH, "🐠", "tropical fish", U_1F420, "1f420.png");
png_name!(BLOWFISH, "🐡", "blowfish", U_1F421, "1f421.png");
png_name!(TURTLE, "🐢", "turtle", U_1F422, "1f422.png");
png_name!(HATCHING_CHICK, "🐣", "hatching chick", U_1F423, "1f423.png");
png_name!(BABY_CHICK, "🐤", "baby chick", U_1F424, "1f424.png");
png_name!(HATCHED_CHICK, "🐥", "front-facing baby chick", U_1F425, "1f425.png");
png_name!(BIRD, "🐦", "bird", U_1F426, "1f426.png");
png_name!(BIRD_FACE, "🐦", "bird", U_1F426, "1f426.png");
png_name!(PENGUIN, "🐧", "penguin", U_1F427, "1f427.png");
png_name!(PENGUIN_FACE, "🐧", "penguin", U_1F427, "1f427.png");
png_name!(KOALA, "🐨", "koala", U_1F428, "1f428.png");
png_name!(KOALA_FACE, "🐨", "koala", U_1F428, "1f428.png");
png_name!(POODLE, "🐩", "poodle", U_1F429, "1f429.png");
png_name!(DROMEDARY_CAMEL, "🐪", "camel", U_1F42A, "1f42a.png");
png_name!(CAMEL, "🐫", "two-hump camel", U_1F42B, "1f42b.png");
png_name!(DOLPHIN, "🐬", "dolphin", U_1F42C, "1f42c.png");
png_name!(MOUSE_FACE, "🐭", "mouse face", U_1F42D, "1f42d.png");
png_name!(COW_FACE, "🐮", "cow face", U_1F42E, "1f42e.png");
png_name!(TIGER_FACE, "🐯", "tiger face", U_1F42F, "1f42f.png");
png_name!(RABBIT_FACE, "🐰", "rabbit face", U_1F430, "1f430.png");
png_name!(CAT_FACE, "🐱", "cat face", U_1F431, "1f431.png");
png_name!(DRAGON_FACE, "🐲", "dragon face", U_1F432, "1f432.png");
png_name!(SPOUTING_WHALE, "🐳", "spouting whale", U_1F433, "1f433.png");
png_name!(HORSE_FACE, "🐴", "horse face", U_1F434, "1f434.png");
png_name!(MONKEY_FACE, "🐵", "monkey face", U_1F435, "1f435.png");
png_name!(DOG_FACE, "🐶", "dog face", U_1F436, "1f436.png");
png_name!(PIG_FACE, "🐷", "pig face", U_1F437, "1f437.png");
png_name!(FROG, "🐸", "frog", U_1F438, "1f438.png");
png_name!(FROG_FACE, "🐸", "frog", U_1F438, "1f438.png");
png_name!(HAMSTER, "🐹", "hamster", U_1F439, "1f439.png");
png_name!(HAMSTER_FACE, "🐹", "hamster", U_1F439, "1f439.png");
png_name!(WOLF, "🐺", "wolf", U_1F43A, "1f43a.png");
png_name!(WOLF_FACE, "🐺", "wolf", U_1F43A, "1f43a.png");
png_name!(POLAR_BEAR, "🐻‍❄️", "polar bear", U_1F43B_200D_2744_FE0F, "1f43b-200d-2744-fe0f.png");
png_name!(POLAR_BEAR_FACE, "🐻‍❄️", "polar bear", U_1F43B_200D_2744_FE0F, "1f43b-200d-2744-fe0f.png");
png_name!(BEAR, "🐻", "bear", U_1F43B, "1f43b.png");
png_name!(BEAR_FACE, "🐻", "bear", U_1F43B, "1f43b.png");
png_name!(PANDA, "🐼", "panda", U_1F43C, "1f43c.png");
png_name!(PANDA_FACE, "🐼", "panda", U_1F43C, "1f43c.png");
png_name!(PIG_NOSE, "🐽", "pig nose", U_1F43D, "1f43d.png");
png_name!(PAW_PRINTS, "🐾", "paw prints", U_1F43E, "1f43e.png");
png_name!(CHIPMUNK, "🐿", "chipmunk", U_1F43F, "1f43f.png");
png_name!(EYES, "👀", "eyes", U_1F440, "1f440.png");
png_name!(EYE, "👁", "eye", U_1F441, "1f441.png");
png_name!(EAR_TONE1, "👂🏻", "", U_1F442_1F3FB, "1f442-1f3fb.png");
png_name!(EAR_TONE2, "👂🏼", "", U_1F442_1F3FC, "1f442-1f3fc.png");
png_name!(EAR_TONE3, "👂🏽", "", U_1F442_1F3FD, "1f442-1f3fd.png");
png_name!(EAR_TONE4, "👂🏾", "", U_1F442_1F3FE, "1f442-1f3fe.png");
png_name!(EAR_TONE5, "👂🏿", "", U_1F442_1F3FF, "1f442-1f3ff.png");
png_name!(EAR, "👂", "ear", U_1F442, "1f442.png");
png_name!(NOSE_TONE1, "👃🏻", "", U_1F443_1F3FB, "1f443-1f3fb.png");
png_name!(NOSE_TONE2, "👃🏼", "", U_1F443_1F3FC, "1f443-1f3fc.png");
png_name!(NOSE_TONE3, "👃🏽", "", U_1F443_1F3FD, "1f443-1f3fd.png");
png_name!(NOSE_TONE4, "👃🏾", "", U_1F443_1F3FE, "1f443-1f3fe.png");
png_name!(NOSE_TONE5, "👃🏿", "", U_1F443_1F3FF, "1f443-1f3ff.png");
png_name!(NOSE, "👃", "nose", U_1F443, "1f443.png");
png_name!(LIPS, "👄", "mouth", U_1F444, "1f444.png");
png_name!(MOUTH, "👄", "mouth", U_1F444, "1f444.png");
png_name!(TONGUE, "👅", "tongue", U_1F445, "1f445.png");
png_name!(POINT_UP_TONE1, "👆🏻", "", U_1F446_1F3FB, "1f446-1f3fb.png");
png_name!(POINT_UP_TONE2, "👆🏼", "", U_1F446_1F3FC, "1f446-1f3fc.png");
png_name!(POINT_UP_TONE3, "👆🏽", "", U_1F446_1F3FD, "1f446-1f3fd.png");
png_name!(POINT_UP_TONE4, "👆🏾", "", U_1F446_1F3FE, "1f446-1f3fe.png");
png_name!(POINT_UP_TONE5, "👆🏿", "", U_1F446_1F3FF, "1f446-1f3ff.png");
png_name!(POINT_UP, "👆", "backhand index pointing up", U_1F446, "1f446.png");
png_name!(POINT_DOWN_TONE1, "👇🏻", "", U_1F447_1F3FB, "1f447-1f3fb.png");
png_name!(POINT_DOWN_TONE2, "👇🏼", "", U_1F447_1F3FC, "1f447-1f3fc.png");
png_name!(POINT_DOWN_TONE3, "👇🏽", "", U_1F447_1F3FD, "1f447-1f3fd.png");
png_name!(POINT_DOWN_TONE4, "👇🏾", "", U_1F447_1F3FE, "1f447-1f3fe.png");
png_name!(POINT_DOWN_TONE5, "👇🏿", "", U_1F447_1F3FF, "1f447-1f3ff.png");
png_name!(POINT_DOWN, "👇", "backhand index pointing down", U_1F447, "1f447.png");
png_name!(POINT_LEFT_TONE1, "👈🏻", "", U_1F448_1F3FB, "1f448-1f3fb.png");
png_name!(POINT_LEFT_TONE2, "👈🏼", "", U_1F448_1F3FC, "1f448-1f3fc.png");
png_name!(POINT_LEFT_TONE3, "👈🏽", "", U_1F448_1F3FD, "1f448-1f3fd.png");
png_name!(POINT_LEFT_TONE4, "👈🏾", "", U_1F448_1F3FE, "1f448-1f3fe.png");
png_name!(POINT_LEFT_TONE5, "👈🏿", "", U_1F448_1F3FF, "1f448-1f3ff.png");
png_name!(POINT_LEFT, "👈", "backhand index pointing left", U_1F448, "1f448.png");
png_name!(POINT_RIGHT_TONE1, "👉🏻", "", U_1F449_1F3FB, "1f449-1f3fb.png");
png_name!(POINT_RIGHT_TONE2, "👉🏼", "", U_1F449_1F3FC, "1f449-1f3fc.png");
png_name!(POINT_RIGHT_TONE3, "👉🏽", "", U_1F449_1F3FD, "1f449-1f3fd.png");
png_name!(POINT_RIGHT_TONE4, "👉🏾", "", U_1F449_1F3FE, "1f449-1f3fe.png");
png_name!(POINT_RIGHT_TONE5, "👉🏿", "", U_1F449_1F3FF, "1f449-1f3ff.png");
png_name!(POINT_RIGHT, "👉", "backhand index pointing right", U_1F449, "1f449.png");
png_name!(PUNCH_TONE1, "👊🏻", "", U_1F44A_1F3FB, "1f44a-1f3fb.png");
png_name!(PUNCH_TONE2, "👊🏼", "", U_1F44A_1F3FC, "1f44a-1f3fc.png");
png_name!(PUNCH_TONE3, "👊🏽", "", U_1F44A_1F3FD, "1f44a-1f3fd.png");
png_name!(PUNCH_TONE4, "👊🏾", "", U_1F44A_1F3FE, "1f44a-1f3fe.png");
png_name!(PUNCH_TONE5, "👊🏿", "", U_1F44A_1F3FF, "1f44a-1f3ff.png");
png_name!(PUNCH, "👊", "oncoming fist", U_1F44A, "1f44a.png");
png_name!(WAVE_TONE1, "👋🏻", "", U_1F44B_1F3FB, "1f44b-1f3fb.png");
png_name!(WAVING_HAND_TONE1, "👋🏻", "", U_1F44B_1F3FB, "1f44b-1f3fb.png");
png_name!(WAVE_TONE2, "👋🏼", "", U_1F44B_1F3FC, "1f44b-1f3fc.png");
png_name!(WAVING_HAND_TONE2, "👋🏼", "", U_1F44B_1F3FC, "1f44b-1f3fc.png");
png_name!(WAVE_TONE3, "👋🏽", "", U_1F44B_1F3FD, "1f44b-1f3fd.png");
png_name!(WAVING_HAND_TONE3, "👋🏽", "", U_1F44B_1F3FD, "1f44b-1f3fd.png");
png_name!(WAVE_TONE4, "👋🏾", "", U_1F44B_1F3FE, "1f44b-1f3fe.png");
png_name!(WAVING_HAND_TONE4, "👋🏾", "", U_1F44B_1F3FE, "1f44b-1f3fe.png");
png_name!(WAVE_TONE5, "👋🏿", "", U_1F44B_1F3FF, "1f44b-1f3ff.png");
png_name!(WAVING_HAND_TONE5, "👋🏿", "", U_1F44B_1F3FF, "1f44b-1f3ff.png");
png_name!(WAVE, "👋", "waving hand", U_1F44B, "1f44b.png");
png_name!(WAVING_HAND, "👋", "waving hand", U_1F44B, "1f44b.png");
png_name!(OK_HAND_TONE1, "👌🏻", "", U_1F44C_1F3FB, "1f44c-1f3fb.png");
png_name!(OK_HAND_TONE2, "👌🏼", "", U_1F44C_1F3FC, "1f44c-1f3fc.png");
png_name!(OK_HAND_TONE3, "👌🏽", "", U_1F44C_1F3FD, "1f44c-1f3fd.png");
png_name!(OK_HAND_TONE4, "👌🏾", "", U_1F44C_1F3FE, "1f44c-1f3fe.png");
png_name!(OK_HAND_TONE5, "👌🏿", "", U_1F44C_1F3FF, "1f44c-1f3ff.png");
png_name!(OK_HAND, "👌", "OK hand", U_1F44C, "1f44c.png");
png_name!(X_1_TONE1, "👍🏻", "", U_1F44D_1F3FB, "1f44d-1f3fb.png");
png_name!(THUMBSUP_TONE1, "👍🏻", "", U_1F44D_1F3FB, "1f44d-1f3fb.png");
png_name!(YES_TONE1, "👍🏻", "", U_1F44D_1F3FB, "1f44d-1f3fb.png");
png_name!(X_1_TONE2, "👍🏼", "", U_1F44D_1F3FC, "1f44d-1f3fc.png");
png_name!(THUMBSUP_TONE2, "👍🏼", "", U_1F44D_1F3FC, "1f44d-1f3fc.png");
png_name!(YES_TONE2, "👍🏼", "", U_1F44D_1F3FC, "1f44d-1f3fc.png");
png_name!(X_1_TONE3, "👍🏽", "", U_1F44D_1F3FD, "1f44d-1f3fd.png");
png_name!(THUMBSUP_TONE3, "👍🏽", "", U_1F44D_1F3FD, "1f44d-1f3fd.png");
png_name!(YES_TONE3, "👍🏽", "", U_1F44D_1F3FD, "1f44d-1f3fd.png");
png_name!(X_1_TONE4, "👍🏾", "", U_1F44D_1F3FE, "1f44d-1f3fe.png");
png_name!(THUMBSUP_TONE4, "👍🏾", "", U_1F44D_1F3FE, "1f44d-1f3fe.png");
png_name!(YES_TONE4, "👍🏾", "", U_1F44D_1F3FE, "1f44d-1f3fe.png");
png_name!(X_1_TONE5, "👍🏿", "", U_1F44D_1F3FF, "1f44d-1f3ff.png");
png_name!(THUMBSUP_TONE5, "👍🏿", "", U_1F44D_1F3FF, "1f44d-1f3ff.png");
png_name!(YES_TONE5, "👍🏿", "", U_1F44D_1F3FF, "1f44d-1f3ff.png");
png_name!(X_1, "👍", "thumbs up", U_1F44D, "1f44d.png");
png_name!(THUMBSUP, "👍", "thumbs up", U_1F44D, "1f44d.png");
png_name!(YES, "👍", "thumbs up", U_1F44D, "1f44d.png");
png_name!(X__1_TONE1, "👎🏻", "", U_1F44E_1F3FB, "1f44e-1f3fb.png");
png_name!(NO_TONE1, "👎🏻", "", U_1F44E_1F3FB, "1f44e-1f3fb.png");
png_name!(THUMBSDOWN_TONE1, "👎🏻", "", U_1F44E_1F3FB, "1f44e-1f3fb.png");
png_name!(X__1_TONE2, "👎🏼", "", U_1F44E_1F3FC, "1f44e-1f3fc.png");
png_name!(NO_TONE2, "👎🏼", "", U_1F44E_1F3FC, "1f44e-1f3fc.png");
png_name!(THUMBSDOWN_TONE2, "👎🏼", "", U_1F44E_1F3FC, "1f44e-1f3fc.png");
png_name!(X__1_TONE3, "👎🏽", "", U_1F44E_1F3FD, "1f44e-1f3fd.png");
png_name!(NO_TONE3, "👎🏽", "", U_1F44E_1F3FD, "1f44e-1f3fd.png");
png_name!(THUMBSDOWN_TONE3, "👎🏽", "", U_1F44E_1F3FD, "1f44e-1f3fd.png");
png_name!(X__1_TONE4, "👎🏾", "", U_1F44E_1F3FE, "1f44e-1f3fe.png");
png_name!(NO_TONE4, "👎🏾", "", U_1F44E_1F3FE, "1f44e-1f3fe.png");
png_name!(THUMBSDOWN_TONE4, "👎🏾", "", U_1F44E_1F3FE, "1f44e-1f3fe.png");
png_name!(X__1_TONE5, "👎🏿", "", U_1F44E_1F3FF, "1f44e-1f3ff.png");
png_name!(NO_TONE5, "👎🏿", "", U_1F44E_1F3FF, "1f44e-1f3ff.png");
png_name!(THUMBSDOWN_TONE5, "👎🏿", "", U_1F44E_1F3FF, "1f44e-1f3ff.png");
png_name!(X__1, "👎", "thumbs down", U_1F44E, "1f44e.png");
png_name!(NO, "👎", "thumbs down", U_1F44E, "1f44e.png");
png_name!(THUMBSDOWN, "👎", "thumbs down", U_1F44E, "1f44e.png");
png_name!(CLAP_TONE1, "👏🏻", "", U_1F44F_1F3FB, "1f44f-1f3fb.png");
png_name!(CLAPPING_HANDS_TONE1, "👏🏻", "", U_1F44F_1F3FB, "1f44f-1f3fb.png");
png_name!(CLAP_TONE2, "👏🏼", "", U_1F44F_1F3FC, "1f44f-1f3fc.png");
png_name!(CLAPPING_HANDS_TONE2, "👏🏼", "", U_1F44F_1F3FC, "1f44f-1f3fc.png");
png_name!(CLAP_TONE3, "👏🏽", "", U_1F44F_1F3FD, "1f44f-1f3fd.png");
png_name!(CLAPPING_HANDS_TONE3, "👏🏽", "", U_1F44F_1F3FD, "1f44f-1f3fd.png");
png_name!(CLAP_TONE4, "👏🏾", "", U_1F44F_1F3FE, "1f44f-1f3fe.png");
png_name!(CLAPPING_HANDS_TONE4, "👏🏾", "", U_1F44F_1F3FE, "1f44f-1f3fe.png");
png_name!(CLAP_TONE5, "👏🏿", "", U_1F44F_1F3FF, "1f44f-1f3ff.png");
png_name!(CLAPPING_HANDS_TONE5, "👏🏿", "", U_1F44F_1F3FF, "1f44f-1f3ff.png");
png_name!(CLAP, "👏", "clapping hands", U_1F44F, "1f44f.png");
png_name!(CLAPPING_HANDS, "👏", "clapping hands", U_1F44F, "1f44f.png");
png_name!(OPEN_HANDS_TONE1, "👐🏻", "", U_1F450_1F3FB, "1f450-1f3fb.png");
png_name!(OPEN_HANDS_TONE2, "👐🏼", "", U_1F450_1F3FC, "1f450-1f3fc.png");
png_name!(OPEN_HANDS_TONE3, "👐🏽", "", U_1F450_1F3FD, "1f450-1f3fd.png");
png_name!(OPEN_HANDS_TONE4, "👐🏾", "", U_1F450_1F3FE, "1f450-1f3fe.png");
png_name!(OPEN_HANDS_TONE5, "👐🏿", "", U_1F450_1F3FF, "1f450-1f3ff.png");
png_name!(OPEN_HANDS, "👐", "open hands", U_1F450, "1f450.png");
png_name!(CROWN, "👑", "crown", U_1F451, "1f451.png");
png_name!(WOMANS_HAT, "👒", "woman’s hat", U_1F452, "1f452.png");
png_name!(EYEGLASSES, "👓", "glasses", U_1F453, "1f453.png");
png_name!(GLASSES, "👓", "glasses", U_1F453, "1f453.png");
png_name!(NECKTIE, "👔", "necktie", U_1F454, "1f454.png");
png_name!(SHIRT, "👕", "t-shirt", U_1F455, "1f455.png");
png_name!(JEANS, "👖", "jeans", U_1F456, "1f456.png");
png_name!(DRESS, "👗", "dress", U_1F457, "1f457.png");
png_name!(KIMONO, "👘", "kimono", U_1F458, "1f458.png");
png_name!(BIKINI, "👙", "bikini", U_1F459, "1f459.png");
png_name!(WOMANS_CLOTHES, "👚", "woman’s clothes", U_1F45A, "1f45a.png");
png_name!(PURSE, "👛", "purse", U_1F45B, "1f45b.png");
png_name!(HANDBAG, "👜", "handbag", U_1F45C, "1f45c.png");
png_name!(CLUTCH_BAG, "👝", "clutch bag", U_1F45D, "1f45d.png");
png_name!(POUCH, "👝", "clutch bag", U_1F45D, "1f45d.png");
png_name!(MANS_SHOE, "👞", "man’s shoe", U_1F45E, "1f45e.png");
png_name!(ATHLETIC_SHOE, "👟", "running shoe", U_1F45F, "1f45f.png");
png_name!(SNEAKER, "👟", "running shoe", U_1F45F, "1f45f.png");
png_name!(HIGH_HEEL, "👠", "high-heeled shoe", U_1F460, "1f460.png");
png_name!(SANDAL, "👡", "woman’s sandal", U_1F461, "1f461.png");
png_name!(BOOT, "👢", "woman’s boot", U_1F462, "1f462.png");
png_name!(FOOTPRINTS, "👣", "footprints", U_1F463, "1f463.png");
png_name!(BUST_IN_SILHOUETTE, "👤", "bust in silhouette", U_1F464, "1f464.png");
png_name!(BUSTS_IN_SILHOUETTE, "👥", "busts in silhouette", U_1F465, "1f465.png");
png_name!(BOY_TONE1, "👦🏻", "", U_1F466_1F3FB, "1f466-1f3fb.png");
png_name!(BOY_TONE2, "👦🏼", "", U_1F466_1F3FC, "1f466-1f3fc.png");
png_name!(BOY_TONE3, "👦🏽", "", U_1F466_1F3FD, "1f466-1f3fd.png");
png_name!(BOY_TONE4, "👦🏾", "", U_1F466_1F3FE, "1f466-1f3fe.png");
png_name!(BOY_TONE5, "👦🏿", "", U_1F466_1F3FF, "1f466-1f3ff.png");
png_name!(BOY, "👦", "boy", U_1F466, "1f466.png");
png_name!(GIRL_TONE1, "👧🏻", "", U_1F467_1F3FB, "1f467-1f3fb.png");
png_name!(GIRL_TONE2, "👧🏼", "", U_1F467_1F3FC, "1f467-1f3fc.png");
png_name!(GIRL_TONE3, "👧🏽", "", U_1F467_1F3FD, "1f467-1f3fd.png");
png_name!(GIRL_TONE4, "👧🏾", "", U_1F467_1F3FE, "1f467-1f3fe.png");
png_name!(GIRL_TONE5, "👧🏿", "", U_1F467_1F3FF, "1f467-1f3ff.png");
png_name!(GIRL, "👧", "girl", U_1F467, "1f467.png");
png_name!(MAN_FARMER_TONE1, "👨🏻‍🌾", "", U_1F468_1F3FB_200D_1F33E, "1f468-1f3fb-200d-1f33e.png");
png_name!(MAN_COOK_TONE1, "👨🏻‍🍳", "", U_1F468_1F3FB_200D_1F373, "1f468-1f3fb-200d-1f373.png");
png_name!(MAN_FEEDING_BABY_TONE1, "👨🏻‍🍼", "", U_1F468_1F3FB_200D_1F37C, "1f468-1f3fb-200d-1f37c.png");
png_name!(MAN_STUDENT_TONE1, "👨🏻‍🎓", "", U_1F468_1F3FB_200D_1F393, "1f468-1f3fb-200d-1f393.png");
png_name!(MAN_SINGER_TONE1, "👨🏻‍🎤", "", U_1F468_1F3FB_200D_1F3A4, "1f468-1f3fb-200d-1f3a4.png");
png_name!(MAN_ARTIST_TONE1, "👨🏻‍🎨", "", U_1F468_1F3FB_200D_1F3A8, "1f468-1f3fb-200d-1f3a8.png");
png_name!(MAN_TEACHER_TONE1, "👨🏻‍🏫", "", U_1F468_1F3FB_200D_1F3EB, "1f468-1f3fb-200d-1f3eb.png");
png_name!(MAN_FACTORY_WORKER_TONE1, "👨🏻‍🏭", "", U_1F468_1F3FB_200D_1F3ED, "1f468-1f3fb-200d-1f3ed.png");
png_name!(MAN_TECHNOLOGIST_TONE1, "👨🏻‍💻", "", U_1F468_1F3FB_200D_1F4BB, "1f468-1f3fb-200d-1f4bb.png");
png_name!(MAN_OFFICE_WORKER_TONE1, "👨🏻‍💼", "", U_1F468_1F3FB_200D_1F4BC, "1f468-1f3fb-200d-1f4bc.png");
png_name!(MAN_MECHANIC_TONE1, "👨🏻‍🔧", "", U_1F468_1F3FB_200D_1F527, "1f468-1f3fb-200d-1f527.png");
png_name!(MAN_SCIENTIST_TONE1, "👨🏻‍🔬", "", U_1F468_1F3FB_200D_1F52C, "1f468-1f3fb-200d-1f52c.png");
png_name!(MAN_ASTRONAUT_TONE1, "👨🏻‍🚀", "", U_1F468_1F3FB_200D_1F680, "1f468-1f3fb-200d-1f680.png");
png_name!(MAN_FIREFIGHTER_TONE1, "👨🏻‍🚒", "", U_1F468_1F3FB_200D_1F692, "1f468-1f3fb-200d-1f692.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE1_2, "👨🏻‍🤝‍👨🏼", "", U_1F468_1F3FB_200D_1F91D_200D_1F468_1F3FC, "1f468-1f3fb-200d-1f91d-200d-1f468-1f3fc.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE1_3, "👨🏻‍🤝‍👨🏽", "", U_1F468_1F3FB_200D_1F91D_200D_1F468_1F3FD, "1f468-1f3fb-200d-1f91d-200d-1f468-1f3fd.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE1_4, "👨🏻‍🤝‍👨🏾", "", U_1F468_1F3FB_200D_1F91D_200D_1F468_1F3FE, "1f468-1f3fb-200d-1f91d-200d-1f468-1f3fe.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE1_5, "👨🏻‍🤝‍👨🏿", "", U_1F468_1F3FB_200D_1F91D_200D_1F468_1F3FF, "1f468-1f3fb-200d-1f91d-200d-1f468-1f3ff.png");
png_name!(MAN_WITH_PROBING_CANE_TONE1, "👨🏻‍🦯", "", U_1F468_1F3FB_200D_1F9AF, "1f468-1f3fb-200d-1f9af.png");
png_name!(MAN_WITH_WHITE_CANE_TONE1, "👨🏻‍🦯", "", U_1F468_1F3FB_200D_1F9AF, "1f468-1f3fb-200d-1f9af.png");
png_name!(MAN_RED_HAIRED_TONE1, "👨🏻‍🦰", "", U_1F468_1F3FB_200D_1F9B0, "1f468-1f3fb-200d-1f9b0.png");
png_name!(MAN_CURLY_HAIRED_TONE1, "👨🏻‍🦱", "", U_1F468_1F3FB_200D_1F9B1, "1f468-1f3fb-200d-1f9b1.png");
png_name!(MAN_BALD_TONE1, "👨🏻‍🦲", "", U_1F468_1F3FB_200D_1F9B2, "1f468-1f3fb-200d-1f9b2.png");
png_name!(MAN_WHITE_HAIRED_TONE1, "👨🏻‍🦳", "", U_1F468_1F3FB_200D_1F9B3, "1f468-1f3fb-200d-1f9b3.png");
png_name!(MAN_IN_MOTORIZED_WHEELCHAIR_TONE1, "👨🏻‍🦼", "", U_1F468_1F3FB_200D_1F9BC, "1f468-1f3fb-200d-1f9bc.png");
png_name!(MAN_IN_MANUAL_WHEELCHAIR_TONE1, "👨🏻‍🦽", "", U_1F468_1F3FB_200D_1F9BD, "1f468-1f3fb-200d-1f9bd.png");
png_name!(MAN_HEALTH_WORKER_TONE1, "👨🏻‍⚕️", "", U_1F468_1F3FB_200D_2695_FE0F, "1f468-1f3fb-200d-2695-fe0f.png");
png_name!(MAN_JUDGE_TONE1, "👨🏻‍⚖️", "", U_1F468_1F3FB_200D_2696_FE0F, "1f468-1f3fb-200d-2696-fe0f.png");
png_name!(MAN_PILOT_TONE1, "👨🏻‍✈️", "", U_1F468_1F3FB_200D_2708_FE0F, "1f468-1f3fb-200d-2708-fe0f.png");
png_name!(COUPLE_WITH_HEART_MM_TONE1, "👨🏻‍❤️‍👨🏻", "", U_1F468_1F3FB_200D_2764_FE0F_200D_1F468_1F3FB, "1f468-1f3fb-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_MM_TONE1_2, "👨🏻‍❤️‍👨🏼", "", U_1F468_1F3FB_200D_2764_FE0F_200D_1F468_1F3FC, "1f468-1f3fb-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_MM_TONE1_3, "👨🏻‍❤️‍👨🏽", "", U_1F468_1F3FB_200D_2764_FE0F_200D_1F468_1F3FD, "1f468-1f3fb-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_MM_TONE1_4, "👨🏻‍❤️‍👨🏾", "", U_1F468_1F3FB_200D_2764_FE0F_200D_1F468_1F3FE, "1f468-1f3fb-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_MM_TONE1_5, "👨🏻‍❤️‍👨🏿", "", U_1F468_1F3FB_200D_2764_FE0F_200D_1F468_1F3FF, "1f468-1f3fb-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(KISS_MM_TONE1, "👨🏻‍❤️‍💋‍👨🏻", "", U_1F468_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f468-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_MM_TONE1_2, "👨🏻‍❤️‍💋‍👨🏼", "", U_1F468_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f468-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_MM_TONE1_3, "👨🏻‍❤️‍💋‍👨🏽", "", U_1F468_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f468-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_MM_TONE1_4, "👨🏻‍❤️‍💋‍👨🏾", "", U_1F468_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f468-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_MM_TONE1_5, "👨🏻‍❤️‍💋‍👨🏿", "", U_1F468_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f468-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(MAN_TONE1, "👨🏻", "", U_1F468_1F3FB, "1f468-1f3fb.png");
png_name!(MAN_FARMER_TONE2, "👨🏼‍🌾", "", U_1F468_1F3FC_200D_1F33E, "1f468-1f3fc-200d-1f33e.png");
png_name!(MAN_COOK_TONE2, "👨🏼‍🍳", "", U_1F468_1F3FC_200D_1F373, "1f468-1f3fc-200d-1f373.png");
png_name!(MAN_FEEDING_BABY_TONE2, "👨🏼‍🍼", "", U_1F468_1F3FC_200D_1F37C, "1f468-1f3fc-200d-1f37c.png");
png_name!(MAN_STUDENT_TONE2, "👨🏼‍🎓", "", U_1F468_1F3FC_200D_1F393, "1f468-1f3fc-200d-1f393.png");
png_name!(MAN_SINGER_TONE2, "👨🏼‍🎤", "", U_1F468_1F3FC_200D_1F3A4, "1f468-1f3fc-200d-1f3a4.png");
png_name!(MAN_ARTIST_TONE2, "👨🏼‍🎨", "", U_1F468_1F3FC_200D_1F3A8, "1f468-1f3fc-200d-1f3a8.png");
png_name!(MAN_TEACHER_TONE2, "👨🏼‍🏫", "", U_1F468_1F3FC_200D_1F3EB, "1f468-1f3fc-200d-1f3eb.png");
png_name!(MAN_FACTORY_WORKER_TONE2, "👨🏼‍🏭", "", U_1F468_1F3FC_200D_1F3ED, "1f468-1f3fc-200d-1f3ed.png");
png_name!(MAN_TECHNOLOGIST_TONE2, "👨🏼‍💻", "", U_1F468_1F3FC_200D_1F4BB, "1f468-1f3fc-200d-1f4bb.png");
png_name!(MAN_OFFICE_WORKER_TONE2, "👨🏼‍💼", "", U_1F468_1F3FC_200D_1F4BC, "1f468-1f3fc-200d-1f4bc.png");
png_name!(MAN_MECHANIC_TONE2, "👨🏼‍🔧", "", U_1F468_1F3FC_200D_1F527, "1f468-1f3fc-200d-1f527.png");
png_name!(MAN_SCIENTIST_TONE2, "👨🏼‍🔬", "", U_1F468_1F3FC_200D_1F52C, "1f468-1f3fc-200d-1f52c.png");
png_name!(MAN_ASTRONAUT_TONE2, "👨🏼‍🚀", "", U_1F468_1F3FC_200D_1F680, "1f468-1f3fc-200d-1f680.png");
png_name!(MAN_FIREFIGHTER_TONE2, "👨🏼‍🚒", "", U_1F468_1F3FC_200D_1F692, "1f468-1f3fc-200d-1f692.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE2_1, "👨🏼‍🤝‍👨🏻", "", U_1F468_1F3FC_200D_1F91D_200D_1F468_1F3FB, "1f468-1f3fc-200d-1f91d-200d-1f468-1f3fb.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE2_3, "👨🏼‍🤝‍👨🏽", "", U_1F468_1F3FC_200D_1F91D_200D_1F468_1F3FD, "1f468-1f3fc-200d-1f91d-200d-1f468-1f3fd.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE2_4, "👨🏼‍🤝‍👨🏾", "", U_1F468_1F3FC_200D_1F91D_200D_1F468_1F3FE, "1f468-1f3fc-200d-1f91d-200d-1f468-1f3fe.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE2_5, "👨🏼‍🤝‍👨🏿", "", U_1F468_1F3FC_200D_1F91D_200D_1F468_1F3FF, "1f468-1f3fc-200d-1f91d-200d-1f468-1f3ff.png");
png_name!(MAN_WITH_PROBING_CANE_TONE2, "👨🏼‍🦯", "", U_1F468_1F3FC_200D_1F9AF, "1f468-1f3fc-200d-1f9af.png");
png_name!(MAN_WITH_WHITE_CANE_TONE2, "👨🏼‍🦯", "", U_1F468_1F3FC_200D_1F9AF, "1f468-1f3fc-200d-1f9af.png");
png_name!(MAN_RED_HAIRED_TONE2, "👨🏼‍🦰", "", U_1F468_1F3FC_200D_1F9B0, "1f468-1f3fc-200d-1f9b0.png");
png_name!(MAN_CURLY_HAIRED_TONE2, "👨🏼‍🦱", "", U_1F468_1F3FC_200D_1F9B1, "1f468-1f3fc-200d-1f9b1.png");
png_name!(MAN_BALD_TONE2, "👨🏼‍🦲", "", U_1F468_1F3FC_200D_1F9B2, "1f468-1f3fc-200d-1f9b2.png");
png_name!(MAN_WHITE_HAIRED_TONE2, "👨🏼‍🦳", "", U_1F468_1F3FC_200D_1F9B3, "1f468-1f3fc-200d-1f9b3.png");
png_name!(MAN_IN_MOTORIZED_WHEELCHAIR_TONE2, "👨🏼‍🦼", "", U_1F468_1F3FC_200D_1F9BC, "1f468-1f3fc-200d-1f9bc.png");
png_name!(MAN_IN_MANUAL_WHEELCHAIR_TONE2, "👨🏼‍🦽", "", U_1F468_1F3FC_200D_1F9BD, "1f468-1f3fc-200d-1f9bd.png");
png_name!(MAN_HEALTH_WORKER_TONE2, "👨🏼‍⚕️", "", U_1F468_1F3FC_200D_2695_FE0F, "1f468-1f3fc-200d-2695-fe0f.png");
png_name!(MAN_JUDGE_TONE2, "👨🏼‍⚖️", "", U_1F468_1F3FC_200D_2696_FE0F, "1f468-1f3fc-200d-2696-fe0f.png");
png_name!(MAN_PILOT_TONE2, "👨🏼‍✈️", "", U_1F468_1F3FC_200D_2708_FE0F, "1f468-1f3fc-200d-2708-fe0f.png");
png_name!(COUPLE_WITH_HEART_MM_TONE2_1, "👨🏼‍❤️‍👨🏻", "", U_1F468_1F3FC_200D_2764_FE0F_200D_1F468_1F3FB, "1f468-1f3fc-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_MM_TONE2, "👨🏼‍❤️‍👨🏼", "", U_1F468_1F3FC_200D_2764_FE0F_200D_1F468_1F3FC, "1f468-1f3fc-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_MM_TONE2_3, "👨🏼‍❤️‍👨🏽", "", U_1F468_1F3FC_200D_2764_FE0F_200D_1F468_1F3FD, "1f468-1f3fc-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_MM_TONE2_4, "👨🏼‍❤️‍👨🏾", "", U_1F468_1F3FC_200D_2764_FE0F_200D_1F468_1F3FE, "1f468-1f3fc-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_MM_TONE2_5, "👨🏼‍❤️‍👨🏿", "", U_1F468_1F3FC_200D_2764_FE0F_200D_1F468_1F3FF, "1f468-1f3fc-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(KISS_MM_TONE2_1, "👨🏼‍❤️‍💋‍👨🏻", "", U_1F468_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f468-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_MM_TONE2, "👨🏼‍❤️‍💋‍👨🏼", "", U_1F468_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f468-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_MM_TONE2_3, "👨🏼‍❤️‍💋‍👨🏽", "", U_1F468_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f468-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_MM_TONE2_4, "👨🏼‍❤️‍💋‍👨🏾", "", U_1F468_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f468-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_MM_TONE2_5, "👨🏼‍❤️‍💋‍👨🏿", "", U_1F468_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f468-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(MAN_TONE2, "👨🏼", "", U_1F468_1F3FC, "1f468-1f3fc.png");
png_name!(MAN_FARMER_TONE3, "👨🏽‍🌾", "", U_1F468_1F3FD_200D_1F33E, "1f468-1f3fd-200d-1f33e.png");
png_name!(MAN_COOK_TONE3, "👨🏽‍🍳", "", U_1F468_1F3FD_200D_1F373, "1f468-1f3fd-200d-1f373.png");
png_name!(MAN_FEEDING_BABY_TONE3, "👨🏽‍🍼", "", U_1F468_1F3FD_200D_1F37C, "1f468-1f3fd-200d-1f37c.png");
png_name!(MAN_STUDENT_TONE3, "👨🏽‍🎓", "", U_1F468_1F3FD_200D_1F393, "1f468-1f3fd-200d-1f393.png");
png_name!(MAN_SINGER_TONE3, "👨🏽‍🎤", "", U_1F468_1F3FD_200D_1F3A4, "1f468-1f3fd-200d-1f3a4.png");
png_name!(MAN_ARTIST_TONE3, "👨🏽‍🎨", "", U_1F468_1F3FD_200D_1F3A8, "1f468-1f3fd-200d-1f3a8.png");
png_name!(MAN_TEACHER_TONE3, "👨🏽‍🏫", "", U_1F468_1F3FD_200D_1F3EB, "1f468-1f3fd-200d-1f3eb.png");
png_name!(MAN_FACTORY_WORKER_TONE3, "👨🏽‍🏭", "", U_1F468_1F3FD_200D_1F3ED, "1f468-1f3fd-200d-1f3ed.png");
png_name!(MAN_TECHNOLOGIST_TONE3, "👨🏽‍💻", "", U_1F468_1F3FD_200D_1F4BB, "1f468-1f3fd-200d-1f4bb.png");
png_name!(MAN_OFFICE_WORKER_TONE3, "👨🏽‍💼", "", U_1F468_1F3FD_200D_1F4BC, "1f468-1f3fd-200d-1f4bc.png");
png_name!(MAN_MECHANIC_TONE3, "👨🏽‍🔧", "", U_1F468_1F3FD_200D_1F527, "1f468-1f3fd-200d-1f527.png");
png_name!(MAN_SCIENTIST_TONE3, "👨🏽‍🔬", "", U_1F468_1F3FD_200D_1F52C, "1f468-1f3fd-200d-1f52c.png");
png_name!(MAN_ASTRONAUT_TONE3, "👨🏽‍🚀", "", U_1F468_1F3FD_200D_1F680, "1f468-1f3fd-200d-1f680.png");
png_name!(MAN_FIREFIGHTER_TONE3, "👨🏽‍🚒", "", U_1F468_1F3FD_200D_1F692, "1f468-1f3fd-200d-1f692.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE3_1, "👨🏽‍🤝‍👨🏻", "", U_1F468_1F3FD_200D_1F91D_200D_1F468_1F3FB, "1f468-1f3fd-200d-1f91d-200d-1f468-1f3fb.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE3_2, "👨🏽‍🤝‍👨🏼", "", U_1F468_1F3FD_200D_1F91D_200D_1F468_1F3FC, "1f468-1f3fd-200d-1f91d-200d-1f468-1f3fc.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE3_4, "👨🏽‍🤝‍👨🏾", "", U_1F468_1F3FD_200D_1F91D_200D_1F468_1F3FE, "1f468-1f3fd-200d-1f91d-200d-1f468-1f3fe.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE3_5, "👨🏽‍🤝‍👨🏿", "", U_1F468_1F3FD_200D_1F91D_200D_1F468_1F3FF, "1f468-1f3fd-200d-1f91d-200d-1f468-1f3ff.png");
png_name!(MAN_WITH_PROBING_CANE_TONE3, "👨🏽‍🦯", "", U_1F468_1F3FD_200D_1F9AF, "1f468-1f3fd-200d-1f9af.png");
png_name!(MAN_WITH_WHITE_CANE_TONE3, "👨🏽‍🦯", "", U_1F468_1F3FD_200D_1F9AF, "1f468-1f3fd-200d-1f9af.png");
png_name!(MAN_RED_HAIRED_TONE3, "👨🏽‍🦰", "", U_1F468_1F3FD_200D_1F9B0, "1f468-1f3fd-200d-1f9b0.png");
png_name!(MAN_CURLY_HAIRED_TONE3, "👨🏽‍🦱", "", U_1F468_1F3FD_200D_1F9B1, "1f468-1f3fd-200d-1f9b1.png");
png_name!(MAN_BALD_TONE3, "👨🏽‍🦲", "", U_1F468_1F3FD_200D_1F9B2, "1f468-1f3fd-200d-1f9b2.png");
png_name!(MAN_WHITE_HAIRED_TONE3, "👨🏽‍🦳", "", U_1F468_1F3FD_200D_1F9B3, "1f468-1f3fd-200d-1f9b3.png");
png_name!(MAN_IN_MOTORIZED_WHEELCHAIR_TONE3, "👨🏽‍🦼", "", U_1F468_1F3FD_200D_1F9BC, "1f468-1f3fd-200d-1f9bc.png");
png_name!(MAN_IN_MANUAL_WHEELCHAIR_TONE3, "👨🏽‍🦽", "", U_1F468_1F3FD_200D_1F9BD, "1f468-1f3fd-200d-1f9bd.png");
png_name!(MAN_HEALTH_WORKER_TONE3, "👨🏽‍⚕️", "", U_1F468_1F3FD_200D_2695_FE0F, "1f468-1f3fd-200d-2695-fe0f.png");
png_name!(MAN_JUDGE_TONE3, "👨🏽‍⚖️", "", U_1F468_1F3FD_200D_2696_FE0F, "1f468-1f3fd-200d-2696-fe0f.png");
png_name!(MAN_PILOT_TONE3, "👨🏽‍✈️", "", U_1F468_1F3FD_200D_2708_FE0F, "1f468-1f3fd-200d-2708-fe0f.png");
png_name!(COUPLE_WITH_HEART_MM_TONE3_1, "👨🏽‍❤️‍👨🏻", "", U_1F468_1F3FD_200D_2764_FE0F_200D_1F468_1F3FB, "1f468-1f3fd-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_MM_TONE3_2, "👨🏽‍❤️‍👨🏼", "", U_1F468_1F3FD_200D_2764_FE0F_200D_1F468_1F3FC, "1f468-1f3fd-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_MM_TONE3, "👨🏽‍❤️‍👨🏽", "", U_1F468_1F3FD_200D_2764_FE0F_200D_1F468_1F3FD, "1f468-1f3fd-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_MM_TONE3_4, "👨🏽‍❤️‍👨🏾", "", U_1F468_1F3FD_200D_2764_FE0F_200D_1F468_1F3FE, "1f468-1f3fd-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_MM_TONE3_5, "👨🏽‍❤️‍👨🏿", "", U_1F468_1F3FD_200D_2764_FE0F_200D_1F468_1F3FF, "1f468-1f3fd-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(KISS_MM_TONE3_1, "👨🏽‍❤️‍💋‍👨🏻", "", U_1F468_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f468-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_MM_TONE3_2, "👨🏽‍❤️‍💋‍👨🏼", "", U_1F468_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f468-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_MM_TONE3, "👨🏽‍❤️‍💋‍👨🏽", "", U_1F468_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f468-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_MM_TONE3_4, "👨🏽‍❤️‍💋‍👨🏾", "", U_1F468_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f468-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_MM_TONE3_5, "👨🏽‍❤️‍💋‍👨🏿", "", U_1F468_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f468-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(MAN_TONE3, "👨🏽", "", U_1F468_1F3FD, "1f468-1f3fd.png");
png_name!(MAN_FARMER_TONE4, "👨🏾‍🌾", "", U_1F468_1F3FE_200D_1F33E, "1f468-1f3fe-200d-1f33e.png");
png_name!(MAN_COOK_TONE4, "👨🏾‍🍳", "", U_1F468_1F3FE_200D_1F373, "1f468-1f3fe-200d-1f373.png");
png_name!(MAN_FEEDING_BABY_TONE4, "👨🏾‍🍼", "", U_1F468_1F3FE_200D_1F37C, "1f468-1f3fe-200d-1f37c.png");
png_name!(MAN_STUDENT_TONE4, "👨🏾‍🎓", "", U_1F468_1F3FE_200D_1F393, "1f468-1f3fe-200d-1f393.png");
png_name!(MAN_SINGER_TONE4, "👨🏾‍🎤", "", U_1F468_1F3FE_200D_1F3A4, "1f468-1f3fe-200d-1f3a4.png");
png_name!(MAN_ARTIST_TONE4, "👨🏾‍🎨", "", U_1F468_1F3FE_200D_1F3A8, "1f468-1f3fe-200d-1f3a8.png");
png_name!(MAN_TEACHER_TONE4, "👨🏾‍🏫", "", U_1F468_1F3FE_200D_1F3EB, "1f468-1f3fe-200d-1f3eb.png");
png_name!(MAN_FACTORY_WORKER_TONE4, "👨🏾‍🏭", "", U_1F468_1F3FE_200D_1F3ED, "1f468-1f3fe-200d-1f3ed.png");
png_name!(MAN_TECHNOLOGIST_TONE4, "👨🏾‍💻", "", U_1F468_1F3FE_200D_1F4BB, "1f468-1f3fe-200d-1f4bb.png");
png_name!(MAN_OFFICE_WORKER_TONE4, "👨🏾‍💼", "", U_1F468_1F3FE_200D_1F4BC, "1f468-1f3fe-200d-1f4bc.png");
png_name!(MAN_MECHANIC_TONE4, "👨🏾‍🔧", "", U_1F468_1F3FE_200D_1F527, "1f468-1f3fe-200d-1f527.png");
png_name!(MAN_SCIENTIST_TONE4, "👨🏾‍🔬", "", U_1F468_1F3FE_200D_1F52C, "1f468-1f3fe-200d-1f52c.png");
png_name!(MAN_ASTRONAUT_TONE4, "👨🏾‍🚀", "", U_1F468_1F3FE_200D_1F680, "1f468-1f3fe-200d-1f680.png");
png_name!(MAN_FIREFIGHTER_TONE4, "👨🏾‍🚒", "", U_1F468_1F3FE_200D_1F692, "1f468-1f3fe-200d-1f692.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE4_1, "👨🏾‍🤝‍👨🏻", "", U_1F468_1F3FE_200D_1F91D_200D_1F468_1F3FB, "1f468-1f3fe-200d-1f91d-200d-1f468-1f3fb.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE4_2, "👨🏾‍🤝‍👨🏼", "", U_1F468_1F3FE_200D_1F91D_200D_1F468_1F3FC, "1f468-1f3fe-200d-1f91d-200d-1f468-1f3fc.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE4_3, "👨🏾‍🤝‍👨🏽", "", U_1F468_1F3FE_200D_1F91D_200D_1F468_1F3FD, "1f468-1f3fe-200d-1f91d-200d-1f468-1f3fd.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE4_5, "👨🏾‍🤝‍👨🏿", "", U_1F468_1F3FE_200D_1F91D_200D_1F468_1F3FF, "1f468-1f3fe-200d-1f91d-200d-1f468-1f3ff.png");
png_name!(MAN_WITH_PROBING_CANE_TONE4, "👨🏾‍🦯", "", U_1F468_1F3FE_200D_1F9AF, "1f468-1f3fe-200d-1f9af.png");
png_name!(MAN_WITH_WHITE_CANE_TONE4, "👨🏾‍🦯", "", U_1F468_1F3FE_200D_1F9AF, "1f468-1f3fe-200d-1f9af.png");
png_name!(MAN_RED_HAIRED_TONE4, "👨🏾‍🦰", "", U_1F468_1F3FE_200D_1F9B0, "1f468-1f3fe-200d-1f9b0.png");
png_name!(MAN_CURLY_HAIRED_TONE4, "👨🏾‍🦱", "", U_1F468_1F3FE_200D_1F9B1, "1f468-1f3fe-200d-1f9b1.png");
png_name!(MAN_BALD_TONE4, "👨🏾‍🦲", "", U_1F468_1F3FE_200D_1F9B2, "1f468-1f3fe-200d-1f9b2.png");
png_name!(MAN_WHITE_HAIRED_TONE4, "👨🏾‍🦳", "", U_1F468_1F3FE_200D_1F9B3, "1f468-1f3fe-200d-1f9b3.png");
png_name!(MAN_IN_MOTORIZED_WHEELCHAIR_TONE4, "👨🏾‍🦼", "", U_1F468_1F3FE_200D_1F9BC, "1f468-1f3fe-200d-1f9bc.png");
png_name!(MAN_IN_MANUAL_WHEELCHAIR_TONE4, "👨🏾‍🦽", "", U_1F468_1F3FE_200D_1F9BD, "1f468-1f3fe-200d-1f9bd.png");
png_name!(MAN_HEALTH_WORKER_TONE4, "👨🏾‍⚕️", "", U_1F468_1F3FE_200D_2695_FE0F, "1f468-1f3fe-200d-2695-fe0f.png");
png_name!(MAN_JUDGE_TONE4, "👨🏾‍⚖️", "", U_1F468_1F3FE_200D_2696_FE0F, "1f468-1f3fe-200d-2696-fe0f.png");
png_name!(MAN_PILOT_TONE4, "👨🏾‍✈️", "", U_1F468_1F3FE_200D_2708_FE0F, "1f468-1f3fe-200d-2708-fe0f.png");
png_name!(COUPLE_WITH_HEART_MM_TONE4_1, "👨🏾‍❤️‍👨🏻", "", U_1F468_1F3FE_200D_2764_FE0F_200D_1F468_1F3FB, "1f468-1f3fe-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_MM_TONE4_2, "👨🏾‍❤️‍👨🏼", "", U_1F468_1F3FE_200D_2764_FE0F_200D_1F468_1F3FC, "1f468-1f3fe-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_MM_TONE4_3, "👨🏾‍❤️‍👨🏽", "", U_1F468_1F3FE_200D_2764_FE0F_200D_1F468_1F3FD, "1f468-1f3fe-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_MM_TONE4, "👨🏾‍❤️‍👨🏾", "", U_1F468_1F3FE_200D_2764_FE0F_200D_1F468_1F3FE, "1f468-1f3fe-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_MM_TONE4_5, "👨🏾‍❤️‍👨🏿", "", U_1F468_1F3FE_200D_2764_FE0F_200D_1F468_1F3FF, "1f468-1f3fe-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(KISS_MM_TONE4_1, "👨🏾‍❤️‍💋‍👨🏻", "", U_1F468_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f468-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_MM_TONE4_2, "👨🏾‍❤️‍💋‍👨🏼", "", U_1F468_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f468-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_MM_TONE4_3, "👨🏾‍❤️‍💋‍👨🏽", "", U_1F468_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f468-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_MM_TONE4, "👨🏾‍❤️‍💋‍👨🏾", "", U_1F468_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f468-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_MM_TONE4_5, "👨🏾‍❤️‍💋‍👨🏿", "", U_1F468_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f468-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(MAN_TONE4, "👨🏾", "", U_1F468_1F3FE, "1f468-1f3fe.png");
png_name!(MAN_FARMER_TONE5, "👨🏿‍🌾", "", U_1F468_1F3FF_200D_1F33E, "1f468-1f3ff-200d-1f33e.png");
png_name!(MAN_COOK_TONE5, "👨🏿‍🍳", "", U_1F468_1F3FF_200D_1F373, "1f468-1f3ff-200d-1f373.png");
png_name!(MAN_FEEDING_BABY_TONE5, "👨🏿‍🍼", "", U_1F468_1F3FF_200D_1F37C, "1f468-1f3ff-200d-1f37c.png");
png_name!(MAN_STUDENT_TONE5, "👨🏿‍🎓", "", U_1F468_1F3FF_200D_1F393, "1f468-1f3ff-200d-1f393.png");
png_name!(MAN_SINGER_TONE5, "👨🏿‍🎤", "", U_1F468_1F3FF_200D_1F3A4, "1f468-1f3ff-200d-1f3a4.png");
png_name!(MAN_ARTIST_TONE5, "👨🏿‍🎨", "", U_1F468_1F3FF_200D_1F3A8, "1f468-1f3ff-200d-1f3a8.png");
png_name!(MAN_TEACHER_TONE5, "👨🏿‍🏫", "", U_1F468_1F3FF_200D_1F3EB, "1f468-1f3ff-200d-1f3eb.png");
png_name!(MAN_FACTORY_WORKER_TONE5, "👨🏿‍🏭", "", U_1F468_1F3FF_200D_1F3ED, "1f468-1f3ff-200d-1f3ed.png");
png_name!(MAN_TECHNOLOGIST_TONE5, "👨🏿‍💻", "", U_1F468_1F3FF_200D_1F4BB, "1f468-1f3ff-200d-1f4bb.png");
png_name!(MAN_OFFICE_WORKER_TONE5, "👨🏿‍💼", "", U_1F468_1F3FF_200D_1F4BC, "1f468-1f3ff-200d-1f4bc.png");
png_name!(MAN_MECHANIC_TONE5, "👨🏿‍🔧", "", U_1F468_1F3FF_200D_1F527, "1f468-1f3ff-200d-1f527.png");
png_name!(MAN_SCIENTIST_TONE5, "👨🏿‍🔬", "", U_1F468_1F3FF_200D_1F52C, "1f468-1f3ff-200d-1f52c.png");
png_name!(MAN_ASTRONAUT_TONE5, "👨🏿‍🚀", "", U_1F468_1F3FF_200D_1F680, "1f468-1f3ff-200d-1f680.png");
png_name!(MAN_FIREFIGHTER_TONE5, "👨🏿‍🚒", "", U_1F468_1F3FF_200D_1F692, "1f468-1f3ff-200d-1f692.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE5_1, "👨🏿‍🤝‍👨🏻", "", U_1F468_1F3FF_200D_1F91D_200D_1F468_1F3FB, "1f468-1f3ff-200d-1f91d-200d-1f468-1f3fb.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE5_2, "👨🏿‍🤝‍👨🏼", "", U_1F468_1F3FF_200D_1F91D_200D_1F468_1F3FC, "1f468-1f3ff-200d-1f91d-200d-1f468-1f3fc.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE5_3, "👨🏿‍🤝‍👨🏽", "", U_1F468_1F3FF_200D_1F91D_200D_1F468_1F3FD, "1f468-1f3ff-200d-1f91d-200d-1f468-1f3fd.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE5_4, "👨🏿‍🤝‍👨🏾", "", U_1F468_1F3FF_200D_1F91D_200D_1F468_1F3FE, "1f468-1f3ff-200d-1f91d-200d-1f468-1f3fe.png");
png_name!(MAN_WITH_PROBING_CANE_TONE5, "👨🏿‍🦯", "", U_1F468_1F3FF_200D_1F9AF, "1f468-1f3ff-200d-1f9af.png");
png_name!(MAN_WITH_WHITE_CANE_TONE5, "👨🏿‍🦯", "", U_1F468_1F3FF_200D_1F9AF, "1f468-1f3ff-200d-1f9af.png");
png_name!(MAN_RED_HAIRED_TONE5, "👨🏿‍🦰", "", U_1F468_1F3FF_200D_1F9B0, "1f468-1f3ff-200d-1f9b0.png");
png_name!(MAN_CURLY_HAIRED_TONE5, "👨🏿‍🦱", "", U_1F468_1F3FF_200D_1F9B1, "1f468-1f3ff-200d-1f9b1.png");
png_name!(MAN_BALD_TONE5, "👨🏿‍🦲", "", U_1F468_1F3FF_200D_1F9B2, "1f468-1f3ff-200d-1f9b2.png");
png_name!(MAN_WHITE_HAIRED_TONE5, "👨🏿‍🦳", "", U_1F468_1F3FF_200D_1F9B3, "1f468-1f3ff-200d-1f9b3.png");
png_name!(MAN_IN_MOTORIZED_WHEELCHAIR_TONE5, "👨🏿‍🦼", "", U_1F468_1F3FF_200D_1F9BC, "1f468-1f3ff-200d-1f9bc.png");
png_name!(MAN_IN_MANUAL_WHEELCHAIR_TONE5, "👨🏿‍🦽", "", U_1F468_1F3FF_200D_1F9BD, "1f468-1f3ff-200d-1f9bd.png");
png_name!(MAN_HEALTH_WORKER_TONE5, "👨🏿‍⚕️", "", U_1F468_1F3FF_200D_2695_FE0F, "1f468-1f3ff-200d-2695-fe0f.png");
png_name!(MAN_JUDGE_TONE5, "👨🏿‍⚖️", "", U_1F468_1F3FF_200D_2696_FE0F, "1f468-1f3ff-200d-2696-fe0f.png");
png_name!(MAN_PILOT_TONE5, "👨🏿‍✈️", "", U_1F468_1F3FF_200D_2708_FE0F, "1f468-1f3ff-200d-2708-fe0f.png");
png_name!(COUPLE_WITH_HEART_MM_TONE5_1, "👨🏿‍❤️‍👨🏻", "", U_1F468_1F3FF_200D_2764_FE0F_200D_1F468_1F3FB, "1f468-1f3ff-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_MM_TONE5_2, "👨🏿‍❤️‍👨🏼", "", U_1F468_1F3FF_200D_2764_FE0F_200D_1F468_1F3FC, "1f468-1f3ff-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_MM_TONE5_3, "👨🏿‍❤️‍👨🏽", "", U_1F468_1F3FF_200D_2764_FE0F_200D_1F468_1F3FD, "1f468-1f3ff-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_MM_TONE5_4, "👨🏿‍❤️‍👨🏾", "", U_1F468_1F3FF_200D_2764_FE0F_200D_1F468_1F3FE, "1f468-1f3ff-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_MM_TONE5, "👨🏿‍❤️‍👨🏿", "", U_1F468_1F3FF_200D_2764_FE0F_200D_1F468_1F3FF, "1f468-1f3ff-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(KISS_MM_TONE5_1, "👨🏿‍❤️‍💋‍👨🏻", "", U_1F468_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f468-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_MM_TONE5_2, "👨🏿‍❤️‍💋‍👨🏼", "", U_1F468_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f468-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_MM_TONE5_3, "👨🏿‍❤️‍💋‍👨🏽", "", U_1F468_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f468-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_MM_TONE5_4, "👨🏿‍❤️‍💋‍👨🏾", "", U_1F468_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f468-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_MM_TONE5, "👨🏿‍❤️‍💋‍👨🏿", "", U_1F468_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f468-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(MAN_TONE5, "👨🏿", "", U_1F468_1F3FF, "1f468-1f3ff.png");
png_name!(MAN_FARMER, "👨‍🌾", "man farmer", U_1F468_200D_1F33E, "1f468-200d-1f33e.png");
png_name!(MAN_COOK, "👨‍🍳", "man cook", U_1F468_200D_1F373, "1f468-200d-1f373.png");
png_name!(MAN_FEEDING_BABY, "👨‍🍼", "man feeding baby", U_1F468_200D_1F37C, "1f468-200d-1f37c.png");
png_name!(MAN_STUDENT, "👨‍🎓", "man student", U_1F468_200D_1F393, "1f468-200d-1f393.png");
png_name!(MAN_SINGER, "👨‍🎤", "man singer", U_1F468_200D_1F3A4, "1f468-200d-1f3a4.png");
png_name!(MAN_ARTIST, "👨‍🎨", "man artist", U_1F468_200D_1F3A8, "1f468-200d-1f3a8.png");
png_name!(MAN_TEACHER, "👨‍🏫", "man teacher", U_1F468_200D_1F3EB, "1f468-200d-1f3eb.png");
png_name!(MAN_FACTORY_WORKER, "👨‍🏭", "man factory worker", U_1F468_200D_1F3ED, "1f468-200d-1f3ed.png");
png_name!(FAMILY_MBB, "👨‍👦‍👦", "family: man, boy, boy", U_1F468_200D_1F466_200D_1F466, "1f468-200d-1f466-200d-1f466.png");
png_name!(FAMILY_MB, "👨‍👦", "family: man, boy", U_1F468_200D_1F466, "1f468-200d-1f466.png");
png_name!(FAMILY_MGB, "👨‍👧‍👦", "family: man, girl, boy", U_1F468_200D_1F467_200D_1F466, "1f468-200d-1f467-200d-1f466.png");
png_name!(FAMILY_MGG, "👨‍👧‍👧", "family: man, girl, girl", U_1F468_200D_1F467_200D_1F467, "1f468-200d-1f467-200d-1f467.png");
png_name!(FAMILY_MG, "👨‍👧", "family: man, girl", U_1F468_200D_1F467, "1f468-200d-1f467.png");
png_name!(FAMILY_MMBB, "👨‍👨‍👦‍👦", "family: man, man, boy, boy", U_1F468_200D_1F468_200D_1F466_200D_1F466, "1f468-200d-1f468-200d-1f466-200d-1f466.png");
png_name!(FAMILY_MMB, "👨‍👨‍👦", "family: man, man, boy", U_1F468_200D_1F468_200D_1F466, "1f468-200d-1f468-200d-1f466.png");
png_name!(FAMILY_MMGB, "👨‍👨‍👧‍👦", "family: man, man, girl, boy", U_1F468_200D_1F468_200D_1F467_200D_1F466, "1f468-200d-1f468-200d-1f467-200d-1f466.png");
png_name!(FAMILY_MMGG, "👨‍👨‍👧‍👧", "family: man, man, girl, girl", U_1F468_200D_1F468_200D_1F467_200D_1F467, "1f468-200d-1f468-200d-1f467-200d-1f467.png");
png_name!(FAMILY_MMG, "👨‍👨‍👧", "family: man, man, girl", U_1F468_200D_1F468_200D_1F467, "1f468-200d-1f468-200d-1f467.png");
png_name!(FAMILY_MWBB, "👨‍👩‍👦‍👦", "family: man, woman, boy, boy", U_1F468_200D_1F469_200D_1F466_200D_1F466, "1f468-200d-1f469-200d-1f466-200d-1f466.png");
png_name!(FAMILY_MWB, "👨‍👩‍👦", "family: man, woman, boy", U_1F468_200D_1F469_200D_1F466, "1f468-200d-1f469-200d-1f466.png");
png_name!(FAMILY_MWGB, "👨‍👩‍👧‍👦", "family: man, woman, girl, boy", U_1F468_200D_1F469_200D_1F467_200D_1F466, "1f468-200d-1f469-200d-1f467-200d-1f466.png");
png_name!(FAMILY_MWGG, "👨‍👩‍👧‍👧", "family: man, woman, girl, girl", U_1F468_200D_1F469_200D_1F467_200D_1F467, "1f468-200d-1f469-200d-1f467-200d-1f467.png");
png_name!(FAMILY_MWG, "👨‍👩‍👧", "family: man, woman, girl", U_1F468_200D_1F469_200D_1F467, "1f468-200d-1f469-200d-1f467.png");
png_name!(MAN_TECHNOLOGIST, "👨‍💻", "man technologist", U_1F468_200D_1F4BB, "1f468-200d-1f4bb.png");
png_name!(MAN_OFFICE_WORKER, "👨‍💼", "man office worker", U_1F468_200D_1F4BC, "1f468-200d-1f4bc.png");
png_name!(MAN_MECHANIC, "👨‍🔧", "man mechanic", U_1F468_200D_1F527, "1f468-200d-1f527.png");
png_name!(MAN_SCIENTIST, "👨‍🔬", "man scientist", U_1F468_200D_1F52C, "1f468-200d-1f52c.png");
png_name!(MAN_ASTRONAUT, "👨‍🚀", "man astronaut", U_1F468_200D_1F680, "1f468-200d-1f680.png");
png_name!(MAN_FIREFIGHTER, "👨‍🚒", "man firefighter", U_1F468_200D_1F692, "1f468-200d-1f692.png");
png_name!(MAN_WITH_PROBING_CANE, "👨‍🦯", "man with white cane", U_1F468_200D_1F9AF, "1f468-200d-1f9af.png");
png_name!(MAN_WITH_WHITE_CANE, "👨‍🦯", "man with white cane", U_1F468_200D_1F9AF, "1f468-200d-1f9af.png");
png_name!(MAN_RED_HAIRED, "👨‍🦰", "man: red hair", U_1F468_200D_1F9B0, "1f468-200d-1f9b0.png");
png_name!(MAN_CURLY_HAIRED, "👨‍🦱", "man: curly hair", U_1F468_200D_1F9B1, "1f468-200d-1f9b1.png");
png_name!(MAN_BALD, "👨‍🦲", "man: bald", U_1F468_200D_1F9B2, "1f468-200d-1f9b2.png");
png_name!(MAN_WHITE_HAIRED, "👨‍🦳", "man: white hair", U_1F468_200D_1F9B3, "1f468-200d-1f9b3.png");
png_name!(MAN_IN_MOTORIZED_WHEELCHAIR, "👨‍🦼", "man in motorized wheelchair", U_1F468_200D_1F9BC, "1f468-200d-1f9bc.png");
png_name!(MAN_IN_MANUAL_WHEELCHAIR, "👨‍🦽", "man in manual wheelchair", U_1F468_200D_1F9BD, "1f468-200d-1f9bd.png");
png_name!(MAN_HEALTH_WORKER, "👨‍⚕️", "man health worker", U_1F468_200D_2695_FE0F, "1f468-200d-2695-fe0f.png");
png_name!(MAN_JUDGE, "👨‍⚖️", "man judge", U_1F468_200D_2696_FE0F, "1f468-200d-2696-fe0f.png");
png_name!(MAN_PILOT, "👨‍✈️", "man pilot", U_1F468_200D_2708_FE0F, "1f468-200d-2708-fe0f.png");
png_name!(COUPLE_WITH_HEART_MM, "👨‍❤️‍👨", "couple with heart: man, man", U_1F468_200D_2764_FE0F_200D_1F468, "1f468-200d-2764-fe0f-200d-1f468.png");
png_name!(KISS_MM, "👨‍❤️‍💋‍👨", "kiss: man, man", U_1F468_200D_2764_FE0F_200D_1F48B_200D_1F468, "1f468-200d-2764-fe0f-200d-1f48b-200d-1f468.png");
png_name!(MAN, "👨", "man", U_1F468, "1f468.png");
png_name!(WOMAN_FARMER_TONE1, "👩🏻‍🌾", "", U_1F469_1F3FB_200D_1F33E, "1f469-1f3fb-200d-1f33e.png");
png_name!(WOMAN_COOK_TONE1, "👩🏻‍🍳", "", U_1F469_1F3FB_200D_1F373, "1f469-1f3fb-200d-1f373.png");
png_name!(WOMAN_FEEDING_BABY_TONE1, "👩🏻‍🍼", "", U_1F469_1F3FB_200D_1F37C, "1f469-1f3fb-200d-1f37c.png");
png_name!(WOMAN_STUDENT_TONE1, "👩🏻‍🎓", "", U_1F469_1F3FB_200D_1F393, "1f469-1f3fb-200d-1f393.png");
png_name!(WOMAN_SINGER_TONE1, "👩🏻‍🎤", "", U_1F469_1F3FB_200D_1F3A4, "1f469-1f3fb-200d-1f3a4.png");
png_name!(WOMAN_ARTIST_TONE1, "👩🏻‍🎨", "", U_1F469_1F3FB_200D_1F3A8, "1f469-1f3fb-200d-1f3a8.png");
png_name!(WOMAN_TEACHER_TONE1, "👩🏻‍🏫", "", U_1F469_1F3FB_200D_1F3EB, "1f469-1f3fb-200d-1f3eb.png");
png_name!(WOMAN_FACTORY_WORKER_TONE1, "👩🏻‍🏭", "", U_1F469_1F3FB_200D_1F3ED, "1f469-1f3fb-200d-1f3ed.png");
png_name!(WOMAN_TECHNOLOGIST_TONE1, "👩🏻‍💻", "", U_1F469_1F3FB_200D_1F4BB, "1f469-1f3fb-200d-1f4bb.png");
png_name!(WOMAN_OFFICE_WORKER_TONE1, "👩🏻‍💼", "", U_1F469_1F3FB_200D_1F4BC, "1f469-1f3fb-200d-1f4bc.png");
png_name!(WOMAN_MECHANIC_TONE1, "👩🏻‍🔧", "", U_1F469_1F3FB_200D_1F527, "1f469-1f3fb-200d-1f527.png");
png_name!(WOMAN_SCIENTIST_TONE1, "👩🏻‍🔬", "", U_1F469_1F3FB_200D_1F52C, "1f469-1f3fb-200d-1f52c.png");
png_name!(WOMAN_ASTRONAUT_TONE1, "👩🏻‍🚀", "", U_1F469_1F3FB_200D_1F680, "1f469-1f3fb-200d-1f680.png");
png_name!(WOMAN_FIREFIGHTER_TONE1, "👩🏻‍🚒", "", U_1F469_1F3FB_200D_1F692, "1f469-1f3fb-200d-1f692.png");
png_name!(COUPLE_TONE1_2, "👩🏻‍🤝‍👨🏼", "", U_1F469_1F3FB_200D_1F91D_200D_1F468_1F3FC, "1f469-1f3fb-200d-1f91d-200d-1f468-1f3fc.png");
png_name!(COUPLE_TONE1_3, "👩🏻‍🤝‍👨🏽", "", U_1F469_1F3FB_200D_1F91D_200D_1F468_1F3FD, "1f469-1f3fb-200d-1f91d-200d-1f468-1f3fd.png");
png_name!(COUPLE_TONE1_4, "👩🏻‍🤝‍👨🏾", "", U_1F469_1F3FB_200D_1F91D_200D_1F468_1F3FE, "1f469-1f3fb-200d-1f91d-200d-1f468-1f3fe.png");
png_name!(COUPLE_TONE1_5, "👩🏻‍🤝‍👨🏿", "", U_1F469_1F3FB_200D_1F91D_200D_1F468_1F3FF, "1f469-1f3fb-200d-1f91d-200d-1f468-1f3ff.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE1_2, "👩🏻‍🤝‍👩🏼", "", U_1F469_1F3FB_200D_1F91D_200D_1F469_1F3FC, "1f469-1f3fb-200d-1f91d-200d-1f469-1f3fc.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE1_3, "👩🏻‍🤝‍👩🏽", "", U_1F469_1F3FB_200D_1F91D_200D_1F469_1F3FD, "1f469-1f3fb-200d-1f91d-200d-1f469-1f3fd.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE1_4, "👩🏻‍🤝‍👩🏾", "", U_1F469_1F3FB_200D_1F91D_200D_1F469_1F3FE, "1f469-1f3fb-200d-1f91d-200d-1f469-1f3fe.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE1_5, "👩🏻‍🤝‍👩🏿", "", U_1F469_1F3FB_200D_1F91D_200D_1F469_1F3FF, "1f469-1f3fb-200d-1f91d-200d-1f469-1f3ff.png");
png_name!(WOMAN_WITH_PROBING_CANE_TONE1, "👩🏻‍🦯", "", U_1F469_1F3FB_200D_1F9AF, "1f469-1f3fb-200d-1f9af.png");
png_name!(WOMAN_WITH_WHITE_CANE_TONE1, "👩🏻‍🦯", "", U_1F469_1F3FB_200D_1F9AF, "1f469-1f3fb-200d-1f9af.png");
png_name!(WOMAN_RED_HAIRED_TONE1, "👩🏻‍🦰", "", U_1F469_1F3FB_200D_1F9B0, "1f469-1f3fb-200d-1f9b0.png");
png_name!(WOMAN_CURLY_HAIRED_TONE1, "👩🏻‍🦱", "", U_1F469_1F3FB_200D_1F9B1, "1f469-1f3fb-200d-1f9b1.png");
png_name!(WOMAN_BALD_TONE1, "👩🏻‍🦲", "", U_1F469_1F3FB_200D_1F9B2, "1f469-1f3fb-200d-1f9b2.png");
png_name!(WOMAN_WHITE_HAIRED_TONE1, "👩🏻‍🦳", "", U_1F469_1F3FB_200D_1F9B3, "1f469-1f3fb-200d-1f9b3.png");
png_name!(WOMAN_IN_MOTORIZED_WHEELCHAIR_TONE1, "👩🏻‍🦼", "", U_1F469_1F3FB_200D_1F9BC, "1f469-1f3fb-200d-1f9bc.png");
png_name!(WOMAN_IN_MANUAL_WHEELCHAIR_TONE1, "👩🏻‍🦽", "", U_1F469_1F3FB_200D_1F9BD, "1f469-1f3fb-200d-1f9bd.png");
png_name!(WOMAN_HEALTH_WORKER_TONE1, "👩🏻‍⚕️", "", U_1F469_1F3FB_200D_2695_FE0F, "1f469-1f3fb-200d-2695-fe0f.png");
png_name!(WOMAN_JUDGE_TONE1, "👩🏻‍⚖️", "", U_1F469_1F3FB_200D_2696_FE0F, "1f469-1f3fb-200d-2696-fe0f.png");
png_name!(WOMAN_PILOT_TONE1, "👩🏻‍✈️", "", U_1F469_1F3FB_200D_2708_FE0F, "1f469-1f3fb-200d-2708-fe0f.png");
png_name!(COUPLE_WITH_HEART_MW_TONE1, "👩🏻‍❤️‍👨🏻", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FB, "1f469-1f3fb-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_WM_TONE1, "👩🏻‍❤️‍👨🏻", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FB, "1f469-1f3fb-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_MW_TONE1_2, "👩🏻‍❤️‍👨🏼", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FC, "1f469-1f3fb-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_WM_TONE1_2, "👩🏻‍❤️‍👨🏼", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FC, "1f469-1f3fb-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_MW_TONE1_3, "👩🏻‍❤️‍👨🏽", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FD, "1f469-1f3fb-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_WM_TONE1_3, "👩🏻‍❤️‍👨🏽", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FD, "1f469-1f3fb-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_MW_TONE1_4, "👩🏻‍❤️‍👨🏾", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FE, "1f469-1f3fb-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_WM_TONE1_4, "👩🏻‍❤️‍👨🏾", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FE, "1f469-1f3fb-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_MW_TONE1_5, "👩🏻‍❤️‍👨🏿", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FF, "1f469-1f3fb-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(COUPLE_WITH_HEART_WM_TONE1_5, "👩🏻‍❤️‍👨🏿", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FF, "1f469-1f3fb-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(COUPLE_WITH_HEART_WW_TONE1, "👩🏻‍❤️‍👩🏻", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F469_1F3FB, "1f469-1f3fb-200d-2764-fe0f-200d-1f469-1f3fb.png");
png_name!(COUPLE_WITH_HEART_WW_TONE1_2, "👩🏻‍❤️‍👩🏼", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F469_1F3FC, "1f469-1f3fb-200d-2764-fe0f-200d-1f469-1f3fc.png");
png_name!(COUPLE_WITH_HEART_WW_TONE1_3, "👩🏻‍❤️‍👩🏽", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F469_1F3FD, "1f469-1f3fb-200d-2764-fe0f-200d-1f469-1f3fd.png");
png_name!(COUPLE_WITH_HEART_WW_TONE1_4, "👩🏻‍❤️‍👩🏾", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F469_1F3FE, "1f469-1f3fb-200d-2764-fe0f-200d-1f469-1f3fe.png");
png_name!(COUPLE_WITH_HEART_WW_TONE1_5, "👩🏻‍❤️‍👩🏿", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F469_1F3FF, "1f469-1f3fb-200d-2764-fe0f-200d-1f469-1f3ff.png");
png_name!(KISS_MW_TONE1, "👩🏻‍❤️‍💋‍👨🏻", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_WM_TONE1, "👩🏻‍❤️‍💋‍👨🏻", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_MW_TONE1_2, "👩🏻‍❤️‍💋‍👨🏼", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_WM_TONE1_2, "👩🏻‍❤️‍💋‍👨🏼", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_MW_TONE1_3, "👩🏻‍❤️‍💋‍👨🏽", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_WM_TONE1_3, "👩🏻‍❤️‍💋‍👨🏽", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_MW_TONE1_4, "👩🏻‍❤️‍💋‍👨🏾", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_WM_TONE1_4, "👩🏻‍❤️‍💋‍👨🏾", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_MW_TONE1_5, "👩🏻‍❤️‍💋‍👨🏿", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(KISS_WM_TONE1_5, "👩🏻‍❤️‍💋‍👨🏿", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(KISS_WW_TONE1, "👩🏻‍❤️‍💋‍👩🏻", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FB, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fb.png");
png_name!(KISS_WW_TONE1_2, "👩🏻‍❤️‍💋‍👩🏼", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FC, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fc.png");
png_name!(KISS_WW_TONE1_3, "👩🏻‍❤️‍💋‍👩🏽", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FD, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fd.png");
png_name!(KISS_WW_TONE1_4, "👩🏻‍❤️‍💋‍👩🏾", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FE, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fe.png");
png_name!(KISS_WW_TONE1_5, "👩🏻‍❤️‍💋‍👩🏿", "", U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FF, "1f469-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3ff.png");
png_name!(WOMAN_TONE1, "👩🏻", "", U_1F469_1F3FB, "1f469-1f3fb.png");
png_name!(WOMAN_FARMER_TONE2, "👩🏼‍🌾", "", U_1F469_1F3FC_200D_1F33E, "1f469-1f3fc-200d-1f33e.png");
png_name!(WOMAN_COOK_TONE2, "👩🏼‍🍳", "", U_1F469_1F3FC_200D_1F373, "1f469-1f3fc-200d-1f373.png");
png_name!(WOMAN_FEEDING_BABY_TONE2, "👩🏼‍🍼", "", U_1F469_1F3FC_200D_1F37C, "1f469-1f3fc-200d-1f37c.png");
png_name!(WOMAN_STUDENT_TONE2, "👩🏼‍🎓", "", U_1F469_1F3FC_200D_1F393, "1f469-1f3fc-200d-1f393.png");
png_name!(WOMAN_SINGER_TONE2, "👩🏼‍🎤", "", U_1F469_1F3FC_200D_1F3A4, "1f469-1f3fc-200d-1f3a4.png");
png_name!(WOMAN_ARTIST_TONE2, "👩🏼‍🎨", "", U_1F469_1F3FC_200D_1F3A8, "1f469-1f3fc-200d-1f3a8.png");
png_name!(WOMAN_TEACHER_TONE2, "👩🏼‍🏫", "", U_1F469_1F3FC_200D_1F3EB, "1f469-1f3fc-200d-1f3eb.png");
png_name!(WOMAN_FACTORY_WORKER_TONE2, "👩🏼‍🏭", "", U_1F469_1F3FC_200D_1F3ED, "1f469-1f3fc-200d-1f3ed.png");
png_name!(WOMAN_TECHNOLOGIST_TONE2, "👩🏼‍💻", "", U_1F469_1F3FC_200D_1F4BB, "1f469-1f3fc-200d-1f4bb.png");
png_name!(WOMAN_OFFICE_WORKER_TONE2, "👩🏼‍💼", "", U_1F469_1F3FC_200D_1F4BC, "1f469-1f3fc-200d-1f4bc.png");
png_name!(WOMAN_MECHANIC_TONE2, "👩🏼‍🔧", "", U_1F469_1F3FC_200D_1F527, "1f469-1f3fc-200d-1f527.png");
png_name!(WOMAN_SCIENTIST_TONE2, "👩🏼‍🔬", "", U_1F469_1F3FC_200D_1F52C, "1f469-1f3fc-200d-1f52c.png");
png_name!(WOMAN_ASTRONAUT_TONE2, "👩🏼‍🚀", "", U_1F469_1F3FC_200D_1F680, "1f469-1f3fc-200d-1f680.png");
png_name!(WOMAN_FIREFIGHTER_TONE2, "👩🏼‍🚒", "", U_1F469_1F3FC_200D_1F692, "1f469-1f3fc-200d-1f692.png");
png_name!(COUPLE_TONE2_1, "👩🏼‍🤝‍👨🏻", "", U_1F469_1F3FC_200D_1F91D_200D_1F468_1F3FB, "1f469-1f3fc-200d-1f91d-200d-1f468-1f3fb.png");
png_name!(COUPLE_TONE2_3, "👩🏼‍🤝‍👨🏽", "", U_1F469_1F3FC_200D_1F91D_200D_1F468_1F3FD, "1f469-1f3fc-200d-1f91d-200d-1f468-1f3fd.png");
png_name!(COUPLE_TONE2_4, "👩🏼‍🤝‍👨🏾", "", U_1F469_1F3FC_200D_1F91D_200D_1F468_1F3FE, "1f469-1f3fc-200d-1f91d-200d-1f468-1f3fe.png");
png_name!(COUPLE_TONE2_5, "👩🏼‍🤝‍👨🏿", "", U_1F469_1F3FC_200D_1F91D_200D_1F468_1F3FF, "1f469-1f3fc-200d-1f91d-200d-1f468-1f3ff.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE2_1, "👩🏼‍🤝‍👩🏻", "", U_1F469_1F3FC_200D_1F91D_200D_1F469_1F3FB, "1f469-1f3fc-200d-1f91d-200d-1f469-1f3fb.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE2_3, "👩🏼‍🤝‍👩🏽", "", U_1F469_1F3FC_200D_1F91D_200D_1F469_1F3FD, "1f469-1f3fc-200d-1f91d-200d-1f469-1f3fd.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE2_4, "👩🏼‍🤝‍👩🏾", "", U_1F469_1F3FC_200D_1F91D_200D_1F469_1F3FE, "1f469-1f3fc-200d-1f91d-200d-1f469-1f3fe.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE2_5, "👩🏼‍🤝‍👩🏿", "", U_1F469_1F3FC_200D_1F91D_200D_1F469_1F3FF, "1f469-1f3fc-200d-1f91d-200d-1f469-1f3ff.png");
png_name!(WOMAN_WITH_PROBING_CANE_TONE2, "👩🏼‍🦯", "", U_1F469_1F3FC_200D_1F9AF, "1f469-1f3fc-200d-1f9af.png");
png_name!(WOMAN_WITH_WHITE_CANE_TONE2, "👩🏼‍🦯", "", U_1F469_1F3FC_200D_1F9AF, "1f469-1f3fc-200d-1f9af.png");
png_name!(WOMAN_RED_HAIRED_TONE2, "👩🏼‍🦰", "", U_1F469_1F3FC_200D_1F9B0, "1f469-1f3fc-200d-1f9b0.png");
png_name!(WOMAN_CURLY_HAIRED_TONE2, "👩🏼‍🦱", "", U_1F469_1F3FC_200D_1F9B1, "1f469-1f3fc-200d-1f9b1.png");
png_name!(WOMAN_BALD_TONE2, "👩🏼‍🦲", "", U_1F469_1F3FC_200D_1F9B2, "1f469-1f3fc-200d-1f9b2.png");
png_name!(WOMAN_WHITE_HAIRED_TONE2, "👩🏼‍🦳", "", U_1F469_1F3FC_200D_1F9B3, "1f469-1f3fc-200d-1f9b3.png");
png_name!(WOMAN_IN_MOTORIZED_WHEELCHAIR_TONE2, "👩🏼‍🦼", "", U_1F469_1F3FC_200D_1F9BC, "1f469-1f3fc-200d-1f9bc.png");
png_name!(WOMAN_IN_MANUAL_WHEELCHAIR_TONE2, "👩🏼‍🦽", "", U_1F469_1F3FC_200D_1F9BD, "1f469-1f3fc-200d-1f9bd.png");
png_name!(WOMAN_HEALTH_WORKER_TONE2, "👩🏼‍⚕️", "", U_1F469_1F3FC_200D_2695_FE0F, "1f469-1f3fc-200d-2695-fe0f.png");
png_name!(WOMAN_JUDGE_TONE2, "👩🏼‍⚖️", "", U_1F469_1F3FC_200D_2696_FE0F, "1f469-1f3fc-200d-2696-fe0f.png");
png_name!(WOMAN_PILOT_TONE2, "👩🏼‍✈️", "", U_1F469_1F3FC_200D_2708_FE0F, "1f469-1f3fc-200d-2708-fe0f.png");
png_name!(COUPLE_WITH_HEART_MW_TONE2_1, "👩🏼‍❤️‍👨🏻", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FB, "1f469-1f3fc-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_WM_TONE2_1, "👩🏼‍❤️‍👨🏻", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FB, "1f469-1f3fc-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_MW_TONE2, "👩🏼‍❤️‍👨🏼", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FC, "1f469-1f3fc-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_WM_TONE2, "👩🏼‍❤️‍👨🏼", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FC, "1f469-1f3fc-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_MW_TONE2_3, "👩🏼‍❤️‍👨🏽", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FD, "1f469-1f3fc-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_WM_TONE2_3, "👩🏼‍❤️‍👨🏽", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FD, "1f469-1f3fc-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_MW_TONE2_4, "👩🏼‍❤️‍👨🏾", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FE, "1f469-1f3fc-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_WM_TONE2_4, "👩🏼‍❤️‍👨🏾", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FE, "1f469-1f3fc-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_MW_TONE2_5, "👩🏼‍❤️‍👨🏿", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FF, "1f469-1f3fc-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(COUPLE_WITH_HEART_WM_TONE2_5, "👩🏼‍❤️‍👨🏿", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FF, "1f469-1f3fc-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(COUPLE_WITH_HEART_WW_TONE2_1, "👩🏼‍❤️‍👩🏻", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F469_1F3FB, "1f469-1f3fc-200d-2764-fe0f-200d-1f469-1f3fb.png");
png_name!(COUPLE_WITH_HEART_WW_TONE2, "👩🏼‍❤️‍👩🏼", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F469_1F3FC, "1f469-1f3fc-200d-2764-fe0f-200d-1f469-1f3fc.png");
png_name!(COUPLE_WITH_HEART_WW_TONE2_3, "👩🏼‍❤️‍👩🏽", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F469_1F3FD, "1f469-1f3fc-200d-2764-fe0f-200d-1f469-1f3fd.png");
png_name!(COUPLE_WITH_HEART_WW_TONE2_4, "👩🏼‍❤️‍👩🏾", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F469_1F3FE, "1f469-1f3fc-200d-2764-fe0f-200d-1f469-1f3fe.png");
png_name!(COUPLE_WITH_HEART_WW_TONE2_5, "👩🏼‍❤️‍👩🏿", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F469_1F3FF, "1f469-1f3fc-200d-2764-fe0f-200d-1f469-1f3ff.png");
png_name!(KISS_MW_TONE2_1, "👩🏼‍❤️‍💋‍👨🏻", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_WM_TONE2_1, "👩🏼‍❤️‍💋‍👨🏻", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_MW_TONE2, "👩🏼‍❤️‍💋‍👨🏼", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_WM_TONE2, "👩🏼‍❤️‍💋‍👨🏼", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_MW_TONE2_3, "👩🏼‍❤️‍💋‍👨🏽", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_WM_TONE2_3, "👩🏼‍❤️‍💋‍👨🏽", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_MW_TONE2_4, "👩🏼‍❤️‍💋‍👨🏾", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_WM_TONE2_4, "👩🏼‍❤️‍💋‍👨🏾", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_MW_TONE2_5, "👩🏼‍❤️‍💋‍👨🏿", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(KISS_WM_TONE2_5, "👩🏼‍❤️‍💋‍👨🏿", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(KISS_WW_TONE2_1, "👩🏼‍❤️‍💋‍👩🏻", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FB, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fb.png");
png_name!(KISS_WW_TONE2, "👩🏼‍❤️‍💋‍👩🏼", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FC, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fc.png");
png_name!(KISS_WW_TONE2_3, "👩🏼‍❤️‍💋‍👩🏽", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FD, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fd.png");
png_name!(KISS_WW_TONE2_4, "👩🏼‍❤️‍💋‍👩🏾", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FE, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fe.png");
png_name!(KISS_WW_TONE2_5, "👩🏼‍❤️‍💋‍👩🏿", "", U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FF, "1f469-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3ff.png");
png_name!(WOMAN_TONE2, "👩🏼", "", U_1F469_1F3FC, "1f469-1f3fc.png");
png_name!(WOMAN_FARMER_TONE3, "👩🏽‍🌾", "", U_1F469_1F3FD_200D_1F33E, "1f469-1f3fd-200d-1f33e.png");
png_name!(WOMAN_COOK_TONE3, "👩🏽‍🍳", "", U_1F469_1F3FD_200D_1F373, "1f469-1f3fd-200d-1f373.png");
png_name!(WOMAN_FEEDING_BABY_TONE3, "👩🏽‍🍼", "", U_1F469_1F3FD_200D_1F37C, "1f469-1f3fd-200d-1f37c.png");
png_name!(WOMAN_STUDENT_TONE3, "👩🏽‍🎓", "", U_1F469_1F3FD_200D_1F393, "1f469-1f3fd-200d-1f393.png");
png_name!(WOMAN_SINGER_TONE3, "👩🏽‍🎤", "", U_1F469_1F3FD_200D_1F3A4, "1f469-1f3fd-200d-1f3a4.png");
png_name!(WOMAN_ARTIST_TONE3, "👩🏽‍🎨", "", U_1F469_1F3FD_200D_1F3A8, "1f469-1f3fd-200d-1f3a8.png");
png_name!(WOMAN_TEACHER_TONE3, "👩🏽‍🏫", "", U_1F469_1F3FD_200D_1F3EB, "1f469-1f3fd-200d-1f3eb.png");
png_name!(WOMAN_FACTORY_WORKER_TONE3, "👩🏽‍🏭", "", U_1F469_1F3FD_200D_1F3ED, "1f469-1f3fd-200d-1f3ed.png");
png_name!(WOMAN_TECHNOLOGIST_TONE3, "👩🏽‍💻", "", U_1F469_1F3FD_200D_1F4BB, "1f469-1f3fd-200d-1f4bb.png");
png_name!(WOMAN_OFFICE_WORKER_TONE3, "👩🏽‍💼", "", U_1F469_1F3FD_200D_1F4BC, "1f469-1f3fd-200d-1f4bc.png");
png_name!(WOMAN_MECHANIC_TONE3, "👩🏽‍🔧", "", U_1F469_1F3FD_200D_1F527, "1f469-1f3fd-200d-1f527.png");
png_name!(WOMAN_SCIENTIST_TONE3, "👩🏽‍🔬", "", U_1F469_1F3FD_200D_1F52C, "1f469-1f3fd-200d-1f52c.png");
png_name!(WOMAN_ASTRONAUT_TONE3, "👩🏽‍🚀", "", U_1F469_1F3FD_200D_1F680, "1f469-1f3fd-200d-1f680.png");
png_name!(WOMAN_FIREFIGHTER_TONE3, "👩🏽‍🚒", "", U_1F469_1F3FD_200D_1F692, "1f469-1f3fd-200d-1f692.png");
png_name!(COUPLE_TONE3_1, "👩🏽‍🤝‍👨🏻", "", U_1F469_1F3FD_200D_1F91D_200D_1F468_1F3FB, "1f469-1f3fd-200d-1f91d-200d-1f468-1f3fb.png");
png_name!(COUPLE_TONE3_2, "👩🏽‍🤝‍👨🏼", "", U_1F469_1F3FD_200D_1F91D_200D_1F468_1F3FC, "1f469-1f3fd-200d-1f91d-200d-1f468-1f3fc.png");
png_name!(COUPLE_TONE3_4, "👩🏽‍🤝‍👨🏾", "", U_1F469_1F3FD_200D_1F91D_200D_1F468_1F3FE, "1f469-1f3fd-200d-1f91d-200d-1f468-1f3fe.png");
png_name!(COUPLE_TONE3_5, "👩🏽‍🤝‍👨🏿", "", U_1F469_1F3FD_200D_1F91D_200D_1F468_1F3FF, "1f469-1f3fd-200d-1f91d-200d-1f468-1f3ff.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE3_1, "👩🏽‍🤝‍👩🏻", "", U_1F469_1F3FD_200D_1F91D_200D_1F469_1F3FB, "1f469-1f3fd-200d-1f91d-200d-1f469-1f3fb.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE3_2, "👩🏽‍🤝‍👩🏼", "", U_1F469_1F3FD_200D_1F91D_200D_1F469_1F3FC, "1f469-1f3fd-200d-1f91d-200d-1f469-1f3fc.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE3_4, "👩🏽‍🤝‍👩🏾", "", U_1F469_1F3FD_200D_1F91D_200D_1F469_1F3FE, "1f469-1f3fd-200d-1f91d-200d-1f469-1f3fe.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE3_5, "👩🏽‍🤝‍👩🏿", "", U_1F469_1F3FD_200D_1F91D_200D_1F469_1F3FF, "1f469-1f3fd-200d-1f91d-200d-1f469-1f3ff.png");
png_name!(WOMAN_WITH_PROBING_CANE_TONE3, "👩🏽‍🦯", "", U_1F469_1F3FD_200D_1F9AF, "1f469-1f3fd-200d-1f9af.png");
png_name!(WOMAN_WITH_WHITE_CANE_TONE3, "👩🏽‍🦯", "", U_1F469_1F3FD_200D_1F9AF, "1f469-1f3fd-200d-1f9af.png");
png_name!(WOMAN_RED_HAIRED_TONE3, "👩🏽‍🦰", "", U_1F469_1F3FD_200D_1F9B0, "1f469-1f3fd-200d-1f9b0.png");
png_name!(WOMAN_CURLY_HAIRED_TONE3, "👩🏽‍🦱", "", U_1F469_1F3FD_200D_1F9B1, "1f469-1f3fd-200d-1f9b1.png");
png_name!(WOMAN_BALD_TONE3, "👩🏽‍🦲", "", U_1F469_1F3FD_200D_1F9B2, "1f469-1f3fd-200d-1f9b2.png");
png_name!(WOMAN_WHITE_HAIRED_TONE3, "👩🏽‍🦳", "", U_1F469_1F3FD_200D_1F9B3, "1f469-1f3fd-200d-1f9b3.png");
png_name!(WOMAN_IN_MOTORIZED_WHEELCHAIR_TONE3, "👩🏽‍🦼", "", U_1F469_1F3FD_200D_1F9BC, "1f469-1f3fd-200d-1f9bc.png");
png_name!(WOMAN_IN_MANUAL_WHEELCHAIR_TONE3, "👩🏽‍🦽", "", U_1F469_1F3FD_200D_1F9BD, "1f469-1f3fd-200d-1f9bd.png");
png_name!(WOMAN_HEALTH_WORKER_TONE3, "👩🏽‍⚕️", "", U_1F469_1F3FD_200D_2695_FE0F, "1f469-1f3fd-200d-2695-fe0f.png");
png_name!(WOMAN_JUDGE_TONE3, "👩🏽‍⚖️", "", U_1F469_1F3FD_200D_2696_FE0F, "1f469-1f3fd-200d-2696-fe0f.png");
png_name!(WOMAN_PILOT_TONE3, "👩🏽‍✈️", "", U_1F469_1F3FD_200D_2708_FE0F, "1f469-1f3fd-200d-2708-fe0f.png");
png_name!(COUPLE_WITH_HEART_MW_TONE3_1, "👩🏽‍❤️‍👨🏻", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FB, "1f469-1f3fd-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_WM_TONE3_1, "👩🏽‍❤️‍👨🏻", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FB, "1f469-1f3fd-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_MW_TONE3_2, "👩🏽‍❤️‍👨🏼", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FC, "1f469-1f3fd-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_WM_TONE3_2, "👩🏽‍❤️‍👨🏼", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FC, "1f469-1f3fd-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_MW_TONE3, "👩🏽‍❤️‍👨🏽", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FD, "1f469-1f3fd-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_WM_TONE3, "👩🏽‍❤️‍👨🏽", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FD, "1f469-1f3fd-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_MW_TONE3_4, "👩🏽‍❤️‍👨🏾", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FE, "1f469-1f3fd-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_WM_TONE3_4, "👩🏽‍❤️‍👨🏾", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FE, "1f469-1f3fd-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_MW_TONE3_5, "👩🏽‍❤️‍👨🏿", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FF, "1f469-1f3fd-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(COUPLE_WITH_HEART_WM_TONE3_5, "👩🏽‍❤️‍👨🏿", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FF, "1f469-1f3fd-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(COUPLE_WITH_HEART_WW_TONE3_1, "👩🏽‍❤️‍👩🏻", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F469_1F3FB, "1f469-1f3fd-200d-2764-fe0f-200d-1f469-1f3fb.png");
png_name!(COUPLE_WITH_HEART_WW_TONE3_2, "👩🏽‍❤️‍👩🏼", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F469_1F3FC, "1f469-1f3fd-200d-2764-fe0f-200d-1f469-1f3fc.png");
png_name!(COUPLE_WITH_HEART_WW_TONE3, "👩🏽‍❤️‍👩🏽", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F469_1F3FD, "1f469-1f3fd-200d-2764-fe0f-200d-1f469-1f3fd.png");
png_name!(COUPLE_WITH_HEART_WW_TONE3_4, "👩🏽‍❤️‍👩🏾", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F469_1F3FE, "1f469-1f3fd-200d-2764-fe0f-200d-1f469-1f3fe.png");
png_name!(COUPLE_WITH_HEART_WW_TONE3_5, "👩🏽‍❤️‍👩🏿", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F469_1F3FF, "1f469-1f3fd-200d-2764-fe0f-200d-1f469-1f3ff.png");
png_name!(KISS_MW_TONE3_1, "👩🏽‍❤️‍💋‍👨🏻", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_WM_TONE3_1, "👩🏽‍❤️‍💋‍👨🏻", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_MW_TONE3_2, "👩🏽‍❤️‍💋‍👨🏼", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_WM_TONE3_2, "👩🏽‍❤️‍💋‍👨🏼", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_MW_TONE3, "👩🏽‍❤️‍💋‍👨🏽", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_WM_TONE3, "👩🏽‍❤️‍💋‍👨🏽", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_MW_TONE3_4, "👩🏽‍❤️‍💋‍👨🏾", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_WM_TONE3_4, "👩🏽‍❤️‍💋‍👨🏾", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_MW_TONE3_5, "👩🏽‍❤️‍💋‍👨🏿", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(KISS_WM_TONE3_5, "👩🏽‍❤️‍💋‍👨🏿", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(KISS_WW_TONE3_1, "👩🏽‍❤️‍💋‍👩🏻", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FB, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fb.png");
png_name!(KISS_WW_TONE3_2, "👩🏽‍❤️‍💋‍👩🏼", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FC, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fc.png");
png_name!(KISS_WW_TONE3, "👩🏽‍❤️‍💋‍👩🏽", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FD, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fd.png");
png_name!(KISS_WW_TONE3_4, "👩🏽‍❤️‍💋‍👩🏾", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FE, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fe.png");
png_name!(KISS_WW_TONE3_5, "👩🏽‍❤️‍💋‍👩🏿", "", U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FF, "1f469-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3ff.png");
png_name!(WOMAN_TONE3, "👩🏽", "", U_1F469_1F3FD, "1f469-1f3fd.png");
png_name!(WOMAN_FARMER_TONE4, "👩🏾‍🌾", "", U_1F469_1F3FE_200D_1F33E, "1f469-1f3fe-200d-1f33e.png");
png_name!(WOMAN_COOK_TONE4, "👩🏾‍🍳", "", U_1F469_1F3FE_200D_1F373, "1f469-1f3fe-200d-1f373.png");
png_name!(WOMAN_FEEDING_BABY_TONE4, "👩🏾‍🍼", "", U_1F469_1F3FE_200D_1F37C, "1f469-1f3fe-200d-1f37c.png");
png_name!(WOMAN_STUDENT_TONE4, "👩🏾‍🎓", "", U_1F469_1F3FE_200D_1F393, "1f469-1f3fe-200d-1f393.png");
png_name!(WOMAN_SINGER_TONE4, "👩🏾‍🎤", "", U_1F469_1F3FE_200D_1F3A4, "1f469-1f3fe-200d-1f3a4.png");
png_name!(WOMAN_ARTIST_TONE4, "👩🏾‍🎨", "", U_1F469_1F3FE_200D_1F3A8, "1f469-1f3fe-200d-1f3a8.png");
png_name!(WOMAN_TEACHER_TONE4, "👩🏾‍🏫", "", U_1F469_1F3FE_200D_1F3EB, "1f469-1f3fe-200d-1f3eb.png");
png_name!(WOMAN_FACTORY_WORKER_TONE4, "👩🏾‍🏭", "", U_1F469_1F3FE_200D_1F3ED, "1f469-1f3fe-200d-1f3ed.png");
png_name!(WOMAN_TECHNOLOGIST_TONE4, "👩🏾‍💻", "", U_1F469_1F3FE_200D_1F4BB, "1f469-1f3fe-200d-1f4bb.png");
png_name!(WOMAN_OFFICE_WORKER_TONE4, "👩🏾‍💼", "", U_1F469_1F3FE_200D_1F4BC, "1f469-1f3fe-200d-1f4bc.png");
png_name!(WOMAN_MECHANIC_TONE4, "👩🏾‍🔧", "", U_1F469_1F3FE_200D_1F527, "1f469-1f3fe-200d-1f527.png");
png_name!(WOMAN_SCIENTIST_TONE4, "👩🏾‍🔬", "", U_1F469_1F3FE_200D_1F52C, "1f469-1f3fe-200d-1f52c.png");
png_name!(WOMAN_ASTRONAUT_TONE4, "👩🏾‍🚀", "", U_1F469_1F3FE_200D_1F680, "1f469-1f3fe-200d-1f680.png");
png_name!(WOMAN_FIREFIGHTER_TONE4, "👩🏾‍🚒", "", U_1F469_1F3FE_200D_1F692, "1f469-1f3fe-200d-1f692.png");
png_name!(COUPLE_TONE4_1, "👩🏾‍🤝‍👨🏻", "", U_1F469_1F3FE_200D_1F91D_200D_1F468_1F3FB, "1f469-1f3fe-200d-1f91d-200d-1f468-1f3fb.png");
png_name!(COUPLE_TONE4_2, "👩🏾‍🤝‍👨🏼", "", U_1F469_1F3FE_200D_1F91D_200D_1F468_1F3FC, "1f469-1f3fe-200d-1f91d-200d-1f468-1f3fc.png");
png_name!(COUPLE_TONE4_3, "👩🏾‍🤝‍👨🏽", "", U_1F469_1F3FE_200D_1F91D_200D_1F468_1F3FD, "1f469-1f3fe-200d-1f91d-200d-1f468-1f3fd.png");
png_name!(COUPLE_TONE4_5, "👩🏾‍🤝‍👨🏿", "", U_1F469_1F3FE_200D_1F91D_200D_1F468_1F3FF, "1f469-1f3fe-200d-1f91d-200d-1f468-1f3ff.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE4_1, "👩🏾‍🤝‍👩🏻", "", U_1F469_1F3FE_200D_1F91D_200D_1F469_1F3FB, "1f469-1f3fe-200d-1f91d-200d-1f469-1f3fb.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE4_2, "👩🏾‍🤝‍👩🏼", "", U_1F469_1F3FE_200D_1F91D_200D_1F469_1F3FC, "1f469-1f3fe-200d-1f91d-200d-1f469-1f3fc.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE4_3, "👩🏾‍🤝‍👩🏽", "", U_1F469_1F3FE_200D_1F91D_200D_1F469_1F3FD, "1f469-1f3fe-200d-1f91d-200d-1f469-1f3fd.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE4_5, "👩🏾‍🤝‍👩🏿", "", U_1F469_1F3FE_200D_1F91D_200D_1F469_1F3FF, "1f469-1f3fe-200d-1f91d-200d-1f469-1f3ff.png");
png_name!(WOMAN_WITH_PROBING_CANE_TONE4, "👩🏾‍🦯", "", U_1F469_1F3FE_200D_1F9AF, "1f469-1f3fe-200d-1f9af.png");
png_name!(WOMAN_WITH_WHITE_CANE_TONE4, "👩🏾‍🦯", "", U_1F469_1F3FE_200D_1F9AF, "1f469-1f3fe-200d-1f9af.png");
png_name!(WOMAN_RED_HAIRED_TONE4, "👩🏾‍🦰", "", U_1F469_1F3FE_200D_1F9B0, "1f469-1f3fe-200d-1f9b0.png");
png_name!(WOMAN_CURLY_HAIRED_TONE4, "👩🏾‍🦱", "", U_1F469_1F3FE_200D_1F9B1, "1f469-1f3fe-200d-1f9b1.png");
png_name!(WOMAN_BALD_TONE4, "👩🏾‍🦲", "", U_1F469_1F3FE_200D_1F9B2, "1f469-1f3fe-200d-1f9b2.png");
png_name!(WOMAN_WHITE_HAIRED_TONE4, "👩🏾‍🦳", "", U_1F469_1F3FE_200D_1F9B3, "1f469-1f3fe-200d-1f9b3.png");
png_name!(WOMAN_IN_MOTORIZED_WHEELCHAIR_TONE4, "👩🏾‍🦼", "", U_1F469_1F3FE_200D_1F9BC, "1f469-1f3fe-200d-1f9bc.png");
png_name!(WOMAN_IN_MANUAL_WHEELCHAIR_TONE4, "👩🏾‍🦽", "", U_1F469_1F3FE_200D_1F9BD, "1f469-1f3fe-200d-1f9bd.png");
png_name!(WOMAN_HEALTH_WORKER_TONE4, "👩🏾‍⚕️", "", U_1F469_1F3FE_200D_2695_FE0F, "1f469-1f3fe-200d-2695-fe0f.png");
png_name!(WOMAN_JUDGE_TONE4, "👩🏾‍⚖️", "", U_1F469_1F3FE_200D_2696_FE0F, "1f469-1f3fe-200d-2696-fe0f.png");
png_name!(WOMAN_PILOT_TONE4, "👩🏾‍✈️", "", U_1F469_1F3FE_200D_2708_FE0F, "1f469-1f3fe-200d-2708-fe0f.png");
png_name!(COUPLE_WITH_HEART_MW_TONE4_1, "👩🏾‍❤️‍👨🏻", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FB, "1f469-1f3fe-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_WM_TONE4_1, "👩🏾‍❤️‍👨🏻", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FB, "1f469-1f3fe-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_MW_TONE4_2, "👩🏾‍❤️‍👨🏼", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FC, "1f469-1f3fe-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_WM_TONE4_2, "👩🏾‍❤️‍👨🏼", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FC, "1f469-1f3fe-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_MW_TONE4_3, "👩🏾‍❤️‍👨🏽", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FD, "1f469-1f3fe-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_WM_TONE4_3, "👩🏾‍❤️‍👨🏽", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FD, "1f469-1f3fe-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_MW_TONE4, "👩🏾‍❤️‍👨🏾", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FE, "1f469-1f3fe-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_WM_TONE4, "👩🏾‍❤️‍👨🏾", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FE, "1f469-1f3fe-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_MW_TONE4_5, "👩🏾‍❤️‍👨🏿", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FF, "1f469-1f3fe-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(COUPLE_WITH_HEART_WM_TONE4_5, "👩🏾‍❤️‍👨🏿", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FF, "1f469-1f3fe-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(COUPLE_WITH_HEART_WW_TONE4_1, "👩🏾‍❤️‍👩🏻", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F469_1F3FB, "1f469-1f3fe-200d-2764-fe0f-200d-1f469-1f3fb.png");
png_name!(COUPLE_WITH_HEART_WW_TONE4_2, "👩🏾‍❤️‍👩🏼", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F469_1F3FC, "1f469-1f3fe-200d-2764-fe0f-200d-1f469-1f3fc.png");
png_name!(COUPLE_WITH_HEART_WW_TONE4_3, "👩🏾‍❤️‍👩🏽", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F469_1F3FD, "1f469-1f3fe-200d-2764-fe0f-200d-1f469-1f3fd.png");
png_name!(COUPLE_WITH_HEART_WW_TONE4, "👩🏾‍❤️‍👩🏾", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F469_1F3FE, "1f469-1f3fe-200d-2764-fe0f-200d-1f469-1f3fe.png");
png_name!(COUPLE_WITH_HEART_WW_TONE4_5, "👩🏾‍❤️‍👩🏿", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F469_1F3FF, "1f469-1f3fe-200d-2764-fe0f-200d-1f469-1f3ff.png");
png_name!(KISS_MW_TONE4_1, "👩🏾‍❤️‍💋‍👨🏻", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_WM_TONE4_1, "👩🏾‍❤️‍💋‍👨🏻", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_MW_TONE4_2, "👩🏾‍❤️‍💋‍👨🏼", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_WM_TONE4_2, "👩🏾‍❤️‍💋‍👨🏼", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_MW_TONE4_3, "👩🏾‍❤️‍💋‍👨🏽", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_WM_TONE4_3, "👩🏾‍❤️‍💋‍👨🏽", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_MW_TONE4, "👩🏾‍❤️‍💋‍👨🏾", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_WM_TONE4, "👩🏾‍❤️‍💋‍👨🏾", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_MW_TONE4_5, "👩🏾‍❤️‍💋‍👨🏿", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(KISS_WM_TONE4_5, "👩🏾‍❤️‍💋‍👨🏿", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(KISS_WW_TONE4_1, "👩🏾‍❤️‍💋‍👩🏻", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FB, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fb.png");
png_name!(KISS_WW_TONE4_2, "👩🏾‍❤️‍💋‍👩🏼", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FC, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fc.png");
png_name!(KISS_WW_TONE4_3, "👩🏾‍❤️‍💋‍👩🏽", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FD, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fd.png");
png_name!(KISS_WW_TONE4, "👩🏾‍❤️‍💋‍👩🏾", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FE, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fe.png");
png_name!(KISS_WW_TONE4_5, "👩🏾‍❤️‍💋‍👩🏿", "", U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FF, "1f469-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3ff.png");
png_name!(WOMAN_TONE4, "👩🏾", "", U_1F469_1F3FE, "1f469-1f3fe.png");
png_name!(WOMAN_FARMER_TONE5, "👩🏿‍🌾", "", U_1F469_1F3FF_200D_1F33E, "1f469-1f3ff-200d-1f33e.png");
png_name!(WOMAN_COOK_TONE5, "👩🏿‍🍳", "", U_1F469_1F3FF_200D_1F373, "1f469-1f3ff-200d-1f373.png");
png_name!(WOMAN_FEEDING_BABY_TONE5, "👩🏿‍🍼", "", U_1F469_1F3FF_200D_1F37C, "1f469-1f3ff-200d-1f37c.png");
png_name!(WOMAN_STUDENT_TONE5, "👩🏿‍🎓", "", U_1F469_1F3FF_200D_1F393, "1f469-1f3ff-200d-1f393.png");
png_name!(WOMAN_SINGER_TONE5, "👩🏿‍🎤", "", U_1F469_1F3FF_200D_1F3A4, "1f469-1f3ff-200d-1f3a4.png");
png_name!(WOMAN_ARTIST_TONE5, "👩🏿‍🎨", "", U_1F469_1F3FF_200D_1F3A8, "1f469-1f3ff-200d-1f3a8.png");
png_name!(WOMAN_TEACHER_TONE5, "👩🏿‍🏫", "", U_1F469_1F3FF_200D_1F3EB, "1f469-1f3ff-200d-1f3eb.png");
png_name!(WOMAN_FACTORY_WORKER_TONE5, "👩🏿‍🏭", "", U_1F469_1F3FF_200D_1F3ED, "1f469-1f3ff-200d-1f3ed.png");
png_name!(WOMAN_TECHNOLOGIST_TONE5, "👩🏿‍💻", "", U_1F469_1F3FF_200D_1F4BB, "1f469-1f3ff-200d-1f4bb.png");
png_name!(WOMAN_OFFICE_WORKER_TONE5, "👩🏿‍💼", "", U_1F469_1F3FF_200D_1F4BC, "1f469-1f3ff-200d-1f4bc.png");
png_name!(WOMAN_MECHANIC_TONE5, "👩🏿‍🔧", "", U_1F469_1F3FF_200D_1F527, "1f469-1f3ff-200d-1f527.png");
png_name!(WOMAN_SCIENTIST_TONE5, "👩🏿‍🔬", "", U_1F469_1F3FF_200D_1F52C, "1f469-1f3ff-200d-1f52c.png");
png_name!(WOMAN_ASTRONAUT_TONE5, "👩🏿‍🚀", "", U_1F469_1F3FF_200D_1F680, "1f469-1f3ff-200d-1f680.png");
png_name!(WOMAN_FIREFIGHTER_TONE5, "👩🏿‍🚒", "", U_1F469_1F3FF_200D_1F692, "1f469-1f3ff-200d-1f692.png");
png_name!(COUPLE_TONE5_1, "👩🏿‍🤝‍👨🏻", "", U_1F469_1F3FF_200D_1F91D_200D_1F468_1F3FB, "1f469-1f3ff-200d-1f91d-200d-1f468-1f3fb.png");
png_name!(COUPLE_TONE5_2, "👩🏿‍🤝‍👨🏼", "", U_1F469_1F3FF_200D_1F91D_200D_1F468_1F3FC, "1f469-1f3ff-200d-1f91d-200d-1f468-1f3fc.png");
png_name!(COUPLE_TONE5_3, "👩🏿‍🤝‍👨🏽", "", U_1F469_1F3FF_200D_1F91D_200D_1F468_1F3FD, "1f469-1f3ff-200d-1f91d-200d-1f468-1f3fd.png");
png_name!(COUPLE_TONE5_4, "👩🏿‍🤝‍👨🏾", "", U_1F469_1F3FF_200D_1F91D_200D_1F468_1F3FE, "1f469-1f3ff-200d-1f91d-200d-1f468-1f3fe.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE5_1, "👩🏿‍🤝‍👩🏻", "", U_1F469_1F3FF_200D_1F91D_200D_1F469_1F3FB, "1f469-1f3ff-200d-1f91d-200d-1f469-1f3fb.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE5_2, "👩🏿‍🤝‍👩🏼", "", U_1F469_1F3FF_200D_1F91D_200D_1F469_1F3FC, "1f469-1f3ff-200d-1f91d-200d-1f469-1f3fc.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE5_3, "👩🏿‍🤝‍👩🏽", "", U_1F469_1F3FF_200D_1F91D_200D_1F469_1F3FD, "1f469-1f3ff-200d-1f91d-200d-1f469-1f3fd.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE5_4, "👩🏿‍🤝‍👩🏾", "", U_1F469_1F3FF_200D_1F91D_200D_1F469_1F3FE, "1f469-1f3ff-200d-1f91d-200d-1f469-1f3fe.png");
png_name!(WOMAN_WITH_PROBING_CANE_TONE5, "👩🏿‍🦯", "", U_1F469_1F3FF_200D_1F9AF, "1f469-1f3ff-200d-1f9af.png");
png_name!(WOMAN_WITH_WHITE_CANE_TONE5, "👩🏿‍🦯", "", U_1F469_1F3FF_200D_1F9AF, "1f469-1f3ff-200d-1f9af.png");
png_name!(WOMAN_RED_HAIRED_TONE5, "👩🏿‍🦰", "", U_1F469_1F3FF_200D_1F9B0, "1f469-1f3ff-200d-1f9b0.png");
png_name!(WOMAN_CURLY_HAIRED_TONE5, "👩🏿‍🦱", "", U_1F469_1F3FF_200D_1F9B1, "1f469-1f3ff-200d-1f9b1.png");
png_name!(WOMAN_BALD_TONE5, "👩🏿‍🦲", "", U_1F469_1F3FF_200D_1F9B2, "1f469-1f3ff-200d-1f9b2.png");
png_name!(WOMAN_WHITE_HAIRED_TONE5, "👩🏿‍🦳", "", U_1F469_1F3FF_200D_1F9B3, "1f469-1f3ff-200d-1f9b3.png");
png_name!(WOMAN_IN_MOTORIZED_WHEELCHAIR_TONE5, "👩🏿‍🦼", "", U_1F469_1F3FF_200D_1F9BC, "1f469-1f3ff-200d-1f9bc.png");
png_name!(WOMAN_IN_MANUAL_WHEELCHAIR_TONE5, "👩🏿‍🦽", "", U_1F469_1F3FF_200D_1F9BD, "1f469-1f3ff-200d-1f9bd.png");
png_name!(WOMAN_HEALTH_WORKER_TONE5, "👩🏿‍⚕️", "", U_1F469_1F3FF_200D_2695_FE0F, "1f469-1f3ff-200d-2695-fe0f.png");
png_name!(WOMAN_JUDGE_TONE5, "👩🏿‍⚖️", "", U_1F469_1F3FF_200D_2696_FE0F, "1f469-1f3ff-200d-2696-fe0f.png");
png_name!(WOMAN_PILOT_TONE5, "👩🏿‍✈️", "", U_1F469_1F3FF_200D_2708_FE0F, "1f469-1f3ff-200d-2708-fe0f.png");
png_name!(COUPLE_WITH_HEART_MW_TONE5_1, "👩🏿‍❤️‍👨🏻", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FB, "1f469-1f3ff-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_WM_TONE5_1, "👩🏿‍❤️‍👨🏻", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FB, "1f469-1f3ff-200d-2764-fe0f-200d-1f468-1f3fb.png");
png_name!(COUPLE_WITH_HEART_MW_TONE5_2, "👩🏿‍❤️‍👨🏼", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FC, "1f469-1f3ff-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_WM_TONE5_2, "👩🏿‍❤️‍👨🏼", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FC, "1f469-1f3ff-200d-2764-fe0f-200d-1f468-1f3fc.png");
png_name!(COUPLE_WITH_HEART_MW_TONE5_3, "👩🏿‍❤️‍👨🏽", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FD, "1f469-1f3ff-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_WM_TONE5_3, "👩🏿‍❤️‍👨🏽", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FD, "1f469-1f3ff-200d-2764-fe0f-200d-1f468-1f3fd.png");
png_name!(COUPLE_WITH_HEART_MW_TONE5_4, "👩🏿‍❤️‍👨🏾", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FE, "1f469-1f3ff-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_WM_TONE5_4, "👩🏿‍❤️‍👨🏾", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FE, "1f469-1f3ff-200d-2764-fe0f-200d-1f468-1f3fe.png");
png_name!(COUPLE_WITH_HEART_MW_TONE5, "👩🏿‍❤️‍👨🏿", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FF, "1f469-1f3ff-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(COUPLE_WITH_HEART_WM_TONE5, "👩🏿‍❤️‍👨🏿", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FF, "1f469-1f3ff-200d-2764-fe0f-200d-1f468-1f3ff.png");
png_name!(COUPLE_WITH_HEART_WW_TONE5_1, "👩🏿‍❤️‍👩🏻", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F469_1F3FB, "1f469-1f3ff-200d-2764-fe0f-200d-1f469-1f3fb.png");
png_name!(COUPLE_WITH_HEART_WW_TONE5_2, "👩🏿‍❤️‍👩🏼", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F469_1F3FC, "1f469-1f3ff-200d-2764-fe0f-200d-1f469-1f3fc.png");
png_name!(COUPLE_WITH_HEART_WW_TONE5_3, "👩🏿‍❤️‍👩🏽", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F469_1F3FD, "1f469-1f3ff-200d-2764-fe0f-200d-1f469-1f3fd.png");
png_name!(COUPLE_WITH_HEART_WW_TONE5_4, "👩🏿‍❤️‍👩🏾", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F469_1F3FE, "1f469-1f3ff-200d-2764-fe0f-200d-1f469-1f3fe.png");
png_name!(COUPLE_WITH_HEART_WW_TONE5, "👩🏿‍❤️‍👩🏿", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F469_1F3FF, "1f469-1f3ff-200d-2764-fe0f-200d-1f469-1f3ff.png");
png_name!(KISS_MW_TONE5_1, "👩🏿‍❤️‍💋‍👨🏻", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_WM_TONE5_1, "👩🏿‍❤️‍💋‍👨🏻", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fb.png");
png_name!(KISS_MW_TONE5_2, "👩🏿‍❤️‍💋‍👨🏼", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_WM_TONE5_2, "👩🏿‍❤️‍💋‍👨🏼", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fc.png");
png_name!(KISS_MW_TONE5_3, "👩🏿‍❤️‍💋‍👨🏽", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_WM_TONE5_3, "👩🏿‍❤️‍💋‍👨🏽", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fd.png");
png_name!(KISS_MW_TONE5_4, "👩🏿‍❤️‍💋‍👨🏾", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_WM_TONE5_4, "👩🏿‍❤️‍💋‍👨🏾", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3fe.png");
png_name!(KISS_MW_TONE5, "👩🏿‍❤️‍💋‍👨🏿", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(KISS_WM_TONE5, "👩🏿‍❤️‍💋‍👨🏿", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f468-1f3ff.png");
png_name!(KISS_WW_TONE5_1, "👩🏿‍❤️‍💋‍👩🏻", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FB, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fb.png");
png_name!(KISS_WW_TONE5_2, "👩🏿‍❤️‍💋‍👩🏼", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FC, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fc.png");
png_name!(KISS_WW_TONE5_3, "👩🏿‍❤️‍💋‍👩🏽", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FD, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fd.png");
png_name!(KISS_WW_TONE5_4, "👩🏿‍❤️‍💋‍👩🏾", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FE, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3fe.png");
png_name!(KISS_WW_TONE5, "👩🏿‍❤️‍💋‍👩🏿", "", U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FF, "1f469-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f469-1f3ff.png");
png_name!(WOMAN_TONE5, "👩🏿", "", U_1F469_1F3FF, "1f469-1f3ff.png");
png_name!(WOMAN_FARMER, "👩‍🌾", "woman farmer", U_1F469_200D_1F33E, "1f469-200d-1f33e.png");
png_name!(WOMAN_COOK, "👩‍🍳", "woman cook", U_1F469_200D_1F373, "1f469-200d-1f373.png");
png_name!(WOMAN_FEEDING_BABY, "👩‍🍼", "woman feeding baby", U_1F469_200D_1F37C, "1f469-200d-1f37c.png");
png_name!(WOMAN_STUDENT, "👩‍🎓", "woman student", U_1F469_200D_1F393, "1f469-200d-1f393.png");
png_name!(WOMAN_SINGER, "👩‍🎤", "woman singer", U_1F469_200D_1F3A4, "1f469-200d-1f3a4.png");
png_name!(WOMAN_ARTIST, "👩‍🎨", "woman artist", U_1F469_200D_1F3A8, "1f469-200d-1f3a8.png");
png_name!(WOMAN_TEACHER, "👩‍🏫", "woman teacher", U_1F469_200D_1F3EB, "1f469-200d-1f3eb.png");
png_name!(WOMAN_FACTORY_WORKER, "👩‍🏭", "woman factory worker", U_1F469_200D_1F3ED, "1f469-200d-1f3ed.png");
png_name!(FAMILY_WBB, "👩‍👦‍👦", "family: woman, boy, boy", U_1F469_200D_1F466_200D_1F466, "1f469-200d-1f466-200d-1f466.png");
png_name!(FAMILY_WB, "👩‍👦", "family: woman, boy", U_1F469_200D_1F466, "1f469-200d-1f466.png");
png_name!(FAMILY_WGB, "👩‍👧‍👦", "family: woman, girl, boy", U_1F469_200D_1F467_200D_1F466, "1f469-200d-1f467-200d-1f466.png");
png_name!(FAMILY_WGG, "👩‍👧‍👧", "family: woman, girl, girl", U_1F469_200D_1F467_200D_1F467, "1f469-200d-1f467-200d-1f467.png");
png_name!(FAMILY_WG, "👩‍👧", "family: woman, girl", U_1F469_200D_1F467, "1f469-200d-1f467.png");
png_name!(FAMILY_WWBB, "👩‍👩‍👦‍👦", "family: woman, woman, boy, boy", U_1F469_200D_1F469_200D_1F466_200D_1F466, "1f469-200d-1f469-200d-1f466-200d-1f466.png");
png_name!(FAMILY_WWB, "👩‍👩‍👦", "family: woman, woman, boy", U_1F469_200D_1F469_200D_1F466, "1f469-200d-1f469-200d-1f466.png");
png_name!(FAMILY_WWGB, "👩‍👩‍👧‍👦", "family: woman, woman, girl, boy", U_1F469_200D_1F469_200D_1F467_200D_1F466, "1f469-200d-1f469-200d-1f467-200d-1f466.png");
png_name!(FAMILY_WWGG, "👩‍👩‍👧‍👧", "family: woman, woman, girl, girl", U_1F469_200D_1F469_200D_1F467_200D_1F467, "1f469-200d-1f469-200d-1f467-200d-1f467.png");
png_name!(FAMILY_WWG, "👩‍👩‍👧", "family: woman, woman, girl", U_1F469_200D_1F469_200D_1F467, "1f469-200d-1f469-200d-1f467.png");
png_name!(WOMAN_TECHNOLOGIST, "👩‍💻", "woman technologist", U_1F469_200D_1F4BB, "1f469-200d-1f4bb.png");
png_name!(WOMAN_OFFICE_WORKER, "👩‍💼", "woman office worker", U_1F469_200D_1F4BC, "1f469-200d-1f4bc.png");
png_name!(WOMAN_MECHANIC, "👩‍🔧", "woman mechanic", U_1F469_200D_1F527, "1f469-200d-1f527.png");
png_name!(WOMAN_SCIENTIST, "👩‍🔬", "woman scientist", U_1F469_200D_1F52C, "1f469-200d-1f52c.png");
png_name!(WOMAN_ASTRONAUT, "👩‍🚀", "woman astronaut", U_1F469_200D_1F680, "1f469-200d-1f680.png");
png_name!(WOMAN_FIREFIGHTER, "👩‍🚒", "woman firefighter", U_1F469_200D_1F692, "1f469-200d-1f692.png");
png_name!(WOMAN_WITH_PROBING_CANE, "👩‍🦯", "woman with white cane", U_1F469_200D_1F9AF, "1f469-200d-1f9af.png");
png_name!(WOMAN_WITH_WHITE_CANE, "👩‍🦯", "woman with white cane", U_1F469_200D_1F9AF, "1f469-200d-1f9af.png");
png_name!(WOMAN_RED_HAIRED, "👩‍🦰", "woman: red hair", U_1F469_200D_1F9B0, "1f469-200d-1f9b0.png");
png_name!(WOMAN_CURLY_HAIRED, "👩‍🦱", "woman: curly hair", U_1F469_200D_1F9B1, "1f469-200d-1f9b1.png");
png_name!(WOMAN_BALD, "👩‍🦲", "woman: bald", U_1F469_200D_1F9B2, "1f469-200d-1f9b2.png");
png_name!(WOMAN_WHITE_HAIRED, "👩‍🦳", "woman: white hair", U_1F469_200D_1F9B3, "1f469-200d-1f9b3.png");
png_name!(WOMAN_IN_MOTORIZED_WHEELCHAIR, "👩‍🦼", "woman in motorized wheelchair", U_1F469_200D_1F9BC, "1f469-200d-1f9bc.png");
png_name!(WOMAN_IN_MANUAL_WHEELCHAIR, "👩‍🦽", "woman in manual wheelchair", U_1F469_200D_1F9BD, "1f469-200d-1f9bd.png");
png_name!(WOMAN_HEALTH_WORKER, "👩‍⚕️", "woman health worker", U_1F469_200D_2695_FE0F, "1f469-200d-2695-fe0f.png");
png_name!(WOMAN_JUDGE, "👩‍⚖️", "woman judge", U_1F469_200D_2696_FE0F, "1f469-200d-2696-fe0f.png");
png_name!(WOMAN_PILOT, "👩‍✈️", "woman pilot", U_1F469_200D_2708_FE0F, "1f469-200d-2708-fe0f.png");
png_name!(COUPLE_WITH_HEART_MW, "👩‍❤️‍👨", "couple with heart: woman, man", U_1F469_200D_2764_FE0F_200D_1F468, "1f469-200d-2764-fe0f-200d-1f468.png");
png_name!(COUPLE_WITH_HEART_WM, "👩‍❤️‍👨", "couple with heart: woman, man", U_1F469_200D_2764_FE0F_200D_1F468, "1f469-200d-2764-fe0f-200d-1f468.png");
png_name!(COUPLE_WITH_HEART_WW, "👩‍❤️‍👩", "couple with heart: woman, woman", U_1F469_200D_2764_FE0F_200D_1F469, "1f469-200d-2764-fe0f-200d-1f469.png");
png_name!(KISS_MW, "👩‍❤️‍💋‍👨", "kiss: woman, man", U_1F469_200D_2764_FE0F_200D_1F48B_200D_1F468, "1f469-200d-2764-fe0f-200d-1f48b-200d-1f468.png");
png_name!(KISS_WM, "👩‍❤️‍💋‍👨", "kiss: woman, man", U_1F469_200D_2764_FE0F_200D_1F48B_200D_1F468, "1f469-200d-2764-fe0f-200d-1f48b-200d-1f468.png");
png_name!(KISS_WW, "👩‍❤️‍💋‍👩", "kiss: woman, woman", U_1F469_200D_2764_FE0F_200D_1F48B_200D_1F469, "1f469-200d-2764-fe0f-200d-1f48b-200d-1f469.png");
png_name!(WOMAN, "👩", "woman", U_1F469, "1f469.png");
png_name!(FAMILY, "👪", "family", U_1F46A, "1f46a.png");
png_name!(COUPLE_TONE1, "👫🏻", "", U_1F46B_1F3FB, "1f46b-1f3fb.png");
png_name!(COUPLE_TONE2, "👫🏼", "", U_1F46B_1F3FC, "1f46b-1f3fc.png");
png_name!(COUPLE_TONE3, "👫🏽", "", U_1F46B_1F3FD, "1f46b-1f3fd.png");
png_name!(COUPLE_TONE4, "👫🏾", "", U_1F46B_1F3FE, "1f46b-1f3fe.png");
png_name!(COUPLE_TONE5, "👫🏿", "", U_1F46B_1F3FF, "1f46b-1f3ff.png");
png_name!(COUPLE, "👫", "woman and man holding hands", U_1F46B, "1f46b.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE1, "👬🏻", "", U_1F46C_1F3FB, "1f46c-1f3fb.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE2, "👬🏼", "", U_1F46C_1F3FC, "1f46c-1f3fc.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE3, "👬🏽", "", U_1F46C_1F3FD, "1f46c-1f3fd.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE4, "👬🏾", "", U_1F46C_1F3FE, "1f46c-1f3fe.png");
png_name!(TWO_MEN_HOLDING_HANDS_TONE5, "👬🏿", "", U_1F46C_1F3FF, "1f46c-1f3ff.png");
png_name!(TWO_MEN_HOLDING_HANDS, "👬", "men holding hands", U_1F46C, "1f46c.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE1, "👭🏻", "", U_1F46D_1F3FB, "1f46d-1f3fb.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE2, "👭🏼", "", U_1F46D_1F3FC, "1f46d-1f3fc.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE3, "👭🏽", "", U_1F46D_1F3FD, "1f46d-1f3fd.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE4, "👭🏾", "", U_1F46D_1F3FE, "1f46d-1f3fe.png");
png_name!(TWO_WOMEN_HOLDING_HANDS_TONE5, "👭🏿", "", U_1F46D_1F3FF, "1f46d-1f3ff.png");
png_name!(TWO_WOMEN_HOLDING_HANDS, "👭", "women holding hands", U_1F46D, "1f46d.png");
png_name!(WOMAN_POLICE_OFFICER_TONE1, "👮🏻‍♀️", "", U_1F46E_1F3FB_200D_2640_FE0F, "1f46e-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_POLICE_OFFICER_TONE1, "👮🏻‍♂️", "", U_1F46E_1F3FB_200D_2642_FE0F, "1f46e-1f3fb-200d-2642-fe0f.png");
png_name!(COP_TONE1, "👮🏻", "", U_1F46E_1F3FB, "1f46e-1f3fb.png");
png_name!(POLICE_OFFICER_TONE1, "👮🏻", "", U_1F46E_1F3FB, "1f46e-1f3fb.png");
png_name!(WOMAN_POLICE_OFFICER_TONE2, "👮🏼‍♀️", "", U_1F46E_1F3FC_200D_2640_FE0F, "1f46e-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_POLICE_OFFICER_TONE2, "👮🏼‍♂️", "", U_1F46E_1F3FC_200D_2642_FE0F, "1f46e-1f3fc-200d-2642-fe0f.png");
png_name!(COP_TONE2, "👮🏼", "", U_1F46E_1F3FC, "1f46e-1f3fc.png");
png_name!(POLICE_OFFICER_TONE2, "👮🏼", "", U_1F46E_1F3FC, "1f46e-1f3fc.png");
png_name!(WOMAN_POLICE_OFFICER_TONE3, "👮🏽‍♀️", "", U_1F46E_1F3FD_200D_2640_FE0F, "1f46e-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_POLICE_OFFICER_TONE3, "👮🏽‍♂️", "", U_1F46E_1F3FD_200D_2642_FE0F, "1f46e-1f3fd-200d-2642-fe0f.png");
png_name!(COP_TONE3, "👮🏽", "", U_1F46E_1F3FD, "1f46e-1f3fd.png");
png_name!(POLICE_OFFICER_TONE3, "👮🏽", "", U_1F46E_1F3FD, "1f46e-1f3fd.png");
png_name!(WOMAN_POLICE_OFFICER_TONE4, "👮🏾‍♀️", "", U_1F46E_1F3FE_200D_2640_FE0F, "1f46e-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_POLICE_OFFICER_TONE4, "👮🏾‍♂️", "", U_1F46E_1F3FE_200D_2642_FE0F, "1f46e-1f3fe-200d-2642-fe0f.png");
png_name!(COP_TONE4, "👮🏾", "", U_1F46E_1F3FE, "1f46e-1f3fe.png");
png_name!(POLICE_OFFICER_TONE4, "👮🏾", "", U_1F46E_1F3FE, "1f46e-1f3fe.png");
png_name!(WOMAN_POLICE_OFFICER_TONE5, "👮🏿‍♀️", "", U_1F46E_1F3FF_200D_2640_FE0F, "1f46e-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_POLICE_OFFICER_TONE5, "👮🏿‍♂️", "", U_1F46E_1F3FF_200D_2642_FE0F, "1f46e-1f3ff-200d-2642-fe0f.png");
png_name!(COP_TONE5, "👮🏿", "", U_1F46E_1F3FF, "1f46e-1f3ff.png");
png_name!(POLICE_OFFICER_TONE5, "👮🏿", "", U_1F46E_1F3FF, "1f46e-1f3ff.png");
png_name!(WOMAN_POLICE_OFFICER, "👮‍♀️", "woman police officer", U_1F46E_200D_2640_FE0F, "1f46e-200d-2640-fe0f.png");
png_name!(MAN_POLICE_OFFICER, "👮‍♂️", "man police officer", U_1F46E_200D_2642_FE0F, "1f46e-200d-2642-fe0f.png");
png_name!(COP, "👮", "police officer", U_1F46E, "1f46e.png");
png_name!(POLICE_OFFICER, "👮", "police officer", U_1F46E, "1f46e.png");
png_name!(WOMEN_WITH_BUNNY_EARS_PARTYING, "👯‍♀️", "women with bunny ears", U_1F46F_200D_2640_FE0F, "1f46f-200d-2640-fe0f.png");
png_name!(MEN_WITH_BUNNY_EARS_PARTYING, "👯‍♂️", "men with bunny ears", U_1F46F_200D_2642_FE0F, "1f46f-200d-2642-fe0f.png");
png_name!(DANCERS, "👯", "people with bunny ears", U_1F46F, "1f46f.png");
png_name!(PEOPLE_WITH_BUNNY_EARS_PARTYING, "👯", "people with bunny ears", U_1F46F, "1f46f.png");
png_name!(WOMAN_WITH_VEIL_TONE1, "👰🏻‍♀️", "", U_1F470_1F3FB_200D_2640_FE0F, "1f470-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_WITH_VEIL_TONE1, "👰🏻‍♂️", "", U_1F470_1F3FB_200D_2642_FE0F, "1f470-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_WITH_VEIL_TONE1, "👰🏻", "", U_1F470_1F3FB, "1f470-1f3fb.png");
png_name!(WOMAN_WITH_VEIL_TONE2, "👰🏼‍♀️", "", U_1F470_1F3FC_200D_2640_FE0F, "1f470-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_WITH_VEIL_TONE2, "👰🏼‍♂️", "", U_1F470_1F3FC_200D_2642_FE0F, "1f470-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_WITH_VEIL_TONE2, "👰🏼", "", U_1F470_1F3FC, "1f470-1f3fc.png");
png_name!(WOMAN_WITH_VEIL_TONE3, "👰🏽‍♀️", "", U_1F470_1F3FD_200D_2640_FE0F, "1f470-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_WITH_VEIL_TONE3, "👰🏽‍♂️", "", U_1F470_1F3FD_200D_2642_FE0F, "1f470-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_WITH_VEIL_TONE3, "👰🏽", "", U_1F470_1F3FD, "1f470-1f3fd.png");
png_name!(WOMAN_WITH_VEIL_TONE4, "👰🏾‍♀️", "", U_1F470_1F3FE_200D_2640_FE0F, "1f470-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_WITH_VEIL_TONE4, "👰🏾‍♂️", "", U_1F470_1F3FE_200D_2642_FE0F, "1f470-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_WITH_VEIL_TONE4, "👰🏾", "", U_1F470_1F3FE, "1f470-1f3fe.png");
png_name!(WOMAN_WITH_VEIL_TONE5, "👰🏿‍♀️", "", U_1F470_1F3FF_200D_2640_FE0F, "1f470-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_WITH_VEIL_TONE5, "👰🏿‍♂️", "", U_1F470_1F3FF_200D_2642_FE0F, "1f470-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_WITH_VEIL_TONE5, "👰🏿", "", U_1F470_1F3FF, "1f470-1f3ff.png");
png_name!(WOMAN_WITH_VEIL, "👰‍♀️", "woman with veil", U_1F470_200D_2640_FE0F, "1f470-200d-2640-fe0f.png");
png_name!(MAN_WITH_VEIL, "👰‍♂️", "man with veil", U_1F470_200D_2642_FE0F, "1f470-200d-2642-fe0f.png");
png_name!(PERSON_WITH_VEIL, "👰", "person with veil", U_1F470, "1f470.png");
png_name!(WOMAN_BLOND_HAIRED_TONE1, "👱🏻‍♀️", "", U_1F471_1F3FB_200D_2640_FE0F, "1f471-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_BLOND_HAIRED_TONE1, "👱🏻‍♂️", "", U_1F471_1F3FB_200D_2642_FE0F, "1f471-1f3fb-200d-2642-fe0f.png");
png_name!(BLOND_HAIRED_TONE1, "👱🏻", "", U_1F471_1F3FB, "1f471-1f3fb.png");
png_name!(WOMAN_BLOND_HAIRED_TONE2, "👱🏼‍♀️", "", U_1F471_1F3FC_200D_2640_FE0F, "1f471-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_BLOND_HAIRED_TONE2, "👱🏼‍♂️", "", U_1F471_1F3FC_200D_2642_FE0F, "1f471-1f3fc-200d-2642-fe0f.png");
png_name!(BLOND_HAIRED_TONE2, "👱🏼", "", U_1F471_1F3FC, "1f471-1f3fc.png");
png_name!(WOMAN_BLOND_HAIRED_TONE3, "👱🏽‍♀️", "", U_1F471_1F3FD_200D_2640_FE0F, "1f471-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_BLOND_HAIRED_TONE3, "👱🏽‍♂️", "", U_1F471_1F3FD_200D_2642_FE0F, "1f471-1f3fd-200d-2642-fe0f.png");
png_name!(BLOND_HAIRED_TONE3, "👱🏽", "", U_1F471_1F3FD, "1f471-1f3fd.png");
png_name!(WOMAN_BLOND_HAIRED_TONE4, "👱🏾‍♀️", "", U_1F471_1F3FE_200D_2640_FE0F, "1f471-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_BLOND_HAIRED_TONE4, "👱🏾‍♂️", "", U_1F471_1F3FE_200D_2642_FE0F, "1f471-1f3fe-200d-2642-fe0f.png");
png_name!(BLOND_HAIRED_TONE4, "👱🏾", "", U_1F471_1F3FE, "1f471-1f3fe.png");
png_name!(WOMAN_BLOND_HAIRED_TONE5, "👱🏿‍♀️", "", U_1F471_1F3FF_200D_2640_FE0F, "1f471-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_BLOND_HAIRED_TONE5, "👱🏿‍♂️", "", U_1F471_1F3FF_200D_2642_FE0F, "1f471-1f3ff-200d-2642-fe0f.png");
png_name!(BLOND_HAIRED_TONE5, "👱🏿", "", U_1F471_1F3FF, "1f471-1f3ff.png");
png_name!(WOMAN_BLOND_HAIRED, "👱‍♀️", "woman: blond hair", U_1F471_200D_2640_FE0F, "1f471-200d-2640-fe0f.png");
png_name!(MAN_BLOND_HAIRED, "👱‍♂️", "man: blond hair", U_1F471_200D_2642_FE0F, "1f471-200d-2642-fe0f.png");
png_name!(BLOND_HAIRED, "👱", "person: blond hair", U_1F471, "1f471.png");
png_name!(PERSON_WITH_SKULLCAP_TONE1, "👲🏻", "", U_1F472_1F3FB, "1f472-1f3fb.png");
png_name!(PERSON_WITH_SKULLCAP_TONE2, "👲🏼", "", U_1F472_1F3FC, "1f472-1f3fc.png");
png_name!(PERSON_WITH_SKULLCAP_TONE3, "👲🏽", "", U_1F472_1F3FD, "1f472-1f3fd.png");
png_name!(PERSON_WITH_SKULLCAP_TONE4, "👲🏾", "", U_1F472_1F3FE, "1f472-1f3fe.png");
png_name!(PERSON_WITH_SKULLCAP_TONE5, "👲🏿", "", U_1F472_1F3FF, "1f472-1f3ff.png");
png_name!(PERSON_WITH_SKULLCAP, "👲", "person with skullcap", U_1F472, "1f472.png");
png_name!(WOMAN_WEARING_TURBAN_TONE1, "👳🏻‍♀️", "", U_1F473_1F3FB_200D_2640_FE0F, "1f473-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_WEARING_TURBAN_TONE1, "👳🏻‍♂️", "", U_1F473_1F3FB_200D_2642_FE0F, "1f473-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_WEARING_TURBAN_TONE1, "👳🏻", "", U_1F473_1F3FB, "1f473-1f3fb.png");
png_name!(WOMAN_WEARING_TURBAN_TONE2, "👳🏼‍♀️", "", U_1F473_1F3FC_200D_2640_FE0F, "1f473-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_WEARING_TURBAN_TONE2, "👳🏼‍♂️", "", U_1F473_1F3FC_200D_2642_FE0F, "1f473-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_WEARING_TURBAN_TONE2, "👳🏼", "", U_1F473_1F3FC, "1f473-1f3fc.png");
png_name!(WOMAN_WEARING_TURBAN_TONE3, "👳🏽‍♀️", "", U_1F473_1F3FD_200D_2640_FE0F, "1f473-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_WEARING_TURBAN_TONE3, "👳🏽‍♂️", "", U_1F473_1F3FD_200D_2642_FE0F, "1f473-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_WEARING_TURBAN_TONE3, "👳🏽", "", U_1F473_1F3FD, "1f473-1f3fd.png");
png_name!(WOMAN_WEARING_TURBAN_TONE4, "👳🏾‍♀️", "", U_1F473_1F3FE_200D_2640_FE0F, "1f473-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_WEARING_TURBAN_TONE4, "👳🏾‍♂️", "", U_1F473_1F3FE_200D_2642_FE0F, "1f473-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_WEARING_TURBAN_TONE4, "👳🏾", "", U_1F473_1F3FE, "1f473-1f3fe.png");
png_name!(WOMAN_WEARING_TURBAN_TONE5, "👳🏿‍♀️", "", U_1F473_1F3FF_200D_2640_FE0F, "1f473-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_WEARING_TURBAN_TONE5, "👳🏿‍♂️", "", U_1F473_1F3FF_200D_2642_FE0F, "1f473-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_WEARING_TURBAN_TONE5, "👳🏿", "", U_1F473_1F3FF, "1f473-1f3ff.png");
png_name!(WOMAN_WEARING_TURBAN, "👳‍♀️", "woman wearing turban", U_1F473_200D_2640_FE0F, "1f473-200d-2640-fe0f.png");
png_name!(MAN_WEARING_TURBAN, "👳‍♂️", "man wearing turban", U_1F473_200D_2642_FE0F, "1f473-200d-2642-fe0f.png");
png_name!(PERSON_WEARING_TURBAN, "👳", "person wearing turban", U_1F473, "1f473.png");
png_name!(OLDER_MAN_TONE1, "👴🏻", "", U_1F474_1F3FB, "1f474-1f3fb.png");
png_name!(OLDER_MAN_TONE2, "👴🏼", "", U_1F474_1F3FC, "1f474-1f3fc.png");
png_name!(OLDER_MAN_TONE3, "👴🏽", "", U_1F474_1F3FD, "1f474-1f3fd.png");
png_name!(OLDER_MAN_TONE4, "👴🏾", "", U_1F474_1F3FE, "1f474-1f3fe.png");
png_name!(OLDER_MAN_TONE5, "👴🏿", "", U_1F474_1F3FF, "1f474-1f3ff.png");
png_name!(OLDER_MAN, "👴", "old man", U_1F474, "1f474.png");
png_name!(OLDER_WOMAN_TONE1, "👵🏻", "", U_1F475_1F3FB, "1f475-1f3fb.png");
png_name!(OLDER_WOMAN_TONE2, "👵🏼", "", U_1F475_1F3FC, "1f475-1f3fc.png");
png_name!(OLDER_WOMAN_TONE3, "👵🏽", "", U_1F475_1F3FD, "1f475-1f3fd.png");
png_name!(OLDER_WOMAN_TONE4, "👵🏾", "", U_1F475_1F3FE, "1f475-1f3fe.png");
png_name!(OLDER_WOMAN_TONE5, "👵🏿", "", U_1F475_1F3FF, "1f475-1f3ff.png");
png_name!(OLDER_WOMAN, "👵", "old woman", U_1F475, "1f475.png");
png_name!(BABY_TONE1, "👶🏻", "", U_1F476_1F3FB, "1f476-1f3fb.png");
png_name!(BABY_TONE2, "👶🏼", "", U_1F476_1F3FC, "1f476-1f3fc.png");
png_name!(BABY_TONE3, "👶🏽", "", U_1F476_1F3FD, "1f476-1f3fd.png");
png_name!(BABY_TONE4, "👶🏾", "", U_1F476_1F3FE, "1f476-1f3fe.png");
png_name!(BABY_TONE5, "👶🏿", "", U_1F476_1F3FF, "1f476-1f3ff.png");
png_name!(BABY, "👶", "baby", U_1F476, "1f476.png");
png_name!(WOMAN_CONSTRUCTION_WORKER_TONE1, "👷🏻‍♀️", "", U_1F477_1F3FB_200D_2640_FE0F, "1f477-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_CONSTRUCTION_WORKER_TONE1, "👷🏻‍♂️", "", U_1F477_1F3FB_200D_2642_FE0F, "1f477-1f3fb-200d-2642-fe0f.png");
png_name!(CONSTRUCTION_WORKER_TONE1, "👷🏻", "", U_1F477_1F3FB, "1f477-1f3fb.png");
png_name!(WOMAN_CONSTRUCTION_WORKER_TONE2, "👷🏼‍♀️", "", U_1F477_1F3FC_200D_2640_FE0F, "1f477-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_CONSTRUCTION_WORKER_TONE2, "👷🏼‍♂️", "", U_1F477_1F3FC_200D_2642_FE0F, "1f477-1f3fc-200d-2642-fe0f.png");
png_name!(CONSTRUCTION_WORKER_TONE2, "👷🏼", "", U_1F477_1F3FC, "1f477-1f3fc.png");
png_name!(WOMAN_CONSTRUCTION_WORKER_TONE3, "👷🏽‍♀️", "", U_1F477_1F3FD_200D_2640_FE0F, "1f477-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_CONSTRUCTION_WORKER_TONE3, "👷🏽‍♂️", "", U_1F477_1F3FD_200D_2642_FE0F, "1f477-1f3fd-200d-2642-fe0f.png");
png_name!(CONSTRUCTION_WORKER_TONE3, "👷🏽", "", U_1F477_1F3FD, "1f477-1f3fd.png");
png_name!(WOMAN_CONSTRUCTION_WORKER_TONE4, "👷🏾‍♀️", "", U_1F477_1F3FE_200D_2640_FE0F, "1f477-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_CONSTRUCTION_WORKER_TONE4, "👷🏾‍♂️", "", U_1F477_1F3FE_200D_2642_FE0F, "1f477-1f3fe-200d-2642-fe0f.png");
png_name!(CONSTRUCTION_WORKER_TONE4, "👷🏾", "", U_1F477_1F3FE, "1f477-1f3fe.png");
png_name!(WOMAN_CONSTRUCTION_WORKER_TONE5, "👷🏿‍♀️", "", U_1F477_1F3FF_200D_2640_FE0F, "1f477-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_CONSTRUCTION_WORKER_TONE5, "👷🏿‍♂️", "", U_1F477_1F3FF_200D_2642_FE0F, "1f477-1f3ff-200d-2642-fe0f.png");
png_name!(CONSTRUCTION_WORKER_TONE5, "👷🏿", "", U_1F477_1F3FF, "1f477-1f3ff.png");
png_name!(WOMAN_CONSTRUCTION_WORKER, "👷‍♀️", "woman construction worker", U_1F477_200D_2640_FE0F, "1f477-200d-2640-fe0f.png");
png_name!(MAN_CONSTRUCTION_WORKER, "👷‍♂️", "man construction worker", U_1F477_200D_2642_FE0F, "1f477-200d-2642-fe0f.png");
png_name!(CONSTRUCTION_WORKER, "👷", "construction worker", U_1F477, "1f477.png");
png_name!(PRINCESS_TONE1, "👸🏻", "", U_1F478_1F3FB, "1f478-1f3fb.png");
png_name!(PRINCESS_TONE2, "👸🏼", "", U_1F478_1F3FC, "1f478-1f3fc.png");
png_name!(PRINCESS_TONE3, "👸🏽", "", U_1F478_1F3FD, "1f478-1f3fd.png");
png_name!(PRINCESS_TONE4, "👸🏾", "", U_1F478_1F3FE, "1f478-1f3fe.png");
png_name!(PRINCESS_TONE5, "👸🏿", "", U_1F478_1F3FF, "1f478-1f3ff.png");
png_name!(PRINCESS, "👸", "princess", U_1F478, "1f478.png");
png_name!(JAPANESE_OGRE, "👹", "ogre", U_1F479, "1f479.png");
png_name!(OGRE, "👹", "ogre", U_1F479, "1f479.png");
png_name!(GOBLIN, "👺", "goblin", U_1F47A, "1f47a.png");
png_name!(JAPANESE_GOBLIN, "👺", "goblin", U_1F47A, "1f47a.png");
png_name!(GHOST, "👻", "ghost", U_1F47B, "1f47b.png");
png_name!(ANGEL_TONE1, "👼🏻", "", U_1F47C_1F3FB, "1f47c-1f3fb.png");
png_name!(ANGEL_TONE2, "👼🏼", "", U_1F47C_1F3FC, "1f47c-1f3fc.png");
png_name!(ANGEL_TONE3, "👼🏽", "", U_1F47C_1F3FD, "1f47c-1f3fd.png");
png_name!(ANGEL_TONE4, "👼🏾", "", U_1F47C_1F3FE, "1f47c-1f3fe.png");
png_name!(ANGEL_TONE5, "👼🏿", "", U_1F47C_1F3FF, "1f47c-1f3ff.png");
png_name!(ANGEL, "👼", "baby angel", U_1F47C, "1f47c.png");
png_name!(ALIEN, "👽", "alien", U_1F47D, "1f47d.png");
png_name!(ALIEN_MONSTER, "👾", "alien monster", U_1F47E, "1f47e.png");
png_name!(SPACE_INVADER, "👾", "alien monster", U_1F47E, "1f47e.png");
png_name!(ANGRY_IMP, "👿", "angry face with horns", U_1F47F, "1f47f.png");
png_name!(IMP, "👿", "angry face with horns", U_1F47F, "1f47f.png");
png_name!(SKULL, "💀", "skull", U_1F480, "1f480.png");
png_name!(WOMAN_TIPPING_HAND_TONE1, "💁🏻‍♀️", "", U_1F481_1F3FB_200D_2640_FE0F, "1f481-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_TIPPING_HAND_TONE1, "💁🏻‍♂️", "", U_1F481_1F3FB_200D_2642_FE0F, "1f481-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_TIPPING_HAND_TONE1, "💁🏻", "", U_1F481_1F3FB, "1f481-1f3fb.png");
png_name!(WOMAN_TIPPING_HAND_TONE2, "💁🏼‍♀️", "", U_1F481_1F3FC_200D_2640_FE0F, "1f481-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_TIPPING_HAND_TONE2, "💁🏼‍♂️", "", U_1F481_1F3FC_200D_2642_FE0F, "1f481-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_TIPPING_HAND_TONE2, "💁🏼", "", U_1F481_1F3FC, "1f481-1f3fc.png");
png_name!(WOMAN_TIPPING_HAND_TONE3, "💁🏽‍♀️", "", U_1F481_1F3FD_200D_2640_FE0F, "1f481-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_TIPPING_HAND_TONE3, "💁🏽‍♂️", "", U_1F481_1F3FD_200D_2642_FE0F, "1f481-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_TIPPING_HAND_TONE3, "💁🏽", "", U_1F481_1F3FD, "1f481-1f3fd.png");
png_name!(WOMAN_TIPPING_HAND_TONE4, "💁🏾‍♀️", "", U_1F481_1F3FE_200D_2640_FE0F, "1f481-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_TIPPING_HAND_TONE4, "💁🏾‍♂️", "", U_1F481_1F3FE_200D_2642_FE0F, "1f481-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_TIPPING_HAND_TONE4, "💁🏾", "", U_1F481_1F3FE, "1f481-1f3fe.png");
png_name!(WOMAN_TIPPING_HAND_TONE5, "💁🏿‍♀️", "", U_1F481_1F3FF_200D_2640_FE0F, "1f481-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_TIPPING_HAND_TONE5, "💁🏿‍♂️", "", U_1F481_1F3FF_200D_2642_FE0F, "1f481-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_TIPPING_HAND_TONE5, "💁🏿", "", U_1F481_1F3FF, "1f481-1f3ff.png");
png_name!(WOMAN_TIPPING_HAND, "💁‍♀️", "woman tipping hand", U_1F481_200D_2640_FE0F, "1f481-200d-2640-fe0f.png");
png_name!(MAN_TIPPING_HAND, "💁‍♂️", "man tipping hand", U_1F481_200D_2642_FE0F, "1f481-200d-2642-fe0f.png");
png_name!(PERSON_TIPPING_HAND, "💁", "person tipping hand", U_1F481, "1f481.png");
png_name!(WOMAN_GUARD_TONE1, "💂🏻‍♀️", "", U_1F482_1F3FB_200D_2640_FE0F, "1f482-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_GUARD_TONE1, "💂🏻‍♂️", "", U_1F482_1F3FB_200D_2642_FE0F, "1f482-1f3fb-200d-2642-fe0f.png");
png_name!(GUARD_TONE1, "💂🏻", "", U_1F482_1F3FB, "1f482-1f3fb.png");
png_name!(WOMAN_GUARD_TONE2, "💂🏼‍♀️", "", U_1F482_1F3FC_200D_2640_FE0F, "1f482-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_GUARD_TONE2, "💂🏼‍♂️", "", U_1F482_1F3FC_200D_2642_FE0F, "1f482-1f3fc-200d-2642-fe0f.png");
png_name!(GUARD_TONE2, "💂🏼", "", U_1F482_1F3FC, "1f482-1f3fc.png");
png_name!(WOMAN_GUARD_TONE3, "💂🏽‍♀️", "", U_1F482_1F3FD_200D_2640_FE0F, "1f482-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_GUARD_TONE3, "💂🏽‍♂️", "", U_1F482_1F3FD_200D_2642_FE0F, "1f482-1f3fd-200d-2642-fe0f.png");
png_name!(GUARD_TONE3, "💂🏽", "", U_1F482_1F3FD, "1f482-1f3fd.png");
png_name!(WOMAN_GUARD_TONE4, "💂🏾‍♀️", "", U_1F482_1F3FE_200D_2640_FE0F, "1f482-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_GUARD_TONE4, "💂🏾‍♂️", "", U_1F482_1F3FE_200D_2642_FE0F, "1f482-1f3fe-200d-2642-fe0f.png");
png_name!(GUARD_TONE4, "💂🏾", "", U_1F482_1F3FE, "1f482-1f3fe.png");
png_name!(WOMAN_GUARD_TONE5, "💂🏿‍♀️", "", U_1F482_1F3FF_200D_2640_FE0F, "1f482-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_GUARD_TONE5, "💂🏿‍♂️", "", U_1F482_1F3FF_200D_2642_FE0F, "1f482-1f3ff-200d-2642-fe0f.png");
png_name!(GUARD_TONE5, "💂🏿", "", U_1F482_1F3FF, "1f482-1f3ff.png");
png_name!(WOMAN_GUARD, "💂‍♀️", "woman guard", U_1F482_200D_2640_FE0F, "1f482-200d-2640-fe0f.png");
png_name!(MAN_GUARD, "💂‍♂️", "man guard", U_1F482_200D_2642_FE0F, "1f482-200d-2642-fe0f.png");
png_name!(GUARD, "💂", "guard", U_1F482, "1f482.png");
png_name!(DANCER_TONE1, "💃🏻", "", U_1F483_1F3FB, "1f483-1f3fb.png");
png_name!(WOMAN_DANCING_TONE1, "💃🏻", "", U_1F483_1F3FB, "1f483-1f3fb.png");
png_name!(DANCER_TONE2, "💃🏼", "", U_1F483_1F3FC, "1f483-1f3fc.png");
png_name!(WOMAN_DANCING_TONE2, "💃🏼", "", U_1F483_1F3FC, "1f483-1f3fc.png");
png_name!(DANCER_TONE3, "💃🏽", "", U_1F483_1F3FD, "1f483-1f3fd.png");
png_name!(WOMAN_DANCING_TONE3, "💃🏽", "", U_1F483_1F3FD, "1f483-1f3fd.png");
png_name!(DANCER_TONE4, "💃🏾", "", U_1F483_1F3FE, "1f483-1f3fe.png");
png_name!(WOMAN_DANCING_TONE4, "💃🏾", "", U_1F483_1F3FE, "1f483-1f3fe.png");
png_name!(DANCER_TONE5, "💃🏿", "", U_1F483_1F3FF, "1f483-1f3ff.png");
png_name!(WOMAN_DANCING_TONE5, "💃🏿", "", U_1F483_1F3FF, "1f483-1f3ff.png");
png_name!(DANCER, "💃", "woman dancing", U_1F483, "1f483.png");
png_name!(WOMAN_DANCING, "💃", "woman dancing", U_1F483, "1f483.png");
png_name!(LIPSTICK, "💄", "lipstick", U_1F484, "1f484.png");
png_name!(NAIL_CARE_TONE1, "💅🏻", "", U_1F485_1F3FB, "1f485-1f3fb.png");
png_name!(NAIL_POLISH_TONE1, "💅🏻", "", U_1F485_1F3FB, "1f485-1f3fb.png");
png_name!(NAIL_CARE_TONE2, "💅🏼", "", U_1F485_1F3FC, "1f485-1f3fc.png");
png_name!(NAIL_POLISH_TONE2, "💅🏼", "", U_1F485_1F3FC, "1f485-1f3fc.png");
png_name!(NAIL_CARE_TONE3, "💅🏽", "", U_1F485_1F3FD, "1f485-1f3fd.png");
png_name!(NAIL_POLISH_TONE3, "💅🏽", "", U_1F485_1F3FD, "1f485-1f3fd.png");
png_name!(NAIL_CARE_TONE4, "💅🏾", "", U_1F485_1F3FE, "1f485-1f3fe.png");
png_name!(NAIL_POLISH_TONE4, "💅🏾", "", U_1F485_1F3FE, "1f485-1f3fe.png");
png_name!(NAIL_CARE_TONE5, "💅🏿", "", U_1F485_1F3FF, "1f485-1f3ff.png");
png_name!(NAIL_POLISH_TONE5, "💅🏿", "", U_1F485_1F3FF, "1f485-1f3ff.png");
png_name!(NAIL_CARE, "💅", "nail polish", U_1F485, "1f485.png");
png_name!(NAIL_POLISH, "💅", "nail polish", U_1F485, "1f485.png");
png_name!(WOMAN_GETTING_MASSAGE_TONE1, "💆🏻‍♀️", "", U_1F486_1F3FB_200D_2640_FE0F, "1f486-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_GETTING_MASSAGE_TONE1, "💆🏻‍♂️", "", U_1F486_1F3FB_200D_2642_FE0F, "1f486-1f3fb-200d-2642-fe0f.png");
png_name!(MASSAGE_TONE1, "💆🏻", "", U_1F486_1F3FB, "1f486-1f3fb.png");
png_name!(PERSON_GETTING_MASSAGE_TONE1, "💆🏻", "", U_1F486_1F3FB, "1f486-1f3fb.png");
png_name!(WOMAN_GETTING_MASSAGE_TONE2, "💆🏼‍♀️", "", U_1F486_1F3FC_200D_2640_FE0F, "1f486-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_GETTING_MASSAGE_TONE2, "💆🏼‍♂️", "", U_1F486_1F3FC_200D_2642_FE0F, "1f486-1f3fc-200d-2642-fe0f.png");
png_name!(MASSAGE_TONE2, "💆🏼", "", U_1F486_1F3FC, "1f486-1f3fc.png");
png_name!(PERSON_GETTING_MASSAGE_TONE2, "💆🏼", "", U_1F486_1F3FC, "1f486-1f3fc.png");
png_name!(WOMAN_GETTING_MASSAGE_TONE3, "💆🏽‍♀️", "", U_1F486_1F3FD_200D_2640_FE0F, "1f486-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_GETTING_MASSAGE_TONE3, "💆🏽‍♂️", "", U_1F486_1F3FD_200D_2642_FE0F, "1f486-1f3fd-200d-2642-fe0f.png");
png_name!(MASSAGE_TONE3, "💆🏽", "", U_1F486_1F3FD, "1f486-1f3fd.png");
png_name!(PERSON_GETTING_MASSAGE_TONE3, "💆🏽", "", U_1F486_1F3FD, "1f486-1f3fd.png");
png_name!(WOMAN_GETTING_MASSAGE_TONE4, "💆🏾‍♀️", "", U_1F486_1F3FE_200D_2640_FE0F, "1f486-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_GETTING_MASSAGE_TONE4, "💆🏾‍♂️", "", U_1F486_1F3FE_200D_2642_FE0F, "1f486-1f3fe-200d-2642-fe0f.png");
png_name!(MASSAGE_TONE4, "💆🏾", "", U_1F486_1F3FE, "1f486-1f3fe.png");
png_name!(PERSON_GETTING_MASSAGE_TONE4, "💆🏾", "", U_1F486_1F3FE, "1f486-1f3fe.png");
png_name!(WOMAN_GETTING_MASSAGE_TONE5, "💆🏿‍♀️", "", U_1F486_1F3FF_200D_2640_FE0F, "1f486-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_GETTING_MASSAGE_TONE5, "💆🏿‍♂️", "", U_1F486_1F3FF_200D_2642_FE0F, "1f486-1f3ff-200d-2642-fe0f.png");
png_name!(MASSAGE_TONE5, "💆🏿", "", U_1F486_1F3FF, "1f486-1f3ff.png");
png_name!(PERSON_GETTING_MASSAGE_TONE5, "💆🏿", "", U_1F486_1F3FF, "1f486-1f3ff.png");
png_name!(WOMAN_GETTING_MASSAGE, "💆‍♀️", "woman getting massage", U_1F486_200D_2640_FE0F, "1f486-200d-2640-fe0f.png");
png_name!(MAN_GETTING_MASSAGE, "💆‍♂️", "man getting massage", U_1F486_200D_2642_FE0F, "1f486-200d-2642-fe0f.png");
png_name!(MASSAGE, "💆", "person getting massage", U_1F486, "1f486.png");
png_name!(PERSON_GETTING_MASSAGE, "💆", "person getting massage", U_1F486, "1f486.png");
png_name!(WOMAN_GETTING_HAIRCUT_TONE1, "💇🏻‍♀️", "", U_1F487_1F3FB_200D_2640_FE0F, "1f487-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_GETTING_HAIRCUT_TONE1, "💇🏻‍♂️", "", U_1F487_1F3FB_200D_2642_FE0F, "1f487-1f3fb-200d-2642-fe0f.png");
png_name!(HAIRCUT_TONE1, "💇🏻", "", U_1F487_1F3FB, "1f487-1f3fb.png");
png_name!(PERSON_GETTING_HAIRCUT_TONE1, "💇🏻", "", U_1F487_1F3FB, "1f487-1f3fb.png");
png_name!(WOMAN_GETTING_HAIRCUT_TONE2, "💇🏼‍♀️", "", U_1F487_1F3FC_200D_2640_FE0F, "1f487-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_GETTING_HAIRCUT_TONE2, "💇🏼‍♂️", "", U_1F487_1F3FC_200D_2642_FE0F, "1f487-1f3fc-200d-2642-fe0f.png");
png_name!(HAIRCUT_TONE2, "💇🏼", "", U_1F487_1F3FC, "1f487-1f3fc.png");
png_name!(PERSON_GETTING_HAIRCUT_TONE2, "💇🏼", "", U_1F487_1F3FC, "1f487-1f3fc.png");
png_name!(WOMAN_GETTING_HAIRCUT_TONE3, "💇🏽‍♀️", "", U_1F487_1F3FD_200D_2640_FE0F, "1f487-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_GETTING_HAIRCUT_TONE3, "💇🏽‍♂️", "", U_1F487_1F3FD_200D_2642_FE0F, "1f487-1f3fd-200d-2642-fe0f.png");
png_name!(HAIRCUT_TONE3, "💇🏽", "", U_1F487_1F3FD, "1f487-1f3fd.png");
png_name!(PERSON_GETTING_HAIRCUT_TONE3, "💇🏽", "", U_1F487_1F3FD, "1f487-1f3fd.png");
png_name!(WOMAN_GETTING_HAIRCUT_TONE4, "💇🏾‍♀️", "", U_1F487_1F3FE_200D_2640_FE0F, "1f487-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_GETTING_HAIRCUT_TONE4, "💇🏾‍♂️", "", U_1F487_1F3FE_200D_2642_FE0F, "1f487-1f3fe-200d-2642-fe0f.png");
png_name!(HAIRCUT_TONE4, "💇🏾", "", U_1F487_1F3FE, "1f487-1f3fe.png");
png_name!(PERSON_GETTING_HAIRCUT_TONE4, "💇🏾", "", U_1F487_1F3FE, "1f487-1f3fe.png");
png_name!(WOMAN_GETTING_HAIRCUT_TONE5, "💇🏿‍♀️", "", U_1F487_1F3FF_200D_2640_FE0F, "1f487-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_GETTING_HAIRCUT_TONE5, "💇🏿‍♂️", "", U_1F487_1F3FF_200D_2642_FE0F, "1f487-1f3ff-200d-2642-fe0f.png");
png_name!(HAIRCUT_TONE5, "💇🏿", "", U_1F487_1F3FF, "1f487-1f3ff.png");
png_name!(PERSON_GETTING_HAIRCUT_TONE5, "💇🏿", "", U_1F487_1F3FF, "1f487-1f3ff.png");
png_name!(WOMAN_GETTING_HAIRCUT, "💇‍♀️", "woman getting haircut", U_1F487_200D_2640_FE0F, "1f487-200d-2640-fe0f.png");
png_name!(MAN_GETTING_HAIRCUT, "💇‍♂️", "man getting haircut", U_1F487_200D_2642_FE0F, "1f487-200d-2642-fe0f.png");
png_name!(HAIRCUT, "💇", "person getting haircut", U_1F487, "1f487.png");
png_name!(PERSON_GETTING_HAIRCUT, "💇", "person getting haircut", U_1F487, "1f487.png");
png_name!(BARBER, "💈", "barber pole", U_1F488, "1f488.png");
png_name!(BARBER_POLE, "💈", "barber pole", U_1F488, "1f488.png");
png_name!(SYRINGE, "💉", "syringe", U_1F489, "1f489.png");
png_name!(PILL, "💊", "pill", U_1F48A, "1f48a.png");
png_name!(KISS, "💋", "kiss mark", U_1F48B, "1f48b.png");
png_name!(LOVE_LETTER, "💌", "love letter", U_1F48C, "1f48c.png");
png_name!(RING, "💍", "ring", U_1F48D, "1f48d.png");
png_name!(GEM, "💎", "gem stone", U_1F48E, "1f48e.png");
png_name!(COUPLE_KISS_TONE1, "💏🏻", "", U_1F48F_1F3FB, "1f48f-1f3fb.png");
png_name!(COUPLEKISS_TONE1, "💏🏻", "", U_1F48F_1F3FB, "1f48f-1f3fb.png");
png_name!(COUPLE_KISS_TONE2, "💏🏼", "", U_1F48F_1F3FC, "1f48f-1f3fc.png");
png_name!(COUPLEKISS_TONE2, "💏🏼", "", U_1F48F_1F3FC, "1f48f-1f3fc.png");
png_name!(COUPLE_KISS_TONE3, "💏🏽", "", U_1F48F_1F3FD, "1f48f-1f3fd.png");
png_name!(COUPLEKISS_TONE3, "💏🏽", "", U_1F48F_1F3FD, "1f48f-1f3fd.png");
png_name!(COUPLE_KISS_TONE4, "💏🏾", "", U_1F48F_1F3FE, "1f48f-1f3fe.png");
png_name!(COUPLEKISS_TONE4, "💏🏾", "", U_1F48F_1F3FE, "1f48f-1f3fe.png");
png_name!(COUPLE_KISS_TONE5, "💏🏿", "", U_1F48F_1F3FF, "1f48f-1f3ff.png");
png_name!(COUPLEKISS_TONE5, "💏🏿", "", U_1F48F_1F3FF, "1f48f-1f3ff.png");
png_name!(COUPLE_KISS, "💏", "kiss", U_1F48F, "1f48f.png");
png_name!(COUPLEKISS, "💏", "kiss", U_1F48F, "1f48f.png");
png_name!(BOUQUET, "💐", "bouquet", U_1F490, "1f490.png");
png_name!(COUPLE_WITH_HEART_TONE1, "💑🏻", "", U_1F491_1F3FB, "1f491-1f3fb.png");
png_name!(COUPLE_WITH_HEART_TONE2, "💑🏼", "", U_1F491_1F3FC, "1f491-1f3fc.png");
png_name!(COUPLE_WITH_HEART_TONE3, "💑🏽", "", U_1F491_1F3FD, "1f491-1f3fd.png");
png_name!(COUPLE_WITH_HEART_TONE4, "💑🏾", "", U_1F491_1F3FE, "1f491-1f3fe.png");
png_name!(COUPLE_WITH_HEART_TONE5, "💑🏿", "", U_1F491_1F3FF, "1f491-1f3ff.png");
png_name!(COUPLE_WITH_HEART, "💑", "couple with heart", U_1F491, "1f491.png");
png_name!(WEDDING, "💒", "wedding", U_1F492, "1f492.png");
png_name!(BEATING_HEART, "💓", "beating heart", U_1F493, "1f493.png");
png_name!(HEARTBEAT, "💓", "beating heart", U_1F493, "1f493.png");
png_name!(BROKEN_HEART, "💔", "broken heart", U_1F494, "1f494.png");
png_name!(TWO_HEARTS, "💕", "two hearts", U_1F495, "1f495.png");
png_name!(SPARKLING_HEART, "💖", "sparkling heart", U_1F496, "1f496.png");
png_name!(GROWING_HEART, "💗", "growing heart", U_1F497, "1f497.png");
png_name!(HEARTPULSE, "💗", "growing heart", U_1F497, "1f497.png");
png_name!(CUPID, "💘", "heart with arrow", U_1F498, "1f498.png");
png_name!(HEART_WITH_ARROW, "💘", "heart with arrow", U_1F498, "1f498.png");
png_name!(BLUE_HEART, "💙", "blue heart", U_1F499, "1f499.png");
png_name!(GREEN_HEART, "💚", "green heart", U_1F49A, "1f49a.png");
png_name!(YELLOW_HEART, "💛", "yellow heart", U_1F49B, "1f49b.png");
png_name!(PURPLE_HEART, "💜", "purple heart", U_1F49C, "1f49c.png");
png_name!(GIFT_HEART, "💝", "heart with ribbon", U_1F49D, "1f49d.png");
png_name!(HEART_WITH_RIBBON, "💝", "heart with ribbon", U_1F49D, "1f49d.png");
png_name!(REVOLVING_HEARTS, "💞", "revolving hearts", U_1F49E, "1f49e.png");
png_name!(HEART_DECORATION, "💟", "heart decoration", U_1F49F, "1f49f.png");
png_name!(DIAMOND_SHAPE_WITH_A_DOT_INSIDE, "💠", "diamond with a dot", U_1F4A0, "1f4a0.png");
png_name!(DIAMOND_WITH_A_DOT, "💠", "diamond with a dot", U_1F4A0, "1f4a0.png");
png_name!(BULB, "💡", "light bulb", U_1F4A1, "1f4a1.png");
png_name!(LIGHT_BULB, "💡", "light bulb", U_1F4A1, "1f4a1.png");
png_name!(ANGER, "💢", "anger symbol", U_1F4A2, "1f4a2.png");
png_name!(BOMB, "💣", "bomb", U_1F4A3, "1f4a3.png");
png_name!(ZZZ, "💤", "ZZZ", U_1F4A4, "1f4a4.png");
png_name!(BOOM, "💥", "collision", U_1F4A5, "1f4a5.png");
png_name!(COLLISION, "💥", "collision", U_1F4A5, "1f4a5.png");
png_name!(SWEAT_DROPS, "💦", "sweat droplets", U_1F4A6, "1f4a6.png");
png_name!(DROPLET, "💧", "droplet", U_1F4A7, "1f4a7.png");
png_name!(DASH, "💨", "dashing away", U_1F4A8, "1f4a8.png");
png_name!(DASHING_AWAY, "💨", "dashing away", U_1F4A8, "1f4a8.png");
png_name!(POOP, "💩", "pile of poo", U_1F4A9, "1f4a9.png");
png_name!(SHIT, "💩", "pile of poo", U_1F4A9, "1f4a9.png");
png_name!(MUSCLE_TONE1, "💪🏻", "", U_1F4AA_1F3FB, "1f4aa-1f3fb.png");
png_name!(RIGHT_BICEP_TONE1, "💪🏻", "", U_1F4AA_1F3FB, "1f4aa-1f3fb.png");
png_name!(MUSCLE_TONE2, "💪🏼", "", U_1F4AA_1F3FC, "1f4aa-1f3fc.png");
png_name!(RIGHT_BICEP_TONE2, "💪🏼", "", U_1F4AA_1F3FC, "1f4aa-1f3fc.png");
png_name!(MUSCLE_TONE3, "💪🏽", "", U_1F4AA_1F3FD, "1f4aa-1f3fd.png");
png_name!(RIGHT_BICEP_TONE3, "💪🏽", "", U_1F4AA_1F3FD, "1f4aa-1f3fd.png");
png_name!(MUSCLE_TONE4, "💪🏾", "", U_1F4AA_1F3FE, "1f4aa-1f3fe.png");
png_name!(RIGHT_BICEP_TONE4, "💪🏾", "", U_1F4AA_1F3FE, "1f4aa-1f3fe.png");
png_name!(MUSCLE_TONE5, "💪🏿", "", U_1F4AA_1F3FF, "1f4aa-1f3ff.png");
png_name!(RIGHT_BICEP_TONE5, "💪🏿", "", U_1F4AA_1F3FF, "1f4aa-1f3ff.png");
png_name!(MUSCLE, "💪", "flexed biceps", U_1F4AA, "1f4aa.png");
png_name!(RIGHT_BICEP, "💪", "flexed biceps", U_1F4AA, "1f4aa.png");
png_name!(DIZZY, "💫", "dizzy", U_1F4AB, "1f4ab.png");
png_name!(SPEECH_BALLOON, "💬", "speech balloon", U_1F4AC, "1f4ac.png");
png_name!(THOUGHT_BALLOON, "💭", "thought balloon", U_1F4AD, "1f4ad.png");
png_name!(WHITE_FLOWER, "💮", "white flower", U_1F4AE, "1f4ae.png");
png_name!(X_100, "💯", "hundred points", U_1F4AF, "1f4af.png");
png_name!(MONEYBAG, "💰", "money bag", U_1F4B0, "1f4b0.png");
png_name!(CURRENCY_EXCHANGE, "💱", "currency exchange", U_1F4B1, "1f4b1.png");
png_name!(HEAVY_DOLLAR_SIGN, "💲", "heavy dollar sign", U_1F4B2, "1f4b2.png");
png_name!(CREDIT_CARD, "💳", "credit card", U_1F4B3, "1f4b3.png");
png_name!(YEN, "💴", "yen banknote", U_1F4B4, "1f4b4.png");
png_name!(DOLLAR, "💵", "dollar banknote", U_1F4B5, "1f4b5.png");
png_name!(EURO, "💶", "euro banknote", U_1F4B6, "1f4b6.png");
png_name!(POUND, "💷", "pound banknote", U_1F4B7, "1f4b7.png");
png_name!(MONEY_WITH_WINGS, "💸", "money with wings", U_1F4B8, "1f4b8.png");
png_name!(CHART, "💹", "chart increasing with yen", U_1F4B9, "1f4b9.png");
png_name!(SEAT, "💺", "seat", U_1F4BA, "1f4ba.png");
png_name!(LAPTOP, "💻", "laptop", U_1F4BB, "1f4bb.png");
png_name!(BRIEFCASE, "💼", "briefcase", U_1F4BC, "1f4bc.png");
png_name!(COMPUTER_DISK, "💽", "computer disk", U_1F4BD, "1f4bd.png");
png_name!(MINIDISC, "💽", "computer disk", U_1F4BD, "1f4bd.png");
png_name!(FLOPPY_DISK, "💾", "floppy disk", U_1F4BE, "1f4be.png");
png_name!(CD, "💿", "optical disk", U_1F4BF, "1f4bf.png");
png_name!(OPTICAL_DISK, "💿", "optical disk", U_1F4BF, "1f4bf.png");
png_name!(DVD, "📀", "dvd", U_1F4C0, "1f4c0.png");
png_name!(FILE_FOLDER, "📁", "file folder", U_1F4C1, "1f4c1.png");
png_name!(OPEN_FILE_FOLDER, "📂", "open file folder", U_1F4C2, "1f4c2.png");
png_name!(PAGE_WITH_CURL, "📃", "page with curl", U_1F4C3, "1f4c3.png");
png_name!(PAGE_FACING_UP, "📄", "page facing up", U_1F4C4, "1f4c4.png");
png_name!(DATE, "📅", "calendar", U_1F4C5, "1f4c5.png");
png_name!(CALENDAR, "📆", "tear-off calendar", U_1F4C6, "1f4c6.png");
png_name!(CARD_INDEX, "📇", "card index", U_1F4C7, "1f4c7.png");
png_name!(CHART_INCREASING, "📈", "chart increasing", U_1F4C8, "1f4c8.png");
png_name!(CHART_WITH_UPWARDS_TREND, "📈", "chart increasing", U_1F4C8, "1f4c8.png");
png_name!(CHART_DECREASING, "📉", "chart decreasing", U_1F4C9, "1f4c9.png");
png_name!(CHART_WITH_DOWNWARDS_TREND, "📉", "chart decreasing", U_1F4C9, "1f4c9.png");
png_name!(BAR_CHART, "📊", "bar chart", U_1F4CA, "1f4ca.png");
png_name!(CLIPBOARD, "📋", "clipboard", U_1F4CB, "1f4cb.png");
png_name!(PUSHPIN, "📌", "pushpin", U_1F4CC, "1f4cc.png");
png_name!(ROUND_PUSHPIN, "📍", "round pushpin", U_1F4CD, "1f4cd.png");
png_name!(PAPERCLIP, "📎", "paperclip", U_1F4CE, "1f4ce.png");
png_name!(STRAIGHT_RULER, "📏", "straight ruler", U_1F4CF, "1f4cf.png");
png_name!(TRIANGULAR_RULER, "📐", "triangular ruler", U_1F4D0, "1f4d0.png");
png_name!(BOOKMARK_TABS, "📑", "bookmark tabs", U_1F4D1, "1f4d1.png");
png_name!(LEDGER, "📒", "ledger", U_1F4D2, "1f4d2.png");
png_name!(NOTEBOOK, "📓", "notebook", U_1F4D3, "1f4d3.png");
png_name!(NOTEBOOK_WITH_DECORATIVE_COVER, "📔", "notebook with decorative cover", U_1F4D4, "1f4d4.png");
png_name!(CLOSED_BOOK, "📕", "closed book", U_1F4D5, "1f4d5.png");
png_name!(BOOK, "📖", "open book", U_1F4D6, "1f4d6.png");
png_name!(OPEN_BOOK, "📖", "open book", U_1F4D6, "1f4d6.png");
png_name!(GREEN_BOOK, "📗", "green book", U_1F4D7, "1f4d7.png");
png_name!(BLUE_BOOK, "📘", "blue book", U_1F4D8, "1f4d8.png");
png_name!(ORANGE_BOOK, "📙", "orange book", U_1F4D9, "1f4d9.png");
png_name!(BOOKS, "📚", "books", U_1F4DA, "1f4da.png");
png_name!(NAME_BADGE, "📛", "name badge", U_1F4DB, "1f4db.png");
png_name!(SCROLL, "📜", "scroll", U_1F4DC, "1f4dc.png");
png_name!(MEMO, "📝", "memo", U_1F4DD, "1f4dd.png");
png_name!(TELEPHONE_RECEIVER, "📞", "telephone receiver", U_1F4DE, "1f4de.png");
png_name!(PAGER, "📟", "pager", U_1F4DF, "1f4df.png");
png_name!(FAX, "📠", "fax machine", U_1F4E0, "1f4e0.png");
png_name!(FAX_MACHINE, "📠", "fax machine", U_1F4E0, "1f4e0.png");
png_name!(SATELLITE_ANTENNA, "📡", "satellite antenna", U_1F4E1, "1f4e1.png");
png_name!(LOUDSPEAKER, "📢", "loudspeaker", U_1F4E2, "1f4e2.png");
png_name!(MEGA, "📣", "megaphone", U_1F4E3, "1f4e3.png");
png_name!(MEGAPHONE, "📣", "megaphone", U_1F4E3, "1f4e3.png");
png_name!(OUTBOX_TRAY, "📤", "outbox tray", U_1F4E4, "1f4e4.png");
png_name!(INBOX_TRAY, "📥", "inbox tray", U_1F4E5, "1f4e5.png");
png_name!(PACKAGE, "📦", "package", U_1F4E6, "1f4e6.png");
png_name!(E_MAIL, "📧", "e-mail", U_1F4E7, "1f4e7.png");
png_name!(EMAIL, "📧", "e-mail", U_1F4E7, "1f4e7.png");
png_name!(INCOMING_ENVELOPE, "📨", "incoming envelope", U_1F4E8, "1f4e8.png");
png_name!(ENVELOPE_WITH_ARROW, "📩", "envelope with arrow", U_1F4E9, "1f4e9.png");
png_name!(MAILBOX_CLOSED, "📪", "closed mailbox with lowered flag", U_1F4EA, "1f4ea.png");
png_name!(MAILBOX, "📫", "closed mailbox with raised flag", U_1F4EB, "1f4eb.png");
png_name!(MAILBOX_WITH_MAIL, "📬", "open mailbox with raised flag", U_1F4EC, "1f4ec.png");
png_name!(MAILBOX_WITH_NO_MAIL, "📭", "open mailbox with lowered flag", U_1F4ED, "1f4ed.png");
png_name!(POSTBOX, "📮", "postbox", U_1F4EE, "1f4ee.png");
png_name!(POSTAL_HORN, "📯", "postal horn", U_1F4EF, "1f4ef.png");
png_name!(NEWSPAPER, "📰", "newspaper", U_1F4F0, "1f4f0.png");
png_name!(ANDROID, "📱", "mobile phone", U_1F4F1, "1f4f1.png");
png_name!(IPHONE, "📱", "mobile phone", U_1F4F1, "1f4f1.png");
png_name!(MOBILE_PHONE, "📱", "mobile phone", U_1F4F1, "1f4f1.png");
png_name!(CALLING, "📲", "mobile phone with arrow", U_1F4F2, "1f4f2.png");
png_name!(MOBILE_PHONE_ARROW, "📲", "mobile phone with arrow", U_1F4F2, "1f4f2.png");
png_name!(VIBRATION_MODE, "📳", "vibration mode", U_1F4F3, "1f4f3.png");
png_name!(MOBILE_PHONE_OFF, "📴", "mobile phone off", U_1F4F4, "1f4f4.png");
png_name!(NO_MOBILE_PHONES, "📵", "no mobile phones", U_1F4F5, "1f4f5.png");
png_name!(ANTENNA_BARS, "📶", "antenna bars", U_1F4F6, "1f4f6.png");
png_name!(SIGNAL_STRENGTH, "📶", "antenna bars", U_1F4F6, "1f4f6.png");
png_name!(CAMERA, "📷", "camera", U_1F4F7, "1f4f7.png");
png_name!(CAMERA_WITH_FLASH, "📸", "camera with flash", U_1F4F8, "1f4f8.png");
png_name!(VIDEO_CAMERA, "📹", "video camera", U_1F4F9, "1f4f9.png");
png_name!(TV, "📺", "television", U_1F4FA, "1f4fa.png");
png_name!(RADIO, "📻", "radio", U_1F4FB, "1f4fb.png");
png_name!(VHS, "📼", "videocassette", U_1F4FC, "1f4fc.png");
png_name!(VIDEOCASSETTE, "📼", "videocassette", U_1F4FC, "1f4fc.png");
png_name!(FILM_PROJECTOR, "📽", "film projector", U_1F4FD, "1f4fd.png");
png_name!(PRAYER_BEADS, "📿", "prayer beads", U_1F4FF, "1f4ff.png");
png_name!(SHUFFLE, "🔀", "shuffle tracks button", U_1F500, "1f500.png");
png_name!(TWISTED_RIGHTWARDS_ARROWS, "🔀", "shuffle tracks button", U_1F500, "1f500.png");
png_name!(REPEAT, "🔁", "repeat button", U_1F501, "1f501.png");
png_name!(REPEAT_ONE, "🔂", "repeat single button", U_1F502, "1f502.png");
png_name!(ARROWS_CLOCKWISE, "🔃", "clockwise vertical arrows", U_1F503, "1f503.png");
png_name!(CLOCKWISE, "🔃", "clockwise vertical arrows", U_1F503, "1f503.png");
png_name!(ARROWS_COUNTERCLOCKWISE, "🔄", "counterclockwise arrows button", U_1F504, "1f504.png");
png_name!(COUNTERCLOCKWISE, "🔄", "counterclockwise arrows button", U_1F504, "1f504.png");
png_name!(DIM_BUTTON, "🔅", "dim button", U_1F505, "1f505.png");
png_name!(LOW_BRIGHTNESS, "🔅", "dim button", U_1F505, "1f505.png");
png_name!(BRIGHT_BUTTON, "🔆", "bright button", U_1F506, "1f506.png");
png_name!(HIGH_BRIGHTNESS, "🔆", "bright button", U_1F506, "1f506.png");
png_name!(MUTE, "🔇", "muted speaker", U_1F507, "1f507.png");
png_name!(NO_SOUND, "🔇", "muted speaker", U_1F507, "1f507.png");
png_name!(LOW_VOLUME, "🔈", "speaker low volume", U_1F508, "1f508.png");
png_name!(QUIET_SOUND, "🔈", "speaker low volume", U_1F508, "1f508.png");
png_name!(SPEAKER, "🔈", "speaker low volume", U_1F508, "1f508.png");
png_name!(MEDIUM_VOLUMNE, "🔉", "speaker medium volume", U_1F509, "1f509.png");
png_name!(SOUND, "🔉", "speaker medium volume", U_1F509, "1f509.png");
png_name!(HIGH_VOLUME, "🔊", "speaker high volume", U_1F50A, "1f50a.png");
png_name!(LOUD_SOUND, "🔊", "speaker high volume", U_1F50A, "1f50a.png");
png_name!(BATTERY, "🔋", "battery", U_1F50B, "1f50b.png");
png_name!(ELECTRIC_PLUG, "🔌", "electric plug", U_1F50C, "1f50c.png");
png_name!(MAG, "🔍", "magnifying glass tilted left", U_1F50D, "1f50d.png");
png_name!(MAG_RIGHT, "🔎", "magnifying glass tilted right", U_1F50E, "1f50e.png");
png_name!(LOCK_WITH_INK_PEN, "🔏", "locked with pen", U_1F50F, "1f50f.png");
png_name!(LOCKED_WITH_PEN, "🔏", "locked with pen", U_1F50F, "1f50f.png");
png_name!(CLOSED_LOCK_WITH_KEY, "🔐", "locked with key", U_1F510, "1f510.png");
png_name!(LOCKED_WITH_KEY, "🔐", "locked with key", U_1F510, "1f510.png");
png_name!(KEY, "🔑", "key", U_1F511, "1f511.png");
png_name!(LOCK, "🔒", "locked", U_1F512, "1f512.png");
png_name!(LOCKED, "🔒", "locked", U_1F512, "1f512.png");
png_name!(UNLOCK, "🔓", "unlocked", U_1F513, "1f513.png");
png_name!(UNLOCKED, "🔓", "unlocked", U_1F513, "1f513.png");
png_name!(BELL, "🔔", "bell", U_1F514, "1f514.png");
png_name!(NO_BELL, "🔕", "bell with slash", U_1F515, "1f515.png");
png_name!(BOOKMARK, "🔖", "bookmark", U_1F516, "1f516.png");
png_name!(LINK, "🔗", "link", U_1F517, "1f517.png");
png_name!(RADIO_BUTTON, "🔘", "radio button", U_1F518, "1f518.png");
png_name!(BACK, "🔙", "BACK arrow", U_1F519, "1f519.png");
png_name!(END, "🔚", "END arrow", U_1F51A, "1f51a.png");
png_name!(ON, "🔛", "ON! arrow", U_1F51B, "1f51b.png");
png_name!(SOON, "🔜", "SOON arrow", U_1F51C, "1f51c.png");
png_name!(TOP, "🔝", "TOP arrow", U_1F51D, "1f51d.png");
png_name!(NO_ONE_UNDER_18, "🔞", "no one under eighteen", U_1F51E, "1f51e.png");
png_name!(UNDERAGE, "🔞", "no one under eighteen", U_1F51E, "1f51e.png");
png_name!(TEN, "🔟", "keycap: 10", U_1F51F, "1f51f.png");
png_name!(CAPITAL_ABCD, "🔠", "input latin uppercase", U_1F520, "1f520.png");
png_name!(ABCD, "🔡", "input latin lowercase", U_1F521, "1f521.png");
png_name!(X_1234, "🔢", "input numbers", U_1F522, "1f522.png");
png_name!(SYMBOLS, "🔣", "input symbols", U_1F523, "1f523.png");
png_name!(ABC, "🔤", "input latin letters", U_1F524, "1f524.png");
png_name!(FIRE, "🔥", "fire", U_1F525, "1f525.png");
png_name!(FLASHLIGHT, "🔦", "flashlight", U_1F526, "1f526.png");
png_name!(WRENCH, "🔧", "wrench", U_1F527, "1f527.png");
png_name!(HAMMER, "🔨", "hammer", U_1F528, "1f528.png");
png_name!(NUT_AND_BOLT, "🔩", "nut and bolt", U_1F529, "1f529.png");
png_name!(KNIFE, "🔪", "kitchen knife", U_1F52A, "1f52a.png");
png_name!(GUN, "🔫", "water pistol", U_1F52B, "1f52b.png");
png_name!(PISTOL, "🔫", "water pistol", U_1F52B, "1f52b.png");
png_name!(MICROSCOPE, "🔬", "microscope", U_1F52C, "1f52c.png");
png_name!(TELESCOPE, "🔭", "telescope", U_1F52D, "1f52d.png");
png_name!(CRYSTAL_BALL, "🔮", "crystal ball", U_1F52E, "1f52e.png");
png_name!(SIX_POINTED_STAR, "🔯", "dotted six-pointed star", U_1F52F, "1f52f.png");
png_name!(BEGINNER, "🔰", "Japanese symbol for beginner", U_1F530, "1f530.png");
png_name!(TRIDENT, "🔱", "trident emblem", U_1F531, "1f531.png");
png_name!(BLACK_SQUARE_BUTTON, "🔲", "black square button", U_1F532, "1f532.png");
png_name!(WHITE_SQUARE_BUTTON, "🔳", "white square button", U_1F533, "1f533.png");
png_name!(RED_CIRCLE, "🔴", "red circle", U_1F534, "1f534.png");
png_name!(BLUE_CIRCLE, "🔵", "blue circle", U_1F535, "1f535.png");
png_name!(LARGE_ORANGE_DIAMOND, "🔶", "large orange diamond", U_1F536, "1f536.png");
png_name!(LARGE_BLUE_DIAMOND, "🔷", "large blue diamond", U_1F537, "1f537.png");
png_name!(SMALL_ORANGE_DIAMOND, "🔸", "small orange diamond", U_1F538, "1f538.png");
png_name!(SMALL_BLUE_DIAMOND, "🔹", "small blue diamond", U_1F539, "1f539.png");
png_name!(SMALL_RED_TRIANGLE, "🔺", "red triangle pointed up", U_1F53A, "1f53a.png");
png_name!(SMALL_RED_TRIANGLE_DOWN, "🔻", "red triangle pointed down", U_1F53B, "1f53b.png");
png_name!(ARROW_UP_SMALL, "🔼", "upwards button", U_1F53C, "1f53c.png");
png_name!(UP, "🔼", "upwards button", U_1F53C, "1f53c.png");
png_name!(ARROW_DOWN_SMALL, "🔽", "downwards button", U_1F53D, "1f53d.png");
png_name!(DOWN, "🔽", "downwards button", U_1F53D, "1f53d.png");
png_name!(OM, "🕉", "om", U_1F549, "1f549.png");
png_name!(DOVE, "🕊", "dove", U_1F54A, "1f54a.png");
png_name!(KAABA, "🕋", "kaaba", U_1F54B, "1f54b.png");
png_name!(MOSQUE, "🕌", "mosque", U_1F54C, "1f54c.png");
png_name!(SYNAGOGUE, "🕍", "synagogue", U_1F54D, "1f54d.png");
png_name!(MENORAH, "🕎", "menorah", U_1F54E, "1f54e.png");
png_name!(CLOCK1, "🕐", "one o’clock", U_1F550, "1f550.png");
png_name!(CLOCK2, "🕑", "two o’clock", U_1F551, "1f551.png");
png_name!(CLOCK3, "🕒", "three o’clock", U_1F552, "1f552.png");
png_name!(CLOCK4, "🕓", "four o’clock", U_1F553, "1f553.png");
png_name!(CLOCK5, "🕔", "five o’clock", U_1F554, "1f554.png");
png_name!(CLOCK6, "🕕", "six o’clock", U_1F555, "1f555.png");
png_name!(CLOCK7, "🕖", "seven o’clock", U_1F556, "1f556.png");
png_name!(CLOCK8, "🕗", "eight o’clock", U_1F557, "1f557.png");
png_name!(CLOCK9, "🕘", "nine o’clock", U_1F558, "1f558.png");
png_name!(CLOCK10, "🕙", "ten o’clock", U_1F559, "1f559.png");
png_name!(CLOCK11, "🕚", "eleven o’clock", U_1F55A, "1f55a.png");
png_name!(CLOCK12, "🕛", "twelve o’clock", U_1F55B, "1f55b.png");
png_name!(CLOCK130, "🕜", "one-thirty", U_1F55C, "1f55c.png");
png_name!(CLOCK230, "🕝", "two-thirty", U_1F55D, "1f55d.png");
png_name!(CLOCK330, "🕞", "three-thirty", U_1F55E, "1f55e.png");
png_name!(CLOCK430, "🕟", "four-thirty", U_1F55F, "1f55f.png");
png_name!(CLOCK530, "🕠", "five-thirty", U_1F560, "1f560.png");
png_name!(CLOCK630, "🕡", "six-thirty", U_1F561, "1f561.png");
png_name!(CLOCK730, "🕢", "seven-thirty", U_1F562, "1f562.png");
png_name!(CLOCK830, "🕣", "eight-thirty", U_1F563, "1f563.png");
png_name!(CLOCK930, "🕤", "nine-thirty", U_1F564, "1f564.png");
png_name!(CLOCK1030, "🕥", "ten-thirty", U_1F565, "1f565.png");
png_name!(CLOCK1130, "🕦", "eleven-thirty", U_1F566, "1f566.png");
png_name!(CLOCK1230, "🕧", "twelve-thirty", U_1F567, "1f567.png");
png_name!(CANDLE, "🕯", "candle", U_1F56F, "1f56f.png");
png_name!(CLOCK, "🕰", "mantelpiece clock", U_1F570, "1f570.png");
png_name!(HOLE, "🕳", "hole", U_1F573, "1f573.png");
png_name!(LEVITATE_TONE1, "🕴🏻", "", U_1F574_1F3FB, "1f574-1f3fb.png");
png_name!(LEVITATING_TONE1, "🕴🏻", "", U_1F574_1F3FB, "1f574-1f3fb.png");
png_name!(PERSON_IN_SUIT_LEVITATING_TONE1, "🕴🏻", "", U_1F574_1F3FB, "1f574-1f3fb.png");
png_name!(LEVITATE_TONE2, "🕴🏼", "", U_1F574_1F3FC, "1f574-1f3fc.png");
png_name!(LEVITATING_TONE2, "🕴🏼", "", U_1F574_1F3FC, "1f574-1f3fc.png");
png_name!(PERSON_IN_SUIT_LEVITATING_TONE2, "🕴🏼", "", U_1F574_1F3FC, "1f574-1f3fc.png");
png_name!(LEVITATE_TONE3, "🕴🏽", "", U_1F574_1F3FD, "1f574-1f3fd.png");
png_name!(LEVITATING_TONE3, "🕴🏽", "", U_1F574_1F3FD, "1f574-1f3fd.png");
png_name!(PERSON_IN_SUIT_LEVITATING_TONE3, "🕴🏽", "", U_1F574_1F3FD, "1f574-1f3fd.png");
png_name!(LEVITATE_TONE4, "🕴🏾", "", U_1F574_1F3FE, "1f574-1f3fe.png");
png_name!(LEVITATING_TONE4, "🕴🏾", "", U_1F574_1F3FE, "1f574-1f3fe.png");
png_name!(PERSON_IN_SUIT_LEVITATING_TONE4, "🕴🏾", "", U_1F574_1F3FE, "1f574-1f3fe.png");
png_name!(LEVITATE_TONE5, "🕴🏿", "", U_1F574_1F3FF, "1f574-1f3ff.png");
png_name!(LEVITATING_TONE5, "🕴🏿", "", U_1F574_1F3FF, "1f574-1f3ff.png");
png_name!(PERSON_IN_SUIT_LEVITATING_TONE5, "🕴🏿", "", U_1F574_1F3FF, "1f574-1f3ff.png");
png_name!(LEVITATE, "🕴", "person in suit levitating", U_1F574, "1f574.png");
png_name!(LEVITATING, "🕴", "person in suit levitating", U_1F574, "1f574.png");
png_name!(PERSON_IN_SUIT_LEVITATING, "🕴", "person in suit levitating", U_1F574, "1f574.png");
png_name!(WOMAN_DETECTIVE_TONE1, "🕵🏻‍♀️", "", U_1F575_1F3FB_200D_2640_FE0F, "1f575-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_DETECTIVE_TONE1, "🕵🏻‍♂️", "", U_1F575_1F3FB_200D_2642_FE0F, "1f575-1f3fb-200d-2642-fe0f.png");
png_name!(DETECTIVE_TONE1, "🕵🏻", "", U_1F575_1F3FB, "1f575-1f3fb.png");
png_name!(WOMAN_DETECTIVE_TONE2, "🕵🏼‍♀️", "", U_1F575_1F3FC_200D_2640_FE0F, "1f575-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_DETECTIVE_TONE2, "🕵🏼‍♂️", "", U_1F575_1F3FC_200D_2642_FE0F, "1f575-1f3fc-200d-2642-fe0f.png");
png_name!(DETECTIVE_TONE2, "🕵🏼", "", U_1F575_1F3FC, "1f575-1f3fc.png");
png_name!(WOMAN_DETECTIVE_TONE3, "🕵🏽‍♀️", "", U_1F575_1F3FD_200D_2640_FE0F, "1f575-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_DETECTIVE_TONE3, "🕵🏽‍♂️", "", U_1F575_1F3FD_200D_2642_FE0F, "1f575-1f3fd-200d-2642-fe0f.png");
png_name!(DETECTIVE_TONE3, "🕵🏽", "", U_1F575_1F3FD, "1f575-1f3fd.png");
png_name!(WOMAN_DETECTIVE_TONE4, "🕵🏾‍♀️", "", U_1F575_1F3FE_200D_2640_FE0F, "1f575-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_DETECTIVE_TONE4, "🕵🏾‍♂️", "", U_1F575_1F3FE_200D_2642_FE0F, "1f575-1f3fe-200d-2642-fe0f.png");
png_name!(DETECTIVE_TONE4, "🕵🏾", "", U_1F575_1F3FE, "1f575-1f3fe.png");
png_name!(WOMAN_DETECTIVE_TONE5, "🕵🏿‍♀️", "", U_1F575_1F3FF_200D_2640_FE0F, "1f575-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_DETECTIVE_TONE5, "🕵🏿‍♂️", "", U_1F575_1F3FF_200D_2642_FE0F, "1f575-1f3ff-200d-2642-fe0f.png");
png_name!(DETECTIVE_TONE5, "🕵🏿", "", U_1F575_1F3FF, "1f575-1f3ff.png");
png_name!(WOMAN_DETECTIVE, "🕵️‍♀️", "woman detective", U_1F575_FE0F_200D_2640_FE0F, "1f575-fe0f-200d-2640-fe0f.png");
png_name!(MAN_DETECTIVE, "🕵️‍♂️", "man detective", U_1F575_FE0F_200D_2642_FE0F, "1f575-fe0f-200d-2642-fe0f.png");
png_name!(DETECTIVE, "🕵", "detective", U_1F575, "1f575.png");
png_name!(SUNGLASSES, "🕶", "sunglasses", U_1F576, "1f576.png");
png_name!(SPIDER, "🕷", "spider", U_1F577, "1f577.png");
png_name!(SPIDER_WEB, "🕸", "spider web", U_1F578, "1f578.png");
png_name!(JOYSTICK, "🕹", "joystick", U_1F579, "1f579.png");
png_name!(MAN_DANCING_TONE1, "🕺🏻", "", U_1F57A_1F3FB, "1f57a-1f3fb.png");
png_name!(MAN_DANCING_TONE2, "🕺🏼", "", U_1F57A_1F3FC, "1f57a-1f3fc.png");
png_name!(MAN_DANCING_TONE3, "🕺🏽", "", U_1F57A_1F3FD, "1f57a-1f3fd.png");
png_name!(MAN_DANCING_TONE4, "🕺🏾", "", U_1F57A_1F3FE, "1f57a-1f3fe.png");
png_name!(MAN_DANCING_TONE5, "🕺🏿", "", U_1F57A_1F3FF, "1f57a-1f3ff.png");
png_name!(MAN_DANCING, "🕺", "man dancing", U_1F57A, "1f57a.png");
png_name!(PAPERCLIPS, "🖇", "linked paperclips", U_1F587, "1f587.png");
png_name!(PEN, "🖊", "pen", U_1F58A, "1f58a.png");
png_name!(FOUNTAIN_PEN, "🖋", "fountain pen", U_1F58B, "1f58b.png");
png_name!(PAINTBRUSH, "🖌", "paintbrush", U_1F58C, "1f58c.png");
png_name!(CRAYON, "🖍", "crayon", U_1F58D, "1f58d.png");
png_name!(RAISED_HAND_WITH_FINGERS_SPLAYED_TONE1, "🖐🏻", "", U_1F590_1F3FB, "1f590-1f3fb.png");
png_name!(RAISED_HAND_WITH_FINGERS_SPLAYED_TONE2, "🖐🏼", "", U_1F590_1F3FC, "1f590-1f3fc.png");
png_name!(RAISED_HAND_WITH_FINGERS_SPLAYED_TONE3, "🖐🏽", "", U_1F590_1F3FD, "1f590-1f3fd.png");
png_name!(RAISED_HAND_WITH_FINGERS_SPLAYED_TONE4, "🖐🏾", "", U_1F590_1F3FE, "1f590-1f3fe.png");
png_name!(RAISED_HAND_WITH_FINGERS_SPLAYED_TONE5, "🖐🏿", "", U_1F590_1F3FF, "1f590-1f3ff.png");
png_name!(RAISED_HAND_WITH_FINGERS_SPLAYED, "🖐", "hand with fingers splayed", U_1F590, "1f590.png");
png_name!(MIDDLE_FINGER_TONE1, "🖕🏻", "", U_1F595_1F3FB, "1f595-1f3fb.png");
png_name!(MIDDLE_FINGER_TONE2, "🖕🏼", "", U_1F595_1F3FC, "1f595-1f3fc.png");
png_name!(MIDDLE_FINGER_TONE3, "🖕🏽", "", U_1F595_1F3FD, "1f595-1f3fd.png");
png_name!(MIDDLE_FINGER_TONE4, "🖕🏾", "", U_1F595_1F3FE, "1f595-1f3fe.png");
png_name!(MIDDLE_FINGER_TONE5, "🖕🏿", "", U_1F595_1F3FF, "1f595-1f3ff.png");
png_name!(MIDDLE_FINGER, "🖕", "middle finger", U_1F595, "1f595.png");
png_name!(VULCAN_TONE1, "🖖🏻", "", U_1F596_1F3FB, "1f596-1f3fb.png");
png_name!(VULCAN_TONE2, "🖖🏼", "", U_1F596_1F3FC, "1f596-1f3fc.png");
png_name!(VULCAN_TONE3, "🖖🏽", "", U_1F596_1F3FD, "1f596-1f3fd.png");
png_name!(VULCAN_TONE4, "🖖🏾", "", U_1F596_1F3FE, "1f596-1f3fe.png");
png_name!(VULCAN_TONE5, "🖖🏿", "", U_1F596_1F3FF, "1f596-1f3ff.png");
png_name!(VULCAN, "🖖", "vulcan salute", U_1F596, "1f596.png");
png_name!(BLACK_HEART, "🖤", "black heart", U_1F5A4, "1f5a4.png");
png_name!(COMPUTER, "🖥", "desktop computer", U_1F5A5, "1f5a5.png");
png_name!(DESKTOP_COMPUTER, "🖥", "desktop computer", U_1F5A5, "1f5a5.png");
png_name!(PRINTER, "🖨", "printer", U_1F5A8, "1f5a8.png");
png_name!(COMPUTER_MOUSE, "🖱", "computer mouse", U_1F5B1, "1f5b1.png");
png_name!(TRACKBALL, "🖲", "trackball", U_1F5B2, "1f5b2.png");
png_name!(FRAME_WITH_PICTURE, "🖼", "framed picture", U_1F5BC, "1f5bc.png");
png_name!(FRAMED_PICTURE, "🖼", "framed picture", U_1F5BC, "1f5bc.png");
png_name!(CARD_INDEX_DIVIDERS, "🗂", "card index dividers", U_1F5C2, "1f5c2.png");
png_name!(CARD_FILE_BOX, "🗃", "card file box", U_1F5C3, "1f5c3.png");
png_name!(FILE_CABINET, "🗄", "file cabinet", U_1F5C4, "1f5c4.png");
png_name!(TRASHCAN, "🗑", "wastebasket", U_1F5D1, "1f5d1.png");
png_name!(WASTEBASKET, "🗑", "wastebasket", U_1F5D1, "1f5d1.png");
png_name!(NOTEPAD_SPIRAL, "🗒", "spiral notepad", U_1F5D2, "1f5d2.png");
png_name!(CALENDAR_SPIRAL, "🗓", "spiral calendar", U_1F5D3, "1f5d3.png");
png_name!(CLAMP, "🗜", "clamp", U_1F5DC, "1f5dc.png");
png_name!(COMPRESSION, "🗜", "clamp", U_1F5DC, "1f5dc.png");
png_name!(OLD_KEY, "🗝", "old key", U_1F5DD, "1f5dd.png");
png_name!(ROLLED_UP_NEWSPAPER, "🗞", "rolled-up newspaper", U_1F5DE, "1f5de.png");
png_name!(DAGGER, "🗡", "dagger", U_1F5E1, "1f5e1.png");
png_name!(SPEAKING_HEAD, "🗣", "speaking head", U_1F5E3, "1f5e3.png");
png_name!(LEFT_SPEECH_BUBBLE, "🗨", "left speech bubble", U_1F5E8, "1f5e8.png");
png_name!(RIGHT_ANGER_BUBBLE, "🗯", "right anger bubble", U_1F5EF, "1f5ef.png");
png_name!(BALLOT_BOX, "🗳", "ballot box with ballot", U_1F5F3, "1f5f3.png");
png_name!(WORLD_MAP, "🗺", "world map", U_1F5FA, "1f5fa.png");
png_name!(MOUNT_FUJI, "🗻", "mount fuji", U_1F5FB, "1f5fb.png");
png_name!(TOKYO_TOWER, "🗼", "Tokyo tower", U_1F5FC, "1f5fc.png");
png_name!(STATUE_OF_LIBERTY, "🗽", "Statue of Liberty", U_1F5FD, "1f5fd.png");
png_name!(JAPAN_MAP, "🗾", "map of Japan", U_1F5FE, "1f5fe.png");
png_name!(MOAI, "🗿", "moai", U_1F5FF, "1f5ff.png");
png_name!(MOYAI, "🗿", "moai", U_1F5FF, "1f5ff.png");
png_name!(GRINNING, "😀", "grinning face", U_1F600, "1f600.png");
png_name!(GRINNING_FACE, "😀", "grinning face", U_1F600, "1f600.png");
png_name!(BEAMING_FACE, "😁", "beaming face with smiling eyes", U_1F601, "1f601.png");
png_name!(GRIN, "😁", "beaming face with smiling eyes", U_1F601, "1f601.png");
png_name!(JOY, "😂", "face with tears of joy", U_1F602, "1f602.png");
png_name!(LMAO, "😂", "face with tears of joy", U_1F602, "1f602.png");
png_name!(TEARS_OF_JOY, "😂", "face with tears of joy", U_1F602, "1f602.png");
png_name!(GRINNING_FACE_WITH_BIG_EYES, "😃", "grinning face with big eyes", U_1F603, "1f603.png");
png_name!(SMILEY, "😃", "grinning face with big eyes", U_1F603, "1f603.png");
png_name!(GRINNING_FACE_WITH_CLOSED_EYES, "😄", "grinning face with smiling eyes", U_1F604, "1f604.png");
png_name!(SMILE, "😄", "grinning face with smiling eyes", U_1F604, "1f604.png");
png_name!(GRINNING_FACE_WITH_SWEAT, "😅", "grinning face with sweat", U_1F605, "1f605.png");
png_name!(SWEAT_SMILE, "😅", "grinning face with sweat", U_1F605, "1f605.png");
png_name!(LAUGHING, "😆", "grinning squinting face", U_1F606, "1f606.png");
png_name!(LOL, "😆", "grinning squinting face", U_1F606, "1f606.png");
png_name!(SATISFIED, "😆", "grinning squinting face", U_1F606, "1f606.png");
png_name!(SQUINTING_FACE, "😆", "grinning squinting face", U_1F606, "1f606.png");
png_name!(HALO, "😇", "smiling face with halo", U_1F607, "1f607.png");
png_name!(INNOCENT, "😇", "smiling face with halo", U_1F607, "1f607.png");
png_name!(SMILING_IMP, "😈", "smiling face with horns", U_1F608, "1f608.png");
png_name!(WINK, "😉", "winking face", U_1F609, "1f609.png");
png_name!(WINKING_FACE, "😉", "winking face", U_1F609, "1f609.png");
png_name!(BLUSH, "😊", "smiling face with smiling eyes", U_1F60A, "1f60a.png");
png_name!(SMILING_FACE_WITH_CLOSED_EYES, "😊", "smiling face with smiling eyes", U_1F60A, "1f60a.png");
png_name!(SAVORING_FOOD, "😋", "face savoring food", U_1F60B, "1f60b.png");
png_name!(YUM, "😋", "face savoring food", U_1F60B, "1f60b.png");
png_name!(RELIEVED, "😌", "relieved face", U_1F60C, "1f60c.png");
png_name!(RELIEVED_FACE, "😌", "relieved face", U_1F60C, "1f60c.png");
png_name!(HEART_EYES, "😍", "smiling face with heart-eyes", U_1F60D, "1f60d.png");
png_name!(SMILING_FACE_WITH_HEART_EYES, "😍", "smiling face with heart-eyes", U_1F60D, "1f60d.png");
png_name!(SMILING_FACE_WITH_SUNGLASSES, "😎", "smiling face with sunglasses", U_1F60E, "1f60e.png");
png_name!(SUNGLASSES_COOL, "😎", "smiling face with sunglasses", U_1F60E, "1f60e.png");
png_name!(TOO_COOL, "😎", "smiling face with sunglasses", U_1F60E, "1f60e.png");
png_name!(SMIRK, "😏", "smirking face", U_1F60F, "1f60f.png");
png_name!(SMIRKING, "😏", "smirking face", U_1F60F, "1f60f.png");
png_name!(SMIRKING_FACE, "😏", "smirking face", U_1F60F, "1f60f.png");
png_name!(NEUTRAL, "😐", "neutral face", U_1F610, "1f610.png");
png_name!(NEUTRAL_FACE, "😐", "neutral face", U_1F610, "1f610.png");
png_name!(EXPRESSIONLESS, "😑", "expressionless face", U_1F611, "1f611.png");
png_name!(EXPRESSIONLESS_FACE, "😑", "expressionless face", U_1F611, "1f611.png");
png_name!(UNAMUSED, "😒", "unamused face", U_1F612, "1f612.png");
png_name!(UNAMUSED_FACE, "😒", "unamused face", U_1F612, "1f612.png");
png_name!(DOWNCAST_FACE, "😓", "downcast face with sweat", U_1F613, "1f613.png");
png_name!(SWEAT, "😓", "downcast face with sweat", U_1F613, "1f613.png");
png_name!(PENSIVE, "😔", "pensive face", U_1F614, "1f614.png");
png_name!(PENSIVE_FACE, "😔", "pensive face", U_1F614, "1f614.png");
png_name!(CONFUSED, "😕", "confused face", U_1F615, "1f615.png");
png_name!(CONFUSED_FACE, "😕", "confused face", U_1F615, "1f615.png");
png_name!(CONFOUNDED, "😖", "confounded face", U_1F616, "1f616.png");
png_name!(CONFOUNDED_FACE, "😖", "confounded face", U_1F616, "1f616.png");
png_name!(KISSING, "😗", "kissing face", U_1F617, "1f617.png");
png_name!(KISSING_FACE, "😗", "kissing face", U_1F617, "1f617.png");
png_name!(BLOWING_A_KISS, "😘", "face blowing a kiss", U_1F618, "1f618.png");
png_name!(KISSING_HEART, "😘", "face blowing a kiss", U_1F618, "1f618.png");
png_name!(KISSING_FACE_WITH_SMILING_EYES, "😙", "kissing face with smiling eyes", U_1F619, "1f619.png");
png_name!(KISSING_SMILING_EYES, "😙", "kissing face with smiling eyes", U_1F619, "1f619.png");
png_name!(KISSING_CLOSED_EYES, "😚", "kissing face with closed eyes", U_1F61A, "1f61a.png");
png_name!(KISSING_FACE_WITH_CLOSED_EYES, "😚", "kissing face with closed eyes", U_1F61A, "1f61a.png");
png_name!(FACE_WITH_TONGUE, "😛", "face with tongue", U_1F61B, "1f61b.png");
png_name!(STUCK_OUT_TONGUE, "😛", "face with tongue", U_1F61B, "1f61b.png");
png_name!(STUCK_OUT_TONGUE_WINKING_EYE, "😜", "winking face with tongue", U_1F61C, "1f61c.png");
png_name!(STUCK_OUT_TONGUE_CLOSED_EYES, "😝", "squinting face with tongue", U_1F61D, "1f61d.png");
png_name!(DISAPPOINTED, "😞", "disappointed face", U_1F61E, "1f61e.png");
png_name!(DISAPPOINTED_FACE, "😞", "disappointed face", U_1F61E, "1f61e.png");
png_name!(WORRIED, "😟", "worried face", U_1F61F, "1f61f.png");
png_name!(WORRIED_FACE, "😟", "worried face", U_1F61F, "1f61f.png");
png_name!(ANGRY, "😠", "angry face", U_1F620, "1f620.png");
png_name!(ANGRY_FACE, "😠", "angry face", U_1F620, "1f620.png");
png_name!(POUT, "😡", "enraged face", U_1F621, "1f621.png");
png_name!(POUTING_FACE, "😡", "enraged face", U_1F621, "1f621.png");
png_name!(RAGE, "😡", "enraged face", U_1F621, "1f621.png");
png_name!(CRY, "😢", "crying face", U_1F622, "1f622.png");
png_name!(CRYING_FACE, "😢", "crying face", U_1F622, "1f622.png");
png_name!(PERSEVERE, "😣", "persevering face", U_1F623, "1f623.png");
png_name!(PERSEVERING_FACE, "😣", "persevering face", U_1F623, "1f623.png");
png_name!(NOSE_STEAM, "😤", "face with steam from nose", U_1F624, "1f624.png");
png_name!(TRIUMPH, "😤", "face with steam from nose", U_1F624, "1f624.png");
png_name!(DISAPPOINTED_RELIEVED, "😥", "sad but relieved face", U_1F625, "1f625.png");
png_name!(SAD_RELIEVED_FACE, "😥", "sad but relieved face", U_1F625, "1f625.png");
png_name!(FROWNING, "😦", "frowning face with open mouth", U_1F626, "1f626.png");
png_name!(FROWNING_FACE, "😦", "frowning face with open mouth", U_1F626, "1f626.png");
png_name!(ANGUISHED, "😧", "anguished face", U_1F627, "1f627.png");
png_name!(ANGUISHED_FACE, "😧", "anguished face", U_1F627, "1f627.png");
png_name!(FEARFUL, "😨", "fearful face", U_1F628, "1f628.png");
png_name!(FEARFUL_FACE, "😨", "fearful face", U_1F628, "1f628.png");
png_name!(WEARY, "😩", "weary face", U_1F629, "1f629.png");
png_name!(WEARY_FACE, "😩", "weary face", U_1F629, "1f629.png");
png_name!(SLEEPY, "😪", "sleepy face", U_1F62A, "1f62a.png");
png_name!(SLEEPY_FACE, "😪", "sleepy face", U_1F62A, "1f62a.png");
png_name!(TIRED, "😫", "tired face", U_1F62B, "1f62b.png");
png_name!(TIRED_FACE, "😫", "tired face", U_1F62B, "1f62b.png");
png_name!(GRIMACING, "😬", "grimacing face", U_1F62C, "1f62c.png");
png_name!(GRIMACING_FACE, "😬", "grimacing face", U_1F62C, "1f62c.png");
png_name!(LOUDLY_CRYING_FACE, "😭", "loudly crying face", U_1F62D, "1f62d.png");
png_name!(SOB, "😭", "loudly crying face", U_1F62D, "1f62d.png");
png_name!(EXHALE, "😮‍💨", "face exhaling", U_1F62E_200D_1F4A8, "1f62e-200d-1f4a8.png");
png_name!(EXHALING, "😮‍💨", "face exhaling", U_1F62E_200D_1F4A8, "1f62e-200d-1f4a8.png");
png_name!(FACE_WITH_OPEN_MOUTH, "😮", "face with open mouth", U_1F62E, "1f62e.png");
png_name!(OPEN_MOUTH, "😮", "face with open mouth", U_1F62E, "1f62e.png");
png_name!(HUSHED, "😯", "hushed face", U_1F62F, "1f62f.png");
png_name!(HUSHED_FACE, "😯", "hushed face", U_1F62F, "1f62f.png");
png_name!(ANXIOUS, "😰", "anxious face with sweat", U_1F630, "1f630.png");
png_name!(ANXIOUS_FACE, "😰", "anxious face with sweat", U_1F630, "1f630.png");
png_name!(COLD_SWEAT, "😰", "anxious face with sweat", U_1F630, "1f630.png");
png_name!(SCREAM, "😱", "face screaming in fear", U_1F631, "1f631.png");
png_name!(SCREAMING_IN_FEAR, "😱", "face screaming in fear", U_1F631, "1f631.png");
png_name!(ASTONISHED, "😲", "astonished face", U_1F632, "1f632.png");
png_name!(ASTONISHED_FACE, "😲", "astonished face", U_1F632, "1f632.png");
png_name!(FLUSHED, "😳", "flushed face", U_1F633, "1f633.png");
png_name!(FLUSHED_FACE, "😳", "flushed face", U_1F633, "1f633.png");
png_name!(SLEEPING, "😴", "sleeping face", U_1F634, "1f634.png");
png_name!(SLEEPING_FACE, "😴", "sleeping face", U_1F634, "1f634.png");
png_name!(DIZZY_EYES, "😵‍💫", "face with spiral eyes", U_1F635_200D_1F4AB, "1f635-200d-1f4ab.png");
png_name!(DIZZY_FACE, "😵", "face with crossed-out eyes", U_1F635, "1f635.png");
png_name!(KNOCKED_OUT, "😵", "face with crossed-out eyes", U_1F635, "1f635.png");
png_name!(IN_CLOUDS, "😶‍🌫️", "face in clouds", U_1F636_200D_1F32B_FE0F, "1f636-200d-1f32b-fe0f.png");
png_name!(NO_MOUTH, "😶", "face without mouth", U_1F636, "1f636.png");
png_name!(MASK, "😷", "face with medical mask", U_1F637, "1f637.png");
png_name!(MEDICAL_MASK, "😷", "face with medical mask", U_1F637, "1f637.png");
png_name!(GRINNING_CAT_WITH_CLOSED_EYES, "😸", "grinning cat with smiling eyes", U_1F638, "1f638.png");
png_name!(SMILE_CAT, "😸", "grinning cat with smiling eyes", U_1F638, "1f638.png");
png_name!(JOY_CAT, "😹", "cat with tears of joy", U_1F639, "1f639.png");
png_name!(TEARS_OF_JOY_CAT, "😹", "cat with tears of joy", U_1F639, "1f639.png");
png_name!(GRINNING_CAT, "😺", "grinning cat", U_1F63A, "1f63a.png");
png_name!(SMILEY_CAT, "😺", "grinning cat", U_1F63A, "1f63a.png");
png_name!(HEART_EYES_CAT, "😻", "smiling cat with heart-eyes", U_1F63B, "1f63b.png");
png_name!(SMILING_CAT_WITH_HEART_EYES, "😻", "smiling cat with heart-eyes", U_1F63B, "1f63b.png");
png_name!(SMIRK_CAT, "😼", "cat with wry smile", U_1F63C, "1f63c.png");
png_name!(WRY_SMILE_CAT, "😼", "cat with wry smile", U_1F63C, "1f63c.png");
png_name!(KISSING_CAT, "😽", "kissing cat", U_1F63D, "1f63d.png");
png_name!(POUTING_CAT, "😾", "pouting cat", U_1F63E, "1f63e.png");
png_name!(CRYING_CAT, "😿", "crying cat", U_1F63F, "1f63f.png");
png_name!(SCREAM_CAT, "🙀", "weary cat", U_1F640, "1f640.png");
png_name!(WEARY_CAT, "🙀", "weary cat", U_1F640, "1f640.png");
png_name!(SLIGHTLY_FROWNING_FACE, "🙁", "slightly frowning face", U_1F641, "1f641.png");
png_name!(SLIGHTLY_SMILING_FACE, "🙂", "slightly smiling face", U_1F642, "1f642.png");
png_name!(UPSIDE_DOWN_FACE, "🙃", "upside-down face", U_1F643, "1f643.png");
png_name!(ROLLING_EYES, "🙄", "face with rolling eyes", U_1F644, "1f644.png");
png_name!(WOMAN_GESTURING_NO_TONE1, "🙅🏻‍♀️", "", U_1F645_1F3FB_200D_2640_FE0F, "1f645-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_GESTURING_NO_TONE1, "🙅🏻‍♂️", "", U_1F645_1F3FB_200D_2642_FE0F, "1f645-1f3fb-200d-2642-fe0f.png");
png_name!(NO_GOOD_TONE1, "🙅🏻", "", U_1F645_1F3FB, "1f645-1f3fb.png");
png_name!(PERSON_GESTURING_NO_TONE1, "🙅🏻", "", U_1F645_1F3FB, "1f645-1f3fb.png");
png_name!(WOMAN_GESTURING_NO_TONE2, "🙅🏼‍♀️", "", U_1F645_1F3FC_200D_2640_FE0F, "1f645-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_GESTURING_NO_TONE2, "🙅🏼‍♂️", "", U_1F645_1F3FC_200D_2642_FE0F, "1f645-1f3fc-200d-2642-fe0f.png");
png_name!(NO_GOOD_TONE2, "🙅🏼", "", U_1F645_1F3FC, "1f645-1f3fc.png");
png_name!(PERSON_GESTURING_NO_TONE2, "🙅🏼", "", U_1F645_1F3FC, "1f645-1f3fc.png");
png_name!(WOMAN_GESTURING_NO_TONE3, "🙅🏽‍♀️", "", U_1F645_1F3FD_200D_2640_FE0F, "1f645-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_GESTURING_NO_TONE3, "🙅🏽‍♂️", "", U_1F645_1F3FD_200D_2642_FE0F, "1f645-1f3fd-200d-2642-fe0f.png");
png_name!(NO_GOOD_TONE3, "🙅🏽", "", U_1F645_1F3FD, "1f645-1f3fd.png");
png_name!(PERSON_GESTURING_NO_TONE3, "🙅🏽", "", U_1F645_1F3FD, "1f645-1f3fd.png");
png_name!(WOMAN_GESTURING_NO_TONE4, "🙅🏾‍♀️", "", U_1F645_1F3FE_200D_2640_FE0F, "1f645-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_GESTURING_NO_TONE4, "🙅🏾‍♂️", "", U_1F645_1F3FE_200D_2642_FE0F, "1f645-1f3fe-200d-2642-fe0f.png");
png_name!(NO_GOOD_TONE4, "🙅🏾", "", U_1F645_1F3FE, "1f645-1f3fe.png");
png_name!(PERSON_GESTURING_NO_TONE4, "🙅🏾", "", U_1F645_1F3FE, "1f645-1f3fe.png");
png_name!(WOMAN_GESTURING_NO_TONE5, "🙅🏿‍♀️", "", U_1F645_1F3FF_200D_2640_FE0F, "1f645-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_GESTURING_NO_TONE5, "🙅🏿‍♂️", "", U_1F645_1F3FF_200D_2642_FE0F, "1f645-1f3ff-200d-2642-fe0f.png");
png_name!(NO_GOOD_TONE5, "🙅🏿", "", U_1F645_1F3FF, "1f645-1f3ff.png");
png_name!(PERSON_GESTURING_NO_TONE5, "🙅🏿", "", U_1F645_1F3FF, "1f645-1f3ff.png");
png_name!(WOMAN_GESTURING_NO, "🙅‍♀️", "woman gesturing NO", U_1F645_200D_2640_FE0F, "1f645-200d-2640-fe0f.png");
png_name!(MAN_GESTURING_NO, "🙅‍♂️", "man gesturing NO", U_1F645_200D_2642_FE0F, "1f645-200d-2642-fe0f.png");
png_name!(NO_GOOD, "🙅", "person gesturing NO", U_1F645, "1f645.png");
png_name!(PERSON_GESTURING_NO, "🙅", "person gesturing NO", U_1F645, "1f645.png");
png_name!(WOMAN_GESTURING_OK_TONE1, "🙆🏻‍♀️", "", U_1F646_1F3FB_200D_2640_FE0F, "1f646-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_GESTURING_OK_TONE1, "🙆🏻‍♂️", "", U_1F646_1F3FB_200D_2642_FE0F, "1f646-1f3fb-200d-2642-fe0f.png");
png_name!(ALL_GOOD_TONE1, "🙆🏻", "", U_1F646_1F3FB, "1f646-1f3fb.png");
png_name!(PERSON_GESTURING_OK_TONE1, "🙆🏻", "", U_1F646_1F3FB, "1f646-1f3fb.png");
png_name!(WOMAN_GESTURING_OK_TONE2, "🙆🏼‍♀️", "", U_1F646_1F3FC_200D_2640_FE0F, "1f646-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_GESTURING_OK_TONE2, "🙆🏼‍♂️", "", U_1F646_1F3FC_200D_2642_FE0F, "1f646-1f3fc-200d-2642-fe0f.png");
png_name!(ALL_GOOD_TONE2, "🙆🏼", "", U_1F646_1F3FC, "1f646-1f3fc.png");
png_name!(PERSON_GESTURING_OK_TONE2, "🙆🏼", "", U_1F646_1F3FC, "1f646-1f3fc.png");
png_name!(WOMAN_GESTURING_OK_TONE3, "🙆🏽‍♀️", "", U_1F646_1F3FD_200D_2640_FE0F, "1f646-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_GESTURING_OK_TONE3, "🙆🏽‍♂️", "", U_1F646_1F3FD_200D_2642_FE0F, "1f646-1f3fd-200d-2642-fe0f.png");
png_name!(ALL_GOOD_TONE3, "🙆🏽", "", U_1F646_1F3FD, "1f646-1f3fd.png");
png_name!(PERSON_GESTURING_OK_TONE3, "🙆🏽", "", U_1F646_1F3FD, "1f646-1f3fd.png");
png_name!(WOMAN_GESTURING_OK_TONE4, "🙆🏾‍♀️", "", U_1F646_1F3FE_200D_2640_FE0F, "1f646-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_GESTURING_OK_TONE4, "🙆🏾‍♂️", "", U_1F646_1F3FE_200D_2642_FE0F, "1f646-1f3fe-200d-2642-fe0f.png");
png_name!(ALL_GOOD_TONE4, "🙆🏾", "", U_1F646_1F3FE, "1f646-1f3fe.png");
png_name!(PERSON_GESTURING_OK_TONE4, "🙆🏾", "", U_1F646_1F3FE, "1f646-1f3fe.png");
png_name!(WOMAN_GESTURING_OK_TONE5, "🙆🏿‍♀️", "", U_1F646_1F3FF_200D_2640_FE0F, "1f646-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_GESTURING_OK_TONE5, "🙆🏿‍♂️", "", U_1F646_1F3FF_200D_2642_FE0F, "1f646-1f3ff-200d-2642-fe0f.png");
png_name!(ALL_GOOD_TONE5, "🙆🏿", "", U_1F646_1F3FF, "1f646-1f3ff.png");
png_name!(PERSON_GESTURING_OK_TONE5, "🙆🏿", "", U_1F646_1F3FF, "1f646-1f3ff.png");
png_name!(WOMAN_GESTURING_OK, "🙆‍♀️", "woman gesturing OK", U_1F646_200D_2640_FE0F, "1f646-200d-2640-fe0f.png");
png_name!(MAN_GESTURING_OK, "🙆‍♂️", "man gesturing OK", U_1F646_200D_2642_FE0F, "1f646-200d-2642-fe0f.png");
png_name!(ALL_GOOD, "🙆", "person gesturing OK", U_1F646, "1f646.png");
png_name!(PERSON_GESTURING_OK, "🙆", "person gesturing OK", U_1F646, "1f646.png");
png_name!(WOMAN_BOWING_TONE1, "🙇🏻‍♀️", "", U_1F647_1F3FB_200D_2640_FE0F, "1f647-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_BOWING_TONE1, "🙇🏻‍♂️", "", U_1F647_1F3FB_200D_2642_FE0F, "1f647-1f3fb-200d-2642-fe0f.png");
png_name!(BOW_TONE1, "🙇🏻", "", U_1F647_1F3FB, "1f647-1f3fb.png");
png_name!(PERSON_BOWING_TONE1, "🙇🏻", "", U_1F647_1F3FB, "1f647-1f3fb.png");
png_name!(WOMAN_BOWING_TONE2, "🙇🏼‍♀️", "", U_1F647_1F3FC_200D_2640_FE0F, "1f647-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_BOWING_TONE2, "🙇🏼‍♂️", "", U_1F647_1F3FC_200D_2642_FE0F, "1f647-1f3fc-200d-2642-fe0f.png");
png_name!(BOW_TONE2, "🙇🏼", "", U_1F647_1F3FC, "1f647-1f3fc.png");
png_name!(PERSON_BOWING_TONE2, "🙇🏼", "", U_1F647_1F3FC, "1f647-1f3fc.png");
png_name!(WOMAN_BOWING_TONE3, "🙇🏽‍♀️", "", U_1F647_1F3FD_200D_2640_FE0F, "1f647-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_BOWING_TONE3, "🙇🏽‍♂️", "", U_1F647_1F3FD_200D_2642_FE0F, "1f647-1f3fd-200d-2642-fe0f.png");
png_name!(BOW_TONE3, "🙇🏽", "", U_1F647_1F3FD, "1f647-1f3fd.png");
png_name!(PERSON_BOWING_TONE3, "🙇🏽", "", U_1F647_1F3FD, "1f647-1f3fd.png");
png_name!(WOMAN_BOWING_TONE4, "🙇🏾‍♀️", "", U_1F647_1F3FE_200D_2640_FE0F, "1f647-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_BOWING_TONE4, "🙇🏾‍♂️", "", U_1F647_1F3FE_200D_2642_FE0F, "1f647-1f3fe-200d-2642-fe0f.png");
png_name!(BOW_TONE4, "🙇🏾", "", U_1F647_1F3FE, "1f647-1f3fe.png");
png_name!(PERSON_BOWING_TONE4, "🙇🏾", "", U_1F647_1F3FE, "1f647-1f3fe.png");
png_name!(WOMAN_BOWING_TONE5, "🙇🏿‍♀️", "", U_1F647_1F3FF_200D_2640_FE0F, "1f647-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_BOWING_TONE5, "🙇🏿‍♂️", "", U_1F647_1F3FF_200D_2642_FE0F, "1f647-1f3ff-200d-2642-fe0f.png");
png_name!(BOW_TONE5, "🙇🏿", "", U_1F647_1F3FF, "1f647-1f3ff.png");
png_name!(PERSON_BOWING_TONE5, "🙇🏿", "", U_1F647_1F3FF, "1f647-1f3ff.png");
png_name!(WOMAN_BOWING, "🙇‍♀️", "woman bowing", U_1F647_200D_2640_FE0F, "1f647-200d-2640-fe0f.png");
png_name!(MAN_BOWING, "🙇‍♂️", "man bowing", U_1F647_200D_2642_FE0F, "1f647-200d-2642-fe0f.png");
png_name!(BOW, "🙇", "person bowing", U_1F647, "1f647.png");
png_name!(PERSON_BOWING, "🙇", "person bowing", U_1F647, "1f647.png");
png_name!(SEE_NO_EVIL, "🙈", "see-no-evil monkey", U_1F648, "1f648.png");
png_name!(HEAR_NO_EVIL, "🙉", "hear-no-evil monkey", U_1F649, "1f649.png");
png_name!(SPEAK_NO_EVIL, "🙊", "speak-no-evil monkey", U_1F64A, "1f64a.png");
png_name!(WOMAN_RAISING_HAND_TONE1, "🙋🏻‍♀️", "", U_1F64B_1F3FB_200D_2640_FE0F, "1f64b-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_RAISING_HAND_TONE1, "🙋🏻‍♂️", "", U_1F64B_1F3FB_200D_2642_FE0F, "1f64b-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_RAISING_HAND_TONE1, "🙋🏻", "", U_1F64B_1F3FB, "1f64b-1f3fb.png");
png_name!(WOMAN_RAISING_HAND_TONE2, "🙋🏼‍♀️", "", U_1F64B_1F3FC_200D_2640_FE0F, "1f64b-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_RAISING_HAND_TONE2, "🙋🏼‍♂️", "", U_1F64B_1F3FC_200D_2642_FE0F, "1f64b-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_RAISING_HAND_TONE2, "🙋🏼", "", U_1F64B_1F3FC, "1f64b-1f3fc.png");
png_name!(WOMAN_RAISING_HAND_TONE3, "🙋🏽‍♀️", "", U_1F64B_1F3FD_200D_2640_FE0F, "1f64b-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_RAISING_HAND_TONE3, "🙋🏽‍♂️", "", U_1F64B_1F3FD_200D_2642_FE0F, "1f64b-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_RAISING_HAND_TONE3, "🙋🏽", "", U_1F64B_1F3FD, "1f64b-1f3fd.png");
png_name!(WOMAN_RAISING_HAND_TONE4, "🙋🏾‍♀️", "", U_1F64B_1F3FE_200D_2640_FE0F, "1f64b-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_RAISING_HAND_TONE4, "🙋🏾‍♂️", "", U_1F64B_1F3FE_200D_2642_FE0F, "1f64b-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_RAISING_HAND_TONE4, "🙋🏾", "", U_1F64B_1F3FE, "1f64b-1f3fe.png");
png_name!(WOMAN_RAISING_HAND_TONE5, "🙋🏿‍♀️", "", U_1F64B_1F3FF_200D_2640_FE0F, "1f64b-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_RAISING_HAND_TONE5, "🙋🏿‍♂️", "", U_1F64B_1F3FF_200D_2642_FE0F, "1f64b-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_RAISING_HAND_TONE5, "🙋🏿", "", U_1F64B_1F3FF, "1f64b-1f3ff.png");
png_name!(WOMAN_RAISING_HAND, "🙋‍♀️", "woman raising hand", U_1F64B_200D_2640_FE0F, "1f64b-200d-2640-fe0f.png");
png_name!(MAN_RAISING_HAND, "🙋‍♂️", "man raising hand", U_1F64B_200D_2642_FE0F, "1f64b-200d-2642-fe0f.png");
png_name!(PERSON_RAISING_HAND, "🙋", "person raising hand", U_1F64B, "1f64b.png");
png_name!(RAISED_HANDS_TONE1, "🙌🏻", "", U_1F64C_1F3FB, "1f64c-1f3fb.png");
png_name!(RAISED_HANDS_TONE2, "🙌🏼", "", U_1F64C_1F3FC, "1f64c-1f3fc.png");
png_name!(RAISED_HANDS_TONE3, "🙌🏽", "", U_1F64C_1F3FD, "1f64c-1f3fd.png");
png_name!(RAISED_HANDS_TONE4, "🙌🏾", "", U_1F64C_1F3FE, "1f64c-1f3fe.png");
png_name!(RAISED_HANDS_TONE5, "🙌🏿", "", U_1F64C_1F3FF, "1f64c-1f3ff.png");
png_name!(RAISED_HANDS, "🙌", "raising hands", U_1F64C, "1f64c.png");
png_name!(WOMAN_FROWNING_TONE1, "🙍🏻‍♀️", "", U_1F64D_1F3FB_200D_2640_FE0F, "1f64d-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_FROWNING_TONE1, "🙍🏻‍♂️", "", U_1F64D_1F3FB_200D_2642_FE0F, "1f64d-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_FROWNING_TONE1, "🙍🏻", "", U_1F64D_1F3FB, "1f64d-1f3fb.png");
png_name!(WOMAN_FROWNING_TONE2, "🙍🏼‍♀️", "", U_1F64D_1F3FC_200D_2640_FE0F, "1f64d-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_FROWNING_TONE2, "🙍🏼‍♂️", "", U_1F64D_1F3FC_200D_2642_FE0F, "1f64d-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_FROWNING_TONE2, "🙍🏼", "", U_1F64D_1F3FC, "1f64d-1f3fc.png");
png_name!(WOMAN_FROWNING_TONE3, "🙍🏽‍♀️", "", U_1F64D_1F3FD_200D_2640_FE0F, "1f64d-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_FROWNING_TONE3, "🙍🏽‍♂️", "", U_1F64D_1F3FD_200D_2642_FE0F, "1f64d-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_FROWNING_TONE3, "🙍🏽", "", U_1F64D_1F3FD, "1f64d-1f3fd.png");
png_name!(WOMAN_FROWNING_TONE4, "🙍🏾‍♀️", "", U_1F64D_1F3FE_200D_2640_FE0F, "1f64d-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_FROWNING_TONE4, "🙍🏾‍♂️", "", U_1F64D_1F3FE_200D_2642_FE0F, "1f64d-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_FROWNING_TONE4, "🙍🏾", "", U_1F64D_1F3FE, "1f64d-1f3fe.png");
png_name!(WOMAN_FROWNING_TONE5, "🙍🏿‍♀️", "", U_1F64D_1F3FF_200D_2640_FE0F, "1f64d-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_FROWNING_TONE5, "🙍🏿‍♂️", "", U_1F64D_1F3FF_200D_2642_FE0F, "1f64d-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_FROWNING_TONE5, "🙍🏿", "", U_1F64D_1F3FF, "1f64d-1f3ff.png");
png_name!(WOMAN_FROWNING, "🙍‍♀️", "woman frowning", U_1F64D_200D_2640_FE0F, "1f64d-200d-2640-fe0f.png");
png_name!(MAN_FROWNING, "🙍‍♂️", "man frowning", U_1F64D_200D_2642_FE0F, "1f64d-200d-2642-fe0f.png");
png_name!(PERSON_FROWNING, "🙍", "person frowning", U_1F64D, "1f64d.png");
png_name!(WOMAN_POUTING_TONE1, "🙎🏻‍♀️", "", U_1F64E_1F3FB_200D_2640_FE0F, "1f64e-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_POUTING_TONE1, "🙎🏻‍♂️", "", U_1F64E_1F3FB_200D_2642_FE0F, "1f64e-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_POUTING_TONE1, "🙎🏻", "", U_1F64E_1F3FB, "1f64e-1f3fb.png");
png_name!(POUTING_TONE1, "🙎🏻", "", U_1F64E_1F3FB, "1f64e-1f3fb.png");
png_name!(WOMAN_POUTING_TONE2, "🙎🏼‍♀️", "", U_1F64E_1F3FC_200D_2640_FE0F, "1f64e-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_POUTING_TONE2, "🙎🏼‍♂️", "", U_1F64E_1F3FC_200D_2642_FE0F, "1f64e-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_POUTING_TONE2, "🙎🏼", "", U_1F64E_1F3FC, "1f64e-1f3fc.png");
png_name!(POUTING_TONE2, "🙎🏼", "", U_1F64E_1F3FC, "1f64e-1f3fc.png");
png_name!(WOMAN_POUTING_TONE3, "🙎🏽‍♀️", "", U_1F64E_1F3FD_200D_2640_FE0F, "1f64e-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_POUTING_TONE3, "🙎🏽‍♂️", "", U_1F64E_1F3FD_200D_2642_FE0F, "1f64e-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_POUTING_TONE3, "🙎🏽", "", U_1F64E_1F3FD, "1f64e-1f3fd.png");
png_name!(POUTING_TONE3, "🙎🏽", "", U_1F64E_1F3FD, "1f64e-1f3fd.png");
png_name!(WOMAN_POUTING_TONE4, "🙎🏾‍♀️", "", U_1F64E_1F3FE_200D_2640_FE0F, "1f64e-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_POUTING_TONE4, "🙎🏾‍♂️", "", U_1F64E_1F3FE_200D_2642_FE0F, "1f64e-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_POUTING_TONE4, "🙎🏾", "", U_1F64E_1F3FE, "1f64e-1f3fe.png");
png_name!(POUTING_TONE4, "🙎🏾", "", U_1F64E_1F3FE, "1f64e-1f3fe.png");
png_name!(WOMAN_POUTING_TONE5, "🙎🏿‍♀️", "", U_1F64E_1F3FF_200D_2640_FE0F, "1f64e-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_POUTING_TONE5, "🙎🏿‍♂️", "", U_1F64E_1F3FF_200D_2642_FE0F, "1f64e-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_POUTING_TONE5, "🙎🏿", "", U_1F64E_1F3FF, "1f64e-1f3ff.png");
png_name!(POUTING_TONE5, "🙎🏿", "", U_1F64E_1F3FF, "1f64e-1f3ff.png");
png_name!(WOMAN_POUTING, "🙎‍♀️", "woman pouting", U_1F64E_200D_2640_FE0F, "1f64e-200d-2640-fe0f.png");
png_name!(MAN_POUTING, "🙎‍♂️", "man pouting", U_1F64E_200D_2642_FE0F, "1f64e-200d-2642-fe0f.png");
png_name!(PERSON_POUTING, "🙎", "person pouting", U_1F64E, "1f64e.png");
png_name!(POUTING, "🙎", "person pouting", U_1F64E, "1f64e.png");
png_name!(FOLDED_HANDS_TONE1, "🙏🏻", "", U_1F64F_1F3FB, "1f64f-1f3fb.png");
png_name!(PRAY_TONE1, "🙏🏻", "", U_1F64F_1F3FB, "1f64f-1f3fb.png");
png_name!(FOLDED_HANDS_TONE2, "🙏🏼", "", U_1F64F_1F3FC, "1f64f-1f3fc.png");
png_name!(PRAY_TONE2, "🙏🏼", "", U_1F64F_1F3FC, "1f64f-1f3fc.png");
png_name!(FOLDED_HANDS_TONE3, "🙏🏽", "", U_1F64F_1F3FD, "1f64f-1f3fd.png");
png_name!(PRAY_TONE3, "🙏🏽", "", U_1F64F_1F3FD, "1f64f-1f3fd.png");
png_name!(FOLDED_HANDS_TONE4, "🙏🏾", "", U_1F64F_1F3FE, "1f64f-1f3fe.png");
png_name!(PRAY_TONE4, "🙏🏾", "", U_1F64F_1F3FE, "1f64f-1f3fe.png");
png_name!(FOLDED_HANDS_TONE5, "🙏🏿", "", U_1F64F_1F3FF, "1f64f-1f3ff.png");
png_name!(PRAY_TONE5, "🙏🏿", "", U_1F64F_1F3FF, "1f64f-1f3ff.png");
png_name!(FOLDED_HANDS, "🙏", "folded hands", U_1F64F, "1f64f.png");
png_name!(PRAY, "🙏", "folded hands", U_1F64F, "1f64f.png");
png_name!(ROCKET, "🚀", "rocket", U_1F680, "1f680.png");
png_name!(HELICOPTER, "🚁", "helicopter", U_1F681, "1f681.png");
png_name!(STEAM_LOCOMOTIVE, "🚂", "locomotive", U_1F682, "1f682.png");
png_name!(RAILWAY_CAR, "🚃", "railway car", U_1F683, "1f683.png");
png_name!(BULLETTRAIN_SIDE, "🚄", "high-speed train", U_1F684, "1f684.png");
png_name!(BULLETTRAIN_FRONT, "🚅", "bullet train", U_1F685, "1f685.png");
png_name!(TRAIN, "🚆", "train", U_1F686, "1f686.png");
png_name!(METRO, "🚇", "metro", U_1F687, "1f687.png");
png_name!(LIGHT_RAIL, "🚈", "light rail", U_1F688, "1f688.png");
png_name!(STATION, "🚉", "station", U_1F689, "1f689.png");
png_name!(TRAM, "🚊", "tram", U_1F68A, "1f68a.png");
png_name!(TRAM_CAR, "🚋", "tram car", U_1F68B, "1f68b.png");
png_name!(BUS, "🚌", "bus", U_1F68C, "1f68c.png");
png_name!(ONCOMING_BUS, "🚍", "oncoming bus", U_1F68D, "1f68d.png");
png_name!(TROLLEYBUS, "🚎", "trolleybus", U_1F68E, "1f68e.png");
png_name!(BUSSTOP, "🚏", "bus stop", U_1F68F, "1f68f.png");
png_name!(MINIBUS, "🚐", "minibus", U_1F690, "1f690.png");
png_name!(AMBULANCE, "🚑", "ambulance", U_1F691, "1f691.png");
png_name!(FIRE_ENGINE, "🚒", "fire engine", U_1F692, "1f692.png");
png_name!(POLICE_CAR, "🚓", "police car", U_1F693, "1f693.png");
png_name!(ONCOMING_POLICE_CAR, "🚔", "oncoming police car", U_1F694, "1f694.png");
png_name!(TAXI, "🚕", "taxi", U_1F695, "1f695.png");
png_name!(ONCOMING_TAXI, "🚖", "oncoming taxi", U_1F696, "1f696.png");
png_name!(CAR, "🚗", "automobile", U_1F697, "1f697.png");
png_name!(RED_CAR, "🚗", "automobile", U_1F697, "1f697.png");
png_name!(ONCOMING_AUTOMOBILE, "🚘", "oncoming automobile", U_1F698, "1f698.png");
png_name!(BLUE_CAR, "🚙", "sport utility vehicle", U_1F699, "1f699.png");
png_name!(SUV, "🚙", "sport utility vehicle", U_1F699, "1f699.png");
png_name!(DELIVERY_TRUCK, "🚚", "delivery truck", U_1F69A, "1f69a.png");
png_name!(TRUCK, "🚚", "delivery truck", U_1F69A, "1f69a.png");
png_name!(ARTICULATED_LORRY, "🚛", "articulated lorry", U_1F69B, "1f69b.png");
png_name!(TRACTOR, "🚜", "tractor", U_1F69C, "1f69c.png");
png_name!(MONORAIL, "🚝", "monorail", U_1F69D, "1f69d.png");
png_name!(MOUNTAIN_RAILWAY, "🚞", "mountain railway", U_1F69E, "1f69e.png");
png_name!(SUSPENSION_RAILWAY, "🚟", "suspension railway", U_1F69F, "1f69f.png");
png_name!(MOUNTAIN_CABLEWAY, "🚠", "mountain cableway", U_1F6A0, "1f6a0.png");
png_name!(AERIAL_TRAMWAY, "🚡", "aerial tramway", U_1F6A1, "1f6a1.png");
png_name!(SHIP, "🚢", "ship", U_1F6A2, "1f6a2.png");
png_name!(WOMAN_ROWING_BOAT_TONE1, "🚣🏻‍♀️", "", U_1F6A3_1F3FB_200D_2640_FE0F, "1f6a3-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_ROWING_BOAT_TONE1, "🚣🏻‍♂️", "", U_1F6A3_1F3FB_200D_2642_FE0F, "1f6a3-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_ROWING_BOAT_TONE1, "🚣🏻", "", U_1F6A3_1F3FB, "1f6a3-1f3fb.png");
png_name!(ROWBOAT_TONE1, "🚣🏻", "", U_1F6A3_1F3FB, "1f6a3-1f3fb.png");
png_name!(WOMAN_ROWING_BOAT_TONE2, "🚣🏼‍♀️", "", U_1F6A3_1F3FC_200D_2640_FE0F, "1f6a3-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_ROWING_BOAT_TONE2, "🚣🏼‍♂️", "", U_1F6A3_1F3FC_200D_2642_FE0F, "1f6a3-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_ROWING_BOAT_TONE2, "🚣🏼", "", U_1F6A3_1F3FC, "1f6a3-1f3fc.png");
png_name!(ROWBOAT_TONE2, "🚣🏼", "", U_1F6A3_1F3FC, "1f6a3-1f3fc.png");
png_name!(WOMAN_ROWING_BOAT_TONE3, "🚣🏽‍♀️", "", U_1F6A3_1F3FD_200D_2640_FE0F, "1f6a3-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_ROWING_BOAT_TONE3, "🚣🏽‍♂️", "", U_1F6A3_1F3FD_200D_2642_FE0F, "1f6a3-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_ROWING_BOAT_TONE3, "🚣🏽", "", U_1F6A3_1F3FD, "1f6a3-1f3fd.png");
png_name!(ROWBOAT_TONE3, "🚣🏽", "", U_1F6A3_1F3FD, "1f6a3-1f3fd.png");
png_name!(WOMAN_ROWING_BOAT_TONE4, "🚣🏾‍♀️", "", U_1F6A3_1F3FE_200D_2640_FE0F, "1f6a3-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_ROWING_BOAT_TONE4, "🚣🏾‍♂️", "", U_1F6A3_1F3FE_200D_2642_FE0F, "1f6a3-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_ROWING_BOAT_TONE4, "🚣🏾", "", U_1F6A3_1F3FE, "1f6a3-1f3fe.png");
png_name!(ROWBOAT_TONE4, "🚣🏾", "", U_1F6A3_1F3FE, "1f6a3-1f3fe.png");
png_name!(WOMAN_ROWING_BOAT_TONE5, "🚣🏿‍♀️", "", U_1F6A3_1F3FF_200D_2640_FE0F, "1f6a3-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_ROWING_BOAT_TONE5, "🚣🏿‍♂️", "", U_1F6A3_1F3FF_200D_2642_FE0F, "1f6a3-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_ROWING_BOAT_TONE5, "🚣🏿", "", U_1F6A3_1F3FF, "1f6a3-1f3ff.png");
png_name!(ROWBOAT_TONE5, "🚣🏿", "", U_1F6A3_1F3FF, "1f6a3-1f3ff.png");
png_name!(WOMAN_ROWING_BOAT, "🚣‍♀️", "woman rowing boat", U_1F6A3_200D_2640_FE0F, "1f6a3-200d-2640-fe0f.png");
png_name!(MAN_ROWING_BOAT, "🚣‍♂️", "man rowing boat", U_1F6A3_200D_2642_FE0F, "1f6a3-200d-2642-fe0f.png");
png_name!(PERSON_ROWING_BOAT, "🚣", "person rowing boat", U_1F6A3, "1f6a3.png");
png_name!(ROWBOAT, "🚣", "person rowing boat", U_1F6A3, "1f6a3.png");
png_name!(SPEEDBOAT, "🚤", "speedboat", U_1F6A4, "1f6a4.png");
png_name!(TRAFFIC_LIGHT, "🚥", "horizontal traffic light", U_1F6A5, "1f6a5.png");
png_name!(VERTICAL_TRAFFIC_LIGHT, "🚦", "vertical traffic light", U_1F6A6, "1f6a6.png");
png_name!(CONSTRUCTION, "🚧", "construction", U_1F6A7, "1f6a7.png");
png_name!(ROTATING_LIGHT, "🚨", "police car light", U_1F6A8, "1f6a8.png");
png_name!(TRIANGULAR_FLAG, "🚩", "triangular flag", U_1F6A9, "1f6a9.png");
png_name!(TRIANGULAR_FLAG_ON_POST, "🚩", "triangular flag", U_1F6A9, "1f6a9.png");
png_name!(DOOR, "🚪", "door", U_1F6AA, "1f6aa.png");
png_name!(NO_ENTRY_SIGN, "🚫", "prohibited", U_1F6AB, "1f6ab.png");
png_name!(CIGARETTE, "🚬", "cigarette", U_1F6AC, "1f6ac.png");
png_name!(SMOKING, "🚬", "cigarette", U_1F6AC, "1f6ac.png");
png_name!(NO_SMOKING, "🚭", "no smoking", U_1F6AD, "1f6ad.png");
png_name!(LITTER_BIN, "🚮", "litter in bin sign", U_1F6AE, "1f6ae.png");
png_name!(PUT_LITTER_IN_ITS_PLACE, "🚮", "litter in bin sign", U_1F6AE, "1f6ae.png");
png_name!(DO_NOT_LITTER, "🚯", "no littering", U_1F6AF, "1f6af.png");
png_name!(NO_LITTERING, "🚯", "no littering", U_1F6AF, "1f6af.png");
png_name!(POTABLE_WATER, "🚰", "potable water", U_1F6B0, "1f6b0.png");
png_name!(NON_POTABLE_WATER, "🚱", "non-potable water", U_1F6B1, "1f6b1.png");
png_name!(BICYCLE, "🚲", "bicycle", U_1F6B2, "1f6b2.png");
png_name!(BIKE, "🚲", "bicycle", U_1F6B2, "1f6b2.png");
png_name!(NO_BICYCLES, "🚳", "no bicycles", U_1F6B3, "1f6b3.png");
png_name!(WOMAN_BIKING_TONE1, "🚴🏻‍♀️", "", U_1F6B4_1F3FB_200D_2640_FE0F, "1f6b4-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_BIKING_TONE1, "🚴🏻‍♂️", "", U_1F6B4_1F3FB_200D_2642_FE0F, "1f6b4-1f3fb-200d-2642-fe0f.png");
png_name!(BICYCLIST_TONE1, "🚴🏻", "", U_1F6B4_1F3FB, "1f6b4-1f3fb.png");
png_name!(BIKING_TONE1, "🚴🏻", "", U_1F6B4_1F3FB, "1f6b4-1f3fb.png");
png_name!(PERSON_BIKING_TONE1, "🚴🏻", "", U_1F6B4_1F3FB, "1f6b4-1f3fb.png");
png_name!(WOMAN_BIKING_TONE2, "🚴🏼‍♀️", "", U_1F6B4_1F3FC_200D_2640_FE0F, "1f6b4-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_BIKING_TONE2, "🚴🏼‍♂️", "", U_1F6B4_1F3FC_200D_2642_FE0F, "1f6b4-1f3fc-200d-2642-fe0f.png");
png_name!(BICYCLIST_TONE2, "🚴🏼", "", U_1F6B4_1F3FC, "1f6b4-1f3fc.png");
png_name!(BIKING_TONE2, "🚴🏼", "", U_1F6B4_1F3FC, "1f6b4-1f3fc.png");
png_name!(PERSON_BIKING_TONE2, "🚴🏼", "", U_1F6B4_1F3FC, "1f6b4-1f3fc.png");
png_name!(WOMAN_BIKING_TONE3, "🚴🏽‍♀️", "", U_1F6B4_1F3FD_200D_2640_FE0F, "1f6b4-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_BIKING_TONE3, "🚴🏽‍♂️", "", U_1F6B4_1F3FD_200D_2642_FE0F, "1f6b4-1f3fd-200d-2642-fe0f.png");
png_name!(BICYCLIST_TONE3, "🚴🏽", "", U_1F6B4_1F3FD, "1f6b4-1f3fd.png");
png_name!(BIKING_TONE3, "🚴🏽", "", U_1F6B4_1F3FD, "1f6b4-1f3fd.png");
png_name!(PERSON_BIKING_TONE3, "🚴🏽", "", U_1F6B4_1F3FD, "1f6b4-1f3fd.png");
png_name!(WOMAN_BIKING_TONE4, "🚴🏾‍♀️", "", U_1F6B4_1F3FE_200D_2640_FE0F, "1f6b4-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_BIKING_TONE4, "🚴🏾‍♂️", "", U_1F6B4_1F3FE_200D_2642_FE0F, "1f6b4-1f3fe-200d-2642-fe0f.png");
png_name!(BICYCLIST_TONE4, "🚴🏾", "", U_1F6B4_1F3FE, "1f6b4-1f3fe.png");
png_name!(BIKING_TONE4, "🚴🏾", "", U_1F6B4_1F3FE, "1f6b4-1f3fe.png");
png_name!(PERSON_BIKING_TONE4, "🚴🏾", "", U_1F6B4_1F3FE, "1f6b4-1f3fe.png");
png_name!(WOMAN_BIKING_TONE5, "🚴🏿‍♀️", "", U_1F6B4_1F3FF_200D_2640_FE0F, "1f6b4-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_BIKING_TONE5, "🚴🏿‍♂️", "", U_1F6B4_1F3FF_200D_2642_FE0F, "1f6b4-1f3ff-200d-2642-fe0f.png");
png_name!(BICYCLIST_TONE5, "🚴🏿", "", U_1F6B4_1F3FF, "1f6b4-1f3ff.png");
png_name!(BIKING_TONE5, "🚴🏿", "", U_1F6B4_1F3FF, "1f6b4-1f3ff.png");
png_name!(PERSON_BIKING_TONE5, "🚴🏿", "", U_1F6B4_1F3FF, "1f6b4-1f3ff.png");
png_name!(WOMAN_BIKING, "🚴‍♀️", "woman biking", U_1F6B4_200D_2640_FE0F, "1f6b4-200d-2640-fe0f.png");
png_name!(MAN_BIKING, "🚴‍♂️", "man biking", U_1F6B4_200D_2642_FE0F, "1f6b4-200d-2642-fe0f.png");
png_name!(BICYCLIST, "🚴", "person biking", U_1F6B4, "1f6b4.png");
png_name!(BIKING, "🚴", "person biking", U_1F6B4, "1f6b4.png");
png_name!(PERSON_BIKING, "🚴", "person biking", U_1F6B4, "1f6b4.png");
png_name!(WOMAN_MOUNTAIN_BIKING_TONE1, "🚵🏻‍♀️", "", U_1F6B5_1F3FB_200D_2640_FE0F, "1f6b5-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_MOUNTAIN_BIKING_TONE1, "🚵🏻‍♂️", "", U_1F6B5_1F3FB_200D_2642_FE0F, "1f6b5-1f3fb-200d-2642-fe0f.png");
png_name!(MOUNTAIN_BICYCLIST_TONE1, "🚵🏻", "", U_1F6B5_1F3FB, "1f6b5-1f3fb.png");
png_name!(MOUNTAIN_BIKING_TONE1, "🚵🏻", "", U_1F6B5_1F3FB, "1f6b5-1f3fb.png");
png_name!(PERSON_MOUNTAIN_BIKING_TONE1, "🚵🏻", "", U_1F6B5_1F3FB, "1f6b5-1f3fb.png");
png_name!(WOMAN_MOUNTAIN_BIKING_TONE2, "🚵🏼‍♀️", "", U_1F6B5_1F3FC_200D_2640_FE0F, "1f6b5-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_MOUNTAIN_BIKING_TONE2, "🚵🏼‍♂️", "", U_1F6B5_1F3FC_200D_2642_FE0F, "1f6b5-1f3fc-200d-2642-fe0f.png");
png_name!(MOUNTAIN_BICYCLIST_TONE2, "🚵🏼", "", U_1F6B5_1F3FC, "1f6b5-1f3fc.png");
png_name!(MOUNTAIN_BIKING_TONE2, "🚵🏼", "", U_1F6B5_1F3FC, "1f6b5-1f3fc.png");
png_name!(PERSON_MOUNTAIN_BIKING_TONE2, "🚵🏼", "", U_1F6B5_1F3FC, "1f6b5-1f3fc.png");
png_name!(WOMAN_MOUNTAIN_BIKING_TONE3, "🚵🏽‍♀️", "", U_1F6B5_1F3FD_200D_2640_FE0F, "1f6b5-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_MOUNTAIN_BIKING_TONE3, "🚵🏽‍♂️", "", U_1F6B5_1F3FD_200D_2642_FE0F, "1f6b5-1f3fd-200d-2642-fe0f.png");
png_name!(MOUNTAIN_BICYCLIST_TONE3, "🚵🏽", "", U_1F6B5_1F3FD, "1f6b5-1f3fd.png");
png_name!(MOUNTAIN_BIKING_TONE3, "🚵🏽", "", U_1F6B5_1F3FD, "1f6b5-1f3fd.png");
png_name!(PERSON_MOUNTAIN_BIKING_TONE3, "🚵🏽", "", U_1F6B5_1F3FD, "1f6b5-1f3fd.png");
png_name!(WOMAN_MOUNTAIN_BIKING_TONE4, "🚵🏾‍♀️", "", U_1F6B5_1F3FE_200D_2640_FE0F, "1f6b5-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_MOUNTAIN_BIKING_TONE4, "🚵🏾‍♂️", "", U_1F6B5_1F3FE_200D_2642_FE0F, "1f6b5-1f3fe-200d-2642-fe0f.png");
png_name!(MOUNTAIN_BICYCLIST_TONE4, "🚵🏾", "", U_1F6B5_1F3FE, "1f6b5-1f3fe.png");
png_name!(MOUNTAIN_BIKING_TONE4, "🚵🏾", "", U_1F6B5_1F3FE, "1f6b5-1f3fe.png");
png_name!(PERSON_MOUNTAIN_BIKING_TONE4, "🚵🏾", "", U_1F6B5_1F3FE, "1f6b5-1f3fe.png");
png_name!(WOMAN_MOUNTAIN_BIKING_TONE5, "🚵🏿‍♀️", "", U_1F6B5_1F3FF_200D_2640_FE0F, "1f6b5-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_MOUNTAIN_BIKING_TONE5, "🚵🏿‍♂️", "", U_1F6B5_1F3FF_200D_2642_FE0F, "1f6b5-1f3ff-200d-2642-fe0f.png");
png_name!(MOUNTAIN_BICYCLIST_TONE5, "🚵🏿", "", U_1F6B5_1F3FF, "1f6b5-1f3ff.png");
png_name!(MOUNTAIN_BIKING_TONE5, "🚵🏿", "", U_1F6B5_1F3FF, "1f6b5-1f3ff.png");
png_name!(PERSON_MOUNTAIN_BIKING_TONE5, "🚵🏿", "", U_1F6B5_1F3FF, "1f6b5-1f3ff.png");
png_name!(WOMAN_MOUNTAIN_BIKING, "🚵‍♀️", "woman mountain biking", U_1F6B5_200D_2640_FE0F, "1f6b5-200d-2640-fe0f.png");
png_name!(MAN_MOUNTAIN_BIKING, "🚵‍♂️", "man mountain biking", U_1F6B5_200D_2642_FE0F, "1f6b5-200d-2642-fe0f.png");
png_name!(MOUNTAIN_BICYCLIST, "🚵", "person mountain biking", U_1F6B5, "1f6b5.png");
png_name!(MOUNTAIN_BIKING, "🚵", "person mountain biking", U_1F6B5, "1f6b5.png");
png_name!(PERSON_MOUNTAIN_BIKING, "🚵", "person mountain biking", U_1F6B5, "1f6b5.png");
png_name!(WOMAN_WALKING_TONE1, "🚶🏻‍♀️", "", U_1F6B6_1F3FB_200D_2640_FE0F, "1f6b6-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_WALKING_TONE1, "🚶🏻‍♂️", "", U_1F6B6_1F3FB_200D_2642_FE0F, "1f6b6-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_WALKING_TONE1, "🚶🏻", "", U_1F6B6_1F3FB, "1f6b6-1f3fb.png");
png_name!(WALKING_TONE1, "🚶🏻", "", U_1F6B6_1F3FB, "1f6b6-1f3fb.png");
png_name!(WOMAN_WALKING_TONE2, "🚶🏼‍♀️", "", U_1F6B6_1F3FC_200D_2640_FE0F, "1f6b6-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_WALKING_TONE2, "🚶🏼‍♂️", "", U_1F6B6_1F3FC_200D_2642_FE0F, "1f6b6-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_WALKING_TONE2, "🚶🏼", "", U_1F6B6_1F3FC, "1f6b6-1f3fc.png");
png_name!(WALKING_TONE2, "🚶🏼", "", U_1F6B6_1F3FC, "1f6b6-1f3fc.png");
png_name!(WOMAN_WALKING_TONE3, "🚶🏽‍♀️", "", U_1F6B6_1F3FD_200D_2640_FE0F, "1f6b6-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_WALKING_TONE3, "🚶🏽‍♂️", "", U_1F6B6_1F3FD_200D_2642_FE0F, "1f6b6-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_WALKING_TONE3, "🚶🏽", "", U_1F6B6_1F3FD, "1f6b6-1f3fd.png");
png_name!(WALKING_TONE3, "🚶🏽", "", U_1F6B6_1F3FD, "1f6b6-1f3fd.png");
png_name!(WOMAN_WALKING_TONE4, "🚶🏾‍♀️", "", U_1F6B6_1F3FE_200D_2640_FE0F, "1f6b6-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_WALKING_TONE4, "🚶🏾‍♂️", "", U_1F6B6_1F3FE_200D_2642_FE0F, "1f6b6-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_WALKING_TONE4, "🚶🏾", "", U_1F6B6_1F3FE, "1f6b6-1f3fe.png");
png_name!(WALKING_TONE4, "🚶🏾", "", U_1F6B6_1F3FE, "1f6b6-1f3fe.png");
png_name!(WOMAN_WALKING_TONE5, "🚶🏿‍♀️", "", U_1F6B6_1F3FF_200D_2640_FE0F, "1f6b6-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_WALKING_TONE5, "🚶🏿‍♂️", "", U_1F6B6_1F3FF_200D_2642_FE0F, "1f6b6-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_WALKING_TONE5, "🚶🏿", "", U_1F6B6_1F3FF, "1f6b6-1f3ff.png");
png_name!(WALKING_TONE5, "🚶🏿", "", U_1F6B6_1F3FF, "1f6b6-1f3ff.png");
png_name!(WOMAN_WALKING, "🚶‍♀️", "woman walking", U_1F6B6_200D_2640_FE0F, "1f6b6-200d-2640-fe0f.png");
png_name!(MAN_WALKING, "🚶‍♂️", "man walking", U_1F6B6_200D_2642_FE0F, "1f6b6-200d-2642-fe0f.png");
png_name!(PERSON_WALKING, "🚶", "person walking", U_1F6B6, "1f6b6.png");
png_name!(WALKING, "🚶", "person walking", U_1F6B6, "1f6b6.png");
png_name!(NO_PEDESTRIANS, "🚷", "no pedestrians", U_1F6B7, "1f6b7.png");
png_name!(CHILDREN_CROSSING, "🚸", "children crossing", U_1F6B8, "1f6b8.png");
png_name!(MENS, "🚹", "men’s room", U_1F6B9, "1f6b9.png");
png_name!(WOMENS, "🚺", "women’s room", U_1F6BA, "1f6ba.png");
png_name!(BATHROOM, "🚻", "restroom", U_1F6BB, "1f6bb.png");
png_name!(RESTROOM, "🚻", "restroom", U_1F6BB, "1f6bb.png");
png_name!(BABY_SYMBOL, "🚼", "baby symbol", U_1F6BC, "1f6bc.png");
png_name!(TOILET, "🚽", "toilet", U_1F6BD, "1f6bd.png");
png_name!(WATER_CLOSET, "🚾", "water closet", U_1F6BE, "1f6be.png");
png_name!(WC, "🚾", "water closet", U_1F6BE, "1f6be.png");
png_name!(SHOWER, "🚿", "shower", U_1F6BF, "1f6bf.png");
png_name!(BATH_TONE1, "🛀🏻", "", U_1F6C0_1F3FB, "1f6c0-1f3fb.png");
png_name!(PERSON_TAKING_BATH_TONE1, "🛀🏻", "", U_1F6C0_1F3FB, "1f6c0-1f3fb.png");
png_name!(BATH_TONE2, "🛀🏼", "", U_1F6C0_1F3FC, "1f6c0-1f3fc.png");
png_name!(PERSON_TAKING_BATH_TONE2, "🛀🏼", "", U_1F6C0_1F3FC, "1f6c0-1f3fc.png");
png_name!(BATH_TONE3, "🛀🏽", "", U_1F6C0_1F3FD, "1f6c0-1f3fd.png");
png_name!(PERSON_TAKING_BATH_TONE3, "🛀🏽", "", U_1F6C0_1F3FD, "1f6c0-1f3fd.png");
png_name!(BATH_TONE4, "🛀🏾", "", U_1F6C0_1F3FE, "1f6c0-1f3fe.png");
png_name!(PERSON_TAKING_BATH_TONE4, "🛀🏾", "", U_1F6C0_1F3FE, "1f6c0-1f3fe.png");
png_name!(BATH_TONE5, "🛀🏿", "", U_1F6C0_1F3FF, "1f6c0-1f3ff.png");
png_name!(PERSON_TAKING_BATH_TONE5, "🛀🏿", "", U_1F6C0_1F3FF, "1f6c0-1f3ff.png");
png_name!(BATH, "🛀", "person taking bath", U_1F6C0, "1f6c0.png");
png_name!(PERSON_TAKING_BATH, "🛀", "person taking bath", U_1F6C0, "1f6c0.png");
png_name!(BATHTUB, "🛁", "bathtub", U_1F6C1, "1f6c1.png");
png_name!(PASSPORT_CONTROL, "🛂", "passport control", U_1F6C2, "1f6c2.png");
png_name!(CUSTOMS, "🛃", "customs", U_1F6C3, "1f6c3.png");
png_name!(BAGGAGE_CLAIM, "🛄", "baggage claim", U_1F6C4, "1f6c4.png");
png_name!(LEFT_LUGGAGE, "🛅", "left luggage", U_1F6C5, "1f6c5.png");
png_name!(COUCH_AND_LAMP, "🛋", "couch and lamp", U_1F6CB, "1f6cb.png");
png_name!(PERSON_IN_BED_TONE1, "🛌🏻", "", U_1F6CC_1F3FB, "1f6cc-1f3fb.png");
png_name!(SLEEPING_ACCOMMODATION_TONE1, "🛌🏻", "", U_1F6CC_1F3FB, "1f6cc-1f3fb.png");
png_name!(PERSON_IN_BED_TONE2, "🛌🏼", "", U_1F6CC_1F3FC, "1f6cc-1f3fc.png");
png_name!(SLEEPING_ACCOMMODATION_TONE2, "🛌🏼", "", U_1F6CC_1F3FC, "1f6cc-1f3fc.png");
png_name!(PERSON_IN_BED_TONE3, "🛌🏽", "", U_1F6CC_1F3FD, "1f6cc-1f3fd.png");
png_name!(SLEEPING_ACCOMMODATION_TONE3, "🛌🏽", "", U_1F6CC_1F3FD, "1f6cc-1f3fd.png");
png_name!(PERSON_IN_BED_TONE4, "🛌🏾", "", U_1F6CC_1F3FE, "1f6cc-1f3fe.png");
png_name!(SLEEPING_ACCOMMODATION_TONE4, "🛌🏾", "", U_1F6CC_1F3FE, "1f6cc-1f3fe.png");
png_name!(PERSON_IN_BED_TONE5, "🛌🏿", "", U_1F6CC_1F3FF, "1f6cc-1f3ff.png");
png_name!(SLEEPING_ACCOMMODATION_TONE5, "🛌🏿", "", U_1F6CC_1F3FF, "1f6cc-1f3ff.png");
png_name!(PERSON_IN_BED, "🛌", "person in bed", U_1F6CC, "1f6cc.png");
png_name!(SLEEPING_ACCOMMODATION, "🛌", "person in bed", U_1F6CC, "1f6cc.png");
png_name!(SHOPPING_BAGS, "🛍", "shopping bags", U_1F6CD, "1f6cd.png");
png_name!(BELLHOP, "🛎", "bellhop bell", U_1F6CE, "1f6ce.png");
png_name!(BED, "🛏", "bed", U_1F6CF, "1f6cf.png");
png_name!(PLACE_OF_WORSHIP, "🛐", "place of worship", U_1F6D0, "1f6d0.png");
png_name!(OCTAGONAL_SIGN, "🛑", "stop sign", U_1F6D1, "1f6d1.png");
png_name!(STOP_SIGN, "🛑", "stop sign", U_1F6D1, "1f6d1.png");
png_name!(SHOPPING_CART, "🛒", "shopping cart", U_1F6D2, "1f6d2.png");
png_name!(HINDU_TEMPLE, "🛕", "hindu temple", U_1F6D5, "1f6d5.png");
png_name!(HUT, "🛖", "hut", U_1F6D6, "1f6d6.png");
png_name!(ELEVATOR, "🛗", "elevator", U_1F6D7, "1f6d7.png");
png_name!(PLAYGROUND_SLIDE, "🛝", "playground slide", U_1F6DD, "1f6dd.png");
png_name!(SLIDE, "🛝", "playground slide", U_1F6DD, "1f6dd.png");
png_name!(WHEEL, "🛞", "wheel", U_1F6DE, "1f6de.png");
png_name!(LIFEBUOY, "🛟", "ring buoy", U_1F6DF, "1f6df.png");
png_name!(RING_BUOY, "🛟", "ring buoy", U_1F6DF, "1f6df.png");
png_name!(HAMMER_AND_WRENCH, "🛠", "hammer and wrench", U_1F6E0, "1f6e0.png");
png_name!(SHIELD, "🛡", "shield", U_1F6E1, "1f6e1.png");
png_name!(OIL_DRUM, "🛢", "oil drum", U_1F6E2, "1f6e2.png");
png_name!(MOTORWAY, "🛣", "motorway", U_1F6E3, "1f6e3.png");
png_name!(RAILWAY_TRACK, "🛤", "railway track", U_1F6E4, "1f6e4.png");
png_name!(MOTORBOAT, "🛥", "motor boat", U_1F6E5, "1f6e5.png");
png_name!(SMALL_AIRPLANE, "🛩", "small airplane", U_1F6E9, "1f6e9.png");
png_name!(AIRPLANE_DEPARTURE, "🛫", "airplane departure", U_1F6EB, "1f6eb.png");
png_name!(AIRPLANE_ARRIVING, "🛬", "airplane arrival", U_1F6EC, "1f6ec.png");
png_name!(SATELLITE, "🛰", "satellite", U_1F6F0, "1f6f0.png");
png_name!(CRUISE_SHIP, "🛳", "passenger ship", U_1F6F3, "1f6f3.png");
png_name!(PASSENGER_SHIP, "🛳", "passenger ship", U_1F6F3, "1f6f3.png");
png_name!(SCOOTER, "🛴", "kick scooter", U_1F6F4, "1f6f4.png");
png_name!(MOTOR_SCOOTER, "🛵", "motor scooter", U_1F6F5, "1f6f5.png");
png_name!(CANOE, "🛶", "canoe", U_1F6F6, "1f6f6.png");
png_name!(SLED, "🛷", "sled", U_1F6F7, "1f6f7.png");
png_name!(FLYING_SAUCER, "🛸", "flying saucer", U_1F6F8, "1f6f8.png");
png_name!(SKATEBOARD, "🛹", "skateboard", U_1F6F9, "1f6f9.png");
png_name!(AUTO_RICKSHAW, "🛺", "auto rickshaw", U_1F6FA, "1f6fa.png");
png_name!(PICKUP_TRUCK, "🛻", "pickup truck", U_1F6FB, "1f6fb.png");
png_name!(ROLLER_SKATE, "🛼", "roller skate", U_1F6FC, "1f6fc.png");
png_name!(ORANGE_CIRCLE, "🟠", "orange circle", U_1F7E0, "1f7e0.png");
png_name!(YELLOW_CIRCLE, "🟡", "yellow circle", U_1F7E1, "1f7e1.png");
png_name!(GREEN_CIRCLE, "🟢", "green circle", U_1F7E2, "1f7e2.png");
png_name!(PURPLE_CIRCLE, "🟣", "purple circle", U_1F7E3, "1f7e3.png");
png_name!(BROWN_CIRCLE, "🟤", "brown circle", U_1F7E4, "1f7e4.png");
png_name!(RED_SQUARE, "🟥", "red square", U_1F7E5, "1f7e5.png");
png_name!(BLUE_SQUARE, "🟦", "blue square", U_1F7E6, "1f7e6.png");
png_name!(ORANGE_SQUARE, "🟧", "orange square", U_1F7E7, "1f7e7.png");
png_name!(YELLOW_SQUARE, "🟨", "yellow square", U_1F7E8, "1f7e8.png");
png_name!(GREEN_SQUARE, "🟩", "green square", U_1F7E9, "1f7e9.png");
png_name!(PURPLE_SQUARE, "🟪", "purple square", U_1F7EA, "1f7ea.png");
png_name!(BROWN_SQUARE, "🟫", "brown square", U_1F7EB, "1f7eb.png");
png_name!(HEAVY_EQUALS_SIGN, "🟰", "heavy equals sign", U_1F7F0, "1f7f0.png");
png_name!(PINCH_TONE1, "🤌🏻", "", U_1F90C_1F3FB, "1f90c-1f3fb.png");
png_name!(PINCHED_FINGERS_TONE1, "🤌🏻", "", U_1F90C_1F3FB, "1f90c-1f3fb.png");
png_name!(PINCH_TONE2, "🤌🏼", "", U_1F90C_1F3FC, "1f90c-1f3fc.png");
png_name!(PINCHED_FINGERS_TONE2, "🤌🏼", "", U_1F90C_1F3FC, "1f90c-1f3fc.png");
png_name!(PINCH_TONE3, "🤌🏽", "", U_1F90C_1F3FD, "1f90c-1f3fd.png");
png_name!(PINCHED_FINGERS_TONE3, "🤌🏽", "", U_1F90C_1F3FD, "1f90c-1f3fd.png");
png_name!(PINCH_TONE4, "🤌🏾", "", U_1F90C_1F3FE, "1f90c-1f3fe.png");
png_name!(PINCHED_FINGERS_TONE4, "🤌🏾", "", U_1F90C_1F3FE, "1f90c-1f3fe.png");
png_name!(PINCH_TONE5, "🤌🏿", "", U_1F90C_1F3FF, "1f90c-1f3ff.png");
png_name!(PINCHED_FINGERS_TONE5, "🤌🏿", "", U_1F90C_1F3FF, "1f90c-1f3ff.png");
png_name!(PINCH, "🤌", "pinched fingers", U_1F90C, "1f90c.png");
png_name!(PINCHED_FINGERS, "🤌", "pinched fingers", U_1F90C, "1f90c.png");
png_name!(WHITE_HEART, "🤍", "white heart", U_1F90D, "1f90d.png");
png_name!(BROWN_HEART, "🤎", "brown heart", U_1F90E, "1f90e.png");
png_name!(PINCHING_HAND_TONE1, "🤏🏻", "", U_1F90F_1F3FB, "1f90f-1f3fb.png");
png_name!(PINCHING_HAND_TONE2, "🤏🏼", "", U_1F90F_1F3FC, "1f90f-1f3fc.png");
png_name!(PINCHING_HAND_TONE3, "🤏🏽", "", U_1F90F_1F3FD, "1f90f-1f3fd.png");
png_name!(PINCHING_HAND_TONE4, "🤏🏾", "", U_1F90F_1F3FE, "1f90f-1f3fe.png");
png_name!(PINCHING_HAND_TONE5, "🤏🏿", "", U_1F90F_1F3FF, "1f90f-1f3ff.png");
png_name!(PINCHING_HAND, "🤏", "pinching hand", U_1F90F, "1f90f.png");
png_name!(ZIPPER_MOUTH, "🤐", "zipper-mouth face", U_1F910, "1f910.png");
png_name!(ZIPPER_MOUTH_FACE, "🤐", "zipper-mouth face", U_1F910, "1f910.png");
png_name!(MONEY_MOUTH_FACE, "🤑", "money-mouth face", U_1F911, "1f911.png");
png_name!(FACE_WITH_THERMOMETER, "🤒", "face with thermometer", U_1F912, "1f912.png");
png_name!(NERD, "🤓", "nerd face", U_1F913, "1f913.png");
png_name!(NERD_FACE, "🤓", "nerd face", U_1F913, "1f913.png");
png_name!(THINKING, "🤔", "thinking face", U_1F914, "1f914.png");
png_name!(THINKING_FACE, "🤔", "thinking face", U_1F914, "1f914.png");
png_name!(WTF, "🤔", "thinking face", U_1F914, "1f914.png");
png_name!(FACE_WITH_HEAD_BANDAGE, "🤕", "face with head-bandage", U_1F915, "1f915.png");
png_name!(ROBOT, "🤖", "robot", U_1F916, "1f916.png");
png_name!(ROBOT_FACE, "🤖", "robot", U_1F916, "1f916.png");
png_name!(HUG, "🤗", "smiling face with open hands", U_1F917, "1f917.png");
png_name!(HUGGING, "🤗", "smiling face with open hands", U_1F917, "1f917.png");
png_name!(HUGGING_FACE, "🤗", "smiling face with open hands", U_1F917, "1f917.png");
png_name!(METAL_TONE1, "🤘🏻", "", U_1F918_1F3FB, "1f918-1f3fb.png");
png_name!(SIGN_OF_THE_HORNS_TONE1, "🤘🏻", "", U_1F918_1F3FB, "1f918-1f3fb.png");
png_name!(METAL_TONE2, "🤘🏼", "", U_1F918_1F3FC, "1f918-1f3fc.png");
png_name!(SIGN_OF_THE_HORNS_TONE2, "🤘🏼", "", U_1F918_1F3FC, "1f918-1f3fc.png");
png_name!(METAL_TONE3, "🤘🏽", "", U_1F918_1F3FD, "1f918-1f3fd.png");
png_name!(SIGN_OF_THE_HORNS_TONE3, "🤘🏽", "", U_1F918_1F3FD, "1f918-1f3fd.png");
png_name!(METAL_TONE4, "🤘🏾", "", U_1F918_1F3FE, "1f918-1f3fe.png");
png_name!(SIGN_OF_THE_HORNS_TONE4, "🤘🏾", "", U_1F918_1F3FE, "1f918-1f3fe.png");
png_name!(METAL_TONE5, "🤘🏿", "", U_1F918_1F3FF, "1f918-1f3ff.png");
png_name!(SIGN_OF_THE_HORNS_TONE5, "🤘🏿", "", U_1F918_1F3FF, "1f918-1f3ff.png");
png_name!(METAL, "🤘", "sign of the horns", U_1F918, "1f918.png");
png_name!(SIGN_OF_THE_HORNS, "🤘", "sign of the horns", U_1F918, "1f918.png");
png_name!(CALL_ME_HAND_TONE1, "🤙🏻", "", U_1F919_1F3FB, "1f919-1f3fb.png");
png_name!(CALL_ME_HAND_TONE2, "🤙🏼", "", U_1F919_1F3FC, "1f919-1f3fc.png");
png_name!(CALL_ME_HAND_TONE3, "🤙🏽", "", U_1F919_1F3FD, "1f919-1f3fd.png");
png_name!(CALL_ME_HAND_TONE4, "🤙🏾", "", U_1F919_1F3FE, "1f919-1f3fe.png");
png_name!(CALL_ME_HAND_TONE5, "🤙🏿", "", U_1F919_1F3FF, "1f919-1f3ff.png");
png_name!(CALL_ME_HAND, "🤙", "call me hand", U_1F919, "1f919.png");
png_name!(RAISED_BACK_OF_HAND_TONE1, "🤚🏻", "", U_1F91A_1F3FB, "1f91a-1f3fb.png");
png_name!(RAISED_BACK_OF_HAND_TONE2, "🤚🏼", "", U_1F91A_1F3FC, "1f91a-1f3fc.png");
png_name!(RAISED_BACK_OF_HAND_TONE3, "🤚🏽", "", U_1F91A_1F3FD, "1f91a-1f3fd.png");
png_name!(RAISED_BACK_OF_HAND_TONE4, "🤚🏾", "", U_1F91A_1F3FE, "1f91a-1f3fe.png");
png_name!(RAISED_BACK_OF_HAND_TONE5, "🤚🏿", "", U_1F91A_1F3FF, "1f91a-1f3ff.png");
png_name!(RAISED_BACK_OF_HAND, "🤚", "raised back of hand", U_1F91A, "1f91a.png");
png_name!(LEFT_FACING_FIST_TONE1, "🤛🏻", "", U_1F91B_1F3FB, "1f91b-1f3fb.png");
png_name!(LEFT_FACING_FIST_TONE2, "🤛🏼", "", U_1F91B_1F3FC, "1f91b-1f3fc.png");
png_name!(LEFT_FACING_FIST_TONE3, "🤛🏽", "", U_1F91B_1F3FD, "1f91b-1f3fd.png");
png_name!(LEFT_FACING_FIST_TONE4, "🤛🏾", "", U_1F91B_1F3FE, "1f91b-1f3fe.png");
png_name!(LEFT_FACING_FIST_TONE5, "🤛🏿", "", U_1F91B_1F3FF, "1f91b-1f3ff.png");
png_name!(LEFT_FACING_FIST, "🤛", "left-facing fist", U_1F91B, "1f91b.png");
png_name!(RIGHT_FACING_FIST_TONE1, "🤜🏻", "", U_1F91C_1F3FB, "1f91c-1f3fb.png");
png_name!(RIGHT_FACING_FIST_TONE2, "🤜🏼", "", U_1F91C_1F3FC, "1f91c-1f3fc.png");
png_name!(RIGHT_FACING_FIST_TONE3, "🤜🏽", "", U_1F91C_1F3FD, "1f91c-1f3fd.png");
png_name!(RIGHT_FACING_FIST_TONE4, "🤜🏾", "", U_1F91C_1F3FE, "1f91c-1f3fe.png");
png_name!(RIGHT_FACING_FIST_TONE5, "🤜🏿", "", U_1F91C_1F3FF, "1f91c-1f3ff.png");
png_name!(RIGHT_FACING_FIST, "🤜", "right-facing fist", U_1F91C, "1f91c.png");
png_name!(HANDSHAKE_TONE1, "🤝🏻", "", U_1F91D_1F3FB, "1f91d-1f3fb.png");
png_name!(HANDSHAKE_TONE2, "🤝🏼", "", U_1F91D_1F3FC, "1f91d-1f3fc.png");
png_name!(HANDSHAKE_TONE3, "🤝🏽", "", U_1F91D_1F3FD, "1f91d-1f3fd.png");
png_name!(HANDSHAKE_TONE4, "🤝🏾", "", U_1F91D_1F3FE, "1f91d-1f3fe.png");
png_name!(HANDSHAKE_TONE5, "🤝🏿", "", U_1F91D_1F3FF, "1f91d-1f3ff.png");
png_name!(HANDSHAKE, "🤝", "handshake", U_1F91D, "1f91d.png");
png_name!(FINGERS_CROSSED_TONE1, "🤞🏻", "", U_1F91E_1F3FB, "1f91e-1f3fb.png");
png_name!(FINGERS_CROSSED_TONE2, "🤞🏼", "", U_1F91E_1F3FC, "1f91e-1f3fc.png");
png_name!(FINGERS_CROSSED_TONE3, "🤞🏽", "", U_1F91E_1F3FD, "1f91e-1f3fd.png");
png_name!(FINGERS_CROSSED_TONE4, "🤞🏾", "", U_1F91E_1F3FE, "1f91e-1f3fe.png");
png_name!(FINGERS_CROSSED_TONE5, "🤞🏿", "", U_1F91E_1F3FF, "1f91e-1f3ff.png");
png_name!(FINGERS_CROSSED, "🤞", "crossed fingers", U_1F91E, "1f91e.png");
png_name!(LOVE_YOU_GESTURE_TONE1, "🤟🏻", "", U_1F91F_1F3FB, "1f91f-1f3fb.png");
png_name!(LOVE_YOU_GESTURE_TONE2, "🤟🏼", "", U_1F91F_1F3FC, "1f91f-1f3fc.png");
png_name!(LOVE_YOU_GESTURE_TONE3, "🤟🏽", "", U_1F91F_1F3FD, "1f91f-1f3fd.png");
png_name!(LOVE_YOU_GESTURE_TONE4, "🤟🏾", "", U_1F91F_1F3FE, "1f91f-1f3fe.png");
png_name!(LOVE_YOU_GESTURE_TONE5, "🤟🏿", "", U_1F91F_1F3FF, "1f91f-1f3ff.png");
png_name!(LOVE_YOU_GESTURE, "🤟", "love-you gesture", U_1F91F, "1f91f.png");
png_name!(COWBOY, "🤠", "cowboy hat face", U_1F920, "1f920.png");
png_name!(COWBOY_FACE, "🤠", "cowboy hat face", U_1F920, "1f920.png");
png_name!(CLOWN, "🤡", "clown face", U_1F921, "1f921.png");
png_name!(CLOWN_FACE, "🤡", "clown face", U_1F921, "1f921.png");
png_name!(NAUSEATED, "🤢", "nauseated face", U_1F922, "1f922.png");
png_name!(NAUSEATED_FACE, "🤢", "nauseated face", U_1F922, "1f922.png");
png_name!(ROFL, "🤣", "rolling on the floor laughing", U_1F923, "1f923.png");
png_name!(DROOLING, "🤤", "drooling face", U_1F924, "1f924.png");
png_name!(DROOLING_FACE, "🤤", "drooling face", U_1F924, "1f924.png");
png_name!(LYING, "🤥", "lying face", U_1F925, "1f925.png");
png_name!(LYING_FACE, "🤥", "lying face", U_1F925, "1f925.png");
png_name!(WOMAN_FACEPALMING_TONE1, "🤦🏻‍♀️", "", U_1F926_1F3FB_200D_2640_FE0F, "1f926-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_FACEPALMING_TONE1, "🤦🏻‍♂️", "", U_1F926_1F3FB_200D_2642_FE0F, "1f926-1f3fb-200d-2642-fe0f.png");
png_name!(FACEPALM_TONE1, "🤦🏻", "", U_1F926_1F3FB, "1f926-1f3fb.png");
png_name!(PERSON_FACEPALMING_TONE1, "🤦🏻", "", U_1F926_1F3FB, "1f926-1f3fb.png");
png_name!(WOMAN_FACEPALMING_TONE2, "🤦🏼‍♀️", "", U_1F926_1F3FC_200D_2640_FE0F, "1f926-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_FACEPALMING_TONE2, "🤦🏼‍♂️", "", U_1F926_1F3FC_200D_2642_FE0F, "1f926-1f3fc-200d-2642-fe0f.png");
png_name!(FACEPALM_TONE2, "🤦🏼", "", U_1F926_1F3FC, "1f926-1f3fc.png");
png_name!(PERSON_FACEPALMING_TONE2, "🤦🏼", "", U_1F926_1F3FC, "1f926-1f3fc.png");
png_name!(WOMAN_FACEPALMING_TONE3, "🤦🏽‍♀️", "", U_1F926_1F3FD_200D_2640_FE0F, "1f926-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_FACEPALMING_TONE3, "🤦🏽‍♂️", "", U_1F926_1F3FD_200D_2642_FE0F, "1f926-1f3fd-200d-2642-fe0f.png");
png_name!(FACEPALM_TONE3, "🤦🏽", "", U_1F926_1F3FD, "1f926-1f3fd.png");
png_name!(PERSON_FACEPALMING_TONE3, "🤦🏽", "", U_1F926_1F3FD, "1f926-1f3fd.png");
png_name!(WOMAN_FACEPALMING_TONE4, "🤦🏾‍♀️", "", U_1F926_1F3FE_200D_2640_FE0F, "1f926-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_FACEPALMING_TONE4, "🤦🏾‍♂️", "", U_1F926_1F3FE_200D_2642_FE0F, "1f926-1f3fe-200d-2642-fe0f.png");
png_name!(FACEPALM_TONE4, "🤦🏾", "", U_1F926_1F3FE, "1f926-1f3fe.png");
png_name!(PERSON_FACEPALMING_TONE4, "🤦🏾", "", U_1F926_1F3FE, "1f926-1f3fe.png");
png_name!(WOMAN_FACEPALMING_TONE5, "🤦🏿‍♀️", "", U_1F926_1F3FF_200D_2640_FE0F, "1f926-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_FACEPALMING_TONE5, "🤦🏿‍♂️", "", U_1F926_1F3FF_200D_2642_FE0F, "1f926-1f3ff-200d-2642-fe0f.png");
png_name!(FACEPALM_TONE5, "🤦🏿", "", U_1F926_1F3FF, "1f926-1f3ff.png");
png_name!(PERSON_FACEPALMING_TONE5, "🤦🏿", "", U_1F926_1F3FF, "1f926-1f3ff.png");
png_name!(WOMAN_FACEPALMING, "🤦‍♀️", "woman facepalming", U_1F926_200D_2640_FE0F, "1f926-200d-2640-fe0f.png");
png_name!(MAN_FACEPALMING, "🤦‍♂️", "man facepalming", U_1F926_200D_2642_FE0F, "1f926-200d-2642-fe0f.png");
png_name!(FACEPALM, "🤦", "person facepalming", U_1F926, "1f926.png");
png_name!(PERSON_FACEPALMING, "🤦", "person facepalming", U_1F926, "1f926.png");
png_name!(SNEEZING, "🤧", "sneezing face", U_1F927, "1f927.png");
png_name!(SNEEZING_FACE, "🤧", "sneezing face", U_1F927, "1f927.png");
png_name!(FACE_WITH_RAISED_EYEBROW, "🤨", "face with raised eyebrow", U_1F928, "1f928.png");
png_name!(RAISED_EYEBROW, "🤨", "face with raised eyebrow", U_1F928, "1f928.png");
png_name!(STAR_STRUCK, "🤩", "star-struck", U_1F929, "1f929.png");
png_name!(ZANY, "🤪", "zany face", U_1F92A, "1f92a.png");
png_name!(ZANY_FACE, "🤪", "zany face", U_1F92A, "1f92a.png");
png_name!(SHUSH, "🤫", "shushing face", U_1F92B, "1f92b.png");
png_name!(SHUSHING_FACE, "🤫", "shushing face", U_1F92B, "1f92b.png");
png_name!(CENSORED, "🤬", "face with symbols on mouth", U_1F92C, "1f92c.png");
png_name!(FACE_WITH_SYMBOLS_ON_MOUTH, "🤬", "face with symbols on mouth", U_1F92C, "1f92c.png");
png_name!(FACE_WITH_HAND_OVER_MOUTH, "🤭", "face with hand over mouth", U_1F92D, "1f92d.png");
png_name!(HAND_OVER_MOUTH, "🤭", "face with hand over mouth", U_1F92D, "1f92d.png");
png_name!(FACE_VOMITING, "🤮", "face vomiting", U_1F92E, "1f92e.png");
png_name!(VOMITING, "🤮", "face vomiting", U_1F92E, "1f92e.png");
png_name!(EXPLODING_HEAD, "🤯", "exploding head", U_1F92F, "1f92f.png");
png_name!(PREGNANT_WOMAN_TONE1, "🤰🏻", "", U_1F930_1F3FB, "1f930-1f3fb.png");
png_name!(PREGNANT_WOMAN_TONE2, "🤰🏼", "", U_1F930_1F3FC, "1f930-1f3fc.png");
png_name!(PREGNANT_WOMAN_TONE3, "🤰🏽", "", U_1F930_1F3FD, "1f930-1f3fd.png");
png_name!(PREGNANT_WOMAN_TONE4, "🤰🏾", "", U_1F930_1F3FE, "1f930-1f3fe.png");
png_name!(PREGNANT_WOMAN_TONE5, "🤰🏿", "", U_1F930_1F3FF, "1f930-1f3ff.png");
png_name!(PREGNANT_WOMAN, "🤰", "pregnant woman", U_1F930, "1f930.png");
png_name!(BREAST_FEEDING_TONE1, "🤱🏻", "", U_1F931_1F3FB, "1f931-1f3fb.png");
png_name!(BREAST_FEEDING_TONE2, "🤱🏼", "", U_1F931_1F3FC, "1f931-1f3fc.png");
png_name!(BREAST_FEEDING_TONE3, "🤱🏽", "", U_1F931_1F3FD, "1f931-1f3fd.png");
png_name!(BREAST_FEEDING_TONE4, "🤱🏾", "", U_1F931_1F3FE, "1f931-1f3fe.png");
png_name!(BREAST_FEEDING_TONE5, "🤱🏿", "", U_1F931_1F3FF, "1f931-1f3ff.png");
png_name!(BREAST_FEEDING, "🤱", "breast-feeding", U_1F931, "1f931.png");
png_name!(PALMS_UP_TOGETHER_TONE1, "🤲🏻", "", U_1F932_1F3FB, "1f932-1f3fb.png");
png_name!(PALMS_UP_TOGETHER_TONE2, "🤲🏼", "", U_1F932_1F3FC, "1f932-1f3fc.png");
png_name!(PALMS_UP_TOGETHER_TONE3, "🤲🏽", "", U_1F932_1F3FD, "1f932-1f3fd.png");
png_name!(PALMS_UP_TOGETHER_TONE4, "🤲🏾", "", U_1F932_1F3FE, "1f932-1f3fe.png");
png_name!(PALMS_UP_TOGETHER_TONE5, "🤲🏿", "", U_1F932_1F3FF, "1f932-1f3ff.png");
png_name!(PALMS_UP_TOGETHER, "🤲", "palms up together", U_1F932, "1f932.png");
png_name!(SELFIE_TONE1, "🤳🏻", "", U_1F933_1F3FB, "1f933-1f3fb.png");
png_name!(SELFIE_TONE2, "🤳🏼", "", U_1F933_1F3FC, "1f933-1f3fc.png");
png_name!(SELFIE_TONE3, "🤳🏽", "", U_1F933_1F3FD, "1f933-1f3fd.png");
png_name!(SELFIE_TONE4, "🤳🏾", "", U_1F933_1F3FE, "1f933-1f3fe.png");
png_name!(SELFIE_TONE5, "🤳🏿", "", U_1F933_1F3FF, "1f933-1f3ff.png");
png_name!(SELFIE, "🤳", "selfie", U_1F933, "1f933.png");
png_name!(PRINCE_TONE1, "🤴🏻", "", U_1F934_1F3FB, "1f934-1f3fb.png");
png_name!(PRINCE_TONE2, "🤴🏼", "", U_1F934_1F3FC, "1f934-1f3fc.png");
png_name!(PRINCE_TONE3, "🤴🏽", "", U_1F934_1F3FD, "1f934-1f3fd.png");
png_name!(PRINCE_TONE4, "🤴🏾", "", U_1F934_1F3FE, "1f934-1f3fe.png");
png_name!(PRINCE_TONE5, "🤴🏿", "", U_1F934_1F3FF, "1f934-1f3ff.png");
png_name!(PRINCE, "🤴", "prince", U_1F934, "1f934.png");
png_name!(WOMAN_IN_TUXEDO_TONE1, "🤵🏻‍♀️", "", U_1F935_1F3FB_200D_2640_FE0F, "1f935-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_IN_TUXEDO_TONE1, "🤵🏻‍♂️", "", U_1F935_1F3FB_200D_2642_FE0F, "1f935-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_IN_TUXEDO_TONE1, "🤵🏻", "", U_1F935_1F3FB, "1f935-1f3fb.png");
png_name!(WOMAN_IN_TUXEDO_TONE2, "🤵🏼‍♀️", "", U_1F935_1F3FC_200D_2640_FE0F, "1f935-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_IN_TUXEDO_TONE2, "🤵🏼‍♂️", "", U_1F935_1F3FC_200D_2642_FE0F, "1f935-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_IN_TUXEDO_TONE2, "🤵🏼", "", U_1F935_1F3FC, "1f935-1f3fc.png");
png_name!(WOMAN_IN_TUXEDO_TONE3, "🤵🏽‍♀️", "", U_1F935_1F3FD_200D_2640_FE0F, "1f935-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_IN_TUXEDO_TONE3, "🤵🏽‍♂️", "", U_1F935_1F3FD_200D_2642_FE0F, "1f935-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_IN_TUXEDO_TONE3, "🤵🏽", "", U_1F935_1F3FD, "1f935-1f3fd.png");
png_name!(WOMAN_IN_TUXEDO_TONE4, "🤵🏾‍♀️", "", U_1F935_1F3FE_200D_2640_FE0F, "1f935-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_IN_TUXEDO_TONE4, "🤵🏾‍♂️", "", U_1F935_1F3FE_200D_2642_FE0F, "1f935-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_IN_TUXEDO_TONE4, "🤵🏾", "", U_1F935_1F3FE, "1f935-1f3fe.png");
png_name!(WOMAN_IN_TUXEDO_TONE5, "🤵🏿‍♀️", "", U_1F935_1F3FF_200D_2640_FE0F, "1f935-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_IN_TUXEDO_TONE5, "🤵🏿‍♂️", "", U_1F935_1F3FF_200D_2642_FE0F, "1f935-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_IN_TUXEDO_TONE5, "🤵🏿", "", U_1F935_1F3FF, "1f935-1f3ff.png");
png_name!(WOMAN_IN_TUXEDO, "🤵‍♀️", "woman in tuxedo", U_1F935_200D_2640_FE0F, "1f935-200d-2640-fe0f.png");
png_name!(MAN_IN_TUXEDO, "🤵‍♂️", "man in tuxedo", U_1F935_200D_2642_FE0F, "1f935-200d-2642-fe0f.png");
png_name!(PERSON_IN_TUXEDO, "🤵", "person in tuxedo", U_1F935, "1f935.png");
png_name!(MRS_CLAUS_TONE1, "🤶🏻", "", U_1F936_1F3FB, "1f936-1f3fb.png");
png_name!(MRS_CLAUS_TONE2, "🤶🏼", "", U_1F936_1F3FC, "1f936-1f3fc.png");
png_name!(MRS_CLAUS_TONE3, "🤶🏽", "", U_1F936_1F3FD, "1f936-1f3fd.png");
png_name!(MRS_CLAUS_TONE4, "🤶🏾", "", U_1F936_1F3FE, "1f936-1f3fe.png");
png_name!(MRS_CLAUS_TONE5, "🤶🏿", "", U_1F936_1F3FF, "1f936-1f3ff.png");
png_name!(MRS_CLAUS, "🤶", "Mrs. Claus", U_1F936, "1f936.png");
png_name!(WOMAN_SHRUGGING_TONE1, "🤷🏻‍♀️", "", U_1F937_1F3FB_200D_2640_FE0F, "1f937-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_SHRUGGING_TONE1, "🤷🏻‍♂️", "", U_1F937_1F3FB_200D_2642_FE0F, "1f937-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_SHRUGGING_TONE1, "🤷🏻", "", U_1F937_1F3FB, "1f937-1f3fb.png");
png_name!(SHRUG_TONE1, "🤷🏻", "", U_1F937_1F3FB, "1f937-1f3fb.png");
png_name!(WOMAN_SHRUGGING_TONE2, "🤷🏼‍♀️", "", U_1F937_1F3FC_200D_2640_FE0F, "1f937-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_SHRUGGING_TONE2, "🤷🏼‍♂️", "", U_1F937_1F3FC_200D_2642_FE0F, "1f937-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_SHRUGGING_TONE2, "🤷🏼", "", U_1F937_1F3FC, "1f937-1f3fc.png");
png_name!(SHRUG_TONE2, "🤷🏼", "", U_1F937_1F3FC, "1f937-1f3fc.png");
png_name!(WOMAN_SHRUGGING_TONE3, "🤷🏽‍♀️", "", U_1F937_1F3FD_200D_2640_FE0F, "1f937-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_SHRUGGING_TONE3, "🤷🏽‍♂️", "", U_1F937_1F3FD_200D_2642_FE0F, "1f937-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_SHRUGGING_TONE3, "🤷🏽", "", U_1F937_1F3FD, "1f937-1f3fd.png");
png_name!(SHRUG_TONE3, "🤷🏽", "", U_1F937_1F3FD, "1f937-1f3fd.png");
png_name!(WOMAN_SHRUGGING_TONE4, "🤷🏾‍♀️", "", U_1F937_1F3FE_200D_2640_FE0F, "1f937-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_SHRUGGING_TONE4, "🤷🏾‍♂️", "", U_1F937_1F3FE_200D_2642_FE0F, "1f937-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_SHRUGGING_TONE4, "🤷🏾", "", U_1F937_1F3FE, "1f937-1f3fe.png");
png_name!(SHRUG_TONE4, "🤷🏾", "", U_1F937_1F3FE, "1f937-1f3fe.png");
png_name!(WOMAN_SHRUGGING_TONE5, "🤷🏿‍♀️", "", U_1F937_1F3FF_200D_2640_FE0F, "1f937-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_SHRUGGING_TONE5, "🤷🏿‍♂️", "", U_1F937_1F3FF_200D_2642_FE0F, "1f937-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_SHRUGGING_TONE5, "🤷🏿", "", U_1F937_1F3FF, "1f937-1f3ff.png");
png_name!(SHRUG_TONE5, "🤷🏿", "", U_1F937_1F3FF, "1f937-1f3ff.png");
png_name!(WOMAN_SHRUGGING, "🤷‍♀️", "woman shrugging", U_1F937_200D_2640_FE0F, "1f937-200d-2640-fe0f.png");
png_name!(MAN_SHRUGGING, "🤷‍♂️", "man shrugging", U_1F937_200D_2642_FE0F, "1f937-200d-2642-fe0f.png");
png_name!(PERSON_SHRUGGING, "🤷", "person shrugging", U_1F937, "1f937.png");
png_name!(SHRUG, "🤷", "person shrugging", U_1F937, "1f937.png");
png_name!(WOMAN_CARTWHEELING_TONE1, "🤸🏻‍♀️", "", U_1F938_1F3FB_200D_2640_FE0F, "1f938-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_CARTWHEELING_TONE1, "🤸🏻‍♂️", "", U_1F938_1F3FB_200D_2642_FE0F, "1f938-1f3fb-200d-2642-fe0f.png");
png_name!(CARTWHEELING_TONE1, "🤸🏻", "", U_1F938_1F3FB, "1f938-1f3fb.png");
png_name!(PERSON_CARTWHEEL_TONE1, "🤸🏻", "", U_1F938_1F3FB, "1f938-1f3fb.png");
png_name!(WOMAN_CARTWHEELING_TONE2, "🤸🏼‍♀️", "", U_1F938_1F3FC_200D_2640_FE0F, "1f938-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_CARTWHEELING_TONE2, "🤸🏼‍♂️", "", U_1F938_1F3FC_200D_2642_FE0F, "1f938-1f3fc-200d-2642-fe0f.png");
png_name!(CARTWHEELING_TONE2, "🤸🏼", "", U_1F938_1F3FC, "1f938-1f3fc.png");
png_name!(PERSON_CARTWHEEL_TONE2, "🤸🏼", "", U_1F938_1F3FC, "1f938-1f3fc.png");
png_name!(WOMAN_CARTWHEELING_TONE3, "🤸🏽‍♀️", "", U_1F938_1F3FD_200D_2640_FE0F, "1f938-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_CARTWHEELING_TONE3, "🤸🏽‍♂️", "", U_1F938_1F3FD_200D_2642_FE0F, "1f938-1f3fd-200d-2642-fe0f.png");
png_name!(CARTWHEELING_TONE3, "🤸🏽", "", U_1F938_1F3FD, "1f938-1f3fd.png");
png_name!(PERSON_CARTWHEEL_TONE3, "🤸🏽", "", U_1F938_1F3FD, "1f938-1f3fd.png");
png_name!(WOMAN_CARTWHEELING_TONE4, "🤸🏾‍♀️", "", U_1F938_1F3FE_200D_2640_FE0F, "1f938-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_CARTWHEELING_TONE4, "🤸🏾‍♂️", "", U_1F938_1F3FE_200D_2642_FE0F, "1f938-1f3fe-200d-2642-fe0f.png");
png_name!(CARTWHEELING_TONE4, "🤸🏾", "", U_1F938_1F3FE, "1f938-1f3fe.png");
png_name!(PERSON_CARTWHEEL_TONE4, "🤸🏾", "", U_1F938_1F3FE, "1f938-1f3fe.png");
png_name!(WOMAN_CARTWHEELING_TONE5, "🤸🏿‍♀️", "", U_1F938_1F3FF_200D_2640_FE0F, "1f938-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_CARTWHEELING_TONE5, "🤸🏿‍♂️", "", U_1F938_1F3FF_200D_2642_FE0F, "1f938-1f3ff-200d-2642-fe0f.png");
png_name!(CARTWHEELING_TONE5, "🤸🏿", "", U_1F938_1F3FF, "1f938-1f3ff.png");
png_name!(PERSON_CARTWHEEL_TONE5, "🤸🏿", "", U_1F938_1F3FF, "1f938-1f3ff.png");
png_name!(WOMAN_CARTWHEELING, "🤸‍♀️", "woman cartwheeling", U_1F938_200D_2640_FE0F, "1f938-200d-2640-fe0f.png");
png_name!(MAN_CARTWHEELING, "🤸‍♂️", "man cartwheeling", U_1F938_200D_2642_FE0F, "1f938-200d-2642-fe0f.png");
png_name!(CARTWHEELING, "🤸", "person cartwheeling", U_1F938, "1f938.png");
png_name!(PERSON_CARTWHEEL, "🤸", "person cartwheeling", U_1F938, "1f938.png");
png_name!(WOMAN_JUGGLING_TONE1, "🤹🏻‍♀️", "", U_1F939_1F3FB_200D_2640_FE0F, "1f939-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_JUGGLING_TONE1, "🤹🏻‍♂️", "", U_1F939_1F3FB_200D_2642_FE0F, "1f939-1f3fb-200d-2642-fe0f.png");
png_name!(JUGGLER_TONE1, "🤹🏻", "", U_1F939_1F3FB, "1f939-1f3fb.png");
png_name!(JUGGLING_TONE1, "🤹🏻", "", U_1F939_1F3FB, "1f939-1f3fb.png");
png_name!(PERSON_JUGGLING_TONE1, "🤹🏻", "", U_1F939_1F3FB, "1f939-1f3fb.png");
png_name!(WOMAN_JUGGLING_TONE2, "🤹🏼‍♀️", "", U_1F939_1F3FC_200D_2640_FE0F, "1f939-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_JUGGLING_TONE2, "🤹🏼‍♂️", "", U_1F939_1F3FC_200D_2642_FE0F, "1f939-1f3fc-200d-2642-fe0f.png");
png_name!(JUGGLER_TONE2, "🤹🏼", "", U_1F939_1F3FC, "1f939-1f3fc.png");
png_name!(JUGGLING_TONE2, "🤹🏼", "", U_1F939_1F3FC, "1f939-1f3fc.png");
png_name!(PERSON_JUGGLING_TONE2, "🤹🏼", "", U_1F939_1F3FC, "1f939-1f3fc.png");
png_name!(WOMAN_JUGGLING_TONE3, "🤹🏽‍♀️", "", U_1F939_1F3FD_200D_2640_FE0F, "1f939-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_JUGGLING_TONE3, "🤹🏽‍♂️", "", U_1F939_1F3FD_200D_2642_FE0F, "1f939-1f3fd-200d-2642-fe0f.png");
png_name!(JUGGLER_TONE3, "🤹🏽", "", U_1F939_1F3FD, "1f939-1f3fd.png");
png_name!(JUGGLING_TONE3, "🤹🏽", "", U_1F939_1F3FD, "1f939-1f3fd.png");
png_name!(PERSON_JUGGLING_TONE3, "🤹🏽", "", U_1F939_1F3FD, "1f939-1f3fd.png");
png_name!(WOMAN_JUGGLING_TONE4, "🤹🏾‍♀️", "", U_1F939_1F3FE_200D_2640_FE0F, "1f939-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_JUGGLING_TONE4, "🤹🏾‍♂️", "", U_1F939_1F3FE_200D_2642_FE0F, "1f939-1f3fe-200d-2642-fe0f.png");
png_name!(JUGGLER_TONE4, "🤹🏾", "", U_1F939_1F3FE, "1f939-1f3fe.png");
png_name!(JUGGLING_TONE4, "🤹🏾", "", U_1F939_1F3FE, "1f939-1f3fe.png");
png_name!(PERSON_JUGGLING_TONE4, "🤹🏾", "", U_1F939_1F3FE, "1f939-1f3fe.png");
png_name!(WOMAN_JUGGLING_TONE5, "🤹🏿‍♀️", "", U_1F939_1F3FF_200D_2640_FE0F, "1f939-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_JUGGLING_TONE5, "🤹🏿‍♂️", "", U_1F939_1F3FF_200D_2642_FE0F, "1f939-1f3ff-200d-2642-fe0f.png");
png_name!(JUGGLER_TONE5, "🤹🏿", "", U_1F939_1F3FF, "1f939-1f3ff.png");
png_name!(JUGGLING_TONE5, "🤹🏿", "", U_1F939_1F3FF, "1f939-1f3ff.png");
png_name!(PERSON_JUGGLING_TONE5, "🤹🏿", "", U_1F939_1F3FF, "1f939-1f3ff.png");
png_name!(WOMAN_JUGGLING, "🤹‍♀️", "woman juggling", U_1F939_200D_2640_FE0F, "1f939-200d-2640-fe0f.png");
png_name!(MAN_JUGGLING, "🤹‍♂️", "man juggling", U_1F939_200D_2642_FE0F, "1f939-200d-2642-fe0f.png");
png_name!(JUGGLER, "🤹", "person juggling", U_1F939, "1f939.png");
png_name!(JUGGLING, "🤹", "person juggling", U_1F939, "1f939.png");
png_name!(PERSON_JUGGLING, "🤹", "person juggling", U_1F939, "1f939.png");
png_name!(FENCER, "🤺", "person fencing", U_1F93A, "1f93a.png");
png_name!(FENCING, "🤺", "person fencing", U_1F93A, "1f93a.png");
png_name!(PERSON_FENCING, "🤺", "person fencing", U_1F93A, "1f93a.png");
png_name!(WOMEN_WRESTLING, "🤼‍♀️", "women wrestling", U_1F93C_200D_2640_FE0F, "1f93c-200d-2640-fe0f.png");
png_name!(MEN_WRESTLING, "🤼‍♂️", "men wrestling", U_1F93C_200D_2642_FE0F, "1f93c-200d-2642-fe0f.png");
png_name!(PEOPLE_WRESTLING, "🤼", "people wrestling", U_1F93C, "1f93c.png");
png_name!(WRESTLERS, "🤼", "people wrestling", U_1F93C, "1f93c.png");
png_name!(WRESTLING, "🤼", "people wrestling", U_1F93C, "1f93c.png");
png_name!(WOMAN_PLAYING_WATER_POLO_TONE1, "🤽🏻‍♀️", "", U_1F93D_1F3FB_200D_2640_FE0F, "1f93d-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_PLAYING_WATER_POLO_TONE1, "🤽🏻‍♂️", "", U_1F93D_1F3FB_200D_2642_FE0F, "1f93d-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_PLAYING_WATER_POLO_TONE1, "🤽🏻", "", U_1F93D_1F3FB, "1f93d-1f3fb.png");
png_name!(WATER_POLO_TONE1, "🤽🏻", "", U_1F93D_1F3FB, "1f93d-1f3fb.png");
png_name!(WOMAN_PLAYING_WATER_POLO_TONE2, "🤽🏼‍♀️", "", U_1F93D_1F3FC_200D_2640_FE0F, "1f93d-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_PLAYING_WATER_POLO_TONE2, "🤽🏼‍♂️", "", U_1F93D_1F3FC_200D_2642_FE0F, "1f93d-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_PLAYING_WATER_POLO_TONE2, "🤽🏼", "", U_1F93D_1F3FC, "1f93d-1f3fc.png");
png_name!(WATER_POLO_TONE2, "🤽🏼", "", U_1F93D_1F3FC, "1f93d-1f3fc.png");
png_name!(WOMAN_PLAYING_WATER_POLO_TONE3, "🤽🏽‍♀️", "", U_1F93D_1F3FD_200D_2640_FE0F, "1f93d-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_PLAYING_WATER_POLO_TONE3, "🤽🏽‍♂️", "", U_1F93D_1F3FD_200D_2642_FE0F, "1f93d-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_PLAYING_WATER_POLO_TONE3, "🤽🏽", "", U_1F93D_1F3FD, "1f93d-1f3fd.png");
png_name!(WATER_POLO_TONE3, "🤽🏽", "", U_1F93D_1F3FD, "1f93d-1f3fd.png");
png_name!(WOMAN_PLAYING_WATER_POLO_TONE4, "🤽🏾‍♀️", "", U_1F93D_1F3FE_200D_2640_FE0F, "1f93d-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_PLAYING_WATER_POLO_TONE4, "🤽🏾‍♂️", "", U_1F93D_1F3FE_200D_2642_FE0F, "1f93d-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_PLAYING_WATER_POLO_TONE4, "🤽🏾", "", U_1F93D_1F3FE, "1f93d-1f3fe.png");
png_name!(WATER_POLO_TONE4, "🤽🏾", "", U_1F93D_1F3FE, "1f93d-1f3fe.png");
png_name!(WOMAN_PLAYING_WATER_POLO_TONE5, "🤽🏿‍♀️", "", U_1F93D_1F3FF_200D_2640_FE0F, "1f93d-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_PLAYING_WATER_POLO_TONE5, "🤽🏿‍♂️", "", U_1F93D_1F3FF_200D_2642_FE0F, "1f93d-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_PLAYING_WATER_POLO_TONE5, "🤽🏿", "", U_1F93D_1F3FF, "1f93d-1f3ff.png");
png_name!(WATER_POLO_TONE5, "🤽🏿", "", U_1F93D_1F3FF, "1f93d-1f3ff.png");
png_name!(WOMAN_PLAYING_WATER_POLO, "🤽‍♀️", "woman playing water polo", U_1F93D_200D_2640_FE0F, "1f93d-200d-2640-fe0f.png");
png_name!(MAN_PLAYING_WATER_POLO, "🤽‍♂️", "man playing water polo", U_1F93D_200D_2642_FE0F, "1f93d-200d-2642-fe0f.png");
png_name!(PERSON_PLAYING_WATER_POLO, "🤽", "person playing water polo", U_1F93D, "1f93d.png");
png_name!(WATER_POLO, "🤽", "person playing water polo", U_1F93D, "1f93d.png");
png_name!(WOMAN_PLAYING_HANDBALL_TONE1, "🤾🏻‍♀️", "", U_1F93E_1F3FB_200D_2640_FE0F, "1f93e-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_PLAYING_HANDBALL_TONE1, "🤾🏻‍♂️", "", U_1F93E_1F3FB_200D_2642_FE0F, "1f93e-1f3fb-200d-2642-fe0f.png");
png_name!(HANDBALL_TONE1, "🤾🏻", "", U_1F93E_1F3FB, "1f93e-1f3fb.png");
png_name!(PERSON_PLAYING_HANDBALL_TONE1, "🤾🏻", "", U_1F93E_1F3FB, "1f93e-1f3fb.png");
png_name!(WOMAN_PLAYING_HANDBALL_TONE2, "🤾🏼‍♀️", "", U_1F93E_1F3FC_200D_2640_FE0F, "1f93e-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_PLAYING_HANDBALL_TONE2, "🤾🏼‍♂️", "", U_1F93E_1F3FC_200D_2642_FE0F, "1f93e-1f3fc-200d-2642-fe0f.png");
png_name!(HANDBALL_TONE2, "🤾🏼", "", U_1F93E_1F3FC, "1f93e-1f3fc.png");
png_name!(PERSON_PLAYING_HANDBALL_TONE2, "🤾🏼", "", U_1F93E_1F3FC, "1f93e-1f3fc.png");
png_name!(WOMAN_PLAYING_HANDBALL_TONE3, "🤾🏽‍♀️", "", U_1F93E_1F3FD_200D_2640_FE0F, "1f93e-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_PLAYING_HANDBALL_TONE3, "🤾🏽‍♂️", "", U_1F93E_1F3FD_200D_2642_FE0F, "1f93e-1f3fd-200d-2642-fe0f.png");
png_name!(HANDBALL_TONE3, "🤾🏽", "", U_1F93E_1F3FD, "1f93e-1f3fd.png");
png_name!(PERSON_PLAYING_HANDBALL_TONE3, "🤾🏽", "", U_1F93E_1F3FD, "1f93e-1f3fd.png");
png_name!(WOMAN_PLAYING_HANDBALL_TONE4, "🤾🏾‍♀️", "", U_1F93E_1F3FE_200D_2640_FE0F, "1f93e-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_PLAYING_HANDBALL_TONE4, "🤾🏾‍♂️", "", U_1F93E_1F3FE_200D_2642_FE0F, "1f93e-1f3fe-200d-2642-fe0f.png");
png_name!(HANDBALL_TONE4, "🤾🏾", "", U_1F93E_1F3FE, "1f93e-1f3fe.png");
png_name!(PERSON_PLAYING_HANDBALL_TONE4, "🤾🏾", "", U_1F93E_1F3FE, "1f93e-1f3fe.png");
png_name!(WOMAN_PLAYING_HANDBALL_TONE5, "🤾🏿‍♀️", "", U_1F93E_1F3FF_200D_2640_FE0F, "1f93e-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_PLAYING_HANDBALL_TONE5, "🤾🏿‍♂️", "", U_1F93E_1F3FF_200D_2642_FE0F, "1f93e-1f3ff-200d-2642-fe0f.png");
png_name!(HANDBALL_TONE5, "🤾🏿", "", U_1F93E_1F3FF, "1f93e-1f3ff.png");
png_name!(PERSON_PLAYING_HANDBALL_TONE5, "🤾🏿", "", U_1F93E_1F3FF, "1f93e-1f3ff.png");
png_name!(WOMAN_PLAYING_HANDBALL, "🤾‍♀️", "woman playing handball", U_1F93E_200D_2640_FE0F, "1f93e-200d-2640-fe0f.png");
png_name!(MAN_PLAYING_HANDBALL, "🤾‍♂️", "man playing handball", U_1F93E_200D_2642_FE0F, "1f93e-200d-2642-fe0f.png");
png_name!(HANDBALL, "🤾", "person playing handball", U_1F93E, "1f93e.png");
png_name!(PERSON_PLAYING_HANDBALL, "🤾", "person playing handball", U_1F93E, "1f93e.png");
png_name!(DIVING_MASK, "🤿", "diving mask", U_1F93F, "1f93f.png");
png_name!(WILTED_FLOWER, "🥀", "wilted flower", U_1F940, "1f940.png");
png_name!(DRUM, "🥁", "drum", U_1F941, "1f941.png");
png_name!(CLINKING_GLASSES, "🥂", "clinking glasses", U_1F942, "1f942.png");
png_name!(TUMBLER_GLASS, "🥃", "tumbler glass", U_1F943, "1f943.png");
png_name!(WHISKY, "🥃", "tumbler glass", U_1F943, "1f943.png");
png_name!(SPOON, "🥄", "spoon", U_1F944, "1f944.png");
png_name!(GOAL_NET, "🥅", "goal net", U_1F945, "1f945.png");
png_name!(X_1ST, "🥇", "1st place medal", U_1F947, "1f947.png");
png_name!(FIRST_PLACE_MEDAL, "🥇", "1st place medal", U_1F947, "1f947.png");
png_name!(X_2ND, "🥈", "2nd place medal", U_1F948, "1f948.png");
png_name!(SECOND_PLACE_MEDAL, "🥈", "2nd place medal", U_1F948, "1f948.png");
png_name!(X_3RD, "🥉", "3rd place medal", U_1F949, "1f949.png");
png_name!(THIRD_PLACE_MEDAL, "🥉", "3rd place medal", U_1F949, "1f949.png");
png_name!(BOXING_GLOVE, "🥊", "boxing glove", U_1F94A, "1f94a.png");
png_name!(MARTIAL_ARTS_UNIFORM, "🥋", "martial arts uniform", U_1F94B, "1f94b.png");
png_name!(CURLING_STONE, "🥌", "curling stone", U_1F94C, "1f94c.png");
png_name!(LACROSSE, "🥍", "lacrosse", U_1F94D, "1f94d.png");
png_name!(SOFTBALL, "🥎", "softball", U_1F94E, "1f94e.png");
png_name!(FLYING_DISC, "🥏", "flying disc", U_1F94F, "1f94f.png");
png_name!(CROISSANT, "🥐", "croissant", U_1F950, "1f950.png");
png_name!(AVOCADO, "🥑", "avocado", U_1F951, "1f951.png");
png_name!(CUCUMBER, "🥒", "cucumber", U_1F952, "1f952.png");
png_name!(BACON, "🥓", "bacon", U_1F953, "1f953.png");
png_name!(POTATO, "🥔", "potato", U_1F954, "1f954.png");
png_name!(CARROT, "🥕", "carrot", U_1F955, "1f955.png");
png_name!(BAGUETTE_BREAD, "🥖", "baguette bread", U_1F956, "1f956.png");
png_name!(GREEN_SALAD, "🥗", "green salad", U_1F957, "1f957.png");
png_name!(SALAD, "🥗", "green salad", U_1F957, "1f957.png");
png_name!(SHALLOW_PAN_OF_FOOD, "🥘", "shallow pan of food", U_1F958, "1f958.png");
png_name!(STUFFED_FLATBREAD, "🥙", "stuffed flatbread", U_1F959, "1f959.png");
png_name!(EGG, "🥚", "egg", U_1F95A, "1f95a.png");
png_name!(GLASS_OF_MILK, "🥛", "glass of milk", U_1F95B, "1f95b.png");
png_name!(MILK, "🥛", "glass of milk", U_1F95B, "1f95b.png");
png_name!(PEANUTS, "🥜", "peanuts", U_1F95C, "1f95c.png");
png_name!(KIWI, "🥝", "kiwi fruit", U_1F95D, "1f95d.png");
png_name!(PANCAKES, "🥞", "pancakes", U_1F95E, "1f95e.png");
png_name!(DUMPLING, "🥟", "dumpling", U_1F95F, "1f95f.png");
png_name!(FORTUNE_COOKIE, "🥠", "fortune cookie", U_1F960, "1f960.png");
png_name!(TAKEOUT_BOX, "🥡", "takeout box", U_1F961, "1f961.png");
png_name!(CHOPSTICKS, "🥢", "chopsticks", U_1F962, "1f962.png");
png_name!(BOWL_WITH_SPOON, "🥣", "bowl with spoon", U_1F963, "1f963.png");
png_name!(CUP_WITH_STRAW, "🥤", "cup with straw", U_1F964, "1f964.png");
png_name!(COCONUT, "🥥", "coconut", U_1F965, "1f965.png");
png_name!(BROCCOLI, "🥦", "broccoli", U_1F966, "1f966.png");
png_name!(PIE, "🥧", "pie", U_1F967, "1f967.png");
png_name!(PRETZEL, "🥨", "pretzel", U_1F968, "1f968.png");
png_name!(CUT_OF_MEAT, "🥩", "cut of meat", U_1F969, "1f969.png");
png_name!(SANDWICH, "🥪", "sandwich", U_1F96A, "1f96a.png");
png_name!(CANNED_FOOD, "🥫", "canned food", U_1F96B, "1f96b.png");
png_name!(LEAFY_GREEN, "🥬", "leafy green", U_1F96C, "1f96c.png");
png_name!(MANGO, "🥭", "mango", U_1F96D, "1f96d.png");
png_name!(MOON_CAKE, "🥮", "moon cake", U_1F96E, "1f96e.png");
png_name!(BAGEL, "🥯", "bagel", U_1F96F, "1f96f.png");
png_name!(SMILING_FACE_WITH_3_HEARTS, "🥰", "smiling face with hearts", U_1F970, "1f970.png");
png_name!(YAWN, "🥱", "yawning face", U_1F971, "1f971.png");
png_name!(YAWNING, "🥱", "yawning face", U_1F971, "1f971.png");
png_name!(YAWNING_FACE, "🥱", "yawning face", U_1F971, "1f971.png");
png_name!(SMILING_FACE_WITH_TEAR, "🥲", "smiling face with tear", U_1F972, "1f972.png");
png_name!(HOORAY, "🥳", "partying face", U_1F973, "1f973.png");
png_name!(PARTYING, "🥳", "partying face", U_1F973, "1f973.png");
png_name!(PARTYING_FACE, "🥳", "partying face", U_1F973, "1f973.png");
png_name!(WOOZY, "🥴", "woozy face", U_1F974, "1f974.png");
png_name!(WOOZY_FACE, "🥴", "woozy face", U_1F974, "1f974.png");
png_name!(HOT, "🥵", "hot face", U_1F975, "1f975.png");
png_name!(HOT_FACE, "🥵", "hot face", U_1F975, "1f975.png");
png_name!(COLD, "🥶", "cold face", U_1F976, "1f976.png");
png_name!(COLD_FACE, "🥶", "cold face", U_1F976, "1f976.png");
png_name!(NINJA_TONE1, "🥷🏻", "", U_1F977_1F3FB, "1f977-1f3fb.png");
png_name!(NINJA_TONE2, "🥷🏼", "", U_1F977_1F3FC, "1f977-1f3fc.png");
png_name!(NINJA_TONE3, "🥷🏽", "", U_1F977_1F3FD, "1f977-1f3fd.png");
png_name!(NINJA_TONE4, "🥷🏾", "", U_1F977_1F3FE, "1f977-1f3fe.png");
png_name!(NINJA_TONE5, "🥷🏿", "", U_1F977_1F3FF, "1f977-1f3ff.png");
png_name!(NINJA, "🥷", "ninja", U_1F977, "1f977.png");
png_name!(DISGUISED, "🥸", "disguised face", U_1F978, "1f978.png");
png_name!(DISGUISED_FACE, "🥸", "disguised face", U_1F978, "1f978.png");
png_name!(FACE_HOLDING_BACK_TEARS, "🥹", "face holding back tears", U_1F979, "1f979.png");
png_name!(WATERY_EYES, "🥹", "face holding back tears", U_1F979, "1f979.png");
png_name!(PLEADING, "🥺", "pleading face", U_1F97A, "1f97a.png");
png_name!(PLEADING_FACE, "🥺", "pleading face", U_1F97A, "1f97a.png");
png_name!(SARI, "🥻", "sari", U_1F97B, "1f97b.png");
png_name!(LAB_COAT, "🥼", "lab coat", U_1F97C, "1f97c.png");
png_name!(GOGGLES, "🥽", "goggles", U_1F97D, "1f97d.png");
png_name!(HIKING_BOOT, "🥾", "hiking boot", U_1F97E, "1f97e.png");
png_name!(FLAT_SHOE, "🥿", "flat shoe", U_1F97F, "1f97f.png");
png_name!(WOMANS_FLAT_SHOE, "🥿", "flat shoe", U_1F97F, "1f97f.png");
png_name!(CRAB, "🦀", "crab", U_1F980, "1f980.png");
png_name!(LION, "🦁", "lion", U_1F981, "1f981.png");
png_name!(LION_FACE, "🦁", "lion", U_1F981, "1f981.png");
png_name!(SCORPION, "🦂", "scorpion", U_1F982, "1f982.png");
png_name!(TURKEY, "🦃", "turkey", U_1F983, "1f983.png");
png_name!(UNICORN, "🦄", "unicorn", U_1F984, "1f984.png");
png_name!(UNICORN_FACE, "🦄", "unicorn", U_1F984, "1f984.png");
png_name!(EAGLE, "🦅", "eagle", U_1F985, "1f985.png");
png_name!(DUCK, "🦆", "duck", U_1F986, "1f986.png");
png_name!(BAT, "🦇", "bat", U_1F987, "1f987.png");
png_name!(SHARK, "🦈", "shark", U_1F988, "1f988.png");
png_name!(OWL, "🦉", "owl", U_1F989, "1f989.png");
png_name!(FOX, "🦊", "fox", U_1F98A, "1f98a.png");
png_name!(FOX_FACE, "🦊", "fox", U_1F98A, "1f98a.png");
png_name!(BUTTERFLY, "🦋", "butterfly", U_1F98B, "1f98b.png");
png_name!(DEER, "🦌", "deer", U_1F98C, "1f98c.png");
png_name!(GORILLA, "🦍", "gorilla", U_1F98D, "1f98d.png");
png_name!(LIZARD, "🦎", "lizard", U_1F98E, "1f98e.png");
png_name!(RHINO, "🦏", "rhinoceros", U_1F98F, "1f98f.png");
png_name!(RHINOCEROS, "🦏", "rhinoceros", U_1F98F, "1f98f.png");
png_name!(SHRIMP, "🦐", "shrimp", U_1F990, "1f990.png");
png_name!(SQUID, "🦑", "squid", U_1F991, "1f991.png");
png_name!(GIRAFFE, "🦒", "giraffe", U_1F992, "1f992.png");
png_name!(ZEBRA, "🦓", "zebra", U_1F993, "1f993.png");
png_name!(HEDGEHOG, "🦔", "hedgehog", U_1F994, "1f994.png");
png_name!(SAUROPOD, "🦕", "sauropod", U_1F995, "1f995.png");
png_name!(T_REX, "🦖", "T-Rex", U_1F996, "1f996.png");
png_name!(TREX, "🦖", "T-Rex", U_1F996, "1f996.png");
png_name!(CRICKET, "🦗", "cricket", U_1F997, "1f997.png");
png_name!(KANGAROO, "🦘", "kangaroo", U_1F998, "1f998.png");
png_name!(LLAMA, "🦙", "llama", U_1F999, "1f999.png");
png_name!(PEACOCK, "🦚", "peacock", U_1F99A, "1f99a.png");
png_name!(HIPPO, "🦛", "hippopotamus", U_1F99B, "1f99b.png");
png_name!(PARROT, "🦜", "parrot", U_1F99C, "1f99c.png");
png_name!(RACCOON, "🦝", "raccoon", U_1F99D, "1f99d.png");
png_name!(LOBSTER, "🦞", "lobster", U_1F99E, "1f99e.png");
png_name!(MOSQUITO, "🦟", "mosquito", U_1F99F, "1f99f.png");
png_name!(MICROBE, "🦠", "microbe", U_1F9A0, "1f9a0.png");
png_name!(BADGER, "🦡", "badger", U_1F9A1, "1f9a1.png");
png_name!(SWAN, "🦢", "swan", U_1F9A2, "1f9a2.png");
png_name!(MAMMOTH, "🦣", "mammoth", U_1F9A3, "1f9a3.png");
png_name!(DODO, "🦤", "dodo", U_1F9A4, "1f9a4.png");
png_name!(SLOTH, "🦥", "sloth", U_1F9A5, "1f9a5.png");
png_name!(OTTER, "🦦", "otter", U_1F9A6, "1f9a6.png");
png_name!(ORANGUTAN, "🦧", "orangutan", U_1F9A7, "1f9a7.png");
png_name!(SKUNK, "🦨", "skunk", U_1F9A8, "1f9a8.png");
png_name!(FLAMINGO, "🦩", "flamingo", U_1F9A9, "1f9a9.png");
png_name!(OYSTER, "🦪", "oyster", U_1F9AA, "1f9aa.png");
png_name!(BEAVER, "🦫", "beaver", U_1F9AB, "1f9ab.png");
png_name!(BISON, "🦬", "bison", U_1F9AC, "1f9ac.png");
png_name!(SEAL, "🦭", "seal", U_1F9AD, "1f9ad.png");
png_name!(GUIDE_DOG, "🦮", "guide dog", U_1F9AE, "1f9ae.png");
png_name!(PROBING_CANE, "🦯", "white cane", U_1F9AF, "1f9af.png");
png_name!(WHITE_CANE, "🦯", "white cane", U_1F9AF, "1f9af.png");
png_name!(RED_HAIR, "🦰", "red hair", U_1F9B0, "1f9b0.png");
png_name!(CURLY_HAIR, "🦱", "curly hair", U_1F9B1, "1f9b1.png");
png_name!(NO_HAIR, "🦲", "bald", U_1F9B2, "1f9b2.png");
png_name!(WHITE_HAIR, "🦳", "white hair", U_1F9B3, "1f9b3.png");
png_name!(BONE, "🦴", "bone", U_1F9B4, "1f9b4.png");
png_name!(LEG_TONE1, "🦵🏻", "", U_1F9B5_1F3FB, "1f9b5-1f3fb.png");
png_name!(LEG_TONE2, "🦵🏼", "", U_1F9B5_1F3FC, "1f9b5-1f3fc.png");
png_name!(LEG_TONE3, "🦵🏽", "", U_1F9B5_1F3FD, "1f9b5-1f3fd.png");
png_name!(LEG_TONE4, "🦵🏾", "", U_1F9B5_1F3FE, "1f9b5-1f3fe.png");
png_name!(LEG_TONE5, "🦵🏿", "", U_1F9B5_1F3FF, "1f9b5-1f3ff.png");
png_name!(LEG, "🦵", "leg", U_1F9B5, "1f9b5.png");
png_name!(FOOT_TONE1, "🦶🏻", "", U_1F9B6_1F3FB, "1f9b6-1f3fb.png");
png_name!(FOOT_TONE2, "🦶🏼", "", U_1F9B6_1F3FC, "1f9b6-1f3fc.png");
png_name!(FOOT_TONE3, "🦶🏽", "", U_1F9B6_1F3FD, "1f9b6-1f3fd.png");
png_name!(FOOT_TONE4, "🦶🏾", "", U_1F9B6_1F3FE, "1f9b6-1f3fe.png");
png_name!(FOOT_TONE5, "🦶🏿", "", U_1F9B6_1F3FF, "1f9b6-1f3ff.png");
png_name!(FOOT, "🦶", "foot", U_1F9B6, "1f9b6.png");
png_name!(TOOTH, "🦷", "tooth", U_1F9B7, "1f9b7.png");
png_name!(WOMAN_SUPERHERO_TONE1, "🦸🏻‍♀️", "", U_1F9B8_1F3FB_200D_2640_FE0F, "1f9b8-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_SUPERHERO_TONE1, "🦸🏻‍♂️", "", U_1F9B8_1F3FB_200D_2642_FE0F, "1f9b8-1f3fb-200d-2642-fe0f.png");
png_name!(SUPERHERO_TONE1, "🦸🏻", "", U_1F9B8_1F3FB, "1f9b8-1f3fb.png");
png_name!(WOMAN_SUPERHERO_TONE2, "🦸🏼‍♀️", "", U_1F9B8_1F3FC_200D_2640_FE0F, "1f9b8-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_SUPERHERO_TONE2, "🦸🏼‍♂️", "", U_1F9B8_1F3FC_200D_2642_FE0F, "1f9b8-1f3fc-200d-2642-fe0f.png");
png_name!(SUPERHERO_TONE2, "🦸🏼", "", U_1F9B8_1F3FC, "1f9b8-1f3fc.png");
png_name!(WOMAN_SUPERHERO_TONE3, "🦸🏽‍♀️", "", U_1F9B8_1F3FD_200D_2640_FE0F, "1f9b8-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_SUPERHERO_TONE3, "🦸🏽‍♂️", "", U_1F9B8_1F3FD_200D_2642_FE0F, "1f9b8-1f3fd-200d-2642-fe0f.png");
png_name!(SUPERHERO_TONE3, "🦸🏽", "", U_1F9B8_1F3FD, "1f9b8-1f3fd.png");
png_name!(WOMAN_SUPERHERO_TONE4, "🦸🏾‍♀️", "", U_1F9B8_1F3FE_200D_2640_FE0F, "1f9b8-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_SUPERHERO_TONE4, "🦸🏾‍♂️", "", U_1F9B8_1F3FE_200D_2642_FE0F, "1f9b8-1f3fe-200d-2642-fe0f.png");
png_name!(SUPERHERO_TONE4, "🦸🏾", "", U_1F9B8_1F3FE, "1f9b8-1f3fe.png");
png_name!(WOMAN_SUPERHERO_TONE5, "🦸🏿‍♀️", "", U_1F9B8_1F3FF_200D_2640_FE0F, "1f9b8-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_SUPERHERO_TONE5, "🦸🏿‍♂️", "", U_1F9B8_1F3FF_200D_2642_FE0F, "1f9b8-1f3ff-200d-2642-fe0f.png");
png_name!(SUPERHERO_TONE5, "🦸🏿", "", U_1F9B8_1F3FF, "1f9b8-1f3ff.png");
png_name!(WOMAN_SUPERHERO, "🦸‍♀️", "woman superhero", U_1F9B8_200D_2640_FE0F, "1f9b8-200d-2640-fe0f.png");
png_name!(MAN_SUPERHERO, "🦸‍♂️", "man superhero", U_1F9B8_200D_2642_FE0F, "1f9b8-200d-2642-fe0f.png");
png_name!(SUPERHERO, "🦸", "superhero", U_1F9B8, "1f9b8.png");
png_name!(WOMAN_SUPERVILLAIN_TONE1, "🦹🏻‍♀️", "", U_1F9B9_1F3FB_200D_2640_FE0F, "1f9b9-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_SUPERVILLAIN_TONE1, "🦹🏻‍♂️", "", U_1F9B9_1F3FB_200D_2642_FE0F, "1f9b9-1f3fb-200d-2642-fe0f.png");
png_name!(SUPERVILLAIN_TONE1, "🦹🏻", "", U_1F9B9_1F3FB, "1f9b9-1f3fb.png");
png_name!(WOMAN_SUPERVILLAIN_TONE2, "🦹🏼‍♀️", "", U_1F9B9_1F3FC_200D_2640_FE0F, "1f9b9-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_SUPERVILLAIN_TONE2, "🦹🏼‍♂️", "", U_1F9B9_1F3FC_200D_2642_FE0F, "1f9b9-1f3fc-200d-2642-fe0f.png");
png_name!(SUPERVILLAIN_TONE2, "🦹🏼", "", U_1F9B9_1F3FC, "1f9b9-1f3fc.png");
png_name!(WOMAN_SUPERVILLAIN_TONE3, "🦹🏽‍♀️", "", U_1F9B9_1F3FD_200D_2640_FE0F, "1f9b9-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_SUPERVILLAIN_TONE3, "🦹🏽‍♂️", "", U_1F9B9_1F3FD_200D_2642_FE0F, "1f9b9-1f3fd-200d-2642-fe0f.png");
png_name!(SUPERVILLAIN_TONE3, "🦹🏽", "", U_1F9B9_1F3FD, "1f9b9-1f3fd.png");
png_name!(WOMAN_SUPERVILLAIN_TONE4, "🦹🏾‍♀️", "", U_1F9B9_1F3FE_200D_2640_FE0F, "1f9b9-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_SUPERVILLAIN_TONE4, "🦹🏾‍♂️", "", U_1F9B9_1F3FE_200D_2642_FE0F, "1f9b9-1f3fe-200d-2642-fe0f.png");
png_name!(SUPERVILLAIN_TONE4, "🦹🏾", "", U_1F9B9_1F3FE, "1f9b9-1f3fe.png");
png_name!(WOMAN_SUPERVILLAIN_TONE5, "🦹🏿‍♀️", "", U_1F9B9_1F3FF_200D_2640_FE0F, "1f9b9-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_SUPERVILLAIN_TONE5, "🦹🏿‍♂️", "", U_1F9B9_1F3FF_200D_2642_FE0F, "1f9b9-1f3ff-200d-2642-fe0f.png");
png_name!(SUPERVILLAIN_TONE5, "🦹🏿", "", U_1F9B9_1F3FF, "1f9b9-1f3ff.png");
png_name!(WOMAN_SUPERVILLAIN, "🦹‍♀️", "woman supervillain", U_1F9B9_200D_2640_FE0F, "1f9b9-200d-2640-fe0f.png");
png_name!(MAN_SUPERVILLAIN, "🦹‍♂️", "man supervillain", U_1F9B9_200D_2642_FE0F, "1f9b9-200d-2642-fe0f.png");
png_name!(SUPERVILLAIN, "🦹", "supervillain", U_1F9B9, "1f9b9.png");
png_name!(SAFETY_VEST, "🦺", "safety vest", U_1F9BA, "1f9ba.png");
png_name!(EAR_WITH_HEARING_AID_TONE1, "🦻🏻", "", U_1F9BB_1F3FB, "1f9bb-1f3fb.png");
png_name!(HEARING_AID_TONE1, "🦻🏻", "", U_1F9BB_1F3FB, "1f9bb-1f3fb.png");
png_name!(EAR_WITH_HEARING_AID_TONE2, "🦻🏼", "", U_1F9BB_1F3FC, "1f9bb-1f3fc.png");
png_name!(HEARING_AID_TONE2, "🦻🏼", "", U_1F9BB_1F3FC, "1f9bb-1f3fc.png");
png_name!(EAR_WITH_HEARING_AID_TONE3, "🦻🏽", "", U_1F9BB_1F3FD, "1f9bb-1f3fd.png");
png_name!(HEARING_AID_TONE3, "🦻🏽", "", U_1F9BB_1F3FD, "1f9bb-1f3fd.png");
png_name!(EAR_WITH_HEARING_AID_TONE4, "🦻🏾", "", U_1F9BB_1F3FE, "1f9bb-1f3fe.png");
png_name!(HEARING_AID_TONE4, "🦻🏾", "", U_1F9BB_1F3FE, "1f9bb-1f3fe.png");
png_name!(EAR_WITH_HEARING_AID_TONE5, "🦻🏿", "", U_1F9BB_1F3FF, "1f9bb-1f3ff.png");
png_name!(HEARING_AID_TONE5, "🦻🏿", "", U_1F9BB_1F3FF, "1f9bb-1f3ff.png");
png_name!(EAR_WITH_HEARING_AID, "🦻", "ear with hearing aid", U_1F9BB, "1f9bb.png");
png_name!(HEARING_AID, "🦻", "ear with hearing aid", U_1F9BB, "1f9bb.png");
png_name!(MOTORIZED_WHEELCHAIR, "🦼", "motorized wheelchair", U_1F9BC, "1f9bc.png");
png_name!(MANUAL_WHEELCHAIR, "🦽", "manual wheelchair", U_1F9BD, "1f9bd.png");
png_name!(MECHANICAL_ARM, "🦾", "mechanical arm", U_1F9BE, "1f9be.png");
png_name!(MECHANICAL_LEG, "🦿", "mechanical leg", U_1F9BF, "1f9bf.png");
png_name!(CHEESE, "🧀", "cheese wedge", U_1F9C0, "1f9c0.png");
png_name!(CUPCAKE, "🧁", "cupcake", U_1F9C1, "1f9c1.png");
png_name!(SALT, "🧂", "salt", U_1F9C2, "1f9c2.png");
png_name!(BEVERAGE_BOX, "🧃", "beverage box", U_1F9C3, "1f9c3.png");
png_name!(JUICE_BOX, "🧃", "beverage box", U_1F9C3, "1f9c3.png");
png_name!(GARLIC, "🧄", "garlic", U_1F9C4, "1f9c4.png");
png_name!(ONION, "🧅", "onion", U_1F9C5, "1f9c5.png");
png_name!(FALAFEL, "🧆", "falafel", U_1F9C6, "1f9c6.png");
png_name!(WAFFLE, "🧇", "waffle", U_1F9C7, "1f9c7.png");
png_name!(BUTTER, "🧈", "butter", U_1F9C8, "1f9c8.png");
png_name!(MATE, "🧉", "mate", U_1F9C9, "1f9c9.png");
png_name!(ICE, "🧊", "ice", U_1F9CA, "1f9ca.png");
png_name!(ICE_CUBE, "🧊", "ice", U_1F9CA, "1f9ca.png");
png_name!(BOBA_DRINK, "🧋", "bubble tea", U_1F9CB, "1f9cb.png");
png_name!(BUBBLE_TEA, "🧋", "bubble tea", U_1F9CB, "1f9cb.png");
png_name!(TROLL, "🧌", "troll", U_1F9CC, "1f9cc.png");
png_name!(WOMAN_STANDING_TONE1, "🧍🏻‍♀️", "", U_1F9CD_1F3FB_200D_2640_FE0F, "1f9cd-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_STANDING_TONE1, "🧍🏻‍♂️", "", U_1F9CD_1F3FB_200D_2642_FE0F, "1f9cd-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_STANDING_TONE1, "🧍🏻", "", U_1F9CD_1F3FB, "1f9cd-1f3fb.png");
png_name!(STANDING_TONE1, "🧍🏻", "", U_1F9CD_1F3FB, "1f9cd-1f3fb.png");
png_name!(WOMAN_STANDING_TONE2, "🧍🏼‍♀️", "", U_1F9CD_1F3FC_200D_2640_FE0F, "1f9cd-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_STANDING_TONE2, "🧍🏼‍♂️", "", U_1F9CD_1F3FC_200D_2642_FE0F, "1f9cd-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_STANDING_TONE2, "🧍🏼", "", U_1F9CD_1F3FC, "1f9cd-1f3fc.png");
png_name!(STANDING_TONE2, "🧍🏼", "", U_1F9CD_1F3FC, "1f9cd-1f3fc.png");
png_name!(WOMAN_STANDING_TONE3, "🧍🏽‍♀️", "", U_1F9CD_1F3FD_200D_2640_FE0F, "1f9cd-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_STANDING_TONE3, "🧍🏽‍♂️", "", U_1F9CD_1F3FD_200D_2642_FE0F, "1f9cd-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_STANDING_TONE3, "🧍🏽", "", U_1F9CD_1F3FD, "1f9cd-1f3fd.png");
png_name!(STANDING_TONE3, "🧍🏽", "", U_1F9CD_1F3FD, "1f9cd-1f3fd.png");
png_name!(WOMAN_STANDING_TONE4, "🧍🏾‍♀️", "", U_1F9CD_1F3FE_200D_2640_FE0F, "1f9cd-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_STANDING_TONE4, "🧍🏾‍♂️", "", U_1F9CD_1F3FE_200D_2642_FE0F, "1f9cd-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_STANDING_TONE4, "🧍🏾", "", U_1F9CD_1F3FE, "1f9cd-1f3fe.png");
png_name!(STANDING_TONE4, "🧍🏾", "", U_1F9CD_1F3FE, "1f9cd-1f3fe.png");
png_name!(WOMAN_STANDING_TONE5, "🧍🏿‍♀️", "", U_1F9CD_1F3FF_200D_2640_FE0F, "1f9cd-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_STANDING_TONE5, "🧍🏿‍♂️", "", U_1F9CD_1F3FF_200D_2642_FE0F, "1f9cd-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_STANDING_TONE5, "🧍🏿", "", U_1F9CD_1F3FF, "1f9cd-1f3ff.png");
png_name!(STANDING_TONE5, "🧍🏿", "", U_1F9CD_1F3FF, "1f9cd-1f3ff.png");
png_name!(WOMAN_STANDING, "🧍‍♀️", "woman standing", U_1F9CD_200D_2640_FE0F, "1f9cd-200d-2640-fe0f.png");
png_name!(MAN_STANDING, "🧍‍♂️", "man standing", U_1F9CD_200D_2642_FE0F, "1f9cd-200d-2642-fe0f.png");
png_name!(PERSON_STANDING, "🧍", "person standing", U_1F9CD, "1f9cd.png");
png_name!(STANDING, "🧍", "person standing", U_1F9CD, "1f9cd.png");
png_name!(WOMAN_KNEELING_TONE1, "🧎🏻‍♀️", "", U_1F9CE_1F3FB_200D_2640_FE0F, "1f9ce-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_KNEELING_TONE1, "🧎🏻‍♂️", "", U_1F9CE_1F3FB_200D_2642_FE0F, "1f9ce-1f3fb-200d-2642-fe0f.png");
png_name!(KNEELING_TONE1, "🧎🏻", "", U_1F9CE_1F3FB, "1f9ce-1f3fb.png");
png_name!(PERSON_KNEELING_TONE1, "🧎🏻", "", U_1F9CE_1F3FB, "1f9ce-1f3fb.png");
png_name!(WOMAN_KNEELING_TONE2, "🧎🏼‍♀️", "", U_1F9CE_1F3FC_200D_2640_FE0F, "1f9ce-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_KNEELING_TONE2, "🧎🏼‍♂️", "", U_1F9CE_1F3FC_200D_2642_FE0F, "1f9ce-1f3fc-200d-2642-fe0f.png");
png_name!(KNEELING_TONE2, "🧎🏼", "", U_1F9CE_1F3FC, "1f9ce-1f3fc.png");
png_name!(PERSON_KNEELING_TONE2, "🧎🏼", "", U_1F9CE_1F3FC, "1f9ce-1f3fc.png");
png_name!(WOMAN_KNEELING_TONE3, "🧎🏽‍♀️", "", U_1F9CE_1F3FD_200D_2640_FE0F, "1f9ce-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_KNEELING_TONE3, "🧎🏽‍♂️", "", U_1F9CE_1F3FD_200D_2642_FE0F, "1f9ce-1f3fd-200d-2642-fe0f.png");
png_name!(KNEELING_TONE3, "🧎🏽", "", U_1F9CE_1F3FD, "1f9ce-1f3fd.png");
png_name!(PERSON_KNEELING_TONE3, "🧎🏽", "", U_1F9CE_1F3FD, "1f9ce-1f3fd.png");
png_name!(WOMAN_KNEELING_TONE4, "🧎🏾‍♀️", "", U_1F9CE_1F3FE_200D_2640_FE0F, "1f9ce-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_KNEELING_TONE4, "🧎🏾‍♂️", "", U_1F9CE_1F3FE_200D_2642_FE0F, "1f9ce-1f3fe-200d-2642-fe0f.png");
png_name!(KNEELING_TONE4, "🧎🏾", "", U_1F9CE_1F3FE, "1f9ce-1f3fe.png");
png_name!(PERSON_KNEELING_TONE4, "🧎🏾", "", U_1F9CE_1F3FE, "1f9ce-1f3fe.png");
png_name!(WOMAN_KNEELING_TONE5, "🧎🏿‍♀️", "", U_1F9CE_1F3FF_200D_2640_FE0F, "1f9ce-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_KNEELING_TONE5, "🧎🏿‍♂️", "", U_1F9CE_1F3FF_200D_2642_FE0F, "1f9ce-1f3ff-200d-2642-fe0f.png");
png_name!(KNEELING_TONE5, "🧎🏿", "", U_1F9CE_1F3FF, "1f9ce-1f3ff.png");
png_name!(PERSON_KNEELING_TONE5, "🧎🏿", "", U_1F9CE_1F3FF, "1f9ce-1f3ff.png");
png_name!(WOMAN_KNEELING, "🧎‍♀️", "woman kneeling", U_1F9CE_200D_2640_FE0F, "1f9ce-200d-2640-fe0f.png");
png_name!(MAN_KNEELING, "🧎‍♂️", "man kneeling", U_1F9CE_200D_2642_FE0F, "1f9ce-200d-2642-fe0f.png");
png_name!(KNEELING, "🧎", "person kneeling", U_1F9CE, "1f9ce.png");
png_name!(PERSON_KNEELING, "🧎", "person kneeling", U_1F9CE, "1f9ce.png");
png_name!(DEAF_WOMAN_TONE1, "🧏🏻‍♀️", "", U_1F9CF_1F3FB_200D_2640_FE0F, "1f9cf-1f3fb-200d-2640-fe0f.png");
png_name!(DEAF_MAN_TONE1, "🧏🏻‍♂️", "", U_1F9CF_1F3FB_200D_2642_FE0F, "1f9cf-1f3fb-200d-2642-fe0f.png");
png_name!(DEAF_PERSON_TONE1, "🧏🏻", "", U_1F9CF_1F3FB, "1f9cf-1f3fb.png");
png_name!(DEAF_WOMAN_TONE2, "🧏🏼‍♀️", "", U_1F9CF_1F3FC_200D_2640_FE0F, "1f9cf-1f3fc-200d-2640-fe0f.png");
png_name!(DEAF_MAN_TONE2, "🧏🏼‍♂️", "", U_1F9CF_1F3FC_200D_2642_FE0F, "1f9cf-1f3fc-200d-2642-fe0f.png");
png_name!(DEAF_PERSON_TONE2, "🧏🏼", "", U_1F9CF_1F3FC, "1f9cf-1f3fc.png");
png_name!(DEAF_WOMAN_TONE3, "🧏🏽‍♀️", "", U_1F9CF_1F3FD_200D_2640_FE0F, "1f9cf-1f3fd-200d-2640-fe0f.png");
png_name!(DEAF_MAN_TONE3, "🧏🏽‍♂️", "", U_1F9CF_1F3FD_200D_2642_FE0F, "1f9cf-1f3fd-200d-2642-fe0f.png");
png_name!(DEAF_PERSON_TONE3, "🧏🏽", "", U_1F9CF_1F3FD, "1f9cf-1f3fd.png");
png_name!(DEAF_WOMAN_TONE4, "🧏🏾‍♀️", "", U_1F9CF_1F3FE_200D_2640_FE0F, "1f9cf-1f3fe-200d-2640-fe0f.png");
png_name!(DEAF_MAN_TONE4, "🧏🏾‍♂️", "", U_1F9CF_1F3FE_200D_2642_FE0F, "1f9cf-1f3fe-200d-2642-fe0f.png");
png_name!(DEAF_PERSON_TONE4, "🧏🏾", "", U_1F9CF_1F3FE, "1f9cf-1f3fe.png");
png_name!(DEAF_WOMAN_TONE5, "🧏🏿‍♀️", "", U_1F9CF_1F3FF_200D_2640_FE0F, "1f9cf-1f3ff-200d-2640-fe0f.png");
png_name!(DEAF_MAN_TONE5, "🧏🏿‍♂️", "", U_1F9CF_1F3FF_200D_2642_FE0F, "1f9cf-1f3ff-200d-2642-fe0f.png");
png_name!(DEAF_PERSON_TONE5, "🧏🏿", "", U_1F9CF_1F3FF, "1f9cf-1f3ff.png");
png_name!(DEAF_WOMAN, "🧏‍♀️", "deaf woman", U_1F9CF_200D_2640_FE0F, "1f9cf-200d-2640-fe0f.png");
png_name!(DEAF_MAN, "🧏‍♂️", "deaf man", U_1F9CF_200D_2642_FE0F, "1f9cf-200d-2642-fe0f.png");
png_name!(DEAF_PERSON, "🧏", "deaf person", U_1F9CF, "1f9cf.png");
png_name!(FACE_WITH_MONOCLE, "🧐", "face with monocle", U_1F9D0, "1f9d0.png");
png_name!(FARMER_TONE1, "🧑🏻‍🌾", "", U_1F9D1_1F3FB_200D_1F33E, "1f9d1-1f3fb-200d-1f33e.png");
png_name!(COOK_TONE1, "🧑🏻‍🍳", "", U_1F9D1_1F3FB_200D_1F373, "1f9d1-1f3fb-200d-1f373.png");
png_name!(PERSON_FEEDING_BABY_TONE1, "🧑🏻‍🍼", "", U_1F9D1_1F3FB_200D_1F37C, "1f9d1-1f3fb-200d-1f37c.png");
png_name!(MX_CLAUS_TONE1, "🧑🏻‍🎄", "", U_1F9D1_1F3FB_200D_1F384, "1f9d1-1f3fb-200d-1f384.png");
png_name!(STUDENT_TONE1, "🧑🏻‍🎓", "", U_1F9D1_1F3FB_200D_1F393, "1f9d1-1f3fb-200d-1f393.png");
png_name!(SINGER_TONE1, "🧑🏻‍🎤", "", U_1F9D1_1F3FB_200D_1F3A4, "1f9d1-1f3fb-200d-1f3a4.png");
png_name!(ARTIST_TONE1, "🧑🏻‍🎨", "", U_1F9D1_1F3FB_200D_1F3A8, "1f9d1-1f3fb-200d-1f3a8.png");
png_name!(TEACHER_TONE1, "🧑🏻‍🏫", "", U_1F9D1_1F3FB_200D_1F3EB, "1f9d1-1f3fb-200d-1f3eb.png");
png_name!(FACTORY_WORKER_TONE1, "🧑🏻‍🏭", "", U_1F9D1_1F3FB_200D_1F3ED, "1f9d1-1f3fb-200d-1f3ed.png");
png_name!(TECHNOLOGIST_TONE1, "🧑🏻‍💻", "", U_1F9D1_1F3FB_200D_1F4BB, "1f9d1-1f3fb-200d-1f4bb.png");
png_name!(OFFICE_WORKER_TONE1, "🧑🏻‍💼", "", U_1F9D1_1F3FB_200D_1F4BC, "1f9d1-1f3fb-200d-1f4bc.png");
png_name!(MECHANIC_TONE1, "🧑🏻‍🔧", "", U_1F9D1_1F3FB_200D_1F527, "1f9d1-1f3fb-200d-1f527.png");
png_name!(SCIENTIST_TONE1, "🧑🏻‍🔬", "", U_1F9D1_1F3FB_200D_1F52C, "1f9d1-1f3fb-200d-1f52c.png");
png_name!(ASTRONAUT_TONE1, "🧑🏻‍🚀", "", U_1F9D1_1F3FB_200D_1F680, "1f9d1-1f3fb-200d-1f680.png");
png_name!(FIREFIGHTER_TONE1, "🧑🏻‍🚒", "", U_1F9D1_1F3FB_200D_1F692, "1f9d1-1f3fb-200d-1f692.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE1, "🧑🏻‍🤝‍🧑🏻", "", U_1F9D1_1F3FB_200D_1F91D_200D_1F9D1_1F3FB, "1f9d1-1f3fb-200d-1f91d-200d-1f9d1-1f3fb.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE1_2, "🧑🏻‍🤝‍🧑🏼", "", U_1F9D1_1F3FB_200D_1F91D_200D_1F9D1_1F3FC, "1f9d1-1f3fb-200d-1f91d-200d-1f9d1-1f3fc.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE1_3, "🧑🏻‍🤝‍🧑🏽", "", U_1F9D1_1F3FB_200D_1F91D_200D_1F9D1_1F3FD, "1f9d1-1f3fb-200d-1f91d-200d-1f9d1-1f3fd.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE1_4, "🧑🏻‍🤝‍🧑🏾", "", U_1F9D1_1F3FB_200D_1F91D_200D_1F9D1_1F3FE, "1f9d1-1f3fb-200d-1f91d-200d-1f9d1-1f3fe.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE1_5, "🧑🏻‍🤝‍🧑🏿", "", U_1F9D1_1F3FB_200D_1F91D_200D_1F9D1_1F3FF, "1f9d1-1f3fb-200d-1f91d-200d-1f9d1-1f3ff.png");
png_name!(PERSON_WITH_PROBING_CANE_TONE1, "🧑🏻‍🦯", "", U_1F9D1_1F3FB_200D_1F9AF, "1f9d1-1f3fb-200d-1f9af.png");
png_name!(PERSON_WITH_WHITE_CANE_TONE1, "🧑🏻‍🦯", "", U_1F9D1_1F3FB_200D_1F9AF, "1f9d1-1f3fb-200d-1f9af.png");
png_name!(RED_HAIRED_TONE1, "🧑🏻‍🦰", "", U_1F9D1_1F3FB_200D_1F9B0, "1f9d1-1f3fb-200d-1f9b0.png");
png_name!(CURLY_HAIRED_TONE1, "🧑🏻‍🦱", "", U_1F9D1_1F3FB_200D_1F9B1, "1f9d1-1f3fb-200d-1f9b1.png");
png_name!(BALD_TONE1, "🧑🏻‍🦲", "", U_1F9D1_1F3FB_200D_1F9B2, "1f9d1-1f3fb-200d-1f9b2.png");
png_name!(WHITE_HAIRED_TONE1, "🧑🏻‍🦳", "", U_1F9D1_1F3FB_200D_1F9B3, "1f9d1-1f3fb-200d-1f9b3.png");
png_name!(PERSON_IN_MOTORIZED_WHEELCHAIR_TONE1, "🧑🏻‍🦼", "", U_1F9D1_1F3FB_200D_1F9BC, "1f9d1-1f3fb-200d-1f9bc.png");
png_name!(PERSON_IN_MANUAL_WHEELCHAIR_TONE1, "🧑🏻‍🦽", "", U_1F9D1_1F3FB_200D_1F9BD, "1f9d1-1f3fb-200d-1f9bd.png");
png_name!(HEALTH_WORKER_TONE1, "🧑🏻‍⚕️", "", U_1F9D1_1F3FB_200D_2695_FE0F, "1f9d1-1f3fb-200d-2695-fe0f.png");
png_name!(JUDGE_TONE1, "🧑🏻‍⚖️", "", U_1F9D1_1F3FB_200D_2696_FE0F, "1f9d1-1f3fb-200d-2696-fe0f.png");
png_name!(PILOT_TONE1, "🧑🏻‍✈️", "", U_1F9D1_1F3FB_200D_2708_FE0F, "1f9d1-1f3fb-200d-2708-fe0f.png");
png_name!(COUPLE_KISS_TONE1_2, "🧑🏻‍❤️‍💋‍🧑🏼", "", U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC, "1f9d1-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fc.png");
png_name!(COUPLEKISS_TONE1_2, "🧑🏻‍❤️‍💋‍🧑🏼", "", U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC, "1f9d1-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fc.png");
png_name!(COUPLE_KISS_TONE1_3, "🧑🏻‍❤️‍💋‍🧑🏽", "", U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD, "1f9d1-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fd.png");
png_name!(COUPLEKISS_TONE1_3, "🧑🏻‍❤️‍💋‍🧑🏽", "", U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD, "1f9d1-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fd.png");
png_name!(COUPLE_KISS_TONE1_4, "🧑🏻‍❤️‍💋‍🧑🏾", "", U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE, "1f9d1-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fe.png");
png_name!(COUPLEKISS_TONE1_4, "🧑🏻‍❤️‍💋‍🧑🏾", "", U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE, "1f9d1-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fe.png");
png_name!(COUPLE_KISS_TONE1_5, "🧑🏻‍❤️‍💋‍🧑🏿", "", U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF, "1f9d1-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3ff.png");
png_name!(COUPLEKISS_TONE1_5, "🧑🏻‍❤️‍💋‍🧑🏿", "", U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF, "1f9d1-1f3fb-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3ff.png");
png_name!(COUPLE_WITH_HEART_TONE1_2, "🧑🏻‍❤️‍🧑🏼", "", U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F9D1_1F3FC, "1f9d1-1f3fb-200d-2764-fe0f-200d-1f9d1-1f3fc.png");
png_name!(COUPLE_WITH_HEART_TONE1_3, "🧑🏻‍❤️‍🧑🏽", "", U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F9D1_1F3FD, "1f9d1-1f3fb-200d-2764-fe0f-200d-1f9d1-1f3fd.png");
png_name!(COUPLE_WITH_HEART_TONE1_4, "🧑🏻‍❤️‍🧑🏾", "", U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F9D1_1F3FE, "1f9d1-1f3fb-200d-2764-fe0f-200d-1f9d1-1f3fe.png");
png_name!(COUPLE_WITH_HEART_TONE1_5, "🧑🏻‍❤️‍🧑🏿", "", U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F9D1_1F3FF, "1f9d1-1f3fb-200d-2764-fe0f-200d-1f9d1-1f3ff.png");
png_name!(ADULT_TONE1, "🧑🏻", "", U_1F9D1_1F3FB, "1f9d1-1f3fb.png");
png_name!(FARMER_TONE2, "🧑🏼‍🌾", "", U_1F9D1_1F3FC_200D_1F33E, "1f9d1-1f3fc-200d-1f33e.png");
png_name!(COOK_TONE2, "🧑🏼‍🍳", "", U_1F9D1_1F3FC_200D_1F373, "1f9d1-1f3fc-200d-1f373.png");
png_name!(PERSON_FEEDING_BABY_TONE2, "🧑🏼‍🍼", "", U_1F9D1_1F3FC_200D_1F37C, "1f9d1-1f3fc-200d-1f37c.png");
png_name!(MX_CLAUS_TONE2, "🧑🏼‍🎄", "", U_1F9D1_1F3FC_200D_1F384, "1f9d1-1f3fc-200d-1f384.png");
png_name!(STUDENT_TONE2, "🧑🏼‍🎓", "", U_1F9D1_1F3FC_200D_1F393, "1f9d1-1f3fc-200d-1f393.png");
png_name!(SINGER_TONE2, "🧑🏼‍🎤", "", U_1F9D1_1F3FC_200D_1F3A4, "1f9d1-1f3fc-200d-1f3a4.png");
png_name!(ARTIST_TONE2, "🧑🏼‍🎨", "", U_1F9D1_1F3FC_200D_1F3A8, "1f9d1-1f3fc-200d-1f3a8.png");
png_name!(TEACHER_TONE2, "🧑🏼‍🏫", "", U_1F9D1_1F3FC_200D_1F3EB, "1f9d1-1f3fc-200d-1f3eb.png");
png_name!(FACTORY_WORKER_TONE2, "🧑🏼‍🏭", "", U_1F9D1_1F3FC_200D_1F3ED, "1f9d1-1f3fc-200d-1f3ed.png");
png_name!(TECHNOLOGIST_TONE2, "🧑🏼‍💻", "", U_1F9D1_1F3FC_200D_1F4BB, "1f9d1-1f3fc-200d-1f4bb.png");
png_name!(OFFICE_WORKER_TONE2, "🧑🏼‍💼", "", U_1F9D1_1F3FC_200D_1F4BC, "1f9d1-1f3fc-200d-1f4bc.png");
png_name!(MECHANIC_TONE2, "🧑🏼‍🔧", "", U_1F9D1_1F3FC_200D_1F527, "1f9d1-1f3fc-200d-1f527.png");
png_name!(SCIENTIST_TONE2, "🧑🏼‍🔬", "", U_1F9D1_1F3FC_200D_1F52C, "1f9d1-1f3fc-200d-1f52c.png");
png_name!(ASTRONAUT_TONE2, "🧑🏼‍🚀", "", U_1F9D1_1F3FC_200D_1F680, "1f9d1-1f3fc-200d-1f680.png");
png_name!(FIREFIGHTER_TONE2, "🧑🏼‍🚒", "", U_1F9D1_1F3FC_200D_1F692, "1f9d1-1f3fc-200d-1f692.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE2_1, "🧑🏼‍🤝‍🧑🏻", "", U_1F9D1_1F3FC_200D_1F91D_200D_1F9D1_1F3FB, "1f9d1-1f3fc-200d-1f91d-200d-1f9d1-1f3fb.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE2, "🧑🏼‍🤝‍🧑🏼", "", U_1F9D1_1F3FC_200D_1F91D_200D_1F9D1_1F3FC, "1f9d1-1f3fc-200d-1f91d-200d-1f9d1-1f3fc.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE2_3, "🧑🏼‍🤝‍🧑🏽", "", U_1F9D1_1F3FC_200D_1F91D_200D_1F9D1_1F3FD, "1f9d1-1f3fc-200d-1f91d-200d-1f9d1-1f3fd.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE2_4, "🧑🏼‍🤝‍🧑🏾", "", U_1F9D1_1F3FC_200D_1F91D_200D_1F9D1_1F3FE, "1f9d1-1f3fc-200d-1f91d-200d-1f9d1-1f3fe.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE2_5, "🧑🏼‍🤝‍🧑🏿", "", U_1F9D1_1F3FC_200D_1F91D_200D_1F9D1_1F3FF, "1f9d1-1f3fc-200d-1f91d-200d-1f9d1-1f3ff.png");
png_name!(PERSON_WITH_PROBING_CANE_TONE2, "🧑🏼‍🦯", "", U_1F9D1_1F3FC_200D_1F9AF, "1f9d1-1f3fc-200d-1f9af.png");
png_name!(PERSON_WITH_WHITE_CANE_TONE2, "🧑🏼‍🦯", "", U_1F9D1_1F3FC_200D_1F9AF, "1f9d1-1f3fc-200d-1f9af.png");
png_name!(RED_HAIRED_TONE2, "🧑🏼‍🦰", "", U_1F9D1_1F3FC_200D_1F9B0, "1f9d1-1f3fc-200d-1f9b0.png");
png_name!(CURLY_HAIRED_TONE2, "🧑🏼‍🦱", "", U_1F9D1_1F3FC_200D_1F9B1, "1f9d1-1f3fc-200d-1f9b1.png");
png_name!(BALD_TONE2, "🧑🏼‍🦲", "", U_1F9D1_1F3FC_200D_1F9B2, "1f9d1-1f3fc-200d-1f9b2.png");
png_name!(WHITE_HAIRED_TONE2, "🧑🏼‍🦳", "", U_1F9D1_1F3FC_200D_1F9B3, "1f9d1-1f3fc-200d-1f9b3.png");
png_name!(PERSON_IN_MOTORIZED_WHEELCHAIR_TONE2, "🧑🏼‍🦼", "", U_1F9D1_1F3FC_200D_1F9BC, "1f9d1-1f3fc-200d-1f9bc.png");
png_name!(PERSON_IN_MANUAL_WHEELCHAIR_TONE2, "🧑🏼‍🦽", "", U_1F9D1_1F3FC_200D_1F9BD, "1f9d1-1f3fc-200d-1f9bd.png");
png_name!(HEALTH_WORKER_TONE2, "🧑🏼‍⚕️", "", U_1F9D1_1F3FC_200D_2695_FE0F, "1f9d1-1f3fc-200d-2695-fe0f.png");
png_name!(JUDGE_TONE2, "🧑🏼‍⚖️", "", U_1F9D1_1F3FC_200D_2696_FE0F, "1f9d1-1f3fc-200d-2696-fe0f.png");
png_name!(PILOT_TONE2, "🧑🏼‍✈️", "", U_1F9D1_1F3FC_200D_2708_FE0F, "1f9d1-1f3fc-200d-2708-fe0f.png");
png_name!(COUPLE_KISS_TONE2_1, "🧑🏼‍❤️‍💋‍🧑🏻", "", U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB, "1f9d1-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fb.png");
png_name!(COUPLEKISS_TONE2_1, "🧑🏼‍❤️‍💋‍🧑🏻", "", U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB, "1f9d1-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fb.png");
png_name!(COUPLE_KISS_TONE2_3, "🧑🏼‍❤️‍💋‍🧑🏽", "", U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD, "1f9d1-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fd.png");
png_name!(COUPLEKISS_TONE2_3, "🧑🏼‍❤️‍💋‍🧑🏽", "", U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD, "1f9d1-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fd.png");
png_name!(COUPLE_KISS_TONE2_4, "🧑🏼‍❤️‍💋‍🧑🏾", "", U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE, "1f9d1-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fe.png");
png_name!(COUPLEKISS_TONE2_4, "🧑🏼‍❤️‍💋‍🧑🏾", "", U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE, "1f9d1-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fe.png");
png_name!(COUPLE_KISS_TONE2_5, "🧑🏼‍❤️‍💋‍🧑🏿", "", U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF, "1f9d1-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3ff.png");
png_name!(COUPLEKISS_TONE2_5, "🧑🏼‍❤️‍💋‍🧑🏿", "", U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF, "1f9d1-1f3fc-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3ff.png");
png_name!(COUPLE_WITH_HEART_TONE2_1, "🧑🏼‍❤️‍🧑🏻", "", U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F9D1_1F3FB, "1f9d1-1f3fc-200d-2764-fe0f-200d-1f9d1-1f3fb.png");
png_name!(COUPLE_WITH_HEART_TONE2_3, "🧑🏼‍❤️‍🧑🏽", "", U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F9D1_1F3FD, "1f9d1-1f3fc-200d-2764-fe0f-200d-1f9d1-1f3fd.png");
png_name!(COUPLE_WITH_HEART_TONE2_4, "🧑🏼‍❤️‍🧑🏾", "", U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F9D1_1F3FE, "1f9d1-1f3fc-200d-2764-fe0f-200d-1f9d1-1f3fe.png");
png_name!(COUPLE_WITH_HEART_TONE2_5, "🧑🏼‍❤️‍🧑🏿", "", U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F9D1_1F3FF, "1f9d1-1f3fc-200d-2764-fe0f-200d-1f9d1-1f3ff.png");
png_name!(ADULT_TONE2, "🧑🏼", "", U_1F9D1_1F3FC, "1f9d1-1f3fc.png");
png_name!(FARMER_TONE3, "🧑🏽‍🌾", "", U_1F9D1_1F3FD_200D_1F33E, "1f9d1-1f3fd-200d-1f33e.png");
png_name!(COOK_TONE3, "🧑🏽‍🍳", "", U_1F9D1_1F3FD_200D_1F373, "1f9d1-1f3fd-200d-1f373.png");
png_name!(PERSON_FEEDING_BABY_TONE3, "🧑🏽‍🍼", "", U_1F9D1_1F3FD_200D_1F37C, "1f9d1-1f3fd-200d-1f37c.png");
png_name!(MX_CLAUS_TONE3, "🧑🏽‍🎄", "", U_1F9D1_1F3FD_200D_1F384, "1f9d1-1f3fd-200d-1f384.png");
png_name!(STUDENT_TONE3, "🧑🏽‍🎓", "", U_1F9D1_1F3FD_200D_1F393, "1f9d1-1f3fd-200d-1f393.png");
png_name!(SINGER_TONE3, "🧑🏽‍🎤", "", U_1F9D1_1F3FD_200D_1F3A4, "1f9d1-1f3fd-200d-1f3a4.png");
png_name!(ARTIST_TONE3, "🧑🏽‍🎨", "", U_1F9D1_1F3FD_200D_1F3A8, "1f9d1-1f3fd-200d-1f3a8.png");
png_name!(TEACHER_TONE3, "🧑🏽‍🏫", "", U_1F9D1_1F3FD_200D_1F3EB, "1f9d1-1f3fd-200d-1f3eb.png");
png_name!(FACTORY_WORKER_TONE3, "🧑🏽‍🏭", "", U_1F9D1_1F3FD_200D_1F3ED, "1f9d1-1f3fd-200d-1f3ed.png");
png_name!(TECHNOLOGIST_TONE3, "🧑🏽‍💻", "", U_1F9D1_1F3FD_200D_1F4BB, "1f9d1-1f3fd-200d-1f4bb.png");
png_name!(OFFICE_WORKER_TONE3, "🧑🏽‍💼", "", U_1F9D1_1F3FD_200D_1F4BC, "1f9d1-1f3fd-200d-1f4bc.png");
png_name!(MECHANIC_TONE3, "🧑🏽‍🔧", "", U_1F9D1_1F3FD_200D_1F527, "1f9d1-1f3fd-200d-1f527.png");
png_name!(SCIENTIST_TONE3, "🧑🏽‍🔬", "", U_1F9D1_1F3FD_200D_1F52C, "1f9d1-1f3fd-200d-1f52c.png");
png_name!(ASTRONAUT_TONE3, "🧑🏽‍🚀", "", U_1F9D1_1F3FD_200D_1F680, "1f9d1-1f3fd-200d-1f680.png");
png_name!(FIREFIGHTER_TONE3, "🧑🏽‍🚒", "", U_1F9D1_1F3FD_200D_1F692, "1f9d1-1f3fd-200d-1f692.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE3_1, "🧑🏽‍🤝‍🧑🏻", "", U_1F9D1_1F3FD_200D_1F91D_200D_1F9D1_1F3FB, "1f9d1-1f3fd-200d-1f91d-200d-1f9d1-1f3fb.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE3_2, "🧑🏽‍🤝‍🧑🏼", "", U_1F9D1_1F3FD_200D_1F91D_200D_1F9D1_1F3FC, "1f9d1-1f3fd-200d-1f91d-200d-1f9d1-1f3fc.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE3, "🧑🏽‍🤝‍🧑🏽", "", U_1F9D1_1F3FD_200D_1F91D_200D_1F9D1_1F3FD, "1f9d1-1f3fd-200d-1f91d-200d-1f9d1-1f3fd.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE3_4, "🧑🏽‍🤝‍🧑🏾", "", U_1F9D1_1F3FD_200D_1F91D_200D_1F9D1_1F3FE, "1f9d1-1f3fd-200d-1f91d-200d-1f9d1-1f3fe.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE3_5, "🧑🏽‍🤝‍🧑🏿", "", U_1F9D1_1F3FD_200D_1F91D_200D_1F9D1_1F3FF, "1f9d1-1f3fd-200d-1f91d-200d-1f9d1-1f3ff.png");
png_name!(PERSON_WITH_PROBING_CANE_TONE3, "🧑🏽‍🦯", "", U_1F9D1_1F3FD_200D_1F9AF, "1f9d1-1f3fd-200d-1f9af.png");
png_name!(PERSON_WITH_WHITE_CANE_TONE3, "🧑🏽‍🦯", "", U_1F9D1_1F3FD_200D_1F9AF, "1f9d1-1f3fd-200d-1f9af.png");
png_name!(RED_HAIRED_TONE3, "🧑🏽‍🦰", "", U_1F9D1_1F3FD_200D_1F9B0, "1f9d1-1f3fd-200d-1f9b0.png");
png_name!(CURLY_HAIRED_TONE3, "🧑🏽‍🦱", "", U_1F9D1_1F3FD_200D_1F9B1, "1f9d1-1f3fd-200d-1f9b1.png");
png_name!(BALD_TONE3, "🧑🏽‍🦲", "", U_1F9D1_1F3FD_200D_1F9B2, "1f9d1-1f3fd-200d-1f9b2.png");
png_name!(WHITE_HAIRED_TONE3, "🧑🏽‍🦳", "", U_1F9D1_1F3FD_200D_1F9B3, "1f9d1-1f3fd-200d-1f9b3.png");
png_name!(PERSON_IN_MOTORIZED_WHEELCHAIR_TONE3, "🧑🏽‍🦼", "", U_1F9D1_1F3FD_200D_1F9BC, "1f9d1-1f3fd-200d-1f9bc.png");
png_name!(PERSON_IN_MANUAL_WHEELCHAIR_TONE3, "🧑🏽‍🦽", "", U_1F9D1_1F3FD_200D_1F9BD, "1f9d1-1f3fd-200d-1f9bd.png");
png_name!(HEALTH_WORKER_TONE3, "🧑🏽‍⚕️", "", U_1F9D1_1F3FD_200D_2695_FE0F, "1f9d1-1f3fd-200d-2695-fe0f.png");
png_name!(JUDGE_TONE3, "🧑🏽‍⚖️", "", U_1F9D1_1F3FD_200D_2696_FE0F, "1f9d1-1f3fd-200d-2696-fe0f.png");
png_name!(PILOT_TONE3, "🧑🏽‍✈️", "", U_1F9D1_1F3FD_200D_2708_FE0F, "1f9d1-1f3fd-200d-2708-fe0f.png");
png_name!(COUPLE_KISS_TONE3_1, "🧑🏽‍❤️‍💋‍🧑🏻", "", U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB, "1f9d1-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fb.png");
png_name!(COUPLEKISS_TONE3_1, "🧑🏽‍❤️‍💋‍🧑🏻", "", U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB, "1f9d1-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fb.png");
png_name!(COUPLE_KISS_TONE3_2, "🧑🏽‍❤️‍💋‍🧑🏼", "", U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC, "1f9d1-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fc.png");
png_name!(COUPLEKISS_TONE3_2, "🧑🏽‍❤️‍💋‍🧑🏼", "", U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC, "1f9d1-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fc.png");
png_name!(COUPLE_KISS_TONE3_4, "🧑🏽‍❤️‍💋‍🧑🏾", "", U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE, "1f9d1-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fe.png");
png_name!(COUPLEKISS_TONE3_4, "🧑🏽‍❤️‍💋‍🧑🏾", "", U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE, "1f9d1-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fe.png");
png_name!(COUPLE_KISS_TONE3_5, "🧑🏽‍❤️‍💋‍🧑🏿", "", U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF, "1f9d1-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3ff.png");
png_name!(COUPLEKISS_TONE3_5, "🧑🏽‍❤️‍💋‍🧑🏿", "", U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF, "1f9d1-1f3fd-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3ff.png");
png_name!(COUPLE_WITH_HEART_TONE3_1, "🧑🏽‍❤️‍🧑🏻", "", U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F9D1_1F3FB, "1f9d1-1f3fd-200d-2764-fe0f-200d-1f9d1-1f3fb.png");
png_name!(COUPLE_WITH_HEART_TONE3_2, "🧑🏽‍❤️‍🧑🏼", "", U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F9D1_1F3FC, "1f9d1-1f3fd-200d-2764-fe0f-200d-1f9d1-1f3fc.png");
png_name!(COUPLE_WITH_HEART_TONE3_4, "🧑🏽‍❤️‍🧑🏾", "", U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F9D1_1F3FE, "1f9d1-1f3fd-200d-2764-fe0f-200d-1f9d1-1f3fe.png");
png_name!(COUPLE_WITH_HEART_TONE3_5, "🧑🏽‍❤️‍🧑🏿", "", U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F9D1_1F3FF, "1f9d1-1f3fd-200d-2764-fe0f-200d-1f9d1-1f3ff.png");
png_name!(ADULT_TONE3, "🧑🏽", "", U_1F9D1_1F3FD, "1f9d1-1f3fd.png");
png_name!(FARMER_TONE4, "🧑🏾‍🌾", "", U_1F9D1_1F3FE_200D_1F33E, "1f9d1-1f3fe-200d-1f33e.png");
png_name!(COOK_TONE4, "🧑🏾‍🍳", "", U_1F9D1_1F3FE_200D_1F373, "1f9d1-1f3fe-200d-1f373.png");
png_name!(PERSON_FEEDING_BABY_TONE4, "🧑🏾‍🍼", "", U_1F9D1_1F3FE_200D_1F37C, "1f9d1-1f3fe-200d-1f37c.png");
png_name!(MX_CLAUS_TONE4, "🧑🏾‍🎄", "", U_1F9D1_1F3FE_200D_1F384, "1f9d1-1f3fe-200d-1f384.png");
png_name!(STUDENT_TONE4, "🧑🏾‍🎓", "", U_1F9D1_1F3FE_200D_1F393, "1f9d1-1f3fe-200d-1f393.png");
png_name!(SINGER_TONE4, "🧑🏾‍🎤", "", U_1F9D1_1F3FE_200D_1F3A4, "1f9d1-1f3fe-200d-1f3a4.png");
png_name!(ARTIST_TONE4, "🧑🏾‍🎨", "", U_1F9D1_1F3FE_200D_1F3A8, "1f9d1-1f3fe-200d-1f3a8.png");
png_name!(TEACHER_TONE4, "🧑🏾‍🏫", "", U_1F9D1_1F3FE_200D_1F3EB, "1f9d1-1f3fe-200d-1f3eb.png");
png_name!(FACTORY_WORKER_TONE4, "🧑🏾‍🏭", "", U_1F9D1_1F3FE_200D_1F3ED, "1f9d1-1f3fe-200d-1f3ed.png");
png_name!(TECHNOLOGIST_TONE4, "🧑🏾‍💻", "", U_1F9D1_1F3FE_200D_1F4BB, "1f9d1-1f3fe-200d-1f4bb.png");
png_name!(OFFICE_WORKER_TONE4, "🧑🏾‍💼", "", U_1F9D1_1F3FE_200D_1F4BC, "1f9d1-1f3fe-200d-1f4bc.png");
png_name!(MECHANIC_TONE4, "🧑🏾‍🔧", "", U_1F9D1_1F3FE_200D_1F527, "1f9d1-1f3fe-200d-1f527.png");
png_name!(SCIENTIST_TONE4, "🧑🏾‍🔬", "", U_1F9D1_1F3FE_200D_1F52C, "1f9d1-1f3fe-200d-1f52c.png");
png_name!(ASTRONAUT_TONE4, "🧑🏾‍🚀", "", U_1F9D1_1F3FE_200D_1F680, "1f9d1-1f3fe-200d-1f680.png");
png_name!(FIREFIGHTER_TONE4, "🧑🏾‍🚒", "", U_1F9D1_1F3FE_200D_1F692, "1f9d1-1f3fe-200d-1f692.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE4_1, "🧑🏾‍🤝‍🧑🏻", "", U_1F9D1_1F3FE_200D_1F91D_200D_1F9D1_1F3FB, "1f9d1-1f3fe-200d-1f91d-200d-1f9d1-1f3fb.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE4_2, "🧑🏾‍🤝‍🧑🏼", "", U_1F9D1_1F3FE_200D_1F91D_200D_1F9D1_1F3FC, "1f9d1-1f3fe-200d-1f91d-200d-1f9d1-1f3fc.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE4_3, "🧑🏾‍🤝‍🧑🏽", "", U_1F9D1_1F3FE_200D_1F91D_200D_1F9D1_1F3FD, "1f9d1-1f3fe-200d-1f91d-200d-1f9d1-1f3fd.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE4, "🧑🏾‍🤝‍🧑🏾", "", U_1F9D1_1F3FE_200D_1F91D_200D_1F9D1_1F3FE, "1f9d1-1f3fe-200d-1f91d-200d-1f9d1-1f3fe.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE4_5, "🧑🏾‍🤝‍🧑🏿", "", U_1F9D1_1F3FE_200D_1F91D_200D_1F9D1_1F3FF, "1f9d1-1f3fe-200d-1f91d-200d-1f9d1-1f3ff.png");
png_name!(PERSON_WITH_PROBING_CANE_TONE4, "🧑🏾‍🦯", "", U_1F9D1_1F3FE_200D_1F9AF, "1f9d1-1f3fe-200d-1f9af.png");
png_name!(PERSON_WITH_WHITE_CANE_TONE4, "🧑🏾‍🦯", "", U_1F9D1_1F3FE_200D_1F9AF, "1f9d1-1f3fe-200d-1f9af.png");
png_name!(RED_HAIRED_TONE4, "🧑🏾‍🦰", "", U_1F9D1_1F3FE_200D_1F9B0, "1f9d1-1f3fe-200d-1f9b0.png");
png_name!(CURLY_HAIRED_TONE4, "🧑🏾‍🦱", "", U_1F9D1_1F3FE_200D_1F9B1, "1f9d1-1f3fe-200d-1f9b1.png");
png_name!(BALD_TONE4, "🧑🏾‍🦲", "", U_1F9D1_1F3FE_200D_1F9B2, "1f9d1-1f3fe-200d-1f9b2.png");
png_name!(WHITE_HAIRED_TONE4, "🧑🏾‍🦳", "", U_1F9D1_1F3FE_200D_1F9B3, "1f9d1-1f3fe-200d-1f9b3.png");
png_name!(PERSON_IN_MOTORIZED_WHEELCHAIR_TONE4, "🧑🏾‍🦼", "", U_1F9D1_1F3FE_200D_1F9BC, "1f9d1-1f3fe-200d-1f9bc.png");
png_name!(PERSON_IN_MANUAL_WHEELCHAIR_TONE4, "🧑🏾‍🦽", "", U_1F9D1_1F3FE_200D_1F9BD, "1f9d1-1f3fe-200d-1f9bd.png");
png_name!(HEALTH_WORKER_TONE4, "🧑🏾‍⚕️", "", U_1F9D1_1F3FE_200D_2695_FE0F, "1f9d1-1f3fe-200d-2695-fe0f.png");
png_name!(JUDGE_TONE4, "🧑🏾‍⚖️", "", U_1F9D1_1F3FE_200D_2696_FE0F, "1f9d1-1f3fe-200d-2696-fe0f.png");
png_name!(PILOT_TONE4, "🧑🏾‍✈️", "", U_1F9D1_1F3FE_200D_2708_FE0F, "1f9d1-1f3fe-200d-2708-fe0f.png");
png_name!(COUPLE_KISS_TONE4_1, "🧑🏾‍❤️‍💋‍🧑🏻", "", U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB, "1f9d1-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fb.png");
png_name!(COUPLEKISS_TONE4_1, "🧑🏾‍❤️‍💋‍🧑🏻", "", U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB, "1f9d1-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fb.png");
png_name!(COUPLE_KISS_TONE4_2, "🧑🏾‍❤️‍💋‍🧑🏼", "", U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC, "1f9d1-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fc.png");
png_name!(COUPLEKISS_TONE4_2, "🧑🏾‍❤️‍💋‍🧑🏼", "", U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC, "1f9d1-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fc.png");
png_name!(COUPLE_KISS_TONE4_3, "🧑🏾‍❤️‍💋‍🧑🏽", "", U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD, "1f9d1-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fd.png");
png_name!(COUPLEKISS_TONE4_3, "🧑🏾‍❤️‍💋‍🧑🏽", "", U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD, "1f9d1-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fd.png");
png_name!(COUPLE_KISS_TONE4_5, "🧑🏾‍❤️‍💋‍🧑🏿", "", U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF, "1f9d1-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3ff.png");
png_name!(COUPLEKISS_TONE4_5, "🧑🏾‍❤️‍💋‍🧑🏿", "", U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF, "1f9d1-1f3fe-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3ff.png");
png_name!(COUPLE_WITH_HEART_TONE4_1, "🧑🏾‍❤️‍🧑🏻", "", U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F9D1_1F3FB, "1f9d1-1f3fe-200d-2764-fe0f-200d-1f9d1-1f3fb.png");
png_name!(COUPLE_WITH_HEART_TONE4_2, "🧑🏾‍❤️‍🧑🏼", "", U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F9D1_1F3FC, "1f9d1-1f3fe-200d-2764-fe0f-200d-1f9d1-1f3fc.png");
png_name!(COUPLE_WITH_HEART_TONE4_3, "🧑🏾‍❤️‍🧑🏽", "", U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F9D1_1F3FD, "1f9d1-1f3fe-200d-2764-fe0f-200d-1f9d1-1f3fd.png");
png_name!(COUPLE_WITH_HEART_TONE4_5, "🧑🏾‍❤️‍🧑🏿", "", U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F9D1_1F3FF, "1f9d1-1f3fe-200d-2764-fe0f-200d-1f9d1-1f3ff.png");
png_name!(ADULT_TONE4, "🧑🏾", "", U_1F9D1_1F3FE, "1f9d1-1f3fe.png");
png_name!(FARMER_TONE5, "🧑🏿‍🌾", "", U_1F9D1_1F3FF_200D_1F33E, "1f9d1-1f3ff-200d-1f33e.png");
png_name!(COOK_TONE5, "🧑🏿‍🍳", "", U_1F9D1_1F3FF_200D_1F373, "1f9d1-1f3ff-200d-1f373.png");
png_name!(PERSON_FEEDING_BABY_TONE5, "🧑🏿‍🍼", "", U_1F9D1_1F3FF_200D_1F37C, "1f9d1-1f3ff-200d-1f37c.png");
png_name!(MX_CLAUS_TONE5, "🧑🏿‍🎄", "", U_1F9D1_1F3FF_200D_1F384, "1f9d1-1f3ff-200d-1f384.png");
png_name!(STUDENT_TONE5, "🧑🏿‍🎓", "", U_1F9D1_1F3FF_200D_1F393, "1f9d1-1f3ff-200d-1f393.png");
png_name!(SINGER_TONE5, "🧑🏿‍🎤", "", U_1F9D1_1F3FF_200D_1F3A4, "1f9d1-1f3ff-200d-1f3a4.png");
png_name!(ARTIST_TONE5, "🧑🏿‍🎨", "", U_1F9D1_1F3FF_200D_1F3A8, "1f9d1-1f3ff-200d-1f3a8.png");
png_name!(TEACHER_TONE5, "🧑🏿‍🏫", "", U_1F9D1_1F3FF_200D_1F3EB, "1f9d1-1f3ff-200d-1f3eb.png");
png_name!(FACTORY_WORKER_TONE5, "🧑🏿‍🏭", "", U_1F9D1_1F3FF_200D_1F3ED, "1f9d1-1f3ff-200d-1f3ed.png");
png_name!(TECHNOLOGIST_TONE5, "🧑🏿‍💻", "", U_1F9D1_1F3FF_200D_1F4BB, "1f9d1-1f3ff-200d-1f4bb.png");
png_name!(OFFICE_WORKER_TONE5, "🧑🏿‍💼", "", U_1F9D1_1F3FF_200D_1F4BC, "1f9d1-1f3ff-200d-1f4bc.png");
png_name!(MECHANIC_TONE5, "🧑🏿‍🔧", "", U_1F9D1_1F3FF_200D_1F527, "1f9d1-1f3ff-200d-1f527.png");
png_name!(SCIENTIST_TONE5, "🧑🏿‍🔬", "", U_1F9D1_1F3FF_200D_1F52C, "1f9d1-1f3ff-200d-1f52c.png");
png_name!(ASTRONAUT_TONE5, "🧑🏿‍🚀", "", U_1F9D1_1F3FF_200D_1F680, "1f9d1-1f3ff-200d-1f680.png");
png_name!(FIREFIGHTER_TONE5, "🧑🏿‍🚒", "", U_1F9D1_1F3FF_200D_1F692, "1f9d1-1f3ff-200d-1f692.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE5_1, "🧑🏿‍🤝‍🧑🏻", "", U_1F9D1_1F3FF_200D_1F91D_200D_1F9D1_1F3FB, "1f9d1-1f3ff-200d-1f91d-200d-1f9d1-1f3fb.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE5_2, "🧑🏿‍🤝‍🧑🏼", "", U_1F9D1_1F3FF_200D_1F91D_200D_1F9D1_1F3FC, "1f9d1-1f3ff-200d-1f91d-200d-1f9d1-1f3fc.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE5_3, "🧑🏿‍🤝‍🧑🏽", "", U_1F9D1_1F3FF_200D_1F91D_200D_1F9D1_1F3FD, "1f9d1-1f3ff-200d-1f91d-200d-1f9d1-1f3fd.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE5_4, "🧑🏿‍🤝‍🧑🏾", "", U_1F9D1_1F3FF_200D_1F91D_200D_1F9D1_1F3FE, "1f9d1-1f3ff-200d-1f91d-200d-1f9d1-1f3fe.png");
png_name!(PEOPLE_HOLDING_HANDS_TONE5, "🧑🏿‍🤝‍🧑🏿", "", U_1F9D1_1F3FF_200D_1F91D_200D_1F9D1_1F3FF, "1f9d1-1f3ff-200d-1f91d-200d-1f9d1-1f3ff.png");
png_name!(PERSON_WITH_PROBING_CANE_TONE5, "🧑🏿‍🦯", "", U_1F9D1_1F3FF_200D_1F9AF, "1f9d1-1f3ff-200d-1f9af.png");
png_name!(PERSON_WITH_WHITE_CANE_TONE5, "🧑🏿‍🦯", "", U_1F9D1_1F3FF_200D_1F9AF, "1f9d1-1f3ff-200d-1f9af.png");
png_name!(RED_HAIRED_TONE5, "🧑🏿‍🦰", "", U_1F9D1_1F3FF_200D_1F9B0, "1f9d1-1f3ff-200d-1f9b0.png");
png_name!(CURLY_HAIRED_TONE5, "🧑🏿‍🦱", "", U_1F9D1_1F3FF_200D_1F9B1, "1f9d1-1f3ff-200d-1f9b1.png");
png_name!(BALD_TONE5, "🧑🏿‍🦲", "", U_1F9D1_1F3FF_200D_1F9B2, "1f9d1-1f3ff-200d-1f9b2.png");
png_name!(WHITE_HAIRED_TONE5, "🧑🏿‍🦳", "", U_1F9D1_1F3FF_200D_1F9B3, "1f9d1-1f3ff-200d-1f9b3.png");
png_name!(PERSON_IN_MOTORIZED_WHEELCHAIR_TONE5, "🧑🏿‍🦼", "", U_1F9D1_1F3FF_200D_1F9BC, "1f9d1-1f3ff-200d-1f9bc.png");
png_name!(PERSON_IN_MANUAL_WHEELCHAIR_TONE5, "🧑🏿‍🦽", "", U_1F9D1_1F3FF_200D_1F9BD, "1f9d1-1f3ff-200d-1f9bd.png");
png_name!(HEALTH_WORKER_TONE5, "🧑🏿‍⚕️", "", U_1F9D1_1F3FF_200D_2695_FE0F, "1f9d1-1f3ff-200d-2695-fe0f.png");
png_name!(JUDGE_TONE5, "🧑🏿‍⚖️", "", U_1F9D1_1F3FF_200D_2696_FE0F, "1f9d1-1f3ff-200d-2696-fe0f.png");
png_name!(PILOT_TONE5, "🧑🏿‍✈️", "", U_1F9D1_1F3FF_200D_2708_FE0F, "1f9d1-1f3ff-200d-2708-fe0f.png");
png_name!(COUPLE_KISS_TONE5_1, "🧑🏿‍❤️‍💋‍🧑🏻", "", U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB, "1f9d1-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fb.png");
png_name!(COUPLEKISS_TONE5_1, "🧑🏿‍❤️‍💋‍🧑🏻", "", U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB, "1f9d1-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fb.png");
png_name!(COUPLE_KISS_TONE5_2, "🧑🏿‍❤️‍💋‍🧑🏼", "", U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC, "1f9d1-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fc.png");
png_name!(COUPLEKISS_TONE5_2, "🧑🏿‍❤️‍💋‍🧑🏼", "", U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC, "1f9d1-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fc.png");
png_name!(COUPLE_KISS_TONE5_3, "🧑🏿‍❤️‍💋‍🧑🏽", "", U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD, "1f9d1-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fd.png");
png_name!(COUPLEKISS_TONE5_3, "🧑🏿‍❤️‍💋‍🧑🏽", "", U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD, "1f9d1-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fd.png");
png_name!(COUPLE_KISS_TONE5_4, "🧑🏿‍❤️‍💋‍🧑🏾", "", U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE, "1f9d1-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fe.png");
png_name!(COUPLEKISS_TONE5_4, "🧑🏿‍❤️‍💋‍🧑🏾", "", U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE, "1f9d1-1f3ff-200d-2764-fe0f-200d-1f48b-200d-1f9d1-1f3fe.png");
png_name!(COUPLE_WITH_HEART_TONE5_1, "🧑🏿‍❤️‍🧑🏻", "", U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F9D1_1F3FB, "1f9d1-1f3ff-200d-2764-fe0f-200d-1f9d1-1f3fb.png");
png_name!(COUPLE_WITH_HEART_TONE5_2, "🧑🏿‍❤️‍🧑🏼", "", U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F9D1_1F3FC, "1f9d1-1f3ff-200d-2764-fe0f-200d-1f9d1-1f3fc.png");
png_name!(COUPLE_WITH_HEART_TONE5_3, "🧑🏿‍❤️‍🧑🏽", "", U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F9D1_1F3FD, "1f9d1-1f3ff-200d-2764-fe0f-200d-1f9d1-1f3fd.png");
png_name!(COUPLE_WITH_HEART_TONE5_4, "🧑🏿‍❤️‍🧑🏾", "", U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F9D1_1F3FE, "1f9d1-1f3ff-200d-2764-fe0f-200d-1f9d1-1f3fe.png");
png_name!(ADULT_TONE5, "🧑🏿", "", U_1F9D1_1F3FF, "1f9d1-1f3ff.png");
png_name!(FARMER, "🧑‍🌾", "farmer", U_1F9D1_200D_1F33E, "1f9d1-200d-1f33e.png");
png_name!(COOK, "🧑‍🍳", "cook", U_1F9D1_200D_1F373, "1f9d1-200d-1f373.png");
png_name!(PERSON_FEEDING_BABY, "🧑‍🍼", "person feeding baby", U_1F9D1_200D_1F37C, "1f9d1-200d-1f37c.png");
png_name!(MX_CLAUS, "🧑‍🎄", "mx claus", U_1F9D1_200D_1F384, "1f9d1-200d-1f384.png");
png_name!(STUDENT, "🧑‍🎓", "student", U_1F9D1_200D_1F393, "1f9d1-200d-1f393.png");
png_name!(SINGER, "🧑‍🎤", "singer", U_1F9D1_200D_1F3A4, "1f9d1-200d-1f3a4.png");
png_name!(ARTIST, "🧑‍🎨", "artist", U_1F9D1_200D_1F3A8, "1f9d1-200d-1f3a8.png");
png_name!(TEACHER, "🧑‍🏫", "teacher", U_1F9D1_200D_1F3EB, "1f9d1-200d-1f3eb.png");
png_name!(FACTORY_WORKER, "🧑‍🏭", "factory worker", U_1F9D1_200D_1F3ED, "1f9d1-200d-1f3ed.png");
png_name!(TECHNOLOGIST, "🧑‍💻", "technologist", U_1F9D1_200D_1F4BB, "1f9d1-200d-1f4bb.png");
png_name!(OFFICE_WORKER, "🧑‍💼", "office worker", U_1F9D1_200D_1F4BC, "1f9d1-200d-1f4bc.png");
png_name!(MECHANIC, "🧑‍🔧", "mechanic", U_1F9D1_200D_1F527, "1f9d1-200d-1f527.png");
png_name!(SCIENTIST, "🧑‍🔬", "scientist", U_1F9D1_200D_1F52C, "1f9d1-200d-1f52c.png");
png_name!(ASTRONAUT, "🧑‍🚀", "astronaut", U_1F9D1_200D_1F680, "1f9d1-200d-1f680.png");
png_name!(FIREFIGHTER, "🧑‍🚒", "firefighter", U_1F9D1_200D_1F692, "1f9d1-200d-1f692.png");
png_name!(PEOPLE_HOLDING_HANDS, "🧑‍🤝‍🧑", "people holding hands", U_1F9D1_200D_1F91D_200D_1F9D1, "1f9d1-200d-1f91d-200d-1f9d1.png");
png_name!(PERSON_WITH_PROBING_CANE, "🧑‍🦯", "person with white cane", U_1F9D1_200D_1F9AF, "1f9d1-200d-1f9af.png");
png_name!(PERSON_WITH_WHITE_CANE, "🧑‍🦯", "person with white cane", U_1F9D1_200D_1F9AF, "1f9d1-200d-1f9af.png");
png_name!(RED_HAIRED, "🧑‍🦰", "person: red hair", U_1F9D1_200D_1F9B0, "1f9d1-200d-1f9b0.png");
png_name!(CURLY_HAIRED, "🧑‍🦱", "person: curly hair", U_1F9D1_200D_1F9B1, "1f9d1-200d-1f9b1.png");
png_name!(BALD, "🧑‍🦲", "person: bald", U_1F9D1_200D_1F9B2, "1f9d1-200d-1f9b2.png");
png_name!(WHITE_HAIRED, "🧑‍🦳", "person: white hair", U_1F9D1_200D_1F9B3, "1f9d1-200d-1f9b3.png");
png_name!(PERSON_IN_MOTORIZED_WHEELCHAIR, "🧑‍🦼", "person in motorized wheelchair", U_1F9D1_200D_1F9BC, "1f9d1-200d-1f9bc.png");
png_name!(PERSON_IN_MANUAL_WHEELCHAIR, "🧑‍🦽", "person in manual wheelchair", U_1F9D1_200D_1F9BD, "1f9d1-200d-1f9bd.png");
png_name!(HEALTH_WORKER, "🧑‍⚕️", "health worker", U_1F9D1_200D_2695_FE0F, "1f9d1-200d-2695-fe0f.png");
png_name!(JUDGE, "🧑‍⚖️", "judge", U_1F9D1_200D_2696_FE0F, "1f9d1-200d-2696-fe0f.png");
png_name!(PILOT, "🧑‍✈️", "pilot", U_1F9D1_200D_2708_FE0F, "1f9d1-200d-2708-fe0f.png");
png_name!(ADULT, "🧑", "person", U_1F9D1, "1f9d1.png");
png_name!(CHILD_TONE1, "🧒🏻", "", U_1F9D2_1F3FB, "1f9d2-1f3fb.png");
png_name!(CHILD_TONE2, "🧒🏼", "", U_1F9D2_1F3FC, "1f9d2-1f3fc.png");
png_name!(CHILD_TONE3, "🧒🏽", "", U_1F9D2_1F3FD, "1f9d2-1f3fd.png");
png_name!(CHILD_TONE4, "🧒🏾", "", U_1F9D2_1F3FE, "1f9d2-1f3fe.png");
png_name!(CHILD_TONE5, "🧒🏿", "", U_1F9D2_1F3FF, "1f9d2-1f3ff.png");
png_name!(CHILD, "🧒", "child", U_1F9D2, "1f9d2.png");
png_name!(OLDER_ADULT_TONE1, "🧓🏻", "", U_1F9D3_1F3FB, "1f9d3-1f3fb.png");
png_name!(OLDER_ADULT_TONE2, "🧓🏼", "", U_1F9D3_1F3FC, "1f9d3-1f3fc.png");
png_name!(OLDER_ADULT_TONE3, "🧓🏽", "", U_1F9D3_1F3FD, "1f9d3-1f3fd.png");
png_name!(OLDER_ADULT_TONE4, "🧓🏾", "", U_1F9D3_1F3FE, "1f9d3-1f3fe.png");
png_name!(OLDER_ADULT_TONE5, "🧓🏿", "", U_1F9D3_1F3FF, "1f9d3-1f3ff.png");
png_name!(OLDER_ADULT, "🧓", "older person", U_1F9D3, "1f9d3.png");
png_name!(WOMAN_BEARDED_TONE1, "🧔🏻‍♀️", "", U_1F9D4_1F3FB_200D_2640_FE0F, "1f9d4-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_BEARDED_TONE1, "🧔🏻‍♂️", "", U_1F9D4_1F3FB_200D_2642_FE0F, "1f9d4-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_BEARDED_TONE1, "🧔🏻", "", U_1F9D4_1F3FB, "1f9d4-1f3fb.png");
png_name!(WOMAN_BEARDED_TONE2, "🧔🏼‍♀️", "", U_1F9D4_1F3FC_200D_2640_FE0F, "1f9d4-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_BEARDED_TONE2, "🧔🏼‍♂️", "", U_1F9D4_1F3FC_200D_2642_FE0F, "1f9d4-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_BEARDED_TONE2, "🧔🏼", "", U_1F9D4_1F3FC, "1f9d4-1f3fc.png");
png_name!(WOMAN_BEARDED_TONE3, "🧔🏽‍♀️", "", U_1F9D4_1F3FD_200D_2640_FE0F, "1f9d4-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_BEARDED_TONE3, "🧔🏽‍♂️", "", U_1F9D4_1F3FD_200D_2642_FE0F, "1f9d4-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_BEARDED_TONE3, "🧔🏽", "", U_1F9D4_1F3FD, "1f9d4-1f3fd.png");
png_name!(WOMAN_BEARDED_TONE4, "🧔🏾‍♀️", "", U_1F9D4_1F3FE_200D_2640_FE0F, "1f9d4-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_BEARDED_TONE4, "🧔🏾‍♂️", "", U_1F9D4_1F3FE_200D_2642_FE0F, "1f9d4-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_BEARDED_TONE4, "🧔🏾", "", U_1F9D4_1F3FE, "1f9d4-1f3fe.png");
png_name!(WOMAN_BEARDED_TONE5, "🧔🏿‍♀️", "", U_1F9D4_1F3FF_200D_2640_FE0F, "1f9d4-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_BEARDED_TONE5, "🧔🏿‍♂️", "", U_1F9D4_1F3FF_200D_2642_FE0F, "1f9d4-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_BEARDED_TONE5, "🧔🏿", "", U_1F9D4_1F3FF, "1f9d4-1f3ff.png");
png_name!(WOMAN_BEARDED, "🧔‍♀️", "woman: beard", U_1F9D4_200D_2640_FE0F, "1f9d4-200d-2640-fe0f.png");
png_name!(MAN_BEARDED, "🧔‍♂️", "man: beard", U_1F9D4_200D_2642_FE0F, "1f9d4-200d-2642-fe0f.png");
png_name!(PERSON_BEARDED, "🧔", "person: beard", U_1F9D4, "1f9d4.png");
png_name!(WOMAN_WITH_HEADSCARF_TONE1, "🧕🏻", "", U_1F9D5_1F3FB, "1f9d5-1f3fb.png");
png_name!(WOMAN_WITH_HEADSCARF_TONE2, "🧕🏼", "", U_1F9D5_1F3FC, "1f9d5-1f3fc.png");
png_name!(WOMAN_WITH_HEADSCARF_TONE3, "🧕🏽", "", U_1F9D5_1F3FD, "1f9d5-1f3fd.png");
png_name!(WOMAN_WITH_HEADSCARF_TONE4, "🧕🏾", "", U_1F9D5_1F3FE, "1f9d5-1f3fe.png");
png_name!(WOMAN_WITH_HEADSCARF_TONE5, "🧕🏿", "", U_1F9D5_1F3FF, "1f9d5-1f3ff.png");
png_name!(WOMAN_WITH_HEADSCARF, "🧕", "woman with headscarf", U_1F9D5, "1f9d5.png");
png_name!(WOMAN_IN_STEAMY_ROOM_TONE1, "🧖🏻‍♀️", "", U_1F9D6_1F3FB_200D_2640_FE0F, "1f9d6-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_IN_STEAMY_ROOM_TONE1, "🧖🏻‍♂️", "", U_1F9D6_1F3FB_200D_2642_FE0F, "1f9d6-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_IN_STEAMY_ROOM_TONE1, "🧖🏻", "", U_1F9D6_1F3FB, "1f9d6-1f3fb.png");
png_name!(WOMAN_IN_STEAMY_ROOM_TONE2, "🧖🏼‍♀️", "", U_1F9D6_1F3FC_200D_2640_FE0F, "1f9d6-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_IN_STEAMY_ROOM_TONE2, "🧖🏼‍♂️", "", U_1F9D6_1F3FC_200D_2642_FE0F, "1f9d6-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_IN_STEAMY_ROOM_TONE2, "🧖🏼", "", U_1F9D6_1F3FC, "1f9d6-1f3fc.png");
png_name!(WOMAN_IN_STEAMY_ROOM_TONE3, "🧖🏽‍♀️", "", U_1F9D6_1F3FD_200D_2640_FE0F, "1f9d6-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_IN_STEAMY_ROOM_TONE3, "🧖🏽‍♂️", "", U_1F9D6_1F3FD_200D_2642_FE0F, "1f9d6-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_IN_STEAMY_ROOM_TONE3, "🧖🏽", "", U_1F9D6_1F3FD, "1f9d6-1f3fd.png");
png_name!(WOMAN_IN_STEAMY_ROOM_TONE4, "🧖🏾‍♀️", "", U_1F9D6_1F3FE_200D_2640_FE0F, "1f9d6-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_IN_STEAMY_ROOM_TONE4, "🧖🏾‍♂️", "", U_1F9D6_1F3FE_200D_2642_FE0F, "1f9d6-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_IN_STEAMY_ROOM_TONE4, "🧖🏾", "", U_1F9D6_1F3FE, "1f9d6-1f3fe.png");
png_name!(WOMAN_IN_STEAMY_ROOM_TONE5, "🧖🏿‍♀️", "", U_1F9D6_1F3FF_200D_2640_FE0F, "1f9d6-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_IN_STEAMY_ROOM_TONE5, "🧖🏿‍♂️", "", U_1F9D6_1F3FF_200D_2642_FE0F, "1f9d6-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_IN_STEAMY_ROOM_TONE5, "🧖🏿", "", U_1F9D6_1F3FF, "1f9d6-1f3ff.png");
png_name!(WOMAN_IN_STEAMY_ROOM, "🧖‍♀️", "woman in steamy room", U_1F9D6_200D_2640_FE0F, "1f9d6-200d-2640-fe0f.png");
png_name!(MAN_IN_STEAMY_ROOM, "🧖‍♂️", "man in steamy room", U_1F9D6_200D_2642_FE0F, "1f9d6-200d-2642-fe0f.png");
png_name!(PERSON_IN_STEAMY_ROOM, "🧖", "person in steamy room", U_1F9D6, "1f9d6.png");
png_name!(WOMAN_CLIMBING_TONE1, "🧗🏻‍♀️", "", U_1F9D7_1F3FB_200D_2640_FE0F, "1f9d7-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_CLIMBING_TONE1, "🧗🏻‍♂️", "", U_1F9D7_1F3FB_200D_2642_FE0F, "1f9d7-1f3fb-200d-2642-fe0f.png");
png_name!(CLIMBING_TONE1, "🧗🏻", "", U_1F9D7_1F3FB, "1f9d7-1f3fb.png");
png_name!(PERSON_CLIMBING_TONE1, "🧗🏻", "", U_1F9D7_1F3FB, "1f9d7-1f3fb.png");
png_name!(WOMAN_CLIMBING_TONE2, "🧗🏼‍♀️", "", U_1F9D7_1F3FC_200D_2640_FE0F, "1f9d7-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_CLIMBING_TONE2, "🧗🏼‍♂️", "", U_1F9D7_1F3FC_200D_2642_FE0F, "1f9d7-1f3fc-200d-2642-fe0f.png");
png_name!(CLIMBING_TONE2, "🧗🏼", "", U_1F9D7_1F3FC, "1f9d7-1f3fc.png");
png_name!(PERSON_CLIMBING_TONE2, "🧗🏼", "", U_1F9D7_1F3FC, "1f9d7-1f3fc.png");
png_name!(WOMAN_CLIMBING_TONE3, "🧗🏽‍♀️", "", U_1F9D7_1F3FD_200D_2640_FE0F, "1f9d7-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_CLIMBING_TONE3, "🧗🏽‍♂️", "", U_1F9D7_1F3FD_200D_2642_FE0F, "1f9d7-1f3fd-200d-2642-fe0f.png");
png_name!(CLIMBING_TONE3, "🧗🏽", "", U_1F9D7_1F3FD, "1f9d7-1f3fd.png");
png_name!(PERSON_CLIMBING_TONE3, "🧗🏽", "", U_1F9D7_1F3FD, "1f9d7-1f3fd.png");
png_name!(WOMAN_CLIMBING_TONE4, "🧗🏾‍♀️", "", U_1F9D7_1F3FE_200D_2640_FE0F, "1f9d7-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_CLIMBING_TONE4, "🧗🏾‍♂️", "", U_1F9D7_1F3FE_200D_2642_FE0F, "1f9d7-1f3fe-200d-2642-fe0f.png");
png_name!(CLIMBING_TONE4, "🧗🏾", "", U_1F9D7_1F3FE, "1f9d7-1f3fe.png");
png_name!(PERSON_CLIMBING_TONE4, "🧗🏾", "", U_1F9D7_1F3FE, "1f9d7-1f3fe.png");
png_name!(WOMAN_CLIMBING_TONE5, "🧗🏿‍♀️", "", U_1F9D7_1F3FF_200D_2640_FE0F, "1f9d7-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_CLIMBING_TONE5, "🧗🏿‍♂️", "", U_1F9D7_1F3FF_200D_2642_FE0F, "1f9d7-1f3ff-200d-2642-fe0f.png");
png_name!(CLIMBING_TONE5, "🧗🏿", "", U_1F9D7_1F3FF, "1f9d7-1f3ff.png");
png_name!(PERSON_CLIMBING_TONE5, "🧗🏿", "", U_1F9D7_1F3FF, "1f9d7-1f3ff.png");
png_name!(WOMAN_CLIMBING, "🧗‍♀️", "woman climbing", U_1F9D7_200D_2640_FE0F, "1f9d7-200d-2640-fe0f.png");
png_name!(MAN_CLIMBING, "🧗‍♂️", "man climbing", U_1F9D7_200D_2642_FE0F, "1f9d7-200d-2642-fe0f.png");
png_name!(CLIMBING, "🧗", "person climbing", U_1F9D7, "1f9d7.png");
png_name!(PERSON_CLIMBING, "🧗", "person climbing", U_1F9D7, "1f9d7.png");
png_name!(WOMAN_IN_LOTUS_POSITION_TONE1, "🧘🏻‍♀️", "", U_1F9D8_1F3FB_200D_2640_FE0F, "1f9d8-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_IN_LOTUS_POSITION_TONE1, "🧘🏻‍♂️", "", U_1F9D8_1F3FB_200D_2642_FE0F, "1f9d8-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_IN_LOTUS_POSITION_TONE1, "🧘🏻", "", U_1F9D8_1F3FB, "1f9d8-1f3fb.png");
png_name!(WOMAN_IN_LOTUS_POSITION_TONE2, "🧘🏼‍♀️", "", U_1F9D8_1F3FC_200D_2640_FE0F, "1f9d8-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_IN_LOTUS_POSITION_TONE2, "🧘🏼‍♂️", "", U_1F9D8_1F3FC_200D_2642_FE0F, "1f9d8-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_IN_LOTUS_POSITION_TONE2, "🧘🏼", "", U_1F9D8_1F3FC, "1f9d8-1f3fc.png");
png_name!(WOMAN_IN_LOTUS_POSITION_TONE3, "🧘🏽‍♀️", "", U_1F9D8_1F3FD_200D_2640_FE0F, "1f9d8-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_IN_LOTUS_POSITION_TONE3, "🧘🏽‍♂️", "", U_1F9D8_1F3FD_200D_2642_FE0F, "1f9d8-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_IN_LOTUS_POSITION_TONE3, "🧘🏽", "", U_1F9D8_1F3FD, "1f9d8-1f3fd.png");
png_name!(WOMAN_IN_LOTUS_POSITION_TONE4, "🧘🏾‍♀️", "", U_1F9D8_1F3FE_200D_2640_FE0F, "1f9d8-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_IN_LOTUS_POSITION_TONE4, "🧘🏾‍♂️", "", U_1F9D8_1F3FE_200D_2642_FE0F, "1f9d8-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_IN_LOTUS_POSITION_TONE4, "🧘🏾", "", U_1F9D8_1F3FE, "1f9d8-1f3fe.png");
png_name!(WOMAN_IN_LOTUS_POSITION_TONE5, "🧘🏿‍♀️", "", U_1F9D8_1F3FF_200D_2640_FE0F, "1f9d8-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_IN_LOTUS_POSITION_TONE5, "🧘🏿‍♂️", "", U_1F9D8_1F3FF_200D_2642_FE0F, "1f9d8-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_IN_LOTUS_POSITION_TONE5, "🧘🏿", "", U_1F9D8_1F3FF, "1f9d8-1f3ff.png");
png_name!(WOMAN_IN_LOTUS_POSITION, "🧘‍♀️", "woman in lotus position", U_1F9D8_200D_2640_FE0F, "1f9d8-200d-2640-fe0f.png");
png_name!(MAN_IN_LOTUS_POSITION, "🧘‍♂️", "man in lotus position", U_1F9D8_200D_2642_FE0F, "1f9d8-200d-2642-fe0f.png");
png_name!(PERSON_IN_LOTUS_POSITION, "🧘", "person in lotus position", U_1F9D8, "1f9d8.png");
png_name!(WOMAN_MAGE_TONE1, "🧙🏻‍♀️", "", U_1F9D9_1F3FB_200D_2640_FE0F, "1f9d9-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_MAGE_TONE1, "🧙🏻‍♂️", "", U_1F9D9_1F3FB_200D_2642_FE0F, "1f9d9-1f3fb-200d-2642-fe0f.png");
png_name!(MAGE_TONE1, "🧙🏻", "", U_1F9D9_1F3FB, "1f9d9-1f3fb.png");
png_name!(WOMAN_MAGE_TONE2, "🧙🏼‍♀️", "", U_1F9D9_1F3FC_200D_2640_FE0F, "1f9d9-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_MAGE_TONE2, "🧙🏼‍♂️", "", U_1F9D9_1F3FC_200D_2642_FE0F, "1f9d9-1f3fc-200d-2642-fe0f.png");
png_name!(MAGE_TONE2, "🧙🏼", "", U_1F9D9_1F3FC, "1f9d9-1f3fc.png");
png_name!(WOMAN_MAGE_TONE3, "🧙🏽‍♀️", "", U_1F9D9_1F3FD_200D_2640_FE0F, "1f9d9-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_MAGE_TONE3, "🧙🏽‍♂️", "", U_1F9D9_1F3FD_200D_2642_FE0F, "1f9d9-1f3fd-200d-2642-fe0f.png");
png_name!(MAGE_TONE3, "🧙🏽", "", U_1F9D9_1F3FD, "1f9d9-1f3fd.png");
png_name!(WOMAN_MAGE_TONE4, "🧙🏾‍♀️", "", U_1F9D9_1F3FE_200D_2640_FE0F, "1f9d9-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_MAGE_TONE4, "🧙🏾‍♂️", "", U_1F9D9_1F3FE_200D_2642_FE0F, "1f9d9-1f3fe-200d-2642-fe0f.png");
png_name!(MAGE_TONE4, "🧙🏾", "", U_1F9D9_1F3FE, "1f9d9-1f3fe.png");
png_name!(WOMAN_MAGE_TONE5, "🧙🏿‍♀️", "", U_1F9D9_1F3FF_200D_2640_FE0F, "1f9d9-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_MAGE_TONE5, "🧙🏿‍♂️", "", U_1F9D9_1F3FF_200D_2642_FE0F, "1f9d9-1f3ff-200d-2642-fe0f.png");
png_name!(MAGE_TONE5, "🧙🏿", "", U_1F9D9_1F3FF, "1f9d9-1f3ff.png");
png_name!(WOMAN_MAGE, "🧙‍♀️", "woman mage", U_1F9D9_200D_2640_FE0F, "1f9d9-200d-2640-fe0f.png");
png_name!(MAN_MAGE, "🧙‍♂️", "man mage", U_1F9D9_200D_2642_FE0F, "1f9d9-200d-2642-fe0f.png");
png_name!(MAGE, "🧙", "mage", U_1F9D9, "1f9d9.png");
png_name!(WOMAN_FAIRY_TONE1, "🧚🏻‍♀️", "", U_1F9DA_1F3FB_200D_2640_FE0F, "1f9da-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_FAIRY_TONE1, "🧚🏻‍♂️", "", U_1F9DA_1F3FB_200D_2642_FE0F, "1f9da-1f3fb-200d-2642-fe0f.png");
png_name!(FAIRY_TONE1, "🧚🏻", "", U_1F9DA_1F3FB, "1f9da-1f3fb.png");
png_name!(WOMAN_FAIRY_TONE2, "🧚🏼‍♀️", "", U_1F9DA_1F3FC_200D_2640_FE0F, "1f9da-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_FAIRY_TONE2, "🧚🏼‍♂️", "", U_1F9DA_1F3FC_200D_2642_FE0F, "1f9da-1f3fc-200d-2642-fe0f.png");
png_name!(FAIRY_TONE2, "🧚🏼", "", U_1F9DA_1F3FC, "1f9da-1f3fc.png");
png_name!(WOMAN_FAIRY_TONE3, "🧚🏽‍♀️", "", U_1F9DA_1F3FD_200D_2640_FE0F, "1f9da-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_FAIRY_TONE3, "🧚🏽‍♂️", "", U_1F9DA_1F3FD_200D_2642_FE0F, "1f9da-1f3fd-200d-2642-fe0f.png");
png_name!(FAIRY_TONE3, "🧚🏽", "", U_1F9DA_1F3FD, "1f9da-1f3fd.png");
png_name!(WOMAN_FAIRY_TONE4, "🧚🏾‍♀️", "", U_1F9DA_1F3FE_200D_2640_FE0F, "1f9da-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_FAIRY_TONE4, "🧚🏾‍♂️", "", U_1F9DA_1F3FE_200D_2642_FE0F, "1f9da-1f3fe-200d-2642-fe0f.png");
png_name!(FAIRY_TONE4, "🧚🏾", "", U_1F9DA_1F3FE, "1f9da-1f3fe.png");
png_name!(WOMAN_FAIRY_TONE5, "🧚🏿‍♀️", "", U_1F9DA_1F3FF_200D_2640_FE0F, "1f9da-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_FAIRY_TONE5, "🧚🏿‍♂️", "", U_1F9DA_1F3FF_200D_2642_FE0F, "1f9da-1f3ff-200d-2642-fe0f.png");
png_name!(FAIRY_TONE5, "🧚🏿", "", U_1F9DA_1F3FF, "1f9da-1f3ff.png");
png_name!(WOMAN_FAIRY, "🧚‍♀️", "woman fairy", U_1F9DA_200D_2640_FE0F, "1f9da-200d-2640-fe0f.png");
png_name!(MAN_FAIRY, "🧚‍♂️", "man fairy", U_1F9DA_200D_2642_FE0F, "1f9da-200d-2642-fe0f.png");
png_name!(FAIRY, "🧚", "fairy", U_1F9DA, "1f9da.png");
png_name!(WOMAN_VAMPIRE_TONE1, "🧛🏻‍♀️", "", U_1F9DB_1F3FB_200D_2640_FE0F, "1f9db-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_VAMPIRE_TONE1, "🧛🏻‍♂️", "", U_1F9DB_1F3FB_200D_2642_FE0F, "1f9db-1f3fb-200d-2642-fe0f.png");
png_name!(VAMPIRE_TONE1, "🧛🏻", "", U_1F9DB_1F3FB, "1f9db-1f3fb.png");
png_name!(WOMAN_VAMPIRE_TONE2, "🧛🏼‍♀️", "", U_1F9DB_1F3FC_200D_2640_FE0F, "1f9db-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_VAMPIRE_TONE2, "🧛🏼‍♂️", "", U_1F9DB_1F3FC_200D_2642_FE0F, "1f9db-1f3fc-200d-2642-fe0f.png");
png_name!(VAMPIRE_TONE2, "🧛🏼", "", U_1F9DB_1F3FC, "1f9db-1f3fc.png");
png_name!(WOMAN_VAMPIRE_TONE3, "🧛🏽‍♀️", "", U_1F9DB_1F3FD_200D_2640_FE0F, "1f9db-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_VAMPIRE_TONE3, "🧛🏽‍♂️", "", U_1F9DB_1F3FD_200D_2642_FE0F, "1f9db-1f3fd-200d-2642-fe0f.png");
png_name!(VAMPIRE_TONE3, "🧛🏽", "", U_1F9DB_1F3FD, "1f9db-1f3fd.png");
png_name!(WOMAN_VAMPIRE_TONE4, "🧛🏾‍♀️", "", U_1F9DB_1F3FE_200D_2640_FE0F, "1f9db-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_VAMPIRE_TONE4, "🧛🏾‍♂️", "", U_1F9DB_1F3FE_200D_2642_FE0F, "1f9db-1f3fe-200d-2642-fe0f.png");
png_name!(VAMPIRE_TONE4, "🧛🏾", "", U_1F9DB_1F3FE, "1f9db-1f3fe.png");
png_name!(WOMAN_VAMPIRE_TONE5, "🧛🏿‍♀️", "", U_1F9DB_1F3FF_200D_2640_FE0F, "1f9db-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_VAMPIRE_TONE5, "🧛🏿‍♂️", "", U_1F9DB_1F3FF_200D_2642_FE0F, "1f9db-1f3ff-200d-2642-fe0f.png");
png_name!(VAMPIRE_TONE5, "🧛🏿", "", U_1F9DB_1F3FF, "1f9db-1f3ff.png");
png_name!(WOMAN_VAMPIRE, "🧛‍♀️", "woman vampire", U_1F9DB_200D_2640_FE0F, "1f9db-200d-2640-fe0f.png");
png_name!(MAN_VAMPIRE, "🧛‍♂️", "man vampire", U_1F9DB_200D_2642_FE0F, "1f9db-200d-2642-fe0f.png");
png_name!(VAMPIRE, "🧛", "vampire", U_1F9DB, "1f9db.png");
png_name!(MERMAID_TONE1, "🧜🏻‍♀️", "", U_1F9DC_1F3FB_200D_2640_FE0F, "1f9dc-1f3fb-200d-2640-fe0f.png");
png_name!(MERMAN_TONE1, "🧜🏻‍♂️", "", U_1F9DC_1F3FB_200D_2642_FE0F, "1f9dc-1f3fb-200d-2642-fe0f.png");
png_name!(MERPERSON_TONE1, "🧜🏻", "", U_1F9DC_1F3FB, "1f9dc-1f3fb.png");
png_name!(MERMAID_TONE2, "🧜🏼‍♀️", "", U_1F9DC_1F3FC_200D_2640_FE0F, "1f9dc-1f3fc-200d-2640-fe0f.png");
png_name!(MERMAN_TONE2, "🧜🏼‍♂️", "", U_1F9DC_1F3FC_200D_2642_FE0F, "1f9dc-1f3fc-200d-2642-fe0f.png");
png_name!(MERPERSON_TONE2, "🧜🏼", "", U_1F9DC_1F3FC, "1f9dc-1f3fc.png");
png_name!(MERMAID_TONE3, "🧜🏽‍♀️", "", U_1F9DC_1F3FD_200D_2640_FE0F, "1f9dc-1f3fd-200d-2640-fe0f.png");
png_name!(MERMAN_TONE3, "🧜🏽‍♂️", "", U_1F9DC_1F3FD_200D_2642_FE0F, "1f9dc-1f3fd-200d-2642-fe0f.png");
png_name!(MERPERSON_TONE3, "🧜🏽", "", U_1F9DC_1F3FD, "1f9dc-1f3fd.png");
png_name!(MERMAID_TONE4, "🧜🏾‍♀️", "", U_1F9DC_1F3FE_200D_2640_FE0F, "1f9dc-1f3fe-200d-2640-fe0f.png");
png_name!(MERMAN_TONE4, "🧜🏾‍♂️", "", U_1F9DC_1F3FE_200D_2642_FE0F, "1f9dc-1f3fe-200d-2642-fe0f.png");
png_name!(MERPERSON_TONE4, "🧜🏾", "", U_1F9DC_1F3FE, "1f9dc-1f3fe.png");
png_name!(MERMAID_TONE5, "🧜🏿‍♀️", "", U_1F9DC_1F3FF_200D_2640_FE0F, "1f9dc-1f3ff-200d-2640-fe0f.png");
png_name!(MERMAN_TONE5, "🧜🏿‍♂️", "", U_1F9DC_1F3FF_200D_2642_FE0F, "1f9dc-1f3ff-200d-2642-fe0f.png");
png_name!(MERPERSON_TONE5, "🧜🏿", "", U_1F9DC_1F3FF, "1f9dc-1f3ff.png");
png_name!(MERMAID, "🧜‍♀️", "mermaid", U_1F9DC_200D_2640_FE0F, "1f9dc-200d-2640-fe0f.png");
png_name!(MERMAN, "🧜‍♂️", "merman", U_1F9DC_200D_2642_FE0F, "1f9dc-200d-2642-fe0f.png");
png_name!(MERPERSON, "🧜", "merperson", U_1F9DC, "1f9dc.png");
png_name!(WOMAN_ELF_TONE1, "🧝🏻‍♀️", "", U_1F9DD_1F3FB_200D_2640_FE0F, "1f9dd-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_ELF_TONE1, "🧝🏻‍♂️", "", U_1F9DD_1F3FB_200D_2642_FE0F, "1f9dd-1f3fb-200d-2642-fe0f.png");
png_name!(ELF_TONE1, "🧝🏻", "", U_1F9DD_1F3FB, "1f9dd-1f3fb.png");
png_name!(WOMAN_ELF_TONE2, "🧝🏼‍♀️", "", U_1F9DD_1F3FC_200D_2640_FE0F, "1f9dd-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_ELF_TONE2, "🧝🏼‍♂️", "", U_1F9DD_1F3FC_200D_2642_FE0F, "1f9dd-1f3fc-200d-2642-fe0f.png");
png_name!(ELF_TONE2, "🧝🏼", "", U_1F9DD_1F3FC, "1f9dd-1f3fc.png");
png_name!(WOMAN_ELF_TONE3, "🧝🏽‍♀️", "", U_1F9DD_1F3FD_200D_2640_FE0F, "1f9dd-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_ELF_TONE3, "🧝🏽‍♂️", "", U_1F9DD_1F3FD_200D_2642_FE0F, "1f9dd-1f3fd-200d-2642-fe0f.png");
png_name!(ELF_TONE3, "🧝🏽", "", U_1F9DD_1F3FD, "1f9dd-1f3fd.png");
png_name!(WOMAN_ELF_TONE4, "🧝🏾‍♀️", "", U_1F9DD_1F3FE_200D_2640_FE0F, "1f9dd-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_ELF_TONE4, "🧝🏾‍♂️", "", U_1F9DD_1F3FE_200D_2642_FE0F, "1f9dd-1f3fe-200d-2642-fe0f.png");
png_name!(ELF_TONE4, "🧝🏾", "", U_1F9DD_1F3FE, "1f9dd-1f3fe.png");
png_name!(WOMAN_ELF_TONE5, "🧝🏿‍♀️", "", U_1F9DD_1F3FF_200D_2640_FE0F, "1f9dd-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_ELF_TONE5, "🧝🏿‍♂️", "", U_1F9DD_1F3FF_200D_2642_FE0F, "1f9dd-1f3ff-200d-2642-fe0f.png");
png_name!(ELF_TONE5, "🧝🏿", "", U_1F9DD_1F3FF, "1f9dd-1f3ff.png");
png_name!(WOMAN_ELF, "🧝‍♀️", "woman elf", U_1F9DD_200D_2640_FE0F, "1f9dd-200d-2640-fe0f.png");
png_name!(MAN_ELF, "🧝‍♂️", "man elf", U_1F9DD_200D_2642_FE0F, "1f9dd-200d-2642-fe0f.png");
png_name!(ELF, "🧝", "elf", U_1F9DD, "1f9dd.png");
png_name!(WOMAN_GENIE, "🧞‍♀️", "woman genie", U_1F9DE_200D_2640_FE0F, "1f9de-200d-2640-fe0f.png");
png_name!(MAN_GENIE, "🧞‍♂️", "man genie", U_1F9DE_200D_2642_FE0F, "1f9de-200d-2642-fe0f.png");
png_name!(GENIE, "🧞", "genie", U_1F9DE, "1f9de.png");
png_name!(WOMAN_ZOMBIE, "🧟‍♀️", "woman zombie", U_1F9DF_200D_2640_FE0F, "1f9df-200d-2640-fe0f.png");
png_name!(MAN_ZOMBIE, "🧟‍♂️", "man zombie", U_1F9DF_200D_2642_FE0F, "1f9df-200d-2642-fe0f.png");
png_name!(ZOMBIE, "🧟", "zombie", U_1F9DF, "1f9df.png");
png_name!(BRAIN, "🧠", "brain", U_1F9E0, "1f9e0.png");
png_name!(ORANGE_HEART, "🧡", "orange heart", U_1F9E1, "1f9e1.png");
png_name!(BILLED_CAP, "🧢", "billed cap", U_1F9E2, "1f9e2.png");
png_name!(SCARF, "🧣", "scarf", U_1F9E3, "1f9e3.png");
png_name!(GLOVES, "🧤", "gloves", U_1F9E4, "1f9e4.png");
png_name!(COAT, "🧥", "coat", U_1F9E5, "1f9e5.png");
png_name!(SOCKS, "🧦", "socks", U_1F9E6, "1f9e6.png");
png_name!(RED_ENVELOPE, "🧧", "red envelope", U_1F9E7, "1f9e7.png");
png_name!(FIRECRACKER, "🧨", "firecracker", U_1F9E8, "1f9e8.png");
png_name!(JIGSAW, "🧩", "puzzle piece", U_1F9E9, "1f9e9.png");
png_name!(PUZZLE_PIECE, "🧩", "puzzle piece", U_1F9E9, "1f9e9.png");
png_name!(TEST_TUBE, "🧪", "test tube", U_1F9EA, "1f9ea.png");
png_name!(PETRI_DISH, "🧫", "petri dish", U_1F9EB, "1f9eb.png");
png_name!(DNA, "🧬", "dna", U_1F9EC, "1f9ec.png");
png_name!(DOUBLE_HELIX, "🧬", "dna", U_1F9EC, "1f9ec.png");
png_name!(COMPASS, "🧭", "compass", U_1F9ED, "1f9ed.png");
png_name!(ABACUS, "🧮", "abacus", U_1F9EE, "1f9ee.png");
png_name!(FIRE_EXTINGUISHER, "🧯", "fire extinguisher", U_1F9EF, "1f9ef.png");
png_name!(TOOLBOX, "🧰", "toolbox", U_1F9F0, "1f9f0.png");
png_name!(BRICKS, "🧱", "brick", U_1F9F1, "1f9f1.png");
png_name!(MAGNET, "🧲", "magnet", U_1F9F2, "1f9f2.png");
png_name!(LUGGAGE, "🧳", "luggage", U_1F9F3, "1f9f3.png");
png_name!(LOTION_BOTTLE, "🧴", "lotion bottle", U_1F9F4, "1f9f4.png");
png_name!(THREAD, "🧵", "thread", U_1F9F5, "1f9f5.png");
png_name!(YARN, "🧶", "yarn", U_1F9F6, "1f9f6.png");
png_name!(SAFETY_PIN, "🧷", "safety pin", U_1F9F7, "1f9f7.png");
png_name!(TEDDY_BEAR, "🧸", "teddy bear", U_1F9F8, "1f9f8.png");
png_name!(BROOM, "🧹", "broom", U_1F9F9, "1f9f9.png");
png_name!(BASKET, "🧺", "basket", U_1F9FA, "1f9fa.png");
png_name!(ROLL_OF_PAPER, "🧻", "roll of paper", U_1F9FB, "1f9fb.png");
png_name!(TOILET_PAPER, "🧻", "roll of paper", U_1F9FB, "1f9fb.png");
png_name!(SOAP, "🧼", "soap", U_1F9FC, "1f9fc.png");
png_name!(SPONGE, "🧽", "sponge", U_1F9FD, "1f9fd.png");
png_name!(RECEIPT, "🧾", "receipt", U_1F9FE, "1f9fe.png");
png_name!(NAZAR_AMULET, "🧿", "nazar amulet", U_1F9FF, "1f9ff.png");
png_name!(BALLET_SHOES, "🩰", "ballet shoes", U_1FA70, "1fa70.png");
png_name!(ONE_PIECE_SWIMSUIT, "🩱", "one-piece swimsuit", U_1FA71, "1fa71.png");
png_name!(BRIEFS, "🩲", "briefs", U_1FA72, "1fa72.png");
png_name!(SHORTS, "🩳", "shorts", U_1FA73, "1fa73.png");
png_name!(THONG_SANDAL, "🩴", "thong sandal", U_1FA74, "1fa74.png");
png_name!(DROP_OF_BLOOD, "🩸", "drop of blood", U_1FA78, "1fa78.png");
png_name!(ADHESIVE_BANDAGE, "🩹", "adhesive bandage", U_1FA79, "1fa79.png");
png_name!(BANDAID, "🩹", "adhesive bandage", U_1FA79, "1fa79.png");
png_name!(STETHOSCOPE, "🩺", "stethoscope", U_1FA7A, "1fa7a.png");
png_name!(X_RAY, "🩻", "x-ray", U_1FA7B, "1fa7b.png");
png_name!(XRAY, "🩻", "x-ray", U_1FA7B, "1fa7b.png");
png_name!(CRUTCH, "🩼", "crutch", U_1FA7C, "1fa7c.png");
png_name!(YO_YO, "🪀", "yo-yo", U_1FA80, "1fa80.png");
png_name!(KITE, "🪁", "kite", U_1FA81, "1fa81.png");
png_name!(PARACHUTE, "🪂", "parachute", U_1FA82, "1fa82.png");
png_name!(BOOMERANG, "🪃", "boomerang", U_1FA83, "1fa83.png");
png_name!(MAGIC_WAND, "🪄", "magic wand", U_1FA84, "1fa84.png");
png_name!(PINATA, "🪅", "piñata", U_1FA85, "1fa85.png");
png_name!(NESTING_DOLLS, "🪆", "nesting dolls", U_1FA86, "1fa86.png");
png_name!(RINGED_PLANET, "🪐", "ringed planet", U_1FA90, "1fa90.png");
png_name!(SATURN, "🪐", "ringed planet", U_1FA90, "1fa90.png");
png_name!(CHAIR, "🪑", "chair", U_1FA91, "1fa91.png");
png_name!(RAZOR, "🪒", "razor", U_1FA92, "1fa92.png");
png_name!(AXE, "🪓", "axe", U_1FA93, "1fa93.png");
png_name!(DIYA_LAMP, "🪔", "diya lamp", U_1FA94, "1fa94.png");
png_name!(BANJO, "🪕", "banjo", U_1FA95, "1fa95.png");
png_name!(MILITARY_HELMET, "🪖", "military helmet", U_1FA96, "1fa96.png");
png_name!(ACCORDION, "🪗", "accordion", U_1FA97, "1fa97.png");
png_name!(LONG_DRUM, "🪘", "long drum", U_1FA98, "1fa98.png");
png_name!(COIN, "🪙", "coin", U_1FA99, "1fa99.png");
png_name!(CARPENTRY_SAW, "🪚", "carpentry saw", U_1FA9A, "1fa9a.png");
png_name!(SCREWDRIVER, "🪛", "screwdriver", U_1FA9B, "1fa9b.png");
png_name!(LADDER, "🪜", "ladder", U_1FA9C, "1fa9c.png");
png_name!(HOOK, "🪝", "hook", U_1FA9D, "1fa9d.png");
png_name!(MIRROR, "🪞", "mirror", U_1FA9E, "1fa9e.png");
png_name!(WINDOW, "🪟", "window", U_1FA9F, "1fa9f.png");
png_name!(PLUNGER, "🪠", "plunger", U_1FAA0, "1faa0.png");
png_name!(SEWING_NEEDLE, "🪡", "sewing needle", U_1FAA1, "1faa1.png");
png_name!(KNOT, "🪢", "knot", U_1FAA2, "1faa2.png");
png_name!(BUCKET, "🪣", "bucket", U_1FAA3, "1faa3.png");
png_name!(MOUSE_TRAP, "🪤", "mouse trap", U_1FAA4, "1faa4.png");
png_name!(TOOTHBRUSH, "🪥", "toothbrush", U_1FAA5, "1faa5.png");
png_name!(HEADSTONE, "🪦", "headstone", U_1FAA6, "1faa6.png");
png_name!(PLACARD, "🪧", "placard", U_1FAA7, "1faa7.png");
png_name!(ROCK, "🪨", "rock", U_1FAA8, "1faa8.png");
png_name!(DISCO, "🪩", "mirror ball", U_1FAA9, "1faa9.png");
png_name!(DISCO_BALL, "🪩", "mirror ball", U_1FAA9, "1faa9.png");
png_name!(MIRROR_BALL, "🪩", "mirror ball", U_1FAA9, "1faa9.png");
png_name!(ID_CARD, "🪪", "identification card", U_1FAAA, "1faaa.png");
png_name!(LOW_BATTERY, "🪫", "low battery", U_1FAAB, "1faab.png");
png_name!(HAMSA, "🪬", "hamsa", U_1FAAC, "1faac.png");
png_name!(FLY, "🪰", "fly", U_1FAB0, "1fab0.png");
png_name!(WORM, "🪱", "worm", U_1FAB1, "1fab1.png");
png_name!(BEETLE, "🪲", "beetle", U_1FAB2, "1fab2.png");
png_name!(COCKROACH, "🪳", "cockroach", U_1FAB3, "1fab3.png");
png_name!(POTTED_PLANT, "🪴", "potted plant", U_1FAB4, "1fab4.png");
png_name!(WOOD, "🪵", "wood", U_1FAB5, "1fab5.png");
png_name!(FEATHER, "🪶", "feather", U_1FAB6, "1fab6.png");
png_name!(LOTUS, "🪷", "lotus", U_1FAB7, "1fab7.png");
png_name!(CORAL, "🪸", "coral", U_1FAB8, "1fab8.png");
png_name!(EMPTY_NEST, "🪹", "empty nest", U_1FAB9, "1fab9.png");
png_name!(NEST, "🪹", "empty nest", U_1FAB9, "1fab9.png");
png_name!(NEST_WITH_EGGS, "🪺", "nest with eggs", U_1FABA, "1faba.png");
png_name!(ANATOMICAL_HEART, "🫀", "anatomical heart", U_1FAC0, "1fac0.png");
png_name!(LUNGS, "🫁", "lungs", U_1FAC1, "1fac1.png");
png_name!(PEOPLE_HUGGING, "🫂", "people hugging", U_1FAC2, "1fac2.png");
png_name!(PREGNANT_MAN_TONE1, "🫃🏻", "", U_1FAC3_1F3FB, "1fac3-1f3fb.png");
png_name!(PREGNANT_MAN_TONE2, "🫃🏼", "", U_1FAC3_1F3FC, "1fac3-1f3fc.png");
png_name!(PREGNANT_MAN_TONE3, "🫃🏽", "", U_1FAC3_1F3FD, "1fac3-1f3fd.png");
png_name!(PREGNANT_MAN_TONE4, "🫃🏾", "", U_1FAC3_1F3FE, "1fac3-1f3fe.png");
png_name!(PREGNANT_MAN_TONE5, "🫃🏿", "", U_1FAC3_1F3FF, "1fac3-1f3ff.png");
png_name!(PREGNANT_MAN, "🫃", "pregnant man", U_1FAC3, "1fac3.png");
png_name!(PREGNANT_PERSON_TONE1, "🫄🏻", "", U_1FAC4_1F3FB, "1fac4-1f3fb.png");
png_name!(PREGNANT_PERSON_TONE2, "🫄🏼", "", U_1FAC4_1F3FC, "1fac4-1f3fc.png");
png_name!(PREGNANT_PERSON_TONE3, "🫄🏽", "", U_1FAC4_1F3FD, "1fac4-1f3fd.png");
png_name!(PREGNANT_PERSON_TONE4, "🫄🏾", "", U_1FAC4_1F3FE, "1fac4-1f3fe.png");
png_name!(PREGNANT_PERSON_TONE5, "🫄🏿", "", U_1FAC4_1F3FF, "1fac4-1f3ff.png");
png_name!(PREGNANT_PERSON, "🫄", "pregnant person", U_1FAC4, "1fac4.png");
png_name!(PERSON_WITH_CROWN_TONE1, "🫅🏻", "", U_1FAC5_1F3FB, "1fac5-1f3fb.png");
png_name!(ROYALTY_TONE1, "🫅🏻", "", U_1FAC5_1F3FB, "1fac5-1f3fb.png");
png_name!(PERSON_WITH_CROWN_TONE2, "🫅🏼", "", U_1FAC5_1F3FC, "1fac5-1f3fc.png");
png_name!(ROYALTY_TONE2, "🫅🏼", "", U_1FAC5_1F3FC, "1fac5-1f3fc.png");
png_name!(PERSON_WITH_CROWN_TONE3, "🫅🏽", "", U_1FAC5_1F3FD, "1fac5-1f3fd.png");
png_name!(ROYALTY_TONE3, "🫅🏽", "", U_1FAC5_1F3FD, "1fac5-1f3fd.png");
png_name!(PERSON_WITH_CROWN_TONE4, "🫅🏾", "", U_1FAC5_1F3FE, "1fac5-1f3fe.png");
png_name!(ROYALTY_TONE4, "🫅🏾", "", U_1FAC5_1F3FE, "1fac5-1f3fe.png");
png_name!(PERSON_WITH_CROWN_TONE5, "🫅🏿", "", U_1FAC5_1F3FF, "1fac5-1f3ff.png");
png_name!(ROYALTY_TONE5, "🫅🏿", "", U_1FAC5_1F3FF, "1fac5-1f3ff.png");
png_name!(PERSON_WITH_CROWN, "🫅", "person with crown", U_1FAC5, "1fac5.png");
png_name!(ROYALTY, "🫅", "person with crown", U_1FAC5, "1fac5.png");
png_name!(BLUEBERRIES, "🫐", "blueberries", U_1FAD0, "1fad0.png");
png_name!(BELL_PEPPER, "🫑", "bell pepper", U_1FAD1, "1fad1.png");
png_name!(OLIVE, "🫒", "olive", U_1FAD2, "1fad2.png");
png_name!(FLATBREAD, "🫓", "flatbread", U_1FAD3, "1fad3.png");
png_name!(TAMALE, "🫔", "tamale", U_1FAD4, "1fad4.png");
png_name!(FONDUE, "🫕", "fondue", U_1FAD5, "1fad5.png");
png_name!(TEAPOT, "🫖", "teapot", U_1FAD6, "1fad6.png");
png_name!(POUR, "🫗", "pouring liquid", U_1FAD7, "1fad7.png");
png_name!(POURING_LIQUID, "🫗", "pouring liquid", U_1FAD7, "1fad7.png");
png_name!(BEANS, "🫘", "beans", U_1FAD8, "1fad8.png");
png_name!(JAR, "🫙", "jar", U_1FAD9, "1fad9.png");
png_name!(MELT, "🫠", "melting face", U_1FAE0, "1fae0.png");
png_name!(MELTING_FACE, "🫠", "melting face", U_1FAE0, "1fae0.png");
png_name!(SALUTE, "🫡", "saluting face", U_1FAE1, "1fae1.png");
png_name!(SALUTING_FACE, "🫡", "saluting face", U_1FAE1, "1fae1.png");
png_name!(FACE_WITH_OPEN_EYES_HAND_OVER_MOUTH, "🫢", "face with open eyes and hand over mouth", U_1FAE2, "1fae2.png");
png_name!(GASP, "🫢", "face with open eyes and hand over mouth", U_1FAE2, "1fae2.png");
png_name!(FACE_WITH_PEEKING_EYE, "🫣", "face with peeking eye", U_1FAE3, "1fae3.png");
png_name!(PEEK, "🫣", "face with peeking eye", U_1FAE3, "1fae3.png");
png_name!(FACE_WITH_DIAGONAL_MOUTH, "🫤", "face with diagonal mouth", U_1FAE4, "1fae4.png");
png_name!(DOTTED_LINE_FACE, "🫥", "dotted line face", U_1FAE5, "1fae5.png");
png_name!(BITING_LIP, "🫦", "biting lip", U_1FAE6, "1fae6.png");
png_name!(BUBBLES, "🫧", "bubbles", U_1FAE7, "1fae7.png");
png_name!(HAND_WITH_INDEX_FINGER_AND_THUMB_CROSSED_TONE1, "🫰🏻", "", U_1FAF0_1F3FB, "1faf0-1f3fb.png");
png_name!(HAND_WITH_INDEX_FINGER_AND_THUMB_CROSSED_TONE2, "🫰🏼", "", U_1FAF0_1F3FC, "1faf0-1f3fc.png");
png_name!(HAND_WITH_INDEX_FINGER_AND_THUMB_CROSSED_TONE3, "🫰🏽", "", U_1FAF0_1F3FD, "1faf0-1f3fd.png");
png_name!(HAND_WITH_INDEX_FINGER_AND_THUMB_CROSSED_TONE4, "🫰🏾", "", U_1FAF0_1F3FE, "1faf0-1f3fe.png");
png_name!(HAND_WITH_INDEX_FINGER_AND_THUMB_CROSSED_TONE5, "🫰🏿", "", U_1FAF0_1F3FF, "1faf0-1f3ff.png");
png_name!(HAND_WITH_INDEX_FINGER_AND_THUMB_CROSSED, "🫰", "hand with index finger and thumb crossed", U_1FAF0, "1faf0.png");
png_name!(HANDSHAKE_TONE1_2, "🫱🏻‍🫲🏼", "", U_1FAF1_1F3FB_200D_1FAF2_1F3FC, "1faf1-1f3fb-200d-1faf2-1f3fc.png");
png_name!(HANDSHAKE_TONE1_3, "🫱🏻‍🫲🏽", "", U_1FAF1_1F3FB_200D_1FAF2_1F3FD, "1faf1-1f3fb-200d-1faf2-1f3fd.png");
png_name!(HANDSHAKE_TONE1_4, "🫱🏻‍🫲🏾", "", U_1FAF1_1F3FB_200D_1FAF2_1F3FE, "1faf1-1f3fb-200d-1faf2-1f3fe.png");
png_name!(HANDSHAKE_TONE1_5, "🫱🏻‍🫲🏿", "", U_1FAF1_1F3FB_200D_1FAF2_1F3FF, "1faf1-1f3fb-200d-1faf2-1f3ff.png");
png_name!(RIGHTWARDS_HAND_TONE1, "🫱🏻", "", U_1FAF1_1F3FB, "1faf1-1f3fb.png");
png_name!(HANDSHAKE_TONE2_1, "🫱🏼‍🫲🏻", "", U_1FAF1_1F3FC_200D_1FAF2_1F3FB, "1faf1-1f3fc-200d-1faf2-1f3fb.png");
png_name!(HANDSHAKE_TONE2_3, "🫱🏼‍🫲🏽", "", U_1FAF1_1F3FC_200D_1FAF2_1F3FD, "1faf1-1f3fc-200d-1faf2-1f3fd.png");
png_name!(HANDSHAKE_TONE2_4, "🫱🏼‍🫲🏾", "", U_1FAF1_1F3FC_200D_1FAF2_1F3FE, "1faf1-1f3fc-200d-1faf2-1f3fe.png");
png_name!(HANDSHAKE_TONE2_5, "🫱🏼‍🫲🏿", "", U_1FAF1_1F3FC_200D_1FAF2_1F3FF, "1faf1-1f3fc-200d-1faf2-1f3ff.png");
png_name!(RIGHTWARDS_HAND_TONE2, "🫱🏼", "", U_1FAF1_1F3FC, "1faf1-1f3fc.png");
png_name!(HANDSHAKE_TONE3_1, "🫱🏽‍🫲🏻", "", U_1FAF1_1F3FD_200D_1FAF2_1F3FB, "1faf1-1f3fd-200d-1faf2-1f3fb.png");
png_name!(HANDSHAKE_TONE3_2, "🫱🏽‍🫲🏼", "", U_1FAF1_1F3FD_200D_1FAF2_1F3FC, "1faf1-1f3fd-200d-1faf2-1f3fc.png");
png_name!(HANDSHAKE_TONE3_4, "🫱🏽‍🫲🏾", "", U_1FAF1_1F3FD_200D_1FAF2_1F3FE, "1faf1-1f3fd-200d-1faf2-1f3fe.png");
png_name!(HANDSHAKE_TONE3_5, "🫱🏽‍🫲🏿", "", U_1FAF1_1F3FD_200D_1FAF2_1F3FF, "1faf1-1f3fd-200d-1faf2-1f3ff.png");
png_name!(RIGHTWARDS_HAND_TONE3, "🫱🏽", "", U_1FAF1_1F3FD, "1faf1-1f3fd.png");
png_name!(HANDSHAKE_TONE4_1, "🫱🏾‍🫲🏻", "", U_1FAF1_1F3FE_200D_1FAF2_1F3FB, "1faf1-1f3fe-200d-1faf2-1f3fb.png");
png_name!(HANDSHAKE_TONE4_2, "🫱🏾‍🫲🏼", "", U_1FAF1_1F3FE_200D_1FAF2_1F3FC, "1faf1-1f3fe-200d-1faf2-1f3fc.png");
png_name!(HANDSHAKE_TONE4_3, "🫱🏾‍🫲🏽", "", U_1FAF1_1F3FE_200D_1FAF2_1F3FD, "1faf1-1f3fe-200d-1faf2-1f3fd.png");
png_name!(HANDSHAKE_TONE4_5, "🫱🏾‍🫲🏿", "", U_1FAF1_1F3FE_200D_1FAF2_1F3FF, "1faf1-1f3fe-200d-1faf2-1f3ff.png");
png_name!(RIGHTWARDS_HAND_TONE4, "🫱🏾", "", U_1FAF1_1F3FE, "1faf1-1f3fe.png");
png_name!(HANDSHAKE_TONE5_1, "🫱🏿‍🫲🏻", "", U_1FAF1_1F3FF_200D_1FAF2_1F3FB, "1faf1-1f3ff-200d-1faf2-1f3fb.png");
png_name!(HANDSHAKE_TONE5_2, "🫱🏿‍🫲🏼", "", U_1FAF1_1F3FF_200D_1FAF2_1F3FC, "1faf1-1f3ff-200d-1faf2-1f3fc.png");
png_name!(HANDSHAKE_TONE5_3, "🫱🏿‍🫲🏽", "", U_1FAF1_1F3FF_200D_1FAF2_1F3FD, "1faf1-1f3ff-200d-1faf2-1f3fd.png");
png_name!(HANDSHAKE_TONE5_4, "🫱🏿‍🫲🏾", "", U_1FAF1_1F3FF_200D_1FAF2_1F3FE, "1faf1-1f3ff-200d-1faf2-1f3fe.png");
png_name!(RIGHTWARDS_HAND_TONE5, "🫱🏿", "", U_1FAF1_1F3FF, "1faf1-1f3ff.png");
png_name!(RIGHTWARDS_HAND, "🫱", "rightwards hand", U_1FAF1, "1faf1.png");
png_name!(LEFTWARDS_HAND_TONE1, "🫲🏻", "", U_1FAF2_1F3FB, "1faf2-1f3fb.png");
png_name!(LEFTWARDS_HAND_TONE2, "🫲🏼", "", U_1FAF2_1F3FC, "1faf2-1f3fc.png");
png_name!(LEFTWARDS_HAND_TONE3, "🫲🏽", "", U_1FAF2_1F3FD, "1faf2-1f3fd.png");
png_name!(LEFTWARDS_HAND_TONE4, "🫲🏾", "", U_1FAF2_1F3FE, "1faf2-1f3fe.png");
png_name!(LEFTWARDS_HAND_TONE5, "🫲🏿", "", U_1FAF2_1F3FF, "1faf2-1f3ff.png");
png_name!(LEFTWARDS_HAND, "🫲", "leftwards hand", U_1FAF2, "1faf2.png");
png_name!(PALM_DOWN_TONE1, "🫳🏻", "", U_1FAF3_1F3FB, "1faf3-1f3fb.png");
png_name!(PALM_DOWN_TONE2, "🫳🏼", "", U_1FAF3_1F3FC, "1faf3-1f3fc.png");
png_name!(PALM_DOWN_TONE3, "🫳🏽", "", U_1FAF3_1F3FD, "1faf3-1f3fd.png");
png_name!(PALM_DOWN_TONE4, "🫳🏾", "", U_1FAF3_1F3FE, "1faf3-1f3fe.png");
png_name!(PALM_DOWN_TONE5, "🫳🏿", "", U_1FAF3_1F3FF, "1faf3-1f3ff.png");
png_name!(PALM_DOWN, "🫳", "palm down hand", U_1FAF3, "1faf3.png");
png_name!(PALM_UP_TONE1, "🫴🏻", "", U_1FAF4_1F3FB, "1faf4-1f3fb.png");
png_name!(PALM_UP_TONE2, "🫴🏼", "", U_1FAF4_1F3FC, "1faf4-1f3fc.png");
png_name!(PALM_UP_TONE3, "🫴🏽", "", U_1FAF4_1F3FD, "1faf4-1f3fd.png");
png_name!(PALM_UP_TONE4, "🫴🏾", "", U_1FAF4_1F3FE, "1faf4-1f3fe.png");
png_name!(PALM_UP_TONE5, "🫴🏿", "", U_1FAF4_1F3FF, "1faf4-1f3ff.png");
png_name!(PALM_UP, "🫴", "palm up hand", U_1FAF4, "1faf4.png");
png_name!(POINT_FORWARD_TONE1, "🫵🏻", "", U_1FAF5_1F3FB, "1faf5-1f3fb.png");
png_name!(POINT_FORWARD_TONE2, "🫵🏼", "", U_1FAF5_1F3FC, "1faf5-1f3fc.png");
png_name!(POINT_FORWARD_TONE3, "🫵🏽", "", U_1FAF5_1F3FD, "1faf5-1f3fd.png");
png_name!(POINT_FORWARD_TONE4, "🫵🏾", "", U_1FAF5_1F3FE, "1faf5-1f3fe.png");
png_name!(POINT_FORWARD_TONE5, "🫵🏿", "", U_1FAF5_1F3FF, "1faf5-1f3ff.png");
png_name!(POINT_FORWARD, "🫵", "index pointing at the viewer", U_1FAF5, "1faf5.png");
png_name!(HEART_HANDS_TONE1, "🫶🏻", "", U_1FAF6_1F3FB, "1faf6-1f3fb.png");
png_name!(HEART_HANDS_TONE2, "🫶🏼", "", U_1FAF6_1F3FC, "1faf6-1f3fc.png");
png_name!(HEART_HANDS_TONE3, "🫶🏽", "", U_1FAF6_1F3FD, "1faf6-1f3fd.png");
png_name!(HEART_HANDS_TONE4, "🫶🏾", "", U_1FAF6_1F3FE, "1faf6-1f3fe.png");
png_name!(HEART_HANDS_TONE5, "🫶🏿", "", U_1FAF6_1F3FF, "1faf6-1f3ff.png");
png_name!(HEART_HANDS, "🫶", "heart hands", U_1FAF6, "1faf6.png");
png_name!(BANGBANG, "‼", "double exclamation mark", U_203C, "203c.png");
png_name!(DOUBLE_EXCLAMATION, "‼", "double exclamation mark", U_203C, "203c.png");
png_name!(EXCLAMATION_QUESTION, "⁉", "exclamation question mark", U_2049, "2049.png");
png_name!(INTERROBANG, "⁉", "exclamation question mark", U_2049, "2049.png");
png_name!(TM, "™", "trade mark", U_2122, "2122.png");
png_name!(TRADE_MARK, "™", "trade mark", U_2122, "2122.png");
png_name!(INFO, "ℹ", "information", U_2139, "2139.png");
png_name!(INFORMATION_SOURCE, "ℹ", "information", U_2139, "2139.png");
png_name!(LEFT_RIGHT_ARROW, "↔", "left-right arrow", U_2194, "2194.png");
png_name!(ARROW_UP_DOWN, "↕", "up-down arrow", U_2195, "2195.png");
png_name!(ARROW_UPPER_LEFT, "↖", "up-left arrow", U_2196, "2196.png");
png_name!(ARROW_UPPER_RIGHT, "↗", "up-right arrow", U_2197, "2197.png");
png_name!(ARROW_LOWER_RIGHT, "↘", "down-right arrow", U_2198, "2198.png");
png_name!(ARROW_LOWER_LEFT, "↙", "down-left arrow", U_2199, "2199.png");
png_name!(ARROW_LEFT_HOOK, "↩", "right arrow curving left", U_21A9, "21a9.png");
png_name!(LEFTWARDS_ARROW_WITH_HOOK, "↩", "right arrow curving left", U_21A9, "21a9.png");
png_name!(ARROW_RIGHT_HOOK, "↪", "left arrow curving right", U_21AA, "21aa.png");
png_name!(RIGHTWARDS_ARROW_WITH_HOOK, "↪", "left arrow curving right", U_21AA, "21aa.png");
png_name!(WATCH, "⌚", "watch", U_231A, "231a.png");
png_name!(HOURGLASS, "⌛", "hourglass done", U_231B, "231b.png");
png_name!(KEYBOARD, "⌨", "keyboard", U_2328, "2328.png");
png_name!(EJECT, "⏏", "eject button", U_23CF, "23cf.png");
png_name!(FAST_FORWARD, "⏩", "fast-forward button", U_23E9, "23e9.png");
png_name!(FAST_REVERSE, "⏪", "fast reverse button", U_23EA, "23ea.png");
png_name!(REWIND, "⏪", "fast reverse button", U_23EA, "23ea.png");
png_name!(ARROW_DOUBLE_UP, "⏫", "fast up button", U_23EB, "23eb.png");
png_name!(FAST_UP, "⏫", "fast up button", U_23EB, "23eb.png");
png_name!(ARROW_DOUBLE_DOWN, "⏬", "fast down button", U_23EC, "23ec.png");
png_name!(FAST_DOWN, "⏬", "fast down button", U_23EC, "23ec.png");
png_name!(NEXT_TRACK, "⏭", "next track button", U_23ED, "23ed.png");
png_name!(PREVIOUS_TRACK, "⏮", "last track button", U_23EE, "23ee.png");
png_name!(PLAY_PAUSE, "⏯", "play or pause button", U_23EF, "23ef.png");
png_name!(ALARM_CLOCK, "⏰", "alarm clock", U_23F0, "23f0.png");
png_name!(STOPWATCH, "⏱", "stopwatch", U_23F1, "23f1.png");
png_name!(TIMER_CLOCK, "⏲", "timer clock", U_23F2, "23f2.png");
png_name!(HOURGLASS_FLOWING_SAND, "⏳", "hourglass not done", U_23F3, "23f3.png");
png_name!(PAUSE, "⏸", "pause button", U_23F8, "23f8.png");
png_name!(STOP, "⏹", "stop button", U_23F9, "23f9.png");
png_name!(RECORD, "⏺", "record button", U_23FA, "23fa.png");
png_name!(M, "Ⓜ", "circled M", U_24C2, "24c2.png");
png_name!(BLACK_SMALL_SQUARE, "▪", "black small square", U_25AA, "25aa.png");
png_name!(WHITE_SMALL_SQUARE, "▫", "white small square", U_25AB, "25ab.png");
png_name!(ARROW_FORWARD, "▶", "play button", U_25B6, "25b6.png");
png_name!(PLAY, "▶", "play button", U_25B6, "25b6.png");
png_name!(ARROW_BACKWARD, "◀", "reverse button", U_25C0, "25c0.png");
png_name!(REVERSE, "◀", "reverse button", U_25C0, "25c0.png");
png_name!(WHITE_MEDIUM_SQUARE, "◻", "white medium square", U_25FB, "25fb.png");
png_name!(BLACK_MEDIUM_SQUARE, "◼", "black medium square", U_25FC, "25fc.png");
png_name!(WHITE_MEDIUM_SMALL_SQUARE, "◽", "white medium-small square", U_25FD, "25fd.png");
png_name!(BLACK_MEDIUM_SMALL_SQUARE, "◾", "black medium-small square", U_25FE, "25fe.png");
png_name!(SUN, "☀", "sun", U_2600, "2600.png");
png_name!(CLOUD, "☁", "cloud", U_2601, "2601.png");
png_name!(UMBRELLA, "☂", "umbrella", U_2602, "2602.png");
png_name!(SNOWMAN2, "☃", "snowman", U_2603, "2603.png");
png_name!(COMET, "☄", "comet", U_2604, "2604.png");
png_name!(TELEPHONE, "☎", "telephone", U_260E, "260e.png");
png_name!(BALLOT_BOX_WITH_CHECK, "☑", "check box with check", U_2611, "2611.png");
png_name!(UMBRELLA_WITH_RAIN, "☔", "umbrella with rain drops", U_2614, "2614.png");
png_name!(COFFEE, "☕", "hot beverage", U_2615, "2615.png");
png_name!(SHAMROCK, "☘", "shamrock", U_2618, "2618.png");
png_name!(POINT_UP_2_TONE1, "☝🏻", "", U_261D_1F3FB, "261d-1f3fb.png");
png_name!(POINT_UP_2_TONE2, "☝🏼", "", U_261D_1F3FC, "261d-1f3fc.png");
png_name!(POINT_UP_2_TONE3, "☝🏽", "", U_261D_1F3FD, "261d-1f3fd.png");
png_name!(POINT_UP_2_TONE4, "☝🏾", "", U_261D_1F3FE, "261d-1f3fe.png");
png_name!(POINT_UP_2_TONE5, "☝🏿", "", U_261D_1F3FF, "261d-1f3ff.png");
png_name!(POINT_UP_2, "☝", "index pointing up", U_261D, "261d.png");
png_name!(SKULL_AND_CROSSBONES, "☠", "skull and crossbones", U_2620, "2620.png");
png_name!(RADIOACTIVE, "☢", "radioactive", U_2622, "2622.png");
png_name!(BIOHAZARD, "☣", "biohazard", U_2623, "2623.png");
png_name!(ORTHODOX_CROSS, "☦", "orthodox cross", U_2626, "2626.png");
png_name!(STAR_AND_CRESCENT, "☪", "star and crescent", U_262A, "262a.png");
png_name!(PEACE, "☮", "peace symbol", U_262E, "262e.png");
png_name!(PEACE_SYMBOL, "☮", "peace symbol", U_262E, "262e.png");
png_name!(YIN_YANG, "☯", "yin yang", U_262F, "262f.png");
png_name!(WHEEL_OF_DHARMA, "☸", "wheel of dharma", U_2638, "2638.png");
png_name!(WHITE_FROWNING_FACE, "☹", "frowning face", U_2639, "2639.png");
png_name!(RELAXED, "☺", "smiling face", U_263A, "263a.png");
png_name!(SMILING_FACE, "☺", "smiling face", U_263A, "263a.png");
png_name!(FEMALE, "♀", "female sign", U_2640, "2640.png");
png_name!(FEMALE_SIGN, "♀", "female sign", U_2640, "2640.png");
png_name!(MALE, "♂", "male sign", U_2642, "2642.png");
png_name!(MALE_SIGN, "♂", "male sign", U_2642, "2642.png");
png_name!(ARIES, "♈", "Aries", U_2648, "2648.png");
png_name!(TAURUS, "♉", "Taurus", U_2649, "2649.png");
png_name!(GEMINI, "♊", "Gemini", U_264A, "264a.png");
png_name!(CANCER, "♋", "Cancer", U_264B, "264b.png");
png_name!(LEO, "♌", "Leo", U_264C, "264c.png");
png_name!(VIRGO, "♍", "Virgo", U_264D, "264d.png");
png_name!(LIBRA, "♎", "Libra", U_264E, "264e.png");
png_name!(SCORPIUS, "♏", "Scorpio", U_264F, "264f.png");
png_name!(SAGITTARIUS, "♐", "Sagittarius", U_2650, "2650.png");
png_name!(CAPRICORN, "♑", "Capricorn", U_2651, "2651.png");
png_name!(AQUARIUS, "♒", "Aquarius", U_2652, "2652.png");
png_name!(PISCES, "♓", "Pisces", U_2653, "2653.png");
png_name!(CHESS_PAWN, "♟", "chess pawn", U_265F, "265f.png");
png_name!(SPADES, "♠", "spade suit", U_2660, "2660.png");
png_name!(CLUBS, "♣", "club suit", U_2663, "2663.png");
png_name!(HEARTS, "♥", "heart suit", U_2665, "2665.png");
png_name!(DIAMONDS, "♦", "diamond suit", U_2666, "2666.png");
png_name!(HOTSPRINGS, "♨", "hot springs", U_2668, "2668.png");
png_name!(RECYCLE, "♻", "recycling symbol", U_267B, "267b.png");
png_name!(RECYCLING_SYMBOL, "♻", "recycling symbol", U_267B, "267b.png");
png_name!(INFINITY, "♾", "infinity", U_267E, "267e.png");
png_name!(HANDICAPPED, "♿", "wheelchair symbol", U_267F, "267f.png");
png_name!(WHEELCHAIR, "♿", "wheelchair symbol", U_267F, "267f.png");
png_name!(HAMMER_AND_PICK, "⚒", "hammer and pick", U_2692, "2692.png");
png_name!(ANCHOR, "⚓", "anchor", U_2693, "2693.png");
png_name!(CROSSED_SWORDS, "⚔", "crossed swords", U_2694, "2694.png");
png_name!(MEDICAL, "⚕", "medical symbol", U_2695, "2695.png");
png_name!(MEDICAL_SYMBOL, "⚕", "medical symbol", U_2695, "2695.png");
png_name!(SCALES, "⚖", "balance scale", U_2696, "2696.png");
png_name!(ALEMBIC, "⚗", "alembic", U_2697, "2697.png");
png_name!(GEAR, "⚙", "gear", U_2699, "2699.png");
png_name!(ATOM, "⚛", "atom symbol", U_269B, "269b.png");
png_name!(ATOM_SYMBOL, "⚛", "atom symbol", U_269B, "269b.png");
png_name!(FLEUR_DE_LIS, "⚜", "fleur-de-lis", U_269C, "269c.png");
png_name!(WARNING, "⚠", "warning", U_26A0, "26a0.png");
png_name!(HIGH_VOLTAGE, "⚡", "high voltage", U_26A1, "26a1.png");
png_name!(ZAP, "⚡", "high voltage", U_26A1, "26a1.png");
png_name!(TRANSGENDER_SYMBOL, "⚧", "transgender symbol", U_26A7, "26a7.png");
png_name!(WHITE_CIRCLE, "⚪", "white circle", U_26AA, "26aa.png");
png_name!(BLACK_CIRCLE, "⚫", "black circle", U_26AB, "26ab.png");
png_name!(COFFIN, "⚰", "coffin", U_26B0, "26b0.png");
png_name!(FUNERAL_URN, "⚱", "funeral urn", U_26B1, "26b1.png");
png_name!(SOCCER, "⚽", "soccer ball", U_26BD, "26bd.png");
png_name!(BASEBALL, "⚾", "baseball", U_26BE, "26be.png");
png_name!(SNOWMAN, "⛄", "snowman without snow", U_26C4, "26c4.png");
png_name!(PARTLY_SUNNY, "⛅", "sun behind cloud", U_26C5, "26c5.png");
png_name!(SUN_BEHIND_CLOUD, "⛅", "sun behind cloud", U_26C5, "26c5.png");
png_name!(STORMY, "⛈", "cloud with lightning and rain", U_26C8, "26c8.png");
png_name!(THUNDER_CLOUD_AND_RAIN, "⛈", "cloud with lightning and rain", U_26C8, "26c8.png");
png_name!(OPHIUCHUS, "⛎", "Ophiuchus", U_26CE, "26ce.png");
png_name!(PICK, "⛏", "pick", U_26CF, "26cf.png");
png_name!(HELMET_WITH_CROSS, "⛑", "rescue worker’s helmet", U_26D1, "26d1.png");
png_name!(RESCUE_WORKER_HELMET, "⛑", "rescue worker’s helmet", U_26D1, "26d1.png");
png_name!(CHAINS, "⛓", "chains", U_26D3, "26d3.png");
png_name!(NO_ENTRY, "⛔", "no entry", U_26D4, "26d4.png");
png_name!(SHINTO_SHRINE, "⛩", "shinto shrine", U_26E9, "26e9.png");
png_name!(CHURCH, "⛪", "church", U_26EA, "26ea.png");
png_name!(MOUNTAIN, "⛰", "mountain", U_26F0, "26f0.png");
png_name!(BEACH_UMBRELLA, "⛱", "umbrella on ground", U_26F1, "26f1.png");
png_name!(UMBRELLA_ON_GROUND, "⛱", "umbrella on ground", U_26F1, "26f1.png");
png_name!(FOUNTAIN, "⛲", "fountain", U_26F2, "26f2.png");
png_name!(GOLF, "⛳", "flag in hole", U_26F3, "26f3.png");
png_name!(FERRY, "⛴", "ferry", U_26F4, "26f4.png");
png_name!(SAILBOAT, "⛵", "sailboat", U_26F5, "26f5.png");
png_name!(PERSON_SKIING, "⛷", "skier", U_26F7, "26f7.png");
png_name!(SKIER, "⛷", "skier", U_26F7, "26f7.png");
png_name!(SKIING, "⛷", "skier", U_26F7, "26f7.png");
png_name!(ICE_SKATE, "⛸", "ice skate", U_26F8, "26f8.png");
png_name!(WOMAN_BOUNCING_BALL_TONE1, "⛹🏻‍♀️", "", U_26F9_1F3FB_200D_2640_FE0F, "26f9-1f3fb-200d-2640-fe0f.png");
png_name!(MAN_BOUNCING_BALL_TONE1, "⛹🏻‍♂️", "", U_26F9_1F3FB_200D_2642_FE0F, "26f9-1f3fb-200d-2642-fe0f.png");
png_name!(PERSON_BOUNCING_BALL_TONE1, "⛹🏻", "", U_26F9_1F3FB, "26f9-1f3fb.png");
png_name!(WOMAN_BOUNCING_BALL_TONE2, "⛹🏼‍♀️", "", U_26F9_1F3FC_200D_2640_FE0F, "26f9-1f3fc-200d-2640-fe0f.png");
png_name!(MAN_BOUNCING_BALL_TONE2, "⛹🏼‍♂️", "", U_26F9_1F3FC_200D_2642_FE0F, "26f9-1f3fc-200d-2642-fe0f.png");
png_name!(PERSON_BOUNCING_BALL_TONE2, "⛹🏼", "", U_26F9_1F3FC, "26f9-1f3fc.png");
png_name!(WOMAN_BOUNCING_BALL_TONE3, "⛹🏽‍♀️", "", U_26F9_1F3FD_200D_2640_FE0F, "26f9-1f3fd-200d-2640-fe0f.png");
png_name!(MAN_BOUNCING_BALL_TONE3, "⛹🏽‍♂️", "", U_26F9_1F3FD_200D_2642_FE0F, "26f9-1f3fd-200d-2642-fe0f.png");
png_name!(PERSON_BOUNCING_BALL_TONE3, "⛹🏽", "", U_26F9_1F3FD, "26f9-1f3fd.png");
png_name!(WOMAN_BOUNCING_BALL_TONE4, "⛹🏾‍♀️", "", U_26F9_1F3FE_200D_2640_FE0F, "26f9-1f3fe-200d-2640-fe0f.png");
png_name!(MAN_BOUNCING_BALL_TONE4, "⛹🏾‍♂️", "", U_26F9_1F3FE_200D_2642_FE0F, "26f9-1f3fe-200d-2642-fe0f.png");
png_name!(PERSON_BOUNCING_BALL_TONE4, "⛹🏾", "", U_26F9_1F3FE, "26f9-1f3fe.png");
png_name!(WOMAN_BOUNCING_BALL_TONE5, "⛹🏿‍♀️", "", U_26F9_1F3FF_200D_2640_FE0F, "26f9-1f3ff-200d-2640-fe0f.png");
png_name!(MAN_BOUNCING_BALL_TONE5, "⛹🏿‍♂️", "", U_26F9_1F3FF_200D_2642_FE0F, "26f9-1f3ff-200d-2642-fe0f.png");
png_name!(PERSON_BOUNCING_BALL_TONE5, "⛹🏿", "", U_26F9_1F3FF, "26f9-1f3ff.png");
png_name!(WOMAN_BOUNCING_BALL, "⛹️‍♀️", "woman bouncing ball", U_26F9_FE0F_200D_2640_FE0F, "26f9-fe0f-200d-2640-fe0f.png");
png_name!(MAN_BOUNCING_BALL, "⛹️‍♂️", "man bouncing ball", U_26F9_FE0F_200D_2642_FE0F, "26f9-fe0f-200d-2642-fe0f.png");
png_name!(PERSON_BOUNCING_BALL, "⛹", "person bouncing ball", U_26F9, "26f9.png");
png_name!(TENT, "⛺", "tent", U_26FA, "26fa.png");
png_name!(FUELPUMP, "⛽", "fuel pump", U_26FD, "26fd.png");
png_name!(SCISSORS, "✂", "scissors", U_2702, "2702.png");
png_name!(CHECK_MARK_BUTTON, "✅", "check mark button", U_2705, "2705.png");
png_name!(WHITE_CHECK_MARK, "✅", "check mark button", U_2705, "2705.png");
png_name!(AIRPLANE, "✈", "airplane", U_2708, "2708.png");
png_name!(ENVELOPE, "✉", "envelope", U_2709, "2709.png");
png_name!(FIST_TONE1, "✊🏻", "", U_270A_1F3FB, "270a-1f3fb.png");
png_name!(FIST_TONE2, "✊🏼", "", U_270A_1F3FC, "270a-1f3fc.png");
png_name!(FIST_TONE3, "✊🏽", "", U_270A_1F3FD, "270a-1f3fd.png");
png_name!(FIST_TONE4, "✊🏾", "", U_270A_1F3FE, "270a-1f3fe.png");
png_name!(FIST_TONE5, "✊🏿", "", U_270A_1F3FF, "270a-1f3ff.png");
png_name!(FIST, "✊", "raised fist", U_270A, "270a.png");
png_name!(HIGH_FIVE_TONE1, "✋🏻", "", U_270B_1F3FB, "270b-1f3fb.png");
png_name!(RAISED_HAND_TONE1, "✋🏻", "", U_270B_1F3FB, "270b-1f3fb.png");
png_name!(HIGH_FIVE_TONE2, "✋🏼", "", U_270B_1F3FC, "270b-1f3fc.png");
png_name!(RAISED_HAND_TONE2, "✋🏼", "", U_270B_1F3FC, "270b-1f3fc.png");
png_name!(HIGH_FIVE_TONE3, "✋🏽", "", U_270B_1F3FD, "270b-1f3fd.png");
png_name!(RAISED_HAND_TONE3, "✋🏽", "", U_270B_1F3FD, "270b-1f3fd.png");
png_name!(HIGH_FIVE_TONE4, "✋🏾", "", U_270B_1F3FE, "270b-1f3fe.png");
png_name!(RAISED_HAND_TONE4, "✋🏾", "", U_270B_1F3FE, "270b-1f3fe.png");
png_name!(HIGH_FIVE_TONE5, "✋🏿", "", U_270B_1F3FF, "270b-1f3ff.png");
png_name!(RAISED_HAND_TONE5, "✋🏿", "", U_270B_1F3FF, "270b-1f3ff.png");
png_name!(HIGH_FIVE, "✋", "raised hand", U_270B, "270b.png");
png_name!(RAISED_HAND, "✋", "raised hand", U_270B, "270b.png");
png_name!(V_TONE1, "✌🏻", "", U_270C_1F3FB, "270c-1f3fb.png");
png_name!(VICTORY_TONE1, "✌🏻", "", U_270C_1F3FB, "270c-1f3fb.png");
png_name!(V_TONE2, "✌🏼", "", U_270C_1F3FC, "270c-1f3fc.png");
png_name!(VICTORY_TONE2, "✌🏼", "", U_270C_1F3FC, "270c-1f3fc.png");
png_name!(V_TONE3, "✌🏽", "", U_270C_1F3FD, "270c-1f3fd.png");
png_name!(VICTORY_TONE3, "✌🏽", "", U_270C_1F3FD, "270c-1f3fd.png");
png_name!(V_TONE4, "✌🏾", "", U_270C_1F3FE, "270c-1f3fe.png");
png_name!(VICTORY_TONE4, "✌🏾", "", U_270C_1F3FE, "270c-1f3fe.png");
png_name!(V_TONE5, "✌🏿", "", U_270C_1F3FF, "270c-1f3ff.png");
png_name!(VICTORY_TONE5, "✌🏿", "", U_270C_1F3FF, "270c-1f3ff.png");
png_name!(V, "✌", "victory hand", U_270C, "270c.png");
png_name!(VICTORY, "✌", "victory hand", U_270C, "270c.png");
png_name!(WRITING_HAND_TONE1, "✍🏻", "", U_270D_1F3FB, "270d-1f3fb.png");
png_name!(WRITING_HAND_TONE2, "✍🏼", "", U_270D_1F3FC, "270d-1f3fc.png");
png_name!(WRITING_HAND_TONE3, "✍🏽", "", U_270D_1F3FD, "270d-1f3fd.png");
png_name!(WRITING_HAND_TONE4, "✍🏾", "", U_270D_1F3FE, "270d-1f3fe.png");
png_name!(WRITING_HAND_TONE5, "✍🏿", "", U_270D_1F3FF, "270d-1f3ff.png");
png_name!(WRITING_HAND, "✍", "writing hand", U_270D, "270d.png");
png_name!(PENCIL, "✏", "pencil", U_270F, "270f.png");
png_name!(BLACK_NIB, "✒", "black nib", U_2712, "2712.png");
png_name!(CHECK_MARK, "✔", "check mark", U_2714, "2714.png");
png_name!(HEAVY_CHECK_MARK, "✔", "check mark", U_2714, "2714.png");
png_name!(MULTIPLICATION, "✖", "multiply", U_2716, "2716.png");
png_name!(MULTIPLY, "✖", "multiply", U_2716, "2716.png");
png_name!(LATIN_CROSS, "✝", "latin cross", U_271D, "271d.png");
png_name!(STAR_OF_DAVID, "✡", "star of David", U_2721, "2721.png");
png_name!(SPARKLES, "✨", "sparkles", U_2728, "2728.png");
png_name!(EIGHT_SPOKED_ASTERISK, "✳", "eight-spoked asterisk", U_2733, "2733.png");
png_name!(EIGHT_POINTED_BLACK_STAR, "✴", "eight-pointed star", U_2734, "2734.png");
png_name!(SNOWFLAKE, "❄", "snowflake", U_2744, "2744.png");
png_name!(SPARKLE, "❇", "sparkle", U_2747, "2747.png");
png_name!(CROSS_MARK, "❌", "cross mark", U_274C, "274c.png");
png_name!(X, "❌", "cross mark", U_274C, "274c.png");
png_name!(CROSS_MARK_BUTTON, "❎", "cross mark button", U_274E, "274e.png");
png_name!(NEGATIVE_SQUARED_CROSS_MARK, "❎", "cross mark button", U_274E, "274e.png");
png_name!(QUESTION, "❓", "red question mark", U_2753, "2753.png");
png_name!(WHITE_QUESTION, "❔", "white question mark", U_2754, "2754.png");
png_name!(WHITE_EXCLAMATION, "❕", "white exclamation mark", U_2755, "2755.png");
png_name!(EXCLAMATION, "❗", "red exclamation mark", U_2757, "2757.png");
png_name!(HEART_EXCLAMATION, "❣", "heart exclamation", U_2763, "2763.png");
png_name!(HEART_ON_FIRE, "❤️‍🔥", "heart on fire", U_2764_FE0F_200D_1F525, "2764-fe0f-200d-1f525.png");
png_name!(MENDING_HEART, "❤️‍🩹", "mending heart", U_2764_FE0F_200D_1FA79, "2764-fe0f-200d-1fa79.png");
png_name!(HEART, "❤", "red heart", U_2764, "2764.png");
png_name!(RED_HEART, "❤", "red heart", U_2764, "2764.png");
png_name!(PLUS, "➕", "plus", U_2795, "2795.png");
png_name!(MINUS, "➖", "minus", U_2796, "2796.png");
png_name!(DIVIDE, "➗", "divide", U_2797, "2797.png");
png_name!(DIVISION, "➗", "divide", U_2797, "2797.png");
png_name!(ARROW_RIGHT, "➡", "right arrow", U_27A1, "27a1.png");
png_name!(CURLY_LOOP, "➰", "curly loop", U_27B0, "27b0.png");
png_name!(DOUBLE_CURLY_LOOP, "➿", "double curly loop", U_27BF, "27bf.png");
png_name!(LOOP, "➿", "double curly loop", U_27BF, "27bf.png");
png_name!(ARROW_HEADING_UP, "⤴", "right arrow curving up", U_2934, "2934.png");
png_name!(ARROW_HEADING_DOWN, "⤵", "right arrow curving down", U_2935, "2935.png");
png_name!(ARROW_LEFT, "⬅", "left arrow", U_2B05, "2b05.png");
png_name!(ARROW_UP, "⬆", "up arrow", U_2B06, "2b06.png");
png_name!(ARROW_DOWN, "⬇", "down arrow", U_2B07, "2b07.png");
png_name!(BLACK_LARGE_SQUARE, "⬛", "black large square", U_2B1B, "2b1b.png");
png_name!(WHITE_LARGE_SQUARE, "⬜", "white large square", U_2B1C, "2b1c.png");
png_name!(STAR, "⭐", "star", U_2B50, "2b50.png");
png_name!(HOLLOW_RED_CIRCLE, "⭕", "hollow red circle", U_2B55, "2b55.png");
png_name!(RED_O, "⭕", "hollow red circle", U_2B55, "2b55.png");
png_name!(WAVY_DASH, "〰", "wavy dash", U_3030, "3030.png");
png_name!(PART_ALTERNATION_MARK, "〽", "part alternation mark", U_303D, "303d.png");
png_name!(CONGRATULATIONS, "㊗", "Japanese “congratulations” button", U_3297, "3297.png");
png_name!(JA_CONGRATULATIONS, "㊗", "Japanese “congratulations” button", U_3297, "3297.png");
png_name!(JA_SECRET, "㊙", "Japanese “secret” button", U_3299, "3299.png");
png_name!(SECRET, "㊙", "Japanese “secret” button", U_3299, "3299.png");

png_match_name! [
    ((7, "mahjong"), MAHJONG),
    ((11, "black_joker"), BLACK_JOKER),
    ((1, "a"), A),
    ((7, "a_blood"), A_BLOOD),
    ((1, "b"), B),
    ((7, "b_blood"), B_BLOOD),
    ((1, "o"), O),
    ((7, "o_blood"), O_BLOOD),
    ((7, "parking"), PARKING),
    ((2, "ab"), AB),
    ((8, "ab_blood"), AB_BLOOD),
    ((2, "cl"), CL),
    ((4, "cool"), COOL),
    ((4, "free"), FREE),
    ((2, "id"), ID),
    ((3, "new"), NEW),
    ((2, "ng"), NG),
    ((2, "ok"), OK),
    ((3, "sos"), SOS),
    ((3, "up2"), UP2),
    ((2, "vs"), VS),
    ((16, "ascension_island"), ASCENSION_ISLAND),
    ((7, "flag_ac"), FLAG_AC),
    ((7, "andorra"), ANDORRA),
    ((7, "flag_ad"), FLAG_AD),
    ((7, "flag_ae"), FLAG_AE),
    ((20, "united_arab_emirates"), UNITED_ARAB_EMIRATES),
    ((11, "afghanistan"), AFGHANISTAN),
    ((7, "flag_af"), FLAG_AF),
    ((15, "antigua_barbuda"), ANTIGUA_BARBUDA),
    ((7, "flag_ag"), FLAG_AG),
    ((8, "anguilla"), ANGUILLA),
    ((7, "flag_ai"), FLAG_AI),
    ((7, "albania"), ALBANIA),
    ((7, "flag_al"), FLAG_AL),
    ((7, "armenia"), ARMENIA),
    ((7, "flag_am"), FLAG_AM),
    ((6, "angola"), ANGOLA),
    ((7, "flag_ao"), FLAG_AO),
    ((10, "antarctica"), ANTARCTICA),
    ((7, "flag_aq"), FLAG_AQ),
    ((9, "argentina"), ARGENTINA),
    ((7, "flag_ar"), FLAG_AR),
    ((14, "american_samoa"), AMERICAN_SAMOA),
    ((7, "flag_as"), FLAG_AS),
    ((7, "austria"), AUSTRIA),
    ((7, "flag_at"), FLAG_AT),
    ((9, "australia"), AUSTRALIA),
    ((7, "flag_au"), FLAG_AU),
    ((5, "aruba"), ARUBA),
    ((7, "flag_aw"), FLAG_AW),
    ((13, "aland_islands"), ALAND_ISLANDS),
    ((7, "flag_ax"), FLAG_AX),
    ((10, "azerbaijan"), AZERBAIJAN),
    ((7, "flag_az"), FLAG_AZ),
    ((20, "regional_indicator_a"), REGIONAL_INDICATOR_A),
    ((18, "bosnia_herzegovina"), BOSNIA_HERZEGOVINA),
    ((7, "flag_ba"), FLAG_BA),
    ((8, "barbados"), BARBADOS),
    ((7, "flag_bb"), FLAG_BB),
    ((10, "bangladesh"), BANGLADESH),
    ((7, "flag_bd"), FLAG_BD),
    ((7, "belgium"), BELGIUM),
    ((7, "flag_be"), FLAG_BE),
    ((12, "burkina_faso"), BURKINA_FASO),
    ((7, "flag_bf"), FLAG_BF),
    ((8, "bulgaria"), BULGARIA),
    ((7, "flag_bg"), FLAG_BG),
    ((7, "bahrain"), BAHRAIN),
    ((7, "flag_bh"), FLAG_BH),
    ((7, "burundi"), BURUNDI),
    ((7, "flag_bi"), FLAG_BI),
    ((5, "benin"), BENIN),
    ((7, "flag_bj"), FLAG_BJ),
    ((7, "flag_bl"), FLAG_BL),
    ((13, "st_barthelemy"), ST_BARTHELEMY),
    ((7, "bermuda"), BERMUDA),
    ((7, "flag_bm"), FLAG_BM),
    ((6, "brunei"), BRUNEI),
    ((7, "flag_bn"), FLAG_BN),
    ((7, "bolivia"), BOLIVIA),
    ((7, "flag_bo"), FLAG_BO),
    ((21, "caribbean_netherlands"), CARIBBEAN_NETHERLANDS),
    ((7, "flag_bq"), FLAG_BQ),
    ((6, "brazil"), BRAZIL),
    ((7, "flag_br"), FLAG_BR),
    ((7, "bahamas"), BAHAMAS),
    ((7, "flag_bs"), FLAG_BS),
    ((6, "bhutan"), BHUTAN),
    ((7, "flag_bt"), FLAG_BT),
    ((13, "bouvet_island"), BOUVET_ISLAND),
    ((7, "flag_bv"), FLAG_BV),
    ((8, "botswana"), BOTSWANA),
    ((7, "flag_bw"), FLAG_BW),
    ((7, "belarus"), BELARUS),
    ((7, "flag_by"), FLAG_BY),
    ((6, "belize"), BELIZE),
    ((7, "flag_bz"), FLAG_BZ),
    ((20, "regional_indicator_b"), REGIONAL_INDICATOR_B),
    ((6, "canada"), CANADA),
    ((7, "flag_ca"), FLAG_CA),
    ((13, "cocos_islands"), COCOS_ISLANDS),
    ((7, "flag_cc"), FLAG_CC),
    ((14, "congo_kinshasa"), CONGO_KINSHASA),
    ((7, "flag_cd"), FLAG_CD),
    ((24, "central_african_republic"), CENTRAL_AFRICAN_REPUBLIC),
    ((7, "flag_cf"), FLAG_CF),
    ((17, "congo_brazzaville"), CONGO_BRAZZAVILLE),
    ((7, "flag_cg"), FLAG_CG),
    ((7, "flag_ch"), FLAG_CH),
    ((11, "switzerland"), SWITZERLAND),
    ((12, "cote_divoire"), COTE_DIVOIRE),
    ((7, "flag_ci"), FLAG_CI),
    ((12, "cook_islands"), COOK_ISLANDS),
    ((7, "flag_ck"), FLAG_CK),
    ((5, "chile"), CHILE),
    ((7, "flag_cl"), FLAG_CL),
    ((8, "cameroon"), CAMEROON),
    ((7, "flag_cm"), FLAG_CM),
    ((5, "china"), CHINA),
    ((7, "flag_cn"), FLAG_CN),
    ((8, "colombia"), COLOMBIA),
    ((7, "flag_co"), FLAG_CO),
    ((17, "clipperton_island"), CLIPPERTON_ISLAND),
    ((7, "flag_cp"), FLAG_CP),
    ((10, "costa_rica"), COSTA_RICA),
    ((7, "flag_cr"), FLAG_CR),
    ((4, "cuba"), CUBA),
    ((7, "flag_cu"), FLAG_CU),
    ((10, "cape_verde"), CAPE_VERDE),
    ((7, "flag_cv"), FLAG_CV),
    ((7, "curacao"), CURACAO),
    ((7, "flag_cw"), FLAG_CW),
    ((16, "christmas_island"), CHRISTMAS_ISLAND),
    ((7, "flag_cx"), FLAG_CX),
    ((6, "cyprus"), CYPRUS),
    ((7, "flag_cy"), FLAG_CY),
    ((14, "czech_republic"), CZECH_REPUBLIC),
    ((7, "czechia"), CZECHIA),
    ((7, "flag_cz"), FLAG_CZ),
    ((20, "regional_indicator_c"), REGIONAL_INDICATOR_C),
    ((7, "flag_de"), FLAG_DE),
    ((7, "germany"), GERMANY),
    ((12, "diego_garcia"), DIEGO_GARCIA),
    ((7, "flag_dg"), FLAG_DG),
    ((8, "djibouti"), DJIBOUTI),
    ((7, "flag_dj"), FLAG_DJ),
    ((7, "denmark"), DENMARK),
    ((7, "flag_dk"), FLAG_DK),
    ((8, "dominica"), DOMINICA),
    ((7, "flag_dm"), FLAG_DM),
    ((18, "dominican_republic"), DOMINICAN_REPUBLIC),
    ((7, "flag_do"), FLAG_DO),
    ((7, "algeria"), ALGERIA),
    ((7, "flag_dz"), FLAG_DZ),
    ((20, "regional_indicator_d"), REGIONAL_INDICATOR_D),
    ((13, "ceuta_melilla"), CEUTA_MELILLA),
    ((7, "flag_ea"), FLAG_EA),
    ((7, "ecuador"), ECUADOR),
    ((7, "flag_ec"), FLAG_EC),
    ((7, "estonia"), ESTONIA),
    ((7, "flag_ee"), FLAG_EE),
    ((5, "egypt"), EGYPT),
    ((7, "flag_eg"), FLAG_EG),
    ((7, "flag_eh"), FLAG_EH),
    ((14, "western_sahara"), WESTERN_SAHARA),
    ((7, "eritrea"), ERITREA),
    ((7, "flag_er"), FLAG_ER),
    ((7, "flag_es"), FLAG_ES),
    ((5, "spain"), SPAIN),
    ((8, "ethiopia"), ETHIOPIA),
    ((7, "flag_et"), FLAG_ET),
    ((14, "european_union"), EUROPEAN_UNION),
    ((7, "flag_eu"), FLAG_EU),
    ((20, "regional_indicator_e"), REGIONAL_INDICATOR_E),
    ((7, "finland"), FINLAND),
    ((7, "flag_fi"), FLAG_FI),
    ((4, "fiji"), FIJI),
    ((7, "flag_fj"), FLAG_FJ),
    ((16, "falkland_islands"), FALKLAND_ISLANDS),
    ((7, "flag_fk"), FLAG_FK),
    ((7, "flag_fm"), FLAG_FM),
    ((10, "micronesia"), MICRONESIA),
    ((13, "faroe_islands"), FAROE_ISLANDS),
    ((7, "flag_fo"), FLAG_FO),
    ((7, "flag_fr"), FLAG_FR),
    ((6, "france"), FRANCE),
    ((20, "regional_indicator_f"), REGIONAL_INDICATOR_F),
    ((7, "flag_ga"), FLAG_GA),
    ((5, "gabon"), GABON),
    ((7, "flag_gb"), FLAG_GB),
    ((2, "uk"), UK),
    ((14, "united_kingdom"), UNITED_KINGDOM),
    ((7, "flag_gd"), FLAG_GD),
    ((7, "grenada"), GRENADA),
    ((7, "flag_ge"), FLAG_GE),
    ((7, "georgia"), GEORGIA),
    ((7, "flag_gf"), FLAG_GF),
    ((13, "french_guiana"), FRENCH_GUIANA),
    ((7, "flag_gg"), FLAG_GG),
    ((8, "guernsey"), GUERNSEY),
    ((7, "flag_gh"), FLAG_GH),
    ((5, "ghana"), GHANA),
    ((7, "flag_gi"), FLAG_GI),
    ((9, "gibraltar"), GIBRALTAR),
    ((7, "flag_gl"), FLAG_GL),
    ((9, "greenland"), GREENLAND),
    ((7, "flag_gm"), FLAG_GM),
    ((6, "gambia"), GAMBIA),
    ((7, "flag_gn"), FLAG_GN),
    ((6, "guinea"), GUINEA),
    ((7, "flag_gp"), FLAG_GP),
    ((10, "guadeloupe"), GUADELOUPE),
    ((17, "equatorial_guinea"), EQUATORIAL_GUINEA),
    ((7, "flag_gq"), FLAG_GQ),
    ((7, "flag_gr"), FLAG_GR),
    ((6, "greece"), GREECE),
    ((7, "flag_gs"), FLAG_GS),
    ((36, "south_georgia_south_sandwich_islands"), SOUTH_GEORGIA_SOUTH_SANDWICH_ISLANDS),
    ((7, "flag_gt"), FLAG_GT),
    ((9, "guatemala"), GUATEMALA),
    ((7, "flag_gu"), FLAG_GU),
    ((4, "guam"), GUAM),
    ((7, "flag_gw"), FLAG_GW),
    ((13, "guinea_bissau"), GUINEA_BISSAU),
    ((7, "flag_gy"), FLAG_GY),
    ((6, "guyana"), GUYANA),
    ((20, "regional_indicator_g"), REGIONAL_INDICATOR_G),
    ((7, "flag_hk"), FLAG_HK),
    ((9, "hong_kong"), HONG_KONG),
    ((7, "flag_hm"), FLAG_HM),
    ((22, "heard_mcdonald_islands"), HEARD_MCDONALD_ISLANDS),
    ((7, "flag_hn"), FLAG_HN),
    ((8, "honduras"), HONDURAS),
    ((7, "croatia"), CROATIA),
    ((7, "flag_hr"), FLAG_HR),
    ((7, "flag_ht"), FLAG_HT),
    ((5, "haiti"), HAITI),
    ((7, "flag_hu"), FLAG_HU),
    ((7, "hungary"), HUNGARY),
    ((20, "regional_indicator_h"), REGIONAL_INDICATOR_H),
    ((14, "canary_islands"), CANARY_ISLANDS),
    ((7, "flag_ic"), FLAG_IC),
    ((7, "flag_id"), FLAG_ID),
    ((9, "indonesia"), INDONESIA),
    ((7, "flag_ie"), FLAG_IE),
    ((7, "ireland"), IRELAND),
    ((7, "flag_il"), FLAG_IL),
    ((6, "israel"), ISRAEL),
    ((7, "flag_im"), FLAG_IM),
    ((11, "isle_of_man"), ISLE_OF_MAN),
    ((7, "flag_in"), FLAG_IN),
    ((5, "india"), INDIA),
    ((30, "british_indian_ocean_territory"), BRITISH_INDIAN_OCEAN_TERRITORY),
    ((7, "flag_io"), FLAG_IO),
    ((7, "flag_iq"), FLAG_IQ),
    ((4, "iraq"), IRAQ),
    ((7, "flag_ir"), FLAG_IR),
    ((4, "iran"), IRAN),
    ((7, "flag_is"), FLAG_IS),
    ((7, "iceland"), ICELAND),
    ((7, "flag_it"), FLAG_IT),
    ((5, "italy"), ITALY),
    ((20, "regional_indicator_i"), REGIONAL_INDICATOR_I),
    ((7, "flag_je"), FLAG_JE),
    ((6, "jersey"), JERSEY),
    ((7, "flag_jm"), FLAG_JM),
    ((7, "jamaica"), JAMAICA),
    ((7, "flag_jo"), FLAG_JO),
    ((6, "jordan"), JORDAN),
    ((7, "flag_jp"), FLAG_JP),
    ((5, "japan"), JAPAN),
    ((20, "regional_indicator_j"), REGIONAL_INDICATOR_J),
    ((7, "flag_ke"), FLAG_KE),
    ((5, "kenya"), KENYA),
    ((7, "flag_kg"), FLAG_KG),
    ((10, "kyrgyzstan"), KYRGYZSTAN),
    ((8, "cambodia"), CAMBODIA),
    ((7, "flag_kh"), FLAG_KH),
    ((7, "flag_ki"), FLAG_KI),
    ((8, "kiribati"), KIRIBATI),
    ((7, "comoros"), COMOROS),
    ((7, "flag_km"), FLAG_KM),
    ((7, "flag_kn"), FLAG_KN),
    ((14, "st_kitts_nevis"), ST_KITTS_NEVIS),
    ((7, "flag_kp"), FLAG_KP),
    ((11, "north_korea"), NORTH_KOREA),
    ((7, "flag_kr"), FLAG_KR),
    ((11, "south_korea"), SOUTH_KOREA),
    ((7, "flag_kw"), FLAG_KW),
    ((6, "kuwait"), KUWAIT),
    ((14, "cayman_islands"), CAYMAN_ISLANDS),
    ((7, "flag_ky"), FLAG_KY),
    ((7, "flag_kz"), FLAG_KZ),
    ((10, "kazakhstan"), KAZAKHSTAN),
    ((20, "regional_indicator_k"), REGIONAL_INDICATOR_K),
    ((7, "flag_la"), FLAG_LA),
    ((4, "laos"), LAOS),
    ((7, "flag_lb"), FLAG_LB),
    ((7, "lebanon"), LEBANON),
    ((7, "flag_lc"), FLAG_LC),
    ((8, "st_lucia"), ST_LUCIA),
    ((7, "flag_li"), FLAG_LI),
    ((13, "liechtenstein"), LIECHTENSTEIN),
    ((7, "flag_lk"), FLAG_LK),
    ((9, "sri_lanka"), SRI_LANKA),
    ((7, "flag_lr"), FLAG_LR),
    ((7, "liberia"), LIBERIA),
    ((7, "flag_ls"), FLAG_LS),
    ((7, "lesotho"), LESOTHO),
    ((7, "flag_lt"), FLAG_LT),
    ((9, "lithuania"), LITHUANIA),
    ((7, "flag_lu"), FLAG_LU),
    ((10, "luxembourg"), LUXEMBOURG),
    ((7, "flag_lv"), FLAG_LV),
    ((6, "latvia"), LATVIA),
    ((7, "flag_ly"), FLAG_LY),
    ((5, "libya"), LIBYA),
    ((20, "regional_indicator_l"), REGIONAL_INDICATOR_L),
    ((7, "flag_ma"), FLAG_MA),
    ((7, "morocco"), MOROCCO),
    ((7, "flag_mc"), FLAG_MC),
    ((6, "monaco"), MONACO),
    ((7, "flag_md"), FLAG_MD),
    ((7, "moldova"), MOLDOVA),
    ((7, "flag_me"), FLAG_ME),
    ((10, "montenegro"), MONTENEGRO),
    ((7, "flag_mf"), FLAG_MF),
    ((9, "st_martin"), ST_MARTIN),
    ((7, "flag_mg"), FLAG_MG),
    ((10, "madagascar"), MADAGASCAR),
    ((7, "flag_mh"), FLAG_MH),
    ((16, "marshall_islands"), MARSHALL_ISLANDS),
    ((7, "flag_mk"), FLAG_MK),
    ((9, "macedonia"), MACEDONIA),
    ((7, "flag_ml"), FLAG_ML),
    ((4, "mali"), MALI),
    ((5, "burma"), BURMA),
    ((7, "flag_mm"), FLAG_MM),
    ((7, "myanmar"), MYANMAR),
    ((7, "flag_mn"), FLAG_MN),
    ((8, "mongolia"), MONGOLIA),
    ((7, "flag_mo"), FLAG_MO),
    ((5, "macao"), MACAO),
    ((5, "macau"), MACAU),
    ((7, "flag_mp"), FLAG_MP),
    ((24, "northern_mariana_islands"), NORTHERN_MARIANA_ISLANDS),
    ((7, "flag_mq"), FLAG_MQ),
    ((10, "martinique"), MARTINIQUE),
    ((7, "flag_mr"), FLAG_MR),
    ((10, "mauritania"), MAURITANIA),
    ((7, "flag_ms"), FLAG_MS),
    ((10, "montserrat"), MONTSERRAT),
    ((7, "flag_mt"), FLAG_MT),
    ((5, "malta"), MALTA),
    ((7, "flag_mu"), FLAG_MU),
    ((9, "mauritius"), MAURITIUS),
    ((7, "flag_mv"), FLAG_MV),
    ((8, "maldives"), MALDIVES),
    ((7, "flag_mw"), FLAG_MW),
    ((6, "malawi"), MALAWI),
    ((7, "flag_mx"), FLAG_MX),
    ((6, "mexico"), MEXICO),
    ((7, "flag_my"), FLAG_MY),
    ((8, "malaysia"), MALAYSIA),
    ((7, "flag_mz"), FLAG_MZ),
    ((10, "mozambique"), MOZAMBIQUE),
    ((20, "regional_indicator_m"), REGIONAL_INDICATOR_M),
    ((7, "flag_na"), FLAG_NA),
    ((7, "namibia"), NAMIBIA),
    ((7, "flag_nc"), FLAG_NC),
    ((13, "new_caledonia"), NEW_CALEDONIA),
    ((7, "flag_ne"), FLAG_NE),
    ((5, "niger"), NIGER),
    ((7, "flag_nf"), FLAG_NF),
    ((14, "norfolk_island"), NORFOLK_ISLAND),
    ((7, "flag_ng"), FLAG_NG),
    ((7, "nigeria"), NIGERIA),
    ((7, "flag_ni"), FLAG_NI),
    ((9, "nicaragua"), NICARAGUA),
    ((7, "flag_nl"), FLAG_NL),
    ((11, "netherlands"), NETHERLANDS),
    ((7, "flag_no"), FLAG_NO),
    ((6, "norway"), NORWAY),
    ((7, "flag_np"), FLAG_NP),
    ((5, "nepal"), NEPAL),
    ((7, "flag_nr"), FLAG_NR),
    ((5, "nauru"), NAURU),
    ((7, "flag_nu"), FLAG_NU),
    ((4, "niue"), NIUE),
    ((7, "flag_nz"), FLAG_NZ),
    ((11, "new_zealand"), NEW_ZEALAND),
    ((20, "regional_indicator_n"), REGIONAL_INDICATOR_N),
    ((7, "flag_om"), FLAG_OM),
    ((4, "oman"), OMAN),
    ((20, "regional_indicator_o"), REGIONAL_INDICATOR_O),
    ((7, "flag_pa"), FLAG_PA),
    ((6, "panama"), PANAMA),
    ((7, "flag_pe"), FLAG_PE),
    ((4, "peru"), PERU),
    ((7, "flag_pf"), FLAG_PF),
    ((16, "french_polynesia"), FRENCH_POLYNESIA),
    ((7, "flag_pg"), FLAG_PG),
    ((16, "papua_new_guinea"), PAPUA_NEW_GUINEA),
    ((7, "flag_ph"), FLAG_PH),
    ((11, "philippines"), PHILIPPINES),
    ((7, "flag_pk"), FLAG_PK),
    ((8, "pakistan"), PAKISTAN),
    ((7, "flag_pl"), FLAG_PL),
    ((6, "poland"), POLAND),
    ((7, "flag_pm"), FLAG_PM),
    ((18, "st_pierre_miquelon"), ST_PIERRE_MIQUELON),
    ((7, "flag_pn"), FLAG_PN),
    ((16, "pitcairn_islands"), PITCAIRN_ISLANDS),
    ((7, "flag_pr"), FLAG_PR),
    ((11, "puerto_rico"), PUERTO_RICO),
    ((7, "flag_ps"), FLAG_PS),
    ((23, "palestinian_territories"), PALESTINIAN_TERRITORIES),
    ((7, "flag_pt"), FLAG_PT),
    ((8, "portugal"), PORTUGAL),
    ((7, "flag_pw"), FLAG_PW),
    ((5, "palau"), PALAU),
    ((7, "flag_py"), FLAG_PY),
    ((8, "paraguay"), PARAGUAY),
    ((20, "regional_indicator_p"), REGIONAL_INDICATOR_P),
    ((7, "flag_qa"), FLAG_QA),
    ((5, "qatar"), QATAR),
    ((20, "regional_indicator_q"), REGIONAL_INDICATOR_Q),
    ((7, "flag_re"), FLAG_RE),
    ((7, "reunion"), REUNION),
    ((7, "flag_ro"), FLAG_RO),
    ((7, "romania"), ROMANIA),
    ((7, "flag_rs"), FLAG_RS),
    ((6, "serbia"), SERBIA),
    ((7, "flag_ru"), FLAG_RU),
    ((6, "russia"), RUSSIA),
    ((7, "flag_rw"), FLAG_RW),
    ((6, "rwanda"), RWANDA),
    ((20, "regional_indicator_r"), REGIONAL_INDICATOR_R),
    ((7, "flag_sa"), FLAG_SA),
    ((12, "saudi_arabia"), SAUDI_ARABIA),
    ((7, "flag_sb"), FLAG_SB),
    ((15, "solomon_islands"), SOLOMON_ISLANDS),
    ((7, "flag_sc"), FLAG_SC),
    ((10, "seychelles"), SEYCHELLES),
    ((7, "flag_sd"), FLAG_SD),
    ((5, "sudan"), SUDAN),
    ((7, "flag_se"), FLAG_SE),
    ((6, "sweden"), SWEDEN),
    ((7, "flag_sg"), FLAG_SG),
    ((9, "singapore"), SINGAPORE),
    ((7, "flag_sh"), FLAG_SH),
    ((9, "st_helena"), ST_HELENA),
    ((7, "flag_si"), FLAG_SI),
    ((8, "slovenia"), SLOVENIA),
    ((7, "flag_sj"), FLAG_SJ),
    ((18, "svalbard_jan_mayen"), SVALBARD_JAN_MAYEN),
    ((7, "flag_sk"), FLAG_SK),
    ((8, "slovakia"), SLOVAKIA),
    ((7, "flag_sl"), FLAG_SL),
    ((12, "sierra_leone"), SIERRA_LEONE),
    ((7, "flag_sm"), FLAG_SM),
    ((10, "san_marino"), SAN_MARINO),
    ((7, "flag_sn"), FLAG_SN),
    ((7, "senegal"), SENEGAL),
    ((7, "flag_so"), FLAG_SO),
    ((7, "somalia"), SOMALIA),
    ((7, "flag_sr"), FLAG_SR),
    ((8, "suriname"), SURINAME),
    ((7, "flag_ss"), FLAG_SS),
    ((11, "south_sudan"), SOUTH_SUDAN),
    ((7, "flag_st"), FLAG_ST),
    ((17, "sao_tome_principe"), SAO_TOME_PRINCIPE),
    ((11, "el_salvador"), EL_SALVADOR),
    ((7, "flag_sv"), FLAG_SV),
    ((7, "flag_sx"), FLAG_SX),
    ((12, "sint_maarten"), SINT_MAARTEN),
    ((7, "flag_sy"), FLAG_SY),
    ((5, "syria"), SYRIA),
    ((8, "eswatini"), ESWATINI),
    ((7, "flag_sz"), FLAG_SZ),
    ((9, "swaziland"), SWAZILAND),
    ((20, "regional_indicator_s"), REGIONAL_INDICATOR_S),
    ((7, "flag_ta"), FLAG_TA),
    ((16, "tristan_da_cunha"), TRISTAN_DA_CUNHA),
    ((7, "flag_tc"), FLAG_TC),
    ((20, "turks_caicos_islands"), TURKS_CAICOS_ISLANDS),
    ((4, "chad"), CHAD),
    ((7, "flag_td"), FLAG_TD),
    ((7, "flag_tf"), FLAG_TF),
    ((27, "french_southern_territories"), FRENCH_SOUTHERN_TERRITORIES),
    ((7, "flag_tg"), FLAG_TG),
    ((4, "togo"), TOGO),
    ((7, "flag_th"), FLAG_TH),
    ((8, "thailand"), THAILAND),
    ((7, "flag_tj"), FLAG_TJ),
    ((10, "tajikistan"), TAJIKISTAN),
    ((7, "flag_tk"), FLAG_TK),
    ((7, "tokelau"), TOKELAU),
    ((7, "flag_tl"), FLAG_TL),
    ((11, "timor_leste"), TIMOR_LESTE),
    ((7, "flag_tm"), FLAG_TM),
    ((12, "turkmenistan"), TURKMENISTAN),
    ((7, "flag_tn"), FLAG_TN),
    ((7, "tunisia"), TUNISIA),
    ((7, "flag_to"), FLAG_TO),
    ((5, "tonga"), TONGA),
    ((7, "flag_tr"), FLAG_TR),
    ((9, "turkey_tr"), TURKEY_TR),
    ((7, "flag_tt"), FLAG_TT),
    ((15, "trinidad_tobago"), TRINIDAD_TOBAGO),
    ((7, "flag_tv"), FLAG_TV),
    ((6, "tuvalu"), TUVALU),
    ((7, "flag_tw"), FLAG_TW),
    ((6, "taiwan"), TAIWAN),
    ((7, "flag_tz"), FLAG_TZ),
    ((8, "tanzania"), TANZANIA),
    ((20, "regional_indicator_t"), REGIONAL_INDICATOR_T),
    ((7, "flag_ua"), FLAG_UA),
    ((7, "ukraine"), UKRAINE),
    ((7, "flag_ug"), FLAG_UG),
    ((6, "uganda"), UGANDA),
    ((7, "flag_um"), FLAG_UM),
    ((19, "us_outlying_islands"), US_OUTLYING_ISLANDS),
    ((7, "flag_un"), FLAG_UN),
    ((2, "un"), UN),
    ((14, "united_nations"), UNITED_NATIONS),
    ((7, "flag_us"), FLAG_US),
    ((13, "united_states"), UNITED_STATES),
    ((3, "usa"), USA),
    ((7, "flag_uy"), FLAG_UY),
    ((7, "uruguay"), URUGUAY),
    ((7, "flag_uz"), FLAG_UZ),
    ((10, "uzbekistan"), UZBEKISTAN),
    ((20, "regional_indicator_u"), REGIONAL_INDICATOR_U),
    ((7, "flag_va"), FLAG_VA),
    ((12, "vatican_city"), VATICAN_CITY),
    ((7, "flag_vc"), FLAG_VC),
    ((21, "st_vincent_grenadines"), ST_VINCENT_GRENADINES),
    ((7, "flag_ve"), FLAG_VE),
    ((9, "venezuela"), VENEZUELA),
    ((22, "british_virgin_islands"), BRITISH_VIRGIN_ISLANDS),
    ((7, "flag_vg"), FLAG_VG),
    ((7, "flag_vi"), FLAG_VI),
    ((17, "us_virgin_islands"), US_VIRGIN_ISLANDS),
    ((7, "flag_vn"), FLAG_VN),
    ((7, "vietnam"), VIETNAM),
    ((7, "flag_vu"), FLAG_VU),
    ((7, "vanuatu"), VANUATU),
    ((20, "regional_indicator_v"), REGIONAL_INDICATOR_V),
    ((7, "flag_wf"), FLAG_WF),
    ((13, "wallis_futuna"), WALLIS_FUTUNA),
    ((7, "flag_ws"), FLAG_WS),
    ((5, "samoa"), SAMOA),
    ((20, "regional_indicator_w"), REGIONAL_INDICATOR_W),
    ((7, "flag_xk"), FLAG_XK),
    ((6, "kosovo"), KOSOVO),
    ((20, "regional_indicator_x"), REGIONAL_INDICATOR_X),
    ((7, "flag_ye"), FLAG_YE),
    ((5, "yemen"), YEMEN),
    ((7, "flag_yt"), FLAG_YT),
    ((7, "mayotte"), MAYOTTE),
    ((20, "regional_indicator_y"), REGIONAL_INDICATOR_Y),
    ((7, "flag_za"), FLAG_ZA),
    ((12, "south_africa"), SOUTH_AFRICA),
    ((7, "flag_zm"), FLAG_ZM),
    ((6, "zambia"), ZAMBIA),
    ((7, "flag_zw"), FLAG_ZW),
    ((8, "zimbabwe"), ZIMBABWE),
    ((20, "regional_indicator_z"), REGIONAL_INDICATOR_Z),
    ((7, "ja_here"), JA_HERE),
    ((4, "koko"), KOKO),
    ((17, "ja_service_charge"), JA_SERVICE_CHARGE),
    ((17, "ja_free_of_charge"), JA_FREE_OF_CHARGE),
    ((11, "ja_reserved"), JA_RESERVED),
    ((13, "ja_prohibited"), JA_PROHIBITED),
    ((10, "ja_vacancy"), JA_VACANCY),
    ((16, "ja_passing_grade"), JA_PASSING_GRADE),
    ((13, "ja_no_vacancy"), JA_NO_VACANCY),
    ((20, "ja_not_free_of_carge"), JA_NOT_FREE_OF_CARGE),
    ((17, "ja_monthly_amount"), JA_MONTHLY_AMOUNT),
    ((14, "ja_application"), JA_APPLICATION),
    ((11, "ja_discount"), JA_DISCOUNT),
    ((20, "ja_open_for_business"), JA_OPEN_FOR_BUSINESS),
    ((19, "ideograph_advantage"), IDEOGRAPH_ADVANTAGE),
    ((10, "ja_bargain"), JA_BARGAIN),
    ((6, "accept"), ACCEPT),
    ((13, "ja_acceptable"), JA_ACCEPTABLE),
    ((7, "cyclone"), CYCLONE),
    ((5, "foggy"), FOGGY),
    ((15, "closed_umbrella"), CLOSED_UMBRELLA),
    ((16, "night_with_stars"), NIGHT_WITH_STARS),
    ((22, "sunrise_over_mountains"), SUNRISE_OVER_MOUNTAINS),
    ((7, "sunrise"), SUNRISE),
    ((9, "city_dusk"), CITY_DUSK),
    ((12, "city_sunrise"), CITY_SUNRISE),
    ((11, "city_sunset"), CITY_SUNSET),
    ((7, "rainbow"), RAINBOW),
    ((15, "bridge_at_night"), BRIDGE_AT_NIGHT),
    ((5, "ocean"), OCEAN),
    ((10, "water_wave"), WATER_WAVE),
    ((7, "volcano"), VOLCANO),
    ((9, "milky_way"), MILKY_WAY),
    ((12, "earth_africa"), EARTH_AFRICA),
    ((12, "earth_europe"), EARTH_EUROPE),
    ((14, "earth_americas"), EARTH_AMERICAS),
    ((10, "earth_asia"), EARTH_ASIA),
    ((20, "globe_with_meridians"), GLOBE_WITH_MERIDIANS),
    ((8, "new_moon"), NEW_MOON),
    ((20, "waxing_crescent_moon"), WAXING_CRESCENT_MOON),
    ((18, "first_quarter_moon"), FIRST_QUARTER_MOON),
    ((19, "waxing_gibbous_moon"), WAXING_GIBBOUS_MOON),
    ((9, "full_moon"), FULL_MOON),
    ((19, "waning_gibbous_moon"), WANING_GIBBOUS_MOON),
    ((17, "last_quarter_moon"), LAST_QUARTER_MOON),
    ((20, "waning_crescent_moon"), WANING_CRESCENT_MOON),
    ((13, "crescent_moon"), CRESCENT_MOON),
    ((18, "new_moon_with_face"), NEW_MOON_WITH_FACE),
    ((28, "first_quarter_moon_with_face"), FIRST_QUARTER_MOON_WITH_FACE),
    ((27, "last_quarter_moon_with_face"), LAST_QUARTER_MOON_WITH_FACE),
    ((19, "full_moon_with_face"), FULL_MOON_WITH_FACE),
    ((13, "sun_with_face"), SUN_WITH_FACE),
    ((12, "glowing_star"), GLOWING_STAR),
    ((5, "star2"), STAR2),
    ((13, "shooting_star"), SHOOTING_STAR),
    ((5, "stars"), STARS),
    ((11, "thermometer"), THERMOMETER),
    ((22, "sun_behind_small_cloud"), SUN_BEHIND_SMALL_CLOUD),
    ((5, "sunny"), SUNNY),
    ((6, "cloudy"), CLOUDY),
    ((22, "sun_behind_large_cloud"), SUN_BEHIND_LARGE_CLOUD),
    ((12, "sun_and_rain"), SUN_AND_RAIN),
    ((21, "sun_behind_rain_cloud"), SUN_BEHIND_RAIN_CLOUD),
    ((15, "cloud_with_rain"), CLOUD_WITH_RAIN),
    ((5, "rainy"), RAINY),
    ((15, "cloud_with_snow"), CLOUD_WITH_SNOW),
    ((5, "snowy"), SNOWY),
    ((20, "cloud_with_lightning"), CLOUD_WITH_LIGHTNING),
    ((9, "lightning"), LIGHTNING),
    ((7, "tornado"), TORNADO),
    ((3, "fog"), FOG),
    ((17, "wind_blowing_face"), WIND_BLOWING_FACE),
    ((6, "hotdog"), HOTDOG),
    ((4, "taco"), TACO),
    ((7, "burrito"), BURRITO),
    ((8, "chestnut"), CHESTNUT),
    ((8, "seedling"), SEEDLING),
    ((14, "evergreen_tree"), EVERGREEN_TREE),
    ((14, "deciduous_tree"), DECIDUOUS_TREE),
    ((9, "palm_tree"), PALM_TREE),
    ((6, "cactus"), CACTUS),
    ((10, "hot_pepper"), HOT_PEPPER),
    ((5, "tulip"), TULIP),
    ((14, "cherry_blossom"), CHERRY_BLOSSOM),
    ((4, "rose"), ROSE),
    ((8, "hibiscus"), HIBISCUS),
    ((9, "sunflower"), SUNFLOWER),
    ((7, "blossom"), BLOSSOM),
    ((4, "corn"), CORN),
    ((11, "ear_of_corn"), EAR_OF_CORN),
    ((11, "ear_of_rice"), EAR_OF_RICE),
    ((13, "sheaf_of_rice"), SHEAF_OF_RICE),
    ((4, "herb"), HERB),
    ((16, "four_leaf_clover"), FOUR_LEAF_CLOVER),
    ((10, "maple_leaf"), MAPLE_LEAF),
    ((11, "fallen_leaf"), FALLEN_LEAF),
    ((6, "leaves"), LEAVES),
    ((8, "mushroom"), MUSHROOM),
    ((6, "tomato"), TOMATO),
    ((8, "eggplant"), EGGPLANT),
    ((6, "grapes"), GRAPES),
    ((5, "melon"), MELON),
    ((10, "watermelon"), WATERMELON),
    ((6, "orange"), ORANGE),
    ((9, "tangerine"), TANGERINE),
    ((5, "lemon"), LEMON),
    ((6, "banana"), BANANA),
    ((9, "pineapple"), PINEAPPLE),
    ((5, "apple"), APPLE),
    ((9, "red_apple"), RED_APPLE),
    ((11, "green_apple"), GREEN_APPLE),
    ((4, "pear"), PEAR),
    ((5, "peach"), PEACH),
    ((8, "cherries"), CHERRIES),
    ((10, "strawberry"), STRAWBERRY),
    ((9, "hamburger"), HAMBURGER),
    ((5, "pizza"), PIZZA),
    ((12, "meat_on_bone"), MEAT_ON_BONE),
    ((11, "poultry_leg"), POULTRY_LEG),
    ((12, "rice_cracker"), RICE_CRACKER),
    ((9, "rice_ball"), RICE_BALL),
    ((11, "cooked_rice"), COOKED_RICE),
    ((4, "rice"), RICE),
    ((5, "curry"), CURRY),
    ((10, "curry_rice"), CURRY_RICE),
    ((5, "ramen"), RAMEN),
    ((13, "steaming_bowl"), STEAMING_BOWL),
    ((9, "spaghetti"), SPAGHETTI),
    ((5, "bread"), BREAD),
    ((12, "french_fries"), FRENCH_FRIES),
    ((5, "fries"), FRIES),
    ((12, "sweet_potato"), SWEET_POTATO),
    ((5, "dango"), DANGO),
    ((4, "oden"), ODEN),
    ((5, "sushi"), SUSHI),
    ((12, "fried_shrimp"), FRIED_SHRIMP),
    ((9, "fish_cake"), FISH_CAKE),
    ((8, "icecream"), ICECREAM),
    ((10, "soft_serve"), SOFT_SERVE),
    ((10, "shaved_ice"), SHAVED_ICE),
    ((9, "ice_cream"), ICE_CREAM),
    ((8, "doughnut"), DOUGHNUT),
    ((6, "cookie"), COOKIE),
    ((13, "chocolate_bar"), CHOCOLATE_BAR),
    ((5, "candy"), CANDY),
    ((8, "lollipop"), LOLLIPOP),
    ((7, "custard"), CUSTARD),
    ((9, "honey_pot"), HONEY_POT),
    ((4, "cake"), CAKE),
    ((9, "shortcake"), SHORTCAKE),
    ((5, "bento"), BENTO),
    ((9, "bento_box"), BENTO_BOX),
    ((11, "pot_of_food"), POT_OF_FOOD),
    ((4, "stew"), STEW),
    ((7, "cooking"), COOKING),
    ((9, "fried_egg"), FRIED_EGG),
    ((14, "fork_and_knife"), FORK_AND_KNIFE),
    ((3, "tea"), TEA),
    ((4, "sake"), SAKE),
    ((10, "wine_glass"), WINE_GLASS),
    ((8, "cocktail"), COCKTAIL),
    ((14, "tropical_drink"), TROPICAL_DRINK),
    ((4, "beer"), BEER),
    ((5, "beers"), BEERS),
    ((11, "baby_bottle"), BABY_BOTTLE),
    ((16, "fork_knife_plate"), FORK_KNIFE_PLATE),
    ((9, "champagne"), CHAMPAGNE),
    ((7, "popcorn"), POPCORN),
    ((6, "ribbon"), RIBBON),
    ((4, "gift"), GIFT),
    ((8, "birthday"), BIRTHDAY),
    ((13, "birthday_cake"), BIRTHDAY_CAKE),
    ((14, "jack_o_lantern"), JACK_O_LANTERN),
    ((14, "christmas_tree"), CHRISTMAS_TREE),
    ((11, "santa_tone1"), SANTA_TONE1),
    ((11, "santa_tone2"), SANTA_TONE2),
    ((11, "santa_tone3"), SANTA_TONE3),
    ((11, "santa_tone4"), SANTA_TONE4),
    ((11, "santa_tone5"), SANTA_TONE5),
    ((5, "santa"), SANTA),
    ((9, "fireworks"), FIREWORKS),
    ((8, "sparkler"), SPARKLER),
    ((7, "balloon"), BALLOON),
    ((5, "party"), PARTY),
    ((12, "party_popper"), PARTY_POPPER),
    ((4, "tada"), TADA),
    ((13, "confetti_ball"), CONFETTI_BALL),
    ((13, "tanabata_tree"), TANABATA_TREE),
    ((13, "crossed_flags"), CROSSED_FLAGS),
    ((6, "bamboo"), BAMBOO),
    ((5, "dolls"), DOLLS),
    ((13, "carp_streamer"), CARP_STREAMER),
    ((5, "flags"), FLAGS),
    ((10, "wind_chime"), WIND_CHIME),
    ((13, "moon_ceremony"), MOON_CEREMONY),
    ((10, "rice_scene"), RICE_SCENE),
    ((8, "backpack"), BACKPACK),
    ((14, "school_satchel"), SCHOOL_SATCHEL),
    ((14, "graduation_cap"), GRADUATION_CAP),
    ((12, "mortar_board"), MORTAR_BOARD),
    ((14, "military_medal"), MILITARY_MEDAL),
    ((15, "reminder_ribbon"), REMINDER_RIBBON),
    ((17, "studio_microphone"), STUDIO_MICROPHONE),
    ((12, "level_slider"), LEVEL_SLIDER),
    ((13, "control_knobs"), CONTROL_KNOBS),
    ((11, "film_frames"), FILM_FRAMES),
    ((17, "admission_tickets"), ADMISSION_TICKETS),
    ((7, "tickets"), TICKETS),
    ((14, "carousel_horse"), CAROUSEL_HORSE),
    ((12, "ferris_wheel"), FERRIS_WHEEL),
    ((14, "roller_coaster"), ROLLER_COASTER),
    ((12, "fishing_pole"), FISHING_POLE),
    ((21, "fishing_pole_and_fish"), FISHING_POLE_AND_FISH),
    ((10, "microphone"), MICROPHONE),
    ((12, "movie_camera"), MOVIE_CAMERA),
    ((6, "cinema"), CINEMA),
    ((10, "headphones"), HEADPHONES),
    ((3, "art"), ART),
    ((7, "palette"), PALETTE),
    ((7, "top_hat"), TOP_HAT),
    ((6, "tophat"), TOPHAT),
    ((11, "circus_tent"), CIRCUS_TENT),
    ((6, "ticket"), TICKET),
    ((7, "clapper"), CLAPPER),
    ((15, "performing_arts"), PERFORMING_ARTS),
    ((10, "controller"), CONTROLLER),
    ((10, "video_game"), VIDEO_GAME),
    ((8, "bullseye"), BULLSEYE),
    ((4, "dart"), DART),
    ((10, "direct_hit"), DIRECT_HIT),
    ((12, "slot_machine"), SLOT_MACHINE),
    ((5, "8ball"), X_8BALL),
    ((9, "billiards"), BILLIARDS),
    ((8, "game_die"), GAME_DIE),
    ((7, "bowling"), BOWLING),
    ((20, "flower_playing_cards"), FLOWER_PLAYING_CARDS),
    ((12, "musical_note"), MUSICAL_NOTE),
    ((13, "musical_notes"), MUSICAL_NOTES),
    ((5, "notes"), NOTES),
    ((9, "saxophone"), SAXOPHONE),
    ((6, "guitar"), GUITAR),
    ((16, "musical_keyboard"), MUSICAL_KEYBOARD),
    ((7, "trumpet"), TRUMPET),
    ((6, "violin"), VIOLIN),
    ((13, "musical_score"), MUSICAL_SCORE),
    ((13, "running_shirt"), RUNNING_SHIRT),
    ((23, "running_shirt_with_sash"), RUNNING_SHIRT_WITH_SASH),
    ((6, "tennis"), TENNIS),
    ((3, "ski"), SKI),
    ((10, "basketball"), BASKETBALL),
    ((14, "checkered_flag"), CHECKERED_FLAG),
    ((25, "person_snowboarding_tone1"), PERSON_SNOWBOARDING_TONE1),
    ((17, "snowboarder_tone1"), SNOWBOARDER_TONE1),
    ((18, "snowboarding_tone1"), SNOWBOARDING_TONE1),
    ((25, "person_snowboarding_tone2"), PERSON_SNOWBOARDING_TONE2),
    ((17, "snowboarder_tone2"), SNOWBOARDER_TONE2),
    ((18, "snowboarding_tone2"), SNOWBOARDING_TONE2),
    ((25, "person_snowboarding_tone3"), PERSON_SNOWBOARDING_TONE3),
    ((17, "snowboarder_tone3"), SNOWBOARDER_TONE3),
    ((18, "snowboarding_tone3"), SNOWBOARDING_TONE3),
    ((25, "person_snowboarding_tone4"), PERSON_SNOWBOARDING_TONE4),
    ((17, "snowboarder_tone4"), SNOWBOARDER_TONE4),
    ((18, "snowboarding_tone4"), SNOWBOARDING_TONE4),
    ((25, "person_snowboarding_tone5"), PERSON_SNOWBOARDING_TONE5),
    ((17, "snowboarder_tone5"), SNOWBOARDER_TONE5),
    ((18, "snowboarding_tone5"), SNOWBOARDING_TONE5),
    ((19, "person_snowboarding"), PERSON_SNOWBOARDING),
    ((11, "snowboarder"), SNOWBOARDER),
    ((12, "snowboarding"), SNOWBOARDING),
    ((19, "woman_running_tone1"), WOMAN_RUNNING_TONE1),
    ((17, "man_running_tone1"), MAN_RUNNING_TONE1),
    ((20, "person_running_tone1"), PERSON_RUNNING_TONE1),
    ((13, "running_tone1"), RUNNING_TONE1),
    ((19, "woman_running_tone2"), WOMAN_RUNNING_TONE2),
    ((17, "man_running_tone2"), MAN_RUNNING_TONE2),
    ((20, "person_running_tone2"), PERSON_RUNNING_TONE2),
    ((13, "running_tone2"), RUNNING_TONE2),
    ((19, "woman_running_tone3"), WOMAN_RUNNING_TONE3),
    ((17, "man_running_tone3"), MAN_RUNNING_TONE3),
    ((20, "person_running_tone3"), PERSON_RUNNING_TONE3),
    ((13, "running_tone3"), RUNNING_TONE3),
    ((19, "woman_running_tone4"), WOMAN_RUNNING_TONE4),
    ((17, "man_running_tone4"), MAN_RUNNING_TONE4),
    ((20, "person_running_tone4"), PERSON_RUNNING_TONE4),
    ((13, "running_tone4"), RUNNING_TONE4),
    ((19, "woman_running_tone5"), WOMAN_RUNNING_TONE5),
    ((17, "man_running_tone5"), MAN_RUNNING_TONE5),
    ((20, "person_running_tone5"), PERSON_RUNNING_TONE5),
    ((13, "running_tone5"), RUNNING_TONE5),
    ((13, "woman_running"), WOMAN_RUNNING),
    ((11, "man_running"), MAN_RUNNING),
    ((14, "person_running"), PERSON_RUNNING),
    ((7, "running"), RUNNING),
    ((19, "woman_surfing_tone1"), WOMAN_SURFING_TONE1),
    ((17, "man_surfing_tone1"), MAN_SURFING_TONE1),
    ((20, "person_surfing_tone1"), PERSON_SURFING_TONE1),
    ((12, "surfer_tone1"), SURFER_TONE1),
    ((13, "surfing_tone1"), SURFING_TONE1),
    ((19, "woman_surfing_tone2"), WOMAN_SURFING_TONE2),
    ((17, "man_surfing_tone2"), MAN_SURFING_TONE2),
    ((20, "person_surfing_tone2"), PERSON_SURFING_TONE2),
    ((12, "surfer_tone2"), SURFER_TONE2),
    ((13, "surfing_tone2"), SURFING_TONE2),
    ((19, "woman_surfing_tone3"), WOMAN_SURFING_TONE3),
    ((17, "man_surfing_tone3"), MAN_SURFING_TONE3),
    ((20, "person_surfing_tone3"), PERSON_SURFING_TONE3),
    ((12, "surfer_tone3"), SURFER_TONE3),
    ((13, "surfing_tone3"), SURFING_TONE3),
    ((19, "woman_surfing_tone4"), WOMAN_SURFING_TONE4),
    ((17, "man_surfing_tone4"), MAN_SURFING_TONE4),
    ((20, "person_surfing_tone4"), PERSON_SURFING_TONE4),
    ((12, "surfer_tone4"), SURFER_TONE4),
    ((13, "surfing_tone4"), SURFING_TONE4),
    ((19, "woman_surfing_tone5"), WOMAN_SURFING_TONE5),
    ((17, "man_surfing_tone5"), MAN_SURFING_TONE5),
    ((20, "person_surfing_tone5"), PERSON_SURFING_TONE5),
    ((12, "surfer_tone5"), SURFER_TONE5),
    ((13, "surfing_tone5"), SURFING_TONE5),
    ((13, "woman_surfing"), WOMAN_SURFING),
    ((11, "man_surfing"), MAN_SURFING),
    ((14, "person_surfing"), PERSON_SURFING),
    ((6, "surfer"), SURFER),
    ((7, "surfing"), SURFING),
    ((12, "sports_medal"), SPORTS_MEDAL),
    ((6, "trophy"), TROPHY),
    ((18, "horse_racing_tone1"), HORSE_RACING_TONE1),
    ((18, "horse_racing_tone2"), HORSE_RACING_TONE2),
    ((18, "horse_racing_tone3"), HORSE_RACING_TONE3),
    ((18, "horse_racing_tone4"), HORSE_RACING_TONE4),
    ((18, "horse_racing_tone5"), HORSE_RACING_TONE5),
    ((12, "horse_racing"), HORSE_RACING),
    ((8, "football"), FOOTBALL),
    ((14, "rugby_football"), RUGBY_FOOTBALL),
    ((20, "woman_swimming_tone1"), WOMAN_SWIMMING_TONE1),
    ((18, "man_swimming_tone1"), MAN_SWIMMING_TONE1),
    ((21, "person_swimming_tone1"), PERSON_SWIMMING_TONE1),
    ((13, "swimmer_tone1"), SWIMMER_TONE1),
    ((14, "swimming_tone1"), SWIMMING_TONE1),
    ((20, "woman_swimming_tone2"), WOMAN_SWIMMING_TONE2),
    ((18, "man_swimming_tone2"), MAN_SWIMMING_TONE2),
    ((21, "person_swimming_tone2"), PERSON_SWIMMING_TONE2),
    ((13, "swimmer_tone2"), SWIMMER_TONE2),
    ((14, "swimming_tone2"), SWIMMING_TONE2),
    ((20, "woman_swimming_tone3"), WOMAN_SWIMMING_TONE3),
    ((18, "man_swimming_tone3"), MAN_SWIMMING_TONE3),
    ((21, "person_swimming_tone3"), PERSON_SWIMMING_TONE3),
    ((13, "swimmer_tone3"), SWIMMER_TONE3),
    ((14, "swimming_tone3"), SWIMMING_TONE3),
    ((20, "woman_swimming_tone4"), WOMAN_SWIMMING_TONE4),
    ((18, "man_swimming_tone4"), MAN_SWIMMING_TONE4),
    ((21, "person_swimming_tone4"), PERSON_SWIMMING_TONE4),
    ((13, "swimmer_tone4"), SWIMMER_TONE4),
    ((14, "swimming_tone4"), SWIMMING_TONE4),
    ((20, "woman_swimming_tone5"), WOMAN_SWIMMING_TONE5),
    ((18, "man_swimming_tone5"), MAN_SWIMMING_TONE5),
    ((21, "person_swimming_tone5"), PERSON_SWIMMING_TONE5),
    ((13, "swimmer_tone5"), SWIMMER_TONE5),
    ((14, "swimming_tone5"), SWIMMING_TONE5),
    ((14, "woman_swimming"), WOMAN_SWIMMING),
    ((12, "man_swimming"), MAN_SWIMMING),
    ((15, "person_swimming"), PERSON_SWIMMING),
    ((7, "swimmer"), SWIMMER),
    ((8, "swimming"), SWIMMING),
    ((27, "woman_lifting_weights_tone1"), WOMAN_LIFTING_WEIGHTS_TONE1),
    ((25, "man_lifting_weights_tone1"), MAN_LIFTING_WEIGHTS_TONE1),
    ((28, "person_lifting_weights_tone1"), PERSON_LIFTING_WEIGHTS_TONE1),
    ((19, "weight_lifter_tone1"), WEIGHT_LIFTER_TONE1),
    ((20, "weight_lifting_tone1"), WEIGHT_LIFTING_TONE1),
    ((27, "woman_lifting_weights_tone2"), WOMAN_LIFTING_WEIGHTS_TONE2),
    ((25, "man_lifting_weights_tone2"), MAN_LIFTING_WEIGHTS_TONE2),
    ((28, "person_lifting_weights_tone2"), PERSON_LIFTING_WEIGHTS_TONE2),
    ((19, "weight_lifter_tone2"), WEIGHT_LIFTER_TONE2),
    ((20, "weight_lifting_tone2"), WEIGHT_LIFTING_TONE2),
    ((27, "woman_lifting_weights_tone3"), WOMAN_LIFTING_WEIGHTS_TONE3),
    ((25, "man_lifting_weights_tone3"), MAN_LIFTING_WEIGHTS_TONE3),
    ((28, "person_lifting_weights_tone3"), PERSON_LIFTING_WEIGHTS_TONE3),
    ((19, "weight_lifter_tone3"), WEIGHT_LIFTER_TONE3),
    ((20, "weight_lifting_tone3"), WEIGHT_LIFTING_TONE3),
    ((27, "woman_lifting_weights_tone4"), WOMAN_LIFTING_WEIGHTS_TONE4),
    ((25, "man_lifting_weights_tone4"), MAN_LIFTING_WEIGHTS_TONE4),
    ((28, "person_lifting_weights_tone4"), PERSON_LIFTING_WEIGHTS_TONE4),
    ((19, "weight_lifter_tone4"), WEIGHT_LIFTER_TONE4),
    ((20, "weight_lifting_tone4"), WEIGHT_LIFTING_TONE4),
    ((27, "woman_lifting_weights_tone5"), WOMAN_LIFTING_WEIGHTS_TONE5),
    ((25, "man_lifting_weights_tone5"), MAN_LIFTING_WEIGHTS_TONE5),
    ((28, "person_lifting_weights_tone5"), PERSON_LIFTING_WEIGHTS_TONE5),
    ((19, "weight_lifter_tone5"), WEIGHT_LIFTER_TONE5),
    ((20, "weight_lifting_tone5"), WEIGHT_LIFTING_TONE5),
    ((21, "woman_lifting_weights"), WOMAN_LIFTING_WEIGHTS),
    ((19, "man_lifting_weights"), MAN_LIFTING_WEIGHTS),
    ((22, "person_lifting_weights"), PERSON_LIFTING_WEIGHTS),
    ((13, "weight_lifter"), WEIGHT_LIFTER),
    ((14, "weight_lifting"), WEIGHT_LIFTING),
    ((19, "woman_golfing_tone1"), WOMAN_GOLFING_TONE1),
    ((17, "man_golfing_tone1"), MAN_GOLFING_TONE1),
    ((12, "golfer_tone1"), GOLFER_TONE1),
    ((13, "golfing_tone1"), GOLFING_TONE1),
    ((20, "person_golfing_tone1"), PERSON_GOLFING_TONE1),
    ((19, "woman_golfing_tone2"), WOMAN_GOLFING_TONE2),
    ((17, "man_golfing_tone2"), MAN_GOLFING_TONE2),
    ((12, "golfer_tone2"), GOLFER_TONE2),
    ((13, "golfing_tone2"), GOLFING_TONE2),
    ((20, "person_golfing_tone2"), PERSON_GOLFING_TONE2),
    ((19, "woman_golfing_tone3"), WOMAN_GOLFING_TONE3),
    ((17, "man_golfing_tone3"), MAN_GOLFING_TONE3),
    ((12, "golfer_tone3"), GOLFER_TONE3),
    ((13, "golfing_tone3"), GOLFING_TONE3),
    ((20, "person_golfing_tone3"), PERSON_GOLFING_TONE3),
    ((19, "woman_golfing_tone4"), WOMAN_GOLFING_TONE4),
    ((17, "man_golfing_tone4"), MAN_GOLFING_TONE4),
    ((12, "golfer_tone4"), GOLFER_TONE4),
    ((13, "golfing_tone4"), GOLFING_TONE4),
    ((20, "person_golfing_tone4"), PERSON_GOLFING_TONE4),
    ((19, "woman_golfing_tone5"), WOMAN_GOLFING_TONE5),
    ((17, "man_golfing_tone5"), MAN_GOLFING_TONE5),
    ((12, "golfer_tone5"), GOLFER_TONE5),
    ((13, "golfing_tone5"), GOLFING_TONE5),
    ((20, "person_golfing_tone5"), PERSON_GOLFING_TONE5),
    ((13, "woman_golfing"), WOMAN_GOLFING),
    ((11, "man_golfing"), MAN_GOLFING),
    ((6, "golfer"), GOLFER),
    ((7, "golfing"), GOLFING),
    ((14, "person_golfing"), PERSON_GOLFING),
    ((10, "motorcycle"), MOTORCYCLE),
    ((10, "racing_car"), RACING_CAR),
    ((12, "cricket_game"), CRICKET_GAME),
    ((10, "volleyball"), VOLLEYBALL),
    ((12, "field_hockey"), FIELD_HOCKEY),
    ((6, "hockey"), HOCKEY),
    ((9, "ping_pong"), PING_PONG),
    ((13, "mountain_snow"), MOUNTAIN_SNOW),
    ((7, "camping"), CAMPING),
    ((5, "beach"), BEACH),
    ((19, "beach_with_umbrella"), BEACH_WITH_UMBRELLA),
    ((21, "building_construction"), BUILDING_CONSTRUCTION),
    ((17, "construction_site"), CONSTRUCTION_SITE),
    ((5, "homes"), HOMES),
    ((6, "houses"), HOUSES),
    ((9, "cityscape"), CITYSCAPE),
    ((14, "derelict_house"), DERELICT_HOUSE),
    ((15, "house_abandoned"), HOUSE_ABANDONED),
    ((18, "classical_building"), CLASSICAL_BUILDING),
    ((6, "desert"), DESERT),
    ((13, "desert_island"), DESERT_ISLAND),
    ((6, "island"), ISLAND),
    ((13, "national_park"), NATIONAL_PARK),
    ((7, "stadium"), STADIUM),
    ((5, "house"), HOUSE),
    ((17, "house_with_garden"), HOUSE_WITH_GARDEN),
    ((6, "office"), OFFICE),
    ((11, "post_office"), POST_OFFICE),
    ((20, "european_post_office"), EUROPEAN_POST_OFFICE),
    ((8, "hospital"), HOSPITAL),
    ((4, "bank"), BANK),
    ((3, "atm"), ATM),
    ((5, "hotel"), HOTEL),
    ((10, "love_hotel"), LOVE_HOTEL),
    ((17, "convenience_store"), CONVENIENCE_STORE),
    ((6, "school"), SCHOOL),
    ((16, "department_store"), DEPARTMENT_STORE),
    ((7, "factory"), FACTORY),
    ((15, "izakaya_lantern"), IZAKAYA_LANTERN),
    ((17, "red_paper_lantern"), RED_PAPER_LANTERN),
    ((15, "japanese_castle"), JAPANESE_CASTLE),
    ((6, "castle"), CASTLE),
    ((15, "european_castle"), EUROPEAN_CASTLE),
    ((12, "rainbow_flag"), RAINBOW_FLAG),
    ((16, "transgender_flag"), TRANSGENDER_FLAG),
    ((10, "white_flag"), WHITE_FLAG),
    ((11, "jolly_roger"), JOLLY_ROGER),
    ((11, "pirate_flag"), PIRATE_FLAG),
    ((7, "england"), ENGLAND),
    ((10, "flag_gbeng"), FLAG_GBENG),
    ((10, "flag_gbsct"), FLAG_GBSCT),
    ((8, "scotland"), SCOTLAND),
    ((10, "flag_gbwls"), FLAG_GBWLS),
    ((5, "wales"), WALES),
    ((10, "black_flag"), BLACK_FLAG),
    ((7, "rosette"), ROSETTE),
    ((5, "label"), LABEL),
    ((9, "badminton"), BADMINTON),
    ((13, "bow_and_arrow"), BOW_AND_ARROW),
    ((7, "amphora"), AMPHORA),
    ((5, "tone1"), TONE1),
    ((10, "tone_light"), TONE_LIGHT),
    ((5, "tone2"), TONE2),
    ((17, "tone_medium_light"), TONE_MEDIUM_LIGHT),
    ((5, "tone3"), TONE3),
    ((11, "tone_medium"), TONE_MEDIUM),
    ((5, "tone4"), TONE4),
    ((16, "tone_medium_dark"), TONE_MEDIUM_DARK),
    ((5, "tone5"), TONE5),
    ((9, "tone_dark"), TONE_DARK),
    ((3, "rat"), RAT),
    ((5, "mouse"), MOUSE),
    ((2, "ox"), OX),
    ((13, "water_buffalo"), WATER_BUFFALO),
    ((3, "cow"), COW),
    ((5, "tiger"), TIGER),
    ((7, "leopard"), LEOPARD),
    ((6, "rabbit"), RABBIT),
    ((9, "black_cat"), BLACK_CAT),
    ((3, "cat"), CAT),
    ((6, "dragon"), DRAGON),
    ((9, "crocodile"), CROCODILE),
    ((5, "whale"), WHALE),
    ((5, "snail"), SNAIL),
    ((5, "snake"), SNAKE),
    ((5, "horse"), HORSE),
    ((9, "racehorse"), RACEHORSE),
    ((3, "ram"), RAM),
    ((4, "goat"), GOAT),
    ((3, "ewe"), EWE),
    ((5, "sheep"), SHEEP),
    ((6, "monkey"), MONKEY),
    ((7, "rooster"), ROOSTER),
    ((7, "chicken"), CHICKEN),
    ((12, "chicken_face"), CHICKEN_FACE),
    ((11, "service_dog"), SERVICE_DOG),
    ((3, "dog"), DOG),
    ((3, "pig"), PIG),
    ((4, "boar"), BOAR),
    ((8, "elephant"), ELEPHANT),
    ((7, "octopus"), OCTOPUS),
    ((5, "shell"), SHELL),
    ((3, "bug"), BUG),
    ((3, "ant"), ANT),
    ((3, "bee"), BEE),
    ((11, "lady_beetle"), LADY_BEETLE),
    ((4, "fish"), FISH),
    ((13, "tropical_fish"), TROPICAL_FISH),
    ((8, "blowfish"), BLOWFISH),
    ((6, "turtle"), TURTLE),
    ((14, "hatching_chick"), HATCHING_CHICK),
    ((10, "baby_chick"), BABY_CHICK),
    ((13, "hatched_chick"), HATCHED_CHICK),
    ((4, "bird"), BIRD),
    ((9, "bird_face"), BIRD_FACE),
    ((7, "penguin"), PENGUIN),
    ((12, "penguin_face"), PENGUIN_FACE),
    ((5, "koala"), KOALA),
    ((10, "koala_face"), KOALA_FACE),
    ((6, "poodle"), POODLE),
    ((15, "dromedary_camel"), DROMEDARY_CAMEL),
    ((5, "camel"), CAMEL),
    ((7, "dolphin"), DOLPHIN),
    ((10, "mouse_face"), MOUSE_FACE),
    ((8, "cow_face"), COW_FACE),
    ((10, "tiger_face"), TIGER_FACE),
    ((11, "rabbit_face"), RABBIT_FACE),
    ((8, "cat_face"), CAT_FACE),
    ((11, "dragon_face"), DRAGON_FACE),
    ((14, "spouting_whale"), SPOUTING_WHALE),
    ((10, "horse_face"), HORSE_FACE),
    ((11, "monkey_face"), MONKEY_FACE),
    ((8, "dog_face"), DOG_FACE),
    ((8, "pig_face"), PIG_FACE),
    ((4, "frog"), FROG),
    ((9, "frog_face"), FROG_FACE),
    ((7, "hamster"), HAMSTER),
    ((12, "hamster_face"), HAMSTER_FACE),
    ((4, "wolf"), WOLF),
    ((9, "wolf_face"), WOLF_FACE),
    ((10, "polar_bear"), POLAR_BEAR),
    ((15, "polar_bear_face"), POLAR_BEAR_FACE),
    ((4, "bear"), BEAR),
    ((9, "bear_face"), BEAR_FACE),
    ((5, "panda"), PANDA),
    ((10, "panda_face"), PANDA_FACE),
    ((8, "pig_nose"), PIG_NOSE),
    ((10, "paw_prints"), PAW_PRINTS),
    ((8, "chipmunk"), CHIPMUNK),
    ((4, "eyes"), EYES),
    ((3, "eye"), EYE),
    ((9, "ear_tone1"), EAR_TONE1),
    ((9, "ear_tone2"), EAR_TONE2),
    ((9, "ear_tone3"), EAR_TONE3),
    ((9, "ear_tone4"), EAR_TONE4),
    ((9, "ear_tone5"), EAR_TONE5),
    ((3, "ear"), EAR),
    ((10, "nose_tone1"), NOSE_TONE1),
    ((10, "nose_tone2"), NOSE_TONE2),
    ((10, "nose_tone3"), NOSE_TONE3),
    ((10, "nose_tone4"), NOSE_TONE4),
    ((10, "nose_tone5"), NOSE_TONE5),
    ((4, "nose"), NOSE),
    ((4, "lips"), LIPS),
    ((5, "mouth"), MOUTH),
    ((6, "tongue"), TONGUE),
    ((14, "point_up_tone1"), POINT_UP_TONE1),
    ((14, "point_up_tone2"), POINT_UP_TONE2),
    ((14, "point_up_tone3"), POINT_UP_TONE3),
    ((14, "point_up_tone4"), POINT_UP_TONE4),
    ((14, "point_up_tone5"), POINT_UP_TONE5),
    ((8, "point_up"), POINT_UP),
    ((16, "point_down_tone1"), POINT_DOWN_TONE1),
    ((16, "point_down_tone2"), POINT_DOWN_TONE2),
    ((16, "point_down_tone3"), POINT_DOWN_TONE3),
    ((16, "point_down_tone4"), POINT_DOWN_TONE4),
    ((16, "point_down_tone5"), POINT_DOWN_TONE5),
    ((10, "point_down"), POINT_DOWN),
    ((16, "point_left_tone1"), POINT_LEFT_TONE1),
    ((16, "point_left_tone2"), POINT_LEFT_TONE2),
    ((16, "point_left_tone3"), POINT_LEFT_TONE3),
    ((16, "point_left_tone4"), POINT_LEFT_TONE4),
    ((16, "point_left_tone5"), POINT_LEFT_TONE5),
    ((10, "point_left"), POINT_LEFT),
    ((17, "point_right_tone1"), POINT_RIGHT_TONE1),
    ((17, "point_right_tone2"), POINT_RIGHT_TONE2),
    ((17, "point_right_tone3"), POINT_RIGHT_TONE3),
    ((17, "point_right_tone4"), POINT_RIGHT_TONE4),
    ((17, "point_right_tone5"), POINT_RIGHT_TONE5),
    ((11, "point_right"), POINT_RIGHT),
    ((11, "punch_tone1"), PUNCH_TONE1),
    ((11, "punch_tone2"), PUNCH_TONE2),
    ((11, "punch_tone3"), PUNCH_TONE3),
    ((11, "punch_tone4"), PUNCH_TONE4),
    ((11, "punch_tone5"), PUNCH_TONE5),
    ((5, "punch"), PUNCH),
    ((10, "wave_tone1"), WAVE_TONE1),
    ((17, "waving_hand_tone1"), WAVING_HAND_TONE1),
    ((10, "wave_tone2"), WAVE_TONE2),
    ((17, "waving_hand_tone2"), WAVING_HAND_TONE2),
    ((10, "wave_tone3"), WAVE_TONE3),
    ((17, "waving_hand_tone3"), WAVING_HAND_TONE3),
    ((10, "wave_tone4"), WAVE_TONE4),
    ((17, "waving_hand_tone4"), WAVING_HAND_TONE4),
    ((10, "wave_tone5"), WAVE_TONE5),
    ((17, "waving_hand_tone5"), WAVING_HAND_TONE5),
    ((4, "wave"), WAVE),
    ((11, "waving_hand"), WAVING_HAND),
    ((13, "ok_hand_tone1"), OK_HAND_TONE1),
    ((13, "ok_hand_tone2"), OK_HAND_TONE2),
    ((13, "ok_hand_tone3"), OK_HAND_TONE3),
    ((13, "ok_hand_tone4"), OK_HAND_TONE4),
    ((13, "ok_hand_tone5"), OK_HAND_TONE5),
    ((7, "ok_hand"), OK_HAND),
    ((8, "+1_tone1"), X_1_TONE1),
    ((14, "thumbsup_tone1"), THUMBSUP_TONE1),
    ((9, "yes_tone1"), YES_TONE1),
    ((8, "+1_tone2"), X_1_TONE2),
    ((14, "thumbsup_tone2"), THUMBSUP_TONE2),
    ((9, "yes_tone2"), YES_TONE2),
    ((8, "+1_tone3"), X_1_TONE3),
    ((14, "thumbsup_tone3"), THUMBSUP_TONE3),
    ((9, "yes_tone3"), YES_TONE3),
    ((8, "+1_tone4"), X_1_TONE4),
    ((14, "thumbsup_tone4"), THUMBSUP_TONE4),
    ((9, "yes_tone4"), YES_TONE4),
    ((8, "+1_tone5"), X_1_TONE5),
    ((14, "thumbsup_tone5"), THUMBSUP_TONE5),
    ((9, "yes_tone5"), YES_TONE5),
    ((2, "+1"), X_1),
    ((8, "thumbsup"), THUMBSUP),
    ((3, "yes"), YES),
    ((8, "-1_tone1"), X__1_TONE1),
    ((8, "no_tone1"), NO_TONE1),
    ((16, "thumbsdown_tone1"), THUMBSDOWN_TONE1),
    ((8, "-1_tone2"), X__1_TONE2),
    ((8, "no_tone2"), NO_TONE2),
    ((16, "thumbsdown_tone2"), THUMBSDOWN_TONE2),
    ((8, "-1_tone3"), X__1_TONE3),
    ((8, "no_tone3"), NO_TONE3),
    ((16, "thumbsdown_tone3"), THUMBSDOWN_TONE3),
    ((8, "-1_tone4"), X__1_TONE4),
    ((8, "no_tone4"), NO_TONE4),
    ((16, "thumbsdown_tone4"), THUMBSDOWN_TONE4),
    ((8, "-1_tone5"), X__1_TONE5),
    ((8, "no_tone5"), NO_TONE5),
    ((16, "thumbsdown_tone5"), THUMBSDOWN_TONE5),
    ((2, "-1"), X__1),
    ((2, "no"), NO),
    ((10, "thumbsdown"), THUMBSDOWN),
    ((10, "clap_tone1"), CLAP_TONE1),
    ((20, "clapping_hands_tone1"), CLAPPING_HANDS_TONE1),
    ((10, "clap_tone2"), CLAP_TONE2),
    ((20, "clapping_hands_tone2"), CLAPPING_HANDS_TONE2),
    ((10, "clap_tone3"), CLAP_TONE3),
    ((20, "clapping_hands_tone3"), CLAPPING_HANDS_TONE3),
    ((10, "clap_tone4"), CLAP_TONE4),
    ((20, "clapping_hands_tone4"), CLAPPING_HANDS_TONE4),
    ((10, "clap_tone5"), CLAP_TONE5),
    ((20, "clapping_hands_tone5"), CLAPPING_HANDS_TONE5),
    ((4, "clap"), CLAP),
    ((14, "clapping_hands"), CLAPPING_HANDS),
    ((16, "open_hands_tone1"), OPEN_HANDS_TONE1),
    ((16, "open_hands_tone2"), OPEN_HANDS_TONE2),
    ((16, "open_hands_tone3"), OPEN_HANDS_TONE3),
    ((16, "open_hands_tone4"), OPEN_HANDS_TONE4),
    ((16, "open_hands_tone5"), OPEN_HANDS_TONE5),
    ((10, "open_hands"), OPEN_HANDS),
    ((5, "crown"), CROWN),
    ((10, "womans_hat"), WOMANS_HAT),
    ((10, "eyeglasses"), EYEGLASSES),
    ((7, "glasses"), GLASSES),
    ((7, "necktie"), NECKTIE),
    ((5, "shirt"), SHIRT),
    ((5, "jeans"), JEANS),
    ((5, "dress"), DRESS),
    ((6, "kimono"), KIMONO),
    ((6, "bikini"), BIKINI),
    ((14, "womans_clothes"), WOMANS_CLOTHES),
    ((5, "purse"), PURSE),
    ((7, "handbag"), HANDBAG),
    ((10, "clutch_bag"), CLUTCH_BAG),
    ((5, "pouch"), POUCH),
    ((9, "mans_shoe"), MANS_SHOE),
    ((13, "athletic_shoe"), ATHLETIC_SHOE),
    ((7, "sneaker"), SNEAKER),
    ((9, "high_heel"), HIGH_HEEL),
    ((6, "sandal"), SANDAL),
    ((4, "boot"), BOOT),
    ((10, "footprints"), FOOTPRINTS),
    ((18, "bust_in_silhouette"), BUST_IN_SILHOUETTE),
    ((19, "busts_in_silhouette"), BUSTS_IN_SILHOUETTE),
    ((9, "boy_tone1"), BOY_TONE1),
    ((9, "boy_tone2"), BOY_TONE2),
    ((9, "boy_tone3"), BOY_TONE3),
    ((9, "boy_tone4"), BOY_TONE4),
    ((9, "boy_tone5"), BOY_TONE5),
    ((3, "boy"), BOY),
    ((10, "girl_tone1"), GIRL_TONE1),
    ((10, "girl_tone2"), GIRL_TONE2),
    ((10, "girl_tone3"), GIRL_TONE3),
    ((10, "girl_tone4"), GIRL_TONE4),
    ((10, "girl_tone5"), GIRL_TONE5),
    ((4, "girl"), GIRL),
    ((16, "man_farmer_tone1"), MAN_FARMER_TONE1),
    ((14, "man_cook_tone1"), MAN_COOK_TONE1),
    ((22, "man_feeding_baby_tone1"), MAN_FEEDING_BABY_TONE1),
    ((17, "man_student_tone1"), MAN_STUDENT_TONE1),
    ((16, "man_singer_tone1"), MAN_SINGER_TONE1),
    ((16, "man_artist_tone1"), MAN_ARTIST_TONE1),
    ((17, "man_teacher_tone1"), MAN_TEACHER_TONE1),
    ((24, "man_factory_worker_tone1"), MAN_FACTORY_WORKER_TONE1),
    ((22, "man_technologist_tone1"), MAN_TECHNOLOGIST_TONE1),
    ((23, "man_office_worker_tone1"), MAN_OFFICE_WORKER_TONE1),
    ((18, "man_mechanic_tone1"), MAN_MECHANIC_TONE1),
    ((19, "man_scientist_tone1"), MAN_SCIENTIST_TONE1),
    ((19, "man_astronaut_tone1"), MAN_ASTRONAUT_TONE1),
    ((21, "man_firefighter_tone1"), MAN_FIREFIGHTER_TONE1),
    ((29, "two_men_holding_hands_tone1-2"), TWO_MEN_HOLDING_HANDS_TONE1_2),
    ((29, "two_men_holding_hands_tone1-3"), TWO_MEN_HOLDING_HANDS_TONE1_3),
    ((29, "two_men_holding_hands_tone1-4"), TWO_MEN_HOLDING_HANDS_TONE1_4),
    ((29, "two_men_holding_hands_tone1-5"), TWO_MEN_HOLDING_HANDS_TONE1_5),
    ((27, "man_with_probing_cane_tone1"), MAN_WITH_PROBING_CANE_TONE1),
    ((25, "man_with_white_cane_tone1"), MAN_WITH_WHITE_CANE_TONE1),
    ((20, "man_red_haired_tone1"), MAN_RED_HAIRED_TONE1),
    ((22, "man_curly_haired_tone1"), MAN_CURLY_HAIRED_TONE1),
    ((14, "man_bald_tone1"), MAN_BALD_TONE1),
    ((22, "man_white_haired_tone1"), MAN_WHITE_HAIRED_TONE1),
    ((33, "man_in_motorized_wheelchair_tone1"), MAN_IN_MOTORIZED_WHEELCHAIR_TONE1),
    ((30, "man_in_manual_wheelchair_tone1"), MAN_IN_MANUAL_WHEELCHAIR_TONE1),
    ((23, "man_health_worker_tone1"), MAN_HEALTH_WORKER_TONE1),
    ((15, "man_judge_tone1"), MAN_JUDGE_TONE1),
    ((15, "man_pilot_tone1"), MAN_PILOT_TONE1),
    ((26, "couple_with_heart_mm_tone1"), COUPLE_WITH_HEART_MM_TONE1),
    ((28, "couple_with_heart_mm_tone1-2"), COUPLE_WITH_HEART_MM_TONE1_2),
    ((28, "couple_with_heart_mm_tone1-3"), COUPLE_WITH_HEART_MM_TONE1_3),
    ((28, "couple_with_heart_mm_tone1-4"), COUPLE_WITH_HEART_MM_TONE1_4),
    ((28, "couple_with_heart_mm_tone1-5"), COUPLE_WITH_HEART_MM_TONE1_5),
    ((13, "kiss_mm_tone1"), KISS_MM_TONE1),
    ((15, "kiss_mm_tone1-2"), KISS_MM_TONE1_2),
    ((15, "kiss_mm_tone1-3"), KISS_MM_TONE1_3),
    ((15, "kiss_mm_tone1-4"), KISS_MM_TONE1_4),
    ((15, "kiss_mm_tone1-5"), KISS_MM_TONE1_5),
    ((9, "man_tone1"), MAN_TONE1),
    ((16, "man_farmer_tone2"), MAN_FARMER_TONE2),
    ((14, "man_cook_tone2"), MAN_COOK_TONE2),
    ((22, "man_feeding_baby_tone2"), MAN_FEEDING_BABY_TONE2),
    ((17, "man_student_tone2"), MAN_STUDENT_TONE2),
    ((16, "man_singer_tone2"), MAN_SINGER_TONE2),
    ((16, "man_artist_tone2"), MAN_ARTIST_TONE2),
    ((17, "man_teacher_tone2"), MAN_TEACHER_TONE2),
    ((24, "man_factory_worker_tone2"), MAN_FACTORY_WORKER_TONE2),
    ((22, "man_technologist_tone2"), MAN_TECHNOLOGIST_TONE2),
    ((23, "man_office_worker_tone2"), MAN_OFFICE_WORKER_TONE2),
    ((18, "man_mechanic_tone2"), MAN_MECHANIC_TONE2),
    ((19, "man_scientist_tone2"), MAN_SCIENTIST_TONE2),
    ((19, "man_astronaut_tone2"), MAN_ASTRONAUT_TONE2),
    ((21, "man_firefighter_tone2"), MAN_FIREFIGHTER_TONE2),
    ((29, "two_men_holding_hands_tone2-1"), TWO_MEN_HOLDING_HANDS_TONE2_1),
    ((29, "two_men_holding_hands_tone2-3"), TWO_MEN_HOLDING_HANDS_TONE2_3),
    ((29, "two_men_holding_hands_tone2-4"), TWO_MEN_HOLDING_HANDS_TONE2_4),
    ((29, "two_men_holding_hands_tone2-5"), TWO_MEN_HOLDING_HANDS_TONE2_5),
    ((27, "man_with_probing_cane_tone2"), MAN_WITH_PROBING_CANE_TONE2),
    ((25, "man_with_white_cane_tone2"), MAN_WITH_WHITE_CANE_TONE2),
    ((20, "man_red_haired_tone2"), MAN_RED_HAIRED_TONE2),
    ((22, "man_curly_haired_tone2"), MAN_CURLY_HAIRED_TONE2),
    ((14, "man_bald_tone2"), MAN_BALD_TONE2),
    ((22, "man_white_haired_tone2"), MAN_WHITE_HAIRED_TONE2),
    ((33, "man_in_motorized_wheelchair_tone2"), MAN_IN_MOTORIZED_WHEELCHAIR_TONE2),
    ((30, "man_in_manual_wheelchair_tone2"), MAN_IN_MANUAL_WHEELCHAIR_TONE2),
    ((23, "man_health_worker_tone2"), MAN_HEALTH_WORKER_TONE2),
    ((15, "man_judge_tone2"), MAN_JUDGE_TONE2),
    ((15, "man_pilot_tone2"), MAN_PILOT_TONE2),
    ((28, "couple_with_heart_mm_tone2-1"), COUPLE_WITH_HEART_MM_TONE2_1),
    ((26, "couple_with_heart_mm_tone2"), COUPLE_WITH_HEART_MM_TONE2),
    ((28, "couple_with_heart_mm_tone2-3"), COUPLE_WITH_HEART_MM_TONE2_3),
    ((28, "couple_with_heart_mm_tone2-4"), COUPLE_WITH_HEART_MM_TONE2_4),
    ((28, "couple_with_heart_mm_tone2-5"), COUPLE_WITH_HEART_MM_TONE2_5),
    ((15, "kiss_mm_tone2-1"), KISS_MM_TONE2_1),
    ((13, "kiss_mm_tone2"), KISS_MM_TONE2),
    ((15, "kiss_mm_tone2-3"), KISS_MM_TONE2_3),
    ((15, "kiss_mm_tone2-4"), KISS_MM_TONE2_4),
    ((15, "kiss_mm_tone2-5"), KISS_MM_TONE2_5),
    ((9, "man_tone2"), MAN_TONE2),
    ((16, "man_farmer_tone3"), MAN_FARMER_TONE3),
    ((14, "man_cook_tone3"), MAN_COOK_TONE3),
    ((22, "man_feeding_baby_tone3"), MAN_FEEDING_BABY_TONE3),
    ((17, "man_student_tone3"), MAN_STUDENT_TONE3),
    ((16, "man_singer_tone3"), MAN_SINGER_TONE3),
    ((16, "man_artist_tone3"), MAN_ARTIST_TONE3),
    ((17, "man_teacher_tone3"), MAN_TEACHER_TONE3),
    ((24, "man_factory_worker_tone3"), MAN_FACTORY_WORKER_TONE3),
    ((22, "man_technologist_tone3"), MAN_TECHNOLOGIST_TONE3),
    ((23, "man_office_worker_tone3"), MAN_OFFICE_WORKER_TONE3),
    ((18, "man_mechanic_tone3"), MAN_MECHANIC_TONE3),
    ((19, "man_scientist_tone3"), MAN_SCIENTIST_TONE3),
    ((19, "man_astronaut_tone3"), MAN_ASTRONAUT_TONE3),
    ((21, "man_firefighter_tone3"), MAN_FIREFIGHTER_TONE3),
    ((29, "two_men_holding_hands_tone3-1"), TWO_MEN_HOLDING_HANDS_TONE3_1),
    ((29, "two_men_holding_hands_tone3-2"), TWO_MEN_HOLDING_HANDS_TONE3_2),
    ((29, "two_men_holding_hands_tone3-4"), TWO_MEN_HOLDING_HANDS_TONE3_4),
    ((29, "two_men_holding_hands_tone3-5"), TWO_MEN_HOLDING_HANDS_TONE3_5),
    ((27, "man_with_probing_cane_tone3"), MAN_WITH_PROBING_CANE_TONE3),
    ((25, "man_with_white_cane_tone3"), MAN_WITH_WHITE_CANE_TONE3),
    ((20, "man_red_haired_tone3"), MAN_RED_HAIRED_TONE3),
    ((22, "man_curly_haired_tone3"), MAN_CURLY_HAIRED_TONE3),
    ((14, "man_bald_tone3"), MAN_BALD_TONE3),
    ((22, "man_white_haired_tone3"), MAN_WHITE_HAIRED_TONE3),
    ((33, "man_in_motorized_wheelchair_tone3"), MAN_IN_MOTORIZED_WHEELCHAIR_TONE3),
    ((30, "man_in_manual_wheelchair_tone3"), MAN_IN_MANUAL_WHEELCHAIR_TONE3),
    ((23, "man_health_worker_tone3"), MAN_HEALTH_WORKER_TONE3),
    ((15, "man_judge_tone3"), MAN_JUDGE_TONE3),
    ((15, "man_pilot_tone3"), MAN_PILOT_TONE3),
    ((28, "couple_with_heart_mm_tone3-1"), COUPLE_WITH_HEART_MM_TONE3_1),
    ((28, "couple_with_heart_mm_tone3-2"), COUPLE_WITH_HEART_MM_TONE3_2),
    ((26, "couple_with_heart_mm_tone3"), COUPLE_WITH_HEART_MM_TONE3),
    ((28, "couple_with_heart_mm_tone3-4"), COUPLE_WITH_HEART_MM_TONE3_4),
    ((28, "couple_with_heart_mm_tone3-5"), COUPLE_WITH_HEART_MM_TONE3_5),
    ((15, "kiss_mm_tone3-1"), KISS_MM_TONE3_1),
    ((15, "kiss_mm_tone3-2"), KISS_MM_TONE3_2),
    ((13, "kiss_mm_tone3"), KISS_MM_TONE3),
    ((15, "kiss_mm_tone3-4"), KISS_MM_TONE3_4),
    ((15, "kiss_mm_tone3-5"), KISS_MM_TONE3_5),
    ((9, "man_tone3"), MAN_TONE3),
    ((16, "man_farmer_tone4"), MAN_FARMER_TONE4),
    ((14, "man_cook_tone4"), MAN_COOK_TONE4),
    ((22, "man_feeding_baby_tone4"), MAN_FEEDING_BABY_TONE4),
    ((17, "man_student_tone4"), MAN_STUDENT_TONE4),
    ((16, "man_singer_tone4"), MAN_SINGER_TONE4),
    ((16, "man_artist_tone4"), MAN_ARTIST_TONE4),
    ((17, "man_teacher_tone4"), MAN_TEACHER_TONE4),
    ((24, "man_factory_worker_tone4"), MAN_FACTORY_WORKER_TONE4),
    ((22, "man_technologist_tone4"), MAN_TECHNOLOGIST_TONE4),
    ((23, "man_office_worker_tone4"), MAN_OFFICE_WORKER_TONE4),
    ((18, "man_mechanic_tone4"), MAN_MECHANIC_TONE4),
    ((19, "man_scientist_tone4"), MAN_SCIENTIST_TONE4),
    ((19, "man_astronaut_tone4"), MAN_ASTRONAUT_TONE4),
    ((21, "man_firefighter_tone4"), MAN_FIREFIGHTER_TONE4),
    ((29, "two_men_holding_hands_tone4-1"), TWO_MEN_HOLDING_HANDS_TONE4_1),
    ((29, "two_men_holding_hands_tone4-2"), TWO_MEN_HOLDING_HANDS_TONE4_2),
    ((29, "two_men_holding_hands_tone4-3"), TWO_MEN_HOLDING_HANDS_TONE4_3),
    ((29, "two_men_holding_hands_tone4-5"), TWO_MEN_HOLDING_HANDS_TONE4_5),
    ((27, "man_with_probing_cane_tone4"), MAN_WITH_PROBING_CANE_TONE4),
    ((25, "man_with_white_cane_tone4"), MAN_WITH_WHITE_CANE_TONE4),
    ((20, "man_red_haired_tone4"), MAN_RED_HAIRED_TONE4),
    ((22, "man_curly_haired_tone4"), MAN_CURLY_HAIRED_TONE4),
    ((14, "man_bald_tone4"), MAN_BALD_TONE4),
    ((22, "man_white_haired_tone4"), MAN_WHITE_HAIRED_TONE4),
    ((33, "man_in_motorized_wheelchair_tone4"), MAN_IN_MOTORIZED_WHEELCHAIR_TONE4),
    ((30, "man_in_manual_wheelchair_tone4"), MAN_IN_MANUAL_WHEELCHAIR_TONE4),
    ((23, "man_health_worker_tone4"), MAN_HEALTH_WORKER_TONE4),
    ((15, "man_judge_tone4"), MAN_JUDGE_TONE4),
    ((15, "man_pilot_tone4"), MAN_PILOT_TONE4),
    ((28, "couple_with_heart_mm_tone4-1"), COUPLE_WITH_HEART_MM_TONE4_1),
    ((28, "couple_with_heart_mm_tone4-2"), COUPLE_WITH_HEART_MM_TONE4_2),
    ((28, "couple_with_heart_mm_tone4-3"), COUPLE_WITH_HEART_MM_TONE4_3),
    ((26, "couple_with_heart_mm_tone4"), COUPLE_WITH_HEART_MM_TONE4),
    ((28, "couple_with_heart_mm_tone4-5"), COUPLE_WITH_HEART_MM_TONE4_5),
    ((15, "kiss_mm_tone4-1"), KISS_MM_TONE4_1),
    ((15, "kiss_mm_tone4-2"), KISS_MM_TONE4_2),
    ((15, "kiss_mm_tone4-3"), KISS_MM_TONE4_3),
    ((13, "kiss_mm_tone4"), KISS_MM_TONE4),
    ((15, "kiss_mm_tone4-5"), KISS_MM_TONE4_5),
    ((9, "man_tone4"), MAN_TONE4),
    ((16, "man_farmer_tone5"), MAN_FARMER_TONE5),
    ((14, "man_cook_tone5"), MAN_COOK_TONE5),
    ((22, "man_feeding_baby_tone5"), MAN_FEEDING_BABY_TONE5),
    ((17, "man_student_tone5"), MAN_STUDENT_TONE5),
    ((16, "man_singer_tone5"), MAN_SINGER_TONE5),
    ((16, "man_artist_tone5"), MAN_ARTIST_TONE5),
    ((17, "man_teacher_tone5"), MAN_TEACHER_TONE5),
    ((24, "man_factory_worker_tone5"), MAN_FACTORY_WORKER_TONE5),
    ((22, "man_technologist_tone5"), MAN_TECHNOLOGIST_TONE5),
    ((23, "man_office_worker_tone5"), MAN_OFFICE_WORKER_TONE5),
    ((18, "man_mechanic_tone5"), MAN_MECHANIC_TONE5),
    ((19, "man_scientist_tone5"), MAN_SCIENTIST_TONE5),
    ((19, "man_astronaut_tone5"), MAN_ASTRONAUT_TONE5),
    ((21, "man_firefighter_tone5"), MAN_FIREFIGHTER_TONE5),
    ((29, "two_men_holding_hands_tone5-1"), TWO_MEN_HOLDING_HANDS_TONE5_1),
    ((29, "two_men_holding_hands_tone5-2"), TWO_MEN_HOLDING_HANDS_TONE5_2),
    ((29, "two_men_holding_hands_tone5-3"), TWO_MEN_HOLDING_HANDS_TONE5_3),
    ((29, "two_men_holding_hands_tone5-4"), TWO_MEN_HOLDING_HANDS_TONE5_4),
    ((27, "man_with_probing_cane_tone5"), MAN_WITH_PROBING_CANE_TONE5),
    ((25, "man_with_white_cane_tone5"), MAN_WITH_WHITE_CANE_TONE5),
    ((20, "man_red_haired_tone5"), MAN_RED_HAIRED_TONE5),
    ((22, "man_curly_haired_tone5"), MAN_CURLY_HAIRED_TONE5),
    ((14, "man_bald_tone5"), MAN_BALD_TONE5),
    ((22, "man_white_haired_tone5"), MAN_WHITE_HAIRED_TONE5),
    ((33, "man_in_motorized_wheelchair_tone5"), MAN_IN_MOTORIZED_WHEELCHAIR_TONE5),
    ((30, "man_in_manual_wheelchair_tone5"), MAN_IN_MANUAL_WHEELCHAIR_TONE5),
    ((23, "man_health_worker_tone5"), MAN_HEALTH_WORKER_TONE5),
    ((15, "man_judge_tone5"), MAN_JUDGE_TONE5),
    ((15, "man_pilot_tone5"), MAN_PILOT_TONE5),
    ((28, "couple_with_heart_mm_tone5-1"), COUPLE_WITH_HEART_MM_TONE5_1),
    ((28, "couple_with_heart_mm_tone5-2"), COUPLE_WITH_HEART_MM_TONE5_2),
    ((28, "couple_with_heart_mm_tone5-3"), COUPLE_WITH_HEART_MM_TONE5_3),
    ((28, "couple_with_heart_mm_tone5-4"), COUPLE_WITH_HEART_MM_TONE5_4),
    ((26, "couple_with_heart_mm_tone5"), COUPLE_WITH_HEART_MM_TONE5),
    ((15, "kiss_mm_tone5-1"), KISS_MM_TONE5_1),
    ((15, "kiss_mm_tone5-2"), KISS_MM_TONE5_2),
    ((15, "kiss_mm_tone5-3"), KISS_MM_TONE5_3),
    ((15, "kiss_mm_tone5-4"), KISS_MM_TONE5_4),
    ((13, "kiss_mm_tone5"), KISS_MM_TONE5),
    ((9, "man_tone5"), MAN_TONE5),
    ((10, "man_farmer"), MAN_FARMER),
    ((8, "man_cook"), MAN_COOK),
    ((16, "man_feeding_baby"), MAN_FEEDING_BABY),
    ((11, "man_student"), MAN_STUDENT),
    ((10, "man_singer"), MAN_SINGER),
    ((10, "man_artist"), MAN_ARTIST),
    ((11, "man_teacher"), MAN_TEACHER),
    ((18, "man_factory_worker"), MAN_FACTORY_WORKER),
    ((10, "family_mbb"), FAMILY_MBB),
    ((9, "family_mb"), FAMILY_MB),
    ((10, "family_mgb"), FAMILY_MGB),
    ((10, "family_mgg"), FAMILY_MGG),
    ((9, "family_mg"), FAMILY_MG),
    ((11, "family_mmbb"), FAMILY_MMBB),
    ((10, "family_mmb"), FAMILY_MMB),
    ((11, "family_mmgb"), FAMILY_MMGB),
    ((11, "family_mmgg"), FAMILY_MMGG),
    ((10, "family_mmg"), FAMILY_MMG),
    ((11, "family_mwbb"), FAMILY_MWBB),
    ((10, "family_mwb"), FAMILY_MWB),
    ((11, "family_mwgb"), FAMILY_MWGB),
    ((11, "family_mwgg"), FAMILY_MWGG),
    ((10, "family_mwg"), FAMILY_MWG),
    ((16, "man_technologist"), MAN_TECHNOLOGIST),
    ((17, "man_office_worker"), MAN_OFFICE_WORKER),
    ((12, "man_mechanic"), MAN_MECHANIC),
    ((13, "man_scientist"), MAN_SCIENTIST),
    ((13, "man_astronaut"), MAN_ASTRONAUT),
    ((15, "man_firefighter"), MAN_FIREFIGHTER),
    ((21, "man_with_probing_cane"), MAN_WITH_PROBING_CANE),
    ((19, "man_with_white_cane"), MAN_WITH_WHITE_CANE),
    ((14, "man_red_haired"), MAN_RED_HAIRED),
    ((16, "man_curly_haired"), MAN_CURLY_HAIRED),
    ((8, "man_bald"), MAN_BALD),
    ((16, "man_white_haired"), MAN_WHITE_HAIRED),
    ((27, "man_in_motorized_wheelchair"), MAN_IN_MOTORIZED_WHEELCHAIR),
    ((24, "man_in_manual_wheelchair"), MAN_IN_MANUAL_WHEELCHAIR),
    ((17, "man_health_worker"), MAN_HEALTH_WORKER),
    ((9, "man_judge"), MAN_JUDGE),
    ((9, "man_pilot"), MAN_PILOT),
    ((20, "couple_with_heart_mm"), COUPLE_WITH_HEART_MM),
    ((7, "kiss_mm"), KISS_MM),
    ((3, "man"), MAN),
    ((18, "woman_farmer_tone1"), WOMAN_FARMER_TONE1),
    ((16, "woman_cook_tone1"), WOMAN_COOK_TONE1),
    ((24, "woman_feeding_baby_tone1"), WOMAN_FEEDING_BABY_TONE1),
    ((19, "woman_student_tone1"), WOMAN_STUDENT_TONE1),
    ((18, "woman_singer_tone1"), WOMAN_SINGER_TONE1),
    ((18, "woman_artist_tone1"), WOMAN_ARTIST_TONE1),
    ((19, "woman_teacher_tone1"), WOMAN_TEACHER_TONE1),
    ((26, "woman_factory_worker_tone1"), WOMAN_FACTORY_WORKER_TONE1),
    ((24, "woman_technologist_tone1"), WOMAN_TECHNOLOGIST_TONE1),
    ((25, "woman_office_worker_tone1"), WOMAN_OFFICE_WORKER_TONE1),
    ((20, "woman_mechanic_tone1"), WOMAN_MECHANIC_TONE1),
    ((21, "woman_scientist_tone1"), WOMAN_SCIENTIST_TONE1),
    ((21, "woman_astronaut_tone1"), WOMAN_ASTRONAUT_TONE1),
    ((23, "woman_firefighter_tone1"), WOMAN_FIREFIGHTER_TONE1),
    ((14, "couple_tone1-2"), COUPLE_TONE1_2),
    ((14, "couple_tone1-3"), COUPLE_TONE1_3),
    ((14, "couple_tone1-4"), COUPLE_TONE1_4),
    ((14, "couple_tone1-5"), COUPLE_TONE1_5),
    ((31, "two_women_holding_hands_tone1-2"), TWO_WOMEN_HOLDING_HANDS_TONE1_2),
    ((31, "two_women_holding_hands_tone1-3"), TWO_WOMEN_HOLDING_HANDS_TONE1_3),
    ((31, "two_women_holding_hands_tone1-4"), TWO_WOMEN_HOLDING_HANDS_TONE1_4),
    ((31, "two_women_holding_hands_tone1-5"), TWO_WOMEN_HOLDING_HANDS_TONE1_5),
    ((29, "woman_with_probing_cane_tone1"), WOMAN_WITH_PROBING_CANE_TONE1),
    ((27, "woman_with_white_cane_tone1"), WOMAN_WITH_WHITE_CANE_TONE1),
    ((22, "woman_red_haired_tone1"), WOMAN_RED_HAIRED_TONE1),
    ((24, "woman_curly_haired_tone1"), WOMAN_CURLY_HAIRED_TONE1),
    ((16, "woman_bald_tone1"), WOMAN_BALD_TONE1),
    ((24, "woman_white_haired_tone1"), WOMAN_WHITE_HAIRED_TONE1),
    ((35, "woman_in_motorized_wheelchair_tone1"), WOMAN_IN_MOTORIZED_WHEELCHAIR_TONE1),
    ((32, "woman_in_manual_wheelchair_tone1"), WOMAN_IN_MANUAL_WHEELCHAIR_TONE1),
    ((25, "woman_health_worker_tone1"), WOMAN_HEALTH_WORKER_TONE1),
    ((17, "woman_judge_tone1"), WOMAN_JUDGE_TONE1),
    ((17, "woman_pilot_tone1"), WOMAN_PILOT_TONE1),
    ((26, "couple_with_heart_mw_tone1"), COUPLE_WITH_HEART_MW_TONE1),
    ((26, "couple_with_heart_wm_tone1"), COUPLE_WITH_HEART_WM_TONE1),
    ((28, "couple_with_heart_mw_tone1-2"), COUPLE_WITH_HEART_MW_TONE1_2),
    ((28, "couple_with_heart_wm_tone1-2"), COUPLE_WITH_HEART_WM_TONE1_2),
    ((28, "couple_with_heart_mw_tone1-3"), COUPLE_WITH_HEART_MW_TONE1_3),
    ((28, "couple_with_heart_wm_tone1-3"), COUPLE_WITH_HEART_WM_TONE1_3),
    ((28, "couple_with_heart_mw_tone1-4"), COUPLE_WITH_HEART_MW_TONE1_4),
    ((28, "couple_with_heart_wm_tone1-4"), COUPLE_WITH_HEART_WM_TONE1_4),
    ((28, "couple_with_heart_mw_tone1-5"), COUPLE_WITH_HEART_MW_TONE1_5),
    ((28, "couple_with_heart_wm_tone1-5"), COUPLE_WITH_HEART_WM_TONE1_5),
    ((26, "couple_with_heart_ww_tone1"), COUPLE_WITH_HEART_WW_TONE1),
    ((28, "couple_with_heart_ww_tone1-2"), COUPLE_WITH_HEART_WW_TONE1_2),
    ((28, "couple_with_heart_ww_tone1-3"), COUPLE_WITH_HEART_WW_TONE1_3),
    ((28, "couple_with_heart_ww_tone1-4"), COUPLE_WITH_HEART_WW_TONE1_4),
    ((28, "couple_with_heart_ww_tone1-5"), COUPLE_WITH_HEART_WW_TONE1_5),
    ((13, "kiss_mw_tone1"), KISS_MW_TONE1),
    ((13, "kiss_wm_tone1"), KISS_WM_TONE1),
    ((15, "kiss_mw_tone1-2"), KISS_MW_TONE1_2),
    ((15, "kiss_wm_tone1-2"), KISS_WM_TONE1_2),
    ((15, "kiss_mw_tone1-3"), KISS_MW_TONE1_3),
    ((15, "kiss_wm_tone1-3"), KISS_WM_TONE1_3),
    ((15, "kiss_mw_tone1-4"), KISS_MW_TONE1_4),
    ((15, "kiss_wm_tone1-4"), KISS_WM_TONE1_4),
    ((15, "kiss_mw_tone1-5"), KISS_MW_TONE1_5),
    ((15, "kiss_wm_tone1-5"), KISS_WM_TONE1_5),
    ((13, "kiss_ww_tone1"), KISS_WW_TONE1),
    ((15, "kiss_ww_tone1-2"), KISS_WW_TONE1_2),
    ((15, "kiss_ww_tone1-3"), KISS_WW_TONE1_3),
    ((15, "kiss_ww_tone1-4"), KISS_WW_TONE1_4),
    ((15, "kiss_ww_tone1-5"), KISS_WW_TONE1_5),
    ((11, "woman_tone1"), WOMAN_TONE1),
    ((18, "woman_farmer_tone2"), WOMAN_FARMER_TONE2),
    ((16, "woman_cook_tone2"), WOMAN_COOK_TONE2),
    ((24, "woman_feeding_baby_tone2"), WOMAN_FEEDING_BABY_TONE2),
    ((19, "woman_student_tone2"), WOMAN_STUDENT_TONE2),
    ((18, "woman_singer_tone2"), WOMAN_SINGER_TONE2),
    ((18, "woman_artist_tone2"), WOMAN_ARTIST_TONE2),
    ((19, "woman_teacher_tone2"), WOMAN_TEACHER_TONE2),
    ((26, "woman_factory_worker_tone2"), WOMAN_FACTORY_WORKER_TONE2),
    ((24, "woman_technologist_tone2"), WOMAN_TECHNOLOGIST_TONE2),
    ((25, "woman_office_worker_tone2"), WOMAN_OFFICE_WORKER_TONE2),
    ((20, "woman_mechanic_tone2"), WOMAN_MECHANIC_TONE2),
    ((21, "woman_scientist_tone2"), WOMAN_SCIENTIST_TONE2),
    ((21, "woman_astronaut_tone2"), WOMAN_ASTRONAUT_TONE2),
    ((23, "woman_firefighter_tone2"), WOMAN_FIREFIGHTER_TONE2),
    ((14, "couple_tone2-1"), COUPLE_TONE2_1),
    ((14, "couple_tone2-3"), COUPLE_TONE2_3),
    ((14, "couple_tone2-4"), COUPLE_TONE2_4),
    ((14, "couple_tone2-5"), COUPLE_TONE2_5),
    ((31, "two_women_holding_hands_tone2-1"), TWO_WOMEN_HOLDING_HANDS_TONE2_1),
    ((31, "two_women_holding_hands_tone2-3"), TWO_WOMEN_HOLDING_HANDS_TONE2_3),
    ((31, "two_women_holding_hands_tone2-4"), TWO_WOMEN_HOLDING_HANDS_TONE2_4),
    ((31, "two_women_holding_hands_tone2-5"), TWO_WOMEN_HOLDING_HANDS_TONE2_5),
    ((29, "woman_with_probing_cane_tone2"), WOMAN_WITH_PROBING_CANE_TONE2),
    ((27, "woman_with_white_cane_tone2"), WOMAN_WITH_WHITE_CANE_TONE2),
    ((22, "woman_red_haired_tone2"), WOMAN_RED_HAIRED_TONE2),
    ((24, "woman_curly_haired_tone2"), WOMAN_CURLY_HAIRED_TONE2),
    ((16, "woman_bald_tone2"), WOMAN_BALD_TONE2),
    ((24, "woman_white_haired_tone2"), WOMAN_WHITE_HAIRED_TONE2),
    ((35, "woman_in_motorized_wheelchair_tone2"), WOMAN_IN_MOTORIZED_WHEELCHAIR_TONE2),
    ((32, "woman_in_manual_wheelchair_tone2"), WOMAN_IN_MANUAL_WHEELCHAIR_TONE2),
    ((25, "woman_health_worker_tone2"), WOMAN_HEALTH_WORKER_TONE2),
    ((17, "woman_judge_tone2"), WOMAN_JUDGE_TONE2),
    ((17, "woman_pilot_tone2"), WOMAN_PILOT_TONE2),
    ((28, "couple_with_heart_mw_tone2-1"), COUPLE_WITH_HEART_MW_TONE2_1),
    ((28, "couple_with_heart_wm_tone2-1"), COUPLE_WITH_HEART_WM_TONE2_1),
    ((26, "couple_with_heart_mw_tone2"), COUPLE_WITH_HEART_MW_TONE2),
    ((26, "couple_with_heart_wm_tone2"), COUPLE_WITH_HEART_WM_TONE2),
    ((28, "couple_with_heart_mw_tone2-3"), COUPLE_WITH_HEART_MW_TONE2_3),
    ((28, "couple_with_heart_wm_tone2-3"), COUPLE_WITH_HEART_WM_TONE2_3),
    ((28, "couple_with_heart_mw_tone2-4"), COUPLE_WITH_HEART_MW_TONE2_4),
    ((28, "couple_with_heart_wm_tone2-4"), COUPLE_WITH_HEART_WM_TONE2_4),
    ((28, "couple_with_heart_mw_tone2-5"), COUPLE_WITH_HEART_MW_TONE2_5),
    ((28, "couple_with_heart_wm_tone2-5"), COUPLE_WITH_HEART_WM_TONE2_5),
    ((28, "couple_with_heart_ww_tone2-1"), COUPLE_WITH_HEART_WW_TONE2_1),
    ((26, "couple_with_heart_ww_tone2"), COUPLE_WITH_HEART_WW_TONE2),
    ((28, "couple_with_heart_ww_tone2-3"), COUPLE_WITH_HEART_WW_TONE2_3),
    ((28, "couple_with_heart_ww_tone2-4"), COUPLE_WITH_HEART_WW_TONE2_4),
    ((28, "couple_with_heart_ww_tone2-5"), COUPLE_WITH_HEART_WW_TONE2_5),
    ((15, "kiss_mw_tone2-1"), KISS_MW_TONE2_1),
    ((15, "kiss_wm_tone2-1"), KISS_WM_TONE2_1),
    ((13, "kiss_mw_tone2"), KISS_MW_TONE2),
    ((13, "kiss_wm_tone2"), KISS_WM_TONE2),
    ((15, "kiss_mw_tone2-3"), KISS_MW_TONE2_3),
    ((15, "kiss_wm_tone2-3"), KISS_WM_TONE2_3),
    ((15, "kiss_mw_tone2-4"), KISS_MW_TONE2_4),
    ((15, "kiss_wm_tone2-4"), KISS_WM_TONE2_4),
    ((15, "kiss_mw_tone2-5"), KISS_MW_TONE2_5),
    ((15, "kiss_wm_tone2-5"), KISS_WM_TONE2_5),
    ((15, "kiss_ww_tone2-1"), KISS_WW_TONE2_1),
    ((13, "kiss_ww_tone2"), KISS_WW_TONE2),
    ((15, "kiss_ww_tone2-3"), KISS_WW_TONE2_3),
    ((15, "kiss_ww_tone2-4"), KISS_WW_TONE2_4),
    ((15, "kiss_ww_tone2-5"), KISS_WW_TONE2_5),
    ((11, "woman_tone2"), WOMAN_TONE2),
    ((18, "woman_farmer_tone3"), WOMAN_FARMER_TONE3),
    ((16, "woman_cook_tone3"), WOMAN_COOK_TONE3),
    ((24, "woman_feeding_baby_tone3"), WOMAN_FEEDING_BABY_TONE3),
    ((19, "woman_student_tone3"), WOMAN_STUDENT_TONE3),
    ((18, "woman_singer_tone3"), WOMAN_SINGER_TONE3),
    ((18, "woman_artist_tone3"), WOMAN_ARTIST_TONE3),
    ((19, "woman_teacher_tone3"), WOMAN_TEACHER_TONE3),
    ((26, "woman_factory_worker_tone3"), WOMAN_FACTORY_WORKER_TONE3),
    ((24, "woman_technologist_tone3"), WOMAN_TECHNOLOGIST_TONE3),
    ((25, "woman_office_worker_tone3"), WOMAN_OFFICE_WORKER_TONE3),
    ((20, "woman_mechanic_tone3"), WOMAN_MECHANIC_TONE3),
    ((21, "woman_scientist_tone3"), WOMAN_SCIENTIST_TONE3),
    ((21, "woman_astronaut_tone3"), WOMAN_ASTRONAUT_TONE3),
    ((23, "woman_firefighter_tone3"), WOMAN_FIREFIGHTER_TONE3),
    ((14, "couple_tone3-1"), COUPLE_TONE3_1),
    ((14, "couple_tone3-2"), COUPLE_TONE3_2),
    ((14, "couple_tone3-4"), COUPLE_TONE3_4),
    ((14, "couple_tone3-5"), COUPLE_TONE3_5),
    ((31, "two_women_holding_hands_tone3-1"), TWO_WOMEN_HOLDING_HANDS_TONE3_1),
    ((31, "two_women_holding_hands_tone3-2"), TWO_WOMEN_HOLDING_HANDS_TONE3_2),
    ((31, "two_women_holding_hands_tone3-4"), TWO_WOMEN_HOLDING_HANDS_TONE3_4),
    ((31, "two_women_holding_hands_tone3-5"), TWO_WOMEN_HOLDING_HANDS_TONE3_5),
    ((29, "woman_with_probing_cane_tone3"), WOMAN_WITH_PROBING_CANE_TONE3),
    ((27, "woman_with_white_cane_tone3"), WOMAN_WITH_WHITE_CANE_TONE3),
    ((22, "woman_red_haired_tone3"), WOMAN_RED_HAIRED_TONE3),
    ((24, "woman_curly_haired_tone3"), WOMAN_CURLY_HAIRED_TONE3),
    ((16, "woman_bald_tone3"), WOMAN_BALD_TONE3),
    ((24, "woman_white_haired_tone3"), WOMAN_WHITE_HAIRED_TONE3),
    ((35, "woman_in_motorized_wheelchair_tone3"), WOMAN_IN_MOTORIZED_WHEELCHAIR_TONE3),
    ((32, "woman_in_manual_wheelchair_tone3"), WOMAN_IN_MANUAL_WHEELCHAIR_TONE3),
    ((25, "woman_health_worker_tone3"), WOMAN_HEALTH_WORKER_TONE3),
    ((17, "woman_judge_tone3"), WOMAN_JUDGE_TONE3),
    ((17, "woman_pilot_tone3"), WOMAN_PILOT_TONE3),
    ((28, "couple_with_heart_mw_tone3-1"), COUPLE_WITH_HEART_MW_TONE3_1),
    ((28, "couple_with_heart_wm_tone3-1"), COUPLE_WITH_HEART_WM_TONE3_1),
    ((28, "couple_with_heart_mw_tone3-2"), COUPLE_WITH_HEART_MW_TONE3_2),
    ((28, "couple_with_heart_wm_tone3-2"), COUPLE_WITH_HEART_WM_TONE3_2),
    ((26, "couple_with_heart_mw_tone3"), COUPLE_WITH_HEART_MW_TONE3),
    ((26, "couple_with_heart_wm_tone3"), COUPLE_WITH_HEART_WM_TONE3),
    ((28, "couple_with_heart_mw_tone3-4"), COUPLE_WITH_HEART_MW_TONE3_4),
    ((28, "couple_with_heart_wm_tone3-4"), COUPLE_WITH_HEART_WM_TONE3_4),
    ((28, "couple_with_heart_mw_tone3-5"), COUPLE_WITH_HEART_MW_TONE3_5),
    ((28, "couple_with_heart_wm_tone3-5"), COUPLE_WITH_HEART_WM_TONE3_5),
    ((28, "couple_with_heart_ww_tone3-1"), COUPLE_WITH_HEART_WW_TONE3_1),
    ((28, "couple_with_heart_ww_tone3-2"), COUPLE_WITH_HEART_WW_TONE3_2),
    ((26, "couple_with_heart_ww_tone3"), COUPLE_WITH_HEART_WW_TONE3),
    ((28, "couple_with_heart_ww_tone3-4"), COUPLE_WITH_HEART_WW_TONE3_4),
    ((28, "couple_with_heart_ww_tone3-5"), COUPLE_WITH_HEART_WW_TONE3_5),
    ((15, "kiss_mw_tone3-1"), KISS_MW_TONE3_1),
    ((15, "kiss_wm_tone3-1"), KISS_WM_TONE3_1),
    ((15, "kiss_mw_tone3-2"), KISS_MW_TONE3_2),
    ((15, "kiss_wm_tone3-2"), KISS_WM_TONE3_2),
    ((13, "kiss_mw_tone3"), KISS_MW_TONE3),
    ((13, "kiss_wm_tone3"), KISS_WM_TONE3),
    ((15, "kiss_mw_tone3-4"), KISS_MW_TONE3_4),
    ((15, "kiss_wm_tone3-4"), KISS_WM_TONE3_4),
    ((15, "kiss_mw_tone3-5"), KISS_MW_TONE3_5),
    ((15, "kiss_wm_tone3-5"), KISS_WM_TONE3_5),
    ((15, "kiss_ww_tone3-1"), KISS_WW_TONE3_1),
    ((15, "kiss_ww_tone3-2"), KISS_WW_TONE3_2),
    ((13, "kiss_ww_tone3"), KISS_WW_TONE3),
    ((15, "kiss_ww_tone3-4"), KISS_WW_TONE3_4),
    ((15, "kiss_ww_tone3-5"), KISS_WW_TONE3_5),
    ((11, "woman_tone3"), WOMAN_TONE3),
    ((18, "woman_farmer_tone4"), WOMAN_FARMER_TONE4),
    ((16, "woman_cook_tone4"), WOMAN_COOK_TONE4),
    ((24, "woman_feeding_baby_tone4"), WOMAN_FEEDING_BABY_TONE4),
    ((19, "woman_student_tone4"), WOMAN_STUDENT_TONE4),
    ((18, "woman_singer_tone4"), WOMAN_SINGER_TONE4),
    ((18, "woman_artist_tone4"), WOMAN_ARTIST_TONE4),
    ((19, "woman_teacher_tone4"), WOMAN_TEACHER_TONE4),
    ((26, "woman_factory_worker_tone4"), WOMAN_FACTORY_WORKER_TONE4),
    ((24, "woman_technologist_tone4"), WOMAN_TECHNOLOGIST_TONE4),
    ((25, "woman_office_worker_tone4"), WOMAN_OFFICE_WORKER_TONE4),
    ((20, "woman_mechanic_tone4"), WOMAN_MECHANIC_TONE4),
    ((21, "woman_scientist_tone4"), WOMAN_SCIENTIST_TONE4),
    ((21, "woman_astronaut_tone4"), WOMAN_ASTRONAUT_TONE4),
    ((23, "woman_firefighter_tone4"), WOMAN_FIREFIGHTER_TONE4),
    ((14, "couple_tone4-1"), COUPLE_TONE4_1),
    ((14, "couple_tone4-2"), COUPLE_TONE4_2),
    ((14, "couple_tone4-3"), COUPLE_TONE4_3),
    ((14, "couple_tone4-5"), COUPLE_TONE4_5),
    ((31, "two_women_holding_hands_tone4-1"), TWO_WOMEN_HOLDING_HANDS_TONE4_1),
    ((31, "two_women_holding_hands_tone4-2"), TWO_WOMEN_HOLDING_HANDS_TONE4_2),
    ((31, "two_women_holding_hands_tone4-3"), TWO_WOMEN_HOLDING_HANDS_TONE4_3),
    ((31, "two_women_holding_hands_tone4-5"), TWO_WOMEN_HOLDING_HANDS_TONE4_5),
    ((29, "woman_with_probing_cane_tone4"), WOMAN_WITH_PROBING_CANE_TONE4),
    ((27, "woman_with_white_cane_tone4"), WOMAN_WITH_WHITE_CANE_TONE4),
    ((22, "woman_red_haired_tone4"), WOMAN_RED_HAIRED_TONE4),
    ((24, "woman_curly_haired_tone4"), WOMAN_CURLY_HAIRED_TONE4),
    ((16, "woman_bald_tone4"), WOMAN_BALD_TONE4),
    ((24, "woman_white_haired_tone4"), WOMAN_WHITE_HAIRED_TONE4),
    ((35, "woman_in_motorized_wheelchair_tone4"), WOMAN_IN_MOTORIZED_WHEELCHAIR_TONE4),
    ((32, "woman_in_manual_wheelchair_tone4"), WOMAN_IN_MANUAL_WHEELCHAIR_TONE4),
    ((25, "woman_health_worker_tone4"), WOMAN_HEALTH_WORKER_TONE4),
    ((17, "woman_judge_tone4"), WOMAN_JUDGE_TONE4),
    ((17, "woman_pilot_tone4"), WOMAN_PILOT_TONE4),
    ((28, "couple_with_heart_mw_tone4-1"), COUPLE_WITH_HEART_MW_TONE4_1),
    ((28, "couple_with_heart_wm_tone4-1"), COUPLE_WITH_HEART_WM_TONE4_1),
    ((28, "couple_with_heart_mw_tone4-2"), COUPLE_WITH_HEART_MW_TONE4_2),
    ((28, "couple_with_heart_wm_tone4-2"), COUPLE_WITH_HEART_WM_TONE4_2),
    ((28, "couple_with_heart_mw_tone4-3"), COUPLE_WITH_HEART_MW_TONE4_3),
    ((28, "couple_with_heart_wm_tone4-3"), COUPLE_WITH_HEART_WM_TONE4_3),
    ((26, "couple_with_heart_mw_tone4"), COUPLE_WITH_HEART_MW_TONE4),
    ((26, "couple_with_heart_wm_tone4"), COUPLE_WITH_HEART_WM_TONE4),
    ((28, "couple_with_heart_mw_tone4-5"), COUPLE_WITH_HEART_MW_TONE4_5),
    ((28, "couple_with_heart_wm_tone4-5"), COUPLE_WITH_HEART_WM_TONE4_5),
    ((28, "couple_with_heart_ww_tone4-1"), COUPLE_WITH_HEART_WW_TONE4_1),
    ((28, "couple_with_heart_ww_tone4-2"), COUPLE_WITH_HEART_WW_TONE4_2),
    ((28, "couple_with_heart_ww_tone4-3"), COUPLE_WITH_HEART_WW_TONE4_3),
    ((26, "couple_with_heart_ww_tone4"), COUPLE_WITH_HEART_WW_TONE4),
    ((28, "couple_with_heart_ww_tone4-5"), COUPLE_WITH_HEART_WW_TONE4_5),
    ((15, "kiss_mw_tone4-1"), KISS_MW_TONE4_1),
    ((15, "kiss_wm_tone4-1"), KISS_WM_TONE4_1),
    ((15, "kiss_mw_tone4-2"), KISS_MW_TONE4_2),
    ((15, "kiss_wm_tone4-2"), KISS_WM_TONE4_2),
    ((15, "kiss_mw_tone4-3"), KISS_MW_TONE4_3),
    ((15, "kiss_wm_tone4-3"), KISS_WM_TONE4_3),
    ((13, "kiss_mw_tone4"), KISS_MW_TONE4),
    ((13, "kiss_wm_tone4"), KISS_WM_TONE4),
    ((15, "kiss_mw_tone4-5"), KISS_MW_TONE4_5),
    ((15, "kiss_wm_tone4-5"), KISS_WM_TONE4_5),
    ((15, "kiss_ww_tone4-1"), KISS_WW_TONE4_1),
    ((15, "kiss_ww_tone4-2"), KISS_WW_TONE4_2),
    ((15, "kiss_ww_tone4-3"), KISS_WW_TONE4_3),
    ((13, "kiss_ww_tone4"), KISS_WW_TONE4),
    ((15, "kiss_ww_tone4-5"), KISS_WW_TONE4_5),
    ((11, "woman_tone4"), WOMAN_TONE4),
    ((18, "woman_farmer_tone5"), WOMAN_FARMER_TONE5),
    ((16, "woman_cook_tone5"), WOMAN_COOK_TONE5),
    ((24, "woman_feeding_baby_tone5"), WOMAN_FEEDING_BABY_TONE5),
    ((19, "woman_student_tone5"), WOMAN_STUDENT_TONE5),
    ((18, "woman_singer_tone5"), WOMAN_SINGER_TONE5),
    ((18, "woman_artist_tone5"), WOMAN_ARTIST_TONE5),
    ((19, "woman_teacher_tone5"), WOMAN_TEACHER_TONE5),
    ((26, "woman_factory_worker_tone5"), WOMAN_FACTORY_WORKER_TONE5),
    ((24, "woman_technologist_tone5"), WOMAN_TECHNOLOGIST_TONE5),
    ((25, "woman_office_worker_tone5"), WOMAN_OFFICE_WORKER_TONE5),
    ((20, "woman_mechanic_tone5"), WOMAN_MECHANIC_TONE5),
    ((21, "woman_scientist_tone5"), WOMAN_SCIENTIST_TONE5),
    ((21, "woman_astronaut_tone5"), WOMAN_ASTRONAUT_TONE5),
    ((23, "woman_firefighter_tone5"), WOMAN_FIREFIGHTER_TONE5),
    ((14, "couple_tone5-1"), COUPLE_TONE5_1),
    ((14, "couple_tone5-2"), COUPLE_TONE5_2),
    ((14, "couple_tone5-3"), COUPLE_TONE5_3),
    ((14, "couple_tone5-4"), COUPLE_TONE5_4),
    ((31, "two_women_holding_hands_tone5-1"), TWO_WOMEN_HOLDING_HANDS_TONE5_1),
    ((31, "two_women_holding_hands_tone5-2"), TWO_WOMEN_HOLDING_HANDS_TONE5_2),
    ((31, "two_women_holding_hands_tone5-3"), TWO_WOMEN_HOLDING_HANDS_TONE5_3),
    ((31, "two_women_holding_hands_tone5-4"), TWO_WOMEN_HOLDING_HANDS_TONE5_4),
    ((29, "woman_with_probing_cane_tone5"), WOMAN_WITH_PROBING_CANE_TONE5),
    ((27, "woman_with_white_cane_tone5"), WOMAN_WITH_WHITE_CANE_TONE5),
    ((22, "woman_red_haired_tone5"), WOMAN_RED_HAIRED_TONE5),
    ((24, "woman_curly_haired_tone5"), WOMAN_CURLY_HAIRED_TONE5),
    ((16, "woman_bald_tone5"), WOMAN_BALD_TONE5),
    ((24, "woman_white_haired_tone5"), WOMAN_WHITE_HAIRED_TONE5),
    ((35, "woman_in_motorized_wheelchair_tone5"), WOMAN_IN_MOTORIZED_WHEELCHAIR_TONE5),
    ((32, "woman_in_manual_wheelchair_tone5"), WOMAN_IN_MANUAL_WHEELCHAIR_TONE5),
    ((25, "woman_health_worker_tone5"), WOMAN_HEALTH_WORKER_TONE5),
    ((17, "woman_judge_tone5"), WOMAN_JUDGE_TONE5),
    ((17, "woman_pilot_tone5"), WOMAN_PILOT_TONE5),
    ((28, "couple_with_heart_mw_tone5-1"), COUPLE_WITH_HEART_MW_TONE5_1),
    ((28, "couple_with_heart_wm_tone5-1"), COUPLE_WITH_HEART_WM_TONE5_1),
    ((28, "couple_with_heart_mw_tone5-2"), COUPLE_WITH_HEART_MW_TONE5_2),
    ((28, "couple_with_heart_wm_tone5-2"), COUPLE_WITH_HEART_WM_TONE5_2),
    ((28, "couple_with_heart_mw_tone5-3"), COUPLE_WITH_HEART_MW_TONE5_3),
    ((28, "couple_with_heart_wm_tone5-3"), COUPLE_WITH_HEART_WM_TONE5_3),
    ((28, "couple_with_heart_mw_tone5-4"), COUPLE_WITH_HEART_MW_TONE5_4),
    ((28, "couple_with_heart_wm_tone5-4"), COUPLE_WITH_HEART_WM_TONE5_4),
    ((26, "couple_with_heart_mw_tone5"), COUPLE_WITH_HEART_MW_TONE5),
    ((26, "couple_with_heart_wm_tone5"), COUPLE_WITH_HEART_WM_TONE5),
    ((28, "couple_with_heart_ww_tone5-1"), COUPLE_WITH_HEART_WW_TONE5_1),
    ((28, "couple_with_heart_ww_tone5-2"), COUPLE_WITH_HEART_WW_TONE5_2),
    ((28, "couple_with_heart_ww_tone5-3"), COUPLE_WITH_HEART_WW_TONE5_3),
    ((28, "couple_with_heart_ww_tone5-4"), COUPLE_WITH_HEART_WW_TONE5_4),
    ((26, "couple_with_heart_ww_tone5"), COUPLE_WITH_HEART_WW_TONE5),
    ((15, "kiss_mw_tone5-1"), KISS_MW_TONE5_1),
    ((15, "kiss_wm_tone5-1"), KISS_WM_TONE5_1),
    ((15, "kiss_mw_tone5-2"), KISS_MW_TONE5_2),
    ((15, "kiss_wm_tone5-2"), KISS_WM_TONE5_2),
    ((15, "kiss_mw_tone5-3"), KISS_MW_TONE5_3),
    ((15, "kiss_wm_tone5-3"), KISS_WM_TONE5_3),
    ((15, "kiss_mw_tone5-4"), KISS_MW_TONE5_4),
    ((15, "kiss_wm_tone5-4"), KISS_WM_TONE5_4),
    ((13, "kiss_mw_tone5"), KISS_MW_TONE5),
    ((13, "kiss_wm_tone5"), KISS_WM_TONE5),
    ((15, "kiss_ww_tone5-1"), KISS_WW_TONE5_1),
    ((15, "kiss_ww_tone5-2"), KISS_WW_TONE5_2),
    ((15, "kiss_ww_tone5-3"), KISS_WW_TONE5_3),
    ((15, "kiss_ww_tone5-4"), KISS_WW_TONE5_4),
    ((13, "kiss_ww_tone5"), KISS_WW_TONE5),
    ((11, "woman_tone5"), WOMAN_TONE5),
    ((12, "woman_farmer"), WOMAN_FARMER),
    ((10, "woman_cook"), WOMAN_COOK),
    ((18, "woman_feeding_baby"), WOMAN_FEEDING_BABY),
    ((13, "woman_student"), WOMAN_STUDENT),
    ((12, "woman_singer"), WOMAN_SINGER),
    ((12, "woman_artist"), WOMAN_ARTIST),
    ((13, "woman_teacher"), WOMAN_TEACHER),
    ((20, "woman_factory_worker"), WOMAN_FACTORY_WORKER),
    ((10, "family_wbb"), FAMILY_WBB),
    ((9, "family_wb"), FAMILY_WB),
    ((10, "family_wgb"), FAMILY_WGB),
    ((10, "family_wgg"), FAMILY_WGG),
    ((9, "family_wg"), FAMILY_WG),
    ((11, "family_wwbb"), FAMILY_WWBB),
    ((10, "family_wwb"), FAMILY_WWB),
    ((11, "family_wwgb"), FAMILY_WWGB),
    ((11, "family_wwgg"), FAMILY_WWGG),
    ((10, "family_wwg"), FAMILY_WWG),
    ((18, "woman_technologist"), WOMAN_TECHNOLOGIST),
    ((19, "woman_office_worker"), WOMAN_OFFICE_WORKER),
    ((14, "woman_mechanic"), WOMAN_MECHANIC),
    ((15, "woman_scientist"), WOMAN_SCIENTIST),
    ((15, "woman_astronaut"), WOMAN_ASTRONAUT),
    ((17, "woman_firefighter"), WOMAN_FIREFIGHTER),
    ((23, "woman_with_probing_cane"), WOMAN_WITH_PROBING_CANE),
    ((21, "woman_with_white_cane"), WOMAN_WITH_WHITE_CANE),
    ((16, "woman_red_haired"), WOMAN_RED_HAIRED),
    ((18, "woman_curly_haired"), WOMAN_CURLY_HAIRED),
    ((10, "woman_bald"), WOMAN_BALD),
    ((18, "woman_white_haired"), WOMAN_WHITE_HAIRED),
    ((29, "woman_in_motorized_wheelchair"), WOMAN_IN_MOTORIZED_WHEELCHAIR),
    ((26, "woman_in_manual_wheelchair"), WOMAN_IN_MANUAL_WHEELCHAIR),
    ((19, "woman_health_worker"), WOMAN_HEALTH_WORKER),
    ((11, "woman_judge"), WOMAN_JUDGE),
    ((11, "woman_pilot"), WOMAN_PILOT),
    ((20, "couple_with_heart_mw"), COUPLE_WITH_HEART_MW),
    ((20, "couple_with_heart_wm"), COUPLE_WITH_HEART_WM),
    ((20, "couple_with_heart_ww"), COUPLE_WITH_HEART_WW),
    ((7, "kiss_mw"), KISS_MW),
    ((7, "kiss_wm"), KISS_WM),
    ((7, "kiss_ww"), KISS_WW),
    ((5, "woman"), WOMAN),
    ((6, "family"), FAMILY),
    ((12, "couple_tone1"), COUPLE_TONE1),
    ((12, "couple_tone2"), COUPLE_TONE2),
    ((12, "couple_tone3"), COUPLE_TONE3),
    ((12, "couple_tone4"), COUPLE_TONE4),
    ((12, "couple_tone5"), COUPLE_TONE5),
    ((6, "couple"), COUPLE),
    ((27, "two_men_holding_hands_tone1"), TWO_MEN_HOLDING_HANDS_TONE1),
    ((27, "two_men_holding_hands_tone2"), TWO_MEN_HOLDING_HANDS_TONE2),
    ((27, "two_men_holding_hands_tone3"), TWO_MEN_HOLDING_HANDS_TONE3),
    ((27, "two_men_holding_hands_tone4"), TWO_MEN_HOLDING_HANDS_TONE4),
    ((27, "two_men_holding_hands_tone5"), TWO_MEN_HOLDING_HANDS_TONE5),
    ((21, "two_men_holding_hands"), TWO_MEN_HOLDING_HANDS),
    ((29, "two_women_holding_hands_tone1"), TWO_WOMEN_HOLDING_HANDS_TONE1),
    ((29, "two_women_holding_hands_tone2"), TWO_WOMEN_HOLDING_HANDS_TONE2),
    ((29, "two_women_holding_hands_tone3"), TWO_WOMEN_HOLDING_HANDS_TONE3),
    ((29, "two_women_holding_hands_tone4"), TWO_WOMEN_HOLDING_HANDS_TONE4),
    ((29, "two_women_holding_hands_tone5"), TWO_WOMEN_HOLDING_HANDS_TONE5),
    ((23, "two_women_holding_hands"), TWO_WOMEN_HOLDING_HANDS),
    ((26, "woman_police_officer_tone1"), WOMAN_POLICE_OFFICER_TONE1),
    ((24, "man_police_officer_tone1"), MAN_POLICE_OFFICER_TONE1),
    ((9, "cop_tone1"), COP_TONE1),
    ((20, "police_officer_tone1"), POLICE_OFFICER_TONE1),
    ((26, "woman_police_officer_tone2"), WOMAN_POLICE_OFFICER_TONE2),
    ((24, "man_police_officer_tone2"), MAN_POLICE_OFFICER_TONE2),
    ((9, "cop_tone2"), COP_TONE2),
    ((20, "police_officer_tone2"), POLICE_OFFICER_TONE2),
    ((26, "woman_police_officer_tone3"), WOMAN_POLICE_OFFICER_TONE3),
    ((24, "man_police_officer_tone3"), MAN_POLICE_OFFICER_TONE3),
    ((9, "cop_tone3"), COP_TONE3),
    ((20, "police_officer_tone3"), POLICE_OFFICER_TONE3),
    ((26, "woman_police_officer_tone4"), WOMAN_POLICE_OFFICER_TONE4),
    ((24, "man_police_officer_tone4"), MAN_POLICE_OFFICER_TONE4),
    ((9, "cop_tone4"), COP_TONE4),
    ((20, "police_officer_tone4"), POLICE_OFFICER_TONE4),
    ((26, "woman_police_officer_tone5"), WOMAN_POLICE_OFFICER_TONE5),
    ((24, "man_police_officer_tone5"), MAN_POLICE_OFFICER_TONE5),
    ((9, "cop_tone5"), COP_TONE5),
    ((20, "police_officer_tone5"), POLICE_OFFICER_TONE5),
    ((20, "woman_police_officer"), WOMAN_POLICE_OFFICER),
    ((18, "man_police_officer"), MAN_POLICE_OFFICER),
    ((3, "cop"), COP),
    ((14, "police_officer"), POLICE_OFFICER),
    ((30, "women_with_bunny_ears_partying"), WOMEN_WITH_BUNNY_EARS_PARTYING),
    ((28, "men_with_bunny_ears_partying"), MEN_WITH_BUNNY_EARS_PARTYING),
    ((7, "dancers"), DANCERS),
    ((31, "people_with_bunny_ears_partying"), PEOPLE_WITH_BUNNY_EARS_PARTYING),
    ((21, "woman_with_veil_tone1"), WOMAN_WITH_VEIL_TONE1),
    ((19, "man_with_veil_tone1"), MAN_WITH_VEIL_TONE1),
    ((22, "person_with_veil_tone1"), PERSON_WITH_VEIL_TONE1),
    ((21, "woman_with_veil_tone2"), WOMAN_WITH_VEIL_TONE2),
    ((19, "man_with_veil_tone2"), MAN_WITH_VEIL_TONE2),
    ((22, "person_with_veil_tone2"), PERSON_WITH_VEIL_TONE2),
    ((21, "woman_with_veil_tone3"), WOMAN_WITH_VEIL_TONE3),
    ((19, "man_with_veil_tone3"), MAN_WITH_VEIL_TONE3),
    ((22, "person_with_veil_tone3"), PERSON_WITH_VEIL_TONE3),
    ((21, "woman_with_veil_tone4"), WOMAN_WITH_VEIL_TONE4),
    ((19, "man_with_veil_tone4"), MAN_WITH_VEIL_TONE4),
    ((22, "person_with_veil_tone4"), PERSON_WITH_VEIL_TONE4),
    ((21, "woman_with_veil_tone5"), WOMAN_WITH_VEIL_TONE5),
    ((19, "man_with_veil_tone5"), MAN_WITH_VEIL_TONE5),
    ((22, "person_with_veil_tone5"), PERSON_WITH_VEIL_TONE5),
    ((15, "woman_with_veil"), WOMAN_WITH_VEIL),
    ((13, "man_with_veil"), MAN_WITH_VEIL),
    ((16, "person_with_veil"), PERSON_WITH_VEIL),
    ((24, "woman_blond_haired_tone1"), WOMAN_BLOND_HAIRED_TONE1),
    ((22, "man_blond_haired_tone1"), MAN_BLOND_HAIRED_TONE1),
    ((18, "blond_haired_tone1"), BLOND_HAIRED_TONE1),
    ((24, "woman_blond_haired_tone2"), WOMAN_BLOND_HAIRED_TONE2),
    ((22, "man_blond_haired_tone2"), MAN_BLOND_HAIRED_TONE2),
    ((18, "blond_haired_tone2"), BLOND_HAIRED_TONE2),
    ((24, "woman_blond_haired_tone3"), WOMAN_BLOND_HAIRED_TONE3),
    ((22, "man_blond_haired_tone3"), MAN_BLOND_HAIRED_TONE3),
    ((18, "blond_haired_tone3"), BLOND_HAIRED_TONE3),
    ((24, "woman_blond_haired_tone4"), WOMAN_BLOND_HAIRED_TONE4),
    ((22, "man_blond_haired_tone4"), MAN_BLOND_HAIRED_TONE4),
    ((18, "blond_haired_tone4"), BLOND_HAIRED_TONE4),
    ((24, "woman_blond_haired_tone5"), WOMAN_BLOND_HAIRED_TONE5),
    ((22, "man_blond_haired_tone5"), MAN_BLOND_HAIRED_TONE5),
    ((18, "blond_haired_tone5"), BLOND_HAIRED_TONE5),
    ((18, "woman_blond_haired"), WOMAN_BLOND_HAIRED),
    ((16, "man_blond_haired"), MAN_BLOND_HAIRED),
    ((12, "blond_haired"), BLOND_HAIRED),
    ((26, "person_with_skullcap_tone1"), PERSON_WITH_SKULLCAP_TONE1),
    ((26, "person_with_skullcap_tone2"), PERSON_WITH_SKULLCAP_TONE2),
    ((26, "person_with_skullcap_tone3"), PERSON_WITH_SKULLCAP_TONE3),
    ((26, "person_with_skullcap_tone4"), PERSON_WITH_SKULLCAP_TONE4),
    ((26, "person_with_skullcap_tone5"), PERSON_WITH_SKULLCAP_TONE5),
    ((20, "person_with_skullcap"), PERSON_WITH_SKULLCAP),
    ((26, "woman_wearing_turban_tone1"), WOMAN_WEARING_TURBAN_TONE1),
    ((24, "man_wearing_turban_tone1"), MAN_WEARING_TURBAN_TONE1),
    ((27, "person_wearing_turban_tone1"), PERSON_WEARING_TURBAN_TONE1),
    ((26, "woman_wearing_turban_tone2"), WOMAN_WEARING_TURBAN_TONE2),
    ((24, "man_wearing_turban_tone2"), MAN_WEARING_TURBAN_TONE2),
    ((27, "person_wearing_turban_tone2"), PERSON_WEARING_TURBAN_TONE2),
    ((26, "woman_wearing_turban_tone3"), WOMAN_WEARING_TURBAN_TONE3),
    ((24, "man_wearing_turban_tone3"), MAN_WEARING_TURBAN_TONE3),
    ((27, "person_wearing_turban_tone3"), PERSON_WEARING_TURBAN_TONE3),
    ((26, "woman_wearing_turban_tone4"), WOMAN_WEARING_TURBAN_TONE4),
    ((24, "man_wearing_turban_tone4"), MAN_WEARING_TURBAN_TONE4),
    ((27, "person_wearing_turban_tone4"), PERSON_WEARING_TURBAN_TONE4),
    ((26, "woman_wearing_turban_tone5"), WOMAN_WEARING_TURBAN_TONE5),
    ((24, "man_wearing_turban_tone5"), MAN_WEARING_TURBAN_TONE5),
    ((27, "person_wearing_turban_tone5"), PERSON_WEARING_TURBAN_TONE5),
    ((20, "woman_wearing_turban"), WOMAN_WEARING_TURBAN),
    ((18, "man_wearing_turban"), MAN_WEARING_TURBAN),
    ((21, "person_wearing_turban"), PERSON_WEARING_TURBAN),
    ((15, "older_man_tone1"), OLDER_MAN_TONE1),
    ((15, "older_man_tone2"), OLDER_MAN_TONE2),
    ((15, "older_man_tone3"), OLDER_MAN_TONE3),
    ((15, "older_man_tone4"), OLDER_MAN_TONE4),
    ((15, "older_man_tone5"), OLDER_MAN_TONE5),
    ((9, "older_man"), OLDER_MAN),
    ((17, "older_woman_tone1"), OLDER_WOMAN_TONE1),
    ((17, "older_woman_tone2"), OLDER_WOMAN_TONE2),
    ((17, "older_woman_tone3"), OLDER_WOMAN_TONE3),
    ((17, "older_woman_tone4"), OLDER_WOMAN_TONE4),
    ((17, "older_woman_tone5"), OLDER_WOMAN_TONE5),
    ((11, "older_woman"), OLDER_WOMAN),
    ((10, "baby_tone1"), BABY_TONE1),
    ((10, "baby_tone2"), BABY_TONE2),
    ((10, "baby_tone3"), BABY_TONE3),
    ((10, "baby_tone4"), BABY_TONE4),
    ((10, "baby_tone5"), BABY_TONE5),
    ((4, "baby"), BABY),
    ((31, "woman_construction_worker_tone1"), WOMAN_CONSTRUCTION_WORKER_TONE1),
    ((29, "man_construction_worker_tone1"), MAN_CONSTRUCTION_WORKER_TONE1),
    ((25, "construction_worker_tone1"), CONSTRUCTION_WORKER_TONE1),
    ((31, "woman_construction_worker_tone2"), WOMAN_CONSTRUCTION_WORKER_TONE2),
    ((29, "man_construction_worker_tone2"), MAN_CONSTRUCTION_WORKER_TONE2),
    ((25, "construction_worker_tone2"), CONSTRUCTION_WORKER_TONE2),
    ((31, "woman_construction_worker_tone3"), WOMAN_CONSTRUCTION_WORKER_TONE3),
    ((29, "man_construction_worker_tone3"), MAN_CONSTRUCTION_WORKER_TONE3),
    ((25, "construction_worker_tone3"), CONSTRUCTION_WORKER_TONE3),
    ((31, "woman_construction_worker_tone4"), WOMAN_CONSTRUCTION_WORKER_TONE4),
    ((29, "man_construction_worker_tone4"), MAN_CONSTRUCTION_WORKER_TONE4),
    ((25, "construction_worker_tone4"), CONSTRUCTION_WORKER_TONE4),
    ((31, "woman_construction_worker_tone5"), WOMAN_CONSTRUCTION_WORKER_TONE5),
    ((29, "man_construction_worker_tone5"), MAN_CONSTRUCTION_WORKER_TONE5),
    ((25, "construction_worker_tone5"), CONSTRUCTION_WORKER_TONE5),
    ((25, "woman_construction_worker"), WOMAN_CONSTRUCTION_WORKER),
    ((23, "man_construction_worker"), MAN_CONSTRUCTION_WORKER),
    ((19, "construction_worker"), CONSTRUCTION_WORKER),
    ((14, "princess_tone1"), PRINCESS_TONE1),
    ((14, "princess_tone2"), PRINCESS_TONE2),
    ((14, "princess_tone3"), PRINCESS_TONE3),
    ((14, "princess_tone4"), PRINCESS_TONE4),
    ((14, "princess_tone5"), PRINCESS_TONE5),
    ((8, "princess"), PRINCESS),
    ((13, "japanese_ogre"), JAPANESE_OGRE),
    ((4, "ogre"), OGRE),
    ((6, "goblin"), GOBLIN),
    ((15, "japanese_goblin"), JAPANESE_GOBLIN),
    ((5, "ghost"), GHOST),
    ((11, "angel_tone1"), ANGEL_TONE1),
    ((11, "angel_tone2"), ANGEL_TONE2),
    ((11, "angel_tone3"), ANGEL_TONE3),
    ((11, "angel_tone4"), ANGEL_TONE4),
    ((11, "angel_tone5"), ANGEL_TONE5),
    ((5, "angel"), ANGEL),
    ((5, "alien"), ALIEN),
    ((13, "alien_monster"), ALIEN_MONSTER),
    ((13, "space_invader"), SPACE_INVADER),
    ((9, "angry_imp"), ANGRY_IMP),
    ((3, "imp"), IMP),
    ((5, "skull"), SKULL),
    ((24, "woman_tipping_hand_tone1"), WOMAN_TIPPING_HAND_TONE1),
    ((22, "man_tipping_hand_tone1"), MAN_TIPPING_HAND_TONE1),
    ((25, "person_tipping_hand_tone1"), PERSON_TIPPING_HAND_TONE1),
    ((24, "woman_tipping_hand_tone2"), WOMAN_TIPPING_HAND_TONE2),
    ((22, "man_tipping_hand_tone2"), MAN_TIPPING_HAND_TONE2),
    ((25, "person_tipping_hand_tone2"), PERSON_TIPPING_HAND_TONE2),
    ((24, "woman_tipping_hand_tone3"), WOMAN_TIPPING_HAND_TONE3),
    ((22, "man_tipping_hand_tone3"), MAN_TIPPING_HAND_TONE3),
    ((25, "person_tipping_hand_tone3"), PERSON_TIPPING_HAND_TONE3),
    ((24, "woman_tipping_hand_tone4"), WOMAN_TIPPING_HAND_TONE4),
    ((22, "man_tipping_hand_tone4"), MAN_TIPPING_HAND_TONE4),
    ((25, "person_tipping_hand_tone4"), PERSON_TIPPING_HAND_TONE4),
    ((24, "woman_tipping_hand_tone5"), WOMAN_TIPPING_HAND_TONE5),
    ((22, "man_tipping_hand_tone5"), MAN_TIPPING_HAND_TONE5),
    ((25, "person_tipping_hand_tone5"), PERSON_TIPPING_HAND_TONE5),
    ((18, "woman_tipping_hand"), WOMAN_TIPPING_HAND),
    ((16, "man_tipping_hand"), MAN_TIPPING_HAND),
    ((19, "person_tipping_hand"), PERSON_TIPPING_HAND),
    ((17, "woman_guard_tone1"), WOMAN_GUARD_TONE1),
    ((15, "man_guard_tone1"), MAN_GUARD_TONE1),
    ((11, "guard_tone1"), GUARD_TONE1),
    ((17, "woman_guard_tone2"), WOMAN_GUARD_TONE2),
    ((15, "man_guard_tone2"), MAN_GUARD_TONE2),
    ((11, "guard_tone2"), GUARD_TONE2),
    ((17, "woman_guard_tone3"), WOMAN_GUARD_TONE3),
    ((15, "man_guard_tone3"), MAN_GUARD_TONE3),
    ((11, "guard_tone3"), GUARD_TONE3),
    ((17, "woman_guard_tone4"), WOMAN_GUARD_TONE4),
    ((15, "man_guard_tone4"), MAN_GUARD_TONE4),
    ((11, "guard_tone4"), GUARD_TONE4),
    ((17, "woman_guard_tone5"), WOMAN_GUARD_TONE5),
    ((15, "man_guard_tone5"), MAN_GUARD_TONE5),
    ((11, "guard_tone5"), GUARD_TONE5),
    ((11, "woman_guard"), WOMAN_GUARD),
    ((9, "man_guard"), MAN_GUARD),
    ((5, "guard"), GUARD),
    ((12, "dancer_tone1"), DANCER_TONE1),
    ((19, "woman_dancing_tone1"), WOMAN_DANCING_TONE1),
    ((12, "dancer_tone2"), DANCER_TONE2),
    ((19, "woman_dancing_tone2"), WOMAN_DANCING_TONE2),
    ((12, "dancer_tone3"), DANCER_TONE3),
    ((19, "woman_dancing_tone3"), WOMAN_DANCING_TONE3),
    ((12, "dancer_tone4"), DANCER_TONE4),
    ((19, "woman_dancing_tone4"), WOMAN_DANCING_TONE4),
    ((12, "dancer_tone5"), DANCER_TONE5),
    ((19, "woman_dancing_tone5"), WOMAN_DANCING_TONE5),
    ((6, "dancer"), DANCER),
    ((13, "woman_dancing"), WOMAN_DANCING),
    ((8, "lipstick"), LIPSTICK),
    ((15, "nail_care_tone1"), NAIL_CARE_TONE1),
    ((17, "nail_polish_tone1"), NAIL_POLISH_TONE1),
    ((15, "nail_care_tone2"), NAIL_CARE_TONE2),
    ((17, "nail_polish_tone2"), NAIL_POLISH_TONE2),
    ((15, "nail_care_tone3"), NAIL_CARE_TONE3),
    ((17, "nail_polish_tone3"), NAIL_POLISH_TONE3),
    ((15, "nail_care_tone4"), NAIL_CARE_TONE4),
    ((17, "nail_polish_tone4"), NAIL_POLISH_TONE4),
    ((15, "nail_care_tone5"), NAIL_CARE_TONE5),
    ((17, "nail_polish_tone5"), NAIL_POLISH_TONE5),
    ((9, "nail_care"), NAIL_CARE),
    ((11, "nail_polish"), NAIL_POLISH),
    ((27, "woman_getting_massage_tone1"), WOMAN_GETTING_MASSAGE_TONE1),
    ((25, "man_getting_massage_tone1"), MAN_GETTING_MASSAGE_TONE1),
    ((13, "massage_tone1"), MASSAGE_TONE1),
    ((28, "person_getting_massage_tone1"), PERSON_GETTING_MASSAGE_TONE1),
    ((27, "woman_getting_massage_tone2"), WOMAN_GETTING_MASSAGE_TONE2),
    ((25, "man_getting_massage_tone2"), MAN_GETTING_MASSAGE_TONE2),
    ((13, "massage_tone2"), MASSAGE_TONE2),
    ((28, "person_getting_massage_tone2"), PERSON_GETTING_MASSAGE_TONE2),
    ((27, "woman_getting_massage_tone3"), WOMAN_GETTING_MASSAGE_TONE3),
    ((25, "man_getting_massage_tone3"), MAN_GETTING_MASSAGE_TONE3),
    ((13, "massage_tone3"), MASSAGE_TONE3),
    ((28, "person_getting_massage_tone3"), PERSON_GETTING_MASSAGE_TONE3),
    ((27, "woman_getting_massage_tone4"), WOMAN_GETTING_MASSAGE_TONE4),
    ((25, "man_getting_massage_tone4"), MAN_GETTING_MASSAGE_TONE4),
    ((13, "massage_tone4"), MASSAGE_TONE4),
    ((28, "person_getting_massage_tone4"), PERSON_GETTING_MASSAGE_TONE4),
    ((27, "woman_getting_massage_tone5"), WOMAN_GETTING_MASSAGE_TONE5),
    ((25, "man_getting_massage_tone5"), MAN_GETTING_MASSAGE_TONE5),
    ((13, "massage_tone5"), MASSAGE_TONE5),
    ((28, "person_getting_massage_tone5"), PERSON_GETTING_MASSAGE_TONE5),
    ((21, "woman_getting_massage"), WOMAN_GETTING_MASSAGE),
    ((19, "man_getting_massage"), MAN_GETTING_MASSAGE),
    ((7, "massage"), MASSAGE),
    ((22, "person_getting_massage"), PERSON_GETTING_MASSAGE),
    ((27, "woman_getting_haircut_tone1"), WOMAN_GETTING_HAIRCUT_TONE1),
    ((25, "man_getting_haircut_tone1"), MAN_GETTING_HAIRCUT_TONE1),
    ((13, "haircut_tone1"), HAIRCUT_TONE1),
    ((28, "person_getting_haircut_tone1"), PERSON_GETTING_HAIRCUT_TONE1),
    ((27, "woman_getting_haircut_tone2"), WOMAN_GETTING_HAIRCUT_TONE2),
    ((25, "man_getting_haircut_tone2"), MAN_GETTING_HAIRCUT_TONE2),
    ((13, "haircut_tone2"), HAIRCUT_TONE2),
    ((28, "person_getting_haircut_tone2"), PERSON_GETTING_HAIRCUT_TONE2),
    ((27, "woman_getting_haircut_tone3"), WOMAN_GETTING_HAIRCUT_TONE3),
    ((25, "man_getting_haircut_tone3"), MAN_GETTING_HAIRCUT_TONE3),
    ((13, "haircut_tone3"), HAIRCUT_TONE3),
    ((28, "person_getting_haircut_tone3"), PERSON_GETTING_HAIRCUT_TONE3),
    ((27, "woman_getting_haircut_tone4"), WOMAN_GETTING_HAIRCUT_TONE4),
    ((25, "man_getting_haircut_tone4"), MAN_GETTING_HAIRCUT_TONE4),
    ((13, "haircut_tone4"), HAIRCUT_TONE4),
    ((28, "person_getting_haircut_tone4"), PERSON_GETTING_HAIRCUT_TONE4),
    ((27, "woman_getting_haircut_tone5"), WOMAN_GETTING_HAIRCUT_TONE5),
    ((25, "man_getting_haircut_tone5"), MAN_GETTING_HAIRCUT_TONE5),
    ((13, "haircut_tone5"), HAIRCUT_TONE5),
    ((28, "person_getting_haircut_tone5"), PERSON_GETTING_HAIRCUT_TONE5),
    ((21, "woman_getting_haircut"), WOMAN_GETTING_HAIRCUT),
    ((19, "man_getting_haircut"), MAN_GETTING_HAIRCUT),
    ((7, "haircut"), HAIRCUT),
    ((22, "person_getting_haircut"), PERSON_GETTING_HAIRCUT),
    ((6, "barber"), BARBER),
    ((11, "barber_pole"), BARBER_POLE),
    ((7, "syringe"), SYRINGE),
    ((4, "pill"), PILL),
    ((4, "kiss"), KISS),
    ((11, "love_letter"), LOVE_LETTER),
    ((4, "ring"), RING),
    ((3, "gem"), GEM),
    ((17, "couple_kiss_tone1"), COUPLE_KISS_TONE1),
    ((16, "couplekiss_tone1"), COUPLEKISS_TONE1),
    ((17, "couple_kiss_tone2"), COUPLE_KISS_TONE2),
    ((16, "couplekiss_tone2"), COUPLEKISS_TONE2),
    ((17, "couple_kiss_tone3"), COUPLE_KISS_TONE3),
    ((16, "couplekiss_tone3"), COUPLEKISS_TONE3),
    ((17, "couple_kiss_tone4"), COUPLE_KISS_TONE4),
    ((16, "couplekiss_tone4"), COUPLEKISS_TONE4),
    ((17, "couple_kiss_tone5"), COUPLE_KISS_TONE5),
    ((16, "couplekiss_tone5"), COUPLEKISS_TONE5),
    ((11, "couple_kiss"), COUPLE_KISS),
    ((10, "couplekiss"), COUPLEKISS),
    ((7, "bouquet"), BOUQUET),
    ((23, "couple_with_heart_tone1"), COUPLE_WITH_HEART_TONE1),
    ((23, "couple_with_heart_tone2"), COUPLE_WITH_HEART_TONE2),
    ((23, "couple_with_heart_tone3"), COUPLE_WITH_HEART_TONE3),
    ((23, "couple_with_heart_tone4"), COUPLE_WITH_HEART_TONE4),
    ((23, "couple_with_heart_tone5"), COUPLE_WITH_HEART_TONE5),
    ((17, "couple_with_heart"), COUPLE_WITH_HEART),
    ((7, "wedding"), WEDDING),
    ((13, "beating_heart"), BEATING_HEART),
    ((9, "heartbeat"), HEARTBEAT),
    ((12, "broken_heart"), BROKEN_HEART),
    ((10, "two_hearts"), TWO_HEARTS),
    ((15, "sparkling_heart"), SPARKLING_HEART),
    ((13, "growing_heart"), GROWING_HEART),
    ((10, "heartpulse"), HEARTPULSE),
    ((5, "cupid"), CUPID),
    ((16, "heart_with_arrow"), HEART_WITH_ARROW),
    ((10, "blue_heart"), BLUE_HEART),
    ((11, "green_heart"), GREEN_HEART),
    ((12, "yellow_heart"), YELLOW_HEART),
    ((12, "purple_heart"), PURPLE_HEART),
    ((10, "gift_heart"), GIFT_HEART),
    ((17, "heart_with_ribbon"), HEART_WITH_RIBBON),
    ((16, "revolving_hearts"), REVOLVING_HEARTS),
    ((16, "heart_decoration"), HEART_DECORATION),
    ((31, "diamond_shape_with_a_dot_inside"), DIAMOND_SHAPE_WITH_A_DOT_INSIDE),
    ((18, "diamond_with_a_dot"), DIAMOND_WITH_A_DOT),
    ((4, "bulb"), BULB),
    ((10, "light_bulb"), LIGHT_BULB),
    ((5, "anger"), ANGER),
    ((4, "bomb"), BOMB),
    ((3, "zzz"), ZZZ),
    ((4, "boom"), BOOM),
    ((9, "collision"), COLLISION),
    ((11, "sweat_drops"), SWEAT_DROPS),
    ((7, "droplet"), DROPLET),
    ((4, "dash"), DASH),
    ((12, "dashing_away"), DASHING_AWAY),
    ((4, "poop"), POOP),
    ((4, "shit"), SHIT),
    ((12, "muscle_tone1"), MUSCLE_TONE1),
    ((17, "right_bicep_tone1"), RIGHT_BICEP_TONE1),
    ((12, "muscle_tone2"), MUSCLE_TONE2),
    ((17, "right_bicep_tone2"), RIGHT_BICEP_TONE2),
    ((12, "muscle_tone3"), MUSCLE_TONE3),
    ((17, "right_bicep_tone3"), RIGHT_BICEP_TONE3),
    ((12, "muscle_tone4"), MUSCLE_TONE4),
    ((17, "right_bicep_tone4"), RIGHT_BICEP_TONE4),
    ((12, "muscle_tone5"), MUSCLE_TONE5),
    ((17, "right_bicep_tone5"), RIGHT_BICEP_TONE5),
    ((6, "muscle"), MUSCLE),
    ((11, "right_bicep"), RIGHT_BICEP),
    ((5, "dizzy"), DIZZY),
    ((14, "speech_balloon"), SPEECH_BALLOON),
    ((15, "thought_balloon"), THOUGHT_BALLOON),
    ((12, "white_flower"), WHITE_FLOWER),
    ((3, "100"), X_100),
    ((8, "moneybag"), MONEYBAG),
    ((17, "currency_exchange"), CURRENCY_EXCHANGE),
    ((17, "heavy_dollar_sign"), HEAVY_DOLLAR_SIGN),
    ((11, "credit_card"), CREDIT_CARD),
    ((3, "yen"), YEN),
    ((6, "dollar"), DOLLAR),
    ((4, "euro"), EURO),
    ((5, "pound"), POUND),
    ((16, "money_with_wings"), MONEY_WITH_WINGS),
    ((5, "chart"), CHART),
    ((4, "seat"), SEAT),
    ((6, "laptop"), LAPTOP),
    ((9, "briefcase"), BRIEFCASE),
    ((13, "computer_disk"), COMPUTER_DISK),
    ((8, "minidisc"), MINIDISC),
    ((11, "floppy_disk"), FLOPPY_DISK),
    ((2, "cd"), CD),
    ((12, "optical_disk"), OPTICAL_DISK),
    ((3, "dvd"), DVD),
    ((11, "file_folder"), FILE_FOLDER),
    ((16, "open_file_folder"), OPEN_FILE_FOLDER),
    ((14, "page_with_curl"), PAGE_WITH_CURL),
    ((14, "page_facing_up"), PAGE_FACING_UP),
    ((4, "date"), DATE),
    ((8, "calendar"), CALENDAR),
    ((10, "card_index"), CARD_INDEX),
    ((16, "chart_increasing"), CHART_INCREASING),
    ((24, "chart_with_upwards_trend"), CHART_WITH_UPWARDS_TREND),
    ((16, "chart_decreasing"), CHART_DECREASING),
    ((26, "chart_with_downwards_trend"), CHART_WITH_DOWNWARDS_TREND),
    ((9, "bar_chart"), BAR_CHART),
    ((9, "clipboard"), CLIPBOARD),
    ((7, "pushpin"), PUSHPIN),
    ((13, "round_pushpin"), ROUND_PUSHPIN),
    ((9, "paperclip"), PAPERCLIP),
    ((14, "straight_ruler"), STRAIGHT_RULER),
    ((16, "triangular_ruler"), TRIANGULAR_RULER),
    ((13, "bookmark_tabs"), BOOKMARK_TABS),
    ((6, "ledger"), LEDGER),
    ((8, "notebook"), NOTEBOOK),
    ((30, "notebook_with_decorative_cover"), NOTEBOOK_WITH_DECORATIVE_COVER),
    ((11, "closed_book"), CLOSED_BOOK),
    ((4, "book"), BOOK),
    ((9, "open_book"), OPEN_BOOK),
    ((10, "green_book"), GREEN_BOOK),
    ((9, "blue_book"), BLUE_BOOK),
    ((11, "orange_book"), ORANGE_BOOK),
    ((5, "books"), BOOKS),
    ((10, "name_badge"), NAME_BADGE),
    ((6, "scroll"), SCROLL),
    ((4, "memo"), MEMO),
    ((18, "telephone_receiver"), TELEPHONE_RECEIVER),
    ((5, "pager"), PAGER),
    ((3, "fax"), FAX),
    ((11, "fax_machine"), FAX_MACHINE),
    ((17, "satellite_antenna"), SATELLITE_ANTENNA),
    ((11, "loudspeaker"), LOUDSPEAKER),
    ((4, "mega"), MEGA),
    ((9, "megaphone"), MEGAPHONE),
    ((11, "outbox_tray"), OUTBOX_TRAY),
    ((10, "inbox_tray"), INBOX_TRAY),
    ((7, "package"), PACKAGE),
    ((6, "e-mail"), E_MAIL),
    ((5, "email"), EMAIL),
    ((17, "incoming_envelope"), INCOMING_ENVELOPE),
    ((19, "envelope_with_arrow"), ENVELOPE_WITH_ARROW),
    ((14, "mailbox_closed"), MAILBOX_CLOSED),
    ((7, "mailbox"), MAILBOX),
    ((17, "mailbox_with_mail"), MAILBOX_WITH_MAIL),
    ((20, "mailbox_with_no_mail"), MAILBOX_WITH_NO_MAIL),
    ((7, "postbox"), POSTBOX),
    ((11, "postal_horn"), POSTAL_HORN),
    ((9, "newspaper"), NEWSPAPER),
    ((7, "android"), ANDROID),
    ((6, "iphone"), IPHONE),
    ((12, "mobile_phone"), MOBILE_PHONE),
    ((7, "calling"), CALLING),
    ((18, "mobile_phone_arrow"), MOBILE_PHONE_ARROW),
    ((14, "vibration_mode"), VIBRATION_MODE),
    ((16, "mobile_phone_off"), MOBILE_PHONE_OFF),
    ((16, "no_mobile_phones"), NO_MOBILE_PHONES),
    ((12, "antenna_bars"), ANTENNA_BARS),
    ((15, "signal_strength"), SIGNAL_STRENGTH),
    ((6, "camera"), CAMERA),
    ((17, "camera_with_flash"), CAMERA_WITH_FLASH),
    ((12, "video_camera"), VIDEO_CAMERA),
    ((2, "tv"), TV),
    ((5, "radio"), RADIO),
    ((3, "vhs"), VHS),
    ((13, "videocassette"), VIDEOCASSETTE),
    ((14, "film_projector"), FILM_PROJECTOR),
    ((12, "prayer_beads"), PRAYER_BEADS),
    ((7, "shuffle"), SHUFFLE),
    ((25, "twisted_rightwards_arrows"), TWISTED_RIGHTWARDS_ARROWS),
    ((6, "repeat"), REPEAT),
    ((10, "repeat_one"), REPEAT_ONE),
    ((16, "arrows_clockwise"), ARROWS_CLOCKWISE),
    ((9, "clockwise"), CLOCKWISE),
    ((23, "arrows_counterclockwise"), ARROWS_COUNTERCLOCKWISE),
    ((16, "counterclockwise"), COUNTERCLOCKWISE),
    ((10, "dim_button"), DIM_BUTTON),
    ((14, "low_brightness"), LOW_BRIGHTNESS),
    ((13, "bright_button"), BRIGHT_BUTTON),
    ((15, "high_brightness"), HIGH_BRIGHTNESS),
    ((4, "mute"), MUTE),
    ((8, "no_sound"), NO_SOUND),
    ((10, "low_volume"), LOW_VOLUME),
    ((11, "quiet_sound"), QUIET_SOUND),
    ((7, "speaker"), SPEAKER),
    ((14, "medium_volumne"), MEDIUM_VOLUMNE),
    ((5, "sound"), SOUND),
    ((11, "high_volume"), HIGH_VOLUME),
    ((10, "loud_sound"), LOUD_SOUND),
    ((7, "battery"), BATTERY),
    ((13, "electric_plug"), ELECTRIC_PLUG),
    ((3, "mag"), MAG),
    ((9, "mag_right"), MAG_RIGHT),
    ((17, "lock_with_ink_pen"), LOCK_WITH_INK_PEN),
    ((15, "locked_with_pen"), LOCKED_WITH_PEN),
    ((20, "closed_lock_with_key"), CLOSED_LOCK_WITH_KEY),
    ((15, "locked_with_key"), LOCKED_WITH_KEY),
    ((3, "key"), KEY),
    ((4, "lock"), LOCK),
    ((6, "locked"), LOCKED),
    ((6, "unlock"), UNLOCK),
    ((8, "unlocked"), UNLOCKED),
    ((4, "bell"), BELL),
    ((7, "no_bell"), NO_BELL),
    ((8, "bookmark"), BOOKMARK),
    ((4, "link"), LINK),
    ((12, "radio_button"), RADIO_BUTTON),
    ((4, "back"), BACK),
    ((3, "end"), END),
    ((2, "on"), ON),
    ((4, "soon"), SOON),
    ((3, "top"), TOP),
    ((15, "no_one_under_18"), NO_ONE_UNDER_18),
    ((8, "underage"), UNDERAGE),
    ((3, "ten"), TEN),
    ((12, "capital_abcd"), CAPITAL_ABCD),
    ((4, "abcd"), ABCD),
    ((4, "1234"), X_1234),
    ((7, "symbols"), SYMBOLS),
    ((3, "abc"), ABC),
    ((4, "fire"), FIRE),
    ((10, "flashlight"), FLASHLIGHT),
    ((6, "wrench"), WRENCH),
    ((6, "hammer"), HAMMER),
    ((12, "nut_and_bolt"), NUT_AND_BOLT),
    ((5, "knife"), KNIFE),
    ((3, "gun"), GUN),
    ((6, "pistol"), PISTOL),
    ((10, "microscope"), MICROSCOPE),
    ((9, "telescope"), TELESCOPE),
    ((12, "crystal_ball"), CRYSTAL_BALL),
    ((16, "six_pointed_star"), SIX_POINTED_STAR),
    ((8, "beginner"), BEGINNER),
    ((7, "trident"), TRIDENT),
    ((19, "black_square_button"), BLACK_SQUARE_BUTTON),
    ((19, "white_square_button"), WHITE_SQUARE_BUTTON),
    ((10, "red_circle"), RED_CIRCLE),
    ((11, "blue_circle"), BLUE_CIRCLE),
    ((20, "large_orange_diamond"), LARGE_ORANGE_DIAMOND),
    ((18, "large_blue_diamond"), LARGE_BLUE_DIAMOND),
    ((20, "small_orange_diamond"), SMALL_ORANGE_DIAMOND),
    ((18, "small_blue_diamond"), SMALL_BLUE_DIAMOND),
    ((18, "small_red_triangle"), SMALL_RED_TRIANGLE),
    ((23, "small_red_triangle_down"), SMALL_RED_TRIANGLE_DOWN),
    ((14, "arrow_up_small"), ARROW_UP_SMALL),
    ((2, "up"), UP),
    ((16, "arrow_down_small"), ARROW_DOWN_SMALL),
    ((4, "down"), DOWN),
    ((2, "om"), OM),
    ((4, "dove"), DOVE),
    ((5, "kaaba"), KAABA),
    ((6, "mosque"), MOSQUE),
    ((9, "synagogue"), SYNAGOGUE),
    ((7, "menorah"), MENORAH),
    ((6, "clock1"), CLOCK1),
    ((6, "clock2"), CLOCK2),
    ((6, "clock3"), CLOCK3),
    ((6, "clock4"), CLOCK4),
    ((6, "clock5"), CLOCK5),
    ((6, "clock6"), CLOCK6),
    ((6, "clock7"), CLOCK7),
    ((6, "clock8"), CLOCK8),
    ((6, "clock9"), CLOCK9),
    ((7, "clock10"), CLOCK10),
    ((7, "clock11"), CLOCK11),
    ((7, "clock12"), CLOCK12),
    ((8, "clock130"), CLOCK130),
    ((8, "clock230"), CLOCK230),
    ((8, "clock330"), CLOCK330),
    ((8, "clock430"), CLOCK430),
    ((8, "clock530"), CLOCK530),
    ((8, "clock630"), CLOCK630),
    ((8, "clock730"), CLOCK730),
    ((8, "clock830"), CLOCK830),
    ((8, "clock930"), CLOCK930),
    ((9, "clock1030"), CLOCK1030),
    ((9, "clock1130"), CLOCK1130),
    ((9, "clock1230"), CLOCK1230),
    ((6, "candle"), CANDLE),
    ((5, "clock"), CLOCK),
    ((4, "hole"), HOLE),
    ((14, "levitate_tone1"), LEVITATE_TONE1),
    ((16, "levitating_tone1"), LEVITATING_TONE1),
    ((31, "person_in_suit_levitating_tone1"), PERSON_IN_SUIT_LEVITATING_TONE1),
    ((14, "levitate_tone2"), LEVITATE_TONE2),
    ((16, "levitating_tone2"), LEVITATING_TONE2),
    ((31, "person_in_suit_levitating_tone2"), PERSON_IN_SUIT_LEVITATING_TONE2),
    ((14, "levitate_tone3"), LEVITATE_TONE3),
    ((16, "levitating_tone3"), LEVITATING_TONE3),
    ((31, "person_in_suit_levitating_tone3"), PERSON_IN_SUIT_LEVITATING_TONE3),
    ((14, "levitate_tone4"), LEVITATE_TONE4),
    ((16, "levitating_tone4"), LEVITATING_TONE4),
    ((31, "person_in_suit_levitating_tone4"), PERSON_IN_SUIT_LEVITATING_TONE4),
    ((14, "levitate_tone5"), LEVITATE_TONE5),
    ((16, "levitating_tone5"), LEVITATING_TONE5),
    ((31, "person_in_suit_levitating_tone5"), PERSON_IN_SUIT_LEVITATING_TONE5),
    ((8, "levitate"), LEVITATE),
    ((10, "levitating"), LEVITATING),
    ((25, "person_in_suit_levitating"), PERSON_IN_SUIT_LEVITATING),
    ((21, "woman_detective_tone1"), WOMAN_DETECTIVE_TONE1),
    ((19, "man_detective_tone1"), MAN_DETECTIVE_TONE1),
    ((15, "detective_tone1"), DETECTIVE_TONE1),
    ((21, "woman_detective_tone2"), WOMAN_DETECTIVE_TONE2),
    ((19, "man_detective_tone2"), MAN_DETECTIVE_TONE2),
    ((15, "detective_tone2"), DETECTIVE_TONE2),
    ((21, "woman_detective_tone3"), WOMAN_DETECTIVE_TONE3),
    ((19, "man_detective_tone3"), MAN_DETECTIVE_TONE3),
    ((15, "detective_tone3"), DETECTIVE_TONE3),
    ((21, "woman_detective_tone4"), WOMAN_DETECTIVE_TONE4),
    ((19, "man_detective_tone4"), MAN_DETECTIVE_TONE4),
    ((15, "detective_tone4"), DETECTIVE_TONE4),
    ((21, "woman_detective_tone5"), WOMAN_DETECTIVE_TONE5),
    ((19, "man_detective_tone5"), MAN_DETECTIVE_TONE5),
    ((15, "detective_tone5"), DETECTIVE_TONE5),
    ((15, "woman_detective"), WOMAN_DETECTIVE),
    ((13, "man_detective"), MAN_DETECTIVE),
    ((9, "detective"), DETECTIVE),
    ((10, "sunglasses"), SUNGLASSES),
    ((6, "spider"), SPIDER),
    ((10, "spider_web"), SPIDER_WEB),
    ((8, "joystick"), JOYSTICK),
    ((17, "man_dancing_tone1"), MAN_DANCING_TONE1),
    ((17, "man_dancing_tone2"), MAN_DANCING_TONE2),
    ((17, "man_dancing_tone3"), MAN_DANCING_TONE3),
    ((17, "man_dancing_tone4"), MAN_DANCING_TONE4),
    ((17, "man_dancing_tone5"), MAN_DANCING_TONE5),
    ((11, "man_dancing"), MAN_DANCING),
    ((10, "paperclips"), PAPERCLIPS),
    ((3, "pen"), PEN),
    ((12, "fountain_pen"), FOUNTAIN_PEN),
    ((10, "paintbrush"), PAINTBRUSH),
    ((6, "crayon"), CRAYON),
    ((38, "raised_hand_with_fingers_splayed_tone1"), RAISED_HAND_WITH_FINGERS_SPLAYED_TONE1),
    ((38, "raised_hand_with_fingers_splayed_tone2"), RAISED_HAND_WITH_FINGERS_SPLAYED_TONE2),
    ((38, "raised_hand_with_fingers_splayed_tone3"), RAISED_HAND_WITH_FINGERS_SPLAYED_TONE3),
    ((38, "raised_hand_with_fingers_splayed_tone4"), RAISED_HAND_WITH_FINGERS_SPLAYED_TONE4),
    ((38, "raised_hand_with_fingers_splayed_tone5"), RAISED_HAND_WITH_FINGERS_SPLAYED_TONE5),
    ((32, "raised_hand_with_fingers_splayed"), RAISED_HAND_WITH_FINGERS_SPLAYED),
    ((19, "middle_finger_tone1"), MIDDLE_FINGER_TONE1),
    ((19, "middle_finger_tone2"), MIDDLE_FINGER_TONE2),
    ((19, "middle_finger_tone3"), MIDDLE_FINGER_TONE3),
    ((19, "middle_finger_tone4"), MIDDLE_FINGER_TONE4),
    ((19, "middle_finger_tone5"), MIDDLE_FINGER_TONE5),
    ((13, "middle_finger"), MIDDLE_FINGER),
    ((12, "vulcan_tone1"), VULCAN_TONE1),
    ((12, "vulcan_tone2"), VULCAN_TONE2),
    ((12, "vulcan_tone3"), VULCAN_TONE3),
    ((12, "vulcan_tone4"), VULCAN_TONE4),
    ((12, "vulcan_tone5"), VULCAN_TONE5),
    ((6, "vulcan"), VULCAN),
    ((11, "black_heart"), BLACK_HEART),
    ((8, "computer"), COMPUTER),
    ((16, "desktop_computer"), DESKTOP_COMPUTER),
    ((7, "printer"), PRINTER),
    ((14, "computer_mouse"), COMPUTER_MOUSE),
    ((9, "trackball"), TRACKBALL),
    ((18, "frame_with_picture"), FRAME_WITH_PICTURE),
    ((14, "framed_picture"), FRAMED_PICTURE),
    ((19, "card_index_dividers"), CARD_INDEX_DIVIDERS),
    ((13, "card_file_box"), CARD_FILE_BOX),
    ((12, "file_cabinet"), FILE_CABINET),
    ((8, "trashcan"), TRASHCAN),
    ((11, "wastebasket"), WASTEBASKET),
    ((14, "notepad_spiral"), NOTEPAD_SPIRAL),
    ((15, "calendar_spiral"), CALENDAR_SPIRAL),
    ((5, "clamp"), CLAMP),
    ((11, "compression"), COMPRESSION),
    ((7, "old_key"), OLD_KEY),
    ((19, "rolled_up_newspaper"), ROLLED_UP_NEWSPAPER),
    ((6, "dagger"), DAGGER),
    ((13, "speaking_head"), SPEAKING_HEAD),
    ((18, "left_speech_bubble"), LEFT_SPEECH_BUBBLE),
    ((18, "right_anger_bubble"), RIGHT_ANGER_BUBBLE),
    ((10, "ballot_box"), BALLOT_BOX),
    ((9, "world_map"), WORLD_MAP),
    ((10, "mount_fuji"), MOUNT_FUJI),
    ((11, "tokyo_tower"), TOKYO_TOWER),
    ((17, "statue_of_liberty"), STATUE_OF_LIBERTY),
    ((9, "japan_map"), JAPAN_MAP),
    ((4, "moai"), MOAI),
    ((5, "moyai"), MOYAI),
    ((8, "grinning"), GRINNING),
    ((13, "grinning_face"), GRINNING_FACE),
    ((12, "beaming_face"), BEAMING_FACE),
    ((4, "grin"), GRIN),
    ((3, "joy"), JOY),
    ((4, "lmao"), LMAO),
    ((12, "tears_of_joy"), TEARS_OF_JOY),
    ((27, "grinning_face_with_big_eyes"), GRINNING_FACE_WITH_BIG_EYES),
    ((6, "smiley"), SMILEY),
    ((30, "grinning_face_with_closed_eyes"), GRINNING_FACE_WITH_CLOSED_EYES),
    ((5, "smile"), SMILE),
    ((24, "grinning_face_with_sweat"), GRINNING_FACE_WITH_SWEAT),
    ((11, "sweat_smile"), SWEAT_SMILE),
    ((8, "laughing"), LAUGHING),
    ((3, "lol"), LOL),
    ((9, "satisfied"), SATISFIED),
    ((14, "squinting_face"), SQUINTING_FACE),
    ((4, "halo"), HALO),
    ((8, "innocent"), INNOCENT),
    ((11, "smiling_imp"), SMILING_IMP),
    ((4, "wink"), WINK),
    ((12, "winking_face"), WINKING_FACE),
    ((5, "blush"), BLUSH),
    ((29, "smiling_face_with_closed_eyes"), SMILING_FACE_WITH_CLOSED_EYES),
    ((13, "savoring_food"), SAVORING_FOOD),
    ((3, "yum"), YUM),
    ((8, "relieved"), RELIEVED),
    ((13, "relieved_face"), RELIEVED_FACE),
    ((10, "heart_eyes"), HEART_EYES),
    ((28, "smiling_face_with_heart_eyes"), SMILING_FACE_WITH_HEART_EYES),
    ((28, "smiling_face_with_sunglasses"), SMILING_FACE_WITH_SUNGLASSES),
    ((15, "sunglasses_cool"), SUNGLASSES_COOL),
    ((8, "too_cool"), TOO_COOL),
    ((5, "smirk"), SMIRK),
    ((8, "smirking"), SMIRKING),
    ((13, "smirking_face"), SMIRKING_FACE),
    ((7, "neutral"), NEUTRAL),
    ((12, "neutral_face"), NEUTRAL_FACE),
    ((14, "expressionless"), EXPRESSIONLESS),
    ((19, "expressionless_face"), EXPRESSIONLESS_FACE),
    ((8, "unamused"), UNAMUSED),
    ((13, "unamused_face"), UNAMUSED_FACE),
    ((13, "downcast_face"), DOWNCAST_FACE),
    ((5, "sweat"), SWEAT),
    ((7, "pensive"), PENSIVE),
    ((12, "pensive_face"), PENSIVE_FACE),
    ((8, "confused"), CONFUSED),
    ((13, "confused_face"), CONFUSED_FACE),
    ((10, "confounded"), CONFOUNDED),
    ((15, "confounded_face"), CONFOUNDED_FACE),
    ((7, "kissing"), KISSING),
    ((12, "kissing_face"), KISSING_FACE),
    ((14, "blowing_a_kiss"), BLOWING_A_KISS),
    ((13, "kissing_heart"), KISSING_HEART),
    ((30, "kissing_face_with_smiling_eyes"), KISSING_FACE_WITH_SMILING_EYES),
    ((20, "kissing_smiling_eyes"), KISSING_SMILING_EYES),
    ((19, "kissing_closed_eyes"), KISSING_CLOSED_EYES),
    ((29, "kissing_face_with_closed_eyes"), KISSING_FACE_WITH_CLOSED_EYES),
    ((16, "face_with_tongue"), FACE_WITH_TONGUE),
    ((16, "stuck_out_tongue"), STUCK_OUT_TONGUE),
    ((28, "stuck_out_tongue_winking_eye"), STUCK_OUT_TONGUE_WINKING_EYE),
    ((28, "stuck_out_tongue_closed_eyes"), STUCK_OUT_TONGUE_CLOSED_EYES),
    ((12, "disappointed"), DISAPPOINTED),
    ((17, "disappointed_face"), DISAPPOINTED_FACE),
    ((7, "worried"), WORRIED),
    ((12, "worried_face"), WORRIED_FACE),
    ((5, "angry"), ANGRY),
    ((10, "angry_face"), ANGRY_FACE),
    ((4, "pout"), POUT),
    ((12, "pouting_face"), POUTING_FACE),
    ((4, "rage"), RAGE),
    ((3, "cry"), CRY),
    ((11, "crying_face"), CRYING_FACE),
    ((9, "persevere"), PERSEVERE),
    ((16, "persevering_face"), PERSEVERING_FACE),
    ((10, "nose_steam"), NOSE_STEAM),
    ((7, "triumph"), TRIUMPH),
    ((21, "disappointed_relieved"), DISAPPOINTED_RELIEVED),
    ((17, "sad_relieved_face"), SAD_RELIEVED_FACE),
    ((8, "frowning"), FROWNING),
    ((13, "frowning_face"), FROWNING_FACE),
    ((9, "anguished"), ANGUISHED),
    ((14, "anguished_face"), ANGUISHED_FACE),
    ((7, "fearful"), FEARFUL),
    ((12, "fearful_face"), FEARFUL_FACE),
    ((5, "weary"), WEARY),
    ((10, "weary_face"), WEARY_FACE),
    ((6, "sleepy"), SLEEPY),
    ((11, "sleepy_face"), SLEEPY_FACE),
    ((5, "tired"), TIRED),
    ((10, "tired_face"), TIRED_FACE),
    ((9, "grimacing"), GRIMACING),
    ((14, "grimacing_face"), GRIMACING_FACE),
    ((18, "loudly_crying_face"), LOUDLY_CRYING_FACE),
    ((3, "sob"), SOB),
    ((6, "exhale"), EXHALE),
    ((8, "exhaling"), EXHALING),
    ((20, "face_with_open_mouth"), FACE_WITH_OPEN_MOUTH),
    ((10, "open_mouth"), OPEN_MOUTH),
    ((6, "hushed"), HUSHED),
    ((11, "hushed_face"), HUSHED_FACE),
    ((7, "anxious"), ANXIOUS),
    ((12, "anxious_face"), ANXIOUS_FACE),
    ((10, "cold_sweat"), COLD_SWEAT),
    ((6, "scream"), SCREAM),
    ((17, "screaming_in_fear"), SCREAMING_IN_FEAR),
    ((10, "astonished"), ASTONISHED),
    ((15, "astonished_face"), ASTONISHED_FACE),
    ((7, "flushed"), FLUSHED),
    ((12, "flushed_face"), FLUSHED_FACE),
    ((8, "sleeping"), SLEEPING),
    ((13, "sleeping_face"), SLEEPING_FACE),
    ((10, "dizzy_eyes"), DIZZY_EYES),
    ((10, "dizzy_face"), DIZZY_FACE),
    ((11, "knocked_out"), KNOCKED_OUT),
    ((9, "in_clouds"), IN_CLOUDS),
    ((8, "no_mouth"), NO_MOUTH),
    ((4, "mask"), MASK),
    ((12, "medical_mask"), MEDICAL_MASK),
    ((29, "grinning_cat_with_closed_eyes"), GRINNING_CAT_WITH_CLOSED_EYES),
    ((9, "smile_cat"), SMILE_CAT),
    ((7, "joy_cat"), JOY_CAT),
    ((16, "tears_of_joy_cat"), TEARS_OF_JOY_CAT),
    ((12, "grinning_cat"), GRINNING_CAT),
    ((10, "smiley_cat"), SMILEY_CAT),
    ((14, "heart_eyes_cat"), HEART_EYES_CAT),
    ((27, "smiling_cat_with_heart_eyes"), SMILING_CAT_WITH_HEART_EYES),
    ((9, "smirk_cat"), SMIRK_CAT),
    ((13, "wry_smile_cat"), WRY_SMILE_CAT),
    ((11, "kissing_cat"), KISSING_CAT),
    ((11, "pouting_cat"), POUTING_CAT),
    ((10, "crying_cat"), CRYING_CAT),
    ((10, "scream_cat"), SCREAM_CAT),
    ((9, "weary_cat"), WEARY_CAT),
    ((22, "slightly_frowning_face"), SLIGHTLY_FROWNING_FACE),
    ((21, "slightly_smiling_face"), SLIGHTLY_SMILING_FACE),
    ((16, "upside_down_face"), UPSIDE_DOWN_FACE),
    ((12, "rolling_eyes"), ROLLING_EYES),
    ((24, "woman_gesturing_no_tone1"), WOMAN_GESTURING_NO_TONE1),
    ((22, "man_gesturing_no_tone1"), MAN_GESTURING_NO_TONE1),
    ((13, "no_good_tone1"), NO_GOOD_TONE1),
    ((25, "person_gesturing_no_tone1"), PERSON_GESTURING_NO_TONE1),
    ((24, "woman_gesturing_no_tone2"), WOMAN_GESTURING_NO_TONE2),
    ((22, "man_gesturing_no_tone2"), MAN_GESTURING_NO_TONE2),
    ((13, "no_good_tone2"), NO_GOOD_TONE2),
    ((25, "person_gesturing_no_tone2"), PERSON_GESTURING_NO_TONE2),
    ((24, "woman_gesturing_no_tone3"), WOMAN_GESTURING_NO_TONE3),
    ((22, "man_gesturing_no_tone3"), MAN_GESTURING_NO_TONE3),
    ((13, "no_good_tone3"), NO_GOOD_TONE3),
    ((25, "person_gesturing_no_tone3"), PERSON_GESTURING_NO_TONE3),
    ((24, "woman_gesturing_no_tone4"), WOMAN_GESTURING_NO_TONE4),
    ((22, "man_gesturing_no_tone4"), MAN_GESTURING_NO_TONE4),
    ((13, "no_good_tone4"), NO_GOOD_TONE4),
    ((25, "person_gesturing_no_tone4"), PERSON_GESTURING_NO_TONE4),
    ((24, "woman_gesturing_no_tone5"), WOMAN_GESTURING_NO_TONE5),
    ((22, "man_gesturing_no_tone5"), MAN_GESTURING_NO_TONE5),
    ((13, "no_good_tone5"), NO_GOOD_TONE5),
    ((25, "person_gesturing_no_tone5"), PERSON_GESTURING_NO_TONE5),
    ((18, "woman_gesturing_no"), WOMAN_GESTURING_NO),
    ((16, "man_gesturing_no"), MAN_GESTURING_NO),
    ((7, "no_good"), NO_GOOD),
    ((19, "person_gesturing_no"), PERSON_GESTURING_NO),
    ((24, "woman_gesturing_ok_tone1"), WOMAN_GESTURING_OK_TONE1),
    ((22, "man_gesturing_ok_tone1"), MAN_GESTURING_OK_TONE1),
    ((14, "all_good_tone1"), ALL_GOOD_TONE1),
    ((25, "person_gesturing_ok_tone1"), PERSON_GESTURING_OK_TONE1),
    ((24, "woman_gesturing_ok_tone2"), WOMAN_GESTURING_OK_TONE2),
    ((22, "man_gesturing_ok_tone2"), MAN_GESTURING_OK_TONE2),
    ((14, "all_good_tone2"), ALL_GOOD_TONE2),
    ((25, "person_gesturing_ok_tone2"), PERSON_GESTURING_OK_TONE2),
    ((24, "woman_gesturing_ok_tone3"), WOMAN_GESTURING_OK_TONE3),
    ((22, "man_gesturing_ok_tone3"), MAN_GESTURING_OK_TONE3),
    ((14, "all_good_tone3"), ALL_GOOD_TONE3),
    ((25, "person_gesturing_ok_tone3"), PERSON_GESTURING_OK_TONE3),
    ((24, "woman_gesturing_ok_tone4"), WOMAN_GESTURING_OK_TONE4),
    ((22, "man_gesturing_ok_tone4"), MAN_GESTURING_OK_TONE4),
    ((14, "all_good_tone4"), ALL_GOOD_TONE4),
    ((25, "person_gesturing_ok_tone4"), PERSON_GESTURING_OK_TONE4),
    ((24, "woman_gesturing_ok_tone5"), WOMAN_GESTURING_OK_TONE5),
    ((22, "man_gesturing_ok_tone5"), MAN_GESTURING_OK_TONE5),
    ((14, "all_good_tone5"), ALL_GOOD_TONE5),
    ((25, "person_gesturing_ok_tone5"), PERSON_GESTURING_OK_TONE5),
    ((18, "woman_gesturing_ok"), WOMAN_GESTURING_OK),
    ((16, "man_gesturing_ok"), MAN_GESTURING_OK),
    ((8, "all_good"), ALL_GOOD),
    ((19, "person_gesturing_ok"), PERSON_GESTURING_OK),
    ((18, "woman_bowing_tone1"), WOMAN_BOWING_TONE1),
    ((16, "man_bowing_tone1"), MAN_BOWING_TONE1),
    ((9, "bow_tone1"), BOW_TONE1),
    ((19, "person_bowing_tone1"), PERSON_BOWING_TONE1),
    ((18, "woman_bowing_tone2"), WOMAN_BOWING_TONE2),
    ((16, "man_bowing_tone2"), MAN_BOWING_TONE2),
    ((9, "bow_tone2"), BOW_TONE2),
    ((19, "person_bowing_tone2"), PERSON_BOWING_TONE2),
    ((18, "woman_bowing_tone3"), WOMAN_BOWING_TONE3),
    ((16, "man_bowing_tone3"), MAN_BOWING_TONE3),
    ((9, "bow_tone3"), BOW_TONE3),
    ((19, "person_bowing_tone3"), PERSON_BOWING_TONE3),
    ((18, "woman_bowing_tone4"), WOMAN_BOWING_TONE4),
    ((16, "man_bowing_tone4"), MAN_BOWING_TONE4),
    ((9, "bow_tone4"), BOW_TONE4),
    ((19, "person_bowing_tone4"), PERSON_BOWING_TONE4),
    ((18, "woman_bowing_tone5"), WOMAN_BOWING_TONE5),
    ((16, "man_bowing_tone5"), MAN_BOWING_TONE5),
    ((9, "bow_tone5"), BOW_TONE5),
    ((19, "person_bowing_tone5"), PERSON_BOWING_TONE5),
    ((12, "woman_bowing"), WOMAN_BOWING),
    ((10, "man_bowing"), MAN_BOWING),
    ((3, "bow"), BOW),
    ((13, "person_bowing"), PERSON_BOWING),
    ((11, "see_no_evil"), SEE_NO_EVIL),
    ((12, "hear_no_evil"), HEAR_NO_EVIL),
    ((13, "speak_no_evil"), SPEAK_NO_EVIL),
    ((24, "woman_raising_hand_tone1"), WOMAN_RAISING_HAND_TONE1),
    ((22, "man_raising_hand_tone1"), MAN_RAISING_HAND_TONE1),
    ((25, "person_raising_hand_tone1"), PERSON_RAISING_HAND_TONE1),
    ((24, "woman_raising_hand_tone2"), WOMAN_RAISING_HAND_TONE2),
    ((22, "man_raising_hand_tone2"), MAN_RAISING_HAND_TONE2),
    ((25, "person_raising_hand_tone2"), PERSON_RAISING_HAND_TONE2),
    ((24, "woman_raising_hand_tone3"), WOMAN_RAISING_HAND_TONE3),
    ((22, "man_raising_hand_tone3"), MAN_RAISING_HAND_TONE3),
    ((25, "person_raising_hand_tone3"), PERSON_RAISING_HAND_TONE3),
    ((24, "woman_raising_hand_tone4"), WOMAN_RAISING_HAND_TONE4),
    ((22, "man_raising_hand_tone4"), MAN_RAISING_HAND_TONE4),
    ((25, "person_raising_hand_tone4"), PERSON_RAISING_HAND_TONE4),
    ((24, "woman_raising_hand_tone5"), WOMAN_RAISING_HAND_TONE5),
    ((22, "man_raising_hand_tone5"), MAN_RAISING_HAND_TONE5),
    ((25, "person_raising_hand_tone5"), PERSON_RAISING_HAND_TONE5),
    ((18, "woman_raising_hand"), WOMAN_RAISING_HAND),
    ((16, "man_raising_hand"), MAN_RAISING_HAND),
    ((19, "person_raising_hand"), PERSON_RAISING_HAND),
    ((18, "raised_hands_tone1"), RAISED_HANDS_TONE1),
    ((18, "raised_hands_tone2"), RAISED_HANDS_TONE2),
    ((18, "raised_hands_tone3"), RAISED_HANDS_TONE3),
    ((18, "raised_hands_tone4"), RAISED_HANDS_TONE4),
    ((18, "raised_hands_tone5"), RAISED_HANDS_TONE5),
    ((12, "raised_hands"), RAISED_HANDS),
    ((20, "woman_frowning_tone1"), WOMAN_FROWNING_TONE1),
    ((18, "man_frowning_tone1"), MAN_FROWNING_TONE1),
    ((21, "person_frowning_tone1"), PERSON_FROWNING_TONE1),
    ((20, "woman_frowning_tone2"), WOMAN_FROWNING_TONE2),
    ((18, "man_frowning_tone2"), MAN_FROWNING_TONE2),
    ((21, "person_frowning_tone2"), PERSON_FROWNING_TONE2),
    ((20, "woman_frowning_tone3"), WOMAN_FROWNING_TONE3),
    ((18, "man_frowning_tone3"), MAN_FROWNING_TONE3),
    ((21, "person_frowning_tone3"), PERSON_FROWNING_TONE3),
    ((20, "woman_frowning_tone4"), WOMAN_FROWNING_TONE4),
    ((18, "man_frowning_tone4"), MAN_FROWNING_TONE4),
    ((21, "person_frowning_tone4"), PERSON_FROWNING_TONE4),
    ((20, "woman_frowning_tone5"), WOMAN_FROWNING_TONE5),
    ((18, "man_frowning_tone5"), MAN_FROWNING_TONE5),
    ((21, "person_frowning_tone5"), PERSON_FROWNING_TONE5),
    ((14, "woman_frowning"), WOMAN_FROWNING),
    ((12, "man_frowning"), MAN_FROWNING),
    ((15, "person_frowning"), PERSON_FROWNING),
    ((19, "woman_pouting_tone1"), WOMAN_POUTING_TONE1),
    ((17, "man_pouting_tone1"), MAN_POUTING_TONE1),
    ((20, "person_pouting_tone1"), PERSON_POUTING_TONE1),
    ((13, "pouting_tone1"), POUTING_TONE1),
    ((19, "woman_pouting_tone2"), WOMAN_POUTING_TONE2),
    ((17, "man_pouting_tone2"), MAN_POUTING_TONE2),
    ((20, "person_pouting_tone2"), PERSON_POUTING_TONE2),
    ((13, "pouting_tone2"), POUTING_TONE2),
    ((19, "woman_pouting_tone3"), WOMAN_POUTING_TONE3),
    ((17, "man_pouting_tone3"), MAN_POUTING_TONE3),
    ((20, "person_pouting_tone3"), PERSON_POUTING_TONE3),
    ((13, "pouting_tone3"), POUTING_TONE3),
    ((19, "woman_pouting_tone4"), WOMAN_POUTING_TONE4),
    ((17, "man_pouting_tone4"), MAN_POUTING_TONE4),
    ((20, "person_pouting_tone4"), PERSON_POUTING_TONE4),
    ((13, "pouting_tone4"), POUTING_TONE4),
    ((19, "woman_pouting_tone5"), WOMAN_POUTING_TONE5),
    ((17, "man_pouting_tone5"), MAN_POUTING_TONE5),
    ((20, "person_pouting_tone5"), PERSON_POUTING_TONE5),
    ((13, "pouting_tone5"), POUTING_TONE5),
    ((13, "woman_pouting"), WOMAN_POUTING),
    ((11, "man_pouting"), MAN_POUTING),
    ((14, "person_pouting"), PERSON_POUTING),
    ((7, "pouting"), POUTING),
    ((18, "folded_hands_tone1"), FOLDED_HANDS_TONE1),
    ((10, "pray_tone1"), PRAY_TONE1),
    ((18, "folded_hands_tone2"), FOLDED_HANDS_TONE2),
    ((10, "pray_tone2"), PRAY_TONE2),
    ((18, "folded_hands_tone3"), FOLDED_HANDS_TONE3),
    ((10, "pray_tone3"), PRAY_TONE3),
    ((18, "folded_hands_tone4"), FOLDED_HANDS_TONE4),
    ((10, "pray_tone4"), PRAY_TONE4),
    ((18, "folded_hands_tone5"), FOLDED_HANDS_TONE5),
    ((10, "pray_tone5"), PRAY_TONE5),
    ((12, "folded_hands"), FOLDED_HANDS),
    ((4, "pray"), PRAY),
    ((6, "rocket"), ROCKET),
    ((10, "helicopter"), HELICOPTER),
    ((16, "steam_locomotive"), STEAM_LOCOMOTIVE),
    ((11, "railway_car"), RAILWAY_CAR),
    ((16, "bullettrain_side"), BULLETTRAIN_SIDE),
    ((17, "bullettrain_front"), BULLETTRAIN_FRONT),
    ((5, "train"), TRAIN),
    ((5, "metro"), METRO),
    ((10, "light_rail"), LIGHT_RAIL),
    ((7, "station"), STATION),
    ((4, "tram"), TRAM),
    ((8, "tram_car"), TRAM_CAR),
    ((3, "bus"), BUS),
    ((12, "oncoming_bus"), ONCOMING_BUS),
    ((10, "trolleybus"), TROLLEYBUS),
    ((7, "busstop"), BUSSTOP),
    ((7, "minibus"), MINIBUS),
    ((9, "ambulance"), AMBULANCE),
    ((11, "fire_engine"), FIRE_ENGINE),
    ((10, "police_car"), POLICE_CAR),
    ((19, "oncoming_police_car"), ONCOMING_POLICE_CAR),
    ((4, "taxi"), TAXI),
    ((13, "oncoming_taxi"), ONCOMING_TAXI),
    ((3, "car"), CAR),
    ((7, "red_car"), RED_CAR),
    ((19, "oncoming_automobile"), ONCOMING_AUTOMOBILE),
    ((8, "blue_car"), BLUE_CAR),
    ((3, "suv"), SUV),
    ((14, "delivery_truck"), DELIVERY_TRUCK),
    ((5, "truck"), TRUCK),
    ((17, "articulated_lorry"), ARTICULATED_LORRY),
    ((7, "tractor"), TRACTOR),
    ((8, "monorail"), MONORAIL),
    ((16, "mountain_railway"), MOUNTAIN_RAILWAY),
    ((18, "suspension_railway"), SUSPENSION_RAILWAY),
    ((17, "mountain_cableway"), MOUNTAIN_CABLEWAY),
    ((14, "aerial_tramway"), AERIAL_TRAMWAY),
    ((4, "ship"), SHIP),
    ((23, "woman_rowing_boat_tone1"), WOMAN_ROWING_BOAT_TONE1),
    ((21, "man_rowing_boat_tone1"), MAN_ROWING_BOAT_TONE1),
    ((24, "person_rowing_boat_tone1"), PERSON_ROWING_BOAT_TONE1),
    ((13, "rowboat_tone1"), ROWBOAT_TONE1),
    ((23, "woman_rowing_boat_tone2"), WOMAN_ROWING_BOAT_TONE2),
    ((21, "man_rowing_boat_tone2"), MAN_ROWING_BOAT_TONE2),
    ((24, "person_rowing_boat_tone2"), PERSON_ROWING_BOAT_TONE2),
    ((13, "rowboat_tone2"), ROWBOAT_TONE2),
    ((23, "woman_rowing_boat_tone3"), WOMAN_ROWING_BOAT_TONE3),
    ((21, "man_rowing_boat_tone3"), MAN_ROWING_BOAT_TONE3),
    ((24, "person_rowing_boat_tone3"), PERSON_ROWING_BOAT_TONE3),
    ((13, "rowboat_tone3"), ROWBOAT_TONE3),
    ((23, "woman_rowing_boat_tone4"), WOMAN_ROWING_BOAT_TONE4),
    ((21, "man_rowing_boat_tone4"), MAN_ROWING_BOAT_TONE4),
    ((24, "person_rowing_boat_tone4"), PERSON_ROWING_BOAT_TONE4),
    ((13, "rowboat_tone4"), ROWBOAT_TONE4),
    ((23, "woman_rowing_boat_tone5"), WOMAN_ROWING_BOAT_TONE5),
    ((21, "man_rowing_boat_tone5"), MAN_ROWING_BOAT_TONE5),
    ((24, "person_rowing_boat_tone5"), PERSON_ROWING_BOAT_TONE5),
    ((13, "rowboat_tone5"), ROWBOAT_TONE5),
    ((17, "woman_rowing_boat"), WOMAN_ROWING_BOAT),
    ((15, "man_rowing_boat"), MAN_ROWING_BOAT),
    ((18, "person_rowing_boat"), PERSON_ROWING_BOAT),
    ((7, "rowboat"), ROWBOAT),
    ((9, "speedboat"), SPEEDBOAT),
    ((13, "traffic_light"), TRAFFIC_LIGHT),
    ((22, "vertical_traffic_light"), VERTICAL_TRAFFIC_LIGHT),
    ((12, "construction"), CONSTRUCTION),
    ((14, "rotating_light"), ROTATING_LIGHT),
    ((15, "triangular_flag"), TRIANGULAR_FLAG),
    ((23, "triangular_flag_on_post"), TRIANGULAR_FLAG_ON_POST),
    ((4, "door"), DOOR),
    ((13, "no_entry_sign"), NO_ENTRY_SIGN),
    ((9, "cigarette"), CIGARETTE),
    ((7, "smoking"), SMOKING),
    ((10, "no_smoking"), NO_SMOKING),
    ((10, "litter_bin"), LITTER_BIN),
    ((23, "put_litter_in_its_place"), PUT_LITTER_IN_ITS_PLACE),
    ((13, "do_not_litter"), DO_NOT_LITTER),
    ((12, "no_littering"), NO_LITTERING),
    ((13, "potable_water"), POTABLE_WATER),
    ((17, "non-potable_water"), NON_POTABLE_WATER),
    ((7, "bicycle"), BICYCLE),
    ((4, "bike"), BIKE),
    ((11, "no_bicycles"), NO_BICYCLES),
    ((18, "woman_biking_tone1"), WOMAN_BIKING_TONE1),
    ((16, "man_biking_tone1"), MAN_BIKING_TONE1),
    ((15, "bicyclist_tone1"), BICYCLIST_TONE1),
    ((12, "biking_tone1"), BIKING_TONE1),
    ((19, "person_biking_tone1"), PERSON_BIKING_TONE1),
    ((18, "woman_biking_tone2"), WOMAN_BIKING_TONE2),
    ((16, "man_biking_tone2"), MAN_BIKING_TONE2),
    ((15, "bicyclist_tone2"), BICYCLIST_TONE2),
    ((12, "biking_tone2"), BIKING_TONE2),
    ((19, "person_biking_tone2"), PERSON_BIKING_TONE2),
    ((18, "woman_biking_tone3"), WOMAN_BIKING_TONE3),
    ((16, "man_biking_tone3"), MAN_BIKING_TONE3),
    ((15, "bicyclist_tone3"), BICYCLIST_TONE3),
    ((12, "biking_tone3"), BIKING_TONE3),
    ((19, "person_biking_tone3"), PERSON_BIKING_TONE3),
    ((18, "woman_biking_tone4"), WOMAN_BIKING_TONE4),
    ((16, "man_biking_tone4"), MAN_BIKING_TONE4),
    ((15, "bicyclist_tone4"), BICYCLIST_TONE4),
    ((12, "biking_tone4"), BIKING_TONE4),
    ((19, "person_biking_tone4"), PERSON_BIKING_TONE4),
    ((18, "woman_biking_tone5"), WOMAN_BIKING_TONE5),
    ((16, "man_biking_tone5"), MAN_BIKING_TONE5),
    ((15, "bicyclist_tone5"), BICYCLIST_TONE5),
    ((12, "biking_tone5"), BIKING_TONE5),
    ((19, "person_biking_tone5"), PERSON_BIKING_TONE5),
    ((12, "woman_biking"), WOMAN_BIKING),
    ((10, "man_biking"), MAN_BIKING),
    ((9, "bicyclist"), BICYCLIST),
    ((6, "biking"), BIKING),
    ((13, "person_biking"), PERSON_BIKING),
    ((27, "woman_mountain_biking_tone1"), WOMAN_MOUNTAIN_BIKING_TONE1),
    ((25, "man_mountain_biking_tone1"), MAN_MOUNTAIN_BIKING_TONE1),
    ((24, "mountain_bicyclist_tone1"), MOUNTAIN_BICYCLIST_TONE1),
    ((21, "mountain_biking_tone1"), MOUNTAIN_BIKING_TONE1),
    ((28, "person_mountain_biking_tone1"), PERSON_MOUNTAIN_BIKING_TONE1),
    ((27, "woman_mountain_biking_tone2"), WOMAN_MOUNTAIN_BIKING_TONE2),
    ((25, "man_mountain_biking_tone2"), MAN_MOUNTAIN_BIKING_TONE2),
    ((24, "mountain_bicyclist_tone2"), MOUNTAIN_BICYCLIST_TONE2),
    ((21, "mountain_biking_tone2"), MOUNTAIN_BIKING_TONE2),
    ((28, "person_mountain_biking_tone2"), PERSON_MOUNTAIN_BIKING_TONE2),
    ((27, "woman_mountain_biking_tone3"), WOMAN_MOUNTAIN_BIKING_TONE3),
    ((25, "man_mountain_biking_tone3"), MAN_MOUNTAIN_BIKING_TONE3),
    ((24, "mountain_bicyclist_tone3"), MOUNTAIN_BICYCLIST_TONE3),
    ((21, "mountain_biking_tone3"), MOUNTAIN_BIKING_TONE3),
    ((28, "person_mountain_biking_tone3"), PERSON_MOUNTAIN_BIKING_TONE3),
    ((27, "woman_mountain_biking_tone4"), WOMAN_MOUNTAIN_BIKING_TONE4),
    ((25, "man_mountain_biking_tone4"), MAN_MOUNTAIN_BIKING_TONE4),
    ((24, "mountain_bicyclist_tone4"), MOUNTAIN_BICYCLIST_TONE4),
    ((21, "mountain_biking_tone4"), MOUNTAIN_BIKING_TONE4),
    ((28, "person_mountain_biking_tone4"), PERSON_MOUNTAIN_BIKING_TONE4),
    ((27, "woman_mountain_biking_tone5"), WOMAN_MOUNTAIN_BIKING_TONE5),
    ((25, "man_mountain_biking_tone5"), MAN_MOUNTAIN_BIKING_TONE5),
    ((24, "mountain_bicyclist_tone5"), MOUNTAIN_BICYCLIST_TONE5),
    ((21, "mountain_biking_tone5"), MOUNTAIN_BIKING_TONE5),
    ((28, "person_mountain_biking_tone5"), PERSON_MOUNTAIN_BIKING_TONE5),
    ((21, "woman_mountain_biking"), WOMAN_MOUNTAIN_BIKING),
    ((19, "man_mountain_biking"), MAN_MOUNTAIN_BIKING),
    ((18, "mountain_bicyclist"), MOUNTAIN_BICYCLIST),
    ((15, "mountain_biking"), MOUNTAIN_BIKING),
    ((22, "person_mountain_biking"), PERSON_MOUNTAIN_BIKING),
    ((19, "woman_walking_tone1"), WOMAN_WALKING_TONE1),
    ((17, "man_walking_tone1"), MAN_WALKING_TONE1),
    ((20, "person_walking_tone1"), PERSON_WALKING_TONE1),
    ((13, "walking_tone1"), WALKING_TONE1),
    ((19, "woman_walking_tone2"), WOMAN_WALKING_TONE2),
    ((17, "man_walking_tone2"), MAN_WALKING_TONE2),
    ((20, "person_walking_tone2"), PERSON_WALKING_TONE2),
    ((13, "walking_tone2"), WALKING_TONE2),
    ((19, "woman_walking_tone3"), WOMAN_WALKING_TONE3),
    ((17, "man_walking_tone3"), MAN_WALKING_TONE3),
    ((20, "person_walking_tone3"), PERSON_WALKING_TONE3),
    ((13, "walking_tone3"), WALKING_TONE3),
    ((19, "woman_walking_tone4"), WOMAN_WALKING_TONE4),
    ((17, "man_walking_tone4"), MAN_WALKING_TONE4),
    ((20, "person_walking_tone4"), PERSON_WALKING_TONE4),
    ((13, "walking_tone4"), WALKING_TONE4),
    ((19, "woman_walking_tone5"), WOMAN_WALKING_TONE5),
    ((17, "man_walking_tone5"), MAN_WALKING_TONE5),
    ((20, "person_walking_tone5"), PERSON_WALKING_TONE5),
    ((13, "walking_tone5"), WALKING_TONE5),
    ((13, "woman_walking"), WOMAN_WALKING),
    ((11, "man_walking"), MAN_WALKING),
    ((14, "person_walking"), PERSON_WALKING),
    ((7, "walking"), WALKING),
    ((14, "no_pedestrians"), NO_PEDESTRIANS),
    ((17, "children_crossing"), CHILDREN_CROSSING),
    ((4, "mens"), MENS),
    ((6, "womens"), WOMENS),
    ((8, "bathroom"), BATHROOM),
    ((8, "restroom"), RESTROOM),
    ((11, "baby_symbol"), BABY_SYMBOL),
    ((6, "toilet"), TOILET),
    ((12, "water_closet"), WATER_CLOSET),
    ((2, "wc"), WC),
    ((6, "shower"), SHOWER),
    ((10, "bath_tone1"), BATH_TONE1),
    ((24, "person_taking_bath_tone1"), PERSON_TAKING_BATH_TONE1),
    ((10, "bath_tone2"), BATH_TONE2),
    ((24, "person_taking_bath_tone2"), PERSON_TAKING_BATH_TONE2),
    ((10, "bath_tone3"), BATH_TONE3),
    ((24, "person_taking_bath_tone3"), PERSON_TAKING_BATH_TONE3),
    ((10, "bath_tone4"), BATH_TONE4),
    ((24, "person_taking_bath_tone4"), PERSON_TAKING_BATH_TONE4),
    ((10, "bath_tone5"), BATH_TONE5),
    ((24, "person_taking_bath_tone5"), PERSON_TAKING_BATH_TONE5),
    ((4, "bath"), BATH),
    ((18, "person_taking_bath"), PERSON_TAKING_BATH),
    ((7, "bathtub"), BATHTUB),
    ((16, "passport_control"), PASSPORT_CONTROL),
    ((7, "customs"), CUSTOMS),
    ((13, "baggage_claim"), BAGGAGE_CLAIM),
    ((12, "left_luggage"), LEFT_LUGGAGE),
    ((14, "couch_and_lamp"), COUCH_AND_LAMP),
    ((19, "person_in_bed_tone1"), PERSON_IN_BED_TONE1),
    ((28, "sleeping_accommodation_tone1"), SLEEPING_ACCOMMODATION_TONE1),
    ((19, "person_in_bed_tone2"), PERSON_IN_BED_TONE2),
    ((28, "sleeping_accommodation_tone2"), SLEEPING_ACCOMMODATION_TONE2),
    ((19, "person_in_bed_tone3"), PERSON_IN_BED_TONE3),
    ((28, "sleeping_accommodation_tone3"), SLEEPING_ACCOMMODATION_TONE3),
    ((19, "person_in_bed_tone4"), PERSON_IN_BED_TONE4),
    ((28, "sleeping_accommodation_tone4"), SLEEPING_ACCOMMODATION_TONE4),
    ((19, "person_in_bed_tone5"), PERSON_IN_BED_TONE5),
    ((28, "sleeping_accommodation_tone5"), SLEEPING_ACCOMMODATION_TONE5),
    ((13, "person_in_bed"), PERSON_IN_BED),
    ((22, "sleeping_accommodation"), SLEEPING_ACCOMMODATION),
    ((13, "shopping_bags"), SHOPPING_BAGS),
    ((7, "bellhop"), BELLHOP),
    ((3, "bed"), BED),
    ((16, "place_of_worship"), PLACE_OF_WORSHIP),
    ((14, "octagonal_sign"), OCTAGONAL_SIGN),
    ((9, "stop_sign"), STOP_SIGN),
    ((13, "shopping_cart"), SHOPPING_CART),
    ((12, "hindu_temple"), HINDU_TEMPLE),
    ((3, "hut"), HUT),
    ((8, "elevator"), ELEVATOR),
    ((16, "playground_slide"), PLAYGROUND_SLIDE),
    ((5, "slide"), SLIDE),
    ((5, "wheel"), WHEEL),
    ((8, "lifebuoy"), LIFEBUOY),
    ((9, "ring_buoy"), RING_BUOY),
    ((17, "hammer_and_wrench"), HAMMER_AND_WRENCH),
    ((6, "shield"), SHIELD),
    ((8, "oil_drum"), OIL_DRUM),
    ((8, "motorway"), MOTORWAY),
    ((13, "railway_track"), RAILWAY_TRACK),
    ((9, "motorboat"), MOTORBOAT),
    ((14, "small_airplane"), SMALL_AIRPLANE),
    ((18, "airplane_departure"), AIRPLANE_DEPARTURE),
    ((17, "airplane_arriving"), AIRPLANE_ARRIVING),
    ((9, "satellite"), SATELLITE),
    ((11, "cruise_ship"), CRUISE_SHIP),
    ((14, "passenger_ship"), PASSENGER_SHIP),
    ((7, "scooter"), SCOOTER),
    ((13, "motor_scooter"), MOTOR_SCOOTER),
    ((5, "canoe"), CANOE),
    ((4, "sled"), SLED),
    ((13, "flying_saucer"), FLYING_SAUCER),
    ((10, "skateboard"), SKATEBOARD),
    ((13, "auto_rickshaw"), AUTO_RICKSHAW),
    ((12, "pickup_truck"), PICKUP_TRUCK),
    ((12, "roller_skate"), ROLLER_SKATE),
    ((13, "orange_circle"), ORANGE_CIRCLE),
    ((13, "yellow_circle"), YELLOW_CIRCLE),
    ((12, "green_circle"), GREEN_CIRCLE),
    ((13, "purple_circle"), PURPLE_CIRCLE),
    ((12, "brown_circle"), BROWN_CIRCLE),
    ((10, "red_square"), RED_SQUARE),
    ((11, "blue_square"), BLUE_SQUARE),
    ((13, "orange_square"), ORANGE_SQUARE),
    ((13, "yellow_square"), YELLOW_SQUARE),
    ((12, "green_square"), GREEN_SQUARE),
    ((13, "purple_square"), PURPLE_SQUARE),
    ((12, "brown_square"), BROWN_SQUARE),
    ((17, "heavy_equals_sign"), HEAVY_EQUALS_SIGN),
    ((11, "pinch_tone1"), PINCH_TONE1),
    ((21, "pinched_fingers_tone1"), PINCHED_FINGERS_TONE1),
    ((11, "pinch_tone2"), PINCH_TONE2),
    ((21, "pinched_fingers_tone2"), PINCHED_FINGERS_TONE2),
    ((11, "pinch_tone3"), PINCH_TONE3),
    ((21, "pinched_fingers_tone3"), PINCHED_FINGERS_TONE3),
    ((11, "pinch_tone4"), PINCH_TONE4),
    ((21, "pinched_fingers_tone4"), PINCHED_FINGERS_TONE4),
    ((11, "pinch_tone5"), PINCH_TONE5),
    ((21, "pinched_fingers_tone5"), PINCHED_FINGERS_TONE5),
    ((5, "pinch"), PINCH),
    ((15, "pinched_fingers"), PINCHED_FINGERS),
    ((11, "white_heart"), WHITE_HEART),
    ((11, "brown_heart"), BROWN_HEART),
    ((19, "pinching_hand_tone1"), PINCHING_HAND_TONE1),
    ((19, "pinching_hand_tone2"), PINCHING_HAND_TONE2),
    ((19, "pinching_hand_tone3"), PINCHING_HAND_TONE3),
    ((19, "pinching_hand_tone4"), PINCHING_HAND_TONE4),
    ((19, "pinching_hand_tone5"), PINCHING_HAND_TONE5),
    ((13, "pinching_hand"), PINCHING_HAND),
    ((12, "zipper_mouth"), ZIPPER_MOUTH),
    ((17, "zipper_mouth_face"), ZIPPER_MOUTH_FACE),
    ((16, "money_mouth_face"), MONEY_MOUTH_FACE),
    ((21, "face_with_thermometer"), FACE_WITH_THERMOMETER),
    ((4, "nerd"), NERD),
    ((9, "nerd_face"), NERD_FACE),
    ((8, "thinking"), THINKING),
    ((13, "thinking_face"), THINKING_FACE),
    ((3, "wtf"), WTF),
    ((22, "face_with_head_bandage"), FACE_WITH_HEAD_BANDAGE),
    ((5, "robot"), ROBOT),
    ((10, "robot_face"), ROBOT_FACE),
    ((3, "hug"), HUG),
    ((7, "hugging"), HUGGING),
    ((12, "hugging_face"), HUGGING_FACE),
    ((11, "metal_tone1"), METAL_TONE1),
    ((23, "sign_of_the_horns_tone1"), SIGN_OF_THE_HORNS_TONE1),
    ((11, "metal_tone2"), METAL_TONE2),
    ((23, "sign_of_the_horns_tone2"), SIGN_OF_THE_HORNS_TONE2),
    ((11, "metal_tone3"), METAL_TONE3),
    ((23, "sign_of_the_horns_tone3"), SIGN_OF_THE_HORNS_TONE3),
    ((11, "metal_tone4"), METAL_TONE4),
    ((23, "sign_of_the_horns_tone4"), SIGN_OF_THE_HORNS_TONE4),
    ((11, "metal_tone5"), METAL_TONE5),
    ((23, "sign_of_the_horns_tone5"), SIGN_OF_THE_HORNS_TONE5),
    ((5, "metal"), METAL),
    ((17, "sign_of_the_horns"), SIGN_OF_THE_HORNS),
    ((18, "call_me_hand_tone1"), CALL_ME_HAND_TONE1),
    ((18, "call_me_hand_tone2"), CALL_ME_HAND_TONE2),
    ((18, "call_me_hand_tone3"), CALL_ME_HAND_TONE3),
    ((18, "call_me_hand_tone4"), CALL_ME_HAND_TONE4),
    ((18, "call_me_hand_tone5"), CALL_ME_HAND_TONE5),
    ((12, "call_me_hand"), CALL_ME_HAND),
    ((25, "raised_back_of_hand_tone1"), RAISED_BACK_OF_HAND_TONE1),
    ((25, "raised_back_of_hand_tone2"), RAISED_BACK_OF_HAND_TONE2),
    ((25, "raised_back_of_hand_tone3"), RAISED_BACK_OF_HAND_TONE3),
    ((25, "raised_back_of_hand_tone4"), RAISED_BACK_OF_HAND_TONE4),
    ((25, "raised_back_of_hand_tone5"), RAISED_BACK_OF_HAND_TONE5),
    ((19, "raised_back_of_hand"), RAISED_BACK_OF_HAND),
    ((22, "left_facing_fist_tone1"), LEFT_FACING_FIST_TONE1),
    ((22, "left_facing_fist_tone2"), LEFT_FACING_FIST_TONE2),
    ((22, "left_facing_fist_tone3"), LEFT_FACING_FIST_TONE3),
    ((22, "left_facing_fist_tone4"), LEFT_FACING_FIST_TONE4),
    ((22, "left_facing_fist_tone5"), LEFT_FACING_FIST_TONE5),
    ((16, "left_facing_fist"), LEFT_FACING_FIST),
    ((23, "right_facing_fist_tone1"), RIGHT_FACING_FIST_TONE1),
    ((23, "right_facing_fist_tone2"), RIGHT_FACING_FIST_TONE2),
    ((23, "right_facing_fist_tone3"), RIGHT_FACING_FIST_TONE3),
    ((23, "right_facing_fist_tone4"), RIGHT_FACING_FIST_TONE4),
    ((23, "right_facing_fist_tone5"), RIGHT_FACING_FIST_TONE5),
    ((17, "right_facing_fist"), RIGHT_FACING_FIST),
    ((15, "handshake_tone1"), HANDSHAKE_TONE1),
    ((15, "handshake_tone2"), HANDSHAKE_TONE2),
    ((15, "handshake_tone3"), HANDSHAKE_TONE3),
    ((15, "handshake_tone4"), HANDSHAKE_TONE4),
    ((15, "handshake_tone5"), HANDSHAKE_TONE5),
    ((9, "handshake"), HANDSHAKE),
    ((21, "fingers_crossed_tone1"), FINGERS_CROSSED_TONE1),
    ((21, "fingers_crossed_tone2"), FINGERS_CROSSED_TONE2),
    ((21, "fingers_crossed_tone3"), FINGERS_CROSSED_TONE3),
    ((21, "fingers_crossed_tone4"), FINGERS_CROSSED_TONE4),
    ((21, "fingers_crossed_tone5"), FINGERS_CROSSED_TONE5),
    ((15, "fingers_crossed"), FINGERS_CROSSED),
    ((22, "love_you_gesture_tone1"), LOVE_YOU_GESTURE_TONE1),
    ((22, "love_you_gesture_tone2"), LOVE_YOU_GESTURE_TONE2),
    ((22, "love_you_gesture_tone3"), LOVE_YOU_GESTURE_TONE3),
    ((22, "love_you_gesture_tone4"), LOVE_YOU_GESTURE_TONE4),
    ((22, "love_you_gesture_tone5"), LOVE_YOU_GESTURE_TONE5),
    ((16, "love_you_gesture"), LOVE_YOU_GESTURE),
    ((6, "cowboy"), COWBOY),
    ((11, "cowboy_face"), COWBOY_FACE),
    ((5, "clown"), CLOWN),
    ((10, "clown_face"), CLOWN_FACE),
    ((9, "nauseated"), NAUSEATED),
    ((14, "nauseated_face"), NAUSEATED_FACE),
    ((4, "rofl"), ROFL),
    ((8, "drooling"), DROOLING),
    ((13, "drooling_face"), DROOLING_FACE),
    ((5, "lying"), LYING),
    ((10, "lying_face"), LYING_FACE),
    ((23, "woman_facepalming_tone1"), WOMAN_FACEPALMING_TONE1),
    ((21, "man_facepalming_tone1"), MAN_FACEPALMING_TONE1),
    ((14, "facepalm_tone1"), FACEPALM_TONE1),
    ((24, "person_facepalming_tone1"), PERSON_FACEPALMING_TONE1),
    ((23, "woman_facepalming_tone2"), WOMAN_FACEPALMING_TONE2),
    ((21, "man_facepalming_tone2"), MAN_FACEPALMING_TONE2),
    ((14, "facepalm_tone2"), FACEPALM_TONE2),
    ((24, "person_facepalming_tone2"), PERSON_FACEPALMING_TONE2),
    ((23, "woman_facepalming_tone3"), WOMAN_FACEPALMING_TONE3),
    ((21, "man_facepalming_tone3"), MAN_FACEPALMING_TONE3),
    ((14, "facepalm_tone3"), FACEPALM_TONE3),
    ((24, "person_facepalming_tone3"), PERSON_FACEPALMING_TONE3),
    ((23, "woman_facepalming_tone4"), WOMAN_FACEPALMING_TONE4),
    ((21, "man_facepalming_tone4"), MAN_FACEPALMING_TONE4),
    ((14, "facepalm_tone4"), FACEPALM_TONE4),
    ((24, "person_facepalming_tone4"), PERSON_FACEPALMING_TONE4),
    ((23, "woman_facepalming_tone5"), WOMAN_FACEPALMING_TONE5),
    ((21, "man_facepalming_tone5"), MAN_FACEPALMING_TONE5),
    ((14, "facepalm_tone5"), FACEPALM_TONE5),
    ((24, "person_facepalming_tone5"), PERSON_FACEPALMING_TONE5),
    ((17, "woman_facepalming"), WOMAN_FACEPALMING),
    ((15, "man_facepalming"), MAN_FACEPALMING),
    ((8, "facepalm"), FACEPALM),
    ((18, "person_facepalming"), PERSON_FACEPALMING),
    ((8, "sneezing"), SNEEZING),
    ((13, "sneezing_face"), SNEEZING_FACE),
    ((24, "face_with_raised_eyebrow"), FACE_WITH_RAISED_EYEBROW),
    ((14, "raised_eyebrow"), RAISED_EYEBROW),
    ((11, "star_struck"), STAR_STRUCK),
    ((4, "zany"), ZANY),
    ((9, "zany_face"), ZANY_FACE),
    ((5, "shush"), SHUSH),
    ((13, "shushing_face"), SHUSHING_FACE),
    ((8, "censored"), CENSORED),
    ((26, "face_with_symbols_on_mouth"), FACE_WITH_SYMBOLS_ON_MOUTH),
    ((25, "face_with_hand_over_mouth"), FACE_WITH_HAND_OVER_MOUTH),
    ((15, "hand_over_mouth"), HAND_OVER_MOUTH),
    ((13, "face_vomiting"), FACE_VOMITING),
    ((8, "vomiting"), VOMITING),
    ((14, "exploding_head"), EXPLODING_HEAD),
    ((20, "pregnant_woman_tone1"), PREGNANT_WOMAN_TONE1),
    ((20, "pregnant_woman_tone2"), PREGNANT_WOMAN_TONE2),
    ((20, "pregnant_woman_tone3"), PREGNANT_WOMAN_TONE3),
    ((20, "pregnant_woman_tone4"), PREGNANT_WOMAN_TONE4),
    ((20, "pregnant_woman_tone5"), PREGNANT_WOMAN_TONE5),
    ((14, "pregnant_woman"), PREGNANT_WOMAN),
    ((20, "breast_feeding_tone1"), BREAST_FEEDING_TONE1),
    ((20, "breast_feeding_tone2"), BREAST_FEEDING_TONE2),
    ((20, "breast_feeding_tone3"), BREAST_FEEDING_TONE3),
    ((20, "breast_feeding_tone4"), BREAST_FEEDING_TONE4),
    ((20, "breast_feeding_tone5"), BREAST_FEEDING_TONE5),
    ((14, "breast_feeding"), BREAST_FEEDING),
    ((23, "palms_up_together_tone1"), PALMS_UP_TOGETHER_TONE1),
    ((23, "palms_up_together_tone2"), PALMS_UP_TOGETHER_TONE2),
    ((23, "palms_up_together_tone3"), PALMS_UP_TOGETHER_TONE3),
    ((23, "palms_up_together_tone4"), PALMS_UP_TOGETHER_TONE4),
    ((23, "palms_up_together_tone5"), PALMS_UP_TOGETHER_TONE5),
    ((17, "palms_up_together"), PALMS_UP_TOGETHER),
    ((12, "selfie_tone1"), SELFIE_TONE1),
    ((12, "selfie_tone2"), SELFIE_TONE2),
    ((12, "selfie_tone3"), SELFIE_TONE3),
    ((12, "selfie_tone4"), SELFIE_TONE4),
    ((12, "selfie_tone5"), SELFIE_TONE5),
    ((6, "selfie"), SELFIE),
    ((12, "prince_tone1"), PRINCE_TONE1),
    ((12, "prince_tone2"), PRINCE_TONE2),
    ((12, "prince_tone3"), PRINCE_TONE3),
    ((12, "prince_tone4"), PRINCE_TONE4),
    ((12, "prince_tone5"), PRINCE_TONE5),
    ((6, "prince"), PRINCE),
    ((21, "woman_in_tuxedo_tone1"), WOMAN_IN_TUXEDO_TONE1),
    ((19, "man_in_tuxedo_tone1"), MAN_IN_TUXEDO_TONE1),
    ((22, "person_in_tuxedo_tone1"), PERSON_IN_TUXEDO_TONE1),
    ((21, "woman_in_tuxedo_tone2"), WOMAN_IN_TUXEDO_TONE2),
    ((19, "man_in_tuxedo_tone2"), MAN_IN_TUXEDO_TONE2),
    ((22, "person_in_tuxedo_tone2"), PERSON_IN_TUXEDO_TONE2),
    ((21, "woman_in_tuxedo_tone3"), WOMAN_IN_TUXEDO_TONE3),
    ((19, "man_in_tuxedo_tone3"), MAN_IN_TUXEDO_TONE3),
    ((22, "person_in_tuxedo_tone3"), PERSON_IN_TUXEDO_TONE3),
    ((21, "woman_in_tuxedo_tone4"), WOMAN_IN_TUXEDO_TONE4),
    ((19, "man_in_tuxedo_tone4"), MAN_IN_TUXEDO_TONE4),
    ((22, "person_in_tuxedo_tone4"), PERSON_IN_TUXEDO_TONE4),
    ((21, "woman_in_tuxedo_tone5"), WOMAN_IN_TUXEDO_TONE5),
    ((19, "man_in_tuxedo_tone5"), MAN_IN_TUXEDO_TONE5),
    ((22, "person_in_tuxedo_tone5"), PERSON_IN_TUXEDO_TONE5),
    ((15, "woman_in_tuxedo"), WOMAN_IN_TUXEDO),
    ((13, "man_in_tuxedo"), MAN_IN_TUXEDO),
    ((16, "person_in_tuxedo"), PERSON_IN_TUXEDO),
    ((15, "mrs_claus_tone1"), MRS_CLAUS_TONE1),
    ((15, "mrs_claus_tone2"), MRS_CLAUS_TONE2),
    ((15, "mrs_claus_tone3"), MRS_CLAUS_TONE3),
    ((15, "mrs_claus_tone4"), MRS_CLAUS_TONE4),
    ((15, "mrs_claus_tone5"), MRS_CLAUS_TONE5),
    ((9, "mrs_claus"), MRS_CLAUS),
    ((21, "woman_shrugging_tone1"), WOMAN_SHRUGGING_TONE1),
    ((19, "man_shrugging_tone1"), MAN_SHRUGGING_TONE1),
    ((22, "person_shrugging_tone1"), PERSON_SHRUGGING_TONE1),
    ((11, "shrug_tone1"), SHRUG_TONE1),
    ((21, "woman_shrugging_tone2"), WOMAN_SHRUGGING_TONE2),
    ((19, "man_shrugging_tone2"), MAN_SHRUGGING_TONE2),
    ((22, "person_shrugging_tone2"), PERSON_SHRUGGING_TONE2),
    ((11, "shrug_tone2"), SHRUG_TONE2),
    ((21, "woman_shrugging_tone3"), WOMAN_SHRUGGING_TONE3),
    ((19, "man_shrugging_tone3"), MAN_SHRUGGING_TONE3),
    ((22, "person_shrugging_tone3"), PERSON_SHRUGGING_TONE3),
    ((11, "shrug_tone3"), SHRUG_TONE3),
    ((21, "woman_shrugging_tone4"), WOMAN_SHRUGGING_TONE4),
    ((19, "man_shrugging_tone4"), MAN_SHRUGGING_TONE4),
    ((22, "person_shrugging_tone4"), PERSON_SHRUGGING_TONE4),
    ((11, "shrug_tone4"), SHRUG_TONE4),
    ((21, "woman_shrugging_tone5"), WOMAN_SHRUGGING_TONE5),
    ((19, "man_shrugging_tone5"), MAN_SHRUGGING_TONE5),
    ((22, "person_shrugging_tone5"), PERSON_SHRUGGING_TONE5),
    ((11, "shrug_tone5"), SHRUG_TONE5),
    ((15, "woman_shrugging"), WOMAN_SHRUGGING),
    ((13, "man_shrugging"), MAN_SHRUGGING),
    ((16, "person_shrugging"), PERSON_SHRUGGING),
    ((5, "shrug"), SHRUG),
    ((24, "woman_cartwheeling_tone1"), WOMAN_CARTWHEELING_TONE1),
    ((22, "man_cartwheeling_tone1"), MAN_CARTWHEELING_TONE1),
    ((18, "cartwheeling_tone1"), CARTWHEELING_TONE1),
    ((22, "person_cartwheel_tone1"), PERSON_CARTWHEEL_TONE1),
    ((24, "woman_cartwheeling_tone2"), WOMAN_CARTWHEELING_TONE2),
    ((22, "man_cartwheeling_tone2"), MAN_CARTWHEELING_TONE2),
    ((18, "cartwheeling_tone2"), CARTWHEELING_TONE2),
    ((22, "person_cartwheel_tone2"), PERSON_CARTWHEEL_TONE2),
    ((24, "woman_cartwheeling_tone3"), WOMAN_CARTWHEELING_TONE3),
    ((22, "man_cartwheeling_tone3"), MAN_CARTWHEELING_TONE3),
    ((18, "cartwheeling_tone3"), CARTWHEELING_TONE3),
    ((22, "person_cartwheel_tone3"), PERSON_CARTWHEEL_TONE3),
    ((24, "woman_cartwheeling_tone4"), WOMAN_CARTWHEELING_TONE4),
    ((22, "man_cartwheeling_tone4"), MAN_CARTWHEELING_TONE4),
    ((18, "cartwheeling_tone4"), CARTWHEELING_TONE4),
    ((22, "person_cartwheel_tone4"), PERSON_CARTWHEEL_TONE4),
    ((24, "woman_cartwheeling_tone5"), WOMAN_CARTWHEELING_TONE5),
    ((22, "man_cartwheeling_tone5"), MAN_CARTWHEELING_TONE5),
    ((18, "cartwheeling_tone5"), CARTWHEELING_TONE5),
    ((22, "person_cartwheel_tone5"), PERSON_CARTWHEEL_TONE5),
    ((18, "woman_cartwheeling"), WOMAN_CARTWHEELING),
    ((16, "man_cartwheeling"), MAN_CARTWHEELING),
    ((12, "cartwheeling"), CARTWHEELING),
    ((16, "person_cartwheel"), PERSON_CARTWHEEL),
    ((20, "woman_juggling_tone1"), WOMAN_JUGGLING_TONE1),
    ((18, "man_juggling_tone1"), MAN_JUGGLING_TONE1),
    ((13, "juggler_tone1"), JUGGLER_TONE1),
    ((14, "juggling_tone1"), JUGGLING_TONE1),
    ((21, "person_juggling_tone1"), PERSON_JUGGLING_TONE1),
    ((20, "woman_juggling_tone2"), WOMAN_JUGGLING_TONE2),
    ((18, "man_juggling_tone2"), MAN_JUGGLING_TONE2),
    ((13, "juggler_tone2"), JUGGLER_TONE2),
    ((14, "juggling_tone2"), JUGGLING_TONE2),
    ((21, "person_juggling_tone2"), PERSON_JUGGLING_TONE2),
    ((20, "woman_juggling_tone3"), WOMAN_JUGGLING_TONE3),
    ((18, "man_juggling_tone3"), MAN_JUGGLING_TONE3),
    ((13, "juggler_tone3"), JUGGLER_TONE3),
    ((14, "juggling_tone3"), JUGGLING_TONE3),
    ((21, "person_juggling_tone3"), PERSON_JUGGLING_TONE3),
    ((20, "woman_juggling_tone4"), WOMAN_JUGGLING_TONE4),
    ((18, "man_juggling_tone4"), MAN_JUGGLING_TONE4),
    ((13, "juggler_tone4"), JUGGLER_TONE4),
    ((14, "juggling_tone4"), JUGGLING_TONE4),
    ((21, "person_juggling_tone4"), PERSON_JUGGLING_TONE4),
    ((20, "woman_juggling_tone5"), WOMAN_JUGGLING_TONE5),
    ((18, "man_juggling_tone5"), MAN_JUGGLING_TONE5),
    ((13, "juggler_tone5"), JUGGLER_TONE5),
    ((14, "juggling_tone5"), JUGGLING_TONE5),
    ((21, "person_juggling_tone5"), PERSON_JUGGLING_TONE5),
    ((14, "woman_juggling"), WOMAN_JUGGLING),
    ((12, "man_juggling"), MAN_JUGGLING),
    ((7, "juggler"), JUGGLER),
    ((8, "juggling"), JUGGLING),
    ((15, "person_juggling"), PERSON_JUGGLING),
    ((6, "fencer"), FENCER),
    ((7, "fencing"), FENCING),
    ((14, "person_fencing"), PERSON_FENCING),
    ((15, "women_wrestling"), WOMEN_WRESTLING),
    ((13, "men_wrestling"), MEN_WRESTLING),
    ((16, "people_wrestling"), PEOPLE_WRESTLING),
    ((9, "wrestlers"), WRESTLERS),
    ((9, "wrestling"), WRESTLING),
    ((30, "woman_playing_water_polo_tone1"), WOMAN_PLAYING_WATER_POLO_TONE1),
    ((28, "man_playing_water_polo_tone1"), MAN_PLAYING_WATER_POLO_TONE1),
    ((31, "person_playing_water_polo_tone1"), PERSON_PLAYING_WATER_POLO_TONE1),
    ((16, "water_polo_tone1"), WATER_POLO_TONE1),
    ((30, "woman_playing_water_polo_tone2"), WOMAN_PLAYING_WATER_POLO_TONE2),
    ((28, "man_playing_water_polo_tone2"), MAN_PLAYING_WATER_POLO_TONE2),
    ((31, "person_playing_water_polo_tone2"), PERSON_PLAYING_WATER_POLO_TONE2),
    ((16, "water_polo_tone2"), WATER_POLO_TONE2),
    ((30, "woman_playing_water_polo_tone3"), WOMAN_PLAYING_WATER_POLO_TONE3),
    ((28, "man_playing_water_polo_tone3"), MAN_PLAYING_WATER_POLO_TONE3),
    ((31, "person_playing_water_polo_tone3"), PERSON_PLAYING_WATER_POLO_TONE3),
    ((16, "water_polo_tone3"), WATER_POLO_TONE3),
    ((30, "woman_playing_water_polo_tone4"), WOMAN_PLAYING_WATER_POLO_TONE4),
    ((28, "man_playing_water_polo_tone4"), MAN_PLAYING_WATER_POLO_TONE4),
    ((31, "person_playing_water_polo_tone4"), PERSON_PLAYING_WATER_POLO_TONE4),
    ((16, "water_polo_tone4"), WATER_POLO_TONE4),
    ((30, "woman_playing_water_polo_tone5"), WOMAN_PLAYING_WATER_POLO_TONE5),
    ((28, "man_playing_water_polo_tone5"), MAN_PLAYING_WATER_POLO_TONE5),
    ((31, "person_playing_water_polo_tone5"), PERSON_PLAYING_WATER_POLO_TONE5),
    ((16, "water_polo_tone5"), WATER_POLO_TONE5),
    ((24, "woman_playing_water_polo"), WOMAN_PLAYING_WATER_POLO),
    ((22, "man_playing_water_polo"), MAN_PLAYING_WATER_POLO),
    ((25, "person_playing_water_polo"), PERSON_PLAYING_WATER_POLO),
    ((10, "water_polo"), WATER_POLO),
    ((28, "woman_playing_handball_tone1"), WOMAN_PLAYING_HANDBALL_TONE1),
    ((26, "man_playing_handball_tone1"), MAN_PLAYING_HANDBALL_TONE1),
    ((14, "handball_tone1"), HANDBALL_TONE1),
    ((29, "person_playing_handball_tone1"), PERSON_PLAYING_HANDBALL_TONE1),
    ((28, "woman_playing_handball_tone2"), WOMAN_PLAYING_HANDBALL_TONE2),
    ((26, "man_playing_handball_tone2"), MAN_PLAYING_HANDBALL_TONE2),
    ((14, "handball_tone2"), HANDBALL_TONE2),
    ((29, "person_playing_handball_tone2"), PERSON_PLAYING_HANDBALL_TONE2),
    ((28, "woman_playing_handball_tone3"), WOMAN_PLAYING_HANDBALL_TONE3),
    ((26, "man_playing_handball_tone3"), MAN_PLAYING_HANDBALL_TONE3),
    ((14, "handball_tone3"), HANDBALL_TONE3),
    ((29, "person_playing_handball_tone3"), PERSON_PLAYING_HANDBALL_TONE3),
    ((28, "woman_playing_handball_tone4"), WOMAN_PLAYING_HANDBALL_TONE4),
    ((26, "man_playing_handball_tone4"), MAN_PLAYING_HANDBALL_TONE4),
    ((14, "handball_tone4"), HANDBALL_TONE4),
    ((29, "person_playing_handball_tone4"), PERSON_PLAYING_HANDBALL_TONE4),
    ((28, "woman_playing_handball_tone5"), WOMAN_PLAYING_HANDBALL_TONE5),
    ((26, "man_playing_handball_tone5"), MAN_PLAYING_HANDBALL_TONE5),
    ((14, "handball_tone5"), HANDBALL_TONE5),
    ((29, "person_playing_handball_tone5"), PERSON_PLAYING_HANDBALL_TONE5),
    ((22, "woman_playing_handball"), WOMAN_PLAYING_HANDBALL),
    ((20, "man_playing_handball"), MAN_PLAYING_HANDBALL),
    ((8, "handball"), HANDBALL),
    ((23, "person_playing_handball"), PERSON_PLAYING_HANDBALL),
    ((11, "diving_mask"), DIVING_MASK),
    ((13, "wilted_flower"), WILTED_FLOWER),
    ((4, "drum"), DRUM),
    ((16, "clinking_glasses"), CLINKING_GLASSES),
    ((13, "tumbler_glass"), TUMBLER_GLASS),
    ((6, "whisky"), WHISKY),
    ((5, "spoon"), SPOON),
    ((8, "goal_net"), GOAL_NET),
    ((3, "1st"), X_1ST),
    ((17, "first_place_medal"), FIRST_PLACE_MEDAL),
    ((3, "2nd"), X_2ND),
    ((18, "second_place_medal"), SECOND_PLACE_MEDAL),
    ((3, "3rd"), X_3RD),
    ((17, "third_place_medal"), THIRD_PLACE_MEDAL),
    ((12, "boxing_glove"), BOXING_GLOVE),
    ((20, "martial_arts_uniform"), MARTIAL_ARTS_UNIFORM),
    ((13, "curling_stone"), CURLING_STONE),
    ((8, "lacrosse"), LACROSSE),
    ((8, "softball"), SOFTBALL),
    ((11, "flying_disc"), FLYING_DISC),
    ((9, "croissant"), CROISSANT),
    ((7, "avocado"), AVOCADO),
    ((8, "cucumber"), CUCUMBER),
    ((5, "bacon"), BACON),
    ((6, "potato"), POTATO),
    ((6, "carrot"), CARROT),
    ((14, "baguette_bread"), BAGUETTE_BREAD),
    ((11, "green_salad"), GREEN_SALAD),
    ((5, "salad"), SALAD),
    ((19, "shallow_pan_of_food"), SHALLOW_PAN_OF_FOOD),
    ((17, "stuffed_flatbread"), STUFFED_FLATBREAD),
    ((3, "egg"), EGG),
    ((13, "glass_of_milk"), GLASS_OF_MILK),
    ((4, "milk"), MILK),
    ((7, "peanuts"), PEANUTS),
    ((4, "kiwi"), KIWI),
    ((8, "pancakes"), PANCAKES),
    ((8, "dumpling"), DUMPLING),
    ((14, "fortune_cookie"), FORTUNE_COOKIE),
    ((11, "takeout_box"), TAKEOUT_BOX),
    ((10, "chopsticks"), CHOPSTICKS),
    ((15, "bowl_with_spoon"), BOWL_WITH_SPOON),
    ((14, "cup_with_straw"), CUP_WITH_STRAW),
    ((7, "coconut"), COCONUT),
    ((8, "broccoli"), BROCCOLI),
    ((3, "pie"), PIE),
    ((7, "pretzel"), PRETZEL),
    ((11, "cut_of_meat"), CUT_OF_MEAT),
    ((8, "sandwich"), SANDWICH),
    ((11, "canned_food"), CANNED_FOOD),
    ((11, "leafy_green"), LEAFY_GREEN),
    ((5, "mango"), MANGO),
    ((9, "moon_cake"), MOON_CAKE),
    ((5, "bagel"), BAGEL),
    ((26, "smiling_face_with_3_hearts"), SMILING_FACE_WITH_3_HEARTS),
    ((4, "yawn"), YAWN),
    ((7, "yawning"), YAWNING),
    ((12, "yawning_face"), YAWNING_FACE),
    ((22, "smiling_face_with_tear"), SMILING_FACE_WITH_TEAR),
    ((6, "hooray"), HOORAY),
    ((8, "partying"), PARTYING),
    ((13, "partying_face"), PARTYING_FACE),
    ((5, "woozy"), WOOZY),
    ((10, "woozy_face"), WOOZY_FACE),
    ((3, "hot"), HOT),
    ((8, "hot_face"), HOT_FACE),
    ((4, "cold"), COLD),
    ((9, "cold_face"), COLD_FACE),
    ((11, "ninja_tone1"), NINJA_TONE1),
    ((11, "ninja_tone2"), NINJA_TONE2),
    ((11, "ninja_tone3"), NINJA_TONE3),
    ((11, "ninja_tone4"), NINJA_TONE4),
    ((11, "ninja_tone5"), NINJA_TONE5),
    ((5, "ninja"), NINJA),
    ((9, "disguised"), DISGUISED),
    ((14, "disguised_face"), DISGUISED_FACE),
    ((23, "face_holding_back_tears"), FACE_HOLDING_BACK_TEARS),
    ((11, "watery_eyes"), WATERY_EYES),
    ((8, "pleading"), PLEADING),
    ((13, "pleading_face"), PLEADING_FACE),
    ((4, "sari"), SARI),
    ((8, "lab_coat"), LAB_COAT),
    ((7, "goggles"), GOGGLES),
    ((11, "hiking_boot"), HIKING_BOOT),
    ((9, "flat_shoe"), FLAT_SHOE),
    ((16, "womans_flat_shoe"), WOMANS_FLAT_SHOE),
    ((4, "crab"), CRAB),
    ((4, "lion"), LION),
    ((9, "lion_face"), LION_FACE),
    ((8, "scorpion"), SCORPION),
    ((6, "turkey"), TURKEY),
    ((7, "unicorn"), UNICORN),
    ((12, "unicorn_face"), UNICORN_FACE),
    ((5, "eagle"), EAGLE),
    ((4, "duck"), DUCK),
    ((3, "bat"), BAT),
    ((5, "shark"), SHARK),
    ((3, "owl"), OWL),
    ((3, "fox"), FOX),
    ((8, "fox_face"), FOX_FACE),
    ((9, "butterfly"), BUTTERFLY),
    ((4, "deer"), DEER),
    ((7, "gorilla"), GORILLA),
    ((6, "lizard"), LIZARD),
    ((5, "rhino"), RHINO),
    ((10, "rhinoceros"), RHINOCEROS),
    ((6, "shrimp"), SHRIMP),
    ((5, "squid"), SQUID),
    ((7, "giraffe"), GIRAFFE),
    ((5, "zebra"), ZEBRA),
    ((8, "hedgehog"), HEDGEHOG),
    ((8, "sauropod"), SAUROPOD),
    ((5, "t-rex"), T_REX),
    ((4, "trex"), TREX),
    ((7, "cricket"), CRICKET),
    ((8, "kangaroo"), KANGAROO),
    ((5, "llama"), LLAMA),
    ((7, "peacock"), PEACOCK),
    ((5, "hippo"), HIPPO),
    ((6, "parrot"), PARROT),
    ((7, "raccoon"), RACCOON),
    ((7, "lobster"), LOBSTER),
    ((8, "mosquito"), MOSQUITO),
    ((7, "microbe"), MICROBE),
    ((6, "badger"), BADGER),
    ((4, "swan"), SWAN),
    ((7, "mammoth"), MAMMOTH),
    ((4, "dodo"), DODO),
    ((5, "sloth"), SLOTH),
    ((5, "otter"), OTTER),
    ((9, "orangutan"), ORANGUTAN),
    ((5, "skunk"), SKUNK),
    ((8, "flamingo"), FLAMINGO),
    ((6, "oyster"), OYSTER),
    ((6, "beaver"), BEAVER),
    ((5, "bison"), BISON),
    ((4, "seal"), SEAL),
    ((9, "guide_dog"), GUIDE_DOG),
    ((12, "probing_cane"), PROBING_CANE),
    ((10, "white_cane"), WHITE_CANE),
    ((8, "red_hair"), RED_HAIR),
    ((10, "curly_hair"), CURLY_HAIR),
    ((7, "no_hair"), NO_HAIR),
    ((10, "white_hair"), WHITE_HAIR),
    ((4, "bone"), BONE),
    ((9, "leg_tone1"), LEG_TONE1),
    ((9, "leg_tone2"), LEG_TONE2),
    ((9, "leg_tone3"), LEG_TONE3),
    ((9, "leg_tone4"), LEG_TONE4),
    ((9, "leg_tone5"), LEG_TONE5),
    ((3, "leg"), LEG),
    ((10, "foot_tone1"), FOOT_TONE1),
    ((10, "foot_tone2"), FOOT_TONE2),
    ((10, "foot_tone3"), FOOT_TONE3),
    ((10, "foot_tone4"), FOOT_TONE4),
    ((10, "foot_tone5"), FOOT_TONE5),
    ((4, "foot"), FOOT),
    ((5, "tooth"), TOOTH),
    ((21, "woman_superhero_tone1"), WOMAN_SUPERHERO_TONE1),
    ((19, "man_superhero_tone1"), MAN_SUPERHERO_TONE1),
    ((15, "superhero_tone1"), SUPERHERO_TONE1),
    ((21, "woman_superhero_tone2"), WOMAN_SUPERHERO_TONE2),
    ((19, "man_superhero_tone2"), MAN_SUPERHERO_TONE2),
    ((15, "superhero_tone2"), SUPERHERO_TONE2),
    ((21, "woman_superhero_tone3"), WOMAN_SUPERHERO_TONE3),
    ((19, "man_superhero_tone3"), MAN_SUPERHERO_TONE3),
    ((15, "superhero_tone3"), SUPERHERO_TONE3),
    ((21, "woman_superhero_tone4"), WOMAN_SUPERHERO_TONE4),
    ((19, "man_superhero_tone4"), MAN_SUPERHERO_TONE4),
    ((15, "superhero_tone4"), SUPERHERO_TONE4),
    ((21, "woman_superhero_tone5"), WOMAN_SUPERHERO_TONE5),
    ((19, "man_superhero_tone5"), MAN_SUPERHERO_TONE5),
    ((15, "superhero_tone5"), SUPERHERO_TONE5),
    ((15, "woman_superhero"), WOMAN_SUPERHERO),
    ((13, "man_superhero"), MAN_SUPERHERO),
    ((9, "superhero"), SUPERHERO),
    ((24, "woman_supervillain_tone1"), WOMAN_SUPERVILLAIN_TONE1),
    ((22, "man_supervillain_tone1"), MAN_SUPERVILLAIN_TONE1),
    ((18, "supervillain_tone1"), SUPERVILLAIN_TONE1),
    ((24, "woman_supervillain_tone2"), WOMAN_SUPERVILLAIN_TONE2),
    ((22, "man_supervillain_tone2"), MAN_SUPERVILLAIN_TONE2),
    ((18, "supervillain_tone2"), SUPERVILLAIN_TONE2),
    ((24, "woman_supervillain_tone3"), WOMAN_SUPERVILLAIN_TONE3),
    ((22, "man_supervillain_tone3"), MAN_SUPERVILLAIN_TONE3),
    ((18, "supervillain_tone3"), SUPERVILLAIN_TONE3),
    ((24, "woman_supervillain_tone4"), WOMAN_SUPERVILLAIN_TONE4),
    ((22, "man_supervillain_tone4"), MAN_SUPERVILLAIN_TONE4),
    ((18, "supervillain_tone4"), SUPERVILLAIN_TONE4),
    ((24, "woman_supervillain_tone5"), WOMAN_SUPERVILLAIN_TONE5),
    ((22, "man_supervillain_tone5"), MAN_SUPERVILLAIN_TONE5),
    ((18, "supervillain_tone5"), SUPERVILLAIN_TONE5),
    ((18, "woman_supervillain"), WOMAN_SUPERVILLAIN),
    ((16, "man_supervillain"), MAN_SUPERVILLAIN),
    ((12, "supervillain"), SUPERVILLAIN),
    ((11, "safety_vest"), SAFETY_VEST),
    ((26, "ear_with_hearing_aid_tone1"), EAR_WITH_HEARING_AID_TONE1),
    ((17, "hearing_aid_tone1"), HEARING_AID_TONE1),
    ((26, "ear_with_hearing_aid_tone2"), EAR_WITH_HEARING_AID_TONE2),
    ((17, "hearing_aid_tone2"), HEARING_AID_TONE2),
    ((26, "ear_with_hearing_aid_tone3"), EAR_WITH_HEARING_AID_TONE3),
    ((17, "hearing_aid_tone3"), HEARING_AID_TONE3),
    ((26, "ear_with_hearing_aid_tone4"), EAR_WITH_HEARING_AID_TONE4),
    ((17, "hearing_aid_tone4"), HEARING_AID_TONE4),
    ((26, "ear_with_hearing_aid_tone5"), EAR_WITH_HEARING_AID_TONE5),
    ((17, "hearing_aid_tone5"), HEARING_AID_TONE5),
    ((20, "ear_with_hearing_aid"), EAR_WITH_HEARING_AID),
    ((11, "hearing_aid"), HEARING_AID),
    ((20, "motorized_wheelchair"), MOTORIZED_WHEELCHAIR),
    ((17, "manual_wheelchair"), MANUAL_WHEELCHAIR),
    ((14, "mechanical_arm"), MECHANICAL_ARM),
    ((14, "mechanical_leg"), MECHANICAL_LEG),
    ((6, "cheese"), CHEESE),
    ((7, "cupcake"), CUPCAKE),
    ((4, "salt"), SALT),
    ((12, "beverage_box"), BEVERAGE_BOX),
    ((9, "juice_box"), JUICE_BOX),
    ((6, "garlic"), GARLIC),
    ((5, "onion"), ONION),
    ((7, "falafel"), FALAFEL),
    ((6, "waffle"), WAFFLE),
    ((6, "butter"), BUTTER),
    ((4, "mate"), MATE),
    ((3, "ice"), ICE),
    ((8, "ice_cube"), ICE_CUBE),
    ((10, "boba_drink"), BOBA_DRINK),
    ((10, "bubble_tea"), BUBBLE_TEA),
    ((5, "troll"), TROLL),
    ((20, "woman_standing_tone1"), WOMAN_STANDING_TONE1),
    ((18, "man_standing_tone1"), MAN_STANDING_TONE1),
    ((21, "person_standing_tone1"), PERSON_STANDING_TONE1),
    ((14, "standing_tone1"), STANDING_TONE1),
    ((20, "woman_standing_tone2"), WOMAN_STANDING_TONE2),
    ((18, "man_standing_tone2"), MAN_STANDING_TONE2),
    ((21, "person_standing_tone2"), PERSON_STANDING_TONE2),
    ((14, "standing_tone2"), STANDING_TONE2),
    ((20, "woman_standing_tone3"), WOMAN_STANDING_TONE3),
    ((18, "man_standing_tone3"), MAN_STANDING_TONE3),
    ((21, "person_standing_tone3"), PERSON_STANDING_TONE3),
    ((14, "standing_tone3"), STANDING_TONE3),
    ((20, "woman_standing_tone4"), WOMAN_STANDING_TONE4),
    ((18, "man_standing_tone4"), MAN_STANDING_TONE4),
    ((21, "person_standing_tone4"), PERSON_STANDING_TONE4),
    ((14, "standing_tone4"), STANDING_TONE4),
    ((20, "woman_standing_tone5"), WOMAN_STANDING_TONE5),
    ((18, "man_standing_tone5"), MAN_STANDING_TONE5),
    ((21, "person_standing_tone5"), PERSON_STANDING_TONE5),
    ((14, "standing_tone5"), STANDING_TONE5),
    ((14, "woman_standing"), WOMAN_STANDING),
    ((12, "man_standing"), MAN_STANDING),
    ((15, "person_standing"), PERSON_STANDING),
    ((8, "standing"), STANDING),
    ((20, "woman_kneeling_tone1"), WOMAN_KNEELING_TONE1),
    ((18, "man_kneeling_tone1"), MAN_KNEELING_TONE1),
    ((14, "kneeling_tone1"), KNEELING_TONE1),
    ((21, "person_kneeling_tone1"), PERSON_KNEELING_TONE1),
    ((20, "woman_kneeling_tone2"), WOMAN_KNEELING_TONE2),
    ((18, "man_kneeling_tone2"), MAN_KNEELING_TONE2),
    ((14, "kneeling_tone2"), KNEELING_TONE2),
    ((21, "person_kneeling_tone2"), PERSON_KNEELING_TONE2),
    ((20, "woman_kneeling_tone3"), WOMAN_KNEELING_TONE3),
    ((18, "man_kneeling_tone3"), MAN_KNEELING_TONE3),
    ((14, "kneeling_tone3"), KNEELING_TONE3),
    ((21, "person_kneeling_tone3"), PERSON_KNEELING_TONE3),
    ((20, "woman_kneeling_tone4"), WOMAN_KNEELING_TONE4),
    ((18, "man_kneeling_tone4"), MAN_KNEELING_TONE4),
    ((14, "kneeling_tone4"), KNEELING_TONE4),
    ((21, "person_kneeling_tone4"), PERSON_KNEELING_TONE4),
    ((20, "woman_kneeling_tone5"), WOMAN_KNEELING_TONE5),
    ((18, "man_kneeling_tone5"), MAN_KNEELING_TONE5),
    ((14, "kneeling_tone5"), KNEELING_TONE5),
    ((21, "person_kneeling_tone5"), PERSON_KNEELING_TONE5),
    ((14, "woman_kneeling"), WOMAN_KNEELING),
    ((12, "man_kneeling"), MAN_KNEELING),
    ((8, "kneeling"), KNEELING),
    ((15, "person_kneeling"), PERSON_KNEELING),
    ((16, "deaf_woman_tone1"), DEAF_WOMAN_TONE1),
    ((14, "deaf_man_tone1"), DEAF_MAN_TONE1),
    ((17, "deaf_person_tone1"), DEAF_PERSON_TONE1),
    ((16, "deaf_woman_tone2"), DEAF_WOMAN_TONE2),
    ((14, "deaf_man_tone2"), DEAF_MAN_TONE2),
    ((17, "deaf_person_tone2"), DEAF_PERSON_TONE2),
    ((16, "deaf_woman_tone3"), DEAF_WOMAN_TONE3),
    ((14, "deaf_man_tone3"), DEAF_MAN_TONE3),
    ((17, "deaf_person_tone3"), DEAF_PERSON_TONE3),
    ((16, "deaf_woman_tone4"), DEAF_WOMAN_TONE4),
    ((14, "deaf_man_tone4"), DEAF_MAN_TONE4),
    ((17, "deaf_person_tone4"), DEAF_PERSON_TONE4),
    ((16, "deaf_woman_tone5"), DEAF_WOMAN_TONE5),
    ((14, "deaf_man_tone5"), DEAF_MAN_TONE5),
    ((17, "deaf_person_tone5"), DEAF_PERSON_TONE5),
    ((10, "deaf_woman"), DEAF_WOMAN),
    ((8, "deaf_man"), DEAF_MAN),
    ((11, "deaf_person"), DEAF_PERSON),
    ((17, "face_with_monocle"), FACE_WITH_MONOCLE),
    ((12, "farmer_tone1"), FARMER_TONE1),
    ((10, "cook_tone1"), COOK_TONE1),
    ((25, "person_feeding_baby_tone1"), PERSON_FEEDING_BABY_TONE1),
    ((14, "mx_claus_tone1"), MX_CLAUS_TONE1),
    ((13, "student_tone1"), STUDENT_TONE1),
    ((12, "singer_tone1"), SINGER_TONE1),
    ((12, "artist_tone1"), ARTIST_TONE1),
    ((13, "teacher_tone1"), TEACHER_TONE1),
    ((20, "factory_worker_tone1"), FACTORY_WORKER_TONE1),
    ((18, "technologist_tone1"), TECHNOLOGIST_TONE1),
    ((19, "office_worker_tone1"), OFFICE_WORKER_TONE1),
    ((14, "mechanic_tone1"), MECHANIC_TONE1),
    ((15, "scientist_tone1"), SCIENTIST_TONE1),
    ((15, "astronaut_tone1"), ASTRONAUT_TONE1),
    ((17, "firefighter_tone1"), FIREFIGHTER_TONE1),
    ((26, "people_holding_hands_tone1"), PEOPLE_HOLDING_HANDS_TONE1),
    ((28, "people_holding_hands_tone1-2"), PEOPLE_HOLDING_HANDS_TONE1_2),
    ((28, "people_holding_hands_tone1-3"), PEOPLE_HOLDING_HANDS_TONE1_3),
    ((28, "people_holding_hands_tone1-4"), PEOPLE_HOLDING_HANDS_TONE1_4),
    ((28, "people_holding_hands_tone1-5"), PEOPLE_HOLDING_HANDS_TONE1_5),
    ((30, "person_with_probing_cane_tone1"), PERSON_WITH_PROBING_CANE_TONE1),
    ((28, "person_with_white_cane_tone1"), PERSON_WITH_WHITE_CANE_TONE1),
    ((16, "red_haired_tone1"), RED_HAIRED_TONE1),
    ((18, "curly_haired_tone1"), CURLY_HAIRED_TONE1),
    ((10, "bald_tone1"), BALD_TONE1),
    ((18, "white_haired_tone1"), WHITE_HAIRED_TONE1),
    ((36, "person_in_motorized_wheelchair_tone1"), PERSON_IN_MOTORIZED_WHEELCHAIR_TONE1),
    ((33, "person_in_manual_wheelchair_tone1"), PERSON_IN_MANUAL_WHEELCHAIR_TONE1),
    ((19, "health_worker_tone1"), HEALTH_WORKER_TONE1),
    ((11, "judge_tone1"), JUDGE_TONE1),
    ((11, "pilot_tone1"), PILOT_TONE1),
    ((19, "couple_kiss_tone1-2"), COUPLE_KISS_TONE1_2),
    ((18, "couplekiss_tone1-2"), COUPLEKISS_TONE1_2),
    ((19, "couple_kiss_tone1-3"), COUPLE_KISS_TONE1_3),
    ((18, "couplekiss_tone1-3"), COUPLEKISS_TONE1_3),
    ((19, "couple_kiss_tone1-4"), COUPLE_KISS_TONE1_4),
    ((18, "couplekiss_tone1-4"), COUPLEKISS_TONE1_4),
    ((19, "couple_kiss_tone1-5"), COUPLE_KISS_TONE1_5),
    ((18, "couplekiss_tone1-5"), COUPLEKISS_TONE1_5),
    ((25, "couple_with_heart_tone1-2"), COUPLE_WITH_HEART_TONE1_2),
    ((25, "couple_with_heart_tone1-3"), COUPLE_WITH_HEART_TONE1_3),
    ((25, "couple_with_heart_tone1-4"), COUPLE_WITH_HEART_TONE1_4),
    ((25, "couple_with_heart_tone1-5"), COUPLE_WITH_HEART_TONE1_5),
    ((11, "adult_tone1"), ADULT_TONE1),
    ((12, "farmer_tone2"), FARMER_TONE2),
    ((10, "cook_tone2"), COOK_TONE2),
    ((25, "person_feeding_baby_tone2"), PERSON_FEEDING_BABY_TONE2),
    ((14, "mx_claus_tone2"), MX_CLAUS_TONE2),
    ((13, "student_tone2"), STUDENT_TONE2),
    ((12, "singer_tone2"), SINGER_TONE2),
    ((12, "artist_tone2"), ARTIST_TONE2),
    ((13, "teacher_tone2"), TEACHER_TONE2),
    ((20, "factory_worker_tone2"), FACTORY_WORKER_TONE2),
    ((18, "technologist_tone2"), TECHNOLOGIST_TONE2),
    ((19, "office_worker_tone2"), OFFICE_WORKER_TONE2),
    ((14, "mechanic_tone2"), MECHANIC_TONE2),
    ((15, "scientist_tone2"), SCIENTIST_TONE2),
    ((15, "astronaut_tone2"), ASTRONAUT_TONE2),
    ((17, "firefighter_tone2"), FIREFIGHTER_TONE2),
    ((28, "people_holding_hands_tone2-1"), PEOPLE_HOLDING_HANDS_TONE2_1),
    ((26, "people_holding_hands_tone2"), PEOPLE_HOLDING_HANDS_TONE2),
    ((28, "people_holding_hands_tone2-3"), PEOPLE_HOLDING_HANDS_TONE2_3),
    ((28, "people_holding_hands_tone2-4"), PEOPLE_HOLDING_HANDS_TONE2_4),
    ((28, "people_holding_hands_tone2-5"), PEOPLE_HOLDING_HANDS_TONE2_5),
    ((30, "person_with_probing_cane_tone2"), PERSON_WITH_PROBING_CANE_TONE2),
    ((28, "person_with_white_cane_tone2"), PERSON_WITH_WHITE_CANE_TONE2),
    ((16, "red_haired_tone2"), RED_HAIRED_TONE2),
    ((18, "curly_haired_tone2"), CURLY_HAIRED_TONE2),
    ((10, "bald_tone2"), BALD_TONE2),
    ((18, "white_haired_tone2"), WHITE_HAIRED_TONE2),
    ((36, "person_in_motorized_wheelchair_tone2"), PERSON_IN_MOTORIZED_WHEELCHAIR_TONE2),
    ((33, "person_in_manual_wheelchair_tone2"), PERSON_IN_MANUAL_WHEELCHAIR_TONE2),
    ((19, "health_worker_tone2"), HEALTH_WORKER_TONE2),
    ((11, "judge_tone2"), JUDGE_TONE2),
    ((11, "pilot_tone2"), PILOT_TONE2),
    ((19, "couple_kiss_tone2-1"), COUPLE_KISS_TONE2_1),
    ((18, "couplekiss_tone2-1"), COUPLEKISS_TONE2_1),
    ((19, "couple_kiss_tone2-3"), COUPLE_KISS_TONE2_3),
    ((18, "couplekiss_tone2-3"), COUPLEKISS_TONE2_3),
    ((19, "couple_kiss_tone2-4"), COUPLE_KISS_TONE2_4),
    ((18, "couplekiss_tone2-4"), COUPLEKISS_TONE2_4),
    ((19, "couple_kiss_tone2-5"), COUPLE_KISS_TONE2_5),
    ((18, "couplekiss_tone2-5"), COUPLEKISS_TONE2_5),
    ((25, "couple_with_heart_tone2-1"), COUPLE_WITH_HEART_TONE2_1),
    ((25, "couple_with_heart_tone2-3"), COUPLE_WITH_HEART_TONE2_3),
    ((25, "couple_with_heart_tone2-4"), COUPLE_WITH_HEART_TONE2_4),
    ((25, "couple_with_heart_tone2-5"), COUPLE_WITH_HEART_TONE2_5),
    ((11, "adult_tone2"), ADULT_TONE2),
    ((12, "farmer_tone3"), FARMER_TONE3),
    ((10, "cook_tone3"), COOK_TONE3),
    ((25, "person_feeding_baby_tone3"), PERSON_FEEDING_BABY_TONE3),
    ((14, "mx_claus_tone3"), MX_CLAUS_TONE3),
    ((13, "student_tone3"), STUDENT_TONE3),
    ((12, "singer_tone3"), SINGER_TONE3),
    ((12, "artist_tone3"), ARTIST_TONE3),
    ((13, "teacher_tone3"), TEACHER_TONE3),
    ((20, "factory_worker_tone3"), FACTORY_WORKER_TONE3),
    ((18, "technologist_tone3"), TECHNOLOGIST_TONE3),
    ((19, "office_worker_tone3"), OFFICE_WORKER_TONE3),
    ((14, "mechanic_tone3"), MECHANIC_TONE3),
    ((15, "scientist_tone3"), SCIENTIST_TONE3),
    ((15, "astronaut_tone3"), ASTRONAUT_TONE3),
    ((17, "firefighter_tone3"), FIREFIGHTER_TONE3),
    ((28, "people_holding_hands_tone3-1"), PEOPLE_HOLDING_HANDS_TONE3_1),
    ((28, "people_holding_hands_tone3-2"), PEOPLE_HOLDING_HANDS_TONE3_2),
    ((26, "people_holding_hands_tone3"), PEOPLE_HOLDING_HANDS_TONE3),
    ((28, "people_holding_hands_tone3-4"), PEOPLE_HOLDING_HANDS_TONE3_4),
    ((28, "people_holding_hands_tone3-5"), PEOPLE_HOLDING_HANDS_TONE3_5),
    ((30, "person_with_probing_cane_tone3"), PERSON_WITH_PROBING_CANE_TONE3),
    ((28, "person_with_white_cane_tone3"), PERSON_WITH_WHITE_CANE_TONE3),
    ((16, "red_haired_tone3"), RED_HAIRED_TONE3),
    ((18, "curly_haired_tone3"), CURLY_HAIRED_TONE3),
    ((10, "bald_tone3"), BALD_TONE3),
    ((18, "white_haired_tone3"), WHITE_HAIRED_TONE3),
    ((36, "person_in_motorized_wheelchair_tone3"), PERSON_IN_MOTORIZED_WHEELCHAIR_TONE3),
    ((33, "person_in_manual_wheelchair_tone3"), PERSON_IN_MANUAL_WHEELCHAIR_TONE3),
    ((19, "health_worker_tone3"), HEALTH_WORKER_TONE3),
    ((11, "judge_tone3"), JUDGE_TONE3),
    ((11, "pilot_tone3"), PILOT_TONE3),
    ((19, "couple_kiss_tone3-1"), COUPLE_KISS_TONE3_1),
    ((18, "couplekiss_tone3-1"), COUPLEKISS_TONE3_1),
    ((19, "couple_kiss_tone3-2"), COUPLE_KISS_TONE3_2),
    ((18, "couplekiss_tone3-2"), COUPLEKISS_TONE3_2),
    ((19, "couple_kiss_tone3-4"), COUPLE_KISS_TONE3_4),
    ((18, "couplekiss_tone3-4"), COUPLEKISS_TONE3_4),
    ((19, "couple_kiss_tone3-5"), COUPLE_KISS_TONE3_5),
    ((18, "couplekiss_tone3-5"), COUPLEKISS_TONE3_5),
    ((25, "couple_with_heart_tone3-1"), COUPLE_WITH_HEART_TONE3_1),
    ((25, "couple_with_heart_tone3-2"), COUPLE_WITH_HEART_TONE3_2),
    ((25, "couple_with_heart_tone3-4"), COUPLE_WITH_HEART_TONE3_4),
    ((25, "couple_with_heart_tone3-5"), COUPLE_WITH_HEART_TONE3_5),
    ((11, "adult_tone3"), ADULT_TONE3),
    ((12, "farmer_tone4"), FARMER_TONE4),
    ((10, "cook_tone4"), COOK_TONE4),
    ((25, "person_feeding_baby_tone4"), PERSON_FEEDING_BABY_TONE4),
    ((14, "mx_claus_tone4"), MX_CLAUS_TONE4),
    ((13, "student_tone4"), STUDENT_TONE4),
    ((12, "singer_tone4"), SINGER_TONE4),
    ((12, "artist_tone4"), ARTIST_TONE4),
    ((13, "teacher_tone4"), TEACHER_TONE4),
    ((20, "factory_worker_tone4"), FACTORY_WORKER_TONE4),
    ((18, "technologist_tone4"), TECHNOLOGIST_TONE4),
    ((19, "office_worker_tone4"), OFFICE_WORKER_TONE4),
    ((14, "mechanic_tone4"), MECHANIC_TONE4),
    ((15, "scientist_tone4"), SCIENTIST_TONE4),
    ((15, "astronaut_tone4"), ASTRONAUT_TONE4),
    ((17, "firefighter_tone4"), FIREFIGHTER_TONE4),
    ((28, "people_holding_hands_tone4-1"), PEOPLE_HOLDING_HANDS_TONE4_1),
    ((28, "people_holding_hands_tone4-2"), PEOPLE_HOLDING_HANDS_TONE4_2),
    ((28, "people_holding_hands_tone4-3"), PEOPLE_HOLDING_HANDS_TONE4_3),
    ((26, "people_holding_hands_tone4"), PEOPLE_HOLDING_HANDS_TONE4),
    ((28, "people_holding_hands_tone4-5"), PEOPLE_HOLDING_HANDS_TONE4_5),
    ((30, "person_with_probing_cane_tone4"), PERSON_WITH_PROBING_CANE_TONE4),
    ((28, "person_with_white_cane_tone4"), PERSON_WITH_WHITE_CANE_TONE4),
    ((16, "red_haired_tone4"), RED_HAIRED_TONE4),
    ((18, "curly_haired_tone4"), CURLY_HAIRED_TONE4),
    ((10, "bald_tone4"), BALD_TONE4),
    ((18, "white_haired_tone4"), WHITE_HAIRED_TONE4),
    ((36, "person_in_motorized_wheelchair_tone4"), PERSON_IN_MOTORIZED_WHEELCHAIR_TONE4),
    ((33, "person_in_manual_wheelchair_tone4"), PERSON_IN_MANUAL_WHEELCHAIR_TONE4),
    ((19, "health_worker_tone4"), HEALTH_WORKER_TONE4),
    ((11, "judge_tone4"), JUDGE_TONE4),
    ((11, "pilot_tone4"), PILOT_TONE4),
    ((19, "couple_kiss_tone4-1"), COUPLE_KISS_TONE4_1),
    ((18, "couplekiss_tone4-1"), COUPLEKISS_TONE4_1),
    ((19, "couple_kiss_tone4-2"), COUPLE_KISS_TONE4_2),
    ((18, "couplekiss_tone4-2"), COUPLEKISS_TONE4_2),
    ((19, "couple_kiss_tone4-3"), COUPLE_KISS_TONE4_3),
    ((18, "couplekiss_tone4-3"), COUPLEKISS_TONE4_3),
    ((19, "couple_kiss_tone4-5"), COUPLE_KISS_TONE4_5),
    ((18, "couplekiss_tone4-5"), COUPLEKISS_TONE4_5),
    ((25, "couple_with_heart_tone4-1"), COUPLE_WITH_HEART_TONE4_1),
    ((25, "couple_with_heart_tone4-2"), COUPLE_WITH_HEART_TONE4_2),
    ((25, "couple_with_heart_tone4-3"), COUPLE_WITH_HEART_TONE4_3),
    ((25, "couple_with_heart_tone4-5"), COUPLE_WITH_HEART_TONE4_5),
    ((11, "adult_tone4"), ADULT_TONE4),
    ((12, "farmer_tone5"), FARMER_TONE5),
    ((10, "cook_tone5"), COOK_TONE5),
    ((25, "person_feeding_baby_tone5"), PERSON_FEEDING_BABY_TONE5),
    ((14, "mx_claus_tone5"), MX_CLAUS_TONE5),
    ((13, "student_tone5"), STUDENT_TONE5),
    ((12, "singer_tone5"), SINGER_TONE5),
    ((12, "artist_tone5"), ARTIST_TONE5),
    ((13, "teacher_tone5"), TEACHER_TONE5),
    ((20, "factory_worker_tone5"), FACTORY_WORKER_TONE5),
    ((18, "technologist_tone5"), TECHNOLOGIST_TONE5),
    ((19, "office_worker_tone5"), OFFICE_WORKER_TONE5),
    ((14, "mechanic_tone5"), MECHANIC_TONE5),
    ((15, "scientist_tone5"), SCIENTIST_TONE5),
    ((15, "astronaut_tone5"), ASTRONAUT_TONE5),
    ((17, "firefighter_tone5"), FIREFIGHTER_TONE5),
    ((28, "people_holding_hands_tone5-1"), PEOPLE_HOLDING_HANDS_TONE5_1),
    ((28, "people_holding_hands_tone5-2"), PEOPLE_HOLDING_HANDS_TONE5_2),
    ((28, "people_holding_hands_tone5-3"), PEOPLE_HOLDING_HANDS_TONE5_3),
    ((28, "people_holding_hands_tone5-4"), PEOPLE_HOLDING_HANDS_TONE5_4),
    ((26, "people_holding_hands_tone5"), PEOPLE_HOLDING_HANDS_TONE5),
    ((30, "person_with_probing_cane_tone5"), PERSON_WITH_PROBING_CANE_TONE5),
    ((28, "person_with_white_cane_tone5"), PERSON_WITH_WHITE_CANE_TONE5),
    ((16, "red_haired_tone5"), RED_HAIRED_TONE5),
    ((18, "curly_haired_tone5"), CURLY_HAIRED_TONE5),
    ((10, "bald_tone5"), BALD_TONE5),
    ((18, "white_haired_tone5"), WHITE_HAIRED_TONE5),
    ((36, "person_in_motorized_wheelchair_tone5"), PERSON_IN_MOTORIZED_WHEELCHAIR_TONE5),
    ((33, "person_in_manual_wheelchair_tone5"), PERSON_IN_MANUAL_WHEELCHAIR_TONE5),
    ((19, "health_worker_tone5"), HEALTH_WORKER_TONE5),
    ((11, "judge_tone5"), JUDGE_TONE5),
    ((11, "pilot_tone5"), PILOT_TONE5),
    ((19, "couple_kiss_tone5-1"), COUPLE_KISS_TONE5_1),
    ((18, "couplekiss_tone5-1"), COUPLEKISS_TONE5_1),
    ((19, "couple_kiss_tone5-2"), COUPLE_KISS_TONE5_2),
    ((18, "couplekiss_tone5-2"), COUPLEKISS_TONE5_2),
    ((19, "couple_kiss_tone5-3"), COUPLE_KISS_TONE5_3),
    ((18, "couplekiss_tone5-3"), COUPLEKISS_TONE5_3),
    ((19, "couple_kiss_tone5-4"), COUPLE_KISS_TONE5_4),
    ((18, "couplekiss_tone5-4"), COUPLEKISS_TONE5_4),
    ((25, "couple_with_heart_tone5-1"), COUPLE_WITH_HEART_TONE5_1),
    ((25, "couple_with_heart_tone5-2"), COUPLE_WITH_HEART_TONE5_2),
    ((25, "couple_with_heart_tone5-3"), COUPLE_WITH_HEART_TONE5_3),
    ((25, "couple_with_heart_tone5-4"), COUPLE_WITH_HEART_TONE5_4),
    ((11, "adult_tone5"), ADULT_TONE5),
    ((6, "farmer"), FARMER),
    ((4, "cook"), COOK),
    ((19, "person_feeding_baby"), PERSON_FEEDING_BABY),
    ((8, "mx_claus"), MX_CLAUS),
    ((7, "student"), STUDENT),
    ((6, "singer"), SINGER),
    ((6, "artist"), ARTIST),
    ((7, "teacher"), TEACHER),
    ((14, "factory_worker"), FACTORY_WORKER),
    ((12, "technologist"), TECHNOLOGIST),
    ((13, "office_worker"), OFFICE_WORKER),
    ((8, "mechanic"), MECHANIC),
    ((9, "scientist"), SCIENTIST),
    ((9, "astronaut"), ASTRONAUT),
    ((11, "firefighter"), FIREFIGHTER),
    ((20, "people_holding_hands"), PEOPLE_HOLDING_HANDS),
    ((24, "person_with_probing_cane"), PERSON_WITH_PROBING_CANE),
    ((22, "person_with_white_cane"), PERSON_WITH_WHITE_CANE),
    ((10, "red_haired"), RED_HAIRED),
    ((12, "curly_haired"), CURLY_HAIRED),
    ((4, "bald"), BALD),
    ((12, "white_haired"), WHITE_HAIRED),
    ((30, "person_in_motorized_wheelchair"), PERSON_IN_MOTORIZED_WHEELCHAIR),
    ((27, "person_in_manual_wheelchair"), PERSON_IN_MANUAL_WHEELCHAIR),
    ((13, "health_worker"), HEALTH_WORKER),
    ((5, "judge"), JUDGE),
    ((5, "pilot"), PILOT),
    ((5, "adult"), ADULT),
    ((11, "child_tone1"), CHILD_TONE1),
    ((11, "child_tone2"), CHILD_TONE2),
    ((11, "child_tone3"), CHILD_TONE3),
    ((11, "child_tone4"), CHILD_TONE4),
    ((11, "child_tone5"), CHILD_TONE5),
    ((5, "child"), CHILD),
    ((17, "older_adult_tone1"), OLDER_ADULT_TONE1),
    ((17, "older_adult_tone2"), OLDER_ADULT_TONE2),
    ((17, "older_adult_tone3"), OLDER_ADULT_TONE3),
    ((17, "older_adult_tone4"), OLDER_ADULT_TONE4),
    ((17, "older_adult_tone5"), OLDER_ADULT_TONE5),
    ((11, "older_adult"), OLDER_ADULT),
    ((19, "woman_bearded_tone1"), WOMAN_BEARDED_TONE1),
    ((17, "man_bearded_tone1"), MAN_BEARDED_TONE1),
    ((20, "person_bearded_tone1"), PERSON_BEARDED_TONE1),
    ((19, "woman_bearded_tone2"), WOMAN_BEARDED_TONE2),
    ((17, "man_bearded_tone2"), MAN_BEARDED_TONE2),
    ((20, "person_bearded_tone2"), PERSON_BEARDED_TONE2),
    ((19, "woman_bearded_tone3"), WOMAN_BEARDED_TONE3),
    ((17, "man_bearded_tone3"), MAN_BEARDED_TONE3),
    ((20, "person_bearded_tone3"), PERSON_BEARDED_TONE3),
    ((19, "woman_bearded_tone4"), WOMAN_BEARDED_TONE4),
    ((17, "man_bearded_tone4"), MAN_BEARDED_TONE4),
    ((20, "person_bearded_tone4"), PERSON_BEARDED_TONE4),
    ((19, "woman_bearded_tone5"), WOMAN_BEARDED_TONE5),
    ((17, "man_bearded_tone5"), MAN_BEARDED_TONE5),
    ((20, "person_bearded_tone5"), PERSON_BEARDED_TONE5),
    ((13, "woman_bearded"), WOMAN_BEARDED),
    ((11, "man_bearded"), MAN_BEARDED),
    ((14, "person_bearded"), PERSON_BEARDED),
    ((26, "woman_with_headscarf_tone1"), WOMAN_WITH_HEADSCARF_TONE1),
    ((26, "woman_with_headscarf_tone2"), WOMAN_WITH_HEADSCARF_TONE2),
    ((26, "woman_with_headscarf_tone3"), WOMAN_WITH_HEADSCARF_TONE3),
    ((26, "woman_with_headscarf_tone4"), WOMAN_WITH_HEADSCARF_TONE4),
    ((26, "woman_with_headscarf_tone5"), WOMAN_WITH_HEADSCARF_TONE5),
    ((20, "woman_with_headscarf"), WOMAN_WITH_HEADSCARF),
    ((26, "woman_in_steamy_room_tone1"), WOMAN_IN_STEAMY_ROOM_TONE1),
    ((24, "man_in_steamy_room_tone1"), MAN_IN_STEAMY_ROOM_TONE1),
    ((27, "person_in_steamy_room_tone1"), PERSON_IN_STEAMY_ROOM_TONE1),
    ((26, "woman_in_steamy_room_tone2"), WOMAN_IN_STEAMY_ROOM_TONE2),
    ((24, "man_in_steamy_room_tone2"), MAN_IN_STEAMY_ROOM_TONE2),
    ((27, "person_in_steamy_room_tone2"), PERSON_IN_STEAMY_ROOM_TONE2),
    ((26, "woman_in_steamy_room_tone3"), WOMAN_IN_STEAMY_ROOM_TONE3),
    ((24, "man_in_steamy_room_tone3"), MAN_IN_STEAMY_ROOM_TONE3),
    ((27, "person_in_steamy_room_tone3"), PERSON_IN_STEAMY_ROOM_TONE3),
    ((26, "woman_in_steamy_room_tone4"), WOMAN_IN_STEAMY_ROOM_TONE4),
    ((24, "man_in_steamy_room_tone4"), MAN_IN_STEAMY_ROOM_TONE4),
    ((27, "person_in_steamy_room_tone4"), PERSON_IN_STEAMY_ROOM_TONE4),
    ((26, "woman_in_steamy_room_tone5"), WOMAN_IN_STEAMY_ROOM_TONE5),
    ((24, "man_in_steamy_room_tone5"), MAN_IN_STEAMY_ROOM_TONE5),
    ((27, "person_in_steamy_room_tone5"), PERSON_IN_STEAMY_ROOM_TONE5),
    ((20, "woman_in_steamy_room"), WOMAN_IN_STEAMY_ROOM),
    ((18, "man_in_steamy_room"), MAN_IN_STEAMY_ROOM),
    ((21, "person_in_steamy_room"), PERSON_IN_STEAMY_ROOM),
    ((20, "woman_climbing_tone1"), WOMAN_CLIMBING_TONE1),
    ((18, "man_climbing_tone1"), MAN_CLIMBING_TONE1),
    ((14, "climbing_tone1"), CLIMBING_TONE1),
    ((21, "person_climbing_tone1"), PERSON_CLIMBING_TONE1),
    ((20, "woman_climbing_tone2"), WOMAN_CLIMBING_TONE2),
    ((18, "man_climbing_tone2"), MAN_CLIMBING_TONE2),
    ((14, "climbing_tone2"), CLIMBING_TONE2),
    ((21, "person_climbing_tone2"), PERSON_CLIMBING_TONE2),
    ((20, "woman_climbing_tone3"), WOMAN_CLIMBING_TONE3),
    ((18, "man_climbing_tone3"), MAN_CLIMBING_TONE3),
    ((14, "climbing_tone3"), CLIMBING_TONE3),
    ((21, "person_climbing_tone3"), PERSON_CLIMBING_TONE3),
    ((20, "woman_climbing_tone4"), WOMAN_CLIMBING_TONE4),
    ((18, "man_climbing_tone4"), MAN_CLIMBING_TONE4),
    ((14, "climbing_tone4"), CLIMBING_TONE4),
    ((21, "person_climbing_tone4"), PERSON_CLIMBING_TONE4),
    ((20, "woman_climbing_tone5"), WOMAN_CLIMBING_TONE5),
    ((18, "man_climbing_tone5"), MAN_CLIMBING_TONE5),
    ((14, "climbing_tone5"), CLIMBING_TONE5),
    ((21, "person_climbing_tone5"), PERSON_CLIMBING_TONE5),
    ((14, "woman_climbing"), WOMAN_CLIMBING),
    ((12, "man_climbing"), MAN_CLIMBING),
    ((8, "climbing"), CLIMBING),
    ((15, "person_climbing"), PERSON_CLIMBING),
    ((29, "woman_in_lotus_position_tone1"), WOMAN_IN_LOTUS_POSITION_TONE1),
    ((27, "man_in_lotus_position_tone1"), MAN_IN_LOTUS_POSITION_TONE1),
    ((30, "person_in_lotus_position_tone1"), PERSON_IN_LOTUS_POSITION_TONE1),
    ((29, "woman_in_lotus_position_tone2"), WOMAN_IN_LOTUS_POSITION_TONE2),
    ((27, "man_in_lotus_position_tone2"), MAN_IN_LOTUS_POSITION_TONE2),
    ((30, "person_in_lotus_position_tone2"), PERSON_IN_LOTUS_POSITION_TONE2),
    ((29, "woman_in_lotus_position_tone3"), WOMAN_IN_LOTUS_POSITION_TONE3),
    ((27, "man_in_lotus_position_tone3"), MAN_IN_LOTUS_POSITION_TONE3),
    ((30, "person_in_lotus_position_tone3"), PERSON_IN_LOTUS_POSITION_TONE3),
    ((29, "woman_in_lotus_position_tone4"), WOMAN_IN_LOTUS_POSITION_TONE4),
    ((27, "man_in_lotus_position_tone4"), MAN_IN_LOTUS_POSITION_TONE4),
    ((30, "person_in_lotus_position_tone4"), PERSON_IN_LOTUS_POSITION_TONE4),
    ((29, "woman_in_lotus_position_tone5"), WOMAN_IN_LOTUS_POSITION_TONE5),
    ((27, "man_in_lotus_position_tone5"), MAN_IN_LOTUS_POSITION_TONE5),
    ((30, "person_in_lotus_position_tone5"), PERSON_IN_LOTUS_POSITION_TONE5),
    ((23, "woman_in_lotus_position"), WOMAN_IN_LOTUS_POSITION),
    ((21, "man_in_lotus_position"), MAN_IN_LOTUS_POSITION),
    ((24, "person_in_lotus_position"), PERSON_IN_LOTUS_POSITION),
    ((16, "woman_mage_tone1"), WOMAN_MAGE_TONE1),
    ((14, "man_mage_tone1"), MAN_MAGE_TONE1),
    ((10, "mage_tone1"), MAGE_TONE1),
    ((16, "woman_mage_tone2"), WOMAN_MAGE_TONE2),
    ((14, "man_mage_tone2"), MAN_MAGE_TONE2),
    ((10, "mage_tone2"), MAGE_TONE2),
    ((16, "woman_mage_tone3"), WOMAN_MAGE_TONE3),
    ((14, "man_mage_tone3"), MAN_MAGE_TONE3),
    ((10, "mage_tone3"), MAGE_TONE3),
    ((16, "woman_mage_tone4"), WOMAN_MAGE_TONE4),
    ((14, "man_mage_tone4"), MAN_MAGE_TONE4),
    ((10, "mage_tone4"), MAGE_TONE4),
    ((16, "woman_mage_tone5"), WOMAN_MAGE_TONE5),
    ((14, "man_mage_tone5"), MAN_MAGE_TONE5),
    ((10, "mage_tone5"), MAGE_TONE5),
    ((10, "woman_mage"), WOMAN_MAGE),
    ((8, "man_mage"), MAN_MAGE),
    ((4, "mage"), MAGE),
    ((17, "woman_fairy_tone1"), WOMAN_FAIRY_TONE1),
    ((15, "man_fairy_tone1"), MAN_FAIRY_TONE1),
    ((11, "fairy_tone1"), FAIRY_TONE1),
    ((17, "woman_fairy_tone2"), WOMAN_FAIRY_TONE2),
    ((15, "man_fairy_tone2"), MAN_FAIRY_TONE2),
    ((11, "fairy_tone2"), FAIRY_TONE2),
    ((17, "woman_fairy_tone3"), WOMAN_FAIRY_TONE3),
    ((15, "man_fairy_tone3"), MAN_FAIRY_TONE3),
    ((11, "fairy_tone3"), FAIRY_TONE3),
    ((17, "woman_fairy_tone4"), WOMAN_FAIRY_TONE4),
    ((15, "man_fairy_tone4"), MAN_FAIRY_TONE4),
    ((11, "fairy_tone4"), FAIRY_TONE4),
    ((17, "woman_fairy_tone5"), WOMAN_FAIRY_TONE5),
    ((15, "man_fairy_tone5"), MAN_FAIRY_TONE5),
    ((11, "fairy_tone5"), FAIRY_TONE5),
    ((11, "woman_fairy"), WOMAN_FAIRY),
    ((9, "man_fairy"), MAN_FAIRY),
    ((5, "fairy"), FAIRY),
    ((19, "woman_vampire_tone1"), WOMAN_VAMPIRE_TONE1),
    ((17, "man_vampire_tone1"), MAN_VAMPIRE_TONE1),
    ((13, "vampire_tone1"), VAMPIRE_TONE1),
    ((19, "woman_vampire_tone2"), WOMAN_VAMPIRE_TONE2),
    ((17, "man_vampire_tone2"), MAN_VAMPIRE_TONE2),
    ((13, "vampire_tone2"), VAMPIRE_TONE2),
    ((19, "woman_vampire_tone3"), WOMAN_VAMPIRE_TONE3),
    ((17, "man_vampire_tone3"), MAN_VAMPIRE_TONE3),
    ((13, "vampire_tone3"), VAMPIRE_TONE3),
    ((19, "woman_vampire_tone4"), WOMAN_VAMPIRE_TONE4),
    ((17, "man_vampire_tone4"), MAN_VAMPIRE_TONE4),
    ((13, "vampire_tone4"), VAMPIRE_TONE4),
    ((19, "woman_vampire_tone5"), WOMAN_VAMPIRE_TONE5),
    ((17, "man_vampire_tone5"), MAN_VAMPIRE_TONE5),
    ((13, "vampire_tone5"), VAMPIRE_TONE5),
    ((13, "woman_vampire"), WOMAN_VAMPIRE),
    ((11, "man_vampire"), MAN_VAMPIRE),
    ((7, "vampire"), VAMPIRE),
    ((13, "mermaid_tone1"), MERMAID_TONE1),
    ((12, "merman_tone1"), MERMAN_TONE1),
    ((15, "merperson_tone1"), MERPERSON_TONE1),
    ((13, "mermaid_tone2"), MERMAID_TONE2),
    ((12, "merman_tone2"), MERMAN_TONE2),
    ((15, "merperson_tone2"), MERPERSON_TONE2),
    ((13, "mermaid_tone3"), MERMAID_TONE3),
    ((12, "merman_tone3"), MERMAN_TONE3),
    ((15, "merperson_tone3"), MERPERSON_TONE3),
    ((13, "mermaid_tone4"), MERMAID_TONE4),
    ((12, "merman_tone4"), MERMAN_TONE4),
    ((15, "merperson_tone4"), MERPERSON_TONE4),
    ((13, "mermaid_tone5"), MERMAID_TONE5),
    ((12, "merman_tone5"), MERMAN_TONE5),
    ((15, "merperson_tone5"), MERPERSON_TONE5),
    ((7, "mermaid"), MERMAID),
    ((6, "merman"), MERMAN),
    ((9, "merperson"), MERPERSON),
    ((15, "woman_elf_tone1"), WOMAN_ELF_TONE1),
    ((13, "man_elf_tone1"), MAN_ELF_TONE1),
    ((9, "elf_tone1"), ELF_TONE1),
    ((15, "woman_elf_tone2"), WOMAN_ELF_TONE2),
    ((13, "man_elf_tone2"), MAN_ELF_TONE2),
    ((9, "elf_tone2"), ELF_TONE2),
    ((15, "woman_elf_tone3"), WOMAN_ELF_TONE3),
    ((13, "man_elf_tone3"), MAN_ELF_TONE3),
    ((9, "elf_tone3"), ELF_TONE3),
    ((15, "woman_elf_tone4"), WOMAN_ELF_TONE4),
    ((13, "man_elf_tone4"), MAN_ELF_TONE4),
    ((9, "elf_tone4"), ELF_TONE4),
    ((15, "woman_elf_tone5"), WOMAN_ELF_TONE5),
    ((13, "man_elf_tone5"), MAN_ELF_TONE5),
    ((9, "elf_tone5"), ELF_TONE5),
    ((9, "woman_elf"), WOMAN_ELF),
    ((7, "man_elf"), MAN_ELF),
    ((3, "elf"), ELF),
    ((11, "woman_genie"), WOMAN_GENIE),
    ((9, "man_genie"), MAN_GENIE),
    ((5, "genie"), GENIE),
    ((12, "woman_zombie"), WOMAN_ZOMBIE),
    ((10, "man_zombie"), MAN_ZOMBIE),
    ((6, "zombie"), ZOMBIE),
    ((5, "brain"), BRAIN),
    ((12, "orange_heart"), ORANGE_HEART),
    ((10, "billed_cap"), BILLED_CAP),
    ((5, "scarf"), SCARF),
    ((6, "gloves"), GLOVES),
    ((4, "coat"), COAT),
    ((5, "socks"), SOCKS),
    ((12, "red_envelope"), RED_ENVELOPE),
    ((11, "firecracker"), FIRECRACKER),
    ((6, "jigsaw"), JIGSAW),
    ((12, "puzzle_piece"), PUZZLE_PIECE),
    ((9, "test_tube"), TEST_TUBE),
    ((10, "petri_dish"), PETRI_DISH),
    ((3, "dna"), DNA),
    ((12, "double_helix"), DOUBLE_HELIX),
    ((7, "compass"), COMPASS),
    ((6, "abacus"), ABACUS),
    ((17, "fire_extinguisher"), FIRE_EXTINGUISHER),
    ((7, "toolbox"), TOOLBOX),
    ((6, "bricks"), BRICKS),
    ((6, "magnet"), MAGNET),
    ((7, "luggage"), LUGGAGE),
    ((13, "lotion_bottle"), LOTION_BOTTLE),
    ((6, "thread"), THREAD),
    ((4, "yarn"), YARN),
    ((10, "safety_pin"), SAFETY_PIN),
    ((10, "teddy_bear"), TEDDY_BEAR),
    ((5, "broom"), BROOM),
    ((6, "basket"), BASKET),
    ((13, "roll_of_paper"), ROLL_OF_PAPER),
    ((12, "toilet_paper"), TOILET_PAPER),
    ((4, "soap"), SOAP),
    ((6, "sponge"), SPONGE),
    ((7, "receipt"), RECEIPT),
    ((12, "nazar_amulet"), NAZAR_AMULET),
    ((12, "ballet_shoes"), BALLET_SHOES),
    ((18, "one_piece_swimsuit"), ONE_PIECE_SWIMSUIT),
    ((6, "briefs"), BRIEFS),
    ((6, "shorts"), SHORTS),
    ((12, "thong_sandal"), THONG_SANDAL),
    ((13, "drop_of_blood"), DROP_OF_BLOOD),
    ((16, "adhesive_bandage"), ADHESIVE_BANDAGE),
    ((7, "bandaid"), BANDAID),
    ((11, "stethoscope"), STETHOSCOPE),
    ((5, "x-ray"), X_RAY),
    ((4, "xray"), XRAY),
    ((6, "crutch"), CRUTCH),
    ((5, "yo_yo"), YO_YO),
    ((4, "kite"), KITE),
    ((9, "parachute"), PARACHUTE),
    ((9, "boomerang"), BOOMERANG),
    ((10, "magic_wand"), MAGIC_WAND),
    ((6, "pinata"), PINATA),
    ((13, "nesting_dolls"), NESTING_DOLLS),
    ((13, "ringed_planet"), RINGED_PLANET),
    ((6, "saturn"), SATURN),
    ((5, "chair"), CHAIR),
    ((5, "razor"), RAZOR),
    ((3, "axe"), AXE),
    ((9, "diya_lamp"), DIYA_LAMP),
    ((5, "banjo"), BANJO),
    ((15, "military_helmet"), MILITARY_HELMET),
    ((9, "accordion"), ACCORDION),
    ((9, "long_drum"), LONG_DRUM),
    ((4, "coin"), COIN),
    ((13, "carpentry_saw"), CARPENTRY_SAW),
    ((11, "screwdriver"), SCREWDRIVER),
    ((6, "ladder"), LADDER),
    ((4, "hook"), HOOK),
    ((6, "mirror"), MIRROR),
    ((6, "window"), WINDOW),
    ((7, "plunger"), PLUNGER),
    ((13, "sewing_needle"), SEWING_NEEDLE),
    ((4, "knot"), KNOT),
    ((6, "bucket"), BUCKET),
    ((10, "mouse_trap"), MOUSE_TRAP),
    ((10, "toothbrush"), TOOTHBRUSH),
    ((9, "headstone"), HEADSTONE),
    ((7, "placard"), PLACARD),
    ((4, "rock"), ROCK),
    ((5, "disco"), DISCO),
    ((10, "disco_ball"), DISCO_BALL),
    ((11, "mirror_ball"), MIRROR_BALL),
    ((7, "id_card"), ID_CARD),
    ((11, "low_battery"), LOW_BATTERY),
    ((5, "hamsa"), HAMSA),
    ((3, "fly"), FLY),
    ((4, "worm"), WORM),
    ((6, "beetle"), BEETLE),
    ((9, "cockroach"), COCKROACH),
    ((12, "potted_plant"), POTTED_PLANT),
    ((4, "wood"), WOOD),
    ((7, "feather"), FEATHER),
    ((5, "lotus"), LOTUS),
    ((5, "coral"), CORAL),
    ((10, "empty_nest"), EMPTY_NEST),
    ((4, "nest"), NEST),
    ((14, "nest_with_eggs"), NEST_WITH_EGGS),
    ((16, "anatomical_heart"), ANATOMICAL_HEART),
    ((5, "lungs"), LUNGS),
    ((14, "people_hugging"), PEOPLE_HUGGING),
    ((18, "pregnant_man_tone1"), PREGNANT_MAN_TONE1),
    ((18, "pregnant_man_tone2"), PREGNANT_MAN_TONE2),
    ((18, "pregnant_man_tone3"), PREGNANT_MAN_TONE3),
    ((18, "pregnant_man_tone4"), PREGNANT_MAN_TONE4),
    ((18, "pregnant_man_tone5"), PREGNANT_MAN_TONE5),
    ((12, "pregnant_man"), PREGNANT_MAN),
    ((21, "pregnant_person_tone1"), PREGNANT_PERSON_TONE1),
    ((21, "pregnant_person_tone2"), PREGNANT_PERSON_TONE2),
    ((21, "pregnant_person_tone3"), PREGNANT_PERSON_TONE3),
    ((21, "pregnant_person_tone4"), PREGNANT_PERSON_TONE4),
    ((21, "pregnant_person_tone5"), PREGNANT_PERSON_TONE5),
    ((15, "pregnant_person"), PREGNANT_PERSON),
    ((23, "person_with_crown_tone1"), PERSON_WITH_CROWN_TONE1),
    ((13, "royalty_tone1"), ROYALTY_TONE1),
    ((23, "person_with_crown_tone2"), PERSON_WITH_CROWN_TONE2),
    ((13, "royalty_tone2"), ROYALTY_TONE2),
    ((23, "person_with_crown_tone3"), PERSON_WITH_CROWN_TONE3),
    ((13, "royalty_tone3"), ROYALTY_TONE3),
    ((23, "person_with_crown_tone4"), PERSON_WITH_CROWN_TONE4),
    ((13, "royalty_tone4"), ROYALTY_TONE4),
    ((23, "person_with_crown_tone5"), PERSON_WITH_CROWN_TONE5),
    ((13, "royalty_tone5"), ROYALTY_TONE5),
    ((17, "person_with_crown"), PERSON_WITH_CROWN),
    ((7, "royalty"), ROYALTY),
    ((11, "blueberries"), BLUEBERRIES),
    ((11, "bell_pepper"), BELL_PEPPER),
    ((5, "olive"), OLIVE),
    ((9, "flatbread"), FLATBREAD),
    ((6, "tamale"), TAMALE),
    ((6, "fondue"), FONDUE),
    ((6, "teapot"), TEAPOT),
    ((4, "pour"), POUR),
    ((14, "pouring_liquid"), POURING_LIQUID),
    ((5, "beans"), BEANS),
    ((3, "jar"), JAR),
    ((4, "melt"), MELT),
    ((12, "melting_face"), MELTING_FACE),
    ((6, "salute"), SALUTE),
    ((13, "saluting_face"), SALUTING_FACE),
    ((35, "face_with_open_eyes_hand_over_mouth"), FACE_WITH_OPEN_EYES_HAND_OVER_MOUTH),
    ((4, "gasp"), GASP),
    ((21, "face_with_peeking_eye"), FACE_WITH_PEEKING_EYE),
    ((4, "peek"), PEEK),
    ((24, "face_with_diagonal_mouth"), FACE_WITH_DIAGONAL_MOUTH),
    ((16, "dotted_line_face"), DOTTED_LINE_FACE),
    ((10, "biting_lip"), BITING_LIP),
    ((7, "bubbles"), BUBBLES),
    ((46, "hand_with_index_finger_and_thumb_crossed_tone1"), HAND_WITH_INDEX_FINGER_AND_THUMB_CROSSED_TONE1),
    ((46, "hand_with_index_finger_and_thumb_crossed_tone2"), HAND_WITH_INDEX_FINGER_AND_THUMB_CROSSED_TONE2),
    ((46, "hand_with_index_finger_and_thumb_crossed_tone3"), HAND_WITH_INDEX_FINGER_AND_THUMB_CROSSED_TONE3),
    ((46, "hand_with_index_finger_and_thumb_crossed_tone4"), HAND_WITH_INDEX_FINGER_AND_THUMB_CROSSED_TONE4),
    ((46, "hand_with_index_finger_and_thumb_crossed_tone5"), HAND_WITH_INDEX_FINGER_AND_THUMB_CROSSED_TONE5),
    ((40, "hand_with_index_finger_and_thumb_crossed"), HAND_WITH_INDEX_FINGER_AND_THUMB_CROSSED),
    ((17, "handshake_tone1-2"), HANDSHAKE_TONE1_2),
    ((17, "handshake_tone1-3"), HANDSHAKE_TONE1_3),
    ((17, "handshake_tone1-4"), HANDSHAKE_TONE1_4),
    ((17, "handshake_tone1-5"), HANDSHAKE_TONE1_5),
    ((21, "rightwards_hand_tone1"), RIGHTWARDS_HAND_TONE1),
    ((17, "handshake_tone2-1"), HANDSHAKE_TONE2_1),
    ((17, "handshake_tone2-3"), HANDSHAKE_TONE2_3),
    ((17, "handshake_tone2-4"), HANDSHAKE_TONE2_4),
    ((17, "handshake_tone2-5"), HANDSHAKE_TONE2_5),
    ((21, "rightwards_hand_tone2"), RIGHTWARDS_HAND_TONE2),
    ((17, "handshake_tone3-1"), HANDSHAKE_TONE3_1),
    ((17, "handshake_tone3-2"), HANDSHAKE_TONE3_2),
    ((17, "handshake_tone3-4"), HANDSHAKE_TONE3_4),
    ((17, "handshake_tone3-5"), HANDSHAKE_TONE3_5),
    ((21, "rightwards_hand_tone3"), RIGHTWARDS_HAND_TONE3),
    ((17, "handshake_tone4-1"), HANDSHAKE_TONE4_1),
    ((17, "handshake_tone4-2"), HANDSHAKE_TONE4_2),
    ((17, "handshake_tone4-3"), HANDSHAKE_TONE4_3),
    ((17, "handshake_tone4-5"), HANDSHAKE_TONE4_5),
    ((21, "rightwards_hand_tone4"), RIGHTWARDS_HAND_TONE4),
    ((17, "handshake_tone5-1"), HANDSHAKE_TONE5_1),
    ((17, "handshake_tone5-2"), HANDSHAKE_TONE5_2),
    ((17, "handshake_tone5-3"), HANDSHAKE_TONE5_3),
    ((17, "handshake_tone5-4"), HANDSHAKE_TONE5_4),
    ((21, "rightwards_hand_tone5"), RIGHTWARDS_HAND_TONE5),
    ((15, "rightwards_hand"), RIGHTWARDS_HAND),
    ((20, "leftwards_hand_tone1"), LEFTWARDS_HAND_TONE1),
    ((20, "leftwards_hand_tone2"), LEFTWARDS_HAND_TONE2),
    ((20, "leftwards_hand_tone3"), LEFTWARDS_HAND_TONE3),
    ((20, "leftwards_hand_tone4"), LEFTWARDS_HAND_TONE4),
    ((20, "leftwards_hand_tone5"), LEFTWARDS_HAND_TONE5),
    ((14, "leftwards_hand"), LEFTWARDS_HAND),
    ((15, "palm_down_tone1"), PALM_DOWN_TONE1),
    ((15, "palm_down_tone2"), PALM_DOWN_TONE2),
    ((15, "palm_down_tone3"), PALM_DOWN_TONE3),
    ((15, "palm_down_tone4"), PALM_DOWN_TONE4),
    ((15, "palm_down_tone5"), PALM_DOWN_TONE5),
    ((9, "palm_down"), PALM_DOWN),
    ((13, "palm_up_tone1"), PALM_UP_TONE1),
    ((13, "palm_up_tone2"), PALM_UP_TONE2),
    ((13, "palm_up_tone3"), PALM_UP_TONE3),
    ((13, "palm_up_tone4"), PALM_UP_TONE4),
    ((13, "palm_up_tone5"), PALM_UP_TONE5),
    ((7, "palm_up"), PALM_UP),
    ((19, "point_forward_tone1"), POINT_FORWARD_TONE1),
    ((19, "point_forward_tone2"), POINT_FORWARD_TONE2),
    ((19, "point_forward_tone3"), POINT_FORWARD_TONE3),
    ((19, "point_forward_tone4"), POINT_FORWARD_TONE4),
    ((19, "point_forward_tone5"), POINT_FORWARD_TONE5),
    ((13, "point_forward"), POINT_FORWARD),
    ((17, "heart_hands_tone1"), HEART_HANDS_TONE1),
    ((17, "heart_hands_tone2"), HEART_HANDS_TONE2),
    ((17, "heart_hands_tone3"), HEART_HANDS_TONE3),
    ((17, "heart_hands_tone4"), HEART_HANDS_TONE4),
    ((17, "heart_hands_tone5"), HEART_HANDS_TONE5),
    ((11, "heart_hands"), HEART_HANDS),
    ((8, "bangbang"), BANGBANG),
    ((18, "double_exclamation"), DOUBLE_EXCLAMATION),
    ((20, "exclamation_question"), EXCLAMATION_QUESTION),
    ((11, "interrobang"), INTERROBANG),
    ((2, "tm"), TM),
    ((10, "trade_mark"), TRADE_MARK),
    ((4, "info"), INFO),
    ((18, "information_source"), INFORMATION_SOURCE),
    ((16, "left_right_arrow"), LEFT_RIGHT_ARROW),
    ((13, "arrow_up_down"), ARROW_UP_DOWN),
    ((16, "arrow_upper_left"), ARROW_UPPER_LEFT),
    ((17, "arrow_upper_right"), ARROW_UPPER_RIGHT),
    ((17, "arrow_lower_right"), ARROW_LOWER_RIGHT),
    ((16, "arrow_lower_left"), ARROW_LOWER_LEFT),
    ((15, "arrow_left_hook"), ARROW_LEFT_HOOK),
    ((25, "leftwards_arrow_with_hook"), LEFTWARDS_ARROW_WITH_HOOK),
    ((16, "arrow_right_hook"), ARROW_RIGHT_HOOK),
    ((26, "rightwards_arrow_with_hook"), RIGHTWARDS_ARROW_WITH_HOOK),
    ((5, "watch"), WATCH),
    ((9, "hourglass"), HOURGLASS),
    ((8, "keyboard"), KEYBOARD),
    ((5, "eject"), EJECT),
    ((12, "fast_forward"), FAST_FORWARD),
    ((12, "fast_reverse"), FAST_REVERSE),
    ((6, "rewind"), REWIND),
    ((15, "arrow_double_up"), ARROW_DOUBLE_UP),
    ((7, "fast_up"), FAST_UP),
    ((17, "arrow_double_down"), ARROW_DOUBLE_DOWN),
    ((9, "fast_down"), FAST_DOWN),
    ((10, "next_track"), NEXT_TRACK),
    ((14, "previous_track"), PREVIOUS_TRACK),
    ((10, "play_pause"), PLAY_PAUSE),
    ((11, "alarm_clock"), ALARM_CLOCK),
    ((9, "stopwatch"), STOPWATCH),
    ((11, "timer_clock"), TIMER_CLOCK),
    ((22, "hourglass_flowing_sand"), HOURGLASS_FLOWING_SAND),
    ((5, "pause"), PAUSE),
    ((4, "stop"), STOP),
    ((6, "record"), RECORD),
    ((1, "m"), M),
    ((18, "black_small_square"), BLACK_SMALL_SQUARE),
    ((18, "white_small_square"), WHITE_SMALL_SQUARE),
    ((13, "arrow_forward"), ARROW_FORWARD),
    ((4, "play"), PLAY),
    ((14, "arrow_backward"), ARROW_BACKWARD),
    ((7, "reverse"), REVERSE),
    ((19, "white_medium_square"), WHITE_MEDIUM_SQUARE),
    ((19, "black_medium_square"), BLACK_MEDIUM_SQUARE),
    ((25, "white_medium_small_square"), WHITE_MEDIUM_SMALL_SQUARE),
    ((25, "black_medium_small_square"), BLACK_MEDIUM_SMALL_SQUARE),
    ((3, "sun"), SUN),
    ((5, "cloud"), CLOUD),
    ((8, "umbrella"), UMBRELLA),
    ((8, "snowman2"), SNOWMAN2),
    ((5, "comet"), COMET),
    ((9, "telephone"), TELEPHONE),
    ((21, "ballot_box_with_check"), BALLOT_BOX_WITH_CHECK),
    ((18, "umbrella_with_rain"), UMBRELLA_WITH_RAIN),
    ((6, "coffee"), COFFEE),
    ((8, "shamrock"), SHAMROCK),
    ((16, "point_up_2_tone1"), POINT_UP_2_TONE1),
    ((16, "point_up_2_tone2"), POINT_UP_2_TONE2),
    ((16, "point_up_2_tone3"), POINT_UP_2_TONE3),
    ((16, "point_up_2_tone4"), POINT_UP_2_TONE4),
    ((16, "point_up_2_tone5"), POINT_UP_2_TONE5),
    ((10, "point_up_2"), POINT_UP_2),
    ((20, "skull_and_crossbones"), SKULL_AND_CROSSBONES),
    ((11, "radioactive"), RADIOACTIVE),
    ((9, "biohazard"), BIOHAZARD),
    ((14, "orthodox_cross"), ORTHODOX_CROSS),
    ((17, "star_and_crescent"), STAR_AND_CRESCENT),
    ((5, "peace"), PEACE),
    ((12, "peace_symbol"), PEACE_SYMBOL),
    ((8, "yin_yang"), YIN_YANG),
    ((15, "wheel_of_dharma"), WHEEL_OF_DHARMA),
    ((19, "white_frowning_face"), WHITE_FROWNING_FACE),
    ((7, "relaxed"), RELAXED),
    ((12, "smiling_face"), SMILING_FACE),
    ((6, "female"), FEMALE),
    ((11, "female_sign"), FEMALE_SIGN),
    ((4, "male"), MALE),
    ((9, "male_sign"), MALE_SIGN),
    ((5, "aries"), ARIES),
    ((6, "taurus"), TAURUS),
    ((6, "gemini"), GEMINI),
    ((6, "cancer"), CANCER),
    ((3, "leo"), LEO),
    ((5, "virgo"), VIRGO),
    ((5, "libra"), LIBRA),
    ((8, "scorpius"), SCORPIUS),
    ((11, "sagittarius"), SAGITTARIUS),
    ((9, "capricorn"), CAPRICORN),
    ((8, "aquarius"), AQUARIUS),
    ((6, "pisces"), PISCES),
    ((10, "chess_pawn"), CHESS_PAWN),
    ((6, "spades"), SPADES),
    ((5, "clubs"), CLUBS),
    ((6, "hearts"), HEARTS),
    ((8, "diamonds"), DIAMONDS),
    ((10, "hotsprings"), HOTSPRINGS),
    ((7, "recycle"), RECYCLE),
    ((16, "recycling_symbol"), RECYCLING_SYMBOL),
    ((8, "infinity"), INFINITY),
    ((11, "handicapped"), HANDICAPPED),
    ((10, "wheelchair"), WHEELCHAIR),
    ((15, "hammer_and_pick"), HAMMER_AND_PICK),
    ((6, "anchor"), ANCHOR),
    ((14, "crossed_swords"), CROSSED_SWORDS),
    ((7, "medical"), MEDICAL),
    ((14, "medical_symbol"), MEDICAL_SYMBOL),
    ((6, "scales"), SCALES),
    ((7, "alembic"), ALEMBIC),
    ((4, "gear"), GEAR),
    ((4, "atom"), ATOM),
    ((11, "atom_symbol"), ATOM_SYMBOL),
    ((12, "fleur-de-lis"), FLEUR_DE_LIS),
    ((7, "warning"), WARNING),
    ((12, "high_voltage"), HIGH_VOLTAGE),
    ((3, "zap"), ZAP),
    ((18, "transgender_symbol"), TRANSGENDER_SYMBOL),
    ((12, "white_circle"), WHITE_CIRCLE),
    ((12, "black_circle"), BLACK_CIRCLE),
    ((6, "coffin"), COFFIN),
    ((11, "funeral_urn"), FUNERAL_URN),
    ((6, "soccer"), SOCCER),
    ((8, "baseball"), BASEBALL),
    ((7, "snowman"), SNOWMAN),
    ((12, "partly_sunny"), PARTLY_SUNNY),
    ((16, "sun_behind_cloud"), SUN_BEHIND_CLOUD),
    ((6, "stormy"), STORMY),
    ((22, "thunder_cloud_and_rain"), THUNDER_CLOUD_AND_RAIN),
    ((9, "ophiuchus"), OPHIUCHUS),
    ((4, "pick"), PICK),
    ((17, "helmet_with_cross"), HELMET_WITH_CROSS),
    ((20, "rescue_worker_helmet"), RESCUE_WORKER_HELMET),
    ((6, "chains"), CHAINS),
    ((8, "no_entry"), NO_ENTRY),
    ((13, "shinto_shrine"), SHINTO_SHRINE),
    ((6, "church"), CHURCH),
    ((8, "mountain"), MOUNTAIN),
    ((14, "beach_umbrella"), BEACH_UMBRELLA),
    ((18, "umbrella_on_ground"), UMBRELLA_ON_GROUND),
    ((8, "fountain"), FOUNTAIN),
    ((4, "golf"), GOLF),
    ((5, "ferry"), FERRY),
    ((8, "sailboat"), SAILBOAT),
    ((13, "person_skiing"), PERSON_SKIING),
    ((5, "skier"), SKIER),
    ((6, "skiing"), SKIING),
    ((9, "ice_skate"), ICE_SKATE),
    ((25, "woman_bouncing_ball_tone1"), WOMAN_BOUNCING_BALL_TONE1),
    ((23, "man_bouncing_ball_tone1"), MAN_BOUNCING_BALL_TONE1),
    ((26, "person_bouncing_ball_tone1"), PERSON_BOUNCING_BALL_TONE1),
    ((25, "woman_bouncing_ball_tone2"), WOMAN_BOUNCING_BALL_TONE2),
    ((23, "man_bouncing_ball_tone2"), MAN_BOUNCING_BALL_TONE2),
    ((26, "person_bouncing_ball_tone2"), PERSON_BOUNCING_BALL_TONE2),
    ((25, "woman_bouncing_ball_tone3"), WOMAN_BOUNCING_BALL_TONE3),
    ((23, "man_bouncing_ball_tone3"), MAN_BOUNCING_BALL_TONE3),
    ((26, "person_bouncing_ball_tone3"), PERSON_BOUNCING_BALL_TONE3),
    ((25, "woman_bouncing_ball_tone4"), WOMAN_BOUNCING_BALL_TONE4),
    ((23, "man_bouncing_ball_tone4"), MAN_BOUNCING_BALL_TONE4),
    ((26, "person_bouncing_ball_tone4"), PERSON_BOUNCING_BALL_TONE4),
    ((25, "woman_bouncing_ball_tone5"), WOMAN_BOUNCING_BALL_TONE5),
    ((23, "man_bouncing_ball_tone5"), MAN_BOUNCING_BALL_TONE5),
    ((26, "person_bouncing_ball_tone5"), PERSON_BOUNCING_BALL_TONE5),
    ((19, "woman_bouncing_ball"), WOMAN_BOUNCING_BALL),
    ((17, "man_bouncing_ball"), MAN_BOUNCING_BALL),
    ((20, "person_bouncing_ball"), PERSON_BOUNCING_BALL),
    ((4, "tent"), TENT),
    ((8, "fuelpump"), FUELPUMP),
    ((8, "scissors"), SCISSORS),
    ((17, "check_mark_button"), CHECK_MARK_BUTTON),
    ((16, "white_check_mark"), WHITE_CHECK_MARK),
    ((8, "airplane"), AIRPLANE),
    ((8, "envelope"), ENVELOPE),
    ((10, "fist_tone1"), FIST_TONE1),
    ((10, "fist_tone2"), FIST_TONE2),
    ((10, "fist_tone3"), FIST_TONE3),
    ((10, "fist_tone4"), FIST_TONE4),
    ((10, "fist_tone5"), FIST_TONE5),
    ((4, "fist"), FIST),
    ((15, "high_five_tone1"), HIGH_FIVE_TONE1),
    ((17, "raised_hand_tone1"), RAISED_HAND_TONE1),
    ((15, "high_five_tone2"), HIGH_FIVE_TONE2),
    ((17, "raised_hand_tone2"), RAISED_HAND_TONE2),
    ((15, "high_five_tone3"), HIGH_FIVE_TONE3),
    ((17, "raised_hand_tone3"), RAISED_HAND_TONE3),
    ((15, "high_five_tone4"), HIGH_FIVE_TONE4),
    ((17, "raised_hand_tone4"), RAISED_HAND_TONE4),
    ((15, "high_five_tone5"), HIGH_FIVE_TONE5),
    ((17, "raised_hand_tone5"), RAISED_HAND_TONE5),
    ((9, "high_five"), HIGH_FIVE),
    ((11, "raised_hand"), RAISED_HAND),
    ((7, "v_tone1"), V_TONE1),
    ((13, "victory_tone1"), VICTORY_TONE1),
    ((7, "v_tone2"), V_TONE2),
    ((13, "victory_tone2"), VICTORY_TONE2),
    ((7, "v_tone3"), V_TONE3),
    ((13, "victory_tone3"), VICTORY_TONE3),
    ((7, "v_tone4"), V_TONE4),
    ((13, "victory_tone4"), VICTORY_TONE4),
    ((7, "v_tone5"), V_TONE5),
    ((13, "victory_tone5"), VICTORY_TONE5),
    ((1, "v"), V),
    ((7, "victory"), VICTORY),
    ((18, "writing_hand_tone1"), WRITING_HAND_TONE1),
    ((18, "writing_hand_tone2"), WRITING_HAND_TONE2),
    ((18, "writing_hand_tone3"), WRITING_HAND_TONE3),
    ((18, "writing_hand_tone4"), WRITING_HAND_TONE4),
    ((18, "writing_hand_tone5"), WRITING_HAND_TONE5),
    ((12, "writing_hand"), WRITING_HAND),
    ((6, "pencil"), PENCIL),
    ((9, "black_nib"), BLACK_NIB),
    ((10, "check_mark"), CHECK_MARK),
    ((16, "heavy_check_mark"), HEAVY_CHECK_MARK),
    ((14, "multiplication"), MULTIPLICATION),
    ((8, "multiply"), MULTIPLY),
    ((11, "latin_cross"), LATIN_CROSS),
    ((13, "star_of_david"), STAR_OF_DAVID),
    ((8, "sparkles"), SPARKLES),
    ((21, "eight_spoked_asterisk"), EIGHT_SPOKED_ASTERISK),
    ((24, "eight_pointed_black_star"), EIGHT_POINTED_BLACK_STAR),
    ((9, "snowflake"), SNOWFLAKE),
    ((7, "sparkle"), SPARKLE),
    ((10, "cross_mark"), CROSS_MARK),
    ((1, "x"), X),
    ((17, "cross_mark_button"), CROSS_MARK_BUTTON),
    ((27, "negative_squared_cross_mark"), NEGATIVE_SQUARED_CROSS_MARK),
    ((8, "question"), QUESTION),
    ((14, "white_question"), WHITE_QUESTION),
    ((17, "white_exclamation"), WHITE_EXCLAMATION),
    ((11, "exclamation"), EXCLAMATION),
    ((17, "heart_exclamation"), HEART_EXCLAMATION),
    ((13, "heart_on_fire"), HEART_ON_FIRE),
    ((13, "mending_heart"), MENDING_HEART),
    ((5, "heart"), HEART),
    ((9, "red_heart"), RED_HEART),
    ((4, "plus"), PLUS),
    ((5, "minus"), MINUS),
    ((6, "divide"), DIVIDE),
    ((8, "division"), DIVISION),
    ((11, "arrow_right"), ARROW_RIGHT),
    ((10, "curly_loop"), CURLY_LOOP),
    ((17, "double_curly_loop"), DOUBLE_CURLY_LOOP),
    ((4, "loop"), LOOP),
    ((16, "arrow_heading_up"), ARROW_HEADING_UP),
    ((18, "arrow_heading_down"), ARROW_HEADING_DOWN),
    ((10, "arrow_left"), ARROW_LEFT),
    ((8, "arrow_up"), ARROW_UP),
    ((10, "arrow_down"), ARROW_DOWN),
    ((18, "black_large_square"), BLACK_LARGE_SQUARE),
    ((18, "white_large_square"), WHITE_LARGE_SQUARE),
    ((4, "star"), STAR),
    ((17, "hollow_red_circle"), HOLLOW_RED_CIRCLE),
    ((5, "red_o"), RED_O),
    ((9, "wavy_dash"), WAVY_DASH),
    ((21, "part_alternation_mark"), PART_ALTERNATION_MARK),
    ((15, "congratulations"), CONGRATULATIONS),
    ((18, "ja_congratulations"), JA_CONGRATULATIONS),
    ((9, "ja_secret"), JA_SECRET),
    ((6, "secret"), SECRET),
];

#[allow(missing_docs)] // is documented by dummy macro signatures in lib.rs
#[cfg(not(doc))]
#[macro_export]
macro_rules! png_twemoji_asset_from_name {
    ("mahjong") => { &twemoji_assets::png::codes::U_1F004 };
    ("black_joker") => { &twemoji_assets::png::codes::U_1F0CF };
    ("a") => { &twemoji_assets::png::codes::U_1F170 };
    ("a_blood") => { &twemoji_assets::png::codes::U_1F170 };
    ("b") => { &twemoji_assets::png::codes::U_1F171 };
    ("b_blood") => { &twemoji_assets::png::codes::U_1F171 };
    ("o") => { &twemoji_assets::png::codes::U_1F17E };
    ("o_blood") => { &twemoji_assets::png::codes::U_1F17E };
    ("parking") => { &twemoji_assets::png::codes::U_1F17F };
    ("ab") => { &twemoji_assets::png::codes::U_1F18E };
    ("ab_blood") => { &twemoji_assets::png::codes::U_1F18E };
    ("cl") => { &twemoji_assets::png::codes::U_1F191 };
    ("cool") => { &twemoji_assets::png::codes::U_1F192 };
    ("free") => { &twemoji_assets::png::codes::U_1F193 };
    ("id") => { &twemoji_assets::png::codes::U_1F194 };
    ("new") => { &twemoji_assets::png::codes::U_1F195 };
    ("ng") => { &twemoji_assets::png::codes::U_1F196 };
    ("ok") => { &twemoji_assets::png::codes::U_1F197 };
    ("sos") => { &twemoji_assets::png::codes::U_1F198 };
    ("up2") => { &twemoji_assets::png::codes::U_1F199 };
    ("vs") => { &twemoji_assets::png::codes::U_1F19A };
    ("ascension_island") => { &twemoji_assets::png::codes::U_1F1E6_1F1E8 };
    ("flag_ac") => { &twemoji_assets::png::codes::U_1F1E6_1F1E8 };
    ("andorra") => { &twemoji_assets::png::codes::U_1F1E6_1F1E9 };
    ("flag_ad") => { &twemoji_assets::png::codes::U_1F1E6_1F1E9 };
    ("flag_ae") => { &twemoji_assets::png::codes::U_1F1E6_1F1EA };
    ("united_arab_emirates") => { &twemoji_assets::png::codes::U_1F1E6_1F1EA };
    ("afghanistan") => { &twemoji_assets::png::codes::U_1F1E6_1F1EB };
    ("flag_af") => { &twemoji_assets::png::codes::U_1F1E6_1F1EB };
    ("antigua_barbuda") => { &twemoji_assets::png::codes::U_1F1E6_1F1EC };
    ("flag_ag") => { &twemoji_assets::png::codes::U_1F1E6_1F1EC };
    ("anguilla") => { &twemoji_assets::png::codes::U_1F1E6_1F1EE };
    ("flag_ai") => { &twemoji_assets::png::codes::U_1F1E6_1F1EE };
    ("albania") => { &twemoji_assets::png::codes::U_1F1E6_1F1F1 };
    ("flag_al") => { &twemoji_assets::png::codes::U_1F1E6_1F1F1 };
    ("armenia") => { &twemoji_assets::png::codes::U_1F1E6_1F1F2 };
    ("flag_am") => { &twemoji_assets::png::codes::U_1F1E6_1F1F2 };
    ("angola") => { &twemoji_assets::png::codes::U_1F1E6_1F1F4 };
    ("flag_ao") => { &twemoji_assets::png::codes::U_1F1E6_1F1F4 };
    ("antarctica") => { &twemoji_assets::png::codes::U_1F1E6_1F1F6 };
    ("flag_aq") => { &twemoji_assets::png::codes::U_1F1E6_1F1F6 };
    ("argentina") => { &twemoji_assets::png::codes::U_1F1E6_1F1F7 };
    ("flag_ar") => { &twemoji_assets::png::codes::U_1F1E6_1F1F7 };
    ("american_samoa") => { &twemoji_assets::png::codes::U_1F1E6_1F1F8 };
    ("flag_as") => { &twemoji_assets::png::codes::U_1F1E6_1F1F8 };
    ("austria") => { &twemoji_assets::png::codes::U_1F1E6_1F1F9 };
    ("flag_at") => { &twemoji_assets::png::codes::U_1F1E6_1F1F9 };
    ("australia") => { &twemoji_assets::png::codes::U_1F1E6_1F1FA };
    ("flag_au") => { &twemoji_assets::png::codes::U_1F1E6_1F1FA };
    ("aruba") => { &twemoji_assets::png::codes::U_1F1E6_1F1FC };
    ("flag_aw") => { &twemoji_assets::png::codes::U_1F1E6_1F1FC };
    ("aland_islands") => { &twemoji_assets::png::codes::U_1F1E6_1F1FD };
    ("flag_ax") => { &twemoji_assets::png::codes::U_1F1E6_1F1FD };
    ("azerbaijan") => { &twemoji_assets::png::codes::U_1F1E6_1F1FF };
    ("flag_az") => { &twemoji_assets::png::codes::U_1F1E6_1F1FF };
    ("regional_indicator_a") => { &twemoji_assets::png::codes::U_1F1E6 };
    ("bosnia_herzegovina") => { &twemoji_assets::png::codes::U_1F1E7_1F1E6 };
    ("flag_ba") => { &twemoji_assets::png::codes::U_1F1E7_1F1E6 };
    ("barbados") => { &twemoji_assets::png::codes::U_1F1E7_1F1E7 };
    ("flag_bb") => { &twemoji_assets::png::codes::U_1F1E7_1F1E7 };
    ("bangladesh") => { &twemoji_assets::png::codes::U_1F1E7_1F1E9 };
    ("flag_bd") => { &twemoji_assets::png::codes::U_1F1E7_1F1E9 };
    ("belgium") => { &twemoji_assets::png::codes::U_1F1E7_1F1EA };
    ("flag_be") => { &twemoji_assets::png::codes::U_1F1E7_1F1EA };
    ("burkina_faso") => { &twemoji_assets::png::codes::U_1F1E7_1F1EB };
    ("flag_bf") => { &twemoji_assets::png::codes::U_1F1E7_1F1EB };
    ("bulgaria") => { &twemoji_assets::png::codes::U_1F1E7_1F1EC };
    ("flag_bg") => { &twemoji_assets::png::codes::U_1F1E7_1F1EC };
    ("bahrain") => { &twemoji_assets::png::codes::U_1F1E7_1F1ED };
    ("flag_bh") => { &twemoji_assets::png::codes::U_1F1E7_1F1ED };
    ("burundi") => { &twemoji_assets::png::codes::U_1F1E7_1F1EE };
    ("flag_bi") => { &twemoji_assets::png::codes::U_1F1E7_1F1EE };
    ("benin") => { &twemoji_assets::png::codes::U_1F1E7_1F1EF };
    ("flag_bj") => { &twemoji_assets::png::codes::U_1F1E7_1F1EF };
    ("flag_bl") => { &twemoji_assets::png::codes::U_1F1E7_1F1F1 };
    ("st_barthelemy") => { &twemoji_assets::png::codes::U_1F1E7_1F1F1 };
    ("bermuda") => { &twemoji_assets::png::codes::U_1F1E7_1F1F2 };
    ("flag_bm") => { &twemoji_assets::png::codes::U_1F1E7_1F1F2 };
    ("brunei") => { &twemoji_assets::png::codes::U_1F1E7_1F1F3 };
    ("flag_bn") => { &twemoji_assets::png::codes::U_1F1E7_1F1F3 };
    ("bolivia") => { &twemoji_assets::png::codes::U_1F1E7_1F1F4 };
    ("flag_bo") => { &twemoji_assets::png::codes::U_1F1E7_1F1F4 };
    ("caribbean_netherlands") => { &twemoji_assets::png::codes::U_1F1E7_1F1F6 };
    ("flag_bq") => { &twemoji_assets::png::codes::U_1F1E7_1F1F6 };
    ("brazil") => { &twemoji_assets::png::codes::U_1F1E7_1F1F7 };
    ("flag_br") => { &twemoji_assets::png::codes::U_1F1E7_1F1F7 };
    ("bahamas") => { &twemoji_assets::png::codes::U_1F1E7_1F1F8 };
    ("flag_bs") => { &twemoji_assets::png::codes::U_1F1E7_1F1F8 };
    ("bhutan") => { &twemoji_assets::png::codes::U_1F1E7_1F1F9 };
    ("flag_bt") => { &twemoji_assets::png::codes::U_1F1E7_1F1F9 };
    ("bouvet_island") => { &twemoji_assets::png::codes::U_1F1E7_1F1FB };
    ("flag_bv") => { &twemoji_assets::png::codes::U_1F1E7_1F1FB };
    ("botswana") => { &twemoji_assets::png::codes::U_1F1E7_1F1FC };
    ("flag_bw") => { &twemoji_assets::png::codes::U_1F1E7_1F1FC };
    ("belarus") => { &twemoji_assets::png::codes::U_1F1E7_1F1FE };
    ("flag_by") => { &twemoji_assets::png::codes::U_1F1E7_1F1FE };
    ("belize") => { &twemoji_assets::png::codes::U_1F1E7_1F1FF };
    ("flag_bz") => { &twemoji_assets::png::codes::U_1F1E7_1F1FF };
    ("regional_indicator_b") => { &twemoji_assets::png::codes::U_1F1E7 };
    ("canada") => { &twemoji_assets::png::codes::U_1F1E8_1F1E6 };
    ("flag_ca") => { &twemoji_assets::png::codes::U_1F1E8_1F1E6 };
    ("cocos_islands") => { &twemoji_assets::png::codes::U_1F1E8_1F1E8 };
    ("flag_cc") => { &twemoji_assets::png::codes::U_1F1E8_1F1E8 };
    ("congo_kinshasa") => { &twemoji_assets::png::codes::U_1F1E8_1F1E9 };
    ("flag_cd") => { &twemoji_assets::png::codes::U_1F1E8_1F1E9 };
    ("central_african_republic") => { &twemoji_assets::png::codes::U_1F1E8_1F1EB };
    ("flag_cf") => { &twemoji_assets::png::codes::U_1F1E8_1F1EB };
    ("congo_brazzaville") => { &twemoji_assets::png::codes::U_1F1E8_1F1EC };
    ("flag_cg") => { &twemoji_assets::png::codes::U_1F1E8_1F1EC };
    ("flag_ch") => { &twemoji_assets::png::codes::U_1F1E8_1F1ED };
    ("switzerland") => { &twemoji_assets::png::codes::U_1F1E8_1F1ED };
    ("cote_divoire") => { &twemoji_assets::png::codes::U_1F1E8_1F1EE };
    ("flag_ci") => { &twemoji_assets::png::codes::U_1F1E8_1F1EE };
    ("cook_islands") => { &twemoji_assets::png::codes::U_1F1E8_1F1F0 };
    ("flag_ck") => { &twemoji_assets::png::codes::U_1F1E8_1F1F0 };
    ("chile") => { &twemoji_assets::png::codes::U_1F1E8_1F1F1 };
    ("flag_cl") => { &twemoji_assets::png::codes::U_1F1E8_1F1F1 };
    ("cameroon") => { &twemoji_assets::png::codes::U_1F1E8_1F1F2 };
    ("flag_cm") => { &twemoji_assets::png::codes::U_1F1E8_1F1F2 };
    ("china") => { &twemoji_assets::png::codes::U_1F1E8_1F1F3 };
    ("flag_cn") => { &twemoji_assets::png::codes::U_1F1E8_1F1F3 };
    ("colombia") => { &twemoji_assets::png::codes::U_1F1E8_1F1F4 };
    ("flag_co") => { &twemoji_assets::png::codes::U_1F1E8_1F1F4 };
    ("clipperton_island") => { &twemoji_assets::png::codes::U_1F1E8_1F1F5 };
    ("flag_cp") => { &twemoji_assets::png::codes::U_1F1E8_1F1F5 };
    ("costa_rica") => { &twemoji_assets::png::codes::U_1F1E8_1F1F7 };
    ("flag_cr") => { &twemoji_assets::png::codes::U_1F1E8_1F1F7 };
    ("cuba") => { &twemoji_assets::png::codes::U_1F1E8_1F1FA };
    ("flag_cu") => { &twemoji_assets::png::codes::U_1F1E8_1F1FA };
    ("cape_verde") => { &twemoji_assets::png::codes::U_1F1E8_1F1FB };
    ("flag_cv") => { &twemoji_assets::png::codes::U_1F1E8_1F1FB };
    ("curacao") => { &twemoji_assets::png::codes::U_1F1E8_1F1FC };
    ("flag_cw") => { &twemoji_assets::png::codes::U_1F1E8_1F1FC };
    ("christmas_island") => { &twemoji_assets::png::codes::U_1F1E8_1F1FD };
    ("flag_cx") => { &twemoji_assets::png::codes::U_1F1E8_1F1FD };
    ("cyprus") => { &twemoji_assets::png::codes::U_1F1E8_1F1FE };
    ("flag_cy") => { &twemoji_assets::png::codes::U_1F1E8_1F1FE };
    ("czech_republic") => { &twemoji_assets::png::codes::U_1F1E8_1F1FF };
    ("czechia") => { &twemoji_assets::png::codes::U_1F1E8_1F1FF };
    ("flag_cz") => { &twemoji_assets::png::codes::U_1F1E8_1F1FF };
    ("regional_indicator_c") => { &twemoji_assets::png::codes::U_1F1E8 };
    ("flag_de") => { &twemoji_assets::png::codes::U_1F1E9_1F1EA };
    ("germany") => { &twemoji_assets::png::codes::U_1F1E9_1F1EA };
    ("diego_garcia") => { &twemoji_assets::png::codes::U_1F1E9_1F1EC };
    ("flag_dg") => { &twemoji_assets::png::codes::U_1F1E9_1F1EC };
    ("djibouti") => { &twemoji_assets::png::codes::U_1F1E9_1F1EF };
    ("flag_dj") => { &twemoji_assets::png::codes::U_1F1E9_1F1EF };
    ("denmark") => { &twemoji_assets::png::codes::U_1F1E9_1F1F0 };
    ("flag_dk") => { &twemoji_assets::png::codes::U_1F1E9_1F1F0 };
    ("dominica") => { &twemoji_assets::png::codes::U_1F1E9_1F1F2 };
    ("flag_dm") => { &twemoji_assets::png::codes::U_1F1E9_1F1F2 };
    ("dominican_republic") => { &twemoji_assets::png::codes::U_1F1E9_1F1F4 };
    ("flag_do") => { &twemoji_assets::png::codes::U_1F1E9_1F1F4 };
    ("algeria") => { &twemoji_assets::png::codes::U_1F1E9_1F1FF };
    ("flag_dz") => { &twemoji_assets::png::codes::U_1F1E9_1F1FF };
    ("regional_indicator_d") => { &twemoji_assets::png::codes::U_1F1E9 };
    ("ceuta_melilla") => { &twemoji_assets::png::codes::U_1F1EA_1F1E6 };
    ("flag_ea") => { &twemoji_assets::png::codes::U_1F1EA_1F1E6 };
    ("ecuador") => { &twemoji_assets::png::codes::U_1F1EA_1F1E8 };
    ("flag_ec") => { &twemoji_assets::png::codes::U_1F1EA_1F1E8 };
    ("estonia") => { &twemoji_assets::png::codes::U_1F1EA_1F1EA };
    ("flag_ee") => { &twemoji_assets::png::codes::U_1F1EA_1F1EA };
    ("egypt") => { &twemoji_assets::png::codes::U_1F1EA_1F1EC };
    ("flag_eg") => { &twemoji_assets::png::codes::U_1F1EA_1F1EC };
    ("flag_eh") => { &twemoji_assets::png::codes::U_1F1EA_1F1ED };
    ("western_sahara") => { &twemoji_assets::png::codes::U_1F1EA_1F1ED };
    ("eritrea") => { &twemoji_assets::png::codes::U_1F1EA_1F1F7 };
    ("flag_er") => { &twemoji_assets::png::codes::U_1F1EA_1F1F7 };
    ("flag_es") => { &twemoji_assets::png::codes::U_1F1EA_1F1F8 };
    ("spain") => { &twemoji_assets::png::codes::U_1F1EA_1F1F8 };
    ("ethiopia") => { &twemoji_assets::png::codes::U_1F1EA_1F1F9 };
    ("flag_et") => { &twemoji_assets::png::codes::U_1F1EA_1F1F9 };
    ("european_union") => { &twemoji_assets::png::codes::U_1F1EA_1F1FA };
    ("flag_eu") => { &twemoji_assets::png::codes::U_1F1EA_1F1FA };
    ("regional_indicator_e") => { &twemoji_assets::png::codes::U_1F1EA };
    ("finland") => { &twemoji_assets::png::codes::U_1F1EB_1F1EE };
    ("flag_fi") => { &twemoji_assets::png::codes::U_1F1EB_1F1EE };
    ("fiji") => { &twemoji_assets::png::codes::U_1F1EB_1F1EF };
    ("flag_fj") => { &twemoji_assets::png::codes::U_1F1EB_1F1EF };
    ("falkland_islands") => { &twemoji_assets::png::codes::U_1F1EB_1F1F0 };
    ("flag_fk") => { &twemoji_assets::png::codes::U_1F1EB_1F1F0 };
    ("flag_fm") => { &twemoji_assets::png::codes::U_1F1EB_1F1F2 };
    ("micronesia") => { &twemoji_assets::png::codes::U_1F1EB_1F1F2 };
    ("faroe_islands") => { &twemoji_assets::png::codes::U_1F1EB_1F1F4 };
    ("flag_fo") => { &twemoji_assets::png::codes::U_1F1EB_1F1F4 };
    ("flag_fr") => { &twemoji_assets::png::codes::U_1F1EB_1F1F7 };
    ("france") => { &twemoji_assets::png::codes::U_1F1EB_1F1F7 };
    ("regional_indicator_f") => { &twemoji_assets::png::codes::U_1F1EB };
    ("flag_ga") => { &twemoji_assets::png::codes::U_1F1EC_1F1E6 };
    ("gabon") => { &twemoji_assets::png::codes::U_1F1EC_1F1E6 };
    ("flag_gb") => { &twemoji_assets::png::codes::U_1F1EC_1F1E7 };
    ("uk") => { &twemoji_assets::png::codes::U_1F1EC_1F1E7 };
    ("united_kingdom") => { &twemoji_assets::png::codes::U_1F1EC_1F1E7 };
    ("flag_gd") => { &twemoji_assets::png::codes::U_1F1EC_1F1E9 };
    ("grenada") => { &twemoji_assets::png::codes::U_1F1EC_1F1E9 };
    ("flag_ge") => { &twemoji_assets::png::codes::U_1F1EC_1F1EA };
    ("georgia") => { &twemoji_assets::png::codes::U_1F1EC_1F1EA };
    ("flag_gf") => { &twemoji_assets::png::codes::U_1F1EC_1F1EB };
    ("french_guiana") => { &twemoji_assets::png::codes::U_1F1EC_1F1EB };
    ("flag_gg") => { &twemoji_assets::png::codes::U_1F1EC_1F1EC };
    ("guernsey") => { &twemoji_assets::png::codes::U_1F1EC_1F1EC };
    ("flag_gh") => { &twemoji_assets::png::codes::U_1F1EC_1F1ED };
    ("ghana") => { &twemoji_assets::png::codes::U_1F1EC_1F1ED };
    ("flag_gi") => { &twemoji_assets::png::codes::U_1F1EC_1F1EE };
    ("gibraltar") => { &twemoji_assets::png::codes::U_1F1EC_1F1EE };
    ("flag_gl") => { &twemoji_assets::png::codes::U_1F1EC_1F1F1 };
    ("greenland") => { &twemoji_assets::png::codes::U_1F1EC_1F1F1 };
    ("flag_gm") => { &twemoji_assets::png::codes::U_1F1EC_1F1F2 };
    ("gambia") => { &twemoji_assets::png::codes::U_1F1EC_1F1F2 };
    ("flag_gn") => { &twemoji_assets::png::codes::U_1F1EC_1F1F3 };
    ("guinea") => { &twemoji_assets::png::codes::U_1F1EC_1F1F3 };
    ("flag_gp") => { &twemoji_assets::png::codes::U_1F1EC_1F1F5 };
    ("guadeloupe") => { &twemoji_assets::png::codes::U_1F1EC_1F1F5 };
    ("equatorial_guinea") => { &twemoji_assets::png::codes::U_1F1EC_1F1F6 };
    ("flag_gq") => { &twemoji_assets::png::codes::U_1F1EC_1F1F6 };
    ("flag_gr") => { &twemoji_assets::png::codes::U_1F1EC_1F1F7 };
    ("greece") => { &twemoji_assets::png::codes::U_1F1EC_1F1F7 };
    ("flag_gs") => { &twemoji_assets::png::codes::U_1F1EC_1F1F8 };
    ("south_georgia_south_sandwich_islands") => { &twemoji_assets::png::codes::U_1F1EC_1F1F8 };
    ("flag_gt") => { &twemoji_assets::png::codes::U_1F1EC_1F1F9 };
    ("guatemala") => { &twemoji_assets::png::codes::U_1F1EC_1F1F9 };
    ("flag_gu") => { &twemoji_assets::png::codes::U_1F1EC_1F1FA };
    ("guam") => { &twemoji_assets::png::codes::U_1F1EC_1F1FA };
    ("flag_gw") => { &twemoji_assets::png::codes::U_1F1EC_1F1FC };
    ("guinea_bissau") => { &twemoji_assets::png::codes::U_1F1EC_1F1FC };
    ("flag_gy") => { &twemoji_assets::png::codes::U_1F1EC_1F1FE };
    ("guyana") => { &twemoji_assets::png::codes::U_1F1EC_1F1FE };
    ("regional_indicator_g") => { &twemoji_assets::png::codes::U_1F1EC };
    ("flag_hk") => { &twemoji_assets::png::codes::U_1F1ED_1F1F0 };
    ("hong_kong") => { &twemoji_assets::png::codes::U_1F1ED_1F1F0 };
    ("flag_hm") => { &twemoji_assets::png::codes::U_1F1ED_1F1F2 };
    ("heard_mcdonald_islands") => { &twemoji_assets::png::codes::U_1F1ED_1F1F2 };
    ("flag_hn") => { &twemoji_assets::png::codes::U_1F1ED_1F1F3 };
    ("honduras") => { &twemoji_assets::png::codes::U_1F1ED_1F1F3 };
    ("croatia") => { &twemoji_assets::png::codes::U_1F1ED_1F1F7 };
    ("flag_hr") => { &twemoji_assets::png::codes::U_1F1ED_1F1F7 };
    ("flag_ht") => { &twemoji_assets::png::codes::U_1F1ED_1F1F9 };
    ("haiti") => { &twemoji_assets::png::codes::U_1F1ED_1F1F9 };
    ("flag_hu") => { &twemoji_assets::png::codes::U_1F1ED_1F1FA };
    ("hungary") => { &twemoji_assets::png::codes::U_1F1ED_1F1FA };
    ("regional_indicator_h") => { &twemoji_assets::png::codes::U_1F1ED };
    ("canary_islands") => { &twemoji_assets::png::codes::U_1F1EE_1F1E8 };
    ("flag_ic") => { &twemoji_assets::png::codes::U_1F1EE_1F1E8 };
    ("flag_id") => { &twemoji_assets::png::codes::U_1F1EE_1F1E9 };
    ("indonesia") => { &twemoji_assets::png::codes::U_1F1EE_1F1E9 };
    ("flag_ie") => { &twemoji_assets::png::codes::U_1F1EE_1F1EA };
    ("ireland") => { &twemoji_assets::png::codes::U_1F1EE_1F1EA };
    ("flag_il") => { &twemoji_assets::png::codes::U_1F1EE_1F1F1 };
    ("israel") => { &twemoji_assets::png::codes::U_1F1EE_1F1F1 };
    ("flag_im") => { &twemoji_assets::png::codes::U_1F1EE_1F1F2 };
    ("isle_of_man") => { &twemoji_assets::png::codes::U_1F1EE_1F1F2 };
    ("flag_in") => { &twemoji_assets::png::codes::U_1F1EE_1F1F3 };
    ("india") => { &twemoji_assets::png::codes::U_1F1EE_1F1F3 };
    ("british_indian_ocean_territory") => { &twemoji_assets::png::codes::U_1F1EE_1F1F4 };
    ("flag_io") => { &twemoji_assets::png::codes::U_1F1EE_1F1F4 };
    ("flag_iq") => { &twemoji_assets::png::codes::U_1F1EE_1F1F6 };
    ("iraq") => { &twemoji_assets::png::codes::U_1F1EE_1F1F6 };
    ("flag_ir") => { &twemoji_assets::png::codes::U_1F1EE_1F1F7 };
    ("iran") => { &twemoji_assets::png::codes::U_1F1EE_1F1F7 };
    ("flag_is") => { &twemoji_assets::png::codes::U_1F1EE_1F1F8 };
    ("iceland") => { &twemoji_assets::png::codes::U_1F1EE_1F1F8 };
    ("flag_it") => { &twemoji_assets::png::codes::U_1F1EE_1F1F9 };
    ("italy") => { &twemoji_assets::png::codes::U_1F1EE_1F1F9 };
    ("regional_indicator_i") => { &twemoji_assets::png::codes::U_1F1EE };
    ("flag_je") => { &twemoji_assets::png::codes::U_1F1EF_1F1EA };
    ("jersey") => { &twemoji_assets::png::codes::U_1F1EF_1F1EA };
    ("flag_jm") => { &twemoji_assets::png::codes::U_1F1EF_1F1F2 };
    ("jamaica") => { &twemoji_assets::png::codes::U_1F1EF_1F1F2 };
    ("flag_jo") => { &twemoji_assets::png::codes::U_1F1EF_1F1F4 };
    ("jordan") => { &twemoji_assets::png::codes::U_1F1EF_1F1F4 };
    ("flag_jp") => { &twemoji_assets::png::codes::U_1F1EF_1F1F5 };
    ("japan") => { &twemoji_assets::png::codes::U_1F1EF_1F1F5 };
    ("regional_indicator_j") => { &twemoji_assets::png::codes::U_1F1EF };
    ("flag_ke") => { &twemoji_assets::png::codes::U_1F1F0_1F1EA };
    ("kenya") => { &twemoji_assets::png::codes::U_1F1F0_1F1EA };
    ("flag_kg") => { &twemoji_assets::png::codes::U_1F1F0_1F1EC };
    ("kyrgyzstan") => { &twemoji_assets::png::codes::U_1F1F0_1F1EC };
    ("cambodia") => { &twemoji_assets::png::codes::U_1F1F0_1F1ED };
    ("flag_kh") => { &twemoji_assets::png::codes::U_1F1F0_1F1ED };
    ("flag_ki") => { &twemoji_assets::png::codes::U_1F1F0_1F1EE };
    ("kiribati") => { &twemoji_assets::png::codes::U_1F1F0_1F1EE };
    ("comoros") => { &twemoji_assets::png::codes::U_1F1F0_1F1F2 };
    ("flag_km") => { &twemoji_assets::png::codes::U_1F1F0_1F1F2 };
    ("flag_kn") => { &twemoji_assets::png::codes::U_1F1F0_1F1F3 };
    ("st_kitts_nevis") => { &twemoji_assets::png::codes::U_1F1F0_1F1F3 };
    ("flag_kp") => { &twemoji_assets::png::codes::U_1F1F0_1F1F5 };
    ("north_korea") => { &twemoji_assets::png::codes::U_1F1F0_1F1F5 };
    ("flag_kr") => { &twemoji_assets::png::codes::U_1F1F0_1F1F7 };
    ("south_korea") => { &twemoji_assets::png::codes::U_1F1F0_1F1F7 };
    ("flag_kw") => { &twemoji_assets::png::codes::U_1F1F0_1F1FC };
    ("kuwait") => { &twemoji_assets::png::codes::U_1F1F0_1F1FC };
    ("cayman_islands") => { &twemoji_assets::png::codes::U_1F1F0_1F1FE };
    ("flag_ky") => { &twemoji_assets::png::codes::U_1F1F0_1F1FE };
    ("flag_kz") => { &twemoji_assets::png::codes::U_1F1F0_1F1FF };
    ("kazakhstan") => { &twemoji_assets::png::codes::U_1F1F0_1F1FF };
    ("regional_indicator_k") => { &twemoji_assets::png::codes::U_1F1F0 };
    ("flag_la") => { &twemoji_assets::png::codes::U_1F1F1_1F1E6 };
    ("laos") => { &twemoji_assets::png::codes::U_1F1F1_1F1E6 };
    ("flag_lb") => { &twemoji_assets::png::codes::U_1F1F1_1F1E7 };
    ("lebanon") => { &twemoji_assets::png::codes::U_1F1F1_1F1E7 };
    ("flag_lc") => { &twemoji_assets::png::codes::U_1F1F1_1F1E8 };
    ("st_lucia") => { &twemoji_assets::png::codes::U_1F1F1_1F1E8 };
    ("flag_li") => { &twemoji_assets::png::codes::U_1F1F1_1F1EE };
    ("liechtenstein") => { &twemoji_assets::png::codes::U_1F1F1_1F1EE };
    ("flag_lk") => { &twemoji_assets::png::codes::U_1F1F1_1F1F0 };
    ("sri_lanka") => { &twemoji_assets::png::codes::U_1F1F1_1F1F0 };
    ("flag_lr") => { &twemoji_assets::png::codes::U_1F1F1_1F1F7 };
    ("liberia") => { &twemoji_assets::png::codes::U_1F1F1_1F1F7 };
    ("flag_ls") => { &twemoji_assets::png::codes::U_1F1F1_1F1F8 };
    ("lesotho") => { &twemoji_assets::png::codes::U_1F1F1_1F1F8 };
    ("flag_lt") => { &twemoji_assets::png::codes::U_1F1F1_1F1F9 };
    ("lithuania") => { &twemoji_assets::png::codes::U_1F1F1_1F1F9 };
    ("flag_lu") => { &twemoji_assets::png::codes::U_1F1F1_1F1FA };
    ("luxembourg") => { &twemoji_assets::png::codes::U_1F1F1_1F1FA };
    ("flag_lv") => { &twemoji_assets::png::codes::U_1F1F1_1F1FB };
    ("latvia") => { &twemoji_assets::png::codes::U_1F1F1_1F1FB };
    ("flag_ly") => { &twemoji_assets::png::codes::U_1F1F1_1F1FE };
    ("libya") => { &twemoji_assets::png::codes::U_1F1F1_1F1FE };
    ("regional_indicator_l") => { &twemoji_assets::png::codes::U_1F1F1 };
    ("flag_ma") => { &twemoji_assets::png::codes::U_1F1F2_1F1E6 };
    ("morocco") => { &twemoji_assets::png::codes::U_1F1F2_1F1E6 };
    ("flag_mc") => { &twemoji_assets::png::codes::U_1F1F2_1F1E8 };
    ("monaco") => { &twemoji_assets::png::codes::U_1F1F2_1F1E8 };
    ("flag_md") => { &twemoji_assets::png::codes::U_1F1F2_1F1E9 };
    ("moldova") => { &twemoji_assets::png::codes::U_1F1F2_1F1E9 };
    ("flag_me") => { &twemoji_assets::png::codes::U_1F1F2_1F1EA };
    ("montenegro") => { &twemoji_assets::png::codes::U_1F1F2_1F1EA };
    ("flag_mf") => { &twemoji_assets::png::codes::U_1F1F2_1F1EB };
    ("st_martin") => { &twemoji_assets::png::codes::U_1F1F2_1F1EB };
    ("flag_mg") => { &twemoji_assets::png::codes::U_1F1F2_1F1EC };
    ("madagascar") => { &twemoji_assets::png::codes::U_1F1F2_1F1EC };
    ("flag_mh") => { &twemoji_assets::png::codes::U_1F1F2_1F1ED };
    ("marshall_islands") => { &twemoji_assets::png::codes::U_1F1F2_1F1ED };
    ("flag_mk") => { &twemoji_assets::png::codes::U_1F1F2_1F1F0 };
    ("macedonia") => { &twemoji_assets::png::codes::U_1F1F2_1F1F0 };
    ("flag_ml") => { &twemoji_assets::png::codes::U_1F1F2_1F1F1 };
    ("mali") => { &twemoji_assets::png::codes::U_1F1F2_1F1F1 };
    ("burma") => { &twemoji_assets::png::codes::U_1F1F2_1F1F2 };
    ("flag_mm") => { &twemoji_assets::png::codes::U_1F1F2_1F1F2 };
    ("myanmar") => { &twemoji_assets::png::codes::U_1F1F2_1F1F2 };
    ("flag_mn") => { &twemoji_assets::png::codes::U_1F1F2_1F1F3 };
    ("mongolia") => { &twemoji_assets::png::codes::U_1F1F2_1F1F3 };
    ("flag_mo") => { &twemoji_assets::png::codes::U_1F1F2_1F1F4 };
    ("macao") => { &twemoji_assets::png::codes::U_1F1F2_1F1F4 };
    ("macau") => { &twemoji_assets::png::codes::U_1F1F2_1F1F4 };
    ("flag_mp") => { &twemoji_assets::png::codes::U_1F1F2_1F1F5 };
    ("northern_mariana_islands") => { &twemoji_assets::png::codes::U_1F1F2_1F1F5 };
    ("flag_mq") => { &twemoji_assets::png::codes::U_1F1F2_1F1F6 };
    ("martinique") => { &twemoji_assets::png::codes::U_1F1F2_1F1F6 };
    ("flag_mr") => { &twemoji_assets::png::codes::U_1F1F2_1F1F7 };
    ("mauritania") => { &twemoji_assets::png::codes::U_1F1F2_1F1F7 };
    ("flag_ms") => { &twemoji_assets::png::codes::U_1F1F2_1F1F8 };
    ("montserrat") => { &twemoji_assets::png::codes::U_1F1F2_1F1F8 };
    ("flag_mt") => { &twemoji_assets::png::codes::U_1F1F2_1F1F9 };
    ("malta") => { &twemoji_assets::png::codes::U_1F1F2_1F1F9 };
    ("flag_mu") => { &twemoji_assets::png::codes::U_1F1F2_1F1FA };
    ("mauritius") => { &twemoji_assets::png::codes::U_1F1F2_1F1FA };
    ("flag_mv") => { &twemoji_assets::png::codes::U_1F1F2_1F1FB };
    ("maldives") => { &twemoji_assets::png::codes::U_1F1F2_1F1FB };
    ("flag_mw") => { &twemoji_assets::png::codes::U_1F1F2_1F1FC };
    ("malawi") => { &twemoji_assets::png::codes::U_1F1F2_1F1FC };
    ("flag_mx") => { &twemoji_assets::png::codes::U_1F1F2_1F1FD };
    ("mexico") => { &twemoji_assets::png::codes::U_1F1F2_1F1FD };
    ("flag_my") => { &twemoji_assets::png::codes::U_1F1F2_1F1FE };
    ("malaysia") => { &twemoji_assets::png::codes::U_1F1F2_1F1FE };
    ("flag_mz") => { &twemoji_assets::png::codes::U_1F1F2_1F1FF };
    ("mozambique") => { &twemoji_assets::png::codes::U_1F1F2_1F1FF };
    ("regional_indicator_m") => { &twemoji_assets::png::codes::U_1F1F2 };
    ("flag_na") => { &twemoji_assets::png::codes::U_1F1F3_1F1E6 };
    ("namibia") => { &twemoji_assets::png::codes::U_1F1F3_1F1E6 };
    ("flag_nc") => { &twemoji_assets::png::codes::U_1F1F3_1F1E8 };
    ("new_caledonia") => { &twemoji_assets::png::codes::U_1F1F3_1F1E8 };
    ("flag_ne") => { &twemoji_assets::png::codes::U_1F1F3_1F1EA };
    ("niger") => { &twemoji_assets::png::codes::U_1F1F3_1F1EA };
    ("flag_nf") => { &twemoji_assets::png::codes::U_1F1F3_1F1EB };
    ("norfolk_island") => { &twemoji_assets::png::codes::U_1F1F3_1F1EB };
    ("flag_ng") => { &twemoji_assets::png::codes::U_1F1F3_1F1EC };
    ("nigeria") => { &twemoji_assets::png::codes::U_1F1F3_1F1EC };
    ("flag_ni") => { &twemoji_assets::png::codes::U_1F1F3_1F1EE };
    ("nicaragua") => { &twemoji_assets::png::codes::U_1F1F3_1F1EE };
    ("flag_nl") => { &twemoji_assets::png::codes::U_1F1F3_1F1F1 };
    ("netherlands") => { &twemoji_assets::png::codes::U_1F1F3_1F1F1 };
    ("flag_no") => { &twemoji_assets::png::codes::U_1F1F3_1F1F4 };
    ("norway") => { &twemoji_assets::png::codes::U_1F1F3_1F1F4 };
    ("flag_np") => { &twemoji_assets::png::codes::U_1F1F3_1F1F5 };
    ("nepal") => { &twemoji_assets::png::codes::U_1F1F3_1F1F5 };
    ("flag_nr") => { &twemoji_assets::png::codes::U_1F1F3_1F1F7 };
    ("nauru") => { &twemoji_assets::png::codes::U_1F1F3_1F1F7 };
    ("flag_nu") => { &twemoji_assets::png::codes::U_1F1F3_1F1FA };
    ("niue") => { &twemoji_assets::png::codes::U_1F1F3_1F1FA };
    ("flag_nz") => { &twemoji_assets::png::codes::U_1F1F3_1F1FF };
    ("new_zealand") => { &twemoji_assets::png::codes::U_1F1F3_1F1FF };
    ("regional_indicator_n") => { &twemoji_assets::png::codes::U_1F1F3 };
    ("flag_om") => { &twemoji_assets::png::codes::U_1F1F4_1F1F2 };
    ("oman") => { &twemoji_assets::png::codes::U_1F1F4_1F1F2 };
    ("regional_indicator_o") => { &twemoji_assets::png::codes::U_1F1F4 };
    ("flag_pa") => { &twemoji_assets::png::codes::U_1F1F5_1F1E6 };
    ("panama") => { &twemoji_assets::png::codes::U_1F1F5_1F1E6 };
    ("flag_pe") => { &twemoji_assets::png::codes::U_1F1F5_1F1EA };
    ("peru") => { &twemoji_assets::png::codes::U_1F1F5_1F1EA };
    ("flag_pf") => { &twemoji_assets::png::codes::U_1F1F5_1F1EB };
    ("french_polynesia") => { &twemoji_assets::png::codes::U_1F1F5_1F1EB };
    ("flag_pg") => { &twemoji_assets::png::codes::U_1F1F5_1F1EC };
    ("papua_new_guinea") => { &twemoji_assets::png::codes::U_1F1F5_1F1EC };
    ("flag_ph") => { &twemoji_assets::png::codes::U_1F1F5_1F1ED };
    ("philippines") => { &twemoji_assets::png::codes::U_1F1F5_1F1ED };
    ("flag_pk") => { &twemoji_assets::png::codes::U_1F1F5_1F1F0 };
    ("pakistan") => { &twemoji_assets::png::codes::U_1F1F5_1F1F0 };
    ("flag_pl") => { &twemoji_assets::png::codes::U_1F1F5_1F1F1 };
    ("poland") => { &twemoji_assets::png::codes::U_1F1F5_1F1F1 };
    ("flag_pm") => { &twemoji_assets::png::codes::U_1F1F5_1F1F2 };
    ("st_pierre_miquelon") => { &twemoji_assets::png::codes::U_1F1F5_1F1F2 };
    ("flag_pn") => { &twemoji_assets::png::codes::U_1F1F5_1F1F3 };
    ("pitcairn_islands") => { &twemoji_assets::png::codes::U_1F1F5_1F1F3 };
    ("flag_pr") => { &twemoji_assets::png::codes::U_1F1F5_1F1F7 };
    ("puerto_rico") => { &twemoji_assets::png::codes::U_1F1F5_1F1F7 };
    ("flag_ps") => { &twemoji_assets::png::codes::U_1F1F5_1F1F8 };
    ("palestinian_territories") => { &twemoji_assets::png::codes::U_1F1F5_1F1F8 };
    ("flag_pt") => { &twemoji_assets::png::codes::U_1F1F5_1F1F9 };
    ("portugal") => { &twemoji_assets::png::codes::U_1F1F5_1F1F9 };
    ("flag_pw") => { &twemoji_assets::png::codes::U_1F1F5_1F1FC };
    ("palau") => { &twemoji_assets::png::codes::U_1F1F5_1F1FC };
    ("flag_py") => { &twemoji_assets::png::codes::U_1F1F5_1F1FE };
    ("paraguay") => { &twemoji_assets::png::codes::U_1F1F5_1F1FE };
    ("regional_indicator_p") => { &twemoji_assets::png::codes::U_1F1F5 };
    ("flag_qa") => { &twemoji_assets::png::codes::U_1F1F6_1F1E6 };
    ("qatar") => { &twemoji_assets::png::codes::U_1F1F6_1F1E6 };
    ("regional_indicator_q") => { &twemoji_assets::png::codes::U_1F1F6 };
    ("flag_re") => { &twemoji_assets::png::codes::U_1F1F7_1F1EA };
    ("reunion") => { &twemoji_assets::png::codes::U_1F1F7_1F1EA };
    ("flag_ro") => { &twemoji_assets::png::codes::U_1F1F7_1F1F4 };
    ("romania") => { &twemoji_assets::png::codes::U_1F1F7_1F1F4 };
    ("flag_rs") => { &twemoji_assets::png::codes::U_1F1F7_1F1F8 };
    ("serbia") => { &twemoji_assets::png::codes::U_1F1F7_1F1F8 };
    ("flag_ru") => { &twemoji_assets::png::codes::U_1F1F7_1F1FA };
    ("russia") => { &twemoji_assets::png::codes::U_1F1F7_1F1FA };
    ("flag_rw") => { &twemoji_assets::png::codes::U_1F1F7_1F1FC };
    ("rwanda") => { &twemoji_assets::png::codes::U_1F1F7_1F1FC };
    ("regional_indicator_r") => { &twemoji_assets::png::codes::U_1F1F7 };
    ("flag_sa") => { &twemoji_assets::png::codes::U_1F1F8_1F1E6 };
    ("saudi_arabia") => { &twemoji_assets::png::codes::U_1F1F8_1F1E6 };
    ("flag_sb") => { &twemoji_assets::png::codes::U_1F1F8_1F1E7 };
    ("solomon_islands") => { &twemoji_assets::png::codes::U_1F1F8_1F1E7 };
    ("flag_sc") => { &twemoji_assets::png::codes::U_1F1F8_1F1E8 };
    ("seychelles") => { &twemoji_assets::png::codes::U_1F1F8_1F1E8 };
    ("flag_sd") => { &twemoji_assets::png::codes::U_1F1F8_1F1E9 };
    ("sudan") => { &twemoji_assets::png::codes::U_1F1F8_1F1E9 };
    ("flag_se") => { &twemoji_assets::png::codes::U_1F1F8_1F1EA };
    ("sweden") => { &twemoji_assets::png::codes::U_1F1F8_1F1EA };
    ("flag_sg") => { &twemoji_assets::png::codes::U_1F1F8_1F1EC };
    ("singapore") => { &twemoji_assets::png::codes::U_1F1F8_1F1EC };
    ("flag_sh") => { &twemoji_assets::png::codes::U_1F1F8_1F1ED };
    ("st_helena") => { &twemoji_assets::png::codes::U_1F1F8_1F1ED };
    ("flag_si") => { &twemoji_assets::png::codes::U_1F1F8_1F1EE };
    ("slovenia") => { &twemoji_assets::png::codes::U_1F1F8_1F1EE };
    ("flag_sj") => { &twemoji_assets::png::codes::U_1F1F8_1F1EF };
    ("svalbard_jan_mayen") => { &twemoji_assets::png::codes::U_1F1F8_1F1EF };
    ("flag_sk") => { &twemoji_assets::png::codes::U_1F1F8_1F1F0 };
    ("slovakia") => { &twemoji_assets::png::codes::U_1F1F8_1F1F0 };
    ("flag_sl") => { &twemoji_assets::png::codes::U_1F1F8_1F1F1 };
    ("sierra_leone") => { &twemoji_assets::png::codes::U_1F1F8_1F1F1 };
    ("flag_sm") => { &twemoji_assets::png::codes::U_1F1F8_1F1F2 };
    ("san_marino") => { &twemoji_assets::png::codes::U_1F1F8_1F1F2 };
    ("flag_sn") => { &twemoji_assets::png::codes::U_1F1F8_1F1F3 };
    ("senegal") => { &twemoji_assets::png::codes::U_1F1F8_1F1F3 };
    ("flag_so") => { &twemoji_assets::png::codes::U_1F1F8_1F1F4 };
    ("somalia") => { &twemoji_assets::png::codes::U_1F1F8_1F1F4 };
    ("flag_sr") => { &twemoji_assets::png::codes::U_1F1F8_1F1F7 };
    ("suriname") => { &twemoji_assets::png::codes::U_1F1F8_1F1F7 };
    ("flag_ss") => { &twemoji_assets::png::codes::U_1F1F8_1F1F8 };
    ("south_sudan") => { &twemoji_assets::png::codes::U_1F1F8_1F1F8 };
    ("flag_st") => { &twemoji_assets::png::codes::U_1F1F8_1F1F9 };
    ("sao_tome_principe") => { &twemoji_assets::png::codes::U_1F1F8_1F1F9 };
    ("el_salvador") => { &twemoji_assets::png::codes::U_1F1F8_1F1FB };
    ("flag_sv") => { &twemoji_assets::png::codes::U_1F1F8_1F1FB };
    ("flag_sx") => { &twemoji_assets::png::codes::U_1F1F8_1F1FD };
    ("sint_maarten") => { &twemoji_assets::png::codes::U_1F1F8_1F1FD };
    ("flag_sy") => { &twemoji_assets::png::codes::U_1F1F8_1F1FE };
    ("syria") => { &twemoji_assets::png::codes::U_1F1F8_1F1FE };
    ("eswatini") => { &twemoji_assets::png::codes::U_1F1F8_1F1FF };
    ("flag_sz") => { &twemoji_assets::png::codes::U_1F1F8_1F1FF };
    ("swaziland") => { &twemoji_assets::png::codes::U_1F1F8_1F1FF };
    ("regional_indicator_s") => { &twemoji_assets::png::codes::U_1F1F8 };
    ("flag_ta") => { &twemoji_assets::png::codes::U_1F1F9_1F1E6 };
    ("tristan_da_cunha") => { &twemoji_assets::png::codes::U_1F1F9_1F1E6 };
    ("flag_tc") => { &twemoji_assets::png::codes::U_1F1F9_1F1E8 };
    ("turks_caicos_islands") => { &twemoji_assets::png::codes::U_1F1F9_1F1E8 };
    ("chad") => { &twemoji_assets::png::codes::U_1F1F9_1F1E9 };
    ("flag_td") => { &twemoji_assets::png::codes::U_1F1F9_1F1E9 };
    ("flag_tf") => { &twemoji_assets::png::codes::U_1F1F9_1F1EB };
    ("french_southern_territories") => { &twemoji_assets::png::codes::U_1F1F9_1F1EB };
    ("flag_tg") => { &twemoji_assets::png::codes::U_1F1F9_1F1EC };
    ("togo") => { &twemoji_assets::png::codes::U_1F1F9_1F1EC };
    ("flag_th") => { &twemoji_assets::png::codes::U_1F1F9_1F1ED };
    ("thailand") => { &twemoji_assets::png::codes::U_1F1F9_1F1ED };
    ("flag_tj") => { &twemoji_assets::png::codes::U_1F1F9_1F1EF };
    ("tajikistan") => { &twemoji_assets::png::codes::U_1F1F9_1F1EF };
    ("flag_tk") => { &twemoji_assets::png::codes::U_1F1F9_1F1F0 };
    ("tokelau") => { &twemoji_assets::png::codes::U_1F1F9_1F1F0 };
    ("flag_tl") => { &twemoji_assets::png::codes::U_1F1F9_1F1F1 };
    ("timor_leste") => { &twemoji_assets::png::codes::U_1F1F9_1F1F1 };
    ("flag_tm") => { &twemoji_assets::png::codes::U_1F1F9_1F1F2 };
    ("turkmenistan") => { &twemoji_assets::png::codes::U_1F1F9_1F1F2 };
    ("flag_tn") => { &twemoji_assets::png::codes::U_1F1F9_1F1F3 };
    ("tunisia") => { &twemoji_assets::png::codes::U_1F1F9_1F1F3 };
    ("flag_to") => { &twemoji_assets::png::codes::U_1F1F9_1F1F4 };
    ("tonga") => { &twemoji_assets::png::codes::U_1F1F9_1F1F4 };
    ("flag_tr") => { &twemoji_assets::png::codes::U_1F1F9_1F1F7 };
    ("turkey_tr") => { &twemoji_assets::png::codes::U_1F1F9_1F1F7 };
    ("flag_tt") => { &twemoji_assets::png::codes::U_1F1F9_1F1F9 };
    ("trinidad_tobago") => { &twemoji_assets::png::codes::U_1F1F9_1F1F9 };
    ("flag_tv") => { &twemoji_assets::png::codes::U_1F1F9_1F1FB };
    ("tuvalu") => { &twemoji_assets::png::codes::U_1F1F9_1F1FB };
    ("flag_tw") => { &twemoji_assets::png::codes::U_1F1F9_1F1FC };
    ("taiwan") => { &twemoji_assets::png::codes::U_1F1F9_1F1FC };
    ("flag_tz") => { &twemoji_assets::png::codes::U_1F1F9_1F1FF };
    ("tanzania") => { &twemoji_assets::png::codes::U_1F1F9_1F1FF };
    ("regional_indicator_t") => { &twemoji_assets::png::codes::U_1F1F9 };
    ("flag_ua") => { &twemoji_assets::png::codes::U_1F1FA_1F1E6 };
    ("ukraine") => { &twemoji_assets::png::codes::U_1F1FA_1F1E6 };
    ("flag_ug") => { &twemoji_assets::png::codes::U_1F1FA_1F1EC };
    ("uganda") => { &twemoji_assets::png::codes::U_1F1FA_1F1EC };
    ("flag_um") => { &twemoji_assets::png::codes::U_1F1FA_1F1F2 };
    ("us_outlying_islands") => { &twemoji_assets::png::codes::U_1F1FA_1F1F2 };
    ("flag_un") => { &twemoji_assets::png::codes::U_1F1FA_1F1F3 };
    ("un") => { &twemoji_assets::png::codes::U_1F1FA_1F1F3 };
    ("united_nations") => { &twemoji_assets::png::codes::U_1F1FA_1F1F3 };
    ("flag_us") => { &twemoji_assets::png::codes::U_1F1FA_1F1F8 };
    ("united_states") => { &twemoji_assets::png::codes::U_1F1FA_1F1F8 };
    ("usa") => { &twemoji_assets::png::codes::U_1F1FA_1F1F8 };
    ("flag_uy") => { &twemoji_assets::png::codes::U_1F1FA_1F1FE };
    ("uruguay") => { &twemoji_assets::png::codes::U_1F1FA_1F1FE };
    ("flag_uz") => { &twemoji_assets::png::codes::U_1F1FA_1F1FF };
    ("uzbekistan") => { &twemoji_assets::png::codes::U_1F1FA_1F1FF };
    ("regional_indicator_u") => { &twemoji_assets::png::codes::U_1F1FA };
    ("flag_va") => { &twemoji_assets::png::codes::U_1F1FB_1F1E6 };
    ("vatican_city") => { &twemoji_assets::png::codes::U_1F1FB_1F1E6 };
    ("flag_vc") => { &twemoji_assets::png::codes::U_1F1FB_1F1E8 };
    ("st_vincent_grenadines") => { &twemoji_assets::png::codes::U_1F1FB_1F1E8 };
    ("flag_ve") => { &twemoji_assets::png::codes::U_1F1FB_1F1EA };
    ("venezuela") => { &twemoji_assets::png::codes::U_1F1FB_1F1EA };
    ("british_virgin_islands") => { &twemoji_assets::png::codes::U_1F1FB_1F1EC };
    ("flag_vg") => { &twemoji_assets::png::codes::U_1F1FB_1F1EC };
    ("flag_vi") => { &twemoji_assets::png::codes::U_1F1FB_1F1EE };
    ("us_virgin_islands") => { &twemoji_assets::png::codes::U_1F1FB_1F1EE };
    ("flag_vn") => { &twemoji_assets::png::codes::U_1F1FB_1F1F3 };
    ("vietnam") => { &twemoji_assets::png::codes::U_1F1FB_1F1F3 };
    ("flag_vu") => { &twemoji_assets::png::codes::U_1F1FB_1F1FA };
    ("vanuatu") => { &twemoji_assets::png::codes::U_1F1FB_1F1FA };
    ("regional_indicator_v") => { &twemoji_assets::png::codes::U_1F1FB };
    ("flag_wf") => { &twemoji_assets::png::codes::U_1F1FC_1F1EB };
    ("wallis_futuna") => { &twemoji_assets::png::codes::U_1F1FC_1F1EB };
    ("flag_ws") => { &twemoji_assets::png::codes::U_1F1FC_1F1F8 };
    ("samoa") => { &twemoji_assets::png::codes::U_1F1FC_1F1F8 };
    ("regional_indicator_w") => { &twemoji_assets::png::codes::U_1F1FC };
    ("flag_xk") => { &twemoji_assets::png::codes::U_1F1FD_1F1F0 };
    ("kosovo") => { &twemoji_assets::png::codes::U_1F1FD_1F1F0 };
    ("regional_indicator_x") => { &twemoji_assets::png::codes::U_1F1FD };
    ("flag_ye") => { &twemoji_assets::png::codes::U_1F1FE_1F1EA };
    ("yemen") => { &twemoji_assets::png::codes::U_1F1FE_1F1EA };
    ("flag_yt") => { &twemoji_assets::png::codes::U_1F1FE_1F1F9 };
    ("mayotte") => { &twemoji_assets::png::codes::U_1F1FE_1F1F9 };
    ("regional_indicator_y") => { &twemoji_assets::png::codes::U_1F1FE };
    ("flag_za") => { &twemoji_assets::png::codes::U_1F1FF_1F1E6 };
    ("south_africa") => { &twemoji_assets::png::codes::U_1F1FF_1F1E6 };
    ("flag_zm") => { &twemoji_assets::png::codes::U_1F1FF_1F1F2 };
    ("zambia") => { &twemoji_assets::png::codes::U_1F1FF_1F1F2 };
    ("flag_zw") => { &twemoji_assets::png::codes::U_1F1FF_1F1FC };
    ("zimbabwe") => { &twemoji_assets::png::codes::U_1F1FF_1F1FC };
    ("regional_indicator_z") => { &twemoji_assets::png::codes::U_1F1FF };
    ("ja_here") => { &twemoji_assets::png::codes::U_1F201 };
    ("koko") => { &twemoji_assets::png::codes::U_1F201 };
    ("ja_service_charge") => { &twemoji_assets::png::codes::U_1F202 };
    ("ja_free_of_charge") => { &twemoji_assets::png::codes::U_1F21A };
    ("ja_reserved") => { &twemoji_assets::png::codes::U_1F22F };
    ("ja_prohibited") => { &twemoji_assets::png::codes::U_1F232 };
    ("ja_vacancy") => { &twemoji_assets::png::codes::U_1F233 };
    ("ja_passing_grade") => { &twemoji_assets::png::codes::U_1F234 };
    ("ja_no_vacancy") => { &twemoji_assets::png::codes::U_1F235 };
    ("ja_not_free_of_carge") => { &twemoji_assets::png::codes::U_1F236 };
    ("ja_monthly_amount") => { &twemoji_assets::png::codes::U_1F237 };
    ("ja_application") => { &twemoji_assets::png::codes::U_1F238 };
    ("ja_discount") => { &twemoji_assets::png::codes::U_1F239 };
    ("ja_open_for_business") => { &twemoji_assets::png::codes::U_1F23A };
    ("ideograph_advantage") => { &twemoji_assets::png::codes::U_1F250 };
    ("ja_bargain") => { &twemoji_assets::png::codes::U_1F250 };
    ("accept") => { &twemoji_assets::png::codes::U_1F251 };
    ("ja_acceptable") => { &twemoji_assets::png::codes::U_1F251 };
    ("cyclone") => { &twemoji_assets::png::codes::U_1F300 };
    ("foggy") => { &twemoji_assets::png::codes::U_1F301 };
    ("closed_umbrella") => { &twemoji_assets::png::codes::U_1F302 };
    ("night_with_stars") => { &twemoji_assets::png::codes::U_1F303 };
    ("sunrise_over_mountains") => { &twemoji_assets::png::codes::U_1F304 };
    ("sunrise") => { &twemoji_assets::png::codes::U_1F305 };
    ("city_dusk") => { &twemoji_assets::png::codes::U_1F306 };
    ("city_sunrise") => { &twemoji_assets::png::codes::U_1F307 };
    ("city_sunset") => { &twemoji_assets::png::codes::U_1F307 };
    ("rainbow") => { &twemoji_assets::png::codes::U_1F308 };
    ("bridge_at_night") => { &twemoji_assets::png::codes::U_1F309 };
    ("ocean") => { &twemoji_assets::png::codes::U_1F30A };
    ("water_wave") => { &twemoji_assets::png::codes::U_1F30A };
    ("volcano") => { &twemoji_assets::png::codes::U_1F30B };
    ("milky_way") => { &twemoji_assets::png::codes::U_1F30C };
    ("earth_africa") => { &twemoji_assets::png::codes::U_1F30D };
    ("earth_europe") => { &twemoji_assets::png::codes::U_1F30D };
    ("earth_americas") => { &twemoji_assets::png::codes::U_1F30E };
    ("earth_asia") => { &twemoji_assets::png::codes::U_1F30F };
    ("globe_with_meridians") => { &twemoji_assets::png::codes::U_1F310 };
    ("new_moon") => { &twemoji_assets::png::codes::U_1F311 };
    ("waxing_crescent_moon") => { &twemoji_assets::png::codes::U_1F312 };
    ("first_quarter_moon") => { &twemoji_assets::png::codes::U_1F313 };
    ("waxing_gibbous_moon") => { &twemoji_assets::png::codes::U_1F314 };
    ("full_moon") => { &twemoji_assets::png::codes::U_1F315 };
    ("waning_gibbous_moon") => { &twemoji_assets::png::codes::U_1F316 };
    ("last_quarter_moon") => { &twemoji_assets::png::codes::U_1F317 };
    ("waning_crescent_moon") => { &twemoji_assets::png::codes::U_1F318 };
    ("crescent_moon") => { &twemoji_assets::png::codes::U_1F319 };
    ("new_moon_with_face") => { &twemoji_assets::png::codes::U_1F31A };
    ("first_quarter_moon_with_face") => { &twemoji_assets::png::codes::U_1F31B };
    ("last_quarter_moon_with_face") => { &twemoji_assets::png::codes::U_1F31C };
    ("full_moon_with_face") => { &twemoji_assets::png::codes::U_1F31D };
    ("sun_with_face") => { &twemoji_assets::png::codes::U_1F31E };
    ("glowing_star") => { &twemoji_assets::png::codes::U_1F31F };
    ("star2") => { &twemoji_assets::png::codes::U_1F31F };
    ("shooting_star") => { &twemoji_assets::png::codes::U_1F320 };
    ("stars") => { &twemoji_assets::png::codes::U_1F320 };
    ("thermometer") => { &twemoji_assets::png::codes::U_1F321 };
    ("sun_behind_small_cloud") => { &twemoji_assets::png::codes::U_1F324 };
    ("sunny") => { &twemoji_assets::png::codes::U_1F324 };
    ("cloudy") => { &twemoji_assets::png::codes::U_1F325 };
    ("sun_behind_large_cloud") => { &twemoji_assets::png::codes::U_1F325 };
    ("sun_and_rain") => { &twemoji_assets::png::codes::U_1F326 };
    ("sun_behind_rain_cloud") => { &twemoji_assets::png::codes::U_1F326 };
    ("cloud_with_rain") => { &twemoji_assets::png::codes::U_1F327 };
    ("rainy") => { &twemoji_assets::png::codes::U_1F327 };
    ("cloud_with_snow") => { &twemoji_assets::png::codes::U_1F328 };
    ("snowy") => { &twemoji_assets::png::codes::U_1F328 };
    ("cloud_with_lightning") => { &twemoji_assets::png::codes::U_1F329 };
    ("lightning") => { &twemoji_assets::png::codes::U_1F329 };
    ("tornado") => { &twemoji_assets::png::codes::U_1F32A };
    ("fog") => { &twemoji_assets::png::codes::U_1F32B };
    ("wind_blowing_face") => { &twemoji_assets::png::codes::U_1F32C };
    ("hotdog") => { &twemoji_assets::png::codes::U_1F32D };
    ("taco") => { &twemoji_assets::png::codes::U_1F32E };
    ("burrito") => { &twemoji_assets::png::codes::U_1F32F };
    ("chestnut") => { &twemoji_assets::png::codes::U_1F330 };
    ("seedling") => { &twemoji_assets::png::codes::U_1F331 };
    ("evergreen_tree") => { &twemoji_assets::png::codes::U_1F332 };
    ("deciduous_tree") => { &twemoji_assets::png::codes::U_1F333 };
    ("palm_tree") => { &twemoji_assets::png::codes::U_1F334 };
    ("cactus") => { &twemoji_assets::png::codes::U_1F335 };
    ("hot_pepper") => { &twemoji_assets::png::codes::U_1F336 };
    ("tulip") => { &twemoji_assets::png::codes::U_1F337 };
    ("cherry_blossom") => { &twemoji_assets::png::codes::U_1F338 };
    ("rose") => { &twemoji_assets::png::codes::U_1F339 };
    ("hibiscus") => { &twemoji_assets::png::codes::U_1F33A };
    ("sunflower") => { &twemoji_assets::png::codes::U_1F33B };
    ("blossom") => { &twemoji_assets::png::codes::U_1F33C };
    ("corn") => { &twemoji_assets::png::codes::U_1F33D };
    ("ear_of_corn") => { &twemoji_assets::png::codes::U_1F33D };
    ("ear_of_rice") => { &twemoji_assets::png::codes::U_1F33E };
    ("sheaf_of_rice") => { &twemoji_assets::png::codes::U_1F33E };
    ("herb") => { &twemoji_assets::png::codes::U_1F33F };
    ("four_leaf_clover") => { &twemoji_assets::png::codes::U_1F340 };
    ("maple_leaf") => { &twemoji_assets::png::codes::U_1F341 };
    ("fallen_leaf") => { &twemoji_assets::png::codes::U_1F342 };
    ("leaves") => { &twemoji_assets::png::codes::U_1F343 };
    ("mushroom") => { &twemoji_assets::png::codes::U_1F344 };
    ("tomato") => { &twemoji_assets::png::codes::U_1F345 };
    ("eggplant") => { &twemoji_assets::png::codes::U_1F346 };
    ("grapes") => { &twemoji_assets::png::codes::U_1F347 };
    ("melon") => { &twemoji_assets::png::codes::U_1F348 };
    ("watermelon") => { &twemoji_assets::png::codes::U_1F349 };
    ("orange") => { &twemoji_assets::png::codes::U_1F34A };
    ("tangerine") => { &twemoji_assets::png::codes::U_1F34A };
    ("lemon") => { &twemoji_assets::png::codes::U_1F34B };
    ("banana") => { &twemoji_assets::png::codes::U_1F34C };
    ("pineapple") => { &twemoji_assets::png::codes::U_1F34D };
    ("apple") => { &twemoji_assets::png::codes::U_1F34E };
    ("red_apple") => { &twemoji_assets::png::codes::U_1F34E };
    ("green_apple") => { &twemoji_assets::png::codes::U_1F34F };
    ("pear") => { &twemoji_assets::png::codes::U_1F350 };
    ("peach") => { &twemoji_assets::png::codes::U_1F351 };
    ("cherries") => { &twemoji_assets::png::codes::U_1F352 };
    ("strawberry") => { &twemoji_assets::png::codes::U_1F353 };
    ("hamburger") => { &twemoji_assets::png::codes::U_1F354 };
    ("pizza") => { &twemoji_assets::png::codes::U_1F355 };
    ("meat_on_bone") => { &twemoji_assets::png::codes::U_1F356 };
    ("poultry_leg") => { &twemoji_assets::png::codes::U_1F357 };
    ("rice_cracker") => { &twemoji_assets::png::codes::U_1F358 };
    ("rice_ball") => { &twemoji_assets::png::codes::U_1F359 };
    ("cooked_rice") => { &twemoji_assets::png::codes::U_1F35A };
    ("rice") => { &twemoji_assets::png::codes::U_1F35A };
    ("curry") => { &twemoji_assets::png::codes::U_1F35B };
    ("curry_rice") => { &twemoji_assets::png::codes::U_1F35B };
    ("ramen") => { &twemoji_assets::png::codes::U_1F35C };
    ("steaming_bowl") => { &twemoji_assets::png::codes::U_1F35C };
    ("spaghetti") => { &twemoji_assets::png::codes::U_1F35D };
    ("bread") => { &twemoji_assets::png::codes::U_1F35E };
    ("french_fries") => { &twemoji_assets::png::codes::U_1F35F };
    ("fries") => { &twemoji_assets::png::codes::U_1F35F };
    ("sweet_potato") => { &twemoji_assets::png::codes::U_1F360 };
    ("dango") => { &twemoji_assets::png::codes::U_1F361 };
    ("oden") => { &twemoji_assets::png::codes::U_1F362 };
    ("sushi") => { &twemoji_assets::png::codes::U_1F363 };
    ("fried_shrimp") => { &twemoji_assets::png::codes::U_1F364 };
    ("fish_cake") => { &twemoji_assets::png::codes::U_1F365 };
    ("icecream") => { &twemoji_assets::png::codes::U_1F366 };
    ("soft_serve") => { &twemoji_assets::png::codes::U_1F366 };
    ("shaved_ice") => { &twemoji_assets::png::codes::U_1F367 };
    ("ice_cream") => { &twemoji_assets::png::codes::U_1F368 };
    ("doughnut") => { &twemoji_assets::png::codes::U_1F369 };
    ("cookie") => { &twemoji_assets::png::codes::U_1F36A };
    ("chocolate_bar") => { &twemoji_assets::png::codes::U_1F36B };
    ("candy") => { &twemoji_assets::png::codes::U_1F36C };
    ("lollipop") => { &twemoji_assets::png::codes::U_1F36D };
    ("custard") => { &twemoji_assets::png::codes::U_1F36E };
    ("honey_pot") => { &twemoji_assets::png::codes::U_1F36F };
    ("cake") => { &twemoji_assets::png::codes::U_1F370 };
    ("shortcake") => { &twemoji_assets::png::codes::U_1F370 };
    ("bento") => { &twemoji_assets::png::codes::U_1F371 };
    ("bento_box") => { &twemoji_assets::png::codes::U_1F371 };
    ("pot_of_food") => { &twemoji_assets::png::codes::U_1F372 };
    ("stew") => { &twemoji_assets::png::codes::U_1F372 };
    ("cooking") => { &twemoji_assets::png::codes::U_1F373 };
    ("fried_egg") => { &twemoji_assets::png::codes::U_1F373 };
    ("fork_and_knife") => { &twemoji_assets::png::codes::U_1F374 };
    ("tea") => { &twemoji_assets::png::codes::U_1F375 };
    ("sake") => { &twemoji_assets::png::codes::U_1F376 };
    ("wine_glass") => { &twemoji_assets::png::codes::U_1F377 };
    ("cocktail") => { &twemoji_assets::png::codes::U_1F378 };
    ("tropical_drink") => { &twemoji_assets::png::codes::U_1F379 };
    ("beer") => { &twemoji_assets::png::codes::U_1F37A };
    ("beers") => { &twemoji_assets::png::codes::U_1F37B };
    ("baby_bottle") => { &twemoji_assets::png::codes::U_1F37C };
    ("fork_knife_plate") => { &twemoji_assets::png::codes::U_1F37D };
    ("champagne") => { &twemoji_assets::png::codes::U_1F37E };
    ("popcorn") => { &twemoji_assets::png::codes::U_1F37F };
    ("ribbon") => { &twemoji_assets::png::codes::U_1F380 };
    ("gift") => { &twemoji_assets::png::codes::U_1F381 };
    ("birthday") => { &twemoji_assets::png::codes::U_1F382 };
    ("birthday_cake") => { &twemoji_assets::png::codes::U_1F382 };
    ("jack_o_lantern") => { &twemoji_assets::png::codes::U_1F383 };
    ("christmas_tree") => { &twemoji_assets::png::codes::U_1F384 };
    ("santa_tone1") => { &twemoji_assets::png::codes::U_1F385_1F3FB };
    ("santa_tone2") => { &twemoji_assets::png::codes::U_1F385_1F3FC };
    ("santa_tone3") => { &twemoji_assets::png::codes::U_1F385_1F3FD };
    ("santa_tone4") => { &twemoji_assets::png::codes::U_1F385_1F3FE };
    ("santa_tone5") => { &twemoji_assets::png::codes::U_1F385_1F3FF };
    ("santa") => { &twemoji_assets::png::codes::U_1F385 };
    ("fireworks") => { &twemoji_assets::png::codes::U_1F386 };
    ("sparkler") => { &twemoji_assets::png::codes::U_1F387 };
    ("balloon") => { &twemoji_assets::png::codes::U_1F388 };
    ("party") => { &twemoji_assets::png::codes::U_1F389 };
    ("party_popper") => { &twemoji_assets::png::codes::U_1F389 };
    ("tada") => { &twemoji_assets::png::codes::U_1F389 };
    ("confetti_ball") => { &twemoji_assets::png::codes::U_1F38A };
    ("tanabata_tree") => { &twemoji_assets::png::codes::U_1F38B };
    ("crossed_flags") => { &twemoji_assets::png::codes::U_1F38C };
    ("bamboo") => { &twemoji_assets::png::codes::U_1F38D };
    ("dolls") => { &twemoji_assets::png::codes::U_1F38E };
    ("carp_streamer") => { &twemoji_assets::png::codes::U_1F38F };
    ("flags") => { &twemoji_assets::png::codes::U_1F38F };
    ("wind_chime") => { &twemoji_assets::png::codes::U_1F390 };
    ("moon_ceremony") => { &twemoji_assets::png::codes::U_1F391 };
    ("rice_scene") => { &twemoji_assets::png::codes::U_1F391 };
    ("backpack") => { &twemoji_assets::png::codes::U_1F392 };
    ("school_satchel") => { &twemoji_assets::png::codes::U_1F392 };
    ("graduation_cap") => { &twemoji_assets::png::codes::U_1F393 };
    ("mortar_board") => { &twemoji_assets::png::codes::U_1F393 };
    ("military_medal") => { &twemoji_assets::png::codes::U_1F396 };
    ("reminder_ribbon") => { &twemoji_assets::png::codes::U_1F397 };
    ("studio_microphone") => { &twemoji_assets::png::codes::U_1F399 };
    ("level_slider") => { &twemoji_assets::png::codes::U_1F39A };
    ("control_knobs") => { &twemoji_assets::png::codes::U_1F39B };
    ("film_frames") => { &twemoji_assets::png::codes::U_1F39E };
    ("admission_tickets") => { &twemoji_assets::png::codes::U_1F39F };
    ("tickets") => { &twemoji_assets::png::codes::U_1F39F };
    ("carousel_horse") => { &twemoji_assets::png::codes::U_1F3A0 };
    ("ferris_wheel") => { &twemoji_assets::png::codes::U_1F3A1 };
    ("roller_coaster") => { &twemoji_assets::png::codes::U_1F3A2 };
    ("fishing_pole") => { &twemoji_assets::png::codes::U_1F3A3 };
    ("fishing_pole_and_fish") => { &twemoji_assets::png::codes::U_1F3A3 };
    ("microphone") => { &twemoji_assets::png::codes::U_1F3A4 };
    ("movie_camera") => { &twemoji_assets::png::codes::U_1F3A5 };
    ("cinema") => { &twemoji_assets::png::codes::U_1F3A6 };
    ("headphones") => { &twemoji_assets::png::codes::U_1F3A7 };
    ("art") => { &twemoji_assets::png::codes::U_1F3A8 };
    ("palette") => { &twemoji_assets::png::codes::U_1F3A8 };
    ("top_hat") => { &twemoji_assets::png::codes::U_1F3A9 };
    ("tophat") => { &twemoji_assets::png::codes::U_1F3A9 };
    ("circus_tent") => { &twemoji_assets::png::codes::U_1F3AA };
    ("ticket") => { &twemoji_assets::png::codes::U_1F3AB };
    ("clapper") => { &twemoji_assets::png::codes::U_1F3AC };
    ("performing_arts") => { &twemoji_assets::png::codes::U_1F3AD };
    ("controller") => { &twemoji_assets::png::codes::U_1F3AE };
    ("video_game") => { &twemoji_assets::png::codes::U_1F3AE };
    ("bullseye") => { &twemoji_assets::png::codes::U_1F3AF };
    ("dart") => { &twemoji_assets::png::codes::U_1F3AF };
    ("direct_hit") => { &twemoji_assets::png::codes::U_1F3AF };
    ("slot_machine") => { &twemoji_assets::png::codes::U_1F3B0 };
    ("8ball") => { &twemoji_assets::png::codes::U_1F3B1 };
    ("billiards") => { &twemoji_assets::png::codes::U_1F3B1 };
    ("game_die") => { &twemoji_assets::png::codes::U_1F3B2 };
    ("bowling") => { &twemoji_assets::png::codes::U_1F3B3 };
    ("flower_playing_cards") => { &twemoji_assets::png::codes::U_1F3B4 };
    ("musical_note") => { &twemoji_assets::png::codes::U_1F3B5 };
    ("musical_notes") => { &twemoji_assets::png::codes::U_1F3B6 };
    ("notes") => { &twemoji_assets::png::codes::U_1F3B6 };
    ("saxophone") => { &twemoji_assets::png::codes::U_1F3B7 };
    ("guitar") => { &twemoji_assets::png::codes::U_1F3B8 };
    ("musical_keyboard") => { &twemoji_assets::png::codes::U_1F3B9 };
    ("trumpet") => { &twemoji_assets::png::codes::U_1F3BA };
    ("violin") => { &twemoji_assets::png::codes::U_1F3BB };
    ("musical_score") => { &twemoji_assets::png::codes::U_1F3BC };
    ("running_shirt") => { &twemoji_assets::png::codes::U_1F3BD };
    ("running_shirt_with_sash") => { &twemoji_assets::png::codes::U_1F3BD };
    ("tennis") => { &twemoji_assets::png::codes::U_1F3BE };
    ("ski") => { &twemoji_assets::png::codes::U_1F3BF };
    ("basketball") => { &twemoji_assets::png::codes::U_1F3C0 };
    ("checkered_flag") => { &twemoji_assets::png::codes::U_1F3C1 };
    ("person_snowboarding_tone1") => { &twemoji_assets::png::codes::U_1F3C2_1F3FB };
    ("snowboarder_tone1") => { &twemoji_assets::png::codes::U_1F3C2_1F3FB };
    ("snowboarding_tone1") => { &twemoji_assets::png::codes::U_1F3C2_1F3FB };
    ("person_snowboarding_tone2") => { &twemoji_assets::png::codes::U_1F3C2_1F3FC };
    ("snowboarder_tone2") => { &twemoji_assets::png::codes::U_1F3C2_1F3FC };
    ("snowboarding_tone2") => { &twemoji_assets::png::codes::U_1F3C2_1F3FC };
    ("person_snowboarding_tone3") => { &twemoji_assets::png::codes::U_1F3C2_1F3FD };
    ("snowboarder_tone3") => { &twemoji_assets::png::codes::U_1F3C2_1F3FD };
    ("snowboarding_tone3") => { &twemoji_assets::png::codes::U_1F3C2_1F3FD };
    ("person_snowboarding_tone4") => { &twemoji_assets::png::codes::U_1F3C2_1F3FE };
    ("snowboarder_tone4") => { &twemoji_assets::png::codes::U_1F3C2_1F3FE };
    ("snowboarding_tone4") => { &twemoji_assets::png::codes::U_1F3C2_1F3FE };
    ("person_snowboarding_tone5") => { &twemoji_assets::png::codes::U_1F3C2_1F3FF };
    ("snowboarder_tone5") => { &twemoji_assets::png::codes::U_1F3C2_1F3FF };
    ("snowboarding_tone5") => { &twemoji_assets::png::codes::U_1F3C2_1F3FF };
    ("person_snowboarding") => { &twemoji_assets::png::codes::U_1F3C2 };
    ("snowboarder") => { &twemoji_assets::png::codes::U_1F3C2 };
    ("snowboarding") => { &twemoji_assets::png::codes::U_1F3C2 };
    ("woman_running_tone1") => { &twemoji_assets::png::codes::U_1F3C3_1F3FB_200D_2640_FE0F };
    ("man_running_tone1") => { &twemoji_assets::png::codes::U_1F3C3_1F3FB_200D_2642_FE0F };
    ("person_running_tone1") => { &twemoji_assets::png::codes::U_1F3C3_1F3FB };
    ("running_tone1") => { &twemoji_assets::png::codes::U_1F3C3_1F3FB };
    ("woman_running_tone2") => { &twemoji_assets::png::codes::U_1F3C3_1F3FC_200D_2640_FE0F };
    ("man_running_tone2") => { &twemoji_assets::png::codes::U_1F3C3_1F3FC_200D_2642_FE0F };
    ("person_running_tone2") => { &twemoji_assets::png::codes::U_1F3C3_1F3FC };
    ("running_tone2") => { &twemoji_assets::png::codes::U_1F3C3_1F3FC };
    ("woman_running_tone3") => { &twemoji_assets::png::codes::U_1F3C3_1F3FD_200D_2640_FE0F };
    ("man_running_tone3") => { &twemoji_assets::png::codes::U_1F3C3_1F3FD_200D_2642_FE0F };
    ("person_running_tone3") => { &twemoji_assets::png::codes::U_1F3C3_1F3FD };
    ("running_tone3") => { &twemoji_assets::png::codes::U_1F3C3_1F3FD };
    ("woman_running_tone4") => { &twemoji_assets::png::codes::U_1F3C3_1F3FE_200D_2640_FE0F };
    ("man_running_tone4") => { &twemoji_assets::png::codes::U_1F3C3_1F3FE_200D_2642_FE0F };
    ("person_running_tone4") => { &twemoji_assets::png::codes::U_1F3C3_1F3FE };
    ("running_tone4") => { &twemoji_assets::png::codes::U_1F3C3_1F3FE };
    ("woman_running_tone5") => { &twemoji_assets::png::codes::U_1F3C3_1F3FF_200D_2640_FE0F };
    ("man_running_tone5") => { &twemoji_assets::png::codes::U_1F3C3_1F3FF_200D_2642_FE0F };
    ("person_running_tone5") => { &twemoji_assets::png::codes::U_1F3C3_1F3FF };
    ("running_tone5") => { &twemoji_assets::png::codes::U_1F3C3_1F3FF };
    ("woman_running") => { &twemoji_assets::png::codes::U_1F3C3_200D_2640_FE0F };
    ("man_running") => { &twemoji_assets::png::codes::U_1F3C3_200D_2642_FE0F };
    ("person_running") => { &twemoji_assets::png::codes::U_1F3C3 };
    ("running") => { &twemoji_assets::png::codes::U_1F3C3 };
    ("woman_surfing_tone1") => { &twemoji_assets::png::codes::U_1F3C4_1F3FB_200D_2640_FE0F };
    ("man_surfing_tone1") => { &twemoji_assets::png::codes::U_1F3C4_1F3FB_200D_2642_FE0F };
    ("person_surfing_tone1") => { &twemoji_assets::png::codes::U_1F3C4_1F3FB };
    ("surfer_tone1") => { &twemoji_assets::png::codes::U_1F3C4_1F3FB };
    ("surfing_tone1") => { &twemoji_assets::png::codes::U_1F3C4_1F3FB };
    ("woman_surfing_tone2") => { &twemoji_assets::png::codes::U_1F3C4_1F3FC_200D_2640_FE0F };
    ("man_surfing_tone2") => { &twemoji_assets::png::codes::U_1F3C4_1F3FC_200D_2642_FE0F };
    ("person_surfing_tone2") => { &twemoji_assets::png::codes::U_1F3C4_1F3FC };
    ("surfer_tone2") => { &twemoji_assets::png::codes::U_1F3C4_1F3FC };
    ("surfing_tone2") => { &twemoji_assets::png::codes::U_1F3C4_1F3FC };
    ("woman_surfing_tone3") => { &twemoji_assets::png::codes::U_1F3C4_1F3FD_200D_2640_FE0F };
    ("man_surfing_tone3") => { &twemoji_assets::png::codes::U_1F3C4_1F3FD_200D_2642_FE0F };
    ("person_surfing_tone3") => { &twemoji_assets::png::codes::U_1F3C4_1F3FD };
    ("surfer_tone3") => { &twemoji_assets::png::codes::U_1F3C4_1F3FD };
    ("surfing_tone3") => { &twemoji_assets::png::codes::U_1F3C4_1F3FD };
    ("woman_surfing_tone4") => { &twemoji_assets::png::codes::U_1F3C4_1F3FE_200D_2640_FE0F };
    ("man_surfing_tone4") => { &twemoji_assets::png::codes::U_1F3C4_1F3FE_200D_2642_FE0F };
    ("person_surfing_tone4") => { &twemoji_assets::png::codes::U_1F3C4_1F3FE };
    ("surfer_tone4") => { &twemoji_assets::png::codes::U_1F3C4_1F3FE };
    ("surfing_tone4") => { &twemoji_assets::png::codes::U_1F3C4_1F3FE };
    ("woman_surfing_tone5") => { &twemoji_assets::png::codes::U_1F3C4_1F3FF_200D_2640_FE0F };
    ("man_surfing_tone5") => { &twemoji_assets::png::codes::U_1F3C4_1F3FF_200D_2642_FE0F };
    ("person_surfing_tone5") => { &twemoji_assets::png::codes::U_1F3C4_1F3FF };
    ("surfer_tone5") => { &twemoji_assets::png::codes::U_1F3C4_1F3FF };
    ("surfing_tone5") => { &twemoji_assets::png::codes::U_1F3C4_1F3FF };
    ("woman_surfing") => { &twemoji_assets::png::codes::U_1F3C4_200D_2640_FE0F };
    ("man_surfing") => { &twemoji_assets::png::codes::U_1F3C4_200D_2642_FE0F };
    ("person_surfing") => { &twemoji_assets::png::codes::U_1F3C4 };
    ("surfer") => { &twemoji_assets::png::codes::U_1F3C4 };
    ("surfing") => { &twemoji_assets::png::codes::U_1F3C4 };
    ("sports_medal") => { &twemoji_assets::png::codes::U_1F3C5 };
    ("trophy") => { &twemoji_assets::png::codes::U_1F3C6 };
    ("horse_racing_tone1") => { &twemoji_assets::png::codes::U_1F3C7_1F3FB };
    ("horse_racing_tone2") => { &twemoji_assets::png::codes::U_1F3C7_1F3FC };
    ("horse_racing_tone3") => { &twemoji_assets::png::codes::U_1F3C7_1F3FD };
    ("horse_racing_tone4") => { &twemoji_assets::png::codes::U_1F3C7_1F3FE };
    ("horse_racing_tone5") => { &twemoji_assets::png::codes::U_1F3C7_1F3FF };
    ("horse_racing") => { &twemoji_assets::png::codes::U_1F3C7 };
    ("football") => { &twemoji_assets::png::codes::U_1F3C8 };
    ("rugby_football") => { &twemoji_assets::png::codes::U_1F3C9 };
    ("woman_swimming_tone1") => { &twemoji_assets::png::codes::U_1F3CA_1F3FB_200D_2640_FE0F };
    ("man_swimming_tone1") => { &twemoji_assets::png::codes::U_1F3CA_1F3FB_200D_2642_FE0F };
    ("person_swimming_tone1") => { &twemoji_assets::png::codes::U_1F3CA_1F3FB };
    ("swimmer_tone1") => { &twemoji_assets::png::codes::U_1F3CA_1F3FB };
    ("swimming_tone1") => { &twemoji_assets::png::codes::U_1F3CA_1F3FB };
    ("woman_swimming_tone2") => { &twemoji_assets::png::codes::U_1F3CA_1F3FC_200D_2640_FE0F };
    ("man_swimming_tone2") => { &twemoji_assets::png::codes::U_1F3CA_1F3FC_200D_2642_FE0F };
    ("person_swimming_tone2") => { &twemoji_assets::png::codes::U_1F3CA_1F3FC };
    ("swimmer_tone2") => { &twemoji_assets::png::codes::U_1F3CA_1F3FC };
    ("swimming_tone2") => { &twemoji_assets::png::codes::U_1F3CA_1F3FC };
    ("woman_swimming_tone3") => { &twemoji_assets::png::codes::U_1F3CA_1F3FD_200D_2640_FE0F };
    ("man_swimming_tone3") => { &twemoji_assets::png::codes::U_1F3CA_1F3FD_200D_2642_FE0F };
    ("person_swimming_tone3") => { &twemoji_assets::png::codes::U_1F3CA_1F3FD };
    ("swimmer_tone3") => { &twemoji_assets::png::codes::U_1F3CA_1F3FD };
    ("swimming_tone3") => { &twemoji_assets::png::codes::U_1F3CA_1F3FD };
    ("woman_swimming_tone4") => { &twemoji_assets::png::codes::U_1F3CA_1F3FE_200D_2640_FE0F };
    ("man_swimming_tone4") => { &twemoji_assets::png::codes::U_1F3CA_1F3FE_200D_2642_FE0F };
    ("person_swimming_tone4") => { &twemoji_assets::png::codes::U_1F3CA_1F3FE };
    ("swimmer_tone4") => { &twemoji_assets::png::codes::U_1F3CA_1F3FE };
    ("swimming_tone4") => { &twemoji_assets::png::codes::U_1F3CA_1F3FE };
    ("woman_swimming_tone5") => { &twemoji_assets::png::codes::U_1F3CA_1F3FF_200D_2640_FE0F };
    ("man_swimming_tone5") => { &twemoji_assets::png::codes::U_1F3CA_1F3FF_200D_2642_FE0F };
    ("person_swimming_tone5") => { &twemoji_assets::png::codes::U_1F3CA_1F3FF };
    ("swimmer_tone5") => { &twemoji_assets::png::codes::U_1F3CA_1F3FF };
    ("swimming_tone5") => { &twemoji_assets::png::codes::U_1F3CA_1F3FF };
    ("woman_swimming") => { &twemoji_assets::png::codes::U_1F3CA_200D_2640_FE0F };
    ("man_swimming") => { &twemoji_assets::png::codes::U_1F3CA_200D_2642_FE0F };
    ("person_swimming") => { &twemoji_assets::png::codes::U_1F3CA };
    ("swimmer") => { &twemoji_assets::png::codes::U_1F3CA };
    ("swimming") => { &twemoji_assets::png::codes::U_1F3CA };
    ("woman_lifting_weights_tone1") => { &twemoji_assets::png::codes::U_1F3CB_1F3FB_200D_2640_FE0F };
    ("man_lifting_weights_tone1") => { &twemoji_assets::png::codes::U_1F3CB_1F3FB_200D_2642_FE0F };
    ("person_lifting_weights_tone1") => { &twemoji_assets::png::codes::U_1F3CB_1F3FB };
    ("weight_lifter_tone1") => { &twemoji_assets::png::codes::U_1F3CB_1F3FB };
    ("weight_lifting_tone1") => { &twemoji_assets::png::codes::U_1F3CB_1F3FB };
    ("woman_lifting_weights_tone2") => { &twemoji_assets::png::codes::U_1F3CB_1F3FC_200D_2640_FE0F };
    ("man_lifting_weights_tone2") => { &twemoji_assets::png::codes::U_1F3CB_1F3FC_200D_2642_FE0F };
    ("person_lifting_weights_tone2") => { &twemoji_assets::png::codes::U_1F3CB_1F3FC };
    ("weight_lifter_tone2") => { &twemoji_assets::png::codes::U_1F3CB_1F3FC };
    ("weight_lifting_tone2") => { &twemoji_assets::png::codes::U_1F3CB_1F3FC };
    ("woman_lifting_weights_tone3") => { &twemoji_assets::png::codes::U_1F3CB_1F3FD_200D_2640_FE0F };
    ("man_lifting_weights_tone3") => { &twemoji_assets::png::codes::U_1F3CB_1F3FD_200D_2642_FE0F };
    ("person_lifting_weights_tone3") => { &twemoji_assets::png::codes::U_1F3CB_1F3FD };
    ("weight_lifter_tone3") => { &twemoji_assets::png::codes::U_1F3CB_1F3FD };
    ("weight_lifting_tone3") => { &twemoji_assets::png::codes::U_1F3CB_1F3FD };
    ("woman_lifting_weights_tone4") => { &twemoji_assets::png::codes::U_1F3CB_1F3FE_200D_2640_FE0F };
    ("man_lifting_weights_tone4") => { &twemoji_assets::png::codes::U_1F3CB_1F3FE_200D_2642_FE0F };
    ("person_lifting_weights_tone4") => { &twemoji_assets::png::codes::U_1F3CB_1F3FE };
    ("weight_lifter_tone4") => { &twemoji_assets::png::codes::U_1F3CB_1F3FE };
    ("weight_lifting_tone4") => { &twemoji_assets::png::codes::U_1F3CB_1F3FE };
    ("woman_lifting_weights_tone5") => { &twemoji_assets::png::codes::U_1F3CB_1F3FF_200D_2640_FE0F };
    ("man_lifting_weights_tone5") => { &twemoji_assets::png::codes::U_1F3CB_1F3FF_200D_2642_FE0F };
    ("person_lifting_weights_tone5") => { &twemoji_assets::png::codes::U_1F3CB_1F3FF };
    ("weight_lifter_tone5") => { &twemoji_assets::png::codes::U_1F3CB_1F3FF };
    ("weight_lifting_tone5") => { &twemoji_assets::png::codes::U_1F3CB_1F3FF };
    ("woman_lifting_weights") => { &twemoji_assets::png::codes::U_1F3CB_FE0F_200D_2640_FE0F };
    ("man_lifting_weights") => { &twemoji_assets::png::codes::U_1F3CB_FE0F_200D_2642_FE0F };
    ("person_lifting_weights") => { &twemoji_assets::png::codes::U_1F3CB };
    ("weight_lifter") => { &twemoji_assets::png::codes::U_1F3CB };
    ("weight_lifting") => { &twemoji_assets::png::codes::U_1F3CB };
    ("woman_golfing_tone1") => { &twemoji_assets::png::codes::U_1F3CC_1F3FB_200D_2640_FE0F };
    ("man_golfing_tone1") => { &twemoji_assets::png::codes::U_1F3CC_1F3FB_200D_2642_FE0F };
    ("golfer_tone1") => { &twemoji_assets::png::codes::U_1F3CC_1F3FB };
    ("golfing_tone1") => { &twemoji_assets::png::codes::U_1F3CC_1F3FB };
    ("person_golfing_tone1") => { &twemoji_assets::png::codes::U_1F3CC_1F3FB };
    ("woman_golfing_tone2") => { &twemoji_assets::png::codes::U_1F3CC_1F3FC_200D_2640_FE0F };
    ("man_golfing_tone2") => { &twemoji_assets::png::codes::U_1F3CC_1F3FC_200D_2642_FE0F };
    ("golfer_tone2") => { &twemoji_assets::png::codes::U_1F3CC_1F3FC };
    ("golfing_tone2") => { &twemoji_assets::png::codes::U_1F3CC_1F3FC };
    ("person_golfing_tone2") => { &twemoji_assets::png::codes::U_1F3CC_1F3FC };
    ("woman_golfing_tone3") => { &twemoji_assets::png::codes::U_1F3CC_1F3FD_200D_2640_FE0F };
    ("man_golfing_tone3") => { &twemoji_assets::png::codes::U_1F3CC_1F3FD_200D_2642_FE0F };
    ("golfer_tone3") => { &twemoji_assets::png::codes::U_1F3CC_1F3FD };
    ("golfing_tone3") => { &twemoji_assets::png::codes::U_1F3CC_1F3FD };
    ("person_golfing_tone3") => { &twemoji_assets::png::codes::U_1F3CC_1F3FD };
    ("woman_golfing_tone4") => { &twemoji_assets::png::codes::U_1F3CC_1F3FE_200D_2640_FE0F };
    ("man_golfing_tone4") => { &twemoji_assets::png::codes::U_1F3CC_1F3FE_200D_2642_FE0F };
    ("golfer_tone4") => { &twemoji_assets::png::codes::U_1F3CC_1F3FE };
    ("golfing_tone4") => { &twemoji_assets::png::codes::U_1F3CC_1F3FE };
    ("person_golfing_tone4") => { &twemoji_assets::png::codes::U_1F3CC_1F3FE };
    ("woman_golfing_tone5") => { &twemoji_assets::png::codes::U_1F3CC_1F3FF_200D_2640_FE0F };
    ("man_golfing_tone5") => { &twemoji_assets::png::codes::U_1F3CC_1F3FF_200D_2642_FE0F };
    ("golfer_tone5") => { &twemoji_assets::png::codes::U_1F3CC_1F3FF };
    ("golfing_tone5") => { &twemoji_assets::png::codes::U_1F3CC_1F3FF };
    ("person_golfing_tone5") => { &twemoji_assets::png::codes::U_1F3CC_1F3FF };
    ("woman_golfing") => { &twemoji_assets::png::codes::U_1F3CC_FE0F_200D_2640_FE0F };
    ("man_golfing") => { &twemoji_assets::png::codes::U_1F3CC_FE0F_200D_2642_FE0F };
    ("golfer") => { &twemoji_assets::png::codes::U_1F3CC };
    ("golfing") => { &twemoji_assets::png::codes::U_1F3CC };
    ("person_golfing") => { &twemoji_assets::png::codes::U_1F3CC };
    ("motorcycle") => { &twemoji_assets::png::codes::U_1F3CD };
    ("racing_car") => { &twemoji_assets::png::codes::U_1F3CE };
    ("cricket_game") => { &twemoji_assets::png::codes::U_1F3CF };
    ("volleyball") => { &twemoji_assets::png::codes::U_1F3D0 };
    ("field_hockey") => { &twemoji_assets::png::codes::U_1F3D1 };
    ("hockey") => { &twemoji_assets::png::codes::U_1F3D2 };
    ("ping_pong") => { &twemoji_assets::png::codes::U_1F3D3 };
    ("mountain_snow") => { &twemoji_assets::png::codes::U_1F3D4 };
    ("camping") => { &twemoji_assets::png::codes::U_1F3D5 };
    ("beach") => { &twemoji_assets::png::codes::U_1F3D6 };
    ("beach_with_umbrella") => { &twemoji_assets::png::codes::U_1F3D6 };
    ("building_construction") => { &twemoji_assets::png::codes::U_1F3D7 };
    ("construction_site") => { &twemoji_assets::png::codes::U_1F3D7 };
    ("homes") => { &twemoji_assets::png::codes::U_1F3D8 };
    ("houses") => { &twemoji_assets::png::codes::U_1F3D8 };
    ("cityscape") => { &twemoji_assets::png::codes::U_1F3D9 };
    ("derelict_house") => { &twemoji_assets::png::codes::U_1F3DA };
    ("house_abandoned") => { &twemoji_assets::png::codes::U_1F3DA };
    ("classical_building") => { &twemoji_assets::png::codes::U_1F3DB };
    ("desert") => { &twemoji_assets::png::codes::U_1F3DC };
    ("desert_island") => { &twemoji_assets::png::codes::U_1F3DD };
    ("island") => { &twemoji_assets::png::codes::U_1F3DD };
    ("national_park") => { &twemoji_assets::png::codes::U_1F3DE };
    ("stadium") => { &twemoji_assets::png::codes::U_1F3DF };
    ("house") => { &twemoji_assets::png::codes::U_1F3E0 };
    ("house_with_garden") => { &twemoji_assets::png::codes::U_1F3E1 };
    ("office") => { &twemoji_assets::png::codes::U_1F3E2 };
    ("post_office") => { &twemoji_assets::png::codes::U_1F3E3 };
    ("european_post_office") => { &twemoji_assets::png::codes::U_1F3E4 };
    ("hospital") => { &twemoji_assets::png::codes::U_1F3E5 };
    ("bank") => { &twemoji_assets::png::codes::U_1F3E6 };
    ("atm") => { &twemoji_assets::png::codes::U_1F3E7 };
    ("hotel") => { &twemoji_assets::png::codes::U_1F3E8 };
    ("love_hotel") => { &twemoji_assets::png::codes::U_1F3E9 };
    ("convenience_store") => { &twemoji_assets::png::codes::U_1F3EA };
    ("school") => { &twemoji_assets::png::codes::U_1F3EB };
    ("department_store") => { &twemoji_assets::png::codes::U_1F3EC };
    ("factory") => { &twemoji_assets::png::codes::U_1F3ED };
    ("izakaya_lantern") => { &twemoji_assets::png::codes::U_1F3EE };
    ("red_paper_lantern") => { &twemoji_assets::png::codes::U_1F3EE };
    ("japanese_castle") => { &twemoji_assets::png::codes::U_1F3EF };
    ("castle") => { &twemoji_assets::png::codes::U_1F3F0 };
    ("european_castle") => { &twemoji_assets::png::codes::U_1F3F0 };
    ("rainbow_flag") => { &twemoji_assets::png::codes::U_1F3F3_FE0F_200D_1F308 };
    ("transgender_flag") => { &twemoji_assets::png::codes::U_1F3F3_FE0F_200D_26A7_FE0F };
    ("white_flag") => { &twemoji_assets::png::codes::U_1F3F3 };
    ("jolly_roger") => { &twemoji_assets::png::codes::U_1F3F4_200D_2620_FE0F };
    ("pirate_flag") => { &twemoji_assets::png::codes::U_1F3F4_200D_2620_FE0F };
    ("england") => { &twemoji_assets::png::codes::U_1F3F4_E0067_E0062_E0065_E006E_E0067_E007F };
    ("flag_gbeng") => { &twemoji_assets::png::codes::U_1F3F4_E0067_E0062_E0065_E006E_E0067_E007F };
    ("flag_gbsct") => { &twemoji_assets::png::codes::U_1F3F4_E0067_E0062_E0073_E0063_E0074_E007F };
    ("scotland") => { &twemoji_assets::png::codes::U_1F3F4_E0067_E0062_E0073_E0063_E0074_E007F };
    ("flag_gbwls") => { &twemoji_assets::png::codes::U_1F3F4_E0067_E0062_E0077_E006C_E0073_E007F };
    ("wales") => { &twemoji_assets::png::codes::U_1F3F4_E0067_E0062_E0077_E006C_E0073_E007F };
    ("black_flag") => { &twemoji_assets::png::codes::U_1F3F4 };
    ("rosette") => { &twemoji_assets::png::codes::U_1F3F5 };
    ("label") => { &twemoji_assets::png::codes::U_1F3F7 };
    ("badminton") => { &twemoji_assets::png::codes::U_1F3F8 };
    ("bow_and_arrow") => { &twemoji_assets::png::codes::U_1F3F9 };
    ("amphora") => { &twemoji_assets::png::codes::U_1F3FA };
    ("tone1") => { &twemoji_assets::png::codes::U_1F3FB };
    ("tone_light") => { &twemoji_assets::png::codes::U_1F3FB };
    ("tone2") => { &twemoji_assets::png::codes::U_1F3FC };
    ("tone_medium_light") => { &twemoji_assets::png::codes::U_1F3FC };
    ("tone3") => { &twemoji_assets::png::codes::U_1F3FD };
    ("tone_medium") => { &twemoji_assets::png::codes::U_1F3FD };
    ("tone4") => { &twemoji_assets::png::codes::U_1F3FE };
    ("tone_medium_dark") => { &twemoji_assets::png::codes::U_1F3FE };
    ("tone5") => { &twemoji_assets::png::codes::U_1F3FF };
    ("tone_dark") => { &twemoji_assets::png::codes::U_1F3FF };
    ("rat") => { &twemoji_assets::png::codes::U_1F400 };
    ("mouse") => { &twemoji_assets::png::codes::U_1F401 };
    ("ox") => { &twemoji_assets::png::codes::U_1F402 };
    ("water_buffalo") => { &twemoji_assets::png::codes::U_1F403 };
    ("cow") => { &twemoji_assets::png::codes::U_1F404 };
    ("tiger") => { &twemoji_assets::png::codes::U_1F405 };
    ("leopard") => { &twemoji_assets::png::codes::U_1F406 };
    ("rabbit") => { &twemoji_assets::png::codes::U_1F407 };
    ("black_cat") => { &twemoji_assets::png::codes::U_1F408_200D_2B1B };
    ("cat") => { &twemoji_assets::png::codes::U_1F408 };
    ("dragon") => { &twemoji_assets::png::codes::U_1F409 };
    ("crocodile") => { &twemoji_assets::png::codes::U_1F40A };
    ("whale") => { &twemoji_assets::png::codes::U_1F40B };
    ("snail") => { &twemoji_assets::png::codes::U_1F40C };
    ("snake") => { &twemoji_assets::png::codes::U_1F40D };
    ("horse") => { &twemoji_assets::png::codes::U_1F40E };
    ("racehorse") => { &twemoji_assets::png::codes::U_1F40E };
    ("ram") => { &twemoji_assets::png::codes::U_1F40F };
    ("goat") => { &twemoji_assets::png::codes::U_1F410 };
    ("ewe") => { &twemoji_assets::png::codes::U_1F411 };
    ("sheep") => { &twemoji_assets::png::codes::U_1F411 };
    ("monkey") => { &twemoji_assets::png::codes::U_1F412 };
    ("rooster") => { &twemoji_assets::png::codes::U_1F413 };
    ("chicken") => { &twemoji_assets::png::codes::U_1F414 };
    ("chicken_face") => { &twemoji_assets::png::codes::U_1F414 };
    ("service_dog") => { &twemoji_assets::png::codes::U_1F415_200D_1F9BA };
    ("dog") => { &twemoji_assets::png::codes::U_1F415 };
    ("pig") => { &twemoji_assets::png::codes::U_1F416 };
    ("boar") => { &twemoji_assets::png::codes::U_1F417 };
    ("elephant") => { &twemoji_assets::png::codes::U_1F418 };
    ("octopus") => { &twemoji_assets::png::codes::U_1F419 };
    ("shell") => { &twemoji_assets::png::codes::U_1F41A };
    ("bug") => { &twemoji_assets::png::codes::U_1F41B };
    ("ant") => { &twemoji_assets::png::codes::U_1F41C };
    ("bee") => { &twemoji_assets::png::codes::U_1F41D };
    ("lady_beetle") => { &twemoji_assets::png::codes::U_1F41E };
    ("fish") => { &twemoji_assets::png::codes::U_1F41F };
    ("tropical_fish") => { &twemoji_assets::png::codes::U_1F420 };
    ("blowfish") => { &twemoji_assets::png::codes::U_1F421 };
    ("turtle") => { &twemoji_assets::png::codes::U_1F422 };
    ("hatching_chick") => { &twemoji_assets::png::codes::U_1F423 };
    ("baby_chick") => { &twemoji_assets::png::codes::U_1F424 };
    ("hatched_chick") => { &twemoji_assets::png::codes::U_1F425 };
    ("bird") => { &twemoji_assets::png::codes::U_1F426 };
    ("bird_face") => { &twemoji_assets::png::codes::U_1F426 };
    ("penguin") => { &twemoji_assets::png::codes::U_1F427 };
    ("penguin_face") => { &twemoji_assets::png::codes::U_1F427 };
    ("koala") => { &twemoji_assets::png::codes::U_1F428 };
    ("koala_face") => { &twemoji_assets::png::codes::U_1F428 };
    ("poodle") => { &twemoji_assets::png::codes::U_1F429 };
    ("dromedary_camel") => { &twemoji_assets::png::codes::U_1F42A };
    ("camel") => { &twemoji_assets::png::codes::U_1F42B };
    ("dolphin") => { &twemoji_assets::png::codes::U_1F42C };
    ("mouse_face") => { &twemoji_assets::png::codes::U_1F42D };
    ("cow_face") => { &twemoji_assets::png::codes::U_1F42E };
    ("tiger_face") => { &twemoji_assets::png::codes::U_1F42F };
    ("rabbit_face") => { &twemoji_assets::png::codes::U_1F430 };
    ("cat_face") => { &twemoji_assets::png::codes::U_1F431 };
    ("dragon_face") => { &twemoji_assets::png::codes::U_1F432 };
    ("spouting_whale") => { &twemoji_assets::png::codes::U_1F433 };
    ("horse_face") => { &twemoji_assets::png::codes::U_1F434 };
    ("monkey_face") => { &twemoji_assets::png::codes::U_1F435 };
    ("dog_face") => { &twemoji_assets::png::codes::U_1F436 };
    ("pig_face") => { &twemoji_assets::png::codes::U_1F437 };
    ("frog") => { &twemoji_assets::png::codes::U_1F438 };
    ("frog_face") => { &twemoji_assets::png::codes::U_1F438 };
    ("hamster") => { &twemoji_assets::png::codes::U_1F439 };
    ("hamster_face") => { &twemoji_assets::png::codes::U_1F439 };
    ("wolf") => { &twemoji_assets::png::codes::U_1F43A };
    ("wolf_face") => { &twemoji_assets::png::codes::U_1F43A };
    ("polar_bear") => { &twemoji_assets::png::codes::U_1F43B_200D_2744_FE0F };
    ("polar_bear_face") => { &twemoji_assets::png::codes::U_1F43B_200D_2744_FE0F };
    ("bear") => { &twemoji_assets::png::codes::U_1F43B };
    ("bear_face") => { &twemoji_assets::png::codes::U_1F43B };
    ("panda") => { &twemoji_assets::png::codes::U_1F43C };
    ("panda_face") => { &twemoji_assets::png::codes::U_1F43C };
    ("pig_nose") => { &twemoji_assets::png::codes::U_1F43D };
    ("paw_prints") => { &twemoji_assets::png::codes::U_1F43E };
    ("chipmunk") => { &twemoji_assets::png::codes::U_1F43F };
    ("eyes") => { &twemoji_assets::png::codes::U_1F440 };
    ("eye") => { &twemoji_assets::png::codes::U_1F441 };
    ("ear_tone1") => { &twemoji_assets::png::codes::U_1F442_1F3FB };
    ("ear_tone2") => { &twemoji_assets::png::codes::U_1F442_1F3FC };
    ("ear_tone3") => { &twemoji_assets::png::codes::U_1F442_1F3FD };
    ("ear_tone4") => { &twemoji_assets::png::codes::U_1F442_1F3FE };
    ("ear_tone5") => { &twemoji_assets::png::codes::U_1F442_1F3FF };
    ("ear") => { &twemoji_assets::png::codes::U_1F442 };
    ("nose_tone1") => { &twemoji_assets::png::codes::U_1F443_1F3FB };
    ("nose_tone2") => { &twemoji_assets::png::codes::U_1F443_1F3FC };
    ("nose_tone3") => { &twemoji_assets::png::codes::U_1F443_1F3FD };
    ("nose_tone4") => { &twemoji_assets::png::codes::U_1F443_1F3FE };
    ("nose_tone5") => { &twemoji_assets::png::codes::U_1F443_1F3FF };
    ("nose") => { &twemoji_assets::png::codes::U_1F443 };
    ("lips") => { &twemoji_assets::png::codes::U_1F444 };
    ("mouth") => { &twemoji_assets::png::codes::U_1F444 };
    ("tongue") => { &twemoji_assets::png::codes::U_1F445 };
    ("point_up_tone1") => { &twemoji_assets::png::codes::U_1F446_1F3FB };
    ("point_up_tone2") => { &twemoji_assets::png::codes::U_1F446_1F3FC };
    ("point_up_tone3") => { &twemoji_assets::png::codes::U_1F446_1F3FD };
    ("point_up_tone4") => { &twemoji_assets::png::codes::U_1F446_1F3FE };
    ("point_up_tone5") => { &twemoji_assets::png::codes::U_1F446_1F3FF };
    ("point_up") => { &twemoji_assets::png::codes::U_1F446 };
    ("point_down_tone1") => { &twemoji_assets::png::codes::U_1F447_1F3FB };
    ("point_down_tone2") => { &twemoji_assets::png::codes::U_1F447_1F3FC };
    ("point_down_tone3") => { &twemoji_assets::png::codes::U_1F447_1F3FD };
    ("point_down_tone4") => { &twemoji_assets::png::codes::U_1F447_1F3FE };
    ("point_down_tone5") => { &twemoji_assets::png::codes::U_1F447_1F3FF };
    ("point_down") => { &twemoji_assets::png::codes::U_1F447 };
    ("point_left_tone1") => { &twemoji_assets::png::codes::U_1F448_1F3FB };
    ("point_left_tone2") => { &twemoji_assets::png::codes::U_1F448_1F3FC };
    ("point_left_tone3") => { &twemoji_assets::png::codes::U_1F448_1F3FD };
    ("point_left_tone4") => { &twemoji_assets::png::codes::U_1F448_1F3FE };
    ("point_left_tone5") => { &twemoji_assets::png::codes::U_1F448_1F3FF };
    ("point_left") => { &twemoji_assets::png::codes::U_1F448 };
    ("point_right_tone1") => { &twemoji_assets::png::codes::U_1F449_1F3FB };
    ("point_right_tone2") => { &twemoji_assets::png::codes::U_1F449_1F3FC };
    ("point_right_tone3") => { &twemoji_assets::png::codes::U_1F449_1F3FD };
    ("point_right_tone4") => { &twemoji_assets::png::codes::U_1F449_1F3FE };
    ("point_right_tone5") => { &twemoji_assets::png::codes::U_1F449_1F3FF };
    ("point_right") => { &twemoji_assets::png::codes::U_1F449 };
    ("punch_tone1") => { &twemoji_assets::png::codes::U_1F44A_1F3FB };
    ("punch_tone2") => { &twemoji_assets::png::codes::U_1F44A_1F3FC };
    ("punch_tone3") => { &twemoji_assets::png::codes::U_1F44A_1F3FD };
    ("punch_tone4") => { &twemoji_assets::png::codes::U_1F44A_1F3FE };
    ("punch_tone5") => { &twemoji_assets::png::codes::U_1F44A_1F3FF };
    ("punch") => { &twemoji_assets::png::codes::U_1F44A };
    ("wave_tone1") => { &twemoji_assets::png::codes::U_1F44B_1F3FB };
    ("waving_hand_tone1") => { &twemoji_assets::png::codes::U_1F44B_1F3FB };
    ("wave_tone2") => { &twemoji_assets::png::codes::U_1F44B_1F3FC };
    ("waving_hand_tone2") => { &twemoji_assets::png::codes::U_1F44B_1F3FC };
    ("wave_tone3") => { &twemoji_assets::png::codes::U_1F44B_1F3FD };
    ("waving_hand_tone3") => { &twemoji_assets::png::codes::U_1F44B_1F3FD };
    ("wave_tone4") => { &twemoji_assets::png::codes::U_1F44B_1F3FE };
    ("waving_hand_tone4") => { &twemoji_assets::png::codes::U_1F44B_1F3FE };
    ("wave_tone5") => { &twemoji_assets::png::codes::U_1F44B_1F3FF };
    ("waving_hand_tone5") => { &twemoji_assets::png::codes::U_1F44B_1F3FF };
    ("wave") => { &twemoji_assets::png::codes::U_1F44B };
    ("waving_hand") => { &twemoji_assets::png::codes::U_1F44B };
    ("ok_hand_tone1") => { &twemoji_assets::png::codes::U_1F44C_1F3FB };
    ("ok_hand_tone2") => { &twemoji_assets::png::codes::U_1F44C_1F3FC };
    ("ok_hand_tone3") => { &twemoji_assets::png::codes::U_1F44C_1F3FD };
    ("ok_hand_tone4") => { &twemoji_assets::png::codes::U_1F44C_1F3FE };
    ("ok_hand_tone5") => { &twemoji_assets::png::codes::U_1F44C_1F3FF };
    ("ok_hand") => { &twemoji_assets::png::codes::U_1F44C };
    ("+1_tone1") => { &twemoji_assets::png::codes::U_1F44D_1F3FB };
    ("thumbsup_tone1") => { &twemoji_assets::png::codes::U_1F44D_1F3FB };
    ("yes_tone1") => { &twemoji_assets::png::codes::U_1F44D_1F3FB };
    ("+1_tone2") => { &twemoji_assets::png::codes::U_1F44D_1F3FC };
    ("thumbsup_tone2") => { &twemoji_assets::png::codes::U_1F44D_1F3FC };
    ("yes_tone2") => { &twemoji_assets::png::codes::U_1F44D_1F3FC };
    ("+1_tone3") => { &twemoji_assets::png::codes::U_1F44D_1F3FD };
    ("thumbsup_tone3") => { &twemoji_assets::png::codes::U_1F44D_1F3FD };
    ("yes_tone3") => { &twemoji_assets::png::codes::U_1F44D_1F3FD };
    ("+1_tone4") => { &twemoji_assets::png::codes::U_1F44D_1F3FE };
    ("thumbsup_tone4") => { &twemoji_assets::png::codes::U_1F44D_1F3FE };
    ("yes_tone4") => { &twemoji_assets::png::codes::U_1F44D_1F3FE };
    ("+1_tone5") => { &twemoji_assets::png::codes::U_1F44D_1F3FF };
    ("thumbsup_tone5") => { &twemoji_assets::png::codes::U_1F44D_1F3FF };
    ("yes_tone5") => { &twemoji_assets::png::codes::U_1F44D_1F3FF };
    ("+1") => { &twemoji_assets::png::codes::U_1F44D };
    ("thumbsup") => { &twemoji_assets::png::codes::U_1F44D };
    ("yes") => { &twemoji_assets::png::codes::U_1F44D };
    ("-1_tone1") => { &twemoji_assets::png::codes::U_1F44E_1F3FB };
    ("no_tone1") => { &twemoji_assets::png::codes::U_1F44E_1F3FB };
    ("thumbsdown_tone1") => { &twemoji_assets::png::codes::U_1F44E_1F3FB };
    ("-1_tone2") => { &twemoji_assets::png::codes::U_1F44E_1F3FC };
    ("no_tone2") => { &twemoji_assets::png::codes::U_1F44E_1F3FC };
    ("thumbsdown_tone2") => { &twemoji_assets::png::codes::U_1F44E_1F3FC };
    ("-1_tone3") => { &twemoji_assets::png::codes::U_1F44E_1F3FD };
    ("no_tone3") => { &twemoji_assets::png::codes::U_1F44E_1F3FD };
    ("thumbsdown_tone3") => { &twemoji_assets::png::codes::U_1F44E_1F3FD };
    ("-1_tone4") => { &twemoji_assets::png::codes::U_1F44E_1F3FE };
    ("no_tone4") => { &twemoji_assets::png::codes::U_1F44E_1F3FE };
    ("thumbsdown_tone4") => { &twemoji_assets::png::codes::U_1F44E_1F3FE };
    ("-1_tone5") => { &twemoji_assets::png::codes::U_1F44E_1F3FF };
    ("no_tone5") => { &twemoji_assets::png::codes::U_1F44E_1F3FF };
    ("thumbsdown_tone5") => { &twemoji_assets::png::codes::U_1F44E_1F3FF };
    ("-1") => { &twemoji_assets::png::codes::U_1F44E };
    ("no") => { &twemoji_assets::png::codes::U_1F44E };
    ("thumbsdown") => { &twemoji_assets::png::codes::U_1F44E };
    ("clap_tone1") => { &twemoji_assets::png::codes::U_1F44F_1F3FB };
    ("clapping_hands_tone1") => { &twemoji_assets::png::codes::U_1F44F_1F3FB };
    ("clap_tone2") => { &twemoji_assets::png::codes::U_1F44F_1F3FC };
    ("clapping_hands_tone2") => { &twemoji_assets::png::codes::U_1F44F_1F3FC };
    ("clap_tone3") => { &twemoji_assets::png::codes::U_1F44F_1F3FD };
    ("clapping_hands_tone3") => { &twemoji_assets::png::codes::U_1F44F_1F3FD };
    ("clap_tone4") => { &twemoji_assets::png::codes::U_1F44F_1F3FE };
    ("clapping_hands_tone4") => { &twemoji_assets::png::codes::U_1F44F_1F3FE };
    ("clap_tone5") => { &twemoji_assets::png::codes::U_1F44F_1F3FF };
    ("clapping_hands_tone5") => { &twemoji_assets::png::codes::U_1F44F_1F3FF };
    ("clap") => { &twemoji_assets::png::codes::U_1F44F };
    ("clapping_hands") => { &twemoji_assets::png::codes::U_1F44F };
    ("open_hands_tone1") => { &twemoji_assets::png::codes::U_1F450_1F3FB };
    ("open_hands_tone2") => { &twemoji_assets::png::codes::U_1F450_1F3FC };
    ("open_hands_tone3") => { &twemoji_assets::png::codes::U_1F450_1F3FD };
    ("open_hands_tone4") => { &twemoji_assets::png::codes::U_1F450_1F3FE };
    ("open_hands_tone5") => { &twemoji_assets::png::codes::U_1F450_1F3FF };
    ("open_hands") => { &twemoji_assets::png::codes::U_1F450 };
    ("crown") => { &twemoji_assets::png::codes::U_1F451 };
    ("womans_hat") => { &twemoji_assets::png::codes::U_1F452 };
    ("eyeglasses") => { &twemoji_assets::png::codes::U_1F453 };
    ("glasses") => { &twemoji_assets::png::codes::U_1F453 };
    ("necktie") => { &twemoji_assets::png::codes::U_1F454 };
    ("shirt") => { &twemoji_assets::png::codes::U_1F455 };
    ("jeans") => { &twemoji_assets::png::codes::U_1F456 };
    ("dress") => { &twemoji_assets::png::codes::U_1F457 };
    ("kimono") => { &twemoji_assets::png::codes::U_1F458 };
    ("bikini") => { &twemoji_assets::png::codes::U_1F459 };
    ("womans_clothes") => { &twemoji_assets::png::codes::U_1F45A };
    ("purse") => { &twemoji_assets::png::codes::U_1F45B };
    ("handbag") => { &twemoji_assets::png::codes::U_1F45C };
    ("clutch_bag") => { &twemoji_assets::png::codes::U_1F45D };
    ("pouch") => { &twemoji_assets::png::codes::U_1F45D };
    ("mans_shoe") => { &twemoji_assets::png::codes::U_1F45E };
    ("athletic_shoe") => { &twemoji_assets::png::codes::U_1F45F };
    ("sneaker") => { &twemoji_assets::png::codes::U_1F45F };
    ("high_heel") => { &twemoji_assets::png::codes::U_1F460 };
    ("sandal") => { &twemoji_assets::png::codes::U_1F461 };
    ("boot") => { &twemoji_assets::png::codes::U_1F462 };
    ("footprints") => { &twemoji_assets::png::codes::U_1F463 };
    ("bust_in_silhouette") => { &twemoji_assets::png::codes::U_1F464 };
    ("busts_in_silhouette") => { &twemoji_assets::png::codes::U_1F465 };
    ("boy_tone1") => { &twemoji_assets::png::codes::U_1F466_1F3FB };
    ("boy_tone2") => { &twemoji_assets::png::codes::U_1F466_1F3FC };
    ("boy_tone3") => { &twemoji_assets::png::codes::U_1F466_1F3FD };
    ("boy_tone4") => { &twemoji_assets::png::codes::U_1F466_1F3FE };
    ("boy_tone5") => { &twemoji_assets::png::codes::U_1F466_1F3FF };
    ("boy") => { &twemoji_assets::png::codes::U_1F466 };
    ("girl_tone1") => { &twemoji_assets::png::codes::U_1F467_1F3FB };
    ("girl_tone2") => { &twemoji_assets::png::codes::U_1F467_1F3FC };
    ("girl_tone3") => { &twemoji_assets::png::codes::U_1F467_1F3FD };
    ("girl_tone4") => { &twemoji_assets::png::codes::U_1F467_1F3FE };
    ("girl_tone5") => { &twemoji_assets::png::codes::U_1F467_1F3FF };
    ("girl") => { &twemoji_assets::png::codes::U_1F467 };
    ("man_farmer_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F33E };
    ("man_cook_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F373 };
    ("man_feeding_baby_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F37C };
    ("man_student_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F393 };
    ("man_singer_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F3A4 };
    ("man_artist_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F3A8 };
    ("man_teacher_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F3EB };
    ("man_factory_worker_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F3ED };
    ("man_technologist_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F4BB };
    ("man_office_worker_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F4BC };
    ("man_mechanic_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F527 };
    ("man_scientist_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F52C };
    ("man_astronaut_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F680 };
    ("man_firefighter_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F692 };
    ("two_men_holding_hands_tone1-2") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F91D_200D_1F468_1F3FC };
    ("two_men_holding_hands_tone1-3") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F91D_200D_1F468_1F3FD };
    ("two_men_holding_hands_tone1-4") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F91D_200D_1F468_1F3FE };
    ("two_men_holding_hands_tone1-5") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F91D_200D_1F468_1F3FF };
    ("man_with_probing_cane_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F9AF };
    ("man_with_white_cane_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F9AF };
    ("man_red_haired_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F9B0 };
    ("man_curly_haired_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F9B1 };
    ("man_bald_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F9B2 };
    ("man_white_haired_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F9B3 };
    ("man_in_motorized_wheelchair_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F9BC };
    ("man_in_manual_wheelchair_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_1F9BD };
    ("man_health_worker_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2695_FE0F };
    ("man_judge_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2696_FE0F };
    ("man_pilot_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2708_FE0F };
    ("couple_with_heart_mm_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_mm_tone1-2") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_mm_tone1-3") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_mm_tone1-4") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_mm_tone1-5") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2764_FE0F_200D_1F468_1F3FF };
    ("kiss_mm_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_mm_tone1-2") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_mm_tone1-3") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_mm_tone1-4") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_mm_tone1-5") => { &twemoji_assets::png::codes::U_1F468_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("man_tone1") => { &twemoji_assets::png::codes::U_1F468_1F3FB };
    ("man_farmer_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F33E };
    ("man_cook_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F373 };
    ("man_feeding_baby_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F37C };
    ("man_student_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F393 };
    ("man_singer_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F3A4 };
    ("man_artist_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F3A8 };
    ("man_teacher_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F3EB };
    ("man_factory_worker_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F3ED };
    ("man_technologist_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F4BB };
    ("man_office_worker_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F4BC };
    ("man_mechanic_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F527 };
    ("man_scientist_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F52C };
    ("man_astronaut_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F680 };
    ("man_firefighter_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F692 };
    ("two_men_holding_hands_tone2-1") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F91D_200D_1F468_1F3FB };
    ("two_men_holding_hands_tone2-3") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F91D_200D_1F468_1F3FD };
    ("two_men_holding_hands_tone2-4") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F91D_200D_1F468_1F3FE };
    ("two_men_holding_hands_tone2-5") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F91D_200D_1F468_1F3FF };
    ("man_with_probing_cane_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F9AF };
    ("man_with_white_cane_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F9AF };
    ("man_red_haired_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F9B0 };
    ("man_curly_haired_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F9B1 };
    ("man_bald_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F9B2 };
    ("man_white_haired_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F9B3 };
    ("man_in_motorized_wheelchair_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F9BC };
    ("man_in_manual_wheelchair_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_1F9BD };
    ("man_health_worker_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2695_FE0F };
    ("man_judge_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2696_FE0F };
    ("man_pilot_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2708_FE0F };
    ("couple_with_heart_mm_tone2-1") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_mm_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_mm_tone2-3") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_mm_tone2-4") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_mm_tone2-5") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2764_FE0F_200D_1F468_1F3FF };
    ("kiss_mm_tone2-1") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_mm_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_mm_tone2-3") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_mm_tone2-4") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_mm_tone2-5") => { &twemoji_assets::png::codes::U_1F468_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("man_tone2") => { &twemoji_assets::png::codes::U_1F468_1F3FC };
    ("man_farmer_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F33E };
    ("man_cook_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F373 };
    ("man_feeding_baby_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F37C };
    ("man_student_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F393 };
    ("man_singer_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F3A4 };
    ("man_artist_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F3A8 };
    ("man_teacher_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F3EB };
    ("man_factory_worker_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F3ED };
    ("man_technologist_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F4BB };
    ("man_office_worker_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F4BC };
    ("man_mechanic_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F527 };
    ("man_scientist_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F52C };
    ("man_astronaut_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F680 };
    ("man_firefighter_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F692 };
    ("two_men_holding_hands_tone3-1") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F91D_200D_1F468_1F3FB };
    ("two_men_holding_hands_tone3-2") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F91D_200D_1F468_1F3FC };
    ("two_men_holding_hands_tone3-4") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F91D_200D_1F468_1F3FE };
    ("two_men_holding_hands_tone3-5") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F91D_200D_1F468_1F3FF };
    ("man_with_probing_cane_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F9AF };
    ("man_with_white_cane_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F9AF };
    ("man_red_haired_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F9B0 };
    ("man_curly_haired_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F9B1 };
    ("man_bald_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F9B2 };
    ("man_white_haired_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F9B3 };
    ("man_in_motorized_wheelchair_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F9BC };
    ("man_in_manual_wheelchair_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_1F9BD };
    ("man_health_worker_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2695_FE0F };
    ("man_judge_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2696_FE0F };
    ("man_pilot_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2708_FE0F };
    ("couple_with_heart_mm_tone3-1") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_mm_tone3-2") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_mm_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_mm_tone3-4") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_mm_tone3-5") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2764_FE0F_200D_1F468_1F3FF };
    ("kiss_mm_tone3-1") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_mm_tone3-2") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_mm_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_mm_tone3-4") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_mm_tone3-5") => { &twemoji_assets::png::codes::U_1F468_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("man_tone3") => { &twemoji_assets::png::codes::U_1F468_1F3FD };
    ("man_farmer_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F33E };
    ("man_cook_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F373 };
    ("man_feeding_baby_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F37C };
    ("man_student_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F393 };
    ("man_singer_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F3A4 };
    ("man_artist_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F3A8 };
    ("man_teacher_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F3EB };
    ("man_factory_worker_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F3ED };
    ("man_technologist_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F4BB };
    ("man_office_worker_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F4BC };
    ("man_mechanic_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F527 };
    ("man_scientist_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F52C };
    ("man_astronaut_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F680 };
    ("man_firefighter_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F692 };
    ("two_men_holding_hands_tone4-1") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F91D_200D_1F468_1F3FB };
    ("two_men_holding_hands_tone4-2") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F91D_200D_1F468_1F3FC };
    ("two_men_holding_hands_tone4-3") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F91D_200D_1F468_1F3FD };
    ("two_men_holding_hands_tone4-5") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F91D_200D_1F468_1F3FF };
    ("man_with_probing_cane_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F9AF };
    ("man_with_white_cane_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F9AF };
    ("man_red_haired_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F9B0 };
    ("man_curly_haired_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F9B1 };
    ("man_bald_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F9B2 };
    ("man_white_haired_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F9B3 };
    ("man_in_motorized_wheelchair_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F9BC };
    ("man_in_manual_wheelchair_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_1F9BD };
    ("man_health_worker_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2695_FE0F };
    ("man_judge_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2696_FE0F };
    ("man_pilot_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2708_FE0F };
    ("couple_with_heart_mm_tone4-1") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_mm_tone4-2") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_mm_tone4-3") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_mm_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_mm_tone4-5") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2764_FE0F_200D_1F468_1F3FF };
    ("kiss_mm_tone4-1") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_mm_tone4-2") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_mm_tone4-3") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_mm_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_mm_tone4-5") => { &twemoji_assets::png::codes::U_1F468_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("man_tone4") => { &twemoji_assets::png::codes::U_1F468_1F3FE };
    ("man_farmer_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F33E };
    ("man_cook_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F373 };
    ("man_feeding_baby_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F37C };
    ("man_student_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F393 };
    ("man_singer_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F3A4 };
    ("man_artist_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F3A8 };
    ("man_teacher_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F3EB };
    ("man_factory_worker_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F3ED };
    ("man_technologist_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F4BB };
    ("man_office_worker_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F4BC };
    ("man_mechanic_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F527 };
    ("man_scientist_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F52C };
    ("man_astronaut_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F680 };
    ("man_firefighter_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F692 };
    ("two_men_holding_hands_tone5-1") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F91D_200D_1F468_1F3FB };
    ("two_men_holding_hands_tone5-2") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F91D_200D_1F468_1F3FC };
    ("two_men_holding_hands_tone5-3") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F91D_200D_1F468_1F3FD };
    ("two_men_holding_hands_tone5-4") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F91D_200D_1F468_1F3FE };
    ("man_with_probing_cane_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F9AF };
    ("man_with_white_cane_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F9AF };
    ("man_red_haired_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F9B0 };
    ("man_curly_haired_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F9B1 };
    ("man_bald_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F9B2 };
    ("man_white_haired_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F9B3 };
    ("man_in_motorized_wheelchair_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F9BC };
    ("man_in_manual_wheelchair_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_1F9BD };
    ("man_health_worker_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2695_FE0F };
    ("man_judge_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2696_FE0F };
    ("man_pilot_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2708_FE0F };
    ("couple_with_heart_mm_tone5-1") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_mm_tone5-2") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_mm_tone5-3") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_mm_tone5-4") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_mm_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2764_FE0F_200D_1F468_1F3FF };
    ("kiss_mm_tone5-1") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_mm_tone5-2") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_mm_tone5-3") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_mm_tone5-4") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_mm_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("man_tone5") => { &twemoji_assets::png::codes::U_1F468_1F3FF };
    ("man_farmer") => { &twemoji_assets::png::codes::U_1F468_200D_1F33E };
    ("man_cook") => { &twemoji_assets::png::codes::U_1F468_200D_1F373 };
    ("man_feeding_baby") => { &twemoji_assets::png::codes::U_1F468_200D_1F37C };
    ("man_student") => { &twemoji_assets::png::codes::U_1F468_200D_1F393 };
    ("man_singer") => { &twemoji_assets::png::codes::U_1F468_200D_1F3A4 };
    ("man_artist") => { &twemoji_assets::png::codes::U_1F468_200D_1F3A8 };
    ("man_teacher") => { &twemoji_assets::png::codes::U_1F468_200D_1F3EB };
    ("man_factory_worker") => { &twemoji_assets::png::codes::U_1F468_200D_1F3ED };
    ("family_mbb") => { &twemoji_assets::png::codes::U_1F468_200D_1F466_200D_1F466 };
    ("family_mb") => { &twemoji_assets::png::codes::U_1F468_200D_1F466 };
    ("family_mgb") => { &twemoji_assets::png::codes::U_1F468_200D_1F467_200D_1F466 };
    ("family_mgg") => { &twemoji_assets::png::codes::U_1F468_200D_1F467_200D_1F467 };
    ("family_mg") => { &twemoji_assets::png::codes::U_1F468_200D_1F467 };
    ("family_mmbb") => { &twemoji_assets::png::codes::U_1F468_200D_1F468_200D_1F466_200D_1F466 };
    ("family_mmb") => { &twemoji_assets::png::codes::U_1F468_200D_1F468_200D_1F466 };
    ("family_mmgb") => { &twemoji_assets::png::codes::U_1F468_200D_1F468_200D_1F467_200D_1F466 };
    ("family_mmgg") => { &twemoji_assets::png::codes::U_1F468_200D_1F468_200D_1F467_200D_1F467 };
    ("family_mmg") => { &twemoji_assets::png::codes::U_1F468_200D_1F468_200D_1F467 };
    ("family_mwbb") => { &twemoji_assets::png::codes::U_1F468_200D_1F469_200D_1F466_200D_1F466 };
    ("family_mwb") => { &twemoji_assets::png::codes::U_1F468_200D_1F469_200D_1F466 };
    ("family_mwgb") => { &twemoji_assets::png::codes::U_1F468_200D_1F469_200D_1F467_200D_1F466 };
    ("family_mwgg") => { &twemoji_assets::png::codes::U_1F468_200D_1F469_200D_1F467_200D_1F467 };
    ("family_mwg") => { &twemoji_assets::png::codes::U_1F468_200D_1F469_200D_1F467 };
    ("man_technologist") => { &twemoji_assets::png::codes::U_1F468_200D_1F4BB };
    ("man_office_worker") => { &twemoji_assets::png::codes::U_1F468_200D_1F4BC };
    ("man_mechanic") => { &twemoji_assets::png::codes::U_1F468_200D_1F527 };
    ("man_scientist") => { &twemoji_assets::png::codes::U_1F468_200D_1F52C };
    ("man_astronaut") => { &twemoji_assets::png::codes::U_1F468_200D_1F680 };
    ("man_firefighter") => { &twemoji_assets::png::codes::U_1F468_200D_1F692 };
    ("man_with_probing_cane") => { &twemoji_assets::png::codes::U_1F468_200D_1F9AF };
    ("man_with_white_cane") => { &twemoji_assets::png::codes::U_1F468_200D_1F9AF };
    ("man_red_haired") => { &twemoji_assets::png::codes::U_1F468_200D_1F9B0 };
    ("man_curly_haired") => { &twemoji_assets::png::codes::U_1F468_200D_1F9B1 };
    ("man_bald") => { &twemoji_assets::png::codes::U_1F468_200D_1F9B2 };
    ("man_white_haired") => { &twemoji_assets::png::codes::U_1F468_200D_1F9B3 };
    ("man_in_motorized_wheelchair") => { &twemoji_assets::png::codes::U_1F468_200D_1F9BC };
    ("man_in_manual_wheelchair") => { &twemoji_assets::png::codes::U_1F468_200D_1F9BD };
    ("man_health_worker") => { &twemoji_assets::png::codes::U_1F468_200D_2695_FE0F };
    ("man_judge") => { &twemoji_assets::png::codes::U_1F468_200D_2696_FE0F };
    ("man_pilot") => { &twemoji_assets::png::codes::U_1F468_200D_2708_FE0F };
    ("couple_with_heart_mm") => { &twemoji_assets::png::codes::U_1F468_200D_2764_FE0F_200D_1F468 };
    ("kiss_mm") => { &twemoji_assets::png::codes::U_1F468_200D_2764_FE0F_200D_1F48B_200D_1F468 };
    ("man") => { &twemoji_assets::png::codes::U_1F468 };
    ("woman_farmer_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F33E };
    ("woman_cook_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F373 };
    ("woman_feeding_baby_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F37C };
    ("woman_student_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F393 };
    ("woman_singer_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F3A4 };
    ("woman_artist_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F3A8 };
    ("woman_teacher_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F3EB };
    ("woman_factory_worker_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F3ED };
    ("woman_technologist_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F4BB };
    ("woman_office_worker_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F4BC };
    ("woman_mechanic_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F527 };
    ("woman_scientist_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F52C };
    ("woman_astronaut_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F680 };
    ("woman_firefighter_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F692 };
    ("couple_tone1-2") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F91D_200D_1F468_1F3FC };
    ("couple_tone1-3") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F91D_200D_1F468_1F3FD };
    ("couple_tone1-4") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F91D_200D_1F468_1F3FE };
    ("couple_tone1-5") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F91D_200D_1F468_1F3FF };
    ("two_women_holding_hands_tone1-2") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F91D_200D_1F469_1F3FC };
    ("two_women_holding_hands_tone1-3") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F91D_200D_1F469_1F3FD };
    ("two_women_holding_hands_tone1-4") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F91D_200D_1F469_1F3FE };
    ("two_women_holding_hands_tone1-5") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F91D_200D_1F469_1F3FF };
    ("woman_with_probing_cane_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F9AF };
    ("woman_with_white_cane_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F9AF };
    ("woman_red_haired_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F9B0 };
    ("woman_curly_haired_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F9B1 };
    ("woman_bald_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F9B2 };
    ("woman_white_haired_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F9B3 };
    ("woman_in_motorized_wheelchair_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F9BC };
    ("woman_in_manual_wheelchair_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_1F9BD };
    ("woman_health_worker_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2695_FE0F };
    ("woman_judge_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2696_FE0F };
    ("woman_pilot_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2708_FE0F };
    ("couple_with_heart_mw_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_wm_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_mw_tone1-2") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_wm_tone1-2") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_mw_tone1-3") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_wm_tone1-3") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_mw_tone1-4") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_wm_tone1-4") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_mw_tone1-5") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FF };
    ("couple_with_heart_wm_tone1-5") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F468_1F3FF };
    ("couple_with_heart_ww_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F469_1F3FB };
    ("couple_with_heart_ww_tone1-2") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F469_1F3FC };
    ("couple_with_heart_ww_tone1-3") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F469_1F3FD };
    ("couple_with_heart_ww_tone1-4") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F469_1F3FE };
    ("couple_with_heart_ww_tone1-5") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F469_1F3FF };
    ("kiss_mw_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_wm_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_mw_tone1-2") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_wm_tone1-2") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_mw_tone1-3") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_wm_tone1-3") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_mw_tone1-4") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_wm_tone1-4") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_mw_tone1-5") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("kiss_wm_tone1-5") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("kiss_ww_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FB };
    ("kiss_ww_tone1-2") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FC };
    ("kiss_ww_tone1-3") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FD };
    ("kiss_ww_tone1-4") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FE };
    ("kiss_ww_tone1-5") => { &twemoji_assets::png::codes::U_1F469_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FF };
    ("woman_tone1") => { &twemoji_assets::png::codes::U_1F469_1F3FB };
    ("woman_farmer_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F33E };
    ("woman_cook_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F373 };
    ("woman_feeding_baby_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F37C };
    ("woman_student_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F393 };
    ("woman_singer_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F3A4 };
    ("woman_artist_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F3A8 };
    ("woman_teacher_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F3EB };
    ("woman_factory_worker_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F3ED };
    ("woman_technologist_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F4BB };
    ("woman_office_worker_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F4BC };
    ("woman_mechanic_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F527 };
    ("woman_scientist_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F52C };
    ("woman_astronaut_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F680 };
    ("woman_firefighter_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F692 };
    ("couple_tone2-1") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F91D_200D_1F468_1F3FB };
    ("couple_tone2-3") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F91D_200D_1F468_1F3FD };
    ("couple_tone2-4") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F91D_200D_1F468_1F3FE };
    ("couple_tone2-5") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F91D_200D_1F468_1F3FF };
    ("two_women_holding_hands_tone2-1") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F91D_200D_1F469_1F3FB };
    ("two_women_holding_hands_tone2-3") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F91D_200D_1F469_1F3FD };
    ("two_women_holding_hands_tone2-4") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F91D_200D_1F469_1F3FE };
    ("two_women_holding_hands_tone2-5") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F91D_200D_1F469_1F3FF };
    ("woman_with_probing_cane_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F9AF };
    ("woman_with_white_cane_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F9AF };
    ("woman_red_haired_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F9B0 };
    ("woman_curly_haired_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F9B1 };
    ("woman_bald_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F9B2 };
    ("woman_white_haired_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F9B3 };
    ("woman_in_motorized_wheelchair_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F9BC };
    ("woman_in_manual_wheelchair_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_1F9BD };
    ("woman_health_worker_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2695_FE0F };
    ("woman_judge_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2696_FE0F };
    ("woman_pilot_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2708_FE0F };
    ("couple_with_heart_mw_tone2-1") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_wm_tone2-1") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_mw_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_wm_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_mw_tone2-3") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_wm_tone2-3") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_mw_tone2-4") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_wm_tone2-4") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_mw_tone2-5") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FF };
    ("couple_with_heart_wm_tone2-5") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F468_1F3FF };
    ("couple_with_heart_ww_tone2-1") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F469_1F3FB };
    ("couple_with_heart_ww_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F469_1F3FC };
    ("couple_with_heart_ww_tone2-3") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F469_1F3FD };
    ("couple_with_heart_ww_tone2-4") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F469_1F3FE };
    ("couple_with_heart_ww_tone2-5") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F469_1F3FF };
    ("kiss_mw_tone2-1") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_wm_tone2-1") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_mw_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_wm_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_mw_tone2-3") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_wm_tone2-3") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_mw_tone2-4") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_wm_tone2-4") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_mw_tone2-5") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("kiss_wm_tone2-5") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("kiss_ww_tone2-1") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FB };
    ("kiss_ww_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FC };
    ("kiss_ww_tone2-3") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FD };
    ("kiss_ww_tone2-4") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FE };
    ("kiss_ww_tone2-5") => { &twemoji_assets::png::codes::U_1F469_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FF };
    ("woman_tone2") => { &twemoji_assets::png::codes::U_1F469_1F3FC };
    ("woman_farmer_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F33E };
    ("woman_cook_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F373 };
    ("woman_feeding_baby_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F37C };
    ("woman_student_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F393 };
    ("woman_singer_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F3A4 };
    ("woman_artist_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F3A8 };
    ("woman_teacher_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F3EB };
    ("woman_factory_worker_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F3ED };
    ("woman_technologist_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F4BB };
    ("woman_office_worker_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F4BC };
    ("woman_mechanic_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F527 };
    ("woman_scientist_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F52C };
    ("woman_astronaut_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F680 };
    ("woman_firefighter_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F692 };
    ("couple_tone3-1") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F91D_200D_1F468_1F3FB };
    ("couple_tone3-2") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F91D_200D_1F468_1F3FC };
    ("couple_tone3-4") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F91D_200D_1F468_1F3FE };
    ("couple_tone3-5") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F91D_200D_1F468_1F3FF };
    ("two_women_holding_hands_tone3-1") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F91D_200D_1F469_1F3FB };
    ("two_women_holding_hands_tone3-2") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F91D_200D_1F469_1F3FC };
    ("two_women_holding_hands_tone3-4") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F91D_200D_1F469_1F3FE };
    ("two_women_holding_hands_tone3-5") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F91D_200D_1F469_1F3FF };
    ("woman_with_probing_cane_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F9AF };
    ("woman_with_white_cane_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F9AF };
    ("woman_red_haired_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F9B0 };
    ("woman_curly_haired_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F9B1 };
    ("woman_bald_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F9B2 };
    ("woman_white_haired_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F9B3 };
    ("woman_in_motorized_wheelchair_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F9BC };
    ("woman_in_manual_wheelchair_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_1F9BD };
    ("woman_health_worker_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2695_FE0F };
    ("woman_judge_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2696_FE0F };
    ("woman_pilot_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2708_FE0F };
    ("couple_with_heart_mw_tone3-1") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_wm_tone3-1") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_mw_tone3-2") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_wm_tone3-2") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_mw_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_wm_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_mw_tone3-4") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_wm_tone3-4") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_mw_tone3-5") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FF };
    ("couple_with_heart_wm_tone3-5") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F468_1F3FF };
    ("couple_with_heart_ww_tone3-1") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F469_1F3FB };
    ("couple_with_heart_ww_tone3-2") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F469_1F3FC };
    ("couple_with_heart_ww_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F469_1F3FD };
    ("couple_with_heart_ww_tone3-4") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F469_1F3FE };
    ("couple_with_heart_ww_tone3-5") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F469_1F3FF };
    ("kiss_mw_tone3-1") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_wm_tone3-1") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_mw_tone3-2") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_wm_tone3-2") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_mw_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_wm_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_mw_tone3-4") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_wm_tone3-4") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_mw_tone3-5") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("kiss_wm_tone3-5") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("kiss_ww_tone3-1") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FB };
    ("kiss_ww_tone3-2") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FC };
    ("kiss_ww_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FD };
    ("kiss_ww_tone3-4") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FE };
    ("kiss_ww_tone3-5") => { &twemoji_assets::png::codes::U_1F469_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FF };
    ("woman_tone3") => { &twemoji_assets::png::codes::U_1F469_1F3FD };
    ("woman_farmer_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F33E };
    ("woman_cook_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F373 };
    ("woman_feeding_baby_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F37C };
    ("woman_student_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F393 };
    ("woman_singer_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F3A4 };
    ("woman_artist_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F3A8 };
    ("woman_teacher_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F3EB };
    ("woman_factory_worker_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F3ED };
    ("woman_technologist_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F4BB };
    ("woman_office_worker_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F4BC };
    ("woman_mechanic_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F527 };
    ("woman_scientist_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F52C };
    ("woman_astronaut_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F680 };
    ("woman_firefighter_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F692 };
    ("couple_tone4-1") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F91D_200D_1F468_1F3FB };
    ("couple_tone4-2") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F91D_200D_1F468_1F3FC };
    ("couple_tone4-3") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F91D_200D_1F468_1F3FD };
    ("couple_tone4-5") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F91D_200D_1F468_1F3FF };
    ("two_women_holding_hands_tone4-1") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F91D_200D_1F469_1F3FB };
    ("two_women_holding_hands_tone4-2") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F91D_200D_1F469_1F3FC };
    ("two_women_holding_hands_tone4-3") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F91D_200D_1F469_1F3FD };
    ("two_women_holding_hands_tone4-5") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F91D_200D_1F469_1F3FF };
    ("woman_with_probing_cane_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F9AF };
    ("woman_with_white_cane_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F9AF };
    ("woman_red_haired_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F9B0 };
    ("woman_curly_haired_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F9B1 };
    ("woman_bald_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F9B2 };
    ("woman_white_haired_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F9B3 };
    ("woman_in_motorized_wheelchair_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F9BC };
    ("woman_in_manual_wheelchair_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_1F9BD };
    ("woman_health_worker_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2695_FE0F };
    ("woman_judge_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2696_FE0F };
    ("woman_pilot_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2708_FE0F };
    ("couple_with_heart_mw_tone4-1") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_wm_tone4-1") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_mw_tone4-2") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_wm_tone4-2") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_mw_tone4-3") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_wm_tone4-3") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_mw_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_wm_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_mw_tone4-5") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FF };
    ("couple_with_heart_wm_tone4-5") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F468_1F3FF };
    ("couple_with_heart_ww_tone4-1") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F469_1F3FB };
    ("couple_with_heart_ww_tone4-2") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F469_1F3FC };
    ("couple_with_heart_ww_tone4-3") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F469_1F3FD };
    ("couple_with_heart_ww_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F469_1F3FE };
    ("couple_with_heart_ww_tone4-5") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F469_1F3FF };
    ("kiss_mw_tone4-1") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_wm_tone4-1") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_mw_tone4-2") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_wm_tone4-2") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_mw_tone4-3") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_wm_tone4-3") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_mw_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_wm_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_mw_tone4-5") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("kiss_wm_tone4-5") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("kiss_ww_tone4-1") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FB };
    ("kiss_ww_tone4-2") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FC };
    ("kiss_ww_tone4-3") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FD };
    ("kiss_ww_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FE };
    ("kiss_ww_tone4-5") => { &twemoji_assets::png::codes::U_1F469_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FF };
    ("woman_tone4") => { &twemoji_assets::png::codes::U_1F469_1F3FE };
    ("woman_farmer_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F33E };
    ("woman_cook_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F373 };
    ("woman_feeding_baby_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F37C };
    ("woman_student_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F393 };
    ("woman_singer_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F3A4 };
    ("woman_artist_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F3A8 };
    ("woman_teacher_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F3EB };
    ("woman_factory_worker_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F3ED };
    ("woman_technologist_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F4BB };
    ("woman_office_worker_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F4BC };
    ("woman_mechanic_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F527 };
    ("woman_scientist_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F52C };
    ("woman_astronaut_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F680 };
    ("woman_firefighter_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F692 };
    ("couple_tone5-1") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F91D_200D_1F468_1F3FB };
    ("couple_tone5-2") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F91D_200D_1F468_1F3FC };
    ("couple_tone5-3") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F91D_200D_1F468_1F3FD };
    ("couple_tone5-4") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F91D_200D_1F468_1F3FE };
    ("two_women_holding_hands_tone5-1") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F91D_200D_1F469_1F3FB };
    ("two_women_holding_hands_tone5-2") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F91D_200D_1F469_1F3FC };
    ("two_women_holding_hands_tone5-3") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F91D_200D_1F469_1F3FD };
    ("two_women_holding_hands_tone5-4") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F91D_200D_1F469_1F3FE };
    ("woman_with_probing_cane_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F9AF };
    ("woman_with_white_cane_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F9AF };
    ("woman_red_haired_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F9B0 };
    ("woman_curly_haired_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F9B1 };
    ("woman_bald_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F9B2 };
    ("woman_white_haired_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F9B3 };
    ("woman_in_motorized_wheelchair_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F9BC };
    ("woman_in_manual_wheelchair_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_1F9BD };
    ("woman_health_worker_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2695_FE0F };
    ("woman_judge_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2696_FE0F };
    ("woman_pilot_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2708_FE0F };
    ("couple_with_heart_mw_tone5-1") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_wm_tone5-1") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FB };
    ("couple_with_heart_mw_tone5-2") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_wm_tone5-2") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FC };
    ("couple_with_heart_mw_tone5-3") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_wm_tone5-3") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FD };
    ("couple_with_heart_mw_tone5-4") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_wm_tone5-4") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FE };
    ("couple_with_heart_mw_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FF };
    ("couple_with_heart_wm_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F468_1F3FF };
    ("couple_with_heart_ww_tone5-1") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F469_1F3FB };
    ("couple_with_heart_ww_tone5-2") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F469_1F3FC };
    ("couple_with_heart_ww_tone5-3") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F469_1F3FD };
    ("couple_with_heart_ww_tone5-4") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F469_1F3FE };
    ("couple_with_heart_ww_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F469_1F3FF };
    ("kiss_mw_tone5-1") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_wm_tone5-1") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FB };
    ("kiss_mw_tone5-2") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_wm_tone5-2") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FC };
    ("kiss_mw_tone5-3") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_wm_tone5-3") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FD };
    ("kiss_mw_tone5-4") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_wm_tone5-4") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FE };
    ("kiss_mw_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("kiss_wm_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F468_1F3FF };
    ("kiss_ww_tone5-1") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FB };
    ("kiss_ww_tone5-2") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FC };
    ("kiss_ww_tone5-3") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FD };
    ("kiss_ww_tone5-4") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FE };
    ("kiss_ww_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F469_1F3FF };
    ("woman_tone5") => { &twemoji_assets::png::codes::U_1F469_1F3FF };
    ("woman_farmer") => { &twemoji_assets::png::codes::U_1F469_200D_1F33E };
    ("woman_cook") => { &twemoji_assets::png::codes::U_1F469_200D_1F373 };
    ("woman_feeding_baby") => { &twemoji_assets::png::codes::U_1F469_200D_1F37C };
    ("woman_student") => { &twemoji_assets::png::codes::U_1F469_200D_1F393 };
    ("woman_singer") => { &twemoji_assets::png::codes::U_1F469_200D_1F3A4 };
    ("woman_artist") => { &twemoji_assets::png::codes::U_1F469_200D_1F3A8 };
    ("woman_teacher") => { &twemoji_assets::png::codes::U_1F469_200D_1F3EB };
    ("woman_factory_worker") => { &twemoji_assets::png::codes::U_1F469_200D_1F3ED };
    ("family_wbb") => { &twemoji_assets::png::codes::U_1F469_200D_1F466_200D_1F466 };
    ("family_wb") => { &twemoji_assets::png::codes::U_1F469_200D_1F466 };
    ("family_wgb") => { &twemoji_assets::png::codes::U_1F469_200D_1F467_200D_1F466 };
    ("family_wgg") => { &twemoji_assets::png::codes::U_1F469_200D_1F467_200D_1F467 };
    ("family_wg") => { &twemoji_assets::png::codes::U_1F469_200D_1F467 };
    ("family_wwbb") => { &twemoji_assets::png::codes::U_1F469_200D_1F469_200D_1F466_200D_1F466 };
    ("family_wwb") => { &twemoji_assets::png::codes::U_1F469_200D_1F469_200D_1F466 };
    ("family_wwgb") => { &twemoji_assets::png::codes::U_1F469_200D_1F469_200D_1F467_200D_1F466 };
    ("family_wwgg") => { &twemoji_assets::png::codes::U_1F469_200D_1F469_200D_1F467_200D_1F467 };
    ("family_wwg") => { &twemoji_assets::png::codes::U_1F469_200D_1F469_200D_1F467 };
    ("woman_technologist") => { &twemoji_assets::png::codes::U_1F469_200D_1F4BB };
    ("woman_office_worker") => { &twemoji_assets::png::codes::U_1F469_200D_1F4BC };
    ("woman_mechanic") => { &twemoji_assets::png::codes::U_1F469_200D_1F527 };
    ("woman_scientist") => { &twemoji_assets::png::codes::U_1F469_200D_1F52C };
    ("woman_astronaut") => { &twemoji_assets::png::codes::U_1F469_200D_1F680 };
    ("woman_firefighter") => { &twemoji_assets::png::codes::U_1F469_200D_1F692 };
    ("woman_with_probing_cane") => { &twemoji_assets::png::codes::U_1F469_200D_1F9AF };
    ("woman_with_white_cane") => { &twemoji_assets::png::codes::U_1F469_200D_1F9AF };
    ("woman_red_haired") => { &twemoji_assets::png::codes::U_1F469_200D_1F9B0 };
    ("woman_curly_haired") => { &twemoji_assets::png::codes::U_1F469_200D_1F9B1 };
    ("woman_bald") => { &twemoji_assets::png::codes::U_1F469_200D_1F9B2 };
    ("woman_white_haired") => { &twemoji_assets::png::codes::U_1F469_200D_1F9B3 };
    ("woman_in_motorized_wheelchair") => { &twemoji_assets::png::codes::U_1F469_200D_1F9BC };
    ("woman_in_manual_wheelchair") => { &twemoji_assets::png::codes::U_1F469_200D_1F9BD };
    ("woman_health_worker") => { &twemoji_assets::png::codes::U_1F469_200D_2695_FE0F };
    ("woman_judge") => { &twemoji_assets::png::codes::U_1F469_200D_2696_FE0F };
    ("woman_pilot") => { &twemoji_assets::png::codes::U_1F469_200D_2708_FE0F };
    ("couple_with_heart_mw") => { &twemoji_assets::png::codes::U_1F469_200D_2764_FE0F_200D_1F468 };
    ("couple_with_heart_wm") => { &twemoji_assets::png::codes::U_1F469_200D_2764_FE0F_200D_1F468 };
    ("couple_with_heart_ww") => { &twemoji_assets::png::codes::U_1F469_200D_2764_FE0F_200D_1F469 };
    ("kiss_mw") => { &twemoji_assets::png::codes::U_1F469_200D_2764_FE0F_200D_1F48B_200D_1F468 };
    ("kiss_wm") => { &twemoji_assets::png::codes::U_1F469_200D_2764_FE0F_200D_1F48B_200D_1F468 };
    ("kiss_ww") => { &twemoji_assets::png::codes::U_1F469_200D_2764_FE0F_200D_1F48B_200D_1F469 };
    ("woman") => { &twemoji_assets::png::codes::U_1F469 };
    ("family") => { &twemoji_assets::png::codes::U_1F46A };
    ("couple_tone1") => { &twemoji_assets::png::codes::U_1F46B_1F3FB };
    ("couple_tone2") => { &twemoji_assets::png::codes::U_1F46B_1F3FC };
    ("couple_tone3") => { &twemoji_assets::png::codes::U_1F46B_1F3FD };
    ("couple_tone4") => { &twemoji_assets::png::codes::U_1F46B_1F3FE };
    ("couple_tone5") => { &twemoji_assets::png::codes::U_1F46B_1F3FF };
    ("couple") => { &twemoji_assets::png::codes::U_1F46B };
    ("two_men_holding_hands_tone1") => { &twemoji_assets::png::codes::U_1F46C_1F3FB };
    ("two_men_holding_hands_tone2") => { &twemoji_assets::png::codes::U_1F46C_1F3FC };
    ("two_men_holding_hands_tone3") => { &twemoji_assets::png::codes::U_1F46C_1F3FD };
    ("two_men_holding_hands_tone4") => { &twemoji_assets::png::codes::U_1F46C_1F3FE };
    ("two_men_holding_hands_tone5") => { &twemoji_assets::png::codes::U_1F46C_1F3FF };
    ("two_men_holding_hands") => { &twemoji_assets::png::codes::U_1F46C };
    ("two_women_holding_hands_tone1") => { &twemoji_assets::png::codes::U_1F46D_1F3FB };
    ("two_women_holding_hands_tone2") => { &twemoji_assets::png::codes::U_1F46D_1F3FC };
    ("two_women_holding_hands_tone3") => { &twemoji_assets::png::codes::U_1F46D_1F3FD };
    ("two_women_holding_hands_tone4") => { &twemoji_assets::png::codes::U_1F46D_1F3FE };
    ("two_women_holding_hands_tone5") => { &twemoji_assets::png::codes::U_1F46D_1F3FF };
    ("two_women_holding_hands") => { &twemoji_assets::png::codes::U_1F46D };
    ("woman_police_officer_tone1") => { &twemoji_assets::png::codes::U_1F46E_1F3FB_200D_2640_FE0F };
    ("man_police_officer_tone1") => { &twemoji_assets::png::codes::U_1F46E_1F3FB_200D_2642_FE0F };
    ("cop_tone1") => { &twemoji_assets::png::codes::U_1F46E_1F3FB };
    ("police_officer_tone1") => { &twemoji_assets::png::codes::U_1F46E_1F3FB };
    ("woman_police_officer_tone2") => { &twemoji_assets::png::codes::U_1F46E_1F3FC_200D_2640_FE0F };
    ("man_police_officer_tone2") => { &twemoji_assets::png::codes::U_1F46E_1F3FC_200D_2642_FE0F };
    ("cop_tone2") => { &twemoji_assets::png::codes::U_1F46E_1F3FC };
    ("police_officer_tone2") => { &twemoji_assets::png::codes::U_1F46E_1F3FC };
    ("woman_police_officer_tone3") => { &twemoji_assets::png::codes::U_1F46E_1F3FD_200D_2640_FE0F };
    ("man_police_officer_tone3") => { &twemoji_assets::png::codes::U_1F46E_1F3FD_200D_2642_FE0F };
    ("cop_tone3") => { &twemoji_assets::png::codes::U_1F46E_1F3FD };
    ("police_officer_tone3") => { &twemoji_assets::png::codes::U_1F46E_1F3FD };
    ("woman_police_officer_tone4") => { &twemoji_assets::png::codes::U_1F46E_1F3FE_200D_2640_FE0F };
    ("man_police_officer_tone4") => { &twemoji_assets::png::codes::U_1F46E_1F3FE_200D_2642_FE0F };
    ("cop_tone4") => { &twemoji_assets::png::codes::U_1F46E_1F3FE };
    ("police_officer_tone4") => { &twemoji_assets::png::codes::U_1F46E_1F3FE };
    ("woman_police_officer_tone5") => { &twemoji_assets::png::codes::U_1F46E_1F3FF_200D_2640_FE0F };
    ("man_police_officer_tone5") => { &twemoji_assets::png::codes::U_1F46E_1F3FF_200D_2642_FE0F };
    ("cop_tone5") => { &twemoji_assets::png::codes::U_1F46E_1F3FF };
    ("police_officer_tone5") => { &twemoji_assets::png::codes::U_1F46E_1F3FF };
    ("woman_police_officer") => { &twemoji_assets::png::codes::U_1F46E_200D_2640_FE0F };
    ("man_police_officer") => { &twemoji_assets::png::codes::U_1F46E_200D_2642_FE0F };
    ("cop") => { &twemoji_assets::png::codes::U_1F46E };
    ("police_officer") => { &twemoji_assets::png::codes::U_1F46E };
    ("women_with_bunny_ears_partying") => { &twemoji_assets::png::codes::U_1F46F_200D_2640_FE0F };
    ("men_with_bunny_ears_partying") => { &twemoji_assets::png::codes::U_1F46F_200D_2642_FE0F };
    ("dancers") => { &twemoji_assets::png::codes::U_1F46F };
    ("people_with_bunny_ears_partying") => { &twemoji_assets::png::codes::U_1F46F };
    ("woman_with_veil_tone1") => { &twemoji_assets::png::codes::U_1F470_1F3FB_200D_2640_FE0F };
    ("man_with_veil_tone1") => { &twemoji_assets::png::codes::U_1F470_1F3FB_200D_2642_FE0F };
    ("person_with_veil_tone1") => { &twemoji_assets::png::codes::U_1F470_1F3FB };
    ("woman_with_veil_tone2") => { &twemoji_assets::png::codes::U_1F470_1F3FC_200D_2640_FE0F };
    ("man_with_veil_tone2") => { &twemoji_assets::png::codes::U_1F470_1F3FC_200D_2642_FE0F };
    ("person_with_veil_tone2") => { &twemoji_assets::png::codes::U_1F470_1F3FC };
    ("woman_with_veil_tone3") => { &twemoji_assets::png::codes::U_1F470_1F3FD_200D_2640_FE0F };
    ("man_with_veil_tone3") => { &twemoji_assets::png::codes::U_1F470_1F3FD_200D_2642_FE0F };
    ("person_with_veil_tone3") => { &twemoji_assets::png::codes::U_1F470_1F3FD };
    ("woman_with_veil_tone4") => { &twemoji_assets::png::codes::U_1F470_1F3FE_200D_2640_FE0F };
    ("man_with_veil_tone4") => { &twemoji_assets::png::codes::U_1F470_1F3FE_200D_2642_FE0F };
    ("person_with_veil_tone4") => { &twemoji_assets::png::codes::U_1F470_1F3FE };
    ("woman_with_veil_tone5") => { &twemoji_assets::png::codes::U_1F470_1F3FF_200D_2640_FE0F };
    ("man_with_veil_tone5") => { &twemoji_assets::png::codes::U_1F470_1F3FF_200D_2642_FE0F };
    ("person_with_veil_tone5") => { &twemoji_assets::png::codes::U_1F470_1F3FF };
    ("woman_with_veil") => { &twemoji_assets::png::codes::U_1F470_200D_2640_FE0F };
    ("man_with_veil") => { &twemoji_assets::png::codes::U_1F470_200D_2642_FE0F };
    ("person_with_veil") => { &twemoji_assets::png::codes::U_1F470 };
    ("woman_blond_haired_tone1") => { &twemoji_assets::png::codes::U_1F471_1F3FB_200D_2640_FE0F };
    ("man_blond_haired_tone1") => { &twemoji_assets::png::codes::U_1F471_1F3FB_200D_2642_FE0F };
    ("blond_haired_tone1") => { &twemoji_assets::png::codes::U_1F471_1F3FB };
    ("woman_blond_haired_tone2") => { &twemoji_assets::png::codes::U_1F471_1F3FC_200D_2640_FE0F };
    ("man_blond_haired_tone2") => { &twemoji_assets::png::codes::U_1F471_1F3FC_200D_2642_FE0F };
    ("blond_haired_tone2") => { &twemoji_assets::png::codes::U_1F471_1F3FC };
    ("woman_blond_haired_tone3") => { &twemoji_assets::png::codes::U_1F471_1F3FD_200D_2640_FE0F };
    ("man_blond_haired_tone3") => { &twemoji_assets::png::codes::U_1F471_1F3FD_200D_2642_FE0F };
    ("blond_haired_tone3") => { &twemoji_assets::png::codes::U_1F471_1F3FD };
    ("woman_blond_haired_tone4") => { &twemoji_assets::png::codes::U_1F471_1F3FE_200D_2640_FE0F };
    ("man_blond_haired_tone4") => { &twemoji_assets::png::codes::U_1F471_1F3FE_200D_2642_FE0F };
    ("blond_haired_tone4") => { &twemoji_assets::png::codes::U_1F471_1F3FE };
    ("woman_blond_haired_tone5") => { &twemoji_assets::png::codes::U_1F471_1F3FF_200D_2640_FE0F };
    ("man_blond_haired_tone5") => { &twemoji_assets::png::codes::U_1F471_1F3FF_200D_2642_FE0F };
    ("blond_haired_tone5") => { &twemoji_assets::png::codes::U_1F471_1F3FF };
    ("woman_blond_haired") => { &twemoji_assets::png::codes::U_1F471_200D_2640_FE0F };
    ("man_blond_haired") => { &twemoji_assets::png::codes::U_1F471_200D_2642_FE0F };
    ("blond_haired") => { &twemoji_assets::png::codes::U_1F471 };
    ("person_with_skullcap_tone1") => { &twemoji_assets::png::codes::U_1F472_1F3FB };
    ("person_with_skullcap_tone2") => { &twemoji_assets::png::codes::U_1F472_1F3FC };
    ("person_with_skullcap_tone3") => { &twemoji_assets::png::codes::U_1F472_1F3FD };
    ("person_with_skullcap_tone4") => { &twemoji_assets::png::codes::U_1F472_1F3FE };
    ("person_with_skullcap_tone5") => { &twemoji_assets::png::codes::U_1F472_1F3FF };
    ("person_with_skullcap") => { &twemoji_assets::png::codes::U_1F472 };
    ("woman_wearing_turban_tone1") => { &twemoji_assets::png::codes::U_1F473_1F3FB_200D_2640_FE0F };
    ("man_wearing_turban_tone1") => { &twemoji_assets::png::codes::U_1F473_1F3FB_200D_2642_FE0F };
    ("person_wearing_turban_tone1") => { &twemoji_assets::png::codes::U_1F473_1F3FB };
    ("woman_wearing_turban_tone2") => { &twemoji_assets::png::codes::U_1F473_1F3FC_200D_2640_FE0F };
    ("man_wearing_turban_tone2") => { &twemoji_assets::png::codes::U_1F473_1F3FC_200D_2642_FE0F };
    ("person_wearing_turban_tone2") => { &twemoji_assets::png::codes::U_1F473_1F3FC };
    ("woman_wearing_turban_tone3") => { &twemoji_assets::png::codes::U_1F473_1F3FD_200D_2640_FE0F };
    ("man_wearing_turban_tone3") => { &twemoji_assets::png::codes::U_1F473_1F3FD_200D_2642_FE0F };
    ("person_wearing_turban_tone3") => { &twemoji_assets::png::codes::U_1F473_1F3FD };
    ("woman_wearing_turban_tone4") => { &twemoji_assets::png::codes::U_1F473_1F3FE_200D_2640_FE0F };
    ("man_wearing_turban_tone4") => { &twemoji_assets::png::codes::U_1F473_1F3FE_200D_2642_FE0F };
    ("person_wearing_turban_tone4") => { &twemoji_assets::png::codes::U_1F473_1F3FE };
    ("woman_wearing_turban_tone5") => { &twemoji_assets::png::codes::U_1F473_1F3FF_200D_2640_FE0F };
    ("man_wearing_turban_tone5") => { &twemoji_assets::png::codes::U_1F473_1F3FF_200D_2642_FE0F };
    ("person_wearing_turban_tone5") => { &twemoji_assets::png::codes::U_1F473_1F3FF };
    ("woman_wearing_turban") => { &twemoji_assets::png::codes::U_1F473_200D_2640_FE0F };
    ("man_wearing_turban") => { &twemoji_assets::png::codes::U_1F473_200D_2642_FE0F };
    ("person_wearing_turban") => { &twemoji_assets::png::codes::U_1F473 };
    ("older_man_tone1") => { &twemoji_assets::png::codes::U_1F474_1F3FB };
    ("older_man_tone2") => { &twemoji_assets::png::codes::U_1F474_1F3FC };
    ("older_man_tone3") => { &twemoji_assets::png::codes::U_1F474_1F3FD };
    ("older_man_tone4") => { &twemoji_assets::png::codes::U_1F474_1F3FE };
    ("older_man_tone5") => { &twemoji_assets::png::codes::U_1F474_1F3FF };
    ("older_man") => { &twemoji_assets::png::codes::U_1F474 };
    ("older_woman_tone1") => { &twemoji_assets::png::codes::U_1F475_1F3FB };
    ("older_woman_tone2") => { &twemoji_assets::png::codes::U_1F475_1F3FC };
    ("older_woman_tone3") => { &twemoji_assets::png::codes::U_1F475_1F3FD };
    ("older_woman_tone4") => { &twemoji_assets::png::codes::U_1F475_1F3FE };
    ("older_woman_tone5") => { &twemoji_assets::png::codes::U_1F475_1F3FF };
    ("older_woman") => { &twemoji_assets::png::codes::U_1F475 };
    ("baby_tone1") => { &twemoji_assets::png::codes::U_1F476_1F3FB };
    ("baby_tone2") => { &twemoji_assets::png::codes::U_1F476_1F3FC };
    ("baby_tone3") => { &twemoji_assets::png::codes::U_1F476_1F3FD };
    ("baby_tone4") => { &twemoji_assets::png::codes::U_1F476_1F3FE };
    ("baby_tone5") => { &twemoji_assets::png::codes::U_1F476_1F3FF };
    ("baby") => { &twemoji_assets::png::codes::U_1F476 };
    ("woman_construction_worker_tone1") => { &twemoji_assets::png::codes::U_1F477_1F3FB_200D_2640_FE0F };
    ("man_construction_worker_tone1") => { &twemoji_assets::png::codes::U_1F477_1F3FB_200D_2642_FE0F };
    ("construction_worker_tone1") => { &twemoji_assets::png::codes::U_1F477_1F3FB };
    ("woman_construction_worker_tone2") => { &twemoji_assets::png::codes::U_1F477_1F3FC_200D_2640_FE0F };
    ("man_construction_worker_tone2") => { &twemoji_assets::png::codes::U_1F477_1F3FC_200D_2642_FE0F };
    ("construction_worker_tone2") => { &twemoji_assets::png::codes::U_1F477_1F3FC };
    ("woman_construction_worker_tone3") => { &twemoji_assets::png::codes::U_1F477_1F3FD_200D_2640_FE0F };
    ("man_construction_worker_tone3") => { &twemoji_assets::png::codes::U_1F477_1F3FD_200D_2642_FE0F };
    ("construction_worker_tone3") => { &twemoji_assets::png::codes::U_1F477_1F3FD };
    ("woman_construction_worker_tone4") => { &twemoji_assets::png::codes::U_1F477_1F3FE_200D_2640_FE0F };
    ("man_construction_worker_tone4") => { &twemoji_assets::png::codes::U_1F477_1F3FE_200D_2642_FE0F };
    ("construction_worker_tone4") => { &twemoji_assets::png::codes::U_1F477_1F3FE };
    ("woman_construction_worker_tone5") => { &twemoji_assets::png::codes::U_1F477_1F3FF_200D_2640_FE0F };
    ("man_construction_worker_tone5") => { &twemoji_assets::png::codes::U_1F477_1F3FF_200D_2642_FE0F };
    ("construction_worker_tone5") => { &twemoji_assets::png::codes::U_1F477_1F3FF };
    ("woman_construction_worker") => { &twemoji_assets::png::codes::U_1F477_200D_2640_FE0F };
    ("man_construction_worker") => { &twemoji_assets::png::codes::U_1F477_200D_2642_FE0F };
    ("construction_worker") => { &twemoji_assets::png::codes::U_1F477 };
    ("princess_tone1") => { &twemoji_assets::png::codes::U_1F478_1F3FB };
    ("princess_tone2") => { &twemoji_assets::png::codes::U_1F478_1F3FC };
    ("princess_tone3") => { &twemoji_assets::png::codes::U_1F478_1F3FD };
    ("princess_tone4") => { &twemoji_assets::png::codes::U_1F478_1F3FE };
    ("princess_tone5") => { &twemoji_assets::png::codes::U_1F478_1F3FF };
    ("princess") => { &twemoji_assets::png::codes::U_1F478 };
    ("japanese_ogre") => { &twemoji_assets::png::codes::U_1F479 };
    ("ogre") => { &twemoji_assets::png::codes::U_1F479 };
    ("goblin") => { &twemoji_assets::png::codes::U_1F47A };
    ("japanese_goblin") => { &twemoji_assets::png::codes::U_1F47A };
    ("ghost") => { &twemoji_assets::png::codes::U_1F47B };
    ("angel_tone1") => { &twemoji_assets::png::codes::U_1F47C_1F3FB };
    ("angel_tone2") => { &twemoji_assets::png::codes::U_1F47C_1F3FC };
    ("angel_tone3") => { &twemoji_assets::png::codes::U_1F47C_1F3FD };
    ("angel_tone4") => { &twemoji_assets::png::codes::U_1F47C_1F3FE };
    ("angel_tone5") => { &twemoji_assets::png::codes::U_1F47C_1F3FF };
    ("angel") => { &twemoji_assets::png::codes::U_1F47C };
    ("alien") => { &twemoji_assets::png::codes::U_1F47D };
    ("alien_monster") => { &twemoji_assets::png::codes::U_1F47E };
    ("space_invader") => { &twemoji_assets::png::codes::U_1F47E };
    ("angry_imp") => { &twemoji_assets::png::codes::U_1F47F };
    ("imp") => { &twemoji_assets::png::codes::U_1F47F };
    ("skull") => { &twemoji_assets::png::codes::U_1F480 };
    ("woman_tipping_hand_tone1") => { &twemoji_assets::png::codes::U_1F481_1F3FB_200D_2640_FE0F };
    ("man_tipping_hand_tone1") => { &twemoji_assets::png::codes::U_1F481_1F3FB_200D_2642_FE0F };
    ("person_tipping_hand_tone1") => { &twemoji_assets::png::codes::U_1F481_1F3FB };
    ("woman_tipping_hand_tone2") => { &twemoji_assets::png::codes::U_1F481_1F3FC_200D_2640_FE0F };
    ("man_tipping_hand_tone2") => { &twemoji_assets::png::codes::U_1F481_1F3FC_200D_2642_FE0F };
    ("person_tipping_hand_tone2") => { &twemoji_assets::png::codes::U_1F481_1F3FC };
    ("woman_tipping_hand_tone3") => { &twemoji_assets::png::codes::U_1F481_1F3FD_200D_2640_FE0F };
    ("man_tipping_hand_tone3") => { &twemoji_assets::png::codes::U_1F481_1F3FD_200D_2642_FE0F };
    ("person_tipping_hand_tone3") => { &twemoji_assets::png::codes::U_1F481_1F3FD };
    ("woman_tipping_hand_tone4") => { &twemoji_assets::png::codes::U_1F481_1F3FE_200D_2640_FE0F };
    ("man_tipping_hand_tone4") => { &twemoji_assets::png::codes::U_1F481_1F3FE_200D_2642_FE0F };
    ("person_tipping_hand_tone4") => { &twemoji_assets::png::codes::U_1F481_1F3FE };
    ("woman_tipping_hand_tone5") => { &twemoji_assets::png::codes::U_1F481_1F3FF_200D_2640_FE0F };
    ("man_tipping_hand_tone5") => { &twemoji_assets::png::codes::U_1F481_1F3FF_200D_2642_FE0F };
    ("person_tipping_hand_tone5") => { &twemoji_assets::png::codes::U_1F481_1F3FF };
    ("woman_tipping_hand") => { &twemoji_assets::png::codes::U_1F481_200D_2640_FE0F };
    ("man_tipping_hand") => { &twemoji_assets::png::codes::U_1F481_200D_2642_FE0F };
    ("person_tipping_hand") => { &twemoji_assets::png::codes::U_1F481 };
    ("woman_guard_tone1") => { &twemoji_assets::png::codes::U_1F482_1F3FB_200D_2640_FE0F };
    ("man_guard_tone1") => { &twemoji_assets::png::codes::U_1F482_1F3FB_200D_2642_FE0F };
    ("guard_tone1") => { &twemoji_assets::png::codes::U_1F482_1F3FB };
    ("woman_guard_tone2") => { &twemoji_assets::png::codes::U_1F482_1F3FC_200D_2640_FE0F };
    ("man_guard_tone2") => { &twemoji_assets::png::codes::U_1F482_1F3FC_200D_2642_FE0F };
    ("guard_tone2") => { &twemoji_assets::png::codes::U_1F482_1F3FC };
    ("woman_guard_tone3") => { &twemoji_assets::png::codes::U_1F482_1F3FD_200D_2640_FE0F };
    ("man_guard_tone3") => { &twemoji_assets::png::codes::U_1F482_1F3FD_200D_2642_FE0F };
    ("guard_tone3") => { &twemoji_assets::png::codes::U_1F482_1F3FD };
    ("woman_guard_tone4") => { &twemoji_assets::png::codes::U_1F482_1F3FE_200D_2640_FE0F };
    ("man_guard_tone4") => { &twemoji_assets::png::codes::U_1F482_1F3FE_200D_2642_FE0F };
    ("guard_tone4") => { &twemoji_assets::png::codes::U_1F482_1F3FE };
    ("woman_guard_tone5") => { &twemoji_assets::png::codes::U_1F482_1F3FF_200D_2640_FE0F };
    ("man_guard_tone5") => { &twemoji_assets::png::codes::U_1F482_1F3FF_200D_2642_FE0F };
    ("guard_tone5") => { &twemoji_assets::png::codes::U_1F482_1F3FF };
    ("woman_guard") => { &twemoji_assets::png::codes::U_1F482_200D_2640_FE0F };
    ("man_guard") => { &twemoji_assets::png::codes::U_1F482_200D_2642_FE0F };
    ("guard") => { &twemoji_assets::png::codes::U_1F482 };
    ("dancer_tone1") => { &twemoji_assets::png::codes::U_1F483_1F3FB };
    ("woman_dancing_tone1") => { &twemoji_assets::png::codes::U_1F483_1F3FB };
    ("dancer_tone2") => { &twemoji_assets::png::codes::U_1F483_1F3FC };
    ("woman_dancing_tone2") => { &twemoji_assets::png::codes::U_1F483_1F3FC };
    ("dancer_tone3") => { &twemoji_assets::png::codes::U_1F483_1F3FD };
    ("woman_dancing_tone3") => { &twemoji_assets::png::codes::U_1F483_1F3FD };
    ("dancer_tone4") => { &twemoji_assets::png::codes::U_1F483_1F3FE };
    ("woman_dancing_tone4") => { &twemoji_assets::png::codes::U_1F483_1F3FE };
    ("dancer_tone5") => { &twemoji_assets::png::codes::U_1F483_1F3FF };
    ("woman_dancing_tone5") => { &twemoji_assets::png::codes::U_1F483_1F3FF };
    ("dancer") => { &twemoji_assets::png::codes::U_1F483 };
    ("woman_dancing") => { &twemoji_assets::png::codes::U_1F483 };
    ("lipstick") => { &twemoji_assets::png::codes::U_1F484 };
    ("nail_care_tone1") => { &twemoji_assets::png::codes::U_1F485_1F3FB };
    ("nail_polish_tone1") => { &twemoji_assets::png::codes::U_1F485_1F3FB };
    ("nail_care_tone2") => { &twemoji_assets::png::codes::U_1F485_1F3FC };
    ("nail_polish_tone2") => { &twemoji_assets::png::codes::U_1F485_1F3FC };
    ("nail_care_tone3") => { &twemoji_assets::png::codes::U_1F485_1F3FD };
    ("nail_polish_tone3") => { &twemoji_assets::png::codes::U_1F485_1F3FD };
    ("nail_care_tone4") => { &twemoji_assets::png::codes::U_1F485_1F3FE };
    ("nail_polish_tone4") => { &twemoji_assets::png::codes::U_1F485_1F3FE };
    ("nail_care_tone5") => { &twemoji_assets::png::codes::U_1F485_1F3FF };
    ("nail_polish_tone5") => { &twemoji_assets::png::codes::U_1F485_1F3FF };
    ("nail_care") => { &twemoji_assets::png::codes::U_1F485 };
    ("nail_polish") => { &twemoji_assets::png::codes::U_1F485 };
    ("woman_getting_massage_tone1") => { &twemoji_assets::png::codes::U_1F486_1F3FB_200D_2640_FE0F };
    ("man_getting_massage_tone1") => { &twemoji_assets::png::codes::U_1F486_1F3FB_200D_2642_FE0F };
    ("massage_tone1") => { &twemoji_assets::png::codes::U_1F486_1F3FB };
    ("person_getting_massage_tone1") => { &twemoji_assets::png::codes::U_1F486_1F3FB };
    ("woman_getting_massage_tone2") => { &twemoji_assets::png::codes::U_1F486_1F3FC_200D_2640_FE0F };
    ("man_getting_massage_tone2") => { &twemoji_assets::png::codes::U_1F486_1F3FC_200D_2642_FE0F };
    ("massage_tone2") => { &twemoji_assets::png::codes::U_1F486_1F3FC };
    ("person_getting_massage_tone2") => { &twemoji_assets::png::codes::U_1F486_1F3FC };
    ("woman_getting_massage_tone3") => { &twemoji_assets::png::codes::U_1F486_1F3FD_200D_2640_FE0F };
    ("man_getting_massage_tone3") => { &twemoji_assets::png::codes::U_1F486_1F3FD_200D_2642_FE0F };
    ("massage_tone3") => { &twemoji_assets::png::codes::U_1F486_1F3FD };
    ("person_getting_massage_tone3") => { &twemoji_assets::png::codes::U_1F486_1F3FD };
    ("woman_getting_massage_tone4") => { &twemoji_assets::png::codes::U_1F486_1F3FE_200D_2640_FE0F };
    ("man_getting_massage_tone4") => { &twemoji_assets::png::codes::U_1F486_1F3FE_200D_2642_FE0F };
    ("massage_tone4") => { &twemoji_assets::png::codes::U_1F486_1F3FE };
    ("person_getting_massage_tone4") => { &twemoji_assets::png::codes::U_1F486_1F3FE };
    ("woman_getting_massage_tone5") => { &twemoji_assets::png::codes::U_1F486_1F3FF_200D_2640_FE0F };
    ("man_getting_massage_tone5") => { &twemoji_assets::png::codes::U_1F486_1F3FF_200D_2642_FE0F };
    ("massage_tone5") => { &twemoji_assets::png::codes::U_1F486_1F3FF };
    ("person_getting_massage_tone5") => { &twemoji_assets::png::codes::U_1F486_1F3FF };
    ("woman_getting_massage") => { &twemoji_assets::png::codes::U_1F486_200D_2640_FE0F };
    ("man_getting_massage") => { &twemoji_assets::png::codes::U_1F486_200D_2642_FE0F };
    ("massage") => { &twemoji_assets::png::codes::U_1F486 };
    ("person_getting_massage") => { &twemoji_assets::png::codes::U_1F486 };
    ("woman_getting_haircut_tone1") => { &twemoji_assets::png::codes::U_1F487_1F3FB_200D_2640_FE0F };
    ("man_getting_haircut_tone1") => { &twemoji_assets::png::codes::U_1F487_1F3FB_200D_2642_FE0F };
    ("haircut_tone1") => { &twemoji_assets::png::codes::U_1F487_1F3FB };
    ("person_getting_haircut_tone1") => { &twemoji_assets::png::codes::U_1F487_1F3FB };
    ("woman_getting_haircut_tone2") => { &twemoji_assets::png::codes::U_1F487_1F3FC_200D_2640_FE0F };
    ("man_getting_haircut_tone2") => { &twemoji_assets::png::codes::U_1F487_1F3FC_200D_2642_FE0F };
    ("haircut_tone2") => { &twemoji_assets::png::codes::U_1F487_1F3FC };
    ("person_getting_haircut_tone2") => { &twemoji_assets::png::codes::U_1F487_1F3FC };
    ("woman_getting_haircut_tone3") => { &twemoji_assets::png::codes::U_1F487_1F3FD_200D_2640_FE0F };
    ("man_getting_haircut_tone3") => { &twemoji_assets::png::codes::U_1F487_1F3FD_200D_2642_FE0F };
    ("haircut_tone3") => { &twemoji_assets::png::codes::U_1F487_1F3FD };
    ("person_getting_haircut_tone3") => { &twemoji_assets::png::codes::U_1F487_1F3FD };
    ("woman_getting_haircut_tone4") => { &twemoji_assets::png::codes::U_1F487_1F3FE_200D_2640_FE0F };
    ("man_getting_haircut_tone4") => { &twemoji_assets::png::codes::U_1F487_1F3FE_200D_2642_FE0F };
    ("haircut_tone4") => { &twemoji_assets::png::codes::U_1F487_1F3FE };
    ("person_getting_haircut_tone4") => { &twemoji_assets::png::codes::U_1F487_1F3FE };
    ("woman_getting_haircut_tone5") => { &twemoji_assets::png::codes::U_1F487_1F3FF_200D_2640_FE0F };
    ("man_getting_haircut_tone5") => { &twemoji_assets::png::codes::U_1F487_1F3FF_200D_2642_FE0F };
    ("haircut_tone5") => { &twemoji_assets::png::codes::U_1F487_1F3FF };
    ("person_getting_haircut_tone5") => { &twemoji_assets::png::codes::U_1F487_1F3FF };
    ("woman_getting_haircut") => { &twemoji_assets::png::codes::U_1F487_200D_2640_FE0F };
    ("man_getting_haircut") => { &twemoji_assets::png::codes::U_1F487_200D_2642_FE0F };
    ("haircut") => { &twemoji_assets::png::codes::U_1F487 };
    ("person_getting_haircut") => { &twemoji_assets::png::codes::U_1F487 };
    ("barber") => { &twemoji_assets::png::codes::U_1F488 };
    ("barber_pole") => { &twemoji_assets::png::codes::U_1F488 };
    ("syringe") => { &twemoji_assets::png::codes::U_1F489 };
    ("pill") => { &twemoji_assets::png::codes::U_1F48A };
    ("kiss") => { &twemoji_assets::png::codes::U_1F48B };
    ("love_letter") => { &twemoji_assets::png::codes::U_1F48C };
    ("ring") => { &twemoji_assets::png::codes::U_1F48D };
    ("gem") => { &twemoji_assets::png::codes::U_1F48E };
    ("couple_kiss_tone1") => { &twemoji_assets::png::codes::U_1F48F_1F3FB };
    ("couplekiss_tone1") => { &twemoji_assets::png::codes::U_1F48F_1F3FB };
    ("couple_kiss_tone2") => { &twemoji_assets::png::codes::U_1F48F_1F3FC };
    ("couplekiss_tone2") => { &twemoji_assets::png::codes::U_1F48F_1F3FC };
    ("couple_kiss_tone3") => { &twemoji_assets::png::codes::U_1F48F_1F3FD };
    ("couplekiss_tone3") => { &twemoji_assets::png::codes::U_1F48F_1F3FD };
    ("couple_kiss_tone4") => { &twemoji_assets::png::codes::U_1F48F_1F3FE };
    ("couplekiss_tone4") => { &twemoji_assets::png::codes::U_1F48F_1F3FE };
    ("couple_kiss_tone5") => { &twemoji_assets::png::codes::U_1F48F_1F3FF };
    ("couplekiss_tone5") => { &twemoji_assets::png::codes::U_1F48F_1F3FF };
    ("couple_kiss") => { &twemoji_assets::png::codes::U_1F48F };
    ("couplekiss") => { &twemoji_assets::png::codes::U_1F48F };
    ("bouquet") => { &twemoji_assets::png::codes::U_1F490 };
    ("couple_with_heart_tone1") => { &twemoji_assets::png::codes::U_1F491_1F3FB };
    ("couple_with_heart_tone2") => { &twemoji_assets::png::codes::U_1F491_1F3FC };
    ("couple_with_heart_tone3") => { &twemoji_assets::png::codes::U_1F491_1F3FD };
    ("couple_with_heart_tone4") => { &twemoji_assets::png::codes::U_1F491_1F3FE };
    ("couple_with_heart_tone5") => { &twemoji_assets::png::codes::U_1F491_1F3FF };
    ("couple_with_heart") => { &twemoji_assets::png::codes::U_1F491 };
    ("wedding") => { &twemoji_assets::png::codes::U_1F492 };
    ("beating_heart") => { &twemoji_assets::png::codes::U_1F493 };
    ("heartbeat") => { &twemoji_assets::png::codes::U_1F493 };
    ("broken_heart") => { &twemoji_assets::png::codes::U_1F494 };
    ("two_hearts") => { &twemoji_assets::png::codes::U_1F495 };
    ("sparkling_heart") => { &twemoji_assets::png::codes::U_1F496 };
    ("growing_heart") => { &twemoji_assets::png::codes::U_1F497 };
    ("heartpulse") => { &twemoji_assets::png::codes::U_1F497 };
    ("cupid") => { &twemoji_assets::png::codes::U_1F498 };
    ("heart_with_arrow") => { &twemoji_assets::png::codes::U_1F498 };
    ("blue_heart") => { &twemoji_assets::png::codes::U_1F499 };
    ("green_heart") => { &twemoji_assets::png::codes::U_1F49A };
    ("yellow_heart") => { &twemoji_assets::png::codes::U_1F49B };
    ("purple_heart") => { &twemoji_assets::png::codes::U_1F49C };
    ("gift_heart") => { &twemoji_assets::png::codes::U_1F49D };
    ("heart_with_ribbon") => { &twemoji_assets::png::codes::U_1F49D };
    ("revolving_hearts") => { &twemoji_assets::png::codes::U_1F49E };
    ("heart_decoration") => { &twemoji_assets::png::codes::U_1F49F };
    ("diamond_shape_with_a_dot_inside") => { &twemoji_assets::png::codes::U_1F4A0 };
    ("diamond_with_a_dot") => { &twemoji_assets::png::codes::U_1F4A0 };
    ("bulb") => { &twemoji_assets::png::codes::U_1F4A1 };
    ("light_bulb") => { &twemoji_assets::png::codes::U_1F4A1 };
    ("anger") => { &twemoji_assets::png::codes::U_1F4A2 };
    ("bomb") => { &twemoji_assets::png::codes::U_1F4A3 };
    ("zzz") => { &twemoji_assets::png::codes::U_1F4A4 };
    ("boom") => { &twemoji_assets::png::codes::U_1F4A5 };
    ("collision") => { &twemoji_assets::png::codes::U_1F4A5 };
    ("sweat_drops") => { &twemoji_assets::png::codes::U_1F4A6 };
    ("droplet") => { &twemoji_assets::png::codes::U_1F4A7 };
    ("dash") => { &twemoji_assets::png::codes::U_1F4A8 };
    ("dashing_away") => { &twemoji_assets::png::codes::U_1F4A8 };
    ("poop") => { &twemoji_assets::png::codes::U_1F4A9 };
    ("shit") => { &twemoji_assets::png::codes::U_1F4A9 };
    ("muscle_tone1") => { &twemoji_assets::png::codes::U_1F4AA_1F3FB };
    ("right_bicep_tone1") => { &twemoji_assets::png::codes::U_1F4AA_1F3FB };
    ("muscle_tone2") => { &twemoji_assets::png::codes::U_1F4AA_1F3FC };
    ("right_bicep_tone2") => { &twemoji_assets::png::codes::U_1F4AA_1F3FC };
    ("muscle_tone3") => { &twemoji_assets::png::codes::U_1F4AA_1F3FD };
    ("right_bicep_tone3") => { &twemoji_assets::png::codes::U_1F4AA_1F3FD };
    ("muscle_tone4") => { &twemoji_assets::png::codes::U_1F4AA_1F3FE };
    ("right_bicep_tone4") => { &twemoji_assets::png::codes::U_1F4AA_1F3FE };
    ("muscle_tone5") => { &twemoji_assets::png::codes::U_1F4AA_1F3FF };
    ("right_bicep_tone5") => { &twemoji_assets::png::codes::U_1F4AA_1F3FF };
    ("muscle") => { &twemoji_assets::png::codes::U_1F4AA };
    ("right_bicep") => { &twemoji_assets::png::codes::U_1F4AA };
    ("dizzy") => { &twemoji_assets::png::codes::U_1F4AB };
    ("speech_balloon") => { &twemoji_assets::png::codes::U_1F4AC };
    ("thought_balloon") => { &twemoji_assets::png::codes::U_1F4AD };
    ("white_flower") => { &twemoji_assets::png::codes::U_1F4AE };
    ("100") => { &twemoji_assets::png::codes::U_1F4AF };
    ("moneybag") => { &twemoji_assets::png::codes::U_1F4B0 };
    ("currency_exchange") => { &twemoji_assets::png::codes::U_1F4B1 };
    ("heavy_dollar_sign") => { &twemoji_assets::png::codes::U_1F4B2 };
    ("credit_card") => { &twemoji_assets::png::codes::U_1F4B3 };
    ("yen") => { &twemoji_assets::png::codes::U_1F4B4 };
    ("dollar") => { &twemoji_assets::png::codes::U_1F4B5 };
    ("euro") => { &twemoji_assets::png::codes::U_1F4B6 };
    ("pound") => { &twemoji_assets::png::codes::U_1F4B7 };
    ("money_with_wings") => { &twemoji_assets::png::codes::U_1F4B8 };
    ("chart") => { &twemoji_assets::png::codes::U_1F4B9 };
    ("seat") => { &twemoji_assets::png::codes::U_1F4BA };
    ("laptop") => { &twemoji_assets::png::codes::U_1F4BB };
    ("briefcase") => { &twemoji_assets::png::codes::U_1F4BC };
    ("computer_disk") => { &twemoji_assets::png::codes::U_1F4BD };
    ("minidisc") => { &twemoji_assets::png::codes::U_1F4BD };
    ("floppy_disk") => { &twemoji_assets::png::codes::U_1F4BE };
    ("cd") => { &twemoji_assets::png::codes::U_1F4BF };
    ("optical_disk") => { &twemoji_assets::png::codes::U_1F4BF };
    ("dvd") => { &twemoji_assets::png::codes::U_1F4C0 };
    ("file_folder") => { &twemoji_assets::png::codes::U_1F4C1 };
    ("open_file_folder") => { &twemoji_assets::png::codes::U_1F4C2 };
    ("page_with_curl") => { &twemoji_assets::png::codes::U_1F4C3 };
    ("page_facing_up") => { &twemoji_assets::png::codes::U_1F4C4 };
    ("date") => { &twemoji_assets::png::codes::U_1F4C5 };
    ("calendar") => { &twemoji_assets::png::codes::U_1F4C6 };
    ("card_index") => { &twemoji_assets::png::codes::U_1F4C7 };
    ("chart_increasing") => { &twemoji_assets::png::codes::U_1F4C8 };
    ("chart_with_upwards_trend") => { &twemoji_assets::png::codes::U_1F4C8 };
    ("chart_decreasing") => { &twemoji_assets::png::codes::U_1F4C9 };
    ("chart_with_downwards_trend") => { &twemoji_assets::png::codes::U_1F4C9 };
    ("bar_chart") => { &twemoji_assets::png::codes::U_1F4CA };
    ("clipboard") => { &twemoji_assets::png::codes::U_1F4CB };
    ("pushpin") => { &twemoji_assets::png::codes::U_1F4CC };
    ("round_pushpin") => { &twemoji_assets::png::codes::U_1F4CD };
    ("paperclip") => { &twemoji_assets::png::codes::U_1F4CE };
    ("straight_ruler") => { &twemoji_assets::png::codes::U_1F4CF };
    ("triangular_ruler") => { &twemoji_assets::png::codes::U_1F4D0 };
    ("bookmark_tabs") => { &twemoji_assets::png::codes::U_1F4D1 };
    ("ledger") => { &twemoji_assets::png::codes::U_1F4D2 };
    ("notebook") => { &twemoji_assets::png::codes::U_1F4D3 };
    ("notebook_with_decorative_cover") => { &twemoji_assets::png::codes::U_1F4D4 };
    ("closed_book") => { &twemoji_assets::png::codes::U_1F4D5 };
    ("book") => { &twemoji_assets::png::codes::U_1F4D6 };
    ("open_book") => { &twemoji_assets::png::codes::U_1F4D6 };
    ("green_book") => { &twemoji_assets::png::codes::U_1F4D7 };
    ("blue_book") => { &twemoji_assets::png::codes::U_1F4D8 };
    ("orange_book") => { &twemoji_assets::png::codes::U_1F4D9 };
    ("books") => { &twemoji_assets::png::codes::U_1F4DA };
    ("name_badge") => { &twemoji_assets::png::codes::U_1F4DB };
    ("scroll") => { &twemoji_assets::png::codes::U_1F4DC };
    ("memo") => { &twemoji_assets::png::codes::U_1F4DD };
    ("telephone_receiver") => { &twemoji_assets::png::codes::U_1F4DE };
    ("pager") => { &twemoji_assets::png::codes::U_1F4DF };
    ("fax") => { &twemoji_assets::png::codes::U_1F4E0 };
    ("fax_machine") => { &twemoji_assets::png::codes::U_1F4E0 };
    ("satellite_antenna") => { &twemoji_assets::png::codes::U_1F4E1 };
    ("loudspeaker") => { &twemoji_assets::png::codes::U_1F4E2 };
    ("mega") => { &twemoji_assets::png::codes::U_1F4E3 };
    ("megaphone") => { &twemoji_assets::png::codes::U_1F4E3 };
    ("outbox_tray") => { &twemoji_assets::png::codes::U_1F4E4 };
    ("inbox_tray") => { &twemoji_assets::png::codes::U_1F4E5 };
    ("package") => { &twemoji_assets::png::codes::U_1F4E6 };
    ("e-mail") => { &twemoji_assets::png::codes::U_1F4E7 };
    ("email") => { &twemoji_assets::png::codes::U_1F4E7 };
    ("incoming_envelope") => { &twemoji_assets::png::codes::U_1F4E8 };
    ("envelope_with_arrow") => { &twemoji_assets::png::codes::U_1F4E9 };
    ("mailbox_closed") => { &twemoji_assets::png::codes::U_1F4EA };
    ("mailbox") => { &twemoji_assets::png::codes::U_1F4EB };
    ("mailbox_with_mail") => { &twemoji_assets::png::codes::U_1F4EC };
    ("mailbox_with_no_mail") => { &twemoji_assets::png::codes::U_1F4ED };
    ("postbox") => { &twemoji_assets::png::codes::U_1F4EE };
    ("postal_horn") => { &twemoji_assets::png::codes::U_1F4EF };
    ("newspaper") => { &twemoji_assets::png::codes::U_1F4F0 };
    ("android") => { &twemoji_assets::png::codes::U_1F4F1 };
    ("iphone") => { &twemoji_assets::png::codes::U_1F4F1 };
    ("mobile_phone") => { &twemoji_assets::png::codes::U_1F4F1 };
    ("calling") => { &twemoji_assets::png::codes::U_1F4F2 };
    ("mobile_phone_arrow") => { &twemoji_assets::png::codes::U_1F4F2 };
    ("vibration_mode") => { &twemoji_assets::png::codes::U_1F4F3 };
    ("mobile_phone_off") => { &twemoji_assets::png::codes::U_1F4F4 };
    ("no_mobile_phones") => { &twemoji_assets::png::codes::U_1F4F5 };
    ("antenna_bars") => { &twemoji_assets::png::codes::U_1F4F6 };
    ("signal_strength") => { &twemoji_assets::png::codes::U_1F4F6 };
    ("camera") => { &twemoji_assets::png::codes::U_1F4F7 };
    ("camera_with_flash") => { &twemoji_assets::png::codes::U_1F4F8 };
    ("video_camera") => { &twemoji_assets::png::codes::U_1F4F9 };
    ("tv") => { &twemoji_assets::png::codes::U_1F4FA };
    ("radio") => { &twemoji_assets::png::codes::U_1F4FB };
    ("vhs") => { &twemoji_assets::png::codes::U_1F4FC };
    ("videocassette") => { &twemoji_assets::png::codes::U_1F4FC };
    ("film_projector") => { &twemoji_assets::png::codes::U_1F4FD };
    ("prayer_beads") => { &twemoji_assets::png::codes::U_1F4FF };
    ("shuffle") => { &twemoji_assets::png::codes::U_1F500 };
    ("twisted_rightwards_arrows") => { &twemoji_assets::png::codes::U_1F500 };
    ("repeat") => { &twemoji_assets::png::codes::U_1F501 };
    ("repeat_one") => { &twemoji_assets::png::codes::U_1F502 };
    ("arrows_clockwise") => { &twemoji_assets::png::codes::U_1F503 };
    ("clockwise") => { &twemoji_assets::png::codes::U_1F503 };
    ("arrows_counterclockwise") => { &twemoji_assets::png::codes::U_1F504 };
    ("counterclockwise") => { &twemoji_assets::png::codes::U_1F504 };
    ("dim_button") => { &twemoji_assets::png::codes::U_1F505 };
    ("low_brightness") => { &twemoji_assets::png::codes::U_1F505 };
    ("bright_button") => { &twemoji_assets::png::codes::U_1F506 };
    ("high_brightness") => { &twemoji_assets::png::codes::U_1F506 };
    ("mute") => { &twemoji_assets::png::codes::U_1F507 };
    ("no_sound") => { &twemoji_assets::png::codes::U_1F507 };
    ("low_volume") => { &twemoji_assets::png::codes::U_1F508 };
    ("quiet_sound") => { &twemoji_assets::png::codes::U_1F508 };
    ("speaker") => { &twemoji_assets::png::codes::U_1F508 };
    ("medium_volumne") => { &twemoji_assets::png::codes::U_1F509 };
    ("sound") => { &twemoji_assets::png::codes::U_1F509 };
    ("high_volume") => { &twemoji_assets::png::codes::U_1F50A };
    ("loud_sound") => { &twemoji_assets::png::codes::U_1F50A };
    ("battery") => { &twemoji_assets::png::codes::U_1F50B };
    ("electric_plug") => { &twemoji_assets::png::codes::U_1F50C };
    ("mag") => { &twemoji_assets::png::codes::U_1F50D };
    ("mag_right") => { &twemoji_assets::png::codes::U_1F50E };
    ("lock_with_ink_pen") => { &twemoji_assets::png::codes::U_1F50F };
    ("locked_with_pen") => { &twemoji_assets::png::codes::U_1F50F };
    ("closed_lock_with_key") => { &twemoji_assets::png::codes::U_1F510 };
    ("locked_with_key") => { &twemoji_assets::png::codes::U_1F510 };
    ("key") => { &twemoji_assets::png::codes::U_1F511 };
    ("lock") => { &twemoji_assets::png::codes::U_1F512 };
    ("locked") => { &twemoji_assets::png::codes::U_1F512 };
    ("unlock") => { &twemoji_assets::png::codes::U_1F513 };
    ("unlocked") => { &twemoji_assets::png::codes::U_1F513 };
    ("bell") => { &twemoji_assets::png::codes::U_1F514 };
    ("no_bell") => { &twemoji_assets::png::codes::U_1F515 };
    ("bookmark") => { &twemoji_assets::png::codes::U_1F516 };
    ("link") => { &twemoji_assets::png::codes::U_1F517 };
    ("radio_button") => { &twemoji_assets::png::codes::U_1F518 };
    ("back") => { &twemoji_assets::png::codes::U_1F519 };
    ("end") => { &twemoji_assets::png::codes::U_1F51A };
    ("on") => { &twemoji_assets::png::codes::U_1F51B };
    ("soon") => { &twemoji_assets::png::codes::U_1F51C };
    ("top") => { &twemoji_assets::png::codes::U_1F51D };
    ("no_one_under_18") => { &twemoji_assets::png::codes::U_1F51E };
    ("underage") => { &twemoji_assets::png::codes::U_1F51E };
    ("ten") => { &twemoji_assets::png::codes::U_1F51F };
    ("capital_abcd") => { &twemoji_assets::png::codes::U_1F520 };
    ("abcd") => { &twemoji_assets::png::codes::U_1F521 };
    ("1234") => { &twemoji_assets::png::codes::U_1F522 };
    ("symbols") => { &twemoji_assets::png::codes::U_1F523 };
    ("abc") => { &twemoji_assets::png::codes::U_1F524 };
    ("fire") => { &twemoji_assets::png::codes::U_1F525 };
    ("flashlight") => { &twemoji_assets::png::codes::U_1F526 };
    ("wrench") => { &twemoji_assets::png::codes::U_1F527 };
    ("hammer") => { &twemoji_assets::png::codes::U_1F528 };
    ("nut_and_bolt") => { &twemoji_assets::png::codes::U_1F529 };
    ("knife") => { &twemoji_assets::png::codes::U_1F52A };
    ("gun") => { &twemoji_assets::png::codes::U_1F52B };
    ("pistol") => { &twemoji_assets::png::codes::U_1F52B };
    ("microscope") => { &twemoji_assets::png::codes::U_1F52C };
    ("telescope") => { &twemoji_assets::png::codes::U_1F52D };
    ("crystal_ball") => { &twemoji_assets::png::codes::U_1F52E };
    ("six_pointed_star") => { &twemoji_assets::png::codes::U_1F52F };
    ("beginner") => { &twemoji_assets::png::codes::U_1F530 };
    ("trident") => { &twemoji_assets::png::codes::U_1F531 };
    ("black_square_button") => { &twemoji_assets::png::codes::U_1F532 };
    ("white_square_button") => { &twemoji_assets::png::codes::U_1F533 };
    ("red_circle") => { &twemoji_assets::png::codes::U_1F534 };
    ("blue_circle") => { &twemoji_assets::png::codes::U_1F535 };
    ("large_orange_diamond") => { &twemoji_assets::png::codes::U_1F536 };
    ("large_blue_diamond") => { &twemoji_assets::png::codes::U_1F537 };
    ("small_orange_diamond") => { &twemoji_assets::png::codes::U_1F538 };
    ("small_blue_diamond") => { &twemoji_assets::png::codes::U_1F539 };
    ("small_red_triangle") => { &twemoji_assets::png::codes::U_1F53A };
    ("small_red_triangle_down") => { &twemoji_assets::png::codes::U_1F53B };
    ("arrow_up_small") => { &twemoji_assets::png::codes::U_1F53C };
    ("up") => { &twemoji_assets::png::codes::U_1F53C };
    ("arrow_down_small") => { &twemoji_assets::png::codes::U_1F53D };
    ("down") => { &twemoji_assets::png::codes::U_1F53D };
    ("om") => { &twemoji_assets::png::codes::U_1F549 };
    ("dove") => { &twemoji_assets::png::codes::U_1F54A };
    ("kaaba") => { &twemoji_assets::png::codes::U_1F54B };
    ("mosque") => { &twemoji_assets::png::codes::U_1F54C };
    ("synagogue") => { &twemoji_assets::png::codes::U_1F54D };
    ("menorah") => { &twemoji_assets::png::codes::U_1F54E };
    ("clock1") => { &twemoji_assets::png::codes::U_1F550 };
    ("clock2") => { &twemoji_assets::png::codes::U_1F551 };
    ("clock3") => { &twemoji_assets::png::codes::U_1F552 };
    ("clock4") => { &twemoji_assets::png::codes::U_1F553 };
    ("clock5") => { &twemoji_assets::png::codes::U_1F554 };
    ("clock6") => { &twemoji_assets::png::codes::U_1F555 };
    ("clock7") => { &twemoji_assets::png::codes::U_1F556 };
    ("clock8") => { &twemoji_assets::png::codes::U_1F557 };
    ("clock9") => { &twemoji_assets::png::codes::U_1F558 };
    ("clock10") => { &twemoji_assets::png::codes::U_1F559 };
    ("clock11") => { &twemoji_assets::png::codes::U_1F55A };
    ("clock12") => { &twemoji_assets::png::codes::U_1F55B };
    ("clock130") => { &twemoji_assets::png::codes::U_1F55C };
    ("clock230") => { &twemoji_assets::png::codes::U_1F55D };
    ("clock330") => { &twemoji_assets::png::codes::U_1F55E };
    ("clock430") => { &twemoji_assets::png::codes::U_1F55F };
    ("clock530") => { &twemoji_assets::png::codes::U_1F560 };
    ("clock630") => { &twemoji_assets::png::codes::U_1F561 };
    ("clock730") => { &twemoji_assets::png::codes::U_1F562 };
    ("clock830") => { &twemoji_assets::png::codes::U_1F563 };
    ("clock930") => { &twemoji_assets::png::codes::U_1F564 };
    ("clock1030") => { &twemoji_assets::png::codes::U_1F565 };
    ("clock1130") => { &twemoji_assets::png::codes::U_1F566 };
    ("clock1230") => { &twemoji_assets::png::codes::U_1F567 };
    ("candle") => { &twemoji_assets::png::codes::U_1F56F };
    ("clock") => { &twemoji_assets::png::codes::U_1F570 };
    ("hole") => { &twemoji_assets::png::codes::U_1F573 };
    ("levitate_tone1") => { &twemoji_assets::png::codes::U_1F574_1F3FB };
    ("levitating_tone1") => { &twemoji_assets::png::codes::U_1F574_1F3FB };
    ("person_in_suit_levitating_tone1") => { &twemoji_assets::png::codes::U_1F574_1F3FB };
    ("levitate_tone2") => { &twemoji_assets::png::codes::U_1F574_1F3FC };
    ("levitating_tone2") => { &twemoji_assets::png::codes::U_1F574_1F3FC };
    ("person_in_suit_levitating_tone2") => { &twemoji_assets::png::codes::U_1F574_1F3FC };
    ("levitate_tone3") => { &twemoji_assets::png::codes::U_1F574_1F3FD };
    ("levitating_tone3") => { &twemoji_assets::png::codes::U_1F574_1F3FD };
    ("person_in_suit_levitating_tone3") => { &twemoji_assets::png::codes::U_1F574_1F3FD };
    ("levitate_tone4") => { &twemoji_assets::png::codes::U_1F574_1F3FE };
    ("levitating_tone4") => { &twemoji_assets::png::codes::U_1F574_1F3FE };
    ("person_in_suit_levitating_tone4") => { &twemoji_assets::png::codes::U_1F574_1F3FE };
    ("levitate_tone5") => { &twemoji_assets::png::codes::U_1F574_1F3FF };
    ("levitating_tone5") => { &twemoji_assets::png::codes::U_1F574_1F3FF };
    ("person_in_suit_levitating_tone5") => { &twemoji_assets::png::codes::U_1F574_1F3FF };
    ("levitate") => { &twemoji_assets::png::codes::U_1F574 };
    ("levitating") => { &twemoji_assets::png::codes::U_1F574 };
    ("person_in_suit_levitating") => { &twemoji_assets::png::codes::U_1F574 };
    ("woman_detective_tone1") => { &twemoji_assets::png::codes::U_1F575_1F3FB_200D_2640_FE0F };
    ("man_detective_tone1") => { &twemoji_assets::png::codes::U_1F575_1F3FB_200D_2642_FE0F };
    ("detective_tone1") => { &twemoji_assets::png::codes::U_1F575_1F3FB };
    ("woman_detective_tone2") => { &twemoji_assets::png::codes::U_1F575_1F3FC_200D_2640_FE0F };
    ("man_detective_tone2") => { &twemoji_assets::png::codes::U_1F575_1F3FC_200D_2642_FE0F };
    ("detective_tone2") => { &twemoji_assets::png::codes::U_1F575_1F3FC };
    ("woman_detective_tone3") => { &twemoji_assets::png::codes::U_1F575_1F3FD_200D_2640_FE0F };
    ("man_detective_tone3") => { &twemoji_assets::png::codes::U_1F575_1F3FD_200D_2642_FE0F };
    ("detective_tone3") => { &twemoji_assets::png::codes::U_1F575_1F3FD };
    ("woman_detective_tone4") => { &twemoji_assets::png::codes::U_1F575_1F3FE_200D_2640_FE0F };
    ("man_detective_tone4") => { &twemoji_assets::png::codes::U_1F575_1F3FE_200D_2642_FE0F };
    ("detective_tone4") => { &twemoji_assets::png::codes::U_1F575_1F3FE };
    ("woman_detective_tone5") => { &twemoji_assets::png::codes::U_1F575_1F3FF_200D_2640_FE0F };
    ("man_detective_tone5") => { &twemoji_assets::png::codes::U_1F575_1F3FF_200D_2642_FE0F };
    ("detective_tone5") => { &twemoji_assets::png::codes::U_1F575_1F3FF };
    ("woman_detective") => { &twemoji_assets::png::codes::U_1F575_FE0F_200D_2640_FE0F };
    ("man_detective") => { &twemoji_assets::png::codes::U_1F575_FE0F_200D_2642_FE0F };
    ("detective") => { &twemoji_assets::png::codes::U_1F575 };
    ("sunglasses") => { &twemoji_assets::png::codes::U_1F576 };
    ("spider") => { &twemoji_assets::png::codes::U_1F577 };
    ("spider_web") => { &twemoji_assets::png::codes::U_1F578 };
    ("joystick") => { &twemoji_assets::png::codes::U_1F579 };
    ("man_dancing_tone1") => { &twemoji_assets::png::codes::U_1F57A_1F3FB };
    ("man_dancing_tone2") => { &twemoji_assets::png::codes::U_1F57A_1F3FC };
    ("man_dancing_tone3") => { &twemoji_assets::png::codes::U_1F57A_1F3FD };
    ("man_dancing_tone4") => { &twemoji_assets::png::codes::U_1F57A_1F3FE };
    ("man_dancing_tone5") => { &twemoji_assets::png::codes::U_1F57A_1F3FF };
    ("man_dancing") => { &twemoji_assets::png::codes::U_1F57A };
    ("paperclips") => { &twemoji_assets::png::codes::U_1F587 };
    ("pen") => { &twemoji_assets::png::codes::U_1F58A };
    ("fountain_pen") => { &twemoji_assets::png::codes::U_1F58B };
    ("paintbrush") => { &twemoji_assets::png::codes::U_1F58C };
    ("crayon") => { &twemoji_assets::png::codes::U_1F58D };
    ("raised_hand_with_fingers_splayed_tone1") => { &twemoji_assets::png::codes::U_1F590_1F3FB };
    ("raised_hand_with_fingers_splayed_tone2") => { &twemoji_assets::png::codes::U_1F590_1F3FC };
    ("raised_hand_with_fingers_splayed_tone3") => { &twemoji_assets::png::codes::U_1F590_1F3FD };
    ("raised_hand_with_fingers_splayed_tone4") => { &twemoji_assets::png::codes::U_1F590_1F3FE };
    ("raised_hand_with_fingers_splayed_tone5") => { &twemoji_assets::png::codes::U_1F590_1F3FF };
    ("raised_hand_with_fingers_splayed") => { &twemoji_assets::png::codes::U_1F590 };
    ("middle_finger_tone1") => { &twemoji_assets::png::codes::U_1F595_1F3FB };
    ("middle_finger_tone2") => { &twemoji_assets::png::codes::U_1F595_1F3FC };
    ("middle_finger_tone3") => { &twemoji_assets::png::codes::U_1F595_1F3FD };
    ("middle_finger_tone4") => { &twemoji_assets::png::codes::U_1F595_1F3FE };
    ("middle_finger_tone5") => { &twemoji_assets::png::codes::U_1F595_1F3FF };
    ("middle_finger") => { &twemoji_assets::png::codes::U_1F595 };
    ("vulcan_tone1") => { &twemoji_assets::png::codes::U_1F596_1F3FB };
    ("vulcan_tone2") => { &twemoji_assets::png::codes::U_1F596_1F3FC };
    ("vulcan_tone3") => { &twemoji_assets::png::codes::U_1F596_1F3FD };
    ("vulcan_tone4") => { &twemoji_assets::png::codes::U_1F596_1F3FE };
    ("vulcan_tone5") => { &twemoji_assets::png::codes::U_1F596_1F3FF };
    ("vulcan") => { &twemoji_assets::png::codes::U_1F596 };
    ("black_heart") => { &twemoji_assets::png::codes::U_1F5A4 };
    ("computer") => { &twemoji_assets::png::codes::U_1F5A5 };
    ("desktop_computer") => { &twemoji_assets::png::codes::U_1F5A5 };
    ("printer") => { &twemoji_assets::png::codes::U_1F5A8 };
    ("computer_mouse") => { &twemoji_assets::png::codes::U_1F5B1 };
    ("trackball") => { &twemoji_assets::png::codes::U_1F5B2 };
    ("frame_with_picture") => { &twemoji_assets::png::codes::U_1F5BC };
    ("framed_picture") => { &twemoji_assets::png::codes::U_1F5BC };
    ("card_index_dividers") => { &twemoji_assets::png::codes::U_1F5C2 };
    ("card_file_box") => { &twemoji_assets::png::codes::U_1F5C3 };
    ("file_cabinet") => { &twemoji_assets::png::codes::U_1F5C4 };
    ("trashcan") => { &twemoji_assets::png::codes::U_1F5D1 };
    ("wastebasket") => { &twemoji_assets::png::codes::U_1F5D1 };
    ("notepad_spiral") => { &twemoji_assets::png::codes::U_1F5D2 };
    ("calendar_spiral") => { &twemoji_assets::png::codes::U_1F5D3 };
    ("clamp") => { &twemoji_assets::png::codes::U_1F5DC };
    ("compression") => { &twemoji_assets::png::codes::U_1F5DC };
    ("old_key") => { &twemoji_assets::png::codes::U_1F5DD };
    ("rolled_up_newspaper") => { &twemoji_assets::png::codes::U_1F5DE };
    ("dagger") => { &twemoji_assets::png::codes::U_1F5E1 };
    ("speaking_head") => { &twemoji_assets::png::codes::U_1F5E3 };
    ("left_speech_bubble") => { &twemoji_assets::png::codes::U_1F5E8 };
    ("right_anger_bubble") => { &twemoji_assets::png::codes::U_1F5EF };
    ("ballot_box") => { &twemoji_assets::png::codes::U_1F5F3 };
    ("world_map") => { &twemoji_assets::png::codes::U_1F5FA };
    ("mount_fuji") => { &twemoji_assets::png::codes::U_1F5FB };
    ("tokyo_tower") => { &twemoji_assets::png::codes::U_1F5FC };
    ("statue_of_liberty") => { &twemoji_assets::png::codes::U_1F5FD };
    ("japan_map") => { &twemoji_assets::png::codes::U_1F5FE };
    ("moai") => { &twemoji_assets::png::codes::U_1F5FF };
    ("moyai") => { &twemoji_assets::png::codes::U_1F5FF };
    ("grinning") => { &twemoji_assets::png::codes::U_1F600 };
    ("grinning_face") => { &twemoji_assets::png::codes::U_1F600 };
    ("beaming_face") => { &twemoji_assets::png::codes::U_1F601 };
    ("grin") => { &twemoji_assets::png::codes::U_1F601 };
    ("joy") => { &twemoji_assets::png::codes::U_1F602 };
    ("lmao") => { &twemoji_assets::png::codes::U_1F602 };
    ("tears_of_joy") => { &twemoji_assets::png::codes::U_1F602 };
    ("grinning_face_with_big_eyes") => { &twemoji_assets::png::codes::U_1F603 };
    ("smiley") => { &twemoji_assets::png::codes::U_1F603 };
    ("grinning_face_with_closed_eyes") => { &twemoji_assets::png::codes::U_1F604 };
    ("smile") => { &twemoji_assets::png::codes::U_1F604 };
    ("grinning_face_with_sweat") => { &twemoji_assets::png::codes::U_1F605 };
    ("sweat_smile") => { &twemoji_assets::png::codes::U_1F605 };
    ("laughing") => { &twemoji_assets::png::codes::U_1F606 };
    ("lol") => { &twemoji_assets::png::codes::U_1F606 };
    ("satisfied") => { &twemoji_assets::png::codes::U_1F606 };
    ("squinting_face") => { &twemoji_assets::png::codes::U_1F606 };
    ("halo") => { &twemoji_assets::png::codes::U_1F607 };
    ("innocent") => { &twemoji_assets::png::codes::U_1F607 };
    ("smiling_imp") => { &twemoji_assets::png::codes::U_1F608 };
    ("wink") => { &twemoji_assets::png::codes::U_1F609 };
    ("winking_face") => { &twemoji_assets::png::codes::U_1F609 };
    ("blush") => { &twemoji_assets::png::codes::U_1F60A };
    ("smiling_face_with_closed_eyes") => { &twemoji_assets::png::codes::U_1F60A };
    ("savoring_food") => { &twemoji_assets::png::codes::U_1F60B };
    ("yum") => { &twemoji_assets::png::codes::U_1F60B };
    ("relieved") => { &twemoji_assets::png::codes::U_1F60C };
    ("relieved_face") => { &twemoji_assets::png::codes::U_1F60C };
    ("heart_eyes") => { &twemoji_assets::png::codes::U_1F60D };
    ("smiling_face_with_heart_eyes") => { &twemoji_assets::png::codes::U_1F60D };
    ("smiling_face_with_sunglasses") => { &twemoji_assets::png::codes::U_1F60E };
    ("sunglasses_cool") => { &twemoji_assets::png::codes::U_1F60E };
    ("too_cool") => { &twemoji_assets::png::codes::U_1F60E };
    ("smirk") => { &twemoji_assets::png::codes::U_1F60F };
    ("smirking") => { &twemoji_assets::png::codes::U_1F60F };
    ("smirking_face") => { &twemoji_assets::png::codes::U_1F60F };
    ("neutral") => { &twemoji_assets::png::codes::U_1F610 };
    ("neutral_face") => { &twemoji_assets::png::codes::U_1F610 };
    ("expressionless") => { &twemoji_assets::png::codes::U_1F611 };
    ("expressionless_face") => { &twemoji_assets::png::codes::U_1F611 };
    ("unamused") => { &twemoji_assets::png::codes::U_1F612 };
    ("unamused_face") => { &twemoji_assets::png::codes::U_1F612 };
    ("downcast_face") => { &twemoji_assets::png::codes::U_1F613 };
    ("sweat") => { &twemoji_assets::png::codes::U_1F613 };
    ("pensive") => { &twemoji_assets::png::codes::U_1F614 };
    ("pensive_face") => { &twemoji_assets::png::codes::U_1F614 };
    ("confused") => { &twemoji_assets::png::codes::U_1F615 };
    ("confused_face") => { &twemoji_assets::png::codes::U_1F615 };
    ("confounded") => { &twemoji_assets::png::codes::U_1F616 };
    ("confounded_face") => { &twemoji_assets::png::codes::U_1F616 };
    ("kissing") => { &twemoji_assets::png::codes::U_1F617 };
    ("kissing_face") => { &twemoji_assets::png::codes::U_1F617 };
    ("blowing_a_kiss") => { &twemoji_assets::png::codes::U_1F618 };
    ("kissing_heart") => { &twemoji_assets::png::codes::U_1F618 };
    ("kissing_face_with_smiling_eyes") => { &twemoji_assets::png::codes::U_1F619 };
    ("kissing_smiling_eyes") => { &twemoji_assets::png::codes::U_1F619 };
    ("kissing_closed_eyes") => { &twemoji_assets::png::codes::U_1F61A };
    ("kissing_face_with_closed_eyes") => { &twemoji_assets::png::codes::U_1F61A };
    ("face_with_tongue") => { &twemoji_assets::png::codes::U_1F61B };
    ("stuck_out_tongue") => { &twemoji_assets::png::codes::U_1F61B };
    ("stuck_out_tongue_winking_eye") => { &twemoji_assets::png::codes::U_1F61C };
    ("stuck_out_tongue_closed_eyes") => { &twemoji_assets::png::codes::U_1F61D };
    ("disappointed") => { &twemoji_assets::png::codes::U_1F61E };
    ("disappointed_face") => { &twemoji_assets::png::codes::U_1F61E };
    ("worried") => { &twemoji_assets::png::codes::U_1F61F };
    ("worried_face") => { &twemoji_assets::png::codes::U_1F61F };
    ("angry") => { &twemoji_assets::png::codes::U_1F620 };
    ("angry_face") => { &twemoji_assets::png::codes::U_1F620 };
    ("pout") => { &twemoji_assets::png::codes::U_1F621 };
    ("pouting_face") => { &twemoji_assets::png::codes::U_1F621 };
    ("rage") => { &twemoji_assets::png::codes::U_1F621 };
    ("cry") => { &twemoji_assets::png::codes::U_1F622 };
    ("crying_face") => { &twemoji_assets::png::codes::U_1F622 };
    ("persevere") => { &twemoji_assets::png::codes::U_1F623 };
    ("persevering_face") => { &twemoji_assets::png::codes::U_1F623 };
    ("nose_steam") => { &twemoji_assets::png::codes::U_1F624 };
    ("triumph") => { &twemoji_assets::png::codes::U_1F624 };
    ("disappointed_relieved") => { &twemoji_assets::png::codes::U_1F625 };
    ("sad_relieved_face") => { &twemoji_assets::png::codes::U_1F625 };
    ("frowning") => { &twemoji_assets::png::codes::U_1F626 };
    ("frowning_face") => { &twemoji_assets::png::codes::U_1F626 };
    ("anguished") => { &twemoji_assets::png::codes::U_1F627 };
    ("anguished_face") => { &twemoji_assets::png::codes::U_1F627 };
    ("fearful") => { &twemoji_assets::png::codes::U_1F628 };
    ("fearful_face") => { &twemoji_assets::png::codes::U_1F628 };
    ("weary") => { &twemoji_assets::png::codes::U_1F629 };
    ("weary_face") => { &twemoji_assets::png::codes::U_1F629 };
    ("sleepy") => { &twemoji_assets::png::codes::U_1F62A };
    ("sleepy_face") => { &twemoji_assets::png::codes::U_1F62A };
    ("tired") => { &twemoji_assets::png::codes::U_1F62B };
    ("tired_face") => { &twemoji_assets::png::codes::U_1F62B };
    ("grimacing") => { &twemoji_assets::png::codes::U_1F62C };
    ("grimacing_face") => { &twemoji_assets::png::codes::U_1F62C };
    ("loudly_crying_face") => { &twemoji_assets::png::codes::U_1F62D };
    ("sob") => { &twemoji_assets::png::codes::U_1F62D };
    ("exhale") => { &twemoji_assets::png::codes::U_1F62E_200D_1F4A8 };
    ("exhaling") => { &twemoji_assets::png::codes::U_1F62E_200D_1F4A8 };
    ("face_with_open_mouth") => { &twemoji_assets::png::codes::U_1F62E };
    ("open_mouth") => { &twemoji_assets::png::codes::U_1F62E };
    ("hushed") => { &twemoji_assets::png::codes::U_1F62F };
    ("hushed_face") => { &twemoji_assets::png::codes::U_1F62F };
    ("anxious") => { &twemoji_assets::png::codes::U_1F630 };
    ("anxious_face") => { &twemoji_assets::png::codes::U_1F630 };
    ("cold_sweat") => { &twemoji_assets::png::codes::U_1F630 };
    ("scream") => { &twemoji_assets::png::codes::U_1F631 };
    ("screaming_in_fear") => { &twemoji_assets::png::codes::U_1F631 };
    ("astonished") => { &twemoji_assets::png::codes::U_1F632 };
    ("astonished_face") => { &twemoji_assets::png::codes::U_1F632 };
    ("flushed") => { &twemoji_assets::png::codes::U_1F633 };
    ("flushed_face") => { &twemoji_assets::png::codes::U_1F633 };
    ("sleeping") => { &twemoji_assets::png::codes::U_1F634 };
    ("sleeping_face") => { &twemoji_assets::png::codes::U_1F634 };
    ("dizzy_eyes") => { &twemoji_assets::png::codes::U_1F635_200D_1F4AB };
    ("dizzy_face") => { &twemoji_assets::png::codes::U_1F635 };
    ("knocked_out") => { &twemoji_assets::png::codes::U_1F635 };
    ("in_clouds") => { &twemoji_assets::png::codes::U_1F636_200D_1F32B_FE0F };
    ("no_mouth") => { &twemoji_assets::png::codes::U_1F636 };
    ("mask") => { &twemoji_assets::png::codes::U_1F637 };
    ("medical_mask") => { &twemoji_assets::png::codes::U_1F637 };
    ("grinning_cat_with_closed_eyes") => { &twemoji_assets::png::codes::U_1F638 };
    ("smile_cat") => { &twemoji_assets::png::codes::U_1F638 };
    ("joy_cat") => { &twemoji_assets::png::codes::U_1F639 };
    ("tears_of_joy_cat") => { &twemoji_assets::png::codes::U_1F639 };
    ("grinning_cat") => { &twemoji_assets::png::codes::U_1F63A };
    ("smiley_cat") => { &twemoji_assets::png::codes::U_1F63A };
    ("heart_eyes_cat") => { &twemoji_assets::png::codes::U_1F63B };
    ("smiling_cat_with_heart_eyes") => { &twemoji_assets::png::codes::U_1F63B };
    ("smirk_cat") => { &twemoji_assets::png::codes::U_1F63C };
    ("wry_smile_cat") => { &twemoji_assets::png::codes::U_1F63C };
    ("kissing_cat") => { &twemoji_assets::png::codes::U_1F63D };
    ("pouting_cat") => { &twemoji_assets::png::codes::U_1F63E };
    ("crying_cat") => { &twemoji_assets::png::codes::U_1F63F };
    ("scream_cat") => { &twemoji_assets::png::codes::U_1F640 };
    ("weary_cat") => { &twemoji_assets::png::codes::U_1F640 };
    ("slightly_frowning_face") => { &twemoji_assets::png::codes::U_1F641 };
    ("slightly_smiling_face") => { &twemoji_assets::png::codes::U_1F642 };
    ("upside_down_face") => { &twemoji_assets::png::codes::U_1F643 };
    ("rolling_eyes") => { &twemoji_assets::png::codes::U_1F644 };
    ("woman_gesturing_no_tone1") => { &twemoji_assets::png::codes::U_1F645_1F3FB_200D_2640_FE0F };
    ("man_gesturing_no_tone1") => { &twemoji_assets::png::codes::U_1F645_1F3FB_200D_2642_FE0F };
    ("no_good_tone1") => { &twemoji_assets::png::codes::U_1F645_1F3FB };
    ("person_gesturing_no_tone1") => { &twemoji_assets::png::codes::U_1F645_1F3FB };
    ("woman_gesturing_no_tone2") => { &twemoji_assets::png::codes::U_1F645_1F3FC_200D_2640_FE0F };
    ("man_gesturing_no_tone2") => { &twemoji_assets::png::codes::U_1F645_1F3FC_200D_2642_FE0F };
    ("no_good_tone2") => { &twemoji_assets::png::codes::U_1F645_1F3FC };
    ("person_gesturing_no_tone2") => { &twemoji_assets::png::codes::U_1F645_1F3FC };
    ("woman_gesturing_no_tone3") => { &twemoji_assets::png::codes::U_1F645_1F3FD_200D_2640_FE0F };
    ("man_gesturing_no_tone3") => { &twemoji_assets::png::codes::U_1F645_1F3FD_200D_2642_FE0F };
    ("no_good_tone3") => { &twemoji_assets::png::codes::U_1F645_1F3FD };
    ("person_gesturing_no_tone3") => { &twemoji_assets::png::codes::U_1F645_1F3FD };
    ("woman_gesturing_no_tone4") => { &twemoji_assets::png::codes::U_1F645_1F3FE_200D_2640_FE0F };
    ("man_gesturing_no_tone4") => { &twemoji_assets::png::codes::U_1F645_1F3FE_200D_2642_FE0F };
    ("no_good_tone4") => { &twemoji_assets::png::codes::U_1F645_1F3FE };
    ("person_gesturing_no_tone4") => { &twemoji_assets::png::codes::U_1F645_1F3FE };
    ("woman_gesturing_no_tone5") => { &twemoji_assets::png::codes::U_1F645_1F3FF_200D_2640_FE0F };
    ("man_gesturing_no_tone5") => { &twemoji_assets::png::codes::U_1F645_1F3FF_200D_2642_FE0F };
    ("no_good_tone5") => { &twemoji_assets::png::codes::U_1F645_1F3FF };
    ("person_gesturing_no_tone5") => { &twemoji_assets::png::codes::U_1F645_1F3FF };
    ("woman_gesturing_no") => { &twemoji_assets::png::codes::U_1F645_200D_2640_FE0F };
    ("man_gesturing_no") => { &twemoji_assets::png::codes::U_1F645_200D_2642_FE0F };
    ("no_good") => { &twemoji_assets::png::codes::U_1F645 };
    ("person_gesturing_no") => { &twemoji_assets::png::codes::U_1F645 };
    ("woman_gesturing_ok_tone1") => { &twemoji_assets::png::codes::U_1F646_1F3FB_200D_2640_FE0F };
    ("man_gesturing_ok_tone1") => { &twemoji_assets::png::codes::U_1F646_1F3FB_200D_2642_FE0F };
    ("all_good_tone1") => { &twemoji_assets::png::codes::U_1F646_1F3FB };
    ("person_gesturing_ok_tone1") => { &twemoji_assets::png::codes::U_1F646_1F3FB };
    ("woman_gesturing_ok_tone2") => { &twemoji_assets::png::codes::U_1F646_1F3FC_200D_2640_FE0F };
    ("man_gesturing_ok_tone2") => { &twemoji_assets::png::codes::U_1F646_1F3FC_200D_2642_FE0F };
    ("all_good_tone2") => { &twemoji_assets::png::codes::U_1F646_1F3FC };
    ("person_gesturing_ok_tone2") => { &twemoji_assets::png::codes::U_1F646_1F3FC };
    ("woman_gesturing_ok_tone3") => { &twemoji_assets::png::codes::U_1F646_1F3FD_200D_2640_FE0F };
    ("man_gesturing_ok_tone3") => { &twemoji_assets::png::codes::U_1F646_1F3FD_200D_2642_FE0F };
    ("all_good_tone3") => { &twemoji_assets::png::codes::U_1F646_1F3FD };
    ("person_gesturing_ok_tone3") => { &twemoji_assets::png::codes::U_1F646_1F3FD };
    ("woman_gesturing_ok_tone4") => { &twemoji_assets::png::codes::U_1F646_1F3FE_200D_2640_FE0F };
    ("man_gesturing_ok_tone4") => { &twemoji_assets::png::codes::U_1F646_1F3FE_200D_2642_FE0F };
    ("all_good_tone4") => { &twemoji_assets::png::codes::U_1F646_1F3FE };
    ("person_gesturing_ok_tone4") => { &twemoji_assets::png::codes::U_1F646_1F3FE };
    ("woman_gesturing_ok_tone5") => { &twemoji_assets::png::codes::U_1F646_1F3FF_200D_2640_FE0F };
    ("man_gesturing_ok_tone5") => { &twemoji_assets::png::codes::U_1F646_1F3FF_200D_2642_FE0F };
    ("all_good_tone5") => { &twemoji_assets::png::codes::U_1F646_1F3FF };
    ("person_gesturing_ok_tone5") => { &twemoji_assets::png::codes::U_1F646_1F3FF };
    ("woman_gesturing_ok") => { &twemoji_assets::png::codes::U_1F646_200D_2640_FE0F };
    ("man_gesturing_ok") => { &twemoji_assets::png::codes::U_1F646_200D_2642_FE0F };
    ("all_good") => { &twemoji_assets::png::codes::U_1F646 };
    ("person_gesturing_ok") => { &twemoji_assets::png::codes::U_1F646 };
    ("woman_bowing_tone1") => { &twemoji_assets::png::codes::U_1F647_1F3FB_200D_2640_FE0F };
    ("man_bowing_tone1") => { &twemoji_assets::png::codes::U_1F647_1F3FB_200D_2642_FE0F };
    ("bow_tone1") => { &twemoji_assets::png::codes::U_1F647_1F3FB };
    ("person_bowing_tone1") => { &twemoji_assets::png::codes::U_1F647_1F3FB };
    ("woman_bowing_tone2") => { &twemoji_assets::png::codes::U_1F647_1F3FC_200D_2640_FE0F };
    ("man_bowing_tone2") => { &twemoji_assets::png::codes::U_1F647_1F3FC_200D_2642_FE0F };
    ("bow_tone2") => { &twemoji_assets::png::codes::U_1F647_1F3FC };
    ("person_bowing_tone2") => { &twemoji_assets::png::codes::U_1F647_1F3FC };
    ("woman_bowing_tone3") => { &twemoji_assets::png::codes::U_1F647_1F3FD_200D_2640_FE0F };
    ("man_bowing_tone3") => { &twemoji_assets::png::codes::U_1F647_1F3FD_200D_2642_FE0F };
    ("bow_tone3") => { &twemoji_assets::png::codes::U_1F647_1F3FD };
    ("person_bowing_tone3") => { &twemoji_assets::png::codes::U_1F647_1F3FD };
    ("woman_bowing_tone4") => { &twemoji_assets::png::codes::U_1F647_1F3FE_200D_2640_FE0F };
    ("man_bowing_tone4") => { &twemoji_assets::png::codes::U_1F647_1F3FE_200D_2642_FE0F };
    ("bow_tone4") => { &twemoji_assets::png::codes::U_1F647_1F3FE };
    ("person_bowing_tone4") => { &twemoji_assets::png::codes::U_1F647_1F3FE };
    ("woman_bowing_tone5") => { &twemoji_assets::png::codes::U_1F647_1F3FF_200D_2640_FE0F };
    ("man_bowing_tone5") => { &twemoji_assets::png::codes::U_1F647_1F3FF_200D_2642_FE0F };
    ("bow_tone5") => { &twemoji_assets::png::codes::U_1F647_1F3FF };
    ("person_bowing_tone5") => { &twemoji_assets::png::codes::U_1F647_1F3FF };
    ("woman_bowing") => { &twemoji_assets::png::codes::U_1F647_200D_2640_FE0F };
    ("man_bowing") => { &twemoji_assets::png::codes::U_1F647_200D_2642_FE0F };
    ("bow") => { &twemoji_assets::png::codes::U_1F647 };
    ("person_bowing") => { &twemoji_assets::png::codes::U_1F647 };
    ("see_no_evil") => { &twemoji_assets::png::codes::U_1F648 };
    ("hear_no_evil") => { &twemoji_assets::png::codes::U_1F649 };
    ("speak_no_evil") => { &twemoji_assets::png::codes::U_1F64A };
    ("woman_raising_hand_tone1") => { &twemoji_assets::png::codes::U_1F64B_1F3FB_200D_2640_FE0F };
    ("man_raising_hand_tone1") => { &twemoji_assets::png::codes::U_1F64B_1F3FB_200D_2642_FE0F };
    ("person_raising_hand_tone1") => { &twemoji_assets::png::codes::U_1F64B_1F3FB };
    ("woman_raising_hand_tone2") => { &twemoji_assets::png::codes::U_1F64B_1F3FC_200D_2640_FE0F };
    ("man_raising_hand_tone2") => { &twemoji_assets::png::codes::U_1F64B_1F3FC_200D_2642_FE0F };
    ("person_raising_hand_tone2") => { &twemoji_assets::png::codes::U_1F64B_1F3FC };
    ("woman_raising_hand_tone3") => { &twemoji_assets::png::codes::U_1F64B_1F3FD_200D_2640_FE0F };
    ("man_raising_hand_tone3") => { &twemoji_assets::png::codes::U_1F64B_1F3FD_200D_2642_FE0F };
    ("person_raising_hand_tone3") => { &twemoji_assets::png::codes::U_1F64B_1F3FD };
    ("woman_raising_hand_tone4") => { &twemoji_assets::png::codes::U_1F64B_1F3FE_200D_2640_FE0F };
    ("man_raising_hand_tone4") => { &twemoji_assets::png::codes::U_1F64B_1F3FE_200D_2642_FE0F };
    ("person_raising_hand_tone4") => { &twemoji_assets::png::codes::U_1F64B_1F3FE };
    ("woman_raising_hand_tone5") => { &twemoji_assets::png::codes::U_1F64B_1F3FF_200D_2640_FE0F };
    ("man_raising_hand_tone5") => { &twemoji_assets::png::codes::U_1F64B_1F3FF_200D_2642_FE0F };
    ("person_raising_hand_tone5") => { &twemoji_assets::png::codes::U_1F64B_1F3FF };
    ("woman_raising_hand") => { &twemoji_assets::png::codes::U_1F64B_200D_2640_FE0F };
    ("man_raising_hand") => { &twemoji_assets::png::codes::U_1F64B_200D_2642_FE0F };
    ("person_raising_hand") => { &twemoji_assets::png::codes::U_1F64B };
    ("raised_hands_tone1") => { &twemoji_assets::png::codes::U_1F64C_1F3FB };
    ("raised_hands_tone2") => { &twemoji_assets::png::codes::U_1F64C_1F3FC };
    ("raised_hands_tone3") => { &twemoji_assets::png::codes::U_1F64C_1F3FD };
    ("raised_hands_tone4") => { &twemoji_assets::png::codes::U_1F64C_1F3FE };
    ("raised_hands_tone5") => { &twemoji_assets::png::codes::U_1F64C_1F3FF };
    ("raised_hands") => { &twemoji_assets::png::codes::U_1F64C };
    ("woman_frowning_tone1") => { &twemoji_assets::png::codes::U_1F64D_1F3FB_200D_2640_FE0F };
    ("man_frowning_tone1") => { &twemoji_assets::png::codes::U_1F64D_1F3FB_200D_2642_FE0F };
    ("person_frowning_tone1") => { &twemoji_assets::png::codes::U_1F64D_1F3FB };
    ("woman_frowning_tone2") => { &twemoji_assets::png::codes::U_1F64D_1F3FC_200D_2640_FE0F };
    ("man_frowning_tone2") => { &twemoji_assets::png::codes::U_1F64D_1F3FC_200D_2642_FE0F };
    ("person_frowning_tone2") => { &twemoji_assets::png::codes::U_1F64D_1F3FC };
    ("woman_frowning_tone3") => { &twemoji_assets::png::codes::U_1F64D_1F3FD_200D_2640_FE0F };
    ("man_frowning_tone3") => { &twemoji_assets::png::codes::U_1F64D_1F3FD_200D_2642_FE0F };
    ("person_frowning_tone3") => { &twemoji_assets::png::codes::U_1F64D_1F3FD };
    ("woman_frowning_tone4") => { &twemoji_assets::png::codes::U_1F64D_1F3FE_200D_2640_FE0F };
    ("man_frowning_tone4") => { &twemoji_assets::png::codes::U_1F64D_1F3FE_200D_2642_FE0F };
    ("person_frowning_tone4") => { &twemoji_assets::png::codes::U_1F64D_1F3FE };
    ("woman_frowning_tone5") => { &twemoji_assets::png::codes::U_1F64D_1F3FF_200D_2640_FE0F };
    ("man_frowning_tone5") => { &twemoji_assets::png::codes::U_1F64D_1F3FF_200D_2642_FE0F };
    ("person_frowning_tone5") => { &twemoji_assets::png::codes::U_1F64D_1F3FF };
    ("woman_frowning") => { &twemoji_assets::png::codes::U_1F64D_200D_2640_FE0F };
    ("man_frowning") => { &twemoji_assets::png::codes::U_1F64D_200D_2642_FE0F };
    ("person_frowning") => { &twemoji_assets::png::codes::U_1F64D };
    ("woman_pouting_tone1") => { &twemoji_assets::png::codes::U_1F64E_1F3FB_200D_2640_FE0F };
    ("man_pouting_tone1") => { &twemoji_assets::png::codes::U_1F64E_1F3FB_200D_2642_FE0F };
    ("person_pouting_tone1") => { &twemoji_assets::png::codes::U_1F64E_1F3FB };
    ("pouting_tone1") => { &twemoji_assets::png::codes::U_1F64E_1F3FB };
    ("woman_pouting_tone2") => { &twemoji_assets::png::codes::U_1F64E_1F3FC_200D_2640_FE0F };
    ("man_pouting_tone2") => { &twemoji_assets::png::codes::U_1F64E_1F3FC_200D_2642_FE0F };
    ("person_pouting_tone2") => { &twemoji_assets::png::codes::U_1F64E_1F3FC };
    ("pouting_tone2") => { &twemoji_assets::png::codes::U_1F64E_1F3FC };
    ("woman_pouting_tone3") => { &twemoji_assets::png::codes::U_1F64E_1F3FD_200D_2640_FE0F };
    ("man_pouting_tone3") => { &twemoji_assets::png::codes::U_1F64E_1F3FD_200D_2642_FE0F };
    ("person_pouting_tone3") => { &twemoji_assets::png::codes::U_1F64E_1F3FD };
    ("pouting_tone3") => { &twemoji_assets::png::codes::U_1F64E_1F3FD };
    ("woman_pouting_tone4") => { &twemoji_assets::png::codes::U_1F64E_1F3FE_200D_2640_FE0F };
    ("man_pouting_tone4") => { &twemoji_assets::png::codes::U_1F64E_1F3FE_200D_2642_FE0F };
    ("person_pouting_tone4") => { &twemoji_assets::png::codes::U_1F64E_1F3FE };
    ("pouting_tone4") => { &twemoji_assets::png::codes::U_1F64E_1F3FE };
    ("woman_pouting_tone5") => { &twemoji_assets::png::codes::U_1F64E_1F3FF_200D_2640_FE0F };
    ("man_pouting_tone5") => { &twemoji_assets::png::codes::U_1F64E_1F3FF_200D_2642_FE0F };
    ("person_pouting_tone5") => { &twemoji_assets::png::codes::U_1F64E_1F3FF };
    ("pouting_tone5") => { &twemoji_assets::png::codes::U_1F64E_1F3FF };
    ("woman_pouting") => { &twemoji_assets::png::codes::U_1F64E_200D_2640_FE0F };
    ("man_pouting") => { &twemoji_assets::png::codes::U_1F64E_200D_2642_FE0F };
    ("person_pouting") => { &twemoji_assets::png::codes::U_1F64E };
    ("pouting") => { &twemoji_assets::png::codes::U_1F64E };
    ("folded_hands_tone1") => { &twemoji_assets::png::codes::U_1F64F_1F3FB };
    ("pray_tone1") => { &twemoji_assets::png::codes::U_1F64F_1F3FB };
    ("folded_hands_tone2") => { &twemoji_assets::png::codes::U_1F64F_1F3FC };
    ("pray_tone2") => { &twemoji_assets::png::codes::U_1F64F_1F3FC };
    ("folded_hands_tone3") => { &twemoji_assets::png::codes::U_1F64F_1F3FD };
    ("pray_tone3") => { &twemoji_assets::png::codes::U_1F64F_1F3FD };
    ("folded_hands_tone4") => { &twemoji_assets::png::codes::U_1F64F_1F3FE };
    ("pray_tone4") => { &twemoji_assets::png::codes::U_1F64F_1F3FE };
    ("folded_hands_tone5") => { &twemoji_assets::png::codes::U_1F64F_1F3FF };
    ("pray_tone5") => { &twemoji_assets::png::codes::U_1F64F_1F3FF };
    ("folded_hands") => { &twemoji_assets::png::codes::U_1F64F };
    ("pray") => { &twemoji_assets::png::codes::U_1F64F };
    ("rocket") => { &twemoji_assets::png::codes::U_1F680 };
    ("helicopter") => { &twemoji_assets::png::codes::U_1F681 };
    ("steam_locomotive") => { &twemoji_assets::png::codes::U_1F682 };
    ("railway_car") => { &twemoji_assets::png::codes::U_1F683 };
    ("bullettrain_side") => { &twemoji_assets::png::codes::U_1F684 };
    ("bullettrain_front") => { &twemoji_assets::png::codes::U_1F685 };
    ("train") => { &twemoji_assets::png::codes::U_1F686 };
    ("metro") => { &twemoji_assets::png::codes::U_1F687 };
    ("light_rail") => { &twemoji_assets::png::codes::U_1F688 };
    ("station") => { &twemoji_assets::png::codes::U_1F689 };
    ("tram") => { &twemoji_assets::png::codes::U_1F68A };
    ("tram_car") => { &twemoji_assets::png::codes::U_1F68B };
    ("bus") => { &twemoji_assets::png::codes::U_1F68C };
    ("oncoming_bus") => { &twemoji_assets::png::codes::U_1F68D };
    ("trolleybus") => { &twemoji_assets::png::codes::U_1F68E };
    ("busstop") => { &twemoji_assets::png::codes::U_1F68F };
    ("minibus") => { &twemoji_assets::png::codes::U_1F690 };
    ("ambulance") => { &twemoji_assets::png::codes::U_1F691 };
    ("fire_engine") => { &twemoji_assets::png::codes::U_1F692 };
    ("police_car") => { &twemoji_assets::png::codes::U_1F693 };
    ("oncoming_police_car") => { &twemoji_assets::png::codes::U_1F694 };
    ("taxi") => { &twemoji_assets::png::codes::U_1F695 };
    ("oncoming_taxi") => { &twemoji_assets::png::codes::U_1F696 };
    ("car") => { &twemoji_assets::png::codes::U_1F697 };
    ("red_car") => { &twemoji_assets::png::codes::U_1F697 };
    ("oncoming_automobile") => { &twemoji_assets::png::codes::U_1F698 };
    ("blue_car") => { &twemoji_assets::png::codes::U_1F699 };
    ("suv") => { &twemoji_assets::png::codes::U_1F699 };
    ("delivery_truck") => { &twemoji_assets::png::codes::U_1F69A };
    ("truck") => { &twemoji_assets::png::codes::U_1F69A };
    ("articulated_lorry") => { &twemoji_assets::png::codes::U_1F69B };
    ("tractor") => { &twemoji_assets::png::codes::U_1F69C };
    ("monorail") => { &twemoji_assets::png::codes::U_1F69D };
    ("mountain_railway") => { &twemoji_assets::png::codes::U_1F69E };
    ("suspension_railway") => { &twemoji_assets::png::codes::U_1F69F };
    ("mountain_cableway") => { &twemoji_assets::png::codes::U_1F6A0 };
    ("aerial_tramway") => { &twemoji_assets::png::codes::U_1F6A1 };
    ("ship") => { &twemoji_assets::png::codes::U_1F6A2 };
    ("woman_rowing_boat_tone1") => { &twemoji_assets::png::codes::U_1F6A3_1F3FB_200D_2640_FE0F };
    ("man_rowing_boat_tone1") => { &twemoji_assets::png::codes::U_1F6A3_1F3FB_200D_2642_FE0F };
    ("person_rowing_boat_tone1") => { &twemoji_assets::png::codes::U_1F6A3_1F3FB };
    ("rowboat_tone1") => { &twemoji_assets::png::codes::U_1F6A3_1F3FB };
    ("woman_rowing_boat_tone2") => { &twemoji_assets::png::codes::U_1F6A3_1F3FC_200D_2640_FE0F };
    ("man_rowing_boat_tone2") => { &twemoji_assets::png::codes::U_1F6A3_1F3FC_200D_2642_FE0F };
    ("person_rowing_boat_tone2") => { &twemoji_assets::png::codes::U_1F6A3_1F3FC };
    ("rowboat_tone2") => { &twemoji_assets::png::codes::U_1F6A3_1F3FC };
    ("woman_rowing_boat_tone3") => { &twemoji_assets::png::codes::U_1F6A3_1F3FD_200D_2640_FE0F };
    ("man_rowing_boat_tone3") => { &twemoji_assets::png::codes::U_1F6A3_1F3FD_200D_2642_FE0F };
    ("person_rowing_boat_tone3") => { &twemoji_assets::png::codes::U_1F6A3_1F3FD };
    ("rowboat_tone3") => { &twemoji_assets::png::codes::U_1F6A3_1F3FD };
    ("woman_rowing_boat_tone4") => { &twemoji_assets::png::codes::U_1F6A3_1F3FE_200D_2640_FE0F };
    ("man_rowing_boat_tone4") => { &twemoji_assets::png::codes::U_1F6A3_1F3FE_200D_2642_FE0F };
    ("person_rowing_boat_tone4") => { &twemoji_assets::png::codes::U_1F6A3_1F3FE };
    ("rowboat_tone4") => { &twemoji_assets::png::codes::U_1F6A3_1F3FE };
    ("woman_rowing_boat_tone5") => { &twemoji_assets::png::codes::U_1F6A3_1F3FF_200D_2640_FE0F };
    ("man_rowing_boat_tone5") => { &twemoji_assets::png::codes::U_1F6A3_1F3FF_200D_2642_FE0F };
    ("person_rowing_boat_tone5") => { &twemoji_assets::png::codes::U_1F6A3_1F3FF };
    ("rowboat_tone5") => { &twemoji_assets::png::codes::U_1F6A3_1F3FF };
    ("woman_rowing_boat") => { &twemoji_assets::png::codes::U_1F6A3_200D_2640_FE0F };
    ("man_rowing_boat") => { &twemoji_assets::png::codes::U_1F6A3_200D_2642_FE0F };
    ("person_rowing_boat") => { &twemoji_assets::png::codes::U_1F6A3 };
    ("rowboat") => { &twemoji_assets::png::codes::U_1F6A3 };
    ("speedboat") => { &twemoji_assets::png::codes::U_1F6A4 };
    ("traffic_light") => { &twemoji_assets::png::codes::U_1F6A5 };
    ("vertical_traffic_light") => { &twemoji_assets::png::codes::U_1F6A6 };
    ("construction") => { &twemoji_assets::png::codes::U_1F6A7 };
    ("rotating_light") => { &twemoji_assets::png::codes::U_1F6A8 };
    ("triangular_flag") => { &twemoji_assets::png::codes::U_1F6A9 };
    ("triangular_flag_on_post") => { &twemoji_assets::png::codes::U_1F6A9 };
    ("door") => { &twemoji_assets::png::codes::U_1F6AA };
    ("no_entry_sign") => { &twemoji_assets::png::codes::U_1F6AB };
    ("cigarette") => { &twemoji_assets::png::codes::U_1F6AC };
    ("smoking") => { &twemoji_assets::png::codes::U_1F6AC };
    ("no_smoking") => { &twemoji_assets::png::codes::U_1F6AD };
    ("litter_bin") => { &twemoji_assets::png::codes::U_1F6AE };
    ("put_litter_in_its_place") => { &twemoji_assets::png::codes::U_1F6AE };
    ("do_not_litter") => { &twemoji_assets::png::codes::U_1F6AF };
    ("no_littering") => { &twemoji_assets::png::codes::U_1F6AF };
    ("potable_water") => { &twemoji_assets::png::codes::U_1F6B0 };
    ("non-potable_water") => { &twemoji_assets::png::codes::U_1F6B1 };
    ("bicycle") => { &twemoji_assets::png::codes::U_1F6B2 };
    ("bike") => { &twemoji_assets::png::codes::U_1F6B2 };
    ("no_bicycles") => { &twemoji_assets::png::codes::U_1F6B3 };
    ("woman_biking_tone1") => { &twemoji_assets::png::codes::U_1F6B4_1F3FB_200D_2640_FE0F };
    ("man_biking_tone1") => { &twemoji_assets::png::codes::U_1F6B4_1F3FB_200D_2642_FE0F };
    ("bicyclist_tone1") => { &twemoji_assets::png::codes::U_1F6B4_1F3FB };
    ("biking_tone1") => { &twemoji_assets::png::codes::U_1F6B4_1F3FB };
    ("person_biking_tone1") => { &twemoji_assets::png::codes::U_1F6B4_1F3FB };
    ("woman_biking_tone2") => { &twemoji_assets::png::codes::U_1F6B4_1F3FC_200D_2640_FE0F };
    ("man_biking_tone2") => { &twemoji_assets::png::codes::U_1F6B4_1F3FC_200D_2642_FE0F };
    ("bicyclist_tone2") => { &twemoji_assets::png::codes::U_1F6B4_1F3FC };
    ("biking_tone2") => { &twemoji_assets::png::codes::U_1F6B4_1F3FC };
    ("person_biking_tone2") => { &twemoji_assets::png::codes::U_1F6B4_1F3FC };
    ("woman_biking_tone3") => { &twemoji_assets::png::codes::U_1F6B4_1F3FD_200D_2640_FE0F };
    ("man_biking_tone3") => { &twemoji_assets::png::codes::U_1F6B4_1F3FD_200D_2642_FE0F };
    ("bicyclist_tone3") => { &twemoji_assets::png::codes::U_1F6B4_1F3FD };
    ("biking_tone3") => { &twemoji_assets::png::codes::U_1F6B4_1F3FD };
    ("person_biking_tone3") => { &twemoji_assets::png::codes::U_1F6B4_1F3FD };
    ("woman_biking_tone4") => { &twemoji_assets::png::codes::U_1F6B4_1F3FE_200D_2640_FE0F };
    ("man_biking_tone4") => { &twemoji_assets::png::codes::U_1F6B4_1F3FE_200D_2642_FE0F };
    ("bicyclist_tone4") => { &twemoji_assets::png::codes::U_1F6B4_1F3FE };
    ("biking_tone4") => { &twemoji_assets::png::codes::U_1F6B4_1F3FE };
    ("person_biking_tone4") => { &twemoji_assets::png::codes::U_1F6B4_1F3FE };
    ("woman_biking_tone5") => { &twemoji_assets::png::codes::U_1F6B4_1F3FF_200D_2640_FE0F };
    ("man_biking_tone5") => { &twemoji_assets::png::codes::U_1F6B4_1F3FF_200D_2642_FE0F };
    ("bicyclist_tone5") => { &twemoji_assets::png::codes::U_1F6B4_1F3FF };
    ("biking_tone5") => { &twemoji_assets::png::codes::U_1F6B4_1F3FF };
    ("person_biking_tone5") => { &twemoji_assets::png::codes::U_1F6B4_1F3FF };
    ("woman_biking") => { &twemoji_assets::png::codes::U_1F6B4_200D_2640_FE0F };
    ("man_biking") => { &twemoji_assets::png::codes::U_1F6B4_200D_2642_FE0F };
    ("bicyclist") => { &twemoji_assets::png::codes::U_1F6B4 };
    ("biking") => { &twemoji_assets::png::codes::U_1F6B4 };
    ("person_biking") => { &twemoji_assets::png::codes::U_1F6B4 };
    ("woman_mountain_biking_tone1") => { &twemoji_assets::png::codes::U_1F6B5_1F3FB_200D_2640_FE0F };
    ("man_mountain_biking_tone1") => { &twemoji_assets::png::codes::U_1F6B5_1F3FB_200D_2642_FE0F };
    ("mountain_bicyclist_tone1") => { &twemoji_assets::png::codes::U_1F6B5_1F3FB };
    ("mountain_biking_tone1") => { &twemoji_assets::png::codes::U_1F6B5_1F3FB };
    ("person_mountain_biking_tone1") => { &twemoji_assets::png::codes::U_1F6B5_1F3FB };
    ("woman_mountain_biking_tone2") => { &twemoji_assets::png::codes::U_1F6B5_1F3FC_200D_2640_FE0F };
    ("man_mountain_biking_tone2") => { &twemoji_assets::png::codes::U_1F6B5_1F3FC_200D_2642_FE0F };
    ("mountain_bicyclist_tone2") => { &twemoji_assets::png::codes::U_1F6B5_1F3FC };
    ("mountain_biking_tone2") => { &twemoji_assets::png::codes::U_1F6B5_1F3FC };
    ("person_mountain_biking_tone2") => { &twemoji_assets::png::codes::U_1F6B5_1F3FC };
    ("woman_mountain_biking_tone3") => { &twemoji_assets::png::codes::U_1F6B5_1F3FD_200D_2640_FE0F };
    ("man_mountain_biking_tone3") => { &twemoji_assets::png::codes::U_1F6B5_1F3FD_200D_2642_FE0F };
    ("mountain_bicyclist_tone3") => { &twemoji_assets::png::codes::U_1F6B5_1F3FD };
    ("mountain_biking_tone3") => { &twemoji_assets::png::codes::U_1F6B5_1F3FD };
    ("person_mountain_biking_tone3") => { &twemoji_assets::png::codes::U_1F6B5_1F3FD };
    ("woman_mountain_biking_tone4") => { &twemoji_assets::png::codes::U_1F6B5_1F3FE_200D_2640_FE0F };
    ("man_mountain_biking_tone4") => { &twemoji_assets::png::codes::U_1F6B5_1F3FE_200D_2642_FE0F };
    ("mountain_bicyclist_tone4") => { &twemoji_assets::png::codes::U_1F6B5_1F3FE };
    ("mountain_biking_tone4") => { &twemoji_assets::png::codes::U_1F6B5_1F3FE };
    ("person_mountain_biking_tone4") => { &twemoji_assets::png::codes::U_1F6B5_1F3FE };
    ("woman_mountain_biking_tone5") => { &twemoji_assets::png::codes::U_1F6B5_1F3FF_200D_2640_FE0F };
    ("man_mountain_biking_tone5") => { &twemoji_assets::png::codes::U_1F6B5_1F3FF_200D_2642_FE0F };
    ("mountain_bicyclist_tone5") => { &twemoji_assets::png::codes::U_1F6B5_1F3FF };
    ("mountain_biking_tone5") => { &twemoji_assets::png::codes::U_1F6B5_1F3FF };
    ("person_mountain_biking_tone5") => { &twemoji_assets::png::codes::U_1F6B5_1F3FF };
    ("woman_mountain_biking") => { &twemoji_assets::png::codes::U_1F6B5_200D_2640_FE0F };
    ("man_mountain_biking") => { &twemoji_assets::png::codes::U_1F6B5_200D_2642_FE0F };
    ("mountain_bicyclist") => { &twemoji_assets::png::codes::U_1F6B5 };
    ("mountain_biking") => { &twemoji_assets::png::codes::U_1F6B5 };
    ("person_mountain_biking") => { &twemoji_assets::png::codes::U_1F6B5 };
    ("woman_walking_tone1") => { &twemoji_assets::png::codes::U_1F6B6_1F3FB_200D_2640_FE0F };
    ("man_walking_tone1") => { &twemoji_assets::png::codes::U_1F6B6_1F3FB_200D_2642_FE0F };
    ("person_walking_tone1") => { &twemoji_assets::png::codes::U_1F6B6_1F3FB };
    ("walking_tone1") => { &twemoji_assets::png::codes::U_1F6B6_1F3FB };
    ("woman_walking_tone2") => { &twemoji_assets::png::codes::U_1F6B6_1F3FC_200D_2640_FE0F };
    ("man_walking_tone2") => { &twemoji_assets::png::codes::U_1F6B6_1F3FC_200D_2642_FE0F };
    ("person_walking_tone2") => { &twemoji_assets::png::codes::U_1F6B6_1F3FC };
    ("walking_tone2") => { &twemoji_assets::png::codes::U_1F6B6_1F3FC };
    ("woman_walking_tone3") => { &twemoji_assets::png::codes::U_1F6B6_1F3FD_200D_2640_FE0F };
    ("man_walking_tone3") => { &twemoji_assets::png::codes::U_1F6B6_1F3FD_200D_2642_FE0F };
    ("person_walking_tone3") => { &twemoji_assets::png::codes::U_1F6B6_1F3FD };
    ("walking_tone3") => { &twemoji_assets::png::codes::U_1F6B6_1F3FD };
    ("woman_walking_tone4") => { &twemoji_assets::png::codes::U_1F6B6_1F3FE_200D_2640_FE0F };
    ("man_walking_tone4") => { &twemoji_assets::png::codes::U_1F6B6_1F3FE_200D_2642_FE0F };
    ("person_walking_tone4") => { &twemoji_assets::png::codes::U_1F6B6_1F3FE };
    ("walking_tone4") => { &twemoji_assets::png::codes::U_1F6B6_1F3FE };
    ("woman_walking_tone5") => { &twemoji_assets::png::codes::U_1F6B6_1F3FF_200D_2640_FE0F };
    ("man_walking_tone5") => { &twemoji_assets::png::codes::U_1F6B6_1F3FF_200D_2642_FE0F };
    ("person_walking_tone5") => { &twemoji_assets::png::codes::U_1F6B6_1F3FF };
    ("walking_tone5") => { &twemoji_assets::png::codes::U_1F6B6_1F3FF };
    ("woman_walking") => { &twemoji_assets::png::codes::U_1F6B6_200D_2640_FE0F };
    ("man_walking") => { &twemoji_assets::png::codes::U_1F6B6_200D_2642_FE0F };
    ("person_walking") => { &twemoji_assets::png::codes::U_1F6B6 };
    ("walking") => { &twemoji_assets::png::codes::U_1F6B6 };
    ("no_pedestrians") => { &twemoji_assets::png::codes::U_1F6B7 };
    ("children_crossing") => { &twemoji_assets::png::codes::U_1F6B8 };
    ("mens") => { &twemoji_assets::png::codes::U_1F6B9 };
    ("womens") => { &twemoji_assets::png::codes::U_1F6BA };
    ("bathroom") => { &twemoji_assets::png::codes::U_1F6BB };
    ("restroom") => { &twemoji_assets::png::codes::U_1F6BB };
    ("baby_symbol") => { &twemoji_assets::png::codes::U_1F6BC };
    ("toilet") => { &twemoji_assets::png::codes::U_1F6BD };
    ("water_closet") => { &twemoji_assets::png::codes::U_1F6BE };
    ("wc") => { &twemoji_assets::png::codes::U_1F6BE };
    ("shower") => { &twemoji_assets::png::codes::U_1F6BF };
    ("bath_tone1") => { &twemoji_assets::png::codes::U_1F6C0_1F3FB };
    ("person_taking_bath_tone1") => { &twemoji_assets::png::codes::U_1F6C0_1F3FB };
    ("bath_tone2") => { &twemoji_assets::png::codes::U_1F6C0_1F3FC };
    ("person_taking_bath_tone2") => { &twemoji_assets::png::codes::U_1F6C0_1F3FC };
    ("bath_tone3") => { &twemoji_assets::png::codes::U_1F6C0_1F3FD };
    ("person_taking_bath_tone3") => { &twemoji_assets::png::codes::U_1F6C0_1F3FD };
    ("bath_tone4") => { &twemoji_assets::png::codes::U_1F6C0_1F3FE };
    ("person_taking_bath_tone4") => { &twemoji_assets::png::codes::U_1F6C0_1F3FE };
    ("bath_tone5") => { &twemoji_assets::png::codes::U_1F6C0_1F3FF };
    ("person_taking_bath_tone5") => { &twemoji_assets::png::codes::U_1F6C0_1F3FF };
    ("bath") => { &twemoji_assets::png::codes::U_1F6C0 };
    ("person_taking_bath") => { &twemoji_assets::png::codes::U_1F6C0 };
    ("bathtub") => { &twemoji_assets::png::codes::U_1F6C1 };
    ("passport_control") => { &twemoji_assets::png::codes::U_1F6C2 };
    ("customs") => { &twemoji_assets::png::codes::U_1F6C3 };
    ("baggage_claim") => { &twemoji_assets::png::codes::U_1F6C4 };
    ("left_luggage") => { &twemoji_assets::png::codes::U_1F6C5 };
    ("couch_and_lamp") => { &twemoji_assets::png::codes::U_1F6CB };
    ("person_in_bed_tone1") => { &twemoji_assets::png::codes::U_1F6CC_1F3FB };
    ("sleeping_accommodation_tone1") => { &twemoji_assets::png::codes::U_1F6CC_1F3FB };
    ("person_in_bed_tone2") => { &twemoji_assets::png::codes::U_1F6CC_1F3FC };
    ("sleeping_accommodation_tone2") => { &twemoji_assets::png::codes::U_1F6CC_1F3FC };
    ("person_in_bed_tone3") => { &twemoji_assets::png::codes::U_1F6CC_1F3FD };
    ("sleeping_accommodation_tone3") => { &twemoji_assets::png::codes::U_1F6CC_1F3FD };
    ("person_in_bed_tone4") => { &twemoji_assets::png::codes::U_1F6CC_1F3FE };
    ("sleeping_accommodation_tone4") => { &twemoji_assets::png::codes::U_1F6CC_1F3FE };
    ("person_in_bed_tone5") => { &twemoji_assets::png::codes::U_1F6CC_1F3FF };
    ("sleeping_accommodation_tone5") => { &twemoji_assets::png::codes::U_1F6CC_1F3FF };
    ("person_in_bed") => { &twemoji_assets::png::codes::U_1F6CC };
    ("sleeping_accommodation") => { &twemoji_assets::png::codes::U_1F6CC };
    ("shopping_bags") => { &twemoji_assets::png::codes::U_1F6CD };
    ("bellhop") => { &twemoji_assets::png::codes::U_1F6CE };
    ("bed") => { &twemoji_assets::png::codes::U_1F6CF };
    ("place_of_worship") => { &twemoji_assets::png::codes::U_1F6D0 };
    ("octagonal_sign") => { &twemoji_assets::png::codes::U_1F6D1 };
    ("stop_sign") => { &twemoji_assets::png::codes::U_1F6D1 };
    ("shopping_cart") => { &twemoji_assets::png::codes::U_1F6D2 };
    ("hindu_temple") => { &twemoji_assets::png::codes::U_1F6D5 };
    ("hut") => { &twemoji_assets::png::codes::U_1F6D6 };
    ("elevator") => { &twemoji_assets::png::codes::U_1F6D7 };
    ("playground_slide") => { &twemoji_assets::png::codes::U_1F6DD };
    ("slide") => { &twemoji_assets::png::codes::U_1F6DD };
    ("wheel") => { &twemoji_assets::png::codes::U_1F6DE };
    ("lifebuoy") => { &twemoji_assets::png::codes::U_1F6DF };
    ("ring_buoy") => { &twemoji_assets::png::codes::U_1F6DF };
    ("hammer_and_wrench") => { &twemoji_assets::png::codes::U_1F6E0 };
    ("shield") => { &twemoji_assets::png::codes::U_1F6E1 };
    ("oil_drum") => { &twemoji_assets::png::codes::U_1F6E2 };
    ("motorway") => { &twemoji_assets::png::codes::U_1F6E3 };
    ("railway_track") => { &twemoji_assets::png::codes::U_1F6E4 };
    ("motorboat") => { &twemoji_assets::png::codes::U_1F6E5 };
    ("small_airplane") => { &twemoji_assets::png::codes::U_1F6E9 };
    ("airplane_departure") => { &twemoji_assets::png::codes::U_1F6EB };
    ("airplane_arriving") => { &twemoji_assets::png::codes::U_1F6EC };
    ("satellite") => { &twemoji_assets::png::codes::U_1F6F0 };
    ("cruise_ship") => { &twemoji_assets::png::codes::U_1F6F3 };
    ("passenger_ship") => { &twemoji_assets::png::codes::U_1F6F3 };
    ("scooter") => { &twemoji_assets::png::codes::U_1F6F4 };
    ("motor_scooter") => { &twemoji_assets::png::codes::U_1F6F5 };
    ("canoe") => { &twemoji_assets::png::codes::U_1F6F6 };
    ("sled") => { &twemoji_assets::png::codes::U_1F6F7 };
    ("flying_saucer") => { &twemoji_assets::png::codes::U_1F6F8 };
    ("skateboard") => { &twemoji_assets::png::codes::U_1F6F9 };
    ("auto_rickshaw") => { &twemoji_assets::png::codes::U_1F6FA };
    ("pickup_truck") => { &twemoji_assets::png::codes::U_1F6FB };
    ("roller_skate") => { &twemoji_assets::png::codes::U_1F6FC };
    ("orange_circle") => { &twemoji_assets::png::codes::U_1F7E0 };
    ("yellow_circle") => { &twemoji_assets::png::codes::U_1F7E1 };
    ("green_circle") => { &twemoji_assets::png::codes::U_1F7E2 };
    ("purple_circle") => { &twemoji_assets::png::codes::U_1F7E3 };
    ("brown_circle") => { &twemoji_assets::png::codes::U_1F7E4 };
    ("red_square") => { &twemoji_assets::png::codes::U_1F7E5 };
    ("blue_square") => { &twemoji_assets::png::codes::U_1F7E6 };
    ("orange_square") => { &twemoji_assets::png::codes::U_1F7E7 };
    ("yellow_square") => { &twemoji_assets::png::codes::U_1F7E8 };
    ("green_square") => { &twemoji_assets::png::codes::U_1F7E9 };
    ("purple_square") => { &twemoji_assets::png::codes::U_1F7EA };
    ("brown_square") => { &twemoji_assets::png::codes::U_1F7EB };
    ("heavy_equals_sign") => { &twemoji_assets::png::codes::U_1F7F0 };
    ("pinch_tone1") => { &twemoji_assets::png::codes::U_1F90C_1F3FB };
    ("pinched_fingers_tone1") => { &twemoji_assets::png::codes::U_1F90C_1F3FB };
    ("pinch_tone2") => { &twemoji_assets::png::codes::U_1F90C_1F3FC };
    ("pinched_fingers_tone2") => { &twemoji_assets::png::codes::U_1F90C_1F3FC };
    ("pinch_tone3") => { &twemoji_assets::png::codes::U_1F90C_1F3FD };
    ("pinched_fingers_tone3") => { &twemoji_assets::png::codes::U_1F90C_1F3FD };
    ("pinch_tone4") => { &twemoji_assets::png::codes::U_1F90C_1F3FE };
    ("pinched_fingers_tone4") => { &twemoji_assets::png::codes::U_1F90C_1F3FE };
    ("pinch_tone5") => { &twemoji_assets::png::codes::U_1F90C_1F3FF };
    ("pinched_fingers_tone5") => { &twemoji_assets::png::codes::U_1F90C_1F3FF };
    ("pinch") => { &twemoji_assets::png::codes::U_1F90C };
    ("pinched_fingers") => { &twemoji_assets::png::codes::U_1F90C };
    ("white_heart") => { &twemoji_assets::png::codes::U_1F90D };
    ("brown_heart") => { &twemoji_assets::png::codes::U_1F90E };
    ("pinching_hand_tone1") => { &twemoji_assets::png::codes::U_1F90F_1F3FB };
    ("pinching_hand_tone2") => { &twemoji_assets::png::codes::U_1F90F_1F3FC };
    ("pinching_hand_tone3") => { &twemoji_assets::png::codes::U_1F90F_1F3FD };
    ("pinching_hand_tone4") => { &twemoji_assets::png::codes::U_1F90F_1F3FE };
    ("pinching_hand_tone5") => { &twemoji_assets::png::codes::U_1F90F_1F3FF };
    ("pinching_hand") => { &twemoji_assets::png::codes::U_1F90F };
    ("zipper_mouth") => { &twemoji_assets::png::codes::U_1F910 };
    ("zipper_mouth_face") => { &twemoji_assets::png::codes::U_1F910 };
    ("money_mouth_face") => { &twemoji_assets::png::codes::U_1F911 };
    ("face_with_thermometer") => { &twemoji_assets::png::codes::U_1F912 };
    ("nerd") => { &twemoji_assets::png::codes::U_1F913 };
    ("nerd_face") => { &twemoji_assets::png::codes::U_1F913 };
    ("thinking") => { &twemoji_assets::png::codes::U_1F914 };
    ("thinking_face") => { &twemoji_assets::png::codes::U_1F914 };
    ("wtf") => { &twemoji_assets::png::codes::U_1F914 };
    ("face_with_head_bandage") => { &twemoji_assets::png::codes::U_1F915 };
    ("robot") => { &twemoji_assets::png::codes::U_1F916 };
    ("robot_face") => { &twemoji_assets::png::codes::U_1F916 };
    ("hug") => { &twemoji_assets::png::codes::U_1F917 };
    ("hugging") => { &twemoji_assets::png::codes::U_1F917 };
    ("hugging_face") => { &twemoji_assets::png::codes::U_1F917 };
    ("metal_tone1") => { &twemoji_assets::png::codes::U_1F918_1F3FB };
    ("sign_of_the_horns_tone1") => { &twemoji_assets::png::codes::U_1F918_1F3FB };
    ("metal_tone2") => { &twemoji_assets::png::codes::U_1F918_1F3FC };
    ("sign_of_the_horns_tone2") => { &twemoji_assets::png::codes::U_1F918_1F3FC };
    ("metal_tone3") => { &twemoji_assets::png::codes::U_1F918_1F3FD };
    ("sign_of_the_horns_tone3") => { &twemoji_assets::png::codes::U_1F918_1F3FD };
    ("metal_tone4") => { &twemoji_assets::png::codes::U_1F918_1F3FE };
    ("sign_of_the_horns_tone4") => { &twemoji_assets::png::codes::U_1F918_1F3FE };
    ("metal_tone5") => { &twemoji_assets::png::codes::U_1F918_1F3FF };
    ("sign_of_the_horns_tone5") => { &twemoji_assets::png::codes::U_1F918_1F3FF };
    ("metal") => { &twemoji_assets::png::codes::U_1F918 };
    ("sign_of_the_horns") => { &twemoji_assets::png::codes::U_1F918 };
    ("call_me_hand_tone1") => { &twemoji_assets::png::codes::U_1F919_1F3FB };
    ("call_me_hand_tone2") => { &twemoji_assets::png::codes::U_1F919_1F3FC };
    ("call_me_hand_tone3") => { &twemoji_assets::png::codes::U_1F919_1F3FD };
    ("call_me_hand_tone4") => { &twemoji_assets::png::codes::U_1F919_1F3FE };
    ("call_me_hand_tone5") => { &twemoji_assets::png::codes::U_1F919_1F3FF };
    ("call_me_hand") => { &twemoji_assets::png::codes::U_1F919 };
    ("raised_back_of_hand_tone1") => { &twemoji_assets::png::codes::U_1F91A_1F3FB };
    ("raised_back_of_hand_tone2") => { &twemoji_assets::png::codes::U_1F91A_1F3FC };
    ("raised_back_of_hand_tone3") => { &twemoji_assets::png::codes::U_1F91A_1F3FD };
    ("raised_back_of_hand_tone4") => { &twemoji_assets::png::codes::U_1F91A_1F3FE };
    ("raised_back_of_hand_tone5") => { &twemoji_assets::png::codes::U_1F91A_1F3FF };
    ("raised_back_of_hand") => { &twemoji_assets::png::codes::U_1F91A };
    ("left_facing_fist_tone1") => { &twemoji_assets::png::codes::U_1F91B_1F3FB };
    ("left_facing_fist_tone2") => { &twemoji_assets::png::codes::U_1F91B_1F3FC };
    ("left_facing_fist_tone3") => { &twemoji_assets::png::codes::U_1F91B_1F3FD };
    ("left_facing_fist_tone4") => { &twemoji_assets::png::codes::U_1F91B_1F3FE };
    ("left_facing_fist_tone5") => { &twemoji_assets::png::codes::U_1F91B_1F3FF };
    ("left_facing_fist") => { &twemoji_assets::png::codes::U_1F91B };
    ("right_facing_fist_tone1") => { &twemoji_assets::png::codes::U_1F91C_1F3FB };
    ("right_facing_fist_tone2") => { &twemoji_assets::png::codes::U_1F91C_1F3FC };
    ("right_facing_fist_tone3") => { &twemoji_assets::png::codes::U_1F91C_1F3FD };
    ("right_facing_fist_tone4") => { &twemoji_assets::png::codes::U_1F91C_1F3FE };
    ("right_facing_fist_tone5") => { &twemoji_assets::png::codes::U_1F91C_1F3FF };
    ("right_facing_fist") => { &twemoji_assets::png::codes::U_1F91C };
    ("handshake_tone1") => { &twemoji_assets::png::codes::U_1F91D_1F3FB };
    ("handshake_tone2") => { &twemoji_assets::png::codes::U_1F91D_1F3FC };
    ("handshake_tone3") => { &twemoji_assets::png::codes::U_1F91D_1F3FD };
    ("handshake_tone4") => { &twemoji_assets::png::codes::U_1F91D_1F3FE };
    ("handshake_tone5") => { &twemoji_assets::png::codes::U_1F91D_1F3FF };
    ("handshake") => { &twemoji_assets::png::codes::U_1F91D };
    ("fingers_crossed_tone1") => { &twemoji_assets::png::codes::U_1F91E_1F3FB };
    ("fingers_crossed_tone2") => { &twemoji_assets::png::codes::U_1F91E_1F3FC };
    ("fingers_crossed_tone3") => { &twemoji_assets::png::codes::U_1F91E_1F3FD };
    ("fingers_crossed_tone4") => { &twemoji_assets::png::codes::U_1F91E_1F3FE };
    ("fingers_crossed_tone5") => { &twemoji_assets::png::codes::U_1F91E_1F3FF };
    ("fingers_crossed") => { &twemoji_assets::png::codes::U_1F91E };
    ("love_you_gesture_tone1") => { &twemoji_assets::png::codes::U_1F91F_1F3FB };
    ("love_you_gesture_tone2") => { &twemoji_assets::png::codes::U_1F91F_1F3FC };
    ("love_you_gesture_tone3") => { &twemoji_assets::png::codes::U_1F91F_1F3FD };
    ("love_you_gesture_tone4") => { &twemoji_assets::png::codes::U_1F91F_1F3FE };
    ("love_you_gesture_tone5") => { &twemoji_assets::png::codes::U_1F91F_1F3FF };
    ("love_you_gesture") => { &twemoji_assets::png::codes::U_1F91F };
    ("cowboy") => { &twemoji_assets::png::codes::U_1F920 };
    ("cowboy_face") => { &twemoji_assets::png::codes::U_1F920 };
    ("clown") => { &twemoji_assets::png::codes::U_1F921 };
    ("clown_face") => { &twemoji_assets::png::codes::U_1F921 };
    ("nauseated") => { &twemoji_assets::png::codes::U_1F922 };
    ("nauseated_face") => { &twemoji_assets::png::codes::U_1F922 };
    ("rofl") => { &twemoji_assets::png::codes::U_1F923 };
    ("drooling") => { &twemoji_assets::png::codes::U_1F924 };
    ("drooling_face") => { &twemoji_assets::png::codes::U_1F924 };
    ("lying") => { &twemoji_assets::png::codes::U_1F925 };
    ("lying_face") => { &twemoji_assets::png::codes::U_1F925 };
    ("woman_facepalming_tone1") => { &twemoji_assets::png::codes::U_1F926_1F3FB_200D_2640_FE0F };
    ("man_facepalming_tone1") => { &twemoji_assets::png::codes::U_1F926_1F3FB_200D_2642_FE0F };
    ("facepalm_tone1") => { &twemoji_assets::png::codes::U_1F926_1F3FB };
    ("person_facepalming_tone1") => { &twemoji_assets::png::codes::U_1F926_1F3FB };
    ("woman_facepalming_tone2") => { &twemoji_assets::png::codes::U_1F926_1F3FC_200D_2640_FE0F };
    ("man_facepalming_tone2") => { &twemoji_assets::png::codes::U_1F926_1F3FC_200D_2642_FE0F };
    ("facepalm_tone2") => { &twemoji_assets::png::codes::U_1F926_1F3FC };
    ("person_facepalming_tone2") => { &twemoji_assets::png::codes::U_1F926_1F3FC };
    ("woman_facepalming_tone3") => { &twemoji_assets::png::codes::U_1F926_1F3FD_200D_2640_FE0F };
    ("man_facepalming_tone3") => { &twemoji_assets::png::codes::U_1F926_1F3FD_200D_2642_FE0F };
    ("facepalm_tone3") => { &twemoji_assets::png::codes::U_1F926_1F3FD };
    ("person_facepalming_tone3") => { &twemoji_assets::png::codes::U_1F926_1F3FD };
    ("woman_facepalming_tone4") => { &twemoji_assets::png::codes::U_1F926_1F3FE_200D_2640_FE0F };
    ("man_facepalming_tone4") => { &twemoji_assets::png::codes::U_1F926_1F3FE_200D_2642_FE0F };
    ("facepalm_tone4") => { &twemoji_assets::png::codes::U_1F926_1F3FE };
    ("person_facepalming_tone4") => { &twemoji_assets::png::codes::U_1F926_1F3FE };
    ("woman_facepalming_tone5") => { &twemoji_assets::png::codes::U_1F926_1F3FF_200D_2640_FE0F };
    ("man_facepalming_tone5") => { &twemoji_assets::png::codes::U_1F926_1F3FF_200D_2642_FE0F };
    ("facepalm_tone5") => { &twemoji_assets::png::codes::U_1F926_1F3FF };
    ("person_facepalming_tone5") => { &twemoji_assets::png::codes::U_1F926_1F3FF };
    ("woman_facepalming") => { &twemoji_assets::png::codes::U_1F926_200D_2640_FE0F };
    ("man_facepalming") => { &twemoji_assets::png::codes::U_1F926_200D_2642_FE0F };
    ("facepalm") => { &twemoji_assets::png::codes::U_1F926 };
    ("person_facepalming") => { &twemoji_assets::png::codes::U_1F926 };
    ("sneezing") => { &twemoji_assets::png::codes::U_1F927 };
    ("sneezing_face") => { &twemoji_assets::png::codes::U_1F927 };
    ("face_with_raised_eyebrow") => { &twemoji_assets::png::codes::U_1F928 };
    ("raised_eyebrow") => { &twemoji_assets::png::codes::U_1F928 };
    ("star_struck") => { &twemoji_assets::png::codes::U_1F929 };
    ("zany") => { &twemoji_assets::png::codes::U_1F92A };
    ("zany_face") => { &twemoji_assets::png::codes::U_1F92A };
    ("shush") => { &twemoji_assets::png::codes::U_1F92B };
    ("shushing_face") => { &twemoji_assets::png::codes::U_1F92B };
    ("censored") => { &twemoji_assets::png::codes::U_1F92C };
    ("face_with_symbols_on_mouth") => { &twemoji_assets::png::codes::U_1F92C };
    ("face_with_hand_over_mouth") => { &twemoji_assets::png::codes::U_1F92D };
    ("hand_over_mouth") => { &twemoji_assets::png::codes::U_1F92D };
    ("face_vomiting") => { &twemoji_assets::png::codes::U_1F92E };
    ("vomiting") => { &twemoji_assets::png::codes::U_1F92E };
    ("exploding_head") => { &twemoji_assets::png::codes::U_1F92F };
    ("pregnant_woman_tone1") => { &twemoji_assets::png::codes::U_1F930_1F3FB };
    ("pregnant_woman_tone2") => { &twemoji_assets::png::codes::U_1F930_1F3FC };
    ("pregnant_woman_tone3") => { &twemoji_assets::png::codes::U_1F930_1F3FD };
    ("pregnant_woman_tone4") => { &twemoji_assets::png::codes::U_1F930_1F3FE };
    ("pregnant_woman_tone5") => { &twemoji_assets::png::codes::U_1F930_1F3FF };
    ("pregnant_woman") => { &twemoji_assets::png::codes::U_1F930 };
    ("breast_feeding_tone1") => { &twemoji_assets::png::codes::U_1F931_1F3FB };
    ("breast_feeding_tone2") => { &twemoji_assets::png::codes::U_1F931_1F3FC };
    ("breast_feeding_tone3") => { &twemoji_assets::png::codes::U_1F931_1F3FD };
    ("breast_feeding_tone4") => { &twemoji_assets::png::codes::U_1F931_1F3FE };
    ("breast_feeding_tone5") => { &twemoji_assets::png::codes::U_1F931_1F3FF };
    ("breast_feeding") => { &twemoji_assets::png::codes::U_1F931 };
    ("palms_up_together_tone1") => { &twemoji_assets::png::codes::U_1F932_1F3FB };
    ("palms_up_together_tone2") => { &twemoji_assets::png::codes::U_1F932_1F3FC };
    ("palms_up_together_tone3") => { &twemoji_assets::png::codes::U_1F932_1F3FD };
    ("palms_up_together_tone4") => { &twemoji_assets::png::codes::U_1F932_1F3FE };
    ("palms_up_together_tone5") => { &twemoji_assets::png::codes::U_1F932_1F3FF };
    ("palms_up_together") => { &twemoji_assets::png::codes::U_1F932 };
    ("selfie_tone1") => { &twemoji_assets::png::codes::U_1F933_1F3FB };
    ("selfie_tone2") => { &twemoji_assets::png::codes::U_1F933_1F3FC };
    ("selfie_tone3") => { &twemoji_assets::png::codes::U_1F933_1F3FD };
    ("selfie_tone4") => { &twemoji_assets::png::codes::U_1F933_1F3FE };
    ("selfie_tone5") => { &twemoji_assets::png::codes::U_1F933_1F3FF };
    ("selfie") => { &twemoji_assets::png::codes::U_1F933 };
    ("prince_tone1") => { &twemoji_assets::png::codes::U_1F934_1F3FB };
    ("prince_tone2") => { &twemoji_assets::png::codes::U_1F934_1F3FC };
    ("prince_tone3") => { &twemoji_assets::png::codes::U_1F934_1F3FD };
    ("prince_tone4") => { &twemoji_assets::png::codes::U_1F934_1F3FE };
    ("prince_tone5") => { &twemoji_assets::png::codes::U_1F934_1F3FF };
    ("prince") => { &twemoji_assets::png::codes::U_1F934 };
    ("woman_in_tuxedo_tone1") => { &twemoji_assets::png::codes::U_1F935_1F3FB_200D_2640_FE0F };
    ("man_in_tuxedo_tone1") => { &twemoji_assets::png::codes::U_1F935_1F3FB_200D_2642_FE0F };
    ("person_in_tuxedo_tone1") => { &twemoji_assets::png::codes::U_1F935_1F3FB };
    ("woman_in_tuxedo_tone2") => { &twemoji_assets::png::codes::U_1F935_1F3FC_200D_2640_FE0F };
    ("man_in_tuxedo_tone2") => { &twemoji_assets::png::codes::U_1F935_1F3FC_200D_2642_FE0F };
    ("person_in_tuxedo_tone2") => { &twemoji_assets::png::codes::U_1F935_1F3FC };
    ("woman_in_tuxedo_tone3") => { &twemoji_assets::png::codes::U_1F935_1F3FD_200D_2640_FE0F };
    ("man_in_tuxedo_tone3") => { &twemoji_assets::png::codes::U_1F935_1F3FD_200D_2642_FE0F };
    ("person_in_tuxedo_tone3") => { &twemoji_assets::png::codes::U_1F935_1F3FD };
    ("woman_in_tuxedo_tone4") => { &twemoji_assets::png::codes::U_1F935_1F3FE_200D_2640_FE0F };
    ("man_in_tuxedo_tone4") => { &twemoji_assets::png::codes::U_1F935_1F3FE_200D_2642_FE0F };
    ("person_in_tuxedo_tone4") => { &twemoji_assets::png::codes::U_1F935_1F3FE };
    ("woman_in_tuxedo_tone5") => { &twemoji_assets::png::codes::U_1F935_1F3FF_200D_2640_FE0F };
    ("man_in_tuxedo_tone5") => { &twemoji_assets::png::codes::U_1F935_1F3FF_200D_2642_FE0F };
    ("person_in_tuxedo_tone5") => { &twemoji_assets::png::codes::U_1F935_1F3FF };
    ("woman_in_tuxedo") => { &twemoji_assets::png::codes::U_1F935_200D_2640_FE0F };
    ("man_in_tuxedo") => { &twemoji_assets::png::codes::U_1F935_200D_2642_FE0F };
    ("person_in_tuxedo") => { &twemoji_assets::png::codes::U_1F935 };
    ("mrs_claus_tone1") => { &twemoji_assets::png::codes::U_1F936_1F3FB };
    ("mrs_claus_tone2") => { &twemoji_assets::png::codes::U_1F936_1F3FC };
    ("mrs_claus_tone3") => { &twemoji_assets::png::codes::U_1F936_1F3FD };
    ("mrs_claus_tone4") => { &twemoji_assets::png::codes::U_1F936_1F3FE };
    ("mrs_claus_tone5") => { &twemoji_assets::png::codes::U_1F936_1F3FF };
    ("mrs_claus") => { &twemoji_assets::png::codes::U_1F936 };
    ("woman_shrugging_tone1") => { &twemoji_assets::png::codes::U_1F937_1F3FB_200D_2640_FE0F };
    ("man_shrugging_tone1") => { &twemoji_assets::png::codes::U_1F937_1F3FB_200D_2642_FE0F };
    ("person_shrugging_tone1") => { &twemoji_assets::png::codes::U_1F937_1F3FB };
    ("shrug_tone1") => { &twemoji_assets::png::codes::U_1F937_1F3FB };
    ("woman_shrugging_tone2") => { &twemoji_assets::png::codes::U_1F937_1F3FC_200D_2640_FE0F };
    ("man_shrugging_tone2") => { &twemoji_assets::png::codes::U_1F937_1F3FC_200D_2642_FE0F };
    ("person_shrugging_tone2") => { &twemoji_assets::png::codes::U_1F937_1F3FC };
    ("shrug_tone2") => { &twemoji_assets::png::codes::U_1F937_1F3FC };
    ("woman_shrugging_tone3") => { &twemoji_assets::png::codes::U_1F937_1F3FD_200D_2640_FE0F };
    ("man_shrugging_tone3") => { &twemoji_assets::png::codes::U_1F937_1F3FD_200D_2642_FE0F };
    ("person_shrugging_tone3") => { &twemoji_assets::png::codes::U_1F937_1F3FD };
    ("shrug_tone3") => { &twemoji_assets::png::codes::U_1F937_1F3FD };
    ("woman_shrugging_tone4") => { &twemoji_assets::png::codes::U_1F937_1F3FE_200D_2640_FE0F };
    ("man_shrugging_tone4") => { &twemoji_assets::png::codes::U_1F937_1F3FE_200D_2642_FE0F };
    ("person_shrugging_tone4") => { &twemoji_assets::png::codes::U_1F937_1F3FE };
    ("shrug_tone4") => { &twemoji_assets::png::codes::U_1F937_1F3FE };
    ("woman_shrugging_tone5") => { &twemoji_assets::png::codes::U_1F937_1F3FF_200D_2640_FE0F };
    ("man_shrugging_tone5") => { &twemoji_assets::png::codes::U_1F937_1F3FF_200D_2642_FE0F };
    ("person_shrugging_tone5") => { &twemoji_assets::png::codes::U_1F937_1F3FF };
    ("shrug_tone5") => { &twemoji_assets::png::codes::U_1F937_1F3FF };
    ("woman_shrugging") => { &twemoji_assets::png::codes::U_1F937_200D_2640_FE0F };
    ("man_shrugging") => { &twemoji_assets::png::codes::U_1F937_200D_2642_FE0F };
    ("person_shrugging") => { &twemoji_assets::png::codes::U_1F937 };
    ("shrug") => { &twemoji_assets::png::codes::U_1F937 };
    ("woman_cartwheeling_tone1") => { &twemoji_assets::png::codes::U_1F938_1F3FB_200D_2640_FE0F };
    ("man_cartwheeling_tone1") => { &twemoji_assets::png::codes::U_1F938_1F3FB_200D_2642_FE0F };
    ("cartwheeling_tone1") => { &twemoji_assets::png::codes::U_1F938_1F3FB };
    ("person_cartwheel_tone1") => { &twemoji_assets::png::codes::U_1F938_1F3FB };
    ("woman_cartwheeling_tone2") => { &twemoji_assets::png::codes::U_1F938_1F3FC_200D_2640_FE0F };
    ("man_cartwheeling_tone2") => { &twemoji_assets::png::codes::U_1F938_1F3FC_200D_2642_FE0F };
    ("cartwheeling_tone2") => { &twemoji_assets::png::codes::U_1F938_1F3FC };
    ("person_cartwheel_tone2") => { &twemoji_assets::png::codes::U_1F938_1F3FC };
    ("woman_cartwheeling_tone3") => { &twemoji_assets::png::codes::U_1F938_1F3FD_200D_2640_FE0F };
    ("man_cartwheeling_tone3") => { &twemoji_assets::png::codes::U_1F938_1F3FD_200D_2642_FE0F };
    ("cartwheeling_tone3") => { &twemoji_assets::png::codes::U_1F938_1F3FD };
    ("person_cartwheel_tone3") => { &twemoji_assets::png::codes::U_1F938_1F3FD };
    ("woman_cartwheeling_tone4") => { &twemoji_assets::png::codes::U_1F938_1F3FE_200D_2640_FE0F };
    ("man_cartwheeling_tone4") => { &twemoji_assets::png::codes::U_1F938_1F3FE_200D_2642_FE0F };
    ("cartwheeling_tone4") => { &twemoji_assets::png::codes::U_1F938_1F3FE };
    ("person_cartwheel_tone4") => { &twemoji_assets::png::codes::U_1F938_1F3FE };
    ("woman_cartwheeling_tone5") => { &twemoji_assets::png::codes::U_1F938_1F3FF_200D_2640_FE0F };
    ("man_cartwheeling_tone5") => { &twemoji_assets::png::codes::U_1F938_1F3FF_200D_2642_FE0F };
    ("cartwheeling_tone5") => { &twemoji_assets::png::codes::U_1F938_1F3FF };
    ("person_cartwheel_tone5") => { &twemoji_assets::png::codes::U_1F938_1F3FF };
    ("woman_cartwheeling") => { &twemoji_assets::png::codes::U_1F938_200D_2640_FE0F };
    ("man_cartwheeling") => { &twemoji_assets::png::codes::U_1F938_200D_2642_FE0F };
    ("cartwheeling") => { &twemoji_assets::png::codes::U_1F938 };
    ("person_cartwheel") => { &twemoji_assets::png::codes::U_1F938 };
    ("woman_juggling_tone1") => { &twemoji_assets::png::codes::U_1F939_1F3FB_200D_2640_FE0F };
    ("man_juggling_tone1") => { &twemoji_assets::png::codes::U_1F939_1F3FB_200D_2642_FE0F };
    ("juggler_tone1") => { &twemoji_assets::png::codes::U_1F939_1F3FB };
    ("juggling_tone1") => { &twemoji_assets::png::codes::U_1F939_1F3FB };
    ("person_juggling_tone1") => { &twemoji_assets::png::codes::U_1F939_1F3FB };
    ("woman_juggling_tone2") => { &twemoji_assets::png::codes::U_1F939_1F3FC_200D_2640_FE0F };
    ("man_juggling_tone2") => { &twemoji_assets::png::codes::U_1F939_1F3FC_200D_2642_FE0F };
    ("juggler_tone2") => { &twemoji_assets::png::codes::U_1F939_1F3FC };
    ("juggling_tone2") => { &twemoji_assets::png::codes::U_1F939_1F3FC };
    ("person_juggling_tone2") => { &twemoji_assets::png::codes::U_1F939_1F3FC };
    ("woman_juggling_tone3") => { &twemoji_assets::png::codes::U_1F939_1F3FD_200D_2640_FE0F };
    ("man_juggling_tone3") => { &twemoji_assets::png::codes::U_1F939_1F3FD_200D_2642_FE0F };
    ("juggler_tone3") => { &twemoji_assets::png::codes::U_1F939_1F3FD };
    ("juggling_tone3") => { &twemoji_assets::png::codes::U_1F939_1F3FD };
    ("person_juggling_tone3") => { &twemoji_assets::png::codes::U_1F939_1F3FD };
    ("woman_juggling_tone4") => { &twemoji_assets::png::codes::U_1F939_1F3FE_200D_2640_FE0F };
    ("man_juggling_tone4") => { &twemoji_assets::png::codes::U_1F939_1F3FE_200D_2642_FE0F };
    ("juggler_tone4") => { &twemoji_assets::png::codes::U_1F939_1F3FE };
    ("juggling_tone4") => { &twemoji_assets::png::codes::U_1F939_1F3FE };
    ("person_juggling_tone4") => { &twemoji_assets::png::codes::U_1F939_1F3FE };
    ("woman_juggling_tone5") => { &twemoji_assets::png::codes::U_1F939_1F3FF_200D_2640_FE0F };
    ("man_juggling_tone5") => { &twemoji_assets::png::codes::U_1F939_1F3FF_200D_2642_FE0F };
    ("juggler_tone5") => { &twemoji_assets::png::codes::U_1F939_1F3FF };
    ("juggling_tone5") => { &twemoji_assets::png::codes::U_1F939_1F3FF };
    ("person_juggling_tone5") => { &twemoji_assets::png::codes::U_1F939_1F3FF };
    ("woman_juggling") => { &twemoji_assets::png::codes::U_1F939_200D_2640_FE0F };
    ("man_juggling") => { &twemoji_assets::png::codes::U_1F939_200D_2642_FE0F };
    ("juggler") => { &twemoji_assets::png::codes::U_1F939 };
    ("juggling") => { &twemoji_assets::png::codes::U_1F939 };
    ("person_juggling") => { &twemoji_assets::png::codes::U_1F939 };
    ("fencer") => { &twemoji_assets::png::codes::U_1F93A };
    ("fencing") => { &twemoji_assets::png::codes::U_1F93A };
    ("person_fencing") => { &twemoji_assets::png::codes::U_1F93A };
    ("women_wrestling") => { &twemoji_assets::png::codes::U_1F93C_200D_2640_FE0F };
    ("men_wrestling") => { &twemoji_assets::png::codes::U_1F93C_200D_2642_FE0F };
    ("people_wrestling") => { &twemoji_assets::png::codes::U_1F93C };
    ("wrestlers") => { &twemoji_assets::png::codes::U_1F93C };
    ("wrestling") => { &twemoji_assets::png::codes::U_1F93C };
    ("woman_playing_water_polo_tone1") => { &twemoji_assets::png::codes::U_1F93D_1F3FB_200D_2640_FE0F };
    ("man_playing_water_polo_tone1") => { &twemoji_assets::png::codes::U_1F93D_1F3FB_200D_2642_FE0F };
    ("person_playing_water_polo_tone1") => { &twemoji_assets::png::codes::U_1F93D_1F3FB };
    ("water_polo_tone1") => { &twemoji_assets::png::codes::U_1F93D_1F3FB };
    ("woman_playing_water_polo_tone2") => { &twemoji_assets::png::codes::U_1F93D_1F3FC_200D_2640_FE0F };
    ("man_playing_water_polo_tone2") => { &twemoji_assets::png::codes::U_1F93D_1F3FC_200D_2642_FE0F };
    ("person_playing_water_polo_tone2") => { &twemoji_assets::png::codes::U_1F93D_1F3FC };
    ("water_polo_tone2") => { &twemoji_assets::png::codes::U_1F93D_1F3FC };
    ("woman_playing_water_polo_tone3") => { &twemoji_assets::png::codes::U_1F93D_1F3FD_200D_2640_FE0F };
    ("man_playing_water_polo_tone3") => { &twemoji_assets::png::codes::U_1F93D_1F3FD_200D_2642_FE0F };
    ("person_playing_water_polo_tone3") => { &twemoji_assets::png::codes::U_1F93D_1F3FD };
    ("water_polo_tone3") => { &twemoji_assets::png::codes::U_1F93D_1F3FD };
    ("woman_playing_water_polo_tone4") => { &twemoji_assets::png::codes::U_1F93D_1F3FE_200D_2640_FE0F };
    ("man_playing_water_polo_tone4") => { &twemoji_assets::png::codes::U_1F93D_1F3FE_200D_2642_FE0F };
    ("person_playing_water_polo_tone4") => { &twemoji_assets::png::codes::U_1F93D_1F3FE };
    ("water_polo_tone4") => { &twemoji_assets::png::codes::U_1F93D_1F3FE };
    ("woman_playing_water_polo_tone5") => { &twemoji_assets::png::codes::U_1F93D_1F3FF_200D_2640_FE0F };
    ("man_playing_water_polo_tone5") => { &twemoji_assets::png::codes::U_1F93D_1F3FF_200D_2642_FE0F };
    ("person_playing_water_polo_tone5") => { &twemoji_assets::png::codes::U_1F93D_1F3FF };
    ("water_polo_tone5") => { &twemoji_assets::png::codes::U_1F93D_1F3FF };
    ("woman_playing_water_polo") => { &twemoji_assets::png::codes::U_1F93D_200D_2640_FE0F };
    ("man_playing_water_polo") => { &twemoji_assets::png::codes::U_1F93D_200D_2642_FE0F };
    ("person_playing_water_polo") => { &twemoji_assets::png::codes::U_1F93D };
    ("water_polo") => { &twemoji_assets::png::codes::U_1F93D };
    ("woman_playing_handball_tone1") => { &twemoji_assets::png::codes::U_1F93E_1F3FB_200D_2640_FE0F };
    ("man_playing_handball_tone1") => { &twemoji_assets::png::codes::U_1F93E_1F3FB_200D_2642_FE0F };
    ("handball_tone1") => { &twemoji_assets::png::codes::U_1F93E_1F3FB };
    ("person_playing_handball_tone1") => { &twemoji_assets::png::codes::U_1F93E_1F3FB };
    ("woman_playing_handball_tone2") => { &twemoji_assets::png::codes::U_1F93E_1F3FC_200D_2640_FE0F };
    ("man_playing_handball_tone2") => { &twemoji_assets::png::codes::U_1F93E_1F3FC_200D_2642_FE0F };
    ("handball_tone2") => { &twemoji_assets::png::codes::U_1F93E_1F3FC };
    ("person_playing_handball_tone2") => { &twemoji_assets::png::codes::U_1F93E_1F3FC };
    ("woman_playing_handball_tone3") => { &twemoji_assets::png::codes::U_1F93E_1F3FD_200D_2640_FE0F };
    ("man_playing_handball_tone3") => { &twemoji_assets::png::codes::U_1F93E_1F3FD_200D_2642_FE0F };
    ("handball_tone3") => { &twemoji_assets::png::codes::U_1F93E_1F3FD };
    ("person_playing_handball_tone3") => { &twemoji_assets::png::codes::U_1F93E_1F3FD };
    ("woman_playing_handball_tone4") => { &twemoji_assets::png::codes::U_1F93E_1F3FE_200D_2640_FE0F };
    ("man_playing_handball_tone4") => { &twemoji_assets::png::codes::U_1F93E_1F3FE_200D_2642_FE0F };
    ("handball_tone4") => { &twemoji_assets::png::codes::U_1F93E_1F3FE };
    ("person_playing_handball_tone4") => { &twemoji_assets::png::codes::U_1F93E_1F3FE };
    ("woman_playing_handball_tone5") => { &twemoji_assets::png::codes::U_1F93E_1F3FF_200D_2640_FE0F };
    ("man_playing_handball_tone5") => { &twemoji_assets::png::codes::U_1F93E_1F3FF_200D_2642_FE0F };
    ("handball_tone5") => { &twemoji_assets::png::codes::U_1F93E_1F3FF };
    ("person_playing_handball_tone5") => { &twemoji_assets::png::codes::U_1F93E_1F3FF };
    ("woman_playing_handball") => { &twemoji_assets::png::codes::U_1F93E_200D_2640_FE0F };
    ("man_playing_handball") => { &twemoji_assets::png::codes::U_1F93E_200D_2642_FE0F };
    ("handball") => { &twemoji_assets::png::codes::U_1F93E };
    ("person_playing_handball") => { &twemoji_assets::png::codes::U_1F93E };
    ("diving_mask") => { &twemoji_assets::png::codes::U_1F93F };
    ("wilted_flower") => { &twemoji_assets::png::codes::U_1F940 };
    ("drum") => { &twemoji_assets::png::codes::U_1F941 };
    ("clinking_glasses") => { &twemoji_assets::png::codes::U_1F942 };
    ("tumbler_glass") => { &twemoji_assets::png::codes::U_1F943 };
    ("whisky") => { &twemoji_assets::png::codes::U_1F943 };
    ("spoon") => { &twemoji_assets::png::codes::U_1F944 };
    ("goal_net") => { &twemoji_assets::png::codes::U_1F945 };
    ("1st") => { &twemoji_assets::png::codes::U_1F947 };
    ("first_place_medal") => { &twemoji_assets::png::codes::U_1F947 };
    ("2nd") => { &twemoji_assets::png::codes::U_1F948 };
    ("second_place_medal") => { &twemoji_assets::png::codes::U_1F948 };
    ("3rd") => { &twemoji_assets::png::codes::U_1F949 };
    ("third_place_medal") => { &twemoji_assets::png::codes::U_1F949 };
    ("boxing_glove") => { &twemoji_assets::png::codes::U_1F94A };
    ("martial_arts_uniform") => { &twemoji_assets::png::codes::U_1F94B };
    ("curling_stone") => { &twemoji_assets::png::codes::U_1F94C };
    ("lacrosse") => { &twemoji_assets::png::codes::U_1F94D };
    ("softball") => { &twemoji_assets::png::codes::U_1F94E };
    ("flying_disc") => { &twemoji_assets::png::codes::U_1F94F };
    ("croissant") => { &twemoji_assets::png::codes::U_1F950 };
    ("avocado") => { &twemoji_assets::png::codes::U_1F951 };
    ("cucumber") => { &twemoji_assets::png::codes::U_1F952 };
    ("bacon") => { &twemoji_assets::png::codes::U_1F953 };
    ("potato") => { &twemoji_assets::png::codes::U_1F954 };
    ("carrot") => { &twemoji_assets::png::codes::U_1F955 };
    ("baguette_bread") => { &twemoji_assets::png::codes::U_1F956 };
    ("green_salad") => { &twemoji_assets::png::codes::U_1F957 };
    ("salad") => { &twemoji_assets::png::codes::U_1F957 };
    ("shallow_pan_of_food") => { &twemoji_assets::png::codes::U_1F958 };
    ("stuffed_flatbread") => { &twemoji_assets::png::codes::U_1F959 };
    ("egg") => { &twemoji_assets::png::codes::U_1F95A };
    ("glass_of_milk") => { &twemoji_assets::png::codes::U_1F95B };
    ("milk") => { &twemoji_assets::png::codes::U_1F95B };
    ("peanuts") => { &twemoji_assets::png::codes::U_1F95C };
    ("kiwi") => { &twemoji_assets::png::codes::U_1F95D };
    ("pancakes") => { &twemoji_assets::png::codes::U_1F95E };
    ("dumpling") => { &twemoji_assets::png::codes::U_1F95F };
    ("fortune_cookie") => { &twemoji_assets::png::codes::U_1F960 };
    ("takeout_box") => { &twemoji_assets::png::codes::U_1F961 };
    ("chopsticks") => { &twemoji_assets::png::codes::U_1F962 };
    ("bowl_with_spoon") => { &twemoji_assets::png::codes::U_1F963 };
    ("cup_with_straw") => { &twemoji_assets::png::codes::U_1F964 };
    ("coconut") => { &twemoji_assets::png::codes::U_1F965 };
    ("broccoli") => { &twemoji_assets::png::codes::U_1F966 };
    ("pie") => { &twemoji_assets::png::codes::U_1F967 };
    ("pretzel") => { &twemoji_assets::png::codes::U_1F968 };
    ("cut_of_meat") => { &twemoji_assets::png::codes::U_1F969 };
    ("sandwich") => { &twemoji_assets::png::codes::U_1F96A };
    ("canned_food") => { &twemoji_assets::png::codes::U_1F96B };
    ("leafy_green") => { &twemoji_assets::png::codes::U_1F96C };
    ("mango") => { &twemoji_assets::png::codes::U_1F96D };
    ("moon_cake") => { &twemoji_assets::png::codes::U_1F96E };
    ("bagel") => { &twemoji_assets::png::codes::U_1F96F };
    ("smiling_face_with_3_hearts") => { &twemoji_assets::png::codes::U_1F970 };
    ("yawn") => { &twemoji_assets::png::codes::U_1F971 };
    ("yawning") => { &twemoji_assets::png::codes::U_1F971 };
    ("yawning_face") => { &twemoji_assets::png::codes::U_1F971 };
    ("smiling_face_with_tear") => { &twemoji_assets::png::codes::U_1F972 };
    ("hooray") => { &twemoji_assets::png::codes::U_1F973 };
    ("partying") => { &twemoji_assets::png::codes::U_1F973 };
    ("partying_face") => { &twemoji_assets::png::codes::U_1F973 };
    ("woozy") => { &twemoji_assets::png::codes::U_1F974 };
    ("woozy_face") => { &twemoji_assets::png::codes::U_1F974 };
    ("hot") => { &twemoji_assets::png::codes::U_1F975 };
    ("hot_face") => { &twemoji_assets::png::codes::U_1F975 };
    ("cold") => { &twemoji_assets::png::codes::U_1F976 };
    ("cold_face") => { &twemoji_assets::png::codes::U_1F976 };
    ("ninja_tone1") => { &twemoji_assets::png::codes::U_1F977_1F3FB };
    ("ninja_tone2") => { &twemoji_assets::png::codes::U_1F977_1F3FC };
    ("ninja_tone3") => { &twemoji_assets::png::codes::U_1F977_1F3FD };
    ("ninja_tone4") => { &twemoji_assets::png::codes::U_1F977_1F3FE };
    ("ninja_tone5") => { &twemoji_assets::png::codes::U_1F977_1F3FF };
    ("ninja") => { &twemoji_assets::png::codes::U_1F977 };
    ("disguised") => { &twemoji_assets::png::codes::U_1F978 };
    ("disguised_face") => { &twemoji_assets::png::codes::U_1F978 };
    ("face_holding_back_tears") => { &twemoji_assets::png::codes::U_1F979 };
    ("watery_eyes") => { &twemoji_assets::png::codes::U_1F979 };
    ("pleading") => { &twemoji_assets::png::codes::U_1F97A };
    ("pleading_face") => { &twemoji_assets::png::codes::U_1F97A };
    ("sari") => { &twemoji_assets::png::codes::U_1F97B };
    ("lab_coat") => { &twemoji_assets::png::codes::U_1F97C };
    ("goggles") => { &twemoji_assets::png::codes::U_1F97D };
    ("hiking_boot") => { &twemoji_assets::png::codes::U_1F97E };
    ("flat_shoe") => { &twemoji_assets::png::codes::U_1F97F };
    ("womans_flat_shoe") => { &twemoji_assets::png::codes::U_1F97F };
    ("crab") => { &twemoji_assets::png::codes::U_1F980 };
    ("lion") => { &twemoji_assets::png::codes::U_1F981 };
    ("lion_face") => { &twemoji_assets::png::codes::U_1F981 };
    ("scorpion") => { &twemoji_assets::png::codes::U_1F982 };
    ("turkey") => { &twemoji_assets::png::codes::U_1F983 };
    ("unicorn") => { &twemoji_assets::png::codes::U_1F984 };
    ("unicorn_face") => { &twemoji_assets::png::codes::U_1F984 };
    ("eagle") => { &twemoji_assets::png::codes::U_1F985 };
    ("duck") => { &twemoji_assets::png::codes::U_1F986 };
    ("bat") => { &twemoji_assets::png::codes::U_1F987 };
    ("shark") => { &twemoji_assets::png::codes::U_1F988 };
    ("owl") => { &twemoji_assets::png::codes::U_1F989 };
    ("fox") => { &twemoji_assets::png::codes::U_1F98A };
    ("fox_face") => { &twemoji_assets::png::codes::U_1F98A };
    ("butterfly") => { &twemoji_assets::png::codes::U_1F98B };
    ("deer") => { &twemoji_assets::png::codes::U_1F98C };
    ("gorilla") => { &twemoji_assets::png::codes::U_1F98D };
    ("lizard") => { &twemoji_assets::png::codes::U_1F98E };
    ("rhino") => { &twemoji_assets::png::codes::U_1F98F };
    ("rhinoceros") => { &twemoji_assets::png::codes::U_1F98F };
    ("shrimp") => { &twemoji_assets::png::codes::U_1F990 };
    ("squid") => { &twemoji_assets::png::codes::U_1F991 };
    ("giraffe") => { &twemoji_assets::png::codes::U_1F992 };
    ("zebra") => { &twemoji_assets::png::codes::U_1F993 };
    ("hedgehog") => { &twemoji_assets::png::codes::U_1F994 };
    ("sauropod") => { &twemoji_assets::png::codes::U_1F995 };
    ("t-rex") => { &twemoji_assets::png::codes::U_1F996 };
    ("trex") => { &twemoji_assets::png::codes::U_1F996 };
    ("cricket") => { &twemoji_assets::png::codes::U_1F997 };
    ("kangaroo") => { &twemoji_assets::png::codes::U_1F998 };
    ("llama") => { &twemoji_assets::png::codes::U_1F999 };
    ("peacock") => { &twemoji_assets::png::codes::U_1F99A };
    ("hippo") => { &twemoji_assets::png::codes::U_1F99B };
    ("parrot") => { &twemoji_assets::png::codes::U_1F99C };
    ("raccoon") => { &twemoji_assets::png::codes::U_1F99D };
    ("lobster") => { &twemoji_assets::png::codes::U_1F99E };
    ("mosquito") => { &twemoji_assets::png::codes::U_1F99F };
    ("microbe") => { &twemoji_assets::png::codes::U_1F9A0 };
    ("badger") => { &twemoji_assets::png::codes::U_1F9A1 };
    ("swan") => { &twemoji_assets::png::codes::U_1F9A2 };
    ("mammoth") => { &twemoji_assets::png::codes::U_1F9A3 };
    ("dodo") => { &twemoji_assets::png::codes::U_1F9A4 };
    ("sloth") => { &twemoji_assets::png::codes::U_1F9A5 };
    ("otter") => { &twemoji_assets::png::codes::U_1F9A6 };
    ("orangutan") => { &twemoji_assets::png::codes::U_1F9A7 };
    ("skunk") => { &twemoji_assets::png::codes::U_1F9A8 };
    ("flamingo") => { &twemoji_assets::png::codes::U_1F9A9 };
    ("oyster") => { &twemoji_assets::png::codes::U_1F9AA };
    ("beaver") => { &twemoji_assets::png::codes::U_1F9AB };
    ("bison") => { &twemoji_assets::png::codes::U_1F9AC };
    ("seal") => { &twemoji_assets::png::codes::U_1F9AD };
    ("guide_dog") => { &twemoji_assets::png::codes::U_1F9AE };
    ("probing_cane") => { &twemoji_assets::png::codes::U_1F9AF };
    ("white_cane") => { &twemoji_assets::png::codes::U_1F9AF };
    ("red_hair") => { &twemoji_assets::png::codes::U_1F9B0 };
    ("curly_hair") => { &twemoji_assets::png::codes::U_1F9B1 };
    ("no_hair") => { &twemoji_assets::png::codes::U_1F9B2 };
    ("white_hair") => { &twemoji_assets::png::codes::U_1F9B3 };
    ("bone") => { &twemoji_assets::png::codes::U_1F9B4 };
    ("leg_tone1") => { &twemoji_assets::png::codes::U_1F9B5_1F3FB };
    ("leg_tone2") => { &twemoji_assets::png::codes::U_1F9B5_1F3FC };
    ("leg_tone3") => { &twemoji_assets::png::codes::U_1F9B5_1F3FD };
    ("leg_tone4") => { &twemoji_assets::png::codes::U_1F9B5_1F3FE };
    ("leg_tone5") => { &twemoji_assets::png::codes::U_1F9B5_1F3FF };
    ("leg") => { &twemoji_assets::png::codes::U_1F9B5 };
    ("foot_tone1") => { &twemoji_assets::png::codes::U_1F9B6_1F3FB };
    ("foot_tone2") => { &twemoji_assets::png::codes::U_1F9B6_1F3FC };
    ("foot_tone3") => { &twemoji_assets::png::codes::U_1F9B6_1F3FD };
    ("foot_tone4") => { &twemoji_assets::png::codes::U_1F9B6_1F3FE };
    ("foot_tone5") => { &twemoji_assets::png::codes::U_1F9B6_1F3FF };
    ("foot") => { &twemoji_assets::png::codes::U_1F9B6 };
    ("tooth") => { &twemoji_assets::png::codes::U_1F9B7 };
    ("woman_superhero_tone1") => { &twemoji_assets::png::codes::U_1F9B8_1F3FB_200D_2640_FE0F };
    ("man_superhero_tone1") => { &twemoji_assets::png::codes::U_1F9B8_1F3FB_200D_2642_FE0F };
    ("superhero_tone1") => { &twemoji_assets::png::codes::U_1F9B8_1F3FB };
    ("woman_superhero_tone2") => { &twemoji_assets::png::codes::U_1F9B8_1F3FC_200D_2640_FE0F };
    ("man_superhero_tone2") => { &twemoji_assets::png::codes::U_1F9B8_1F3FC_200D_2642_FE0F };
    ("superhero_tone2") => { &twemoji_assets::png::codes::U_1F9B8_1F3FC };
    ("woman_superhero_tone3") => { &twemoji_assets::png::codes::U_1F9B8_1F3FD_200D_2640_FE0F };
    ("man_superhero_tone3") => { &twemoji_assets::png::codes::U_1F9B8_1F3FD_200D_2642_FE0F };
    ("superhero_tone3") => { &twemoji_assets::png::codes::U_1F9B8_1F3FD };
    ("woman_superhero_tone4") => { &twemoji_assets::png::codes::U_1F9B8_1F3FE_200D_2640_FE0F };
    ("man_superhero_tone4") => { &twemoji_assets::png::codes::U_1F9B8_1F3FE_200D_2642_FE0F };
    ("superhero_tone4") => { &twemoji_assets::png::codes::U_1F9B8_1F3FE };
    ("woman_superhero_tone5") => { &twemoji_assets::png::codes::U_1F9B8_1F3FF_200D_2640_FE0F };
    ("man_superhero_tone5") => { &twemoji_assets::png::codes::U_1F9B8_1F3FF_200D_2642_FE0F };
    ("superhero_tone5") => { &twemoji_assets::png::codes::U_1F9B8_1F3FF };
    ("woman_superhero") => { &twemoji_assets::png::codes::U_1F9B8_200D_2640_FE0F };
    ("man_superhero") => { &twemoji_assets::png::codes::U_1F9B8_200D_2642_FE0F };
    ("superhero") => { &twemoji_assets::png::codes::U_1F9B8 };
    ("woman_supervillain_tone1") => { &twemoji_assets::png::codes::U_1F9B9_1F3FB_200D_2640_FE0F };
    ("man_supervillain_tone1") => { &twemoji_assets::png::codes::U_1F9B9_1F3FB_200D_2642_FE0F };
    ("supervillain_tone1") => { &twemoji_assets::png::codes::U_1F9B9_1F3FB };
    ("woman_supervillain_tone2") => { &twemoji_assets::png::codes::U_1F9B9_1F3FC_200D_2640_FE0F };
    ("man_supervillain_tone2") => { &twemoji_assets::png::codes::U_1F9B9_1F3FC_200D_2642_FE0F };
    ("supervillain_tone2") => { &twemoji_assets::png::codes::U_1F9B9_1F3FC };
    ("woman_supervillain_tone3") => { &twemoji_assets::png::codes::U_1F9B9_1F3FD_200D_2640_FE0F };
    ("man_supervillain_tone3") => { &twemoji_assets::png::codes::U_1F9B9_1F3FD_200D_2642_FE0F };
    ("supervillain_tone3") => { &twemoji_assets::png::codes::U_1F9B9_1F3FD };
    ("woman_supervillain_tone4") => { &twemoji_assets::png::codes::U_1F9B9_1F3FE_200D_2640_FE0F };
    ("man_supervillain_tone4") => { &twemoji_assets::png::codes::U_1F9B9_1F3FE_200D_2642_FE0F };
    ("supervillain_tone4") => { &twemoji_assets::png::codes::U_1F9B9_1F3FE };
    ("woman_supervillain_tone5") => { &twemoji_assets::png::codes::U_1F9B9_1F3FF_200D_2640_FE0F };
    ("man_supervillain_tone5") => { &twemoji_assets::png::codes::U_1F9B9_1F3FF_200D_2642_FE0F };
    ("supervillain_tone5") => { &twemoji_assets::png::codes::U_1F9B9_1F3FF };
    ("woman_supervillain") => { &twemoji_assets::png::codes::U_1F9B9_200D_2640_FE0F };
    ("man_supervillain") => { &twemoji_assets::png::codes::U_1F9B9_200D_2642_FE0F };
    ("supervillain") => { &twemoji_assets::png::codes::U_1F9B9 };
    ("safety_vest") => { &twemoji_assets::png::codes::U_1F9BA };
    ("ear_with_hearing_aid_tone1") => { &twemoji_assets::png::codes::U_1F9BB_1F3FB };
    ("hearing_aid_tone1") => { &twemoji_assets::png::codes::U_1F9BB_1F3FB };
    ("ear_with_hearing_aid_tone2") => { &twemoji_assets::png::codes::U_1F9BB_1F3FC };
    ("hearing_aid_tone2") => { &twemoji_assets::png::codes::U_1F9BB_1F3FC };
    ("ear_with_hearing_aid_tone3") => { &twemoji_assets::png::codes::U_1F9BB_1F3FD };
    ("hearing_aid_tone3") => { &twemoji_assets::png::codes::U_1F9BB_1F3FD };
    ("ear_with_hearing_aid_tone4") => { &twemoji_assets::png::codes::U_1F9BB_1F3FE };
    ("hearing_aid_tone4") => { &twemoji_assets::png::codes::U_1F9BB_1F3FE };
    ("ear_with_hearing_aid_tone5") => { &twemoji_assets::png::codes::U_1F9BB_1F3FF };
    ("hearing_aid_tone5") => { &twemoji_assets::png::codes::U_1F9BB_1F3FF };
    ("ear_with_hearing_aid") => { &twemoji_assets::png::codes::U_1F9BB };
    ("hearing_aid") => { &twemoji_assets::png::codes::U_1F9BB };
    ("motorized_wheelchair") => { &twemoji_assets::png::codes::U_1F9BC };
    ("manual_wheelchair") => { &twemoji_assets::png::codes::U_1F9BD };
    ("mechanical_arm") => { &twemoji_assets::png::codes::U_1F9BE };
    ("mechanical_leg") => { &twemoji_assets::png::codes::U_1F9BF };
    ("cheese") => { &twemoji_assets::png::codes::U_1F9C0 };
    ("cupcake") => { &twemoji_assets::png::codes::U_1F9C1 };
    ("salt") => { &twemoji_assets::png::codes::U_1F9C2 };
    ("beverage_box") => { &twemoji_assets::png::codes::U_1F9C3 };
    ("juice_box") => { &twemoji_assets::png::codes::U_1F9C3 };
    ("garlic") => { &twemoji_assets::png::codes::U_1F9C4 };
    ("onion") => { &twemoji_assets::png::codes::U_1F9C5 };
    ("falafel") => { &twemoji_assets::png::codes::U_1F9C6 };
    ("waffle") => { &twemoji_assets::png::codes::U_1F9C7 };
    ("butter") => { &twemoji_assets::png::codes::U_1F9C8 };
    ("mate") => { &twemoji_assets::png::codes::U_1F9C9 };
    ("ice") => { &twemoji_assets::png::codes::U_1F9CA };
    ("ice_cube") => { &twemoji_assets::png::codes::U_1F9CA };
    ("boba_drink") => { &twemoji_assets::png::codes::U_1F9CB };
    ("bubble_tea") => { &twemoji_assets::png::codes::U_1F9CB };
    ("troll") => { &twemoji_assets::png::codes::U_1F9CC };
    ("woman_standing_tone1") => { &twemoji_assets::png::codes::U_1F9CD_1F3FB_200D_2640_FE0F };
    ("man_standing_tone1") => { &twemoji_assets::png::codes::U_1F9CD_1F3FB_200D_2642_FE0F };
    ("person_standing_tone1") => { &twemoji_assets::png::codes::U_1F9CD_1F3FB };
    ("standing_tone1") => { &twemoji_assets::png::codes::U_1F9CD_1F3FB };
    ("woman_standing_tone2") => { &twemoji_assets::png::codes::U_1F9CD_1F3FC_200D_2640_FE0F };
    ("man_standing_tone2") => { &twemoji_assets::png::codes::U_1F9CD_1F3FC_200D_2642_FE0F };
    ("person_standing_tone2") => { &twemoji_assets::png::codes::U_1F9CD_1F3FC };
    ("standing_tone2") => { &twemoji_assets::png::codes::U_1F9CD_1F3FC };
    ("woman_standing_tone3") => { &twemoji_assets::png::codes::U_1F9CD_1F3FD_200D_2640_FE0F };
    ("man_standing_tone3") => { &twemoji_assets::png::codes::U_1F9CD_1F3FD_200D_2642_FE0F };
    ("person_standing_tone3") => { &twemoji_assets::png::codes::U_1F9CD_1F3FD };
    ("standing_tone3") => { &twemoji_assets::png::codes::U_1F9CD_1F3FD };
    ("woman_standing_tone4") => { &twemoji_assets::png::codes::U_1F9CD_1F3FE_200D_2640_FE0F };
    ("man_standing_tone4") => { &twemoji_assets::png::codes::U_1F9CD_1F3FE_200D_2642_FE0F };
    ("person_standing_tone4") => { &twemoji_assets::png::codes::U_1F9CD_1F3FE };
    ("standing_tone4") => { &twemoji_assets::png::codes::U_1F9CD_1F3FE };
    ("woman_standing_tone5") => { &twemoji_assets::png::codes::U_1F9CD_1F3FF_200D_2640_FE0F };
    ("man_standing_tone5") => { &twemoji_assets::png::codes::U_1F9CD_1F3FF_200D_2642_FE0F };
    ("person_standing_tone5") => { &twemoji_assets::png::codes::U_1F9CD_1F3FF };
    ("standing_tone5") => { &twemoji_assets::png::codes::U_1F9CD_1F3FF };
    ("woman_standing") => { &twemoji_assets::png::codes::U_1F9CD_200D_2640_FE0F };
    ("man_standing") => { &twemoji_assets::png::codes::U_1F9CD_200D_2642_FE0F };
    ("person_standing") => { &twemoji_assets::png::codes::U_1F9CD };
    ("standing") => { &twemoji_assets::png::codes::U_1F9CD };
    ("woman_kneeling_tone1") => { &twemoji_assets::png::codes::U_1F9CE_1F3FB_200D_2640_FE0F };
    ("man_kneeling_tone1") => { &twemoji_assets::png::codes::U_1F9CE_1F3FB_200D_2642_FE0F };
    ("kneeling_tone1") => { &twemoji_assets::png::codes::U_1F9CE_1F3FB };
    ("person_kneeling_tone1") => { &twemoji_assets::png::codes::U_1F9CE_1F3FB };
    ("woman_kneeling_tone2") => { &twemoji_assets::png::codes::U_1F9CE_1F3FC_200D_2640_FE0F };
    ("man_kneeling_tone2") => { &twemoji_assets::png::codes::U_1F9CE_1F3FC_200D_2642_FE0F };
    ("kneeling_tone2") => { &twemoji_assets::png::codes::U_1F9CE_1F3FC };
    ("person_kneeling_tone2") => { &twemoji_assets::png::codes::U_1F9CE_1F3FC };
    ("woman_kneeling_tone3") => { &twemoji_assets::png::codes::U_1F9CE_1F3FD_200D_2640_FE0F };
    ("man_kneeling_tone3") => { &twemoji_assets::png::codes::U_1F9CE_1F3FD_200D_2642_FE0F };
    ("kneeling_tone3") => { &twemoji_assets::png::codes::U_1F9CE_1F3FD };
    ("person_kneeling_tone3") => { &twemoji_assets::png::codes::U_1F9CE_1F3FD };
    ("woman_kneeling_tone4") => { &twemoji_assets::png::codes::U_1F9CE_1F3FE_200D_2640_FE0F };
    ("man_kneeling_tone4") => { &twemoji_assets::png::codes::U_1F9CE_1F3FE_200D_2642_FE0F };
    ("kneeling_tone4") => { &twemoji_assets::png::codes::U_1F9CE_1F3FE };
    ("person_kneeling_tone4") => { &twemoji_assets::png::codes::U_1F9CE_1F3FE };
    ("woman_kneeling_tone5") => { &twemoji_assets::png::codes::U_1F9CE_1F3FF_200D_2640_FE0F };
    ("man_kneeling_tone5") => { &twemoji_assets::png::codes::U_1F9CE_1F3FF_200D_2642_FE0F };
    ("kneeling_tone5") => { &twemoji_assets::png::codes::U_1F9CE_1F3FF };
    ("person_kneeling_tone5") => { &twemoji_assets::png::codes::U_1F9CE_1F3FF };
    ("woman_kneeling") => { &twemoji_assets::png::codes::U_1F9CE_200D_2640_FE0F };
    ("man_kneeling") => { &twemoji_assets::png::codes::U_1F9CE_200D_2642_FE0F };
    ("kneeling") => { &twemoji_assets::png::codes::U_1F9CE };
    ("person_kneeling") => { &twemoji_assets::png::codes::U_1F9CE };
    ("deaf_woman_tone1") => { &twemoji_assets::png::codes::U_1F9CF_1F3FB_200D_2640_FE0F };
    ("deaf_man_tone1") => { &twemoji_assets::png::codes::U_1F9CF_1F3FB_200D_2642_FE0F };
    ("deaf_person_tone1") => { &twemoji_assets::png::codes::U_1F9CF_1F3FB };
    ("deaf_woman_tone2") => { &twemoji_assets::png::codes::U_1F9CF_1F3FC_200D_2640_FE0F };
    ("deaf_man_tone2") => { &twemoji_assets::png::codes::U_1F9CF_1F3FC_200D_2642_FE0F };
    ("deaf_person_tone2") => { &twemoji_assets::png::codes::U_1F9CF_1F3FC };
    ("deaf_woman_tone3") => { &twemoji_assets::png::codes::U_1F9CF_1F3FD_200D_2640_FE0F };
    ("deaf_man_tone3") => { &twemoji_assets::png::codes::U_1F9CF_1F3FD_200D_2642_FE0F };
    ("deaf_person_tone3") => { &twemoji_assets::png::codes::U_1F9CF_1F3FD };
    ("deaf_woman_tone4") => { &twemoji_assets::png::codes::U_1F9CF_1F3FE_200D_2640_FE0F };
    ("deaf_man_tone4") => { &twemoji_assets::png::codes::U_1F9CF_1F3FE_200D_2642_FE0F };
    ("deaf_person_tone4") => { &twemoji_assets::png::codes::U_1F9CF_1F3FE };
    ("deaf_woman_tone5") => { &twemoji_assets::png::codes::U_1F9CF_1F3FF_200D_2640_FE0F };
    ("deaf_man_tone5") => { &twemoji_assets::png::codes::U_1F9CF_1F3FF_200D_2642_FE0F };
    ("deaf_person_tone5") => { &twemoji_assets::png::codes::U_1F9CF_1F3FF };
    ("deaf_woman") => { &twemoji_assets::png::codes::U_1F9CF_200D_2640_FE0F };
    ("deaf_man") => { &twemoji_assets::png::codes::U_1F9CF_200D_2642_FE0F };
    ("deaf_person") => { &twemoji_assets::png::codes::U_1F9CF };
    ("face_with_monocle") => { &twemoji_assets::png::codes::U_1F9D0 };
    ("farmer_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F33E };
    ("cook_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F373 };
    ("person_feeding_baby_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F37C };
    ("mx_claus_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F384 };
    ("student_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F393 };
    ("singer_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F3A4 };
    ("artist_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F3A8 };
    ("teacher_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F3EB };
    ("factory_worker_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F3ED };
    ("technologist_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F4BB };
    ("office_worker_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F4BC };
    ("mechanic_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F527 };
    ("scientist_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F52C };
    ("astronaut_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F680 };
    ("firefighter_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F692 };
    ("people_holding_hands_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F91D_200D_1F9D1_1F3FB };
    ("people_holding_hands_tone1-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F91D_200D_1F9D1_1F3FC };
    ("people_holding_hands_tone1-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F91D_200D_1F9D1_1F3FD };
    ("people_holding_hands_tone1-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F91D_200D_1F9D1_1F3FE };
    ("people_holding_hands_tone1-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F91D_200D_1F9D1_1F3FF };
    ("person_with_probing_cane_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F9AF };
    ("person_with_white_cane_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F9AF };
    ("red_haired_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F9B0 };
    ("curly_haired_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F9B1 };
    ("bald_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F9B2 };
    ("white_haired_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F9B3 };
    ("person_in_motorized_wheelchair_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F9BC };
    ("person_in_manual_wheelchair_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_1F9BD };
    ("health_worker_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2695_FE0F };
    ("judge_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2696_FE0F };
    ("pilot_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2708_FE0F };
    ("couple_kiss_tone1-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC };
    ("couplekiss_tone1-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC };
    ("couple_kiss_tone1-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD };
    ("couplekiss_tone1-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD };
    ("couple_kiss_tone1-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE };
    ("couplekiss_tone1-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE };
    ("couple_kiss_tone1-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF };
    ("couplekiss_tone1-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF };
    ("couple_with_heart_tone1-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F9D1_1F3FC };
    ("couple_with_heart_tone1-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F9D1_1F3FD };
    ("couple_with_heart_tone1-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F9D1_1F3FE };
    ("couple_with_heart_tone1-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB_200D_2764_FE0F_200D_1F9D1_1F3FF };
    ("adult_tone1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FB };
    ("farmer_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F33E };
    ("cook_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F373 };
    ("person_feeding_baby_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F37C };
    ("mx_claus_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F384 };
    ("student_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F393 };
    ("singer_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F3A4 };
    ("artist_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F3A8 };
    ("teacher_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F3EB };
    ("factory_worker_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F3ED };
    ("technologist_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F4BB };
    ("office_worker_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F4BC };
    ("mechanic_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F527 };
    ("scientist_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F52C };
    ("astronaut_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F680 };
    ("firefighter_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F692 };
    ("people_holding_hands_tone2-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F91D_200D_1F9D1_1F3FB };
    ("people_holding_hands_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F91D_200D_1F9D1_1F3FC };
    ("people_holding_hands_tone2-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F91D_200D_1F9D1_1F3FD };
    ("people_holding_hands_tone2-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F91D_200D_1F9D1_1F3FE };
    ("people_holding_hands_tone2-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F91D_200D_1F9D1_1F3FF };
    ("person_with_probing_cane_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F9AF };
    ("person_with_white_cane_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F9AF };
    ("red_haired_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F9B0 };
    ("curly_haired_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F9B1 };
    ("bald_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F9B2 };
    ("white_haired_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F9B3 };
    ("person_in_motorized_wheelchair_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F9BC };
    ("person_in_manual_wheelchair_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_1F9BD };
    ("health_worker_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2695_FE0F };
    ("judge_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2696_FE0F };
    ("pilot_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2708_FE0F };
    ("couple_kiss_tone2-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB };
    ("couplekiss_tone2-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB };
    ("couple_kiss_tone2-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD };
    ("couplekiss_tone2-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD };
    ("couple_kiss_tone2-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE };
    ("couplekiss_tone2-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE };
    ("couple_kiss_tone2-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF };
    ("couplekiss_tone2-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF };
    ("couple_with_heart_tone2-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F9D1_1F3FB };
    ("couple_with_heart_tone2-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F9D1_1F3FD };
    ("couple_with_heart_tone2-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F9D1_1F3FE };
    ("couple_with_heart_tone2-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC_200D_2764_FE0F_200D_1F9D1_1F3FF };
    ("adult_tone2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FC };
    ("farmer_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F33E };
    ("cook_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F373 };
    ("person_feeding_baby_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F37C };
    ("mx_claus_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F384 };
    ("student_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F393 };
    ("singer_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F3A4 };
    ("artist_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F3A8 };
    ("teacher_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F3EB };
    ("factory_worker_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F3ED };
    ("technologist_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F4BB };
    ("office_worker_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F4BC };
    ("mechanic_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F527 };
    ("scientist_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F52C };
    ("astronaut_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F680 };
    ("firefighter_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F692 };
    ("people_holding_hands_tone3-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F91D_200D_1F9D1_1F3FB };
    ("people_holding_hands_tone3-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F91D_200D_1F9D1_1F3FC };
    ("people_holding_hands_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F91D_200D_1F9D1_1F3FD };
    ("people_holding_hands_tone3-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F91D_200D_1F9D1_1F3FE };
    ("people_holding_hands_tone3-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F91D_200D_1F9D1_1F3FF };
    ("person_with_probing_cane_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F9AF };
    ("person_with_white_cane_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F9AF };
    ("red_haired_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F9B0 };
    ("curly_haired_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F9B1 };
    ("bald_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F9B2 };
    ("white_haired_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F9B3 };
    ("person_in_motorized_wheelchair_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F9BC };
    ("person_in_manual_wheelchair_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_1F9BD };
    ("health_worker_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2695_FE0F };
    ("judge_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2696_FE0F };
    ("pilot_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2708_FE0F };
    ("couple_kiss_tone3-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB };
    ("couplekiss_tone3-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB };
    ("couple_kiss_tone3-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC };
    ("couplekiss_tone3-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC };
    ("couple_kiss_tone3-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE };
    ("couplekiss_tone3-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE };
    ("couple_kiss_tone3-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF };
    ("couplekiss_tone3-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF };
    ("couple_with_heart_tone3-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F9D1_1F3FB };
    ("couple_with_heart_tone3-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F9D1_1F3FC };
    ("couple_with_heart_tone3-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F9D1_1F3FE };
    ("couple_with_heart_tone3-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD_200D_2764_FE0F_200D_1F9D1_1F3FF };
    ("adult_tone3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FD };
    ("farmer_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F33E };
    ("cook_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F373 };
    ("person_feeding_baby_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F37C };
    ("mx_claus_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F384 };
    ("student_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F393 };
    ("singer_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F3A4 };
    ("artist_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F3A8 };
    ("teacher_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F3EB };
    ("factory_worker_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F3ED };
    ("technologist_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F4BB };
    ("office_worker_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F4BC };
    ("mechanic_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F527 };
    ("scientist_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F52C };
    ("astronaut_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F680 };
    ("firefighter_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F692 };
    ("people_holding_hands_tone4-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F91D_200D_1F9D1_1F3FB };
    ("people_holding_hands_tone4-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F91D_200D_1F9D1_1F3FC };
    ("people_holding_hands_tone4-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F91D_200D_1F9D1_1F3FD };
    ("people_holding_hands_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F91D_200D_1F9D1_1F3FE };
    ("people_holding_hands_tone4-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F91D_200D_1F9D1_1F3FF };
    ("person_with_probing_cane_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F9AF };
    ("person_with_white_cane_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F9AF };
    ("red_haired_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F9B0 };
    ("curly_haired_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F9B1 };
    ("bald_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F9B2 };
    ("white_haired_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F9B3 };
    ("person_in_motorized_wheelchair_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F9BC };
    ("person_in_manual_wheelchair_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_1F9BD };
    ("health_worker_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2695_FE0F };
    ("judge_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2696_FE0F };
    ("pilot_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2708_FE0F };
    ("couple_kiss_tone4-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB };
    ("couplekiss_tone4-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB };
    ("couple_kiss_tone4-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC };
    ("couplekiss_tone4-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC };
    ("couple_kiss_tone4-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD };
    ("couplekiss_tone4-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD };
    ("couple_kiss_tone4-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF };
    ("couplekiss_tone4-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FF };
    ("couple_with_heart_tone4-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F9D1_1F3FB };
    ("couple_with_heart_tone4-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F9D1_1F3FC };
    ("couple_with_heart_tone4-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F9D1_1F3FD };
    ("couple_with_heart_tone4-5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE_200D_2764_FE0F_200D_1F9D1_1F3FF };
    ("adult_tone4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FE };
    ("farmer_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F33E };
    ("cook_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F373 };
    ("person_feeding_baby_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F37C };
    ("mx_claus_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F384 };
    ("student_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F393 };
    ("singer_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F3A4 };
    ("artist_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F3A8 };
    ("teacher_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F3EB };
    ("factory_worker_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F3ED };
    ("technologist_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F4BB };
    ("office_worker_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F4BC };
    ("mechanic_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F527 };
    ("scientist_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F52C };
    ("astronaut_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F680 };
    ("firefighter_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F692 };
    ("people_holding_hands_tone5-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F91D_200D_1F9D1_1F3FB };
    ("people_holding_hands_tone5-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F91D_200D_1F9D1_1F3FC };
    ("people_holding_hands_tone5-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F91D_200D_1F9D1_1F3FD };
    ("people_holding_hands_tone5-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F91D_200D_1F9D1_1F3FE };
    ("people_holding_hands_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F91D_200D_1F9D1_1F3FF };
    ("person_with_probing_cane_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F9AF };
    ("person_with_white_cane_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F9AF };
    ("red_haired_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F9B0 };
    ("curly_haired_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F9B1 };
    ("bald_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F9B2 };
    ("white_haired_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F9B3 };
    ("person_in_motorized_wheelchair_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F9BC };
    ("person_in_manual_wheelchair_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_1F9BD };
    ("health_worker_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2695_FE0F };
    ("judge_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2696_FE0F };
    ("pilot_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2708_FE0F };
    ("couple_kiss_tone5-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB };
    ("couplekiss_tone5-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FB };
    ("couple_kiss_tone5-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC };
    ("couplekiss_tone5-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FC };
    ("couple_kiss_tone5-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD };
    ("couplekiss_tone5-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FD };
    ("couple_kiss_tone5-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE };
    ("couplekiss_tone5-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F48B_200D_1F9D1_1F3FE };
    ("couple_with_heart_tone5-1") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F9D1_1F3FB };
    ("couple_with_heart_tone5-2") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F9D1_1F3FC };
    ("couple_with_heart_tone5-3") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F9D1_1F3FD };
    ("couple_with_heart_tone5-4") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF_200D_2764_FE0F_200D_1F9D1_1F3FE };
    ("adult_tone5") => { &twemoji_assets::png::codes::U_1F9D1_1F3FF };
    ("farmer") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F33E };
    ("cook") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F373 };
    ("person_feeding_baby") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F37C };
    ("mx_claus") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F384 };
    ("student") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F393 };
    ("singer") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F3A4 };
    ("artist") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F3A8 };
    ("teacher") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F3EB };
    ("factory_worker") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F3ED };
    ("technologist") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F4BB };
    ("office_worker") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F4BC };
    ("mechanic") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F527 };
    ("scientist") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F52C };
    ("astronaut") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F680 };
    ("firefighter") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F692 };
    ("people_holding_hands") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F91D_200D_1F9D1 };
    ("person_with_probing_cane") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F9AF };
    ("person_with_white_cane") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F9AF };
    ("red_haired") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F9B0 };
    ("curly_haired") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F9B1 };
    ("bald") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F9B2 };
    ("white_haired") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F9B3 };
    ("person_in_motorized_wheelchair") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F9BC };
    ("person_in_manual_wheelchair") => { &twemoji_assets::png::codes::U_1F9D1_200D_1F9BD };
    ("health_worker") => { &twemoji_assets::png::codes::U_1F9D1_200D_2695_FE0F };
    ("judge") => { &twemoji_assets::png::codes::U_1F9D1_200D_2696_FE0F };
    ("pilot") => { &twemoji_assets::png::codes::U_1F9D1_200D_2708_FE0F };
    ("adult") => { &twemoji_assets::png::codes::U_1F9D1 };
    ("child_tone1") => { &twemoji_assets::png::codes::U_1F9D2_1F3FB };
    ("child_tone2") => { &twemoji_assets::png::codes::U_1F9D2_1F3FC };
    ("child_tone3") => { &twemoji_assets::png::codes::U_1F9D2_1F3FD };
    ("child_tone4") => { &twemoji_assets::png::codes::U_1F9D2_1F3FE };
    ("child_tone5") => { &twemoji_assets::png::codes::U_1F9D2_1F3FF };
    ("child") => { &twemoji_assets::png::codes::U_1F9D2 };
    ("older_adult_tone1") => { &twemoji_assets::png::codes::U_1F9D3_1F3FB };
    ("older_adult_tone2") => { &twemoji_assets::png::codes::U_1F9D3_1F3FC };
    ("older_adult_tone3") => { &twemoji_assets::png::codes::U_1F9D3_1F3FD };
    ("older_adult_tone4") => { &twemoji_assets::png::codes::U_1F9D3_1F3FE };
    ("older_adult_tone5") => { &twemoji_assets::png::codes::U_1F9D3_1F3FF };
    ("older_adult") => { &twemoji_assets::png::codes::U_1F9D3 };
    ("woman_bearded_tone1") => { &twemoji_assets::png::codes::U_1F9D4_1F3FB_200D_2640_FE0F };
    ("man_bearded_tone1") => { &twemoji_assets::png::codes::U_1F9D4_1F3FB_200D_2642_FE0F };
    ("person_bearded_tone1") => { &twemoji_assets::png::codes::U_1F9D4_1F3FB };
    ("woman_bearded_tone2") => { &twemoji_assets::png::codes::U_1F9D4_1F3FC_200D_2640_FE0F };
    ("man_bearded_tone2") => { &twemoji_assets::png::codes::U_1F9D4_1F3FC_200D_2642_FE0F };
    ("person_bearded_tone2") => { &twemoji_assets::png::codes::U_1F9D4_1F3FC };
    ("woman_bearded_tone3") => { &twemoji_assets::png::codes::U_1F9D4_1F3FD_200D_2640_FE0F };
    ("man_bearded_tone3") => { &twemoji_assets::png::codes::U_1F9D4_1F3FD_200D_2642_FE0F };
    ("person_bearded_tone3") => { &twemoji_assets::png::codes::U_1F9D4_1F3FD };
    ("woman_bearded_tone4") => { &twemoji_assets::png::codes::U_1F9D4_1F3FE_200D_2640_FE0F };
    ("man_bearded_tone4") => { &twemoji_assets::png::codes::U_1F9D4_1F3FE_200D_2642_FE0F };
    ("person_bearded_tone4") => { &twemoji_assets::png::codes::U_1F9D4_1F3FE };
    ("woman_bearded_tone5") => { &twemoji_assets::png::codes::U_1F9D4_1F3FF_200D_2640_FE0F };
    ("man_bearded_tone5") => { &twemoji_assets::png::codes::U_1F9D4_1F3FF_200D_2642_FE0F };
    ("person_bearded_tone5") => { &twemoji_assets::png::codes::U_1F9D4_1F3FF };
    ("woman_bearded") => { &twemoji_assets::png::codes::U_1F9D4_200D_2640_FE0F };
    ("man_bearded") => { &twemoji_assets::png::codes::U_1F9D4_200D_2642_FE0F };
    ("person_bearded") => { &twemoji_assets::png::codes::U_1F9D4 };
    ("woman_with_headscarf_tone1") => { &twemoji_assets::png::codes::U_1F9D5_1F3FB };
    ("woman_with_headscarf_tone2") => { &twemoji_assets::png::codes::U_1F9D5_1F3FC };
    ("woman_with_headscarf_tone3") => { &twemoji_assets::png::codes::U_1F9D5_1F3FD };
    ("woman_with_headscarf_tone4") => { &twemoji_assets::png::codes::U_1F9D5_1F3FE };
    ("woman_with_headscarf_tone5") => { &twemoji_assets::png::codes::U_1F9D5_1F3FF };
    ("woman_with_headscarf") => { &twemoji_assets::png::codes::U_1F9D5 };
    ("woman_in_steamy_room_tone1") => { &twemoji_assets::png::codes::U_1F9D6_1F3FB_200D_2640_FE0F };
    ("man_in_steamy_room_tone1") => { &twemoji_assets::png::codes::U_1F9D6_1F3FB_200D_2642_FE0F };
    ("person_in_steamy_room_tone1") => { &twemoji_assets::png::codes::U_1F9D6_1F3FB };
    ("woman_in_steamy_room_tone2") => { &twemoji_assets::png::codes::U_1F9D6_1F3FC_200D_2640_FE0F };
    ("man_in_steamy_room_tone2") => { &twemoji_assets::png::codes::U_1F9D6_1F3FC_200D_2642_FE0F };
    ("person_in_steamy_room_tone2") => { &twemoji_assets::png::codes::U_1F9D6_1F3FC };
    ("woman_in_steamy_room_tone3") => { &twemoji_assets::png::codes::U_1F9D6_1F3FD_200D_2640_FE0F };
    ("man_in_steamy_room_tone3") => { &twemoji_assets::png::codes::U_1F9D6_1F3FD_200D_2642_FE0F };
    ("person_in_steamy_room_tone3") => { &twemoji_assets::png::codes::U_1F9D6_1F3FD };
    ("woman_in_steamy_room_tone4") => { &twemoji_assets::png::codes::U_1F9D6_1F3FE_200D_2640_FE0F };
    ("man_in_steamy_room_tone4") => { &twemoji_assets::png::codes::U_1F9D6_1F3FE_200D_2642_FE0F };
    ("person_in_steamy_room_tone4") => { &twemoji_assets::png::codes::U_1F9D6_1F3FE };
    ("woman_in_steamy_room_tone5") => { &twemoji_assets::png::codes::U_1F9D6_1F3FF_200D_2640_FE0F };
    ("man_in_steamy_room_tone5") => { &twemoji_assets::png::codes::U_1F9D6_1F3FF_200D_2642_FE0F };
    ("person_in_steamy_room_tone5") => { &twemoji_assets::png::codes::U_1F9D6_1F3FF };
    ("woman_in_steamy_room") => { &twemoji_assets::png::codes::U_1F9D6_200D_2640_FE0F };
    ("man_in_steamy_room") => { &twemoji_assets::png::codes::U_1F9D6_200D_2642_FE0F };
    ("person_in_steamy_room") => { &twemoji_assets::png::codes::U_1F9D6 };
    ("woman_climbing_tone1") => { &twemoji_assets::png::codes::U_1F9D7_1F3FB_200D_2640_FE0F };
    ("man_climbing_tone1") => { &twemoji_assets::png::codes::U_1F9D7_1F3FB_200D_2642_FE0F };
    ("climbing_tone1") => { &twemoji_assets::png::codes::U_1F9D7_1F3FB };
    ("person_climbing_tone1") => { &twemoji_assets::png::codes::U_1F9D7_1F3FB };
    ("woman_climbing_tone2") => { &twemoji_assets::png::codes::U_1F9D7_1F3FC_200D_2640_FE0F };
    ("man_climbing_tone2") => { &twemoji_assets::png::codes::U_1F9D7_1F3FC_200D_2642_FE0F };
    ("climbing_tone2") => { &twemoji_assets::png::codes::U_1F9D7_1F3FC };
    ("person_climbing_tone2") => { &twemoji_assets::png::codes::U_1F9D7_1F3FC };
    ("woman_climbing_tone3") => { &twemoji_assets::png::codes::U_1F9D7_1F3FD_200D_2640_FE0F };
    ("man_climbing_tone3") => { &twemoji_assets::png::codes::U_1F9D7_1F3FD_200D_2642_FE0F };
    ("climbing_tone3") => { &twemoji_assets::png::codes::U_1F9D7_1F3FD };
    ("person_climbing_tone3") => { &twemoji_assets::png::codes::U_1F9D7_1F3FD };
    ("woman_climbing_tone4") => { &twemoji_assets::png::codes::U_1F9D7_1F3FE_200D_2640_FE0F };
    ("man_climbing_tone4") => { &twemoji_assets::png::codes::U_1F9D7_1F3FE_200D_2642_FE0F };
    ("climbing_tone4") => { &twemoji_assets::png::codes::U_1F9D7_1F3FE };
    ("person_climbing_tone4") => { &twemoji_assets::png::codes::U_1F9D7_1F3FE };
    ("woman_climbing_tone5") => { &twemoji_assets::png::codes::U_1F9D7_1F3FF_200D_2640_FE0F };
    ("man_climbing_tone5") => { &twemoji_assets::png::codes::U_1F9D7_1F3FF_200D_2642_FE0F };
    ("climbing_tone5") => { &twemoji_assets::png::codes::U_1F9D7_1F3FF };
    ("person_climbing_tone5") => { &twemoji_assets::png::codes::U_1F9D7_1F3FF };
    ("woman_climbing") => { &twemoji_assets::png::codes::U_1F9D7_200D_2640_FE0F };
    ("man_climbing") => { &twemoji_assets::png::codes::U_1F9D7_200D_2642_FE0F };
    ("climbing") => { &twemoji_assets::png::codes::U_1F9D7 };
    ("person_climbing") => { &twemoji_assets::png::codes::U_1F9D7 };
    ("woman_in_lotus_position_tone1") => { &twemoji_assets::png::codes::U_1F9D8_1F3FB_200D_2640_FE0F };
    ("man_in_lotus_position_tone1") => { &twemoji_assets::png::codes::U_1F9D8_1F3FB_200D_2642_FE0F };
    ("person_in_lotus_position_tone1") => { &twemoji_assets::png::codes::U_1F9D8_1F3FB };
    ("woman_in_lotus_position_tone2") => { &twemoji_assets::png::codes::U_1F9D8_1F3FC_200D_2640_FE0F };
    ("man_in_lotus_position_tone2") => { &twemoji_assets::png::codes::U_1F9D8_1F3FC_200D_2642_FE0F };
    ("person_in_lotus_position_tone2") => { &twemoji_assets::png::codes::U_1F9D8_1F3FC };
    ("woman_in_lotus_position_tone3") => { &twemoji_assets::png::codes::U_1F9D8_1F3FD_200D_2640_FE0F };
    ("man_in_lotus_position_tone3") => { &twemoji_assets::png::codes::U_1F9D8_1F3FD_200D_2642_FE0F };
    ("person_in_lotus_position_tone3") => { &twemoji_assets::png::codes::U_1F9D8_1F3FD };
    ("woman_in_lotus_position_tone4") => { &twemoji_assets::png::codes::U_1F9D8_1F3FE_200D_2640_FE0F };
    ("man_in_lotus_position_tone4") => { &twemoji_assets::png::codes::U_1F9D8_1F3FE_200D_2642_FE0F };
    ("person_in_lotus_position_tone4") => { &twemoji_assets::png::codes::U_1F9D8_1F3FE };
    ("woman_in_lotus_position_tone5") => { &twemoji_assets::png::codes::U_1F9D8_1F3FF_200D_2640_FE0F };
    ("man_in_lotus_position_tone5") => { &twemoji_assets::png::codes::U_1F9D8_1F3FF_200D_2642_FE0F };
    ("person_in_lotus_position_tone5") => { &twemoji_assets::png::codes::U_1F9D8_1F3FF };
    ("woman_in_lotus_position") => { &twemoji_assets::png::codes::U_1F9D8_200D_2640_FE0F };
    ("man_in_lotus_position") => { &twemoji_assets::png::codes::U_1F9D8_200D_2642_FE0F };
    ("person_in_lotus_position") => { &twemoji_assets::png::codes::U_1F9D8 };
    ("woman_mage_tone1") => { &twemoji_assets::png::codes::U_1F9D9_1F3FB_200D_2640_FE0F };
    ("man_mage_tone1") => { &twemoji_assets::png::codes::U_1F9D9_1F3FB_200D_2642_FE0F };
    ("mage_tone1") => { &twemoji_assets::png::codes::U_1F9D9_1F3FB };
    ("woman_mage_tone2") => { &twemoji_assets::png::codes::U_1F9D9_1F3FC_200D_2640_FE0F };
    ("man_mage_tone2") => { &twemoji_assets::png::codes::U_1F9D9_1F3FC_200D_2642_FE0F };
    ("mage_tone2") => { &twemoji_assets::png::codes::U_1F9D9_1F3FC };
    ("woman_mage_tone3") => { &twemoji_assets::png::codes::U_1F9D9_1F3FD_200D_2640_FE0F };
    ("man_mage_tone3") => { &twemoji_assets::png::codes::U_1F9D9_1F3FD_200D_2642_FE0F };
    ("mage_tone3") => { &twemoji_assets::png::codes::U_1F9D9_1F3FD };
    ("woman_mage_tone4") => { &twemoji_assets::png::codes::U_1F9D9_1F3FE_200D_2640_FE0F };
    ("man_mage_tone4") => { &twemoji_assets::png::codes::U_1F9D9_1F3FE_200D_2642_FE0F };
    ("mage_tone4") => { &twemoji_assets::png::codes::U_1F9D9_1F3FE };
    ("woman_mage_tone5") => { &twemoji_assets::png::codes::U_1F9D9_1F3FF_200D_2640_FE0F };
    ("man_mage_tone5") => { &twemoji_assets::png::codes::U_1F9D9_1F3FF_200D_2642_FE0F };
    ("mage_tone5") => { &twemoji_assets::png::codes::U_1F9D9_1F3FF };
    ("woman_mage") => { &twemoji_assets::png::codes::U_1F9D9_200D_2640_FE0F };
    ("man_mage") => { &twemoji_assets::png::codes::U_1F9D9_200D_2642_FE0F };
    ("mage") => { &twemoji_assets::png::codes::U_1F9D9 };
    ("woman_fairy_tone1") => { &twemoji_assets::png::codes::U_1F9DA_1F3FB_200D_2640_FE0F };
    ("man_fairy_tone1") => { &twemoji_assets::png::codes::U_1F9DA_1F3FB_200D_2642_FE0F };
    ("fairy_tone1") => { &twemoji_assets::png::codes::U_1F9DA_1F3FB };
    ("woman_fairy_tone2") => { &twemoji_assets::png::codes::U_1F9DA_1F3FC_200D_2640_FE0F };
    ("man_fairy_tone2") => { &twemoji_assets::png::codes::U_1F9DA_1F3FC_200D_2642_FE0F };
    ("fairy_tone2") => { &twemoji_assets::png::codes::U_1F9DA_1F3FC };
    ("woman_fairy_tone3") => { &twemoji_assets::png::codes::U_1F9DA_1F3FD_200D_2640_FE0F };
    ("man_fairy_tone3") => { &twemoji_assets::png::codes::U_1F9DA_1F3FD_200D_2642_FE0F };
    ("fairy_tone3") => { &twemoji_assets::png::codes::U_1F9DA_1F3FD };
    ("woman_fairy_tone4") => { &twemoji_assets::png::codes::U_1F9DA_1F3FE_200D_2640_FE0F };
    ("man_fairy_tone4") => { &twemoji_assets::png::codes::U_1F9DA_1F3FE_200D_2642_FE0F };
    ("fairy_tone4") => { &twemoji_assets::png::codes::U_1F9DA_1F3FE };
    ("woman_fairy_tone5") => { &twemoji_assets::png::codes::U_1F9DA_1F3FF_200D_2640_FE0F };
    ("man_fairy_tone5") => { &twemoji_assets::png::codes::U_1F9DA_1F3FF_200D_2642_FE0F };
    ("fairy_tone5") => { &twemoji_assets::png::codes::U_1F9DA_1F3FF };
    ("woman_fairy") => { &twemoji_assets::png::codes::U_1F9DA_200D_2640_FE0F };
    ("man_fairy") => { &twemoji_assets::png::codes::U_1F9DA_200D_2642_FE0F };
    ("fairy") => { &twemoji_assets::png::codes::U_1F9DA };
    ("woman_vampire_tone1") => { &twemoji_assets::png::codes::U_1F9DB_1F3FB_200D_2640_FE0F };
    ("man_vampire_tone1") => { &twemoji_assets::png::codes::U_1F9DB_1F3FB_200D_2642_FE0F };
    ("vampire_tone1") => { &twemoji_assets::png::codes::U_1F9DB_1F3FB };
    ("woman_vampire_tone2") => { &twemoji_assets::png::codes::U_1F9DB_1F3FC_200D_2640_FE0F };
    ("man_vampire_tone2") => { &twemoji_assets::png::codes::U_1F9DB_1F3FC_200D_2642_FE0F };
    ("vampire_tone2") => { &twemoji_assets::png::codes::U_1F9DB_1F3FC };
    ("woman_vampire_tone3") => { &twemoji_assets::png::codes::U_1F9DB_1F3FD_200D_2640_FE0F };
    ("man_vampire_tone3") => { &twemoji_assets::png::codes::U_1F9DB_1F3FD_200D_2642_FE0F };
    ("vampire_tone3") => { &twemoji_assets::png::codes::U_1F9DB_1F3FD };
    ("woman_vampire_tone4") => { &twemoji_assets::png::codes::U_1F9DB_1F3FE_200D_2640_FE0F };
    ("man_vampire_tone4") => { &twemoji_assets::png::codes::U_1F9DB_1F3FE_200D_2642_FE0F };
    ("vampire_tone4") => { &twemoji_assets::png::codes::U_1F9DB_1F3FE };
    ("woman_vampire_tone5") => { &twemoji_assets::png::codes::U_1F9DB_1F3FF_200D_2640_FE0F };
    ("man_vampire_tone5") => { &twemoji_assets::png::codes::U_1F9DB_1F3FF_200D_2642_FE0F };
    ("vampire_tone5") => { &twemoji_assets::png::codes::U_1F9DB_1F3FF };
    ("woman_vampire") => { &twemoji_assets::png::codes::U_1F9DB_200D_2640_FE0F };
    ("man_vampire") => { &twemoji_assets::png::codes::U_1F9DB_200D_2642_FE0F };
    ("vampire") => { &twemoji_assets::png::codes::U_1F9DB };
    ("mermaid_tone1") => { &twemoji_assets::png::codes::U_1F9DC_1F3FB_200D_2640_FE0F };
    ("merman_tone1") => { &twemoji_assets::png::codes::U_1F9DC_1F3FB_200D_2642_FE0F };
    ("merperson_tone1") => { &twemoji_assets::png::codes::U_1F9DC_1F3FB };
    ("mermaid_tone2") => { &twemoji_assets::png::codes::U_1F9DC_1F3FC_200D_2640_FE0F };
    ("merman_tone2") => { &twemoji_assets::png::codes::U_1F9DC_1F3FC_200D_2642_FE0F };
    ("merperson_tone2") => { &twemoji_assets::png::codes::U_1F9DC_1F3FC };
    ("mermaid_tone3") => { &twemoji_assets::png::codes::U_1F9DC_1F3FD_200D_2640_FE0F };
    ("merman_tone3") => { &twemoji_assets::png::codes::U_1F9DC_1F3FD_200D_2642_FE0F };
    ("merperson_tone3") => { &twemoji_assets::png::codes::U_1F9DC_1F3FD };
    ("mermaid_tone4") => { &twemoji_assets::png::codes::U_1F9DC_1F3FE_200D_2640_FE0F };
    ("merman_tone4") => { &twemoji_assets::png::codes::U_1F9DC_1F3FE_200D_2642_FE0F };
    ("merperson_tone4") => { &twemoji_assets::png::codes::U_1F9DC_1F3FE };
    ("mermaid_tone5") => { &twemoji_assets::png::codes::U_1F9DC_1F3FF_200D_2640_FE0F };
    ("merman_tone5") => { &twemoji_assets::png::codes::U_1F9DC_1F3FF_200D_2642_FE0F };
    ("merperson_tone5") => { &twemoji_assets::png::codes::U_1F9DC_1F3FF };
    ("mermaid") => { &twemoji_assets::png::codes::U_1F9DC_200D_2640_FE0F };
    ("merman") => { &twemoji_assets::png::codes::U_1F9DC_200D_2642_FE0F };
    ("merperson") => { &twemoji_assets::png::codes::U_1F9DC };
    ("woman_elf_tone1") => { &twemoji_assets::png::codes::U_1F9DD_1F3FB_200D_2640_FE0F };
    ("man_elf_tone1") => { &twemoji_assets::png::codes::U_1F9DD_1F3FB_200D_2642_FE0F };
    ("elf_tone1") => { &twemoji_assets::png::codes::U_1F9DD_1F3FB };
    ("woman_elf_tone2") => { &twemoji_assets::png::codes::U_1F9DD_1F3FC_200D_2640_FE0F };
    ("man_elf_tone2") => { &twemoji_assets::png::codes::U_1F9DD_1F3FC_200D_2642_FE0F };
    ("elf_tone2") => { &twemoji_assets::png::codes::U_1F9DD_1F3FC };
    ("woman_elf_tone3") => { &twemoji_assets::png::codes::U_1F9DD_1F3FD_200D_2640_FE0F };
    ("man_elf_tone3") => { &twemoji_assets::png::codes::U_1F9DD_1F3FD_200D_2642_FE0F };
    ("elf_tone3") => { &twemoji_assets::png::codes::U_1F9DD_1F3FD };
    ("woman_elf_tone4") => { &twemoji_assets::png::codes::U_1F9DD_1F3FE_200D_2640_FE0F };
    ("man_elf_tone4") => { &twemoji_assets::png::codes::U_1F9DD_1F3FE_200D_2642_FE0F };
    ("elf_tone4") => { &twemoji_assets::png::codes::U_1F9DD_1F3FE };
    ("woman_elf_tone5") => { &twemoji_assets::png::codes::U_1F9DD_1F3FF_200D_2640_FE0F };
    ("man_elf_tone5") => { &twemoji_assets::png::codes::U_1F9DD_1F3FF_200D_2642_FE0F };
    ("elf_tone5") => { &twemoji_assets::png::codes::U_1F9DD_1F3FF };
    ("woman_elf") => { &twemoji_assets::png::codes::U_1F9DD_200D_2640_FE0F };
    ("man_elf") => { &twemoji_assets::png::codes::U_1F9DD_200D_2642_FE0F };
    ("elf") => { &twemoji_assets::png::codes::U_1F9DD };
    ("woman_genie") => { &twemoji_assets::png::codes::U_1F9DE_200D_2640_FE0F };
    ("man_genie") => { &twemoji_assets::png::codes::U_1F9DE_200D_2642_FE0F };
    ("genie") => { &twemoji_assets::png::codes::U_1F9DE };
    ("woman_zombie") => { &twemoji_assets::png::codes::U_1F9DF_200D_2640_FE0F };
    ("man_zombie") => { &twemoji_assets::png::codes::U_1F9DF_200D_2642_FE0F };
    ("zombie") => { &twemoji_assets::png::codes::U_1F9DF };
    ("brain") => { &twemoji_assets::png::codes::U_1F9E0 };
    ("orange_heart") => { &twemoji_assets::png::codes::U_1F9E1 };
    ("billed_cap") => { &twemoji_assets::png::codes::U_1F9E2 };
    ("scarf") => { &twemoji_assets::png::codes::U_1F9E3 };
    ("gloves") => { &twemoji_assets::png::codes::U_1F9E4 };
    ("coat") => { &twemoji_assets::png::codes::U_1F9E5 };
    ("socks") => { &twemoji_assets::png::codes::U_1F9E6 };
    ("red_envelope") => { &twemoji_assets::png::codes::U_1F9E7 };
    ("firecracker") => { &twemoji_assets::png::codes::U_1F9E8 };
    ("jigsaw") => { &twemoji_assets::png::codes::U_1F9E9 };
    ("puzzle_piece") => { &twemoji_assets::png::codes::U_1F9E9 };
    ("test_tube") => { &twemoji_assets::png::codes::U_1F9EA };
    ("petri_dish") => { &twemoji_assets::png::codes::U_1F9EB };
    ("dna") => { &twemoji_assets::png::codes::U_1F9EC };
    ("double_helix") => { &twemoji_assets::png::codes::U_1F9EC };
    ("compass") => { &twemoji_assets::png::codes::U_1F9ED };
    ("abacus") => { &twemoji_assets::png::codes::U_1F9EE };
    ("fire_extinguisher") => { &twemoji_assets::png::codes::U_1F9EF };
    ("toolbox") => { &twemoji_assets::png::codes::U_1F9F0 };
    ("bricks") => { &twemoji_assets::png::codes::U_1F9F1 };
    ("magnet") => { &twemoji_assets::png::codes::U_1F9F2 };
    ("luggage") => { &twemoji_assets::png::codes::U_1F9F3 };
    ("lotion_bottle") => { &twemoji_assets::png::codes::U_1F9F4 };
    ("thread") => { &twemoji_assets::png::codes::U_1F9F5 };
    ("yarn") => { &twemoji_assets::png::codes::U_1F9F6 };
    ("safety_pin") => { &twemoji_assets::png::codes::U_1F9F7 };
    ("teddy_bear") => { &twemoji_assets::png::codes::U_1F9F8 };
    ("broom") => { &twemoji_assets::png::codes::U_1F9F9 };
    ("basket") => { &twemoji_assets::png::codes::U_1F9FA };
    ("roll_of_paper") => { &twemoji_assets::png::codes::U_1F9FB };
    ("toilet_paper") => { &twemoji_assets::png::codes::U_1F9FB };
    ("soap") => { &twemoji_assets::png::codes::U_1F9FC };
    ("sponge") => { &twemoji_assets::png::codes::U_1F9FD };
    ("receipt") => { &twemoji_assets::png::codes::U_1F9FE };
    ("nazar_amulet") => { &twemoji_assets::png::codes::U_1F9FF };
    ("ballet_shoes") => { &twemoji_assets::png::codes::U_1FA70 };
    ("one_piece_swimsuit") => { &twemoji_assets::png::codes::U_1FA71 };
    ("briefs") => { &twemoji_assets::png::codes::U_1FA72 };
    ("shorts") => { &twemoji_assets::png::codes::U_1FA73 };
    ("thong_sandal") => { &twemoji_assets::png::codes::U_1FA74 };
    ("drop_of_blood") => { &twemoji_assets::png::codes::U_1FA78 };
    ("adhesive_bandage") => { &twemoji_assets::png::codes::U_1FA79 };
    ("bandaid") => { &twemoji_assets::png::codes::U_1FA79 };
    ("stethoscope") => { &twemoji_assets::png::codes::U_1FA7A };
    ("x-ray") => { &twemoji_assets::png::codes::U_1FA7B };
    ("xray") => { &twemoji_assets::png::codes::U_1FA7B };
    ("crutch") => { &twemoji_assets::png::codes::U_1FA7C };
    ("yo_yo") => { &twemoji_assets::png::codes::U_1FA80 };
    ("kite") => { &twemoji_assets::png::codes::U_1FA81 };
    ("parachute") => { &twemoji_assets::png::codes::U_1FA82 };
    ("boomerang") => { &twemoji_assets::png::codes::U_1FA83 };
    ("magic_wand") => { &twemoji_assets::png::codes::U_1FA84 };
    ("pinata") => { &twemoji_assets::png::codes::U_1FA85 };
    ("nesting_dolls") => { &twemoji_assets::png::codes::U_1FA86 };
    ("ringed_planet") => { &twemoji_assets::png::codes::U_1FA90 };
    ("saturn") => { &twemoji_assets::png::codes::U_1FA90 };
    ("chair") => { &twemoji_assets::png::codes::U_1FA91 };
    ("razor") => { &twemoji_assets::png::codes::U_1FA92 };
    ("axe") => { &twemoji_assets::png::codes::U_1FA93 };
    ("diya_lamp") => { &twemoji_assets::png::codes::U_1FA94 };
    ("banjo") => { &twemoji_assets::png::codes::U_1FA95 };
    ("military_helmet") => { &twemoji_assets::png::codes::U_1FA96 };
    ("accordion") => { &twemoji_assets::png::codes::U_1FA97 };
    ("long_drum") => { &twemoji_assets::png::codes::U_1FA98 };
    ("coin") => { &twemoji_assets::png::codes::U_1FA99 };
    ("carpentry_saw") => { &twemoji_assets::png::codes::U_1FA9A };
    ("screwdriver") => { &twemoji_assets::png::codes::U_1FA9B };
    ("ladder") => { &twemoji_assets::png::codes::U_1FA9C };
    ("hook") => { &twemoji_assets::png::codes::U_1FA9D };
    ("mirror") => { &twemoji_assets::png::codes::U_1FA9E };
    ("window") => { &twemoji_assets::png::codes::U_1FA9F };
    ("plunger") => { &twemoji_assets::png::codes::U_1FAA0 };
    ("sewing_needle") => { &twemoji_assets::png::codes::U_1FAA1 };
    ("knot") => { &twemoji_assets::png::codes::U_1FAA2 };
    ("bucket") => { &twemoji_assets::png::codes::U_1FAA3 };
    ("mouse_trap") => { &twemoji_assets::png::codes::U_1FAA4 };
    ("toothbrush") => { &twemoji_assets::png::codes::U_1FAA5 };
    ("headstone") => { &twemoji_assets::png::codes::U_1FAA6 };
    ("placard") => { &twemoji_assets::png::codes::U_1FAA7 };
    ("rock") => { &twemoji_assets::png::codes::U_1FAA8 };
    ("disco") => { &twemoji_assets::png::codes::U_1FAA9 };
    ("disco_ball") => { &twemoji_assets::png::codes::U_1FAA9 };
    ("mirror_ball") => { &twemoji_assets::png::codes::U_1FAA9 };
    ("id_card") => { &twemoji_assets::png::codes::U_1FAAA };
    ("low_battery") => { &twemoji_assets::png::codes::U_1FAAB };
    ("hamsa") => { &twemoji_assets::png::codes::U_1FAAC };
    ("fly") => { &twemoji_assets::png::codes::U_1FAB0 };
    ("worm") => { &twemoji_assets::png::codes::U_1FAB1 };
    ("beetle") => { &twemoji_assets::png::codes::U_1FAB2 };
    ("cockroach") => { &twemoji_assets::png::codes::U_1FAB3 };
    ("potted_plant") => { &twemoji_assets::png::codes::U_1FAB4 };
    ("wood") => { &twemoji_assets::png::codes::U_1FAB5 };
    ("feather") => { &twemoji_assets::png::codes::U_1FAB6 };
    ("lotus") => { &twemoji_assets::png::codes::U_1FAB7 };
    ("coral") => { &twemoji_assets::png::codes::U_1FAB8 };
    ("empty_nest") => { &twemoji_assets::png::codes::U_1FAB9 };
    ("nest") => { &twemoji_assets::png::codes::U_1FAB9 };
    ("nest_with_eggs") => { &twemoji_assets::png::codes::U_1FABA };
    ("anatomical_heart") => { &twemoji_assets::png::codes::U_1FAC0 };
    ("lungs") => { &twemoji_assets::png::codes::U_1FAC1 };
    ("people_hugging") => { &twemoji_assets::png::codes::U_1FAC2 };
    ("pregnant_man_tone1") => { &twemoji_assets::png::codes::U_1FAC3_1F3FB };
    ("pregnant_man_tone2") => { &twemoji_assets::png::codes::U_1FAC3_1F3FC };
    ("pregnant_man_tone3") => { &twemoji_assets::png::codes::U_1FAC3_1F3FD };
    ("pregnant_man_tone4") => { &twemoji_assets::png::codes::U_1FAC3_1F3FE };
    ("pregnant_man_tone5") => { &twemoji_assets::png::codes::U_1FAC3_1F3FF };
    ("pregnant_man") => { &twemoji_assets::png::codes::U_1FAC3 };
    ("pregnant_person_tone1") => { &twemoji_assets::png::codes::U_1FAC4_1F3FB };
    ("pregnant_person_tone2") => { &twemoji_assets::png::codes::U_1FAC4_1F3FC };
    ("pregnant_person_tone3") => { &twemoji_assets::png::codes::U_1FAC4_1F3FD };
    ("pregnant_person_tone4") => { &twemoji_assets::png::codes::U_1FAC4_1F3FE };
    ("pregnant_person_tone5") => { &twemoji_assets::png::codes::U_1FAC4_1F3FF };
    ("pregnant_person") => { &twemoji_assets::png::codes::U_1FAC4 };
    ("person_with_crown_tone1") => { &twemoji_assets::png::codes::U_1FAC5_1F3FB };
    ("royalty_tone1") => { &twemoji_assets::png::codes::U_1FAC5_1F3FB };
    ("person_with_crown_tone2") => { &twemoji_assets::png::codes::U_1FAC5_1F3FC };
    ("royalty_tone2") => { &twemoji_assets::png::codes::U_1FAC5_1F3FC };
    ("person_with_crown_tone3") => { &twemoji_assets::png::codes::U_1FAC5_1F3FD };
    ("royalty_tone3") => { &twemoji_assets::png::codes::U_1FAC5_1F3FD };
    ("person_with_crown_tone4") => { &twemoji_assets::png::codes::U_1FAC5_1F3FE };
    ("royalty_tone4") => { &twemoji_assets::png::codes::U_1FAC5_1F3FE };
    ("person_with_crown_tone5") => { &twemoji_assets::png::codes::U_1FAC5_1F3FF };
    ("royalty_tone5") => { &twemoji_assets::png::codes::U_1FAC5_1F3FF };
    ("person_with_crown") => { &twemoji_assets::png::codes::U_1FAC5 };
    ("royalty") => { &twemoji_assets::png::codes::U_1FAC5 };
    ("blueberries") => { &twemoji_assets::png::codes::U_1FAD0 };
    ("bell_pepper") => { &twemoji_assets::png::codes::U_1FAD1 };
    ("olive") => { &twemoji_assets::png::codes::U_1FAD2 };
    ("flatbread") => { &twemoji_assets::png::codes::U_1FAD3 };
    ("tamale") => { &twemoji_assets::png::codes::U_1FAD4 };
    ("fondue") => { &twemoji_assets::png::codes::U_1FAD5 };
    ("teapot") => { &twemoji_assets::png::codes::U_1FAD6 };
    ("pour") => { &twemoji_assets::png::codes::U_1FAD7 };
    ("pouring_liquid") => { &twemoji_assets::png::codes::U_1FAD7 };
    ("beans") => { &twemoji_assets::png::codes::U_1FAD8 };
    ("jar") => { &twemoji_assets::png::codes::U_1FAD9 };
    ("melt") => { &twemoji_assets::png::codes::U_1FAE0 };
    ("melting_face") => { &twemoji_assets::png::codes::U_1FAE0 };
    ("salute") => { &twemoji_assets::png::codes::U_1FAE1 };
    ("saluting_face") => { &twemoji_assets::png::codes::U_1FAE1 };
    ("face_with_open_eyes_hand_over_mouth") => { &twemoji_assets::png::codes::U_1FAE2 };
    ("gasp") => { &twemoji_assets::png::codes::U_1FAE2 };
    ("face_with_peeking_eye") => { &twemoji_assets::png::codes::U_1FAE3 };
    ("peek") => { &twemoji_assets::png::codes::U_1FAE3 };
    ("face_with_diagonal_mouth") => { &twemoji_assets::png::codes::U_1FAE4 };
    ("dotted_line_face") => { &twemoji_assets::png::codes::U_1FAE5 };
    ("biting_lip") => { &twemoji_assets::png::codes::U_1FAE6 };
    ("bubbles") => { &twemoji_assets::png::codes::U_1FAE7 };
    ("hand_with_index_finger_and_thumb_crossed_tone1") => { &twemoji_assets::png::codes::U_1FAF0_1F3FB };
    ("hand_with_index_finger_and_thumb_crossed_tone2") => { &twemoji_assets::png::codes::U_1FAF0_1F3FC };
    ("hand_with_index_finger_and_thumb_crossed_tone3") => { &twemoji_assets::png::codes::U_1FAF0_1F3FD };
    ("hand_with_index_finger_and_thumb_crossed_tone4") => { &twemoji_assets::png::codes::U_1FAF0_1F3FE };
    ("hand_with_index_finger_and_thumb_crossed_tone5") => { &twemoji_assets::png::codes::U_1FAF0_1F3FF };
    ("hand_with_index_finger_and_thumb_crossed") => { &twemoji_assets::png::codes::U_1FAF0 };
    ("handshake_tone1-2") => { &twemoji_assets::png::codes::U_1FAF1_1F3FB_200D_1FAF2_1F3FC };
    ("handshake_tone1-3") => { &twemoji_assets::png::codes::U_1FAF1_1F3FB_200D_1FAF2_1F3FD };
    ("handshake_tone1-4") => { &twemoji_assets::png::codes::U_1FAF1_1F3FB_200D_1FAF2_1F3FE };
    ("handshake_tone1-5") => { &twemoji_assets::png::codes::U_1FAF1_1F3FB_200D_1FAF2_1F3FF };
    ("rightwards_hand_tone1") => { &twemoji_assets::png::codes::U_1FAF1_1F3FB };
    ("handshake_tone2-1") => { &twemoji_assets::png::codes::U_1FAF1_1F3FC_200D_1FAF2_1F3FB };
    ("handshake_tone2-3") => { &twemoji_assets::png::codes::U_1FAF1_1F3FC_200D_1FAF2_1F3FD };
    ("handshake_tone2-4") => { &twemoji_assets::png::codes::U_1FAF1_1F3FC_200D_1FAF2_1F3FE };
    ("handshake_tone2-5") => { &twemoji_assets::png::codes::U_1FAF1_1F3FC_200D_1FAF2_1F3FF };
    ("rightwards_hand_tone2") => { &twemoji_assets::png::codes::U_1FAF1_1F3FC };
    ("handshake_tone3-1") => { &twemoji_assets::png::codes::U_1FAF1_1F3FD_200D_1FAF2_1F3FB };
    ("handshake_tone3-2") => { &twemoji_assets::png::codes::U_1FAF1_1F3FD_200D_1FAF2_1F3FC };
    ("handshake_tone3-4") => { &twemoji_assets::png::codes::U_1FAF1_1F3FD_200D_1FAF2_1F3FE };
    ("handshake_tone3-5") => { &twemoji_assets::png::codes::U_1FAF1_1F3FD_200D_1FAF2_1F3FF };
    ("rightwards_hand_tone3") => { &twemoji_assets::png::codes::U_1FAF1_1F3FD };
    ("handshake_tone4-1") => { &twemoji_assets::png::codes::U_1FAF1_1F3FE_200D_1FAF2_1F3FB };
    ("handshake_tone4-2") => { &twemoji_assets::png::codes::U_1FAF1_1F3FE_200D_1FAF2_1F3FC };
    ("handshake_tone4-3") => { &twemoji_assets::png::codes::U_1FAF1_1F3FE_200D_1FAF2_1F3FD };
    ("handshake_tone4-5") => { &twemoji_assets::png::codes::U_1FAF1_1F3FE_200D_1FAF2_1F3FF };
    ("rightwards_hand_tone4") => { &twemoji_assets::png::codes::U_1FAF1_1F3FE };
    ("handshake_tone5-1") => { &twemoji_assets::png::codes::U_1FAF1_1F3FF_200D_1FAF2_1F3FB };
    ("handshake_tone5-2") => { &twemoji_assets::png::codes::U_1FAF1_1F3FF_200D_1FAF2_1F3FC };
    ("handshake_tone5-3") => { &twemoji_assets::png::codes::U_1FAF1_1F3FF_200D_1FAF2_1F3FD };
    ("handshake_tone5-4") => { &twemoji_assets::png::codes::U_1FAF1_1F3FF_200D_1FAF2_1F3FE };
    ("rightwards_hand_tone5") => { &twemoji_assets::png::codes::U_1FAF1_1F3FF };
    ("rightwards_hand") => { &twemoji_assets::png::codes::U_1FAF1 };
    ("leftwards_hand_tone1") => { &twemoji_assets::png::codes::U_1FAF2_1F3FB };
    ("leftwards_hand_tone2") => { &twemoji_assets::png::codes::U_1FAF2_1F3FC };
    ("leftwards_hand_tone3") => { &twemoji_assets::png::codes::U_1FAF2_1F3FD };
    ("leftwards_hand_tone4") => { &twemoji_assets::png::codes::U_1FAF2_1F3FE };
    ("leftwards_hand_tone5") => { &twemoji_assets::png::codes::U_1FAF2_1F3FF };
    ("leftwards_hand") => { &twemoji_assets::png::codes::U_1FAF2 };
    ("palm_down_tone1") => { &twemoji_assets::png::codes::U_1FAF3_1F3FB };
    ("palm_down_tone2") => { &twemoji_assets::png::codes::U_1FAF3_1F3FC };
    ("palm_down_tone3") => { &twemoji_assets::png::codes::U_1FAF3_1F3FD };
    ("palm_down_tone4") => { &twemoji_assets::png::codes::U_1FAF3_1F3FE };
    ("palm_down_tone5") => { &twemoji_assets::png::codes::U_1FAF3_1F3FF };
    ("palm_down") => { &twemoji_assets::png::codes::U_1FAF3 };
    ("palm_up_tone1") => { &twemoji_assets::png::codes::U_1FAF4_1F3FB };
    ("palm_up_tone2") => { &twemoji_assets::png::codes::U_1FAF4_1F3FC };
    ("palm_up_tone3") => { &twemoji_assets::png::codes::U_1FAF4_1F3FD };
    ("palm_up_tone4") => { &twemoji_assets::png::codes::U_1FAF4_1F3FE };
    ("palm_up_tone5") => { &twemoji_assets::png::codes::U_1FAF4_1F3FF };
    ("palm_up") => { &twemoji_assets::png::codes::U_1FAF4 };
    ("point_forward_tone1") => { &twemoji_assets::png::codes::U_1FAF5_1F3FB };
    ("point_forward_tone2") => { &twemoji_assets::png::codes::U_1FAF5_1F3FC };
    ("point_forward_tone3") => { &twemoji_assets::png::codes::U_1FAF5_1F3FD };
    ("point_forward_tone4") => { &twemoji_assets::png::codes::U_1FAF5_1F3FE };
    ("point_forward_tone5") => { &twemoji_assets::png::codes::U_1FAF5_1F3FF };
    ("point_forward") => { &twemoji_assets::png::codes::U_1FAF5 };
    ("heart_hands_tone1") => { &twemoji_assets::png::codes::U_1FAF6_1F3FB };
    ("heart_hands_tone2") => { &twemoji_assets::png::codes::U_1FAF6_1F3FC };
    ("heart_hands_tone3") => { &twemoji_assets::png::codes::U_1FAF6_1F3FD };
    ("heart_hands_tone4") => { &twemoji_assets::png::codes::U_1FAF6_1F3FE };
    ("heart_hands_tone5") => { &twemoji_assets::png::codes::U_1FAF6_1F3FF };
    ("heart_hands") => { &twemoji_assets::png::codes::U_1FAF6 };
    ("bangbang") => { &twemoji_assets::png::codes::U_203C };
    ("double_exclamation") => { &twemoji_assets::png::codes::U_203C };
    ("exclamation_question") => { &twemoji_assets::png::codes::U_2049 };
    ("interrobang") => { &twemoji_assets::png::codes::U_2049 };
    ("tm") => { &twemoji_assets::png::codes::U_2122 };
    ("trade_mark") => { &twemoji_assets::png::codes::U_2122 };
    ("info") => { &twemoji_assets::png::codes::U_2139 };
    ("information_source") => { &twemoji_assets::png::codes::U_2139 };
    ("left_right_arrow") => { &twemoji_assets::png::codes::U_2194 };
    ("arrow_up_down") => { &twemoji_assets::png::codes::U_2195 };
    ("arrow_upper_left") => { &twemoji_assets::png::codes::U_2196 };
    ("arrow_upper_right") => { &twemoji_assets::png::codes::U_2197 };
    ("arrow_lower_right") => { &twemoji_assets::png::codes::U_2198 };
    ("arrow_lower_left") => { &twemoji_assets::png::codes::U_2199 };
    ("arrow_left_hook") => { &twemoji_assets::png::codes::U_21A9 };
    ("leftwards_arrow_with_hook") => { &twemoji_assets::png::codes::U_21A9 };
    ("arrow_right_hook") => { &twemoji_assets::png::codes::U_21AA };
    ("rightwards_arrow_with_hook") => { &twemoji_assets::png::codes::U_21AA };
    ("watch") => { &twemoji_assets::png::codes::U_231A };
    ("hourglass") => { &twemoji_assets::png::codes::U_231B };
    ("keyboard") => { &twemoji_assets::png::codes::U_2328 };
    ("eject") => { &twemoji_assets::png::codes::U_23CF };
    ("fast_forward") => { &twemoji_assets::png::codes::U_23E9 };
    ("fast_reverse") => { &twemoji_assets::png::codes::U_23EA };
    ("rewind") => { &twemoji_assets::png::codes::U_23EA };
    ("arrow_double_up") => { &twemoji_assets::png::codes::U_23EB };
    ("fast_up") => { &twemoji_assets::png::codes::U_23EB };
    ("arrow_double_down") => { &twemoji_assets::png::codes::U_23EC };
    ("fast_down") => { &twemoji_assets::png::codes::U_23EC };
    ("next_track") => { &twemoji_assets::png::codes::U_23ED };
    ("previous_track") => { &twemoji_assets::png::codes::U_23EE };
    ("play_pause") => { &twemoji_assets::png::codes::U_23EF };
    ("alarm_clock") => { &twemoji_assets::png::codes::U_23F0 };
    ("stopwatch") => { &twemoji_assets::png::codes::U_23F1 };
    ("timer_clock") => { &twemoji_assets::png::codes::U_23F2 };
    ("hourglass_flowing_sand") => { &twemoji_assets::png::codes::U_23F3 };
    ("pause") => { &twemoji_assets::png::codes::U_23F8 };
    ("stop") => { &twemoji_assets::png::codes::U_23F9 };
    ("record") => { &twemoji_assets::png::codes::U_23FA };
    ("m") => { &twemoji_assets::png::codes::U_24C2 };
    ("black_small_square") => { &twemoji_assets::png::codes::U_25AA };
    ("white_small_square") => { &twemoji_assets::png::codes::U_25AB };
    ("arrow_forward") => { &twemoji_assets::png::codes::U_25B6 };
    ("play") => { &twemoji_assets::png::codes::U_25B6 };
    ("arrow_backward") => { &twemoji_assets::png::codes::U_25C0 };
    ("reverse") => { &twemoji_assets::png::codes::U_25C0 };
    ("white_medium_square") => { &twemoji_assets::png::codes::U_25FB };
    ("black_medium_square") => { &twemoji_assets::png::codes::U_25FC };
    ("white_medium_small_square") => { &twemoji_assets::png::codes::U_25FD };
    ("black_medium_small_square") => { &twemoji_assets::png::codes::U_25FE };
    ("sun") => { &twemoji_assets::png::codes::U_2600 };
    ("cloud") => { &twemoji_assets::png::codes::U_2601 };
    ("umbrella") => { &twemoji_assets::png::codes::U_2602 };
    ("snowman2") => { &twemoji_assets::png::codes::U_2603 };
    ("comet") => { &twemoji_assets::png::codes::U_2604 };
    ("telephone") => { &twemoji_assets::png::codes::U_260E };
    ("ballot_box_with_check") => { &twemoji_assets::png::codes::U_2611 };
    ("umbrella_with_rain") => { &twemoji_assets::png::codes::U_2614 };
    ("coffee") => { &twemoji_assets::png::codes::U_2615 };
    ("shamrock") => { &twemoji_assets::png::codes::U_2618 };
    ("point_up_2_tone1") => { &twemoji_assets::png::codes::U_261D_1F3FB };
    ("point_up_2_tone2") => { &twemoji_assets::png::codes::U_261D_1F3FC };
    ("point_up_2_tone3") => { &twemoji_assets::png::codes::U_261D_1F3FD };
    ("point_up_2_tone4") => { &twemoji_assets::png::codes::U_261D_1F3FE };
    ("point_up_2_tone5") => { &twemoji_assets::png::codes::U_261D_1F3FF };
    ("point_up_2") => { &twemoji_assets::png::codes::U_261D };
    ("skull_and_crossbones") => { &twemoji_assets::png::codes::U_2620 };
    ("radioactive") => { &twemoji_assets::png::codes::U_2622 };
    ("biohazard") => { &twemoji_assets::png::codes::U_2623 };
    ("orthodox_cross") => { &twemoji_assets::png::codes::U_2626 };
    ("star_and_crescent") => { &twemoji_assets::png::codes::U_262A };
    ("peace") => { &twemoji_assets::png::codes::U_262E };
    ("peace_symbol") => { &twemoji_assets::png::codes::U_262E };
    ("yin_yang") => { &twemoji_assets::png::codes::U_262F };
    ("wheel_of_dharma") => { &twemoji_assets::png::codes::U_2638 };
    ("white_frowning_face") => { &twemoji_assets::png::codes::U_2639 };
    ("relaxed") => { &twemoji_assets::png::codes::U_263A };
    ("smiling_face") => { &twemoji_assets::png::codes::U_263A };
    ("female") => { &twemoji_assets::png::codes::U_2640 };
    ("female_sign") => { &twemoji_assets::png::codes::U_2640 };
    ("male") => { &twemoji_assets::png::codes::U_2642 };
    ("male_sign") => { &twemoji_assets::png::codes::U_2642 };
    ("aries") => { &twemoji_assets::png::codes::U_2648 };
    ("taurus") => { &twemoji_assets::png::codes::U_2649 };
    ("gemini") => { &twemoji_assets::png::codes::U_264A };
    ("cancer") => { &twemoji_assets::png::codes::U_264B };
    ("leo") => { &twemoji_assets::png::codes::U_264C };
    ("virgo") => { &twemoji_assets::png::codes::U_264D };
    ("libra") => { &twemoji_assets::png::codes::U_264E };
    ("scorpius") => { &twemoji_assets::png::codes::U_264F };
    ("sagittarius") => { &twemoji_assets::png::codes::U_2650 };
    ("capricorn") => { &twemoji_assets::png::codes::U_2651 };
    ("aquarius") => { &twemoji_assets::png::codes::U_2652 };
    ("pisces") => { &twemoji_assets::png::codes::U_2653 };
    ("chess_pawn") => { &twemoji_assets::png::codes::U_265F };
    ("spades") => { &twemoji_assets::png::codes::U_2660 };
    ("clubs") => { &twemoji_assets::png::codes::U_2663 };
    ("hearts") => { &twemoji_assets::png::codes::U_2665 };
    ("diamonds") => { &twemoji_assets::png::codes::U_2666 };
    ("hotsprings") => { &twemoji_assets::png::codes::U_2668 };
    ("recycle") => { &twemoji_assets::png::codes::U_267B };
    ("recycling_symbol") => { &twemoji_assets::png::codes::U_267B };
    ("infinity") => { &twemoji_assets::png::codes::U_267E };
    ("handicapped") => { &twemoji_assets::png::codes::U_267F };
    ("wheelchair") => { &twemoji_assets::png::codes::U_267F };
    ("hammer_and_pick") => { &twemoji_assets::png::codes::U_2692 };
    ("anchor") => { &twemoji_assets::png::codes::U_2693 };
    ("crossed_swords") => { &twemoji_assets::png::codes::U_2694 };
    ("medical") => { &twemoji_assets::png::codes::U_2695 };
    ("medical_symbol") => { &twemoji_assets::png::codes::U_2695 };
    ("scales") => { &twemoji_assets::png::codes::U_2696 };
    ("alembic") => { &twemoji_assets::png::codes::U_2697 };
    ("gear") => { &twemoji_assets::png::codes::U_2699 };
    ("atom") => { &twemoji_assets::png::codes::U_269B };
    ("atom_symbol") => { &twemoji_assets::png::codes::U_269B };
    ("fleur-de-lis") => { &twemoji_assets::png::codes::U_269C };
    ("warning") => { &twemoji_assets::png::codes::U_26A0 };
    ("high_voltage") => { &twemoji_assets::png::codes::U_26A1 };
    ("zap") => { &twemoji_assets::png::codes::U_26A1 };
    ("transgender_symbol") => { &twemoji_assets::png::codes::U_26A7 };
    ("white_circle") => { &twemoji_assets::png::codes::U_26AA };
    ("black_circle") => { &twemoji_assets::png::codes::U_26AB };
    ("coffin") => { &twemoji_assets::png::codes::U_26B0 };
    ("funeral_urn") => { &twemoji_assets::png::codes::U_26B1 };
    ("soccer") => { &twemoji_assets::png::codes::U_26BD };
    ("baseball") => { &twemoji_assets::png::codes::U_26BE };
    ("snowman") => { &twemoji_assets::png::codes::U_26C4 };
    ("partly_sunny") => { &twemoji_assets::png::codes::U_26C5 };
    ("sun_behind_cloud") => { &twemoji_assets::png::codes::U_26C5 };
    ("stormy") => { &twemoji_assets::png::codes::U_26C8 };
    ("thunder_cloud_and_rain") => { &twemoji_assets::png::codes::U_26C8 };
    ("ophiuchus") => { &twemoji_assets::png::codes::U_26CE };
    ("pick") => { &twemoji_assets::png::codes::U_26CF };
    ("helmet_with_cross") => { &twemoji_assets::png::codes::U_26D1 };
    ("rescue_worker_helmet") => { &twemoji_assets::png::codes::U_26D1 };
    ("chains") => { &twemoji_assets::png::codes::U_26D3 };
    ("no_entry") => { &twemoji_assets::png::codes::U_26D4 };
    ("shinto_shrine") => { &twemoji_assets::png::codes::U_26E9 };
    ("church") => { &twemoji_assets::png::codes::U_26EA };
    ("mountain") => { &twemoji_assets::png::codes::U_26F0 };
    ("beach_umbrella") => { &twemoji_assets::png::codes::U_26F1 };
    ("umbrella_on_ground") => { &twemoji_assets::png::codes::U_26F1 };
    ("fountain") => { &twemoji_assets::png::codes::U_26F2 };
    ("golf") => { &twemoji_assets::png::codes::U_26F3 };
    ("ferry") => { &twemoji_assets::png::codes::U_26F4 };
    ("sailboat") => { &twemoji_assets::png::codes::U_26F5 };
    ("person_skiing") => { &twemoji_assets::png::codes::U_26F7 };
    ("skier") => { &twemoji_assets::png::codes::U_26F7 };
    ("skiing") => { &twemoji_assets::png::codes::U_26F7 };
    ("ice_skate") => { &twemoji_assets::png::codes::U_26F8 };
    ("woman_bouncing_ball_tone1") => { &twemoji_assets::png::codes::U_26F9_1F3FB_200D_2640_FE0F };
    ("man_bouncing_ball_tone1") => { &twemoji_assets::png::codes::U_26F9_1F3FB_200D_2642_FE0F };
    ("person_bouncing_ball_tone1") => { &twemoji_assets::png::codes::U_26F9_1F3FB };
    ("woman_bouncing_ball_tone2") => { &twemoji_assets::png::codes::U_26F9_1F3FC_200D_2640_FE0F };
    ("man_bouncing_ball_tone2") => { &twemoji_assets::png::codes::U_26F9_1F3FC_200D_2642_FE0F };
    ("person_bouncing_ball_tone2") => { &twemoji_assets::png::codes::U_26F9_1F3FC };
    ("woman_bouncing_ball_tone3") => { &twemoji_assets::png::codes::U_26F9_1F3FD_200D_2640_FE0F };
    ("man_bouncing_ball_tone3") => { &twemoji_assets::png::codes::U_26F9_1F3FD_200D_2642_FE0F };
    ("person_bouncing_ball_tone3") => { &twemoji_assets::png::codes::U_26F9_1F3FD };
    ("woman_bouncing_ball_tone4") => { &twemoji_assets::png::codes::U_26F9_1F3FE_200D_2640_FE0F };
    ("man_bouncing_ball_tone4") => { &twemoji_assets::png::codes::U_26F9_1F3FE_200D_2642_FE0F };
    ("person_bouncing_ball_tone4") => { &twemoji_assets::png::codes::U_26F9_1F3FE };
    ("woman_bouncing_ball_tone5") => { &twemoji_assets::png::codes::U_26F9_1F3FF_200D_2640_FE0F };
    ("man_bouncing_ball_tone5") => { &twemoji_assets::png::codes::U_26F9_1F3FF_200D_2642_FE0F };
    ("person_bouncing_ball_tone5") => { &twemoji_assets::png::codes::U_26F9_1F3FF };
    ("woman_bouncing_ball") => { &twemoji_assets::png::codes::U_26F9_FE0F_200D_2640_FE0F };
    ("man_bouncing_ball") => { &twemoji_assets::png::codes::U_26F9_FE0F_200D_2642_FE0F };
    ("person_bouncing_ball") => { &twemoji_assets::png::codes::U_26F9 };
    ("tent") => { &twemoji_assets::png::codes::U_26FA };
    ("fuelpump") => { &twemoji_assets::png::codes::U_26FD };
    ("scissors") => { &twemoji_assets::png::codes::U_2702 };
    ("check_mark_button") => { &twemoji_assets::png::codes::U_2705 };
    ("white_check_mark") => { &twemoji_assets::png::codes::U_2705 };
    ("airplane") => { &twemoji_assets::png::codes::U_2708 };
    ("envelope") => { &twemoji_assets::png::codes::U_2709 };
    ("fist_tone1") => { &twemoji_assets::png::codes::U_270A_1F3FB };
    ("fist_tone2") => { &twemoji_assets::png::codes::U_270A_1F3FC };
    ("fist_tone3") => { &twemoji_assets::png::codes::U_270A_1F3FD };
    ("fist_tone4") => { &twemoji_assets::png::codes::U_270A_1F3FE };
    ("fist_tone5") => { &twemoji_assets::png::codes::U_270A_1F3FF };
    ("fist") => { &twemoji_assets::png::codes::U_270A };
    ("high_five_tone1") => { &twemoji_assets::png::codes::U_270B_1F3FB };
    ("raised_hand_tone1") => { &twemoji_assets::png::codes::U_270B_1F3FB };
    ("high_five_tone2") => { &twemoji_assets::png::codes::U_270B_1F3FC };
    ("raised_hand_tone2") => { &twemoji_assets::png::codes::U_270B_1F3FC };
    ("high_five_tone3") => { &twemoji_assets::png::codes::U_270B_1F3FD };
    ("raised_hand_tone3") => { &twemoji_assets::png::codes::U_270B_1F3FD };
    ("high_five_tone4") => { &twemoji_assets::png::codes::U_270B_1F3FE };
    ("raised_hand_tone4") => { &twemoji_assets::png::codes::U_270B_1F3FE };
    ("high_five_tone5") => { &twemoji_assets::png::codes::U_270B_1F3FF };
    ("raised_hand_tone5") => { &twemoji_assets::png::codes::U_270B_1F3FF };
    ("high_five") => { &twemoji_assets::png::codes::U_270B };
    ("raised_hand") => { &twemoji_assets::png::codes::U_270B };
    ("v_tone1") => { &twemoji_assets::png::codes::U_270C_1F3FB };
    ("victory_tone1") => { &twemoji_assets::png::codes::U_270C_1F3FB };
    ("v_tone2") => { &twemoji_assets::png::codes::U_270C_1F3FC };
    ("victory_tone2") => { &twemoji_assets::png::codes::U_270C_1F3FC };
    ("v_tone3") => { &twemoji_assets::png::codes::U_270C_1F3FD };
    ("victory_tone3") => { &twemoji_assets::png::codes::U_270C_1F3FD };
    ("v_tone4") => { &twemoji_assets::png::codes::U_270C_1F3FE };
    ("victory_tone4") => { &twemoji_assets::png::codes::U_270C_1F3FE };
    ("v_tone5") => { &twemoji_assets::png::codes::U_270C_1F3FF };
    ("victory_tone5") => { &twemoji_assets::png::codes::U_270C_1F3FF };
    ("v") => { &twemoji_assets::png::codes::U_270C };
    ("victory") => { &twemoji_assets::png::codes::U_270C };
    ("writing_hand_tone1") => { &twemoji_assets::png::codes::U_270D_1F3FB };
    ("writing_hand_tone2") => { &twemoji_assets::png::codes::U_270D_1F3FC };
    ("writing_hand_tone3") => { &twemoji_assets::png::codes::U_270D_1F3FD };
    ("writing_hand_tone4") => { &twemoji_assets::png::codes::U_270D_1F3FE };
    ("writing_hand_tone5") => { &twemoji_assets::png::codes::U_270D_1F3FF };
    ("writing_hand") => { &twemoji_assets::png::codes::U_270D };
    ("pencil") => { &twemoji_assets::png::codes::U_270F };
    ("black_nib") => { &twemoji_assets::png::codes::U_2712 };
    ("check_mark") => { &twemoji_assets::png::codes::U_2714 };
    ("heavy_check_mark") => { &twemoji_assets::png::codes::U_2714 };
    ("multiplication") => { &twemoji_assets::png::codes::U_2716 };
    ("multiply") => { &twemoji_assets::png::codes::U_2716 };
    ("latin_cross") => { &twemoji_assets::png::codes::U_271D };
    ("star_of_david") => { &twemoji_assets::png::codes::U_2721 };
    ("sparkles") => { &twemoji_assets::png::codes::U_2728 };
    ("eight_spoked_asterisk") => { &twemoji_assets::png::codes::U_2733 };
    ("eight_pointed_black_star") => { &twemoji_assets::png::codes::U_2734 };
    ("snowflake") => { &twemoji_assets::png::codes::U_2744 };
    ("sparkle") => { &twemoji_assets::png::codes::U_2747 };
    ("cross_mark") => { &twemoji_assets::png::codes::U_274C };
    ("x") => { &twemoji_assets::png::codes::U_274C };
    ("cross_mark_button") => { &twemoji_assets::png::codes::U_274E };
    ("negative_squared_cross_mark") => { &twemoji_assets::png::codes::U_274E };
    ("question") => { &twemoji_assets::png::codes::U_2753 };
    ("white_question") => { &twemoji_assets::png::codes::U_2754 };
    ("white_exclamation") => { &twemoji_assets::png::codes::U_2755 };
    ("exclamation") => { &twemoji_assets::png::codes::U_2757 };
    ("heart_exclamation") => { &twemoji_assets::png::codes::U_2763 };
    ("heart_on_fire") => { &twemoji_assets::png::codes::U_2764_FE0F_200D_1F525 };
    ("mending_heart") => { &twemoji_assets::png::codes::U_2764_FE0F_200D_1FA79 };
    ("heart") => { &twemoji_assets::png::codes::U_2764 };
    ("red_heart") => { &twemoji_assets::png::codes::U_2764 };
    ("plus") => { &twemoji_assets::png::codes::U_2795 };
    ("minus") => { &twemoji_assets::png::codes::U_2796 };
    ("divide") => { &twemoji_assets::png::codes::U_2797 };
    ("division") => { &twemoji_assets::png::codes::U_2797 };
    ("arrow_right") => { &twemoji_assets::png::codes::U_27A1 };
    ("curly_loop") => { &twemoji_assets::png::codes::U_27B0 };
    ("double_curly_loop") => { &twemoji_assets::png::codes::U_27BF };
    ("loop") => { &twemoji_assets::png::codes::U_27BF };
    ("arrow_heading_up") => { &twemoji_assets::png::codes::U_2934 };
    ("arrow_heading_down") => { &twemoji_assets::png::codes::U_2935 };
    ("arrow_left") => { &twemoji_assets::png::codes::U_2B05 };
    ("arrow_up") => { &twemoji_assets::png::codes::U_2B06 };
    ("arrow_down") => { &twemoji_assets::png::codes::U_2B07 };
    ("black_large_square") => { &twemoji_assets::png::codes::U_2B1B };
    ("white_large_square") => { &twemoji_assets::png::codes::U_2B1C };
    ("star") => { &twemoji_assets::png::codes::U_2B50 };
    ("hollow_red_circle") => { &twemoji_assets::png::codes::U_2B55 };
    ("red_o") => { &twemoji_assets::png::codes::U_2B55 };
    ("wavy_dash") => { &twemoji_assets::png::codes::U_3030 };
    ("part_alternation_mark") => { &twemoji_assets::png::codes::U_303D };
    ("congratulations") => { &twemoji_assets::png::codes::U_3297 };
    ("ja_congratulations") => { &twemoji_assets::png::codes::U_3297 };
    ("ja_secret") => { &twemoji_assets::png::codes::U_3299 };
    ("secret") => { &twemoji_assets::png::codes::U_3299 };
}
