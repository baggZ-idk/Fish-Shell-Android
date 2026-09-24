include!("/home/baggz/Downloads/fish-shell/build-android/fish-localization-map-cache/zh_TW");
include!("/home/baggz/Downloads/fish-shell/build-android/fish-localization-map-cache/sv");
include!("/home/baggz/Downloads/fish-shell/build-android/fish-localization-map-cache/zh_CN");
include!("/home/baggz/Downloads/fish-shell/build-android/fish-localization-map-cache/fr");
include!("/home/baggz/Downloads/fish-shell/build-android/fish-localization-map-cache/de");
include!("/home/baggz/Downloads/fish-shell/build-android/fish-localization-map-cache/pt_BR");
include!("/home/baggz/Downloads/fish-shell/build-android/fish-localization-map-cache/pl");
include!("/home/baggz/Downloads/fish-shell/build-android/fish-localization-map-cache/es");
include!("/home/baggz/Downloads/fish-shell/build-android/fish-localization-map-cache/ja_JP");
use fish_localization::Language;
pub static CATALOGS: phf::Map<Language, &phf::Map<&str, &str>> = ::phf::Map {
    key: 5277884738100225802,
    disps: &[
        (7, 0),
        (8, 1),
    ],
    entries: &[
        (Language("de"), &LANG_MAP_de),
        (Language("es"), &LANG_MAP_es),
        (Language("fr"), &LANG_MAP_fr),
        (Language("ja_JP"), &LANG_MAP_ja_JP),
        (Language("pl"), &LANG_MAP_pl),
        (Language("sv"), &LANG_MAP_sv),
        (Language("zh_CN"), &LANG_MAP_zh_CN),
        (Language("pt_BR"), &LANG_MAP_pt_BR),
        (Language("zh_TW"), &LANG_MAP_zh_TW),
    ],
};
