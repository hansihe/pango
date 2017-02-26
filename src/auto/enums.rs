// This file was generated by gir (aacfc92) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use glib_ffi;
use glib::error::ErrorDomain;
use glib::translate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Alignment {
    Left,
    Center,
    Right,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for Alignment {
    type GlibType = ffi::PangoAlignment;

    fn to_glib(&self) -> ffi::PangoAlignment {
        match *self {
            Alignment::Left => ffi::PANGO_ALIGN_LEFT,
            Alignment::Center => ffi::PANGO_ALIGN_CENTER,
            Alignment::Right => ffi::PANGO_ALIGN_RIGHT,
            Alignment::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoAlignment> for Alignment {
    fn from_glib(value: ffi::PangoAlignment) -> Self {
        match value {
            ffi::PANGO_ALIGN_LEFT => Alignment::Left,
            ffi::PANGO_ALIGN_CENTER => Alignment::Center,
            ffi::PANGO_ALIGN_RIGHT => Alignment::Right,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum BidiType {
    L,
    Lre,
    Lro,
    R,
    Al,
    Rle,
    Rlo,
    Pdf,
    En,
    Es,
    Et,
    An,
    Cs,
    Nsm,
    Bn,
    B,
    S,
    Ws,
    On,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for BidiType {
    type GlibType = ffi::PangoBidiType;

    fn to_glib(&self) -> ffi::PangoBidiType {
        match *self {
            BidiType::L => ffi::PANGO_BIDI_TYPE_L,
            BidiType::Lre => ffi::PANGO_BIDI_TYPE_LRE,
            BidiType::Lro => ffi::PANGO_BIDI_TYPE_LRO,
            BidiType::R => ffi::PANGO_BIDI_TYPE_R,
            BidiType::Al => ffi::PANGO_BIDI_TYPE_AL,
            BidiType::Rle => ffi::PANGO_BIDI_TYPE_RLE,
            BidiType::Rlo => ffi::PANGO_BIDI_TYPE_RLO,
            BidiType::Pdf => ffi::PANGO_BIDI_TYPE_PDF,
            BidiType::En => ffi::PANGO_BIDI_TYPE_EN,
            BidiType::Es => ffi::PANGO_BIDI_TYPE_ES,
            BidiType::Et => ffi::PANGO_BIDI_TYPE_ET,
            BidiType::An => ffi::PANGO_BIDI_TYPE_AN,
            BidiType::Cs => ffi::PANGO_BIDI_TYPE_CS,
            BidiType::Nsm => ffi::PANGO_BIDI_TYPE_NSM,
            BidiType::Bn => ffi::PANGO_BIDI_TYPE_BN,
            BidiType::B => ffi::PANGO_BIDI_TYPE_B,
            BidiType::S => ffi::PANGO_BIDI_TYPE_S,
            BidiType::Ws => ffi::PANGO_BIDI_TYPE_WS,
            BidiType::On => ffi::PANGO_BIDI_TYPE_ON,
            BidiType::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoBidiType> for BidiType {
    fn from_glib(value: ffi::PangoBidiType) -> Self {
        match value {
            ffi::PANGO_BIDI_TYPE_L => BidiType::L,
            ffi::PANGO_BIDI_TYPE_LRE => BidiType::Lre,
            ffi::PANGO_BIDI_TYPE_LRO => BidiType::Lro,
            ffi::PANGO_BIDI_TYPE_R => BidiType::R,
            ffi::PANGO_BIDI_TYPE_AL => BidiType::Al,
            ffi::PANGO_BIDI_TYPE_RLE => BidiType::Rle,
            ffi::PANGO_BIDI_TYPE_RLO => BidiType::Rlo,
            ffi::PANGO_BIDI_TYPE_PDF => BidiType::Pdf,
            ffi::PANGO_BIDI_TYPE_EN => BidiType::En,
            ffi::PANGO_BIDI_TYPE_ES => BidiType::Es,
            ffi::PANGO_BIDI_TYPE_ET => BidiType::Et,
            ffi::PANGO_BIDI_TYPE_AN => BidiType::An,
            ffi::PANGO_BIDI_TYPE_CS => BidiType::Cs,
            ffi::PANGO_BIDI_TYPE_NSM => BidiType::Nsm,
            ffi::PANGO_BIDI_TYPE_BN => BidiType::Bn,
            ffi::PANGO_BIDI_TYPE_B => BidiType::B,
            ffi::PANGO_BIDI_TYPE_S => BidiType::S,
            ffi::PANGO_BIDI_TYPE_WS => BidiType::Ws,
            ffi::PANGO_BIDI_TYPE_ON => BidiType::On,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Direction {
    Ltr,
    Rtl,
    TtbLtr,
    TtbRtl,
    WeakLtr,
    WeakRtl,
    Neutral,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for Direction {
    type GlibType = ffi::PangoDirection;

    fn to_glib(&self) -> ffi::PangoDirection {
        match *self {
            Direction::Ltr => ffi::PANGO_DIRECTION_LTR,
            Direction::Rtl => ffi::PANGO_DIRECTION_RTL,
            Direction::TtbLtr => ffi::PANGO_DIRECTION_TTB_LTR,
            Direction::TtbRtl => ffi::PANGO_DIRECTION_TTB_RTL,
            Direction::WeakLtr => ffi::PANGO_DIRECTION_WEAK_LTR,
            Direction::WeakRtl => ffi::PANGO_DIRECTION_WEAK_RTL,
            Direction::Neutral => ffi::PANGO_DIRECTION_NEUTRAL,
            Direction::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoDirection> for Direction {
    fn from_glib(value: ffi::PangoDirection) -> Self {
        match value {
            ffi::PANGO_DIRECTION_LTR => Direction::Ltr,
            ffi::PANGO_DIRECTION_RTL => Direction::Rtl,
            ffi::PANGO_DIRECTION_TTB_LTR => Direction::TtbLtr,
            ffi::PANGO_DIRECTION_TTB_RTL => Direction::TtbRtl,
            ffi::PANGO_DIRECTION_WEAK_LTR => Direction::WeakLtr,
            ffi::PANGO_DIRECTION_WEAK_RTL => Direction::WeakRtl,
            ffi::PANGO_DIRECTION_NEUTRAL => Direction::Neutral,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum EllipsizeMode {
    None,
    Start,
    Middle,
    End,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for EllipsizeMode {
    type GlibType = ffi::PangoEllipsizeMode;

    fn to_glib(&self) -> ffi::PangoEllipsizeMode {
        match *self {
            EllipsizeMode::None => ffi::PANGO_ELLIPSIZE_NONE,
            EllipsizeMode::Start => ffi::PANGO_ELLIPSIZE_START,
            EllipsizeMode::Middle => ffi::PANGO_ELLIPSIZE_MIDDLE,
            EllipsizeMode::End => ffi::PANGO_ELLIPSIZE_END,
            EllipsizeMode::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoEllipsizeMode> for EllipsizeMode {
    fn from_glib(value: ffi::PangoEllipsizeMode) -> Self {
        match value {
            ffi::PANGO_ELLIPSIZE_NONE => EllipsizeMode::None,
            ffi::PANGO_ELLIPSIZE_START => EllipsizeMode::Start,
            ffi::PANGO_ELLIPSIZE_MIDDLE => EllipsizeMode::Middle,
            ffi::PANGO_ELLIPSIZE_END => EllipsizeMode::End,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Gravity {
    South,
    East,
    North,
    West,
    Auto,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for Gravity {
    type GlibType = ffi::PangoGravity;

    fn to_glib(&self) -> ffi::PangoGravity {
        match *self {
            Gravity::South => ffi::PANGO_GRAVITY_SOUTH,
            Gravity::East => ffi::PANGO_GRAVITY_EAST,
            Gravity::North => ffi::PANGO_GRAVITY_NORTH,
            Gravity::West => ffi::PANGO_GRAVITY_WEST,
            Gravity::Auto => ffi::PANGO_GRAVITY_AUTO,
            Gravity::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoGravity> for Gravity {
    fn from_glib(value: ffi::PangoGravity) -> Self {
        match value {
            ffi::PANGO_GRAVITY_SOUTH => Gravity::South,
            ffi::PANGO_GRAVITY_EAST => Gravity::East,
            ffi::PANGO_GRAVITY_NORTH => Gravity::North,
            ffi::PANGO_GRAVITY_WEST => Gravity::West,
            ffi::PANGO_GRAVITY_AUTO => Gravity::Auto,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum GravityHint {
    Natural,
    Strong,
    Line,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for GravityHint {
    type GlibType = ffi::PangoGravityHint;

    fn to_glib(&self) -> ffi::PangoGravityHint {
        match *self {
            GravityHint::Natural => ffi::PANGO_GRAVITY_HINT_NATURAL,
            GravityHint::Strong => ffi::PANGO_GRAVITY_HINT_STRONG,
            GravityHint::Line => ffi::PANGO_GRAVITY_HINT_LINE,
            GravityHint::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoGravityHint> for GravityHint {
    fn from_glib(value: ffi::PangoGravityHint) -> Self {
        match value {
            ffi::PANGO_GRAVITY_HINT_NATURAL => GravityHint::Natural,
            ffi::PANGO_GRAVITY_HINT_STRONG => GravityHint::Strong,
            ffi::PANGO_GRAVITY_HINT_LINE => GravityHint::Line,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Script {
    InvalidCode,
    Common,
    Inherited,
    Arabic,
    Armenian,
    Bengali,
    Bopomofo,
    Cherokee,
    Coptic,
    Cyrillic,
    Deseret,
    Devanagari,
    Ethiopic,
    Georgian,
    Gothic,
    Greek,
    Gujarati,
    Gurmukhi,
    Han,
    Hangul,
    Hebrew,
    Hiragana,
    Kannada,
    Katakana,
    Khmer,
    Lao,
    Latin,
    Malayalam,
    Mongolian,
    Myanmar,
    Ogham,
    OldItalic,
    Oriya,
    Runic,
    Sinhala,
    Syriac,
    Tamil,
    Telugu,
    Thaana,
    Thai,
    Tibetan,
    CanadianAboriginal,
    Yi,
    Tagalog,
    Hanunoo,
    Buhid,
    Tagbanwa,
    Braille,
    Cypriot,
    Limbu,
    Osmanya,
    Shavian,
    LinearB,
    TaiLe,
    Ugaritic,
    NewTaiLue,
    Buginese,
    Glagolitic,
    Tifinagh,
    SylotiNagri,
    OldPersian,
    Kharoshthi,
    Unknown,
    Balinese,
    Cuneiform,
    Phoenician,
    PhagsPa,
    Nko,
    KayahLi,
    Lepcha,
    Rejang,
    Sundanese,
    Saurashtra,
    Cham,
    OlChiki,
    Vai,
    Carian,
    Lycian,
    Lydian,
    Batak,
    Brahmi,
    Mandaic,
    Chakma,
    MeroiticCursive,
    MeroiticHieroglyphs,
    Miao,
    Sharada,
    SoraSompeng,
    Takri,
    BassaVah,
    CaucasianAlbanian,
    Duployan,
    Elbasan,
    Grantha,
    Khojki,
    Khudawadi,
    LinearA,
    Mahajani,
    Manichaean,
    MendeKikakui,
    Modi,
    Mro,
    Nabataean,
    OldNorthArabian,
    OldPermic,
    PahawhHmong,
    Palmyrene,
    PauCinHau,
    PsalterPahlavi,
    Siddham,
    Tirhuta,
    WarangCiti,
    Ahom,
    AnatolianHieroglyphs,
    Hatran,
    Multani,
    OldHungarian,
    Signwriting,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for Script {
    type GlibType = ffi::PangoScript;

    fn to_glib(&self) -> ffi::PangoScript {
        match *self {
            Script::InvalidCode => ffi::PANGO_SCRIPT_INVALID_CODE,
            Script::Common => ffi::PANGO_SCRIPT_COMMON,
            Script::Inherited => ffi::PANGO_SCRIPT_INHERITED,
            Script::Arabic => ffi::PANGO_SCRIPT_ARABIC,
            Script::Armenian => ffi::PANGO_SCRIPT_ARMENIAN,
            Script::Bengali => ffi::PANGO_SCRIPT_BENGALI,
            Script::Bopomofo => ffi::PANGO_SCRIPT_BOPOMOFO,
            Script::Cherokee => ffi::PANGO_SCRIPT_CHEROKEE,
            Script::Coptic => ffi::PANGO_SCRIPT_COPTIC,
            Script::Cyrillic => ffi::PANGO_SCRIPT_CYRILLIC,
            Script::Deseret => ffi::PANGO_SCRIPT_DESERET,
            Script::Devanagari => ffi::PANGO_SCRIPT_DEVANAGARI,
            Script::Ethiopic => ffi::PANGO_SCRIPT_ETHIOPIC,
            Script::Georgian => ffi::PANGO_SCRIPT_GEORGIAN,
            Script::Gothic => ffi::PANGO_SCRIPT_GOTHIC,
            Script::Greek => ffi::PANGO_SCRIPT_GREEK,
            Script::Gujarati => ffi::PANGO_SCRIPT_GUJARATI,
            Script::Gurmukhi => ffi::PANGO_SCRIPT_GURMUKHI,
            Script::Han => ffi::PANGO_SCRIPT_HAN,
            Script::Hangul => ffi::PANGO_SCRIPT_HANGUL,
            Script::Hebrew => ffi::PANGO_SCRIPT_HEBREW,
            Script::Hiragana => ffi::PANGO_SCRIPT_HIRAGANA,
            Script::Kannada => ffi::PANGO_SCRIPT_KANNADA,
            Script::Katakana => ffi::PANGO_SCRIPT_KATAKANA,
            Script::Khmer => ffi::PANGO_SCRIPT_KHMER,
            Script::Lao => ffi::PANGO_SCRIPT_LAO,
            Script::Latin => ffi::PANGO_SCRIPT_LATIN,
            Script::Malayalam => ffi::PANGO_SCRIPT_MALAYALAM,
            Script::Mongolian => ffi::PANGO_SCRIPT_MONGOLIAN,
            Script::Myanmar => ffi::PANGO_SCRIPT_MYANMAR,
            Script::Ogham => ffi::PANGO_SCRIPT_OGHAM,
            Script::OldItalic => ffi::PANGO_SCRIPT_OLD_ITALIC,
            Script::Oriya => ffi::PANGO_SCRIPT_ORIYA,
            Script::Runic => ffi::PANGO_SCRIPT_RUNIC,
            Script::Sinhala => ffi::PANGO_SCRIPT_SINHALA,
            Script::Syriac => ffi::PANGO_SCRIPT_SYRIAC,
            Script::Tamil => ffi::PANGO_SCRIPT_TAMIL,
            Script::Telugu => ffi::PANGO_SCRIPT_TELUGU,
            Script::Thaana => ffi::PANGO_SCRIPT_THAANA,
            Script::Thai => ffi::PANGO_SCRIPT_THAI,
            Script::Tibetan => ffi::PANGO_SCRIPT_TIBETAN,
            Script::CanadianAboriginal => ffi::PANGO_SCRIPT_CANADIAN_ABORIGINAL,
            Script::Yi => ffi::PANGO_SCRIPT_YI,
            Script::Tagalog => ffi::PANGO_SCRIPT_TAGALOG,
            Script::Hanunoo => ffi::PANGO_SCRIPT_HANUNOO,
            Script::Buhid => ffi::PANGO_SCRIPT_BUHID,
            Script::Tagbanwa => ffi::PANGO_SCRIPT_TAGBANWA,
            Script::Braille => ffi::PANGO_SCRIPT_BRAILLE,
            Script::Cypriot => ffi::PANGO_SCRIPT_CYPRIOT,
            Script::Limbu => ffi::PANGO_SCRIPT_LIMBU,
            Script::Osmanya => ffi::PANGO_SCRIPT_OSMANYA,
            Script::Shavian => ffi::PANGO_SCRIPT_SHAVIAN,
            Script::LinearB => ffi::PANGO_SCRIPT_LINEAR_B,
            Script::TaiLe => ffi::PANGO_SCRIPT_TAI_LE,
            Script::Ugaritic => ffi::PANGO_SCRIPT_UGARITIC,
            Script::NewTaiLue => ffi::PANGO_SCRIPT_NEW_TAI_LUE,
            Script::Buginese => ffi::PANGO_SCRIPT_BUGINESE,
            Script::Glagolitic => ffi::PANGO_SCRIPT_GLAGOLITIC,
            Script::Tifinagh => ffi::PANGO_SCRIPT_TIFINAGH,
            Script::SylotiNagri => ffi::PANGO_SCRIPT_SYLOTI_NAGRI,
            Script::OldPersian => ffi::PANGO_SCRIPT_OLD_PERSIAN,
            Script::Kharoshthi => ffi::PANGO_SCRIPT_KHAROSHTHI,
            Script::Unknown => ffi::PANGO_SCRIPT_UNKNOWN,
            Script::Balinese => ffi::PANGO_SCRIPT_BALINESE,
            Script::Cuneiform => ffi::PANGO_SCRIPT_CUNEIFORM,
            Script::Phoenician => ffi::PANGO_SCRIPT_PHOENICIAN,
            Script::PhagsPa => ffi::PANGO_SCRIPT_PHAGS_PA,
            Script::Nko => ffi::PANGO_SCRIPT_NKO,
            Script::KayahLi => ffi::PANGO_SCRIPT_KAYAH_LI,
            Script::Lepcha => ffi::PANGO_SCRIPT_LEPCHA,
            Script::Rejang => ffi::PANGO_SCRIPT_REJANG,
            Script::Sundanese => ffi::PANGO_SCRIPT_SUNDANESE,
            Script::Saurashtra => ffi::PANGO_SCRIPT_SAURASHTRA,
            Script::Cham => ffi::PANGO_SCRIPT_CHAM,
            Script::OlChiki => ffi::PANGO_SCRIPT_OL_CHIKI,
            Script::Vai => ffi::PANGO_SCRIPT_VAI,
            Script::Carian => ffi::PANGO_SCRIPT_CARIAN,
            Script::Lycian => ffi::PANGO_SCRIPT_LYCIAN,
            Script::Lydian => ffi::PANGO_SCRIPT_LYDIAN,
            Script::Batak => ffi::PANGO_SCRIPT_BATAK,
            Script::Brahmi => ffi::PANGO_SCRIPT_BRAHMI,
            Script::Mandaic => ffi::PANGO_SCRIPT_MANDAIC,
            Script::Chakma => ffi::PANGO_SCRIPT_CHAKMA,
            Script::MeroiticCursive => ffi::PANGO_SCRIPT_MEROITIC_CURSIVE,
            Script::MeroiticHieroglyphs => ffi::PANGO_SCRIPT_MEROITIC_HIEROGLYPHS,
            Script::Miao => ffi::PANGO_SCRIPT_MIAO,
            Script::Sharada => ffi::PANGO_SCRIPT_SHARADA,
            Script::SoraSompeng => ffi::PANGO_SCRIPT_SORA_SOMPENG,
            Script::Takri => ffi::PANGO_SCRIPT_TAKRI,
            Script::BassaVah => ffi::PANGO_SCRIPT_BASSA_VAH,
            Script::CaucasianAlbanian => ffi::PANGO_SCRIPT_CAUCASIAN_ALBANIAN,
            Script::Duployan => ffi::PANGO_SCRIPT_DUPLOYAN,
            Script::Elbasan => ffi::PANGO_SCRIPT_ELBASAN,
            Script::Grantha => ffi::PANGO_SCRIPT_GRANTHA,
            Script::Khojki => ffi::PANGO_SCRIPT_KHOJKI,
            Script::Khudawadi => ffi::PANGO_SCRIPT_KHUDAWADI,
            Script::LinearA => ffi::PANGO_SCRIPT_LINEAR_A,
            Script::Mahajani => ffi::PANGO_SCRIPT_MAHAJANI,
            Script::Manichaean => ffi::PANGO_SCRIPT_MANICHAEAN,
            Script::MendeKikakui => ffi::PANGO_SCRIPT_MENDE_KIKAKUI,
            Script::Modi => ffi::PANGO_SCRIPT_MODI,
            Script::Mro => ffi::PANGO_SCRIPT_MRO,
            Script::Nabataean => ffi::PANGO_SCRIPT_NABATAEAN,
            Script::OldNorthArabian => ffi::PANGO_SCRIPT_OLD_NORTH_ARABIAN,
            Script::OldPermic => ffi::PANGO_SCRIPT_OLD_PERMIC,
            Script::PahawhHmong => ffi::PANGO_SCRIPT_PAHAWH_HMONG,
            Script::Palmyrene => ffi::PANGO_SCRIPT_PALMYRENE,
            Script::PauCinHau => ffi::PANGO_SCRIPT_PAU_CIN_HAU,
            Script::PsalterPahlavi => ffi::PANGO_SCRIPT_PSALTER_PAHLAVI,
            Script::Siddham => ffi::PANGO_SCRIPT_SIDDHAM,
            Script::Tirhuta => ffi::PANGO_SCRIPT_TIRHUTA,
            Script::WarangCiti => ffi::PANGO_SCRIPT_WARANG_CITI,
            Script::Ahom => ffi::PANGO_SCRIPT_AHOM,
            Script::AnatolianHieroglyphs => ffi::PANGO_SCRIPT_ANATOLIAN_HIEROGLYPHS,
            Script::Hatran => ffi::PANGO_SCRIPT_HATRAN,
            Script::Multani => ffi::PANGO_SCRIPT_MULTANI,
            Script::OldHungarian => ffi::PANGO_SCRIPT_OLD_HUNGARIAN,
            Script::Signwriting => ffi::PANGO_SCRIPT_SIGNWRITING,
            Script::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoScript> for Script {
    fn from_glib(value: ffi::PangoScript) -> Self {
        match value {
            ffi::PANGO_SCRIPT_INVALID_CODE => Script::InvalidCode,
            ffi::PANGO_SCRIPT_COMMON => Script::Common,
            ffi::PANGO_SCRIPT_INHERITED => Script::Inherited,
            ffi::PANGO_SCRIPT_ARABIC => Script::Arabic,
            ffi::PANGO_SCRIPT_ARMENIAN => Script::Armenian,
            ffi::PANGO_SCRIPT_BENGALI => Script::Bengali,
            ffi::PANGO_SCRIPT_BOPOMOFO => Script::Bopomofo,
            ffi::PANGO_SCRIPT_CHEROKEE => Script::Cherokee,
            ffi::PANGO_SCRIPT_COPTIC => Script::Coptic,
            ffi::PANGO_SCRIPT_CYRILLIC => Script::Cyrillic,
            ffi::PANGO_SCRIPT_DESERET => Script::Deseret,
            ffi::PANGO_SCRIPT_DEVANAGARI => Script::Devanagari,
            ffi::PANGO_SCRIPT_ETHIOPIC => Script::Ethiopic,
            ffi::PANGO_SCRIPT_GEORGIAN => Script::Georgian,
            ffi::PANGO_SCRIPT_GOTHIC => Script::Gothic,
            ffi::PANGO_SCRIPT_GREEK => Script::Greek,
            ffi::PANGO_SCRIPT_GUJARATI => Script::Gujarati,
            ffi::PANGO_SCRIPT_GURMUKHI => Script::Gurmukhi,
            ffi::PANGO_SCRIPT_HAN => Script::Han,
            ffi::PANGO_SCRIPT_HANGUL => Script::Hangul,
            ffi::PANGO_SCRIPT_HEBREW => Script::Hebrew,
            ffi::PANGO_SCRIPT_HIRAGANA => Script::Hiragana,
            ffi::PANGO_SCRIPT_KANNADA => Script::Kannada,
            ffi::PANGO_SCRIPT_KATAKANA => Script::Katakana,
            ffi::PANGO_SCRIPT_KHMER => Script::Khmer,
            ffi::PANGO_SCRIPT_LAO => Script::Lao,
            ffi::PANGO_SCRIPT_LATIN => Script::Latin,
            ffi::PANGO_SCRIPT_MALAYALAM => Script::Malayalam,
            ffi::PANGO_SCRIPT_MONGOLIAN => Script::Mongolian,
            ffi::PANGO_SCRIPT_MYANMAR => Script::Myanmar,
            ffi::PANGO_SCRIPT_OGHAM => Script::Ogham,
            ffi::PANGO_SCRIPT_OLD_ITALIC => Script::OldItalic,
            ffi::PANGO_SCRIPT_ORIYA => Script::Oriya,
            ffi::PANGO_SCRIPT_RUNIC => Script::Runic,
            ffi::PANGO_SCRIPT_SINHALA => Script::Sinhala,
            ffi::PANGO_SCRIPT_SYRIAC => Script::Syriac,
            ffi::PANGO_SCRIPT_TAMIL => Script::Tamil,
            ffi::PANGO_SCRIPT_TELUGU => Script::Telugu,
            ffi::PANGO_SCRIPT_THAANA => Script::Thaana,
            ffi::PANGO_SCRIPT_THAI => Script::Thai,
            ffi::PANGO_SCRIPT_TIBETAN => Script::Tibetan,
            ffi::PANGO_SCRIPT_CANADIAN_ABORIGINAL => Script::CanadianAboriginal,
            ffi::PANGO_SCRIPT_YI => Script::Yi,
            ffi::PANGO_SCRIPT_TAGALOG => Script::Tagalog,
            ffi::PANGO_SCRIPT_HANUNOO => Script::Hanunoo,
            ffi::PANGO_SCRIPT_BUHID => Script::Buhid,
            ffi::PANGO_SCRIPT_TAGBANWA => Script::Tagbanwa,
            ffi::PANGO_SCRIPT_BRAILLE => Script::Braille,
            ffi::PANGO_SCRIPT_CYPRIOT => Script::Cypriot,
            ffi::PANGO_SCRIPT_LIMBU => Script::Limbu,
            ffi::PANGO_SCRIPT_OSMANYA => Script::Osmanya,
            ffi::PANGO_SCRIPT_SHAVIAN => Script::Shavian,
            ffi::PANGO_SCRIPT_LINEAR_B => Script::LinearB,
            ffi::PANGO_SCRIPT_TAI_LE => Script::TaiLe,
            ffi::PANGO_SCRIPT_UGARITIC => Script::Ugaritic,
            ffi::PANGO_SCRIPT_NEW_TAI_LUE => Script::NewTaiLue,
            ffi::PANGO_SCRIPT_BUGINESE => Script::Buginese,
            ffi::PANGO_SCRIPT_GLAGOLITIC => Script::Glagolitic,
            ffi::PANGO_SCRIPT_TIFINAGH => Script::Tifinagh,
            ffi::PANGO_SCRIPT_SYLOTI_NAGRI => Script::SylotiNagri,
            ffi::PANGO_SCRIPT_OLD_PERSIAN => Script::OldPersian,
            ffi::PANGO_SCRIPT_KHAROSHTHI => Script::Kharoshthi,
            ffi::PANGO_SCRIPT_UNKNOWN => Script::Unknown,
            ffi::PANGO_SCRIPT_BALINESE => Script::Balinese,
            ffi::PANGO_SCRIPT_CUNEIFORM => Script::Cuneiform,
            ffi::PANGO_SCRIPT_PHOENICIAN => Script::Phoenician,
            ffi::PANGO_SCRIPT_PHAGS_PA => Script::PhagsPa,
            ffi::PANGO_SCRIPT_NKO => Script::Nko,
            ffi::PANGO_SCRIPT_KAYAH_LI => Script::KayahLi,
            ffi::PANGO_SCRIPT_LEPCHA => Script::Lepcha,
            ffi::PANGO_SCRIPT_REJANG => Script::Rejang,
            ffi::PANGO_SCRIPT_SUNDANESE => Script::Sundanese,
            ffi::PANGO_SCRIPT_SAURASHTRA => Script::Saurashtra,
            ffi::PANGO_SCRIPT_CHAM => Script::Cham,
            ffi::PANGO_SCRIPT_OL_CHIKI => Script::OlChiki,
            ffi::PANGO_SCRIPT_VAI => Script::Vai,
            ffi::PANGO_SCRIPT_CARIAN => Script::Carian,
            ffi::PANGO_SCRIPT_LYCIAN => Script::Lycian,
            ffi::PANGO_SCRIPT_LYDIAN => Script::Lydian,
            ffi::PANGO_SCRIPT_BATAK => Script::Batak,
            ffi::PANGO_SCRIPT_BRAHMI => Script::Brahmi,
            ffi::PANGO_SCRIPT_MANDAIC => Script::Mandaic,
            ffi::PANGO_SCRIPT_CHAKMA => Script::Chakma,
            ffi::PANGO_SCRIPT_MEROITIC_CURSIVE => Script::MeroiticCursive,
            ffi::PANGO_SCRIPT_MEROITIC_HIEROGLYPHS => Script::MeroiticHieroglyphs,
            ffi::PANGO_SCRIPT_MIAO => Script::Miao,
            ffi::PANGO_SCRIPT_SHARADA => Script::Sharada,
            ffi::PANGO_SCRIPT_SORA_SOMPENG => Script::SoraSompeng,
            ffi::PANGO_SCRIPT_TAKRI => Script::Takri,
            ffi::PANGO_SCRIPT_BASSA_VAH => Script::BassaVah,
            ffi::PANGO_SCRIPT_CAUCASIAN_ALBANIAN => Script::CaucasianAlbanian,
            ffi::PANGO_SCRIPT_DUPLOYAN => Script::Duployan,
            ffi::PANGO_SCRIPT_ELBASAN => Script::Elbasan,
            ffi::PANGO_SCRIPT_GRANTHA => Script::Grantha,
            ffi::PANGO_SCRIPT_KHOJKI => Script::Khojki,
            ffi::PANGO_SCRIPT_KHUDAWADI => Script::Khudawadi,
            ffi::PANGO_SCRIPT_LINEAR_A => Script::LinearA,
            ffi::PANGO_SCRIPT_MAHAJANI => Script::Mahajani,
            ffi::PANGO_SCRIPT_MANICHAEAN => Script::Manichaean,
            ffi::PANGO_SCRIPT_MENDE_KIKAKUI => Script::MendeKikakui,
            ffi::PANGO_SCRIPT_MODI => Script::Modi,
            ffi::PANGO_SCRIPT_MRO => Script::Mro,
            ffi::PANGO_SCRIPT_NABATAEAN => Script::Nabataean,
            ffi::PANGO_SCRIPT_OLD_NORTH_ARABIAN => Script::OldNorthArabian,
            ffi::PANGO_SCRIPT_OLD_PERMIC => Script::OldPermic,
            ffi::PANGO_SCRIPT_PAHAWH_HMONG => Script::PahawhHmong,
            ffi::PANGO_SCRIPT_PALMYRENE => Script::Palmyrene,
            ffi::PANGO_SCRIPT_PAU_CIN_HAU => Script::PauCinHau,
            ffi::PANGO_SCRIPT_PSALTER_PAHLAVI => Script::PsalterPahlavi,
            ffi::PANGO_SCRIPT_SIDDHAM => Script::Siddham,
            ffi::PANGO_SCRIPT_TIRHUTA => Script::Tirhuta,
            ffi::PANGO_SCRIPT_WARANG_CITI => Script::WarangCiti,
            ffi::PANGO_SCRIPT_AHOM => Script::Ahom,
            ffi::PANGO_SCRIPT_ANATOLIAN_HIEROGLYPHS => Script::AnatolianHieroglyphs,
            ffi::PANGO_SCRIPT_HATRAN => Script::Hatran,
            ffi::PANGO_SCRIPT_MULTANI => Script::Multani,
            ffi::PANGO_SCRIPT_OLD_HUNGARIAN => Script::OldHungarian,
            ffi::PANGO_SCRIPT_SIGNWRITING => Script::Signwriting,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Stretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for Stretch {
    type GlibType = ffi::PangoStretch;

    fn to_glib(&self) -> ffi::PangoStretch {
        match *self {
            Stretch::UltraCondensed => ffi::PANGO_STRETCH_ULTRA_CONDENSED,
            Stretch::ExtraCondensed => ffi::PANGO_STRETCH_EXTRA_CONDENSED,
            Stretch::Condensed => ffi::PANGO_STRETCH_CONDENSED,
            Stretch::SemiCondensed => ffi::PANGO_STRETCH_SEMI_CONDENSED,
            Stretch::Normal => ffi::PANGO_STRETCH_NORMAL,
            Stretch::SemiExpanded => ffi::PANGO_STRETCH_SEMI_EXPANDED,
            Stretch::Expanded => ffi::PANGO_STRETCH_EXPANDED,
            Stretch::ExtraExpanded => ffi::PANGO_STRETCH_EXTRA_EXPANDED,
            Stretch::UltraExpanded => ffi::PANGO_STRETCH_ULTRA_EXPANDED,
            Stretch::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoStretch> for Stretch {
    fn from_glib(value: ffi::PangoStretch) -> Self {
        match value {
            ffi::PANGO_STRETCH_ULTRA_CONDENSED => Stretch::UltraCondensed,
            ffi::PANGO_STRETCH_EXTRA_CONDENSED => Stretch::ExtraCondensed,
            ffi::PANGO_STRETCH_CONDENSED => Stretch::Condensed,
            ffi::PANGO_STRETCH_SEMI_CONDENSED => Stretch::SemiCondensed,
            ffi::PANGO_STRETCH_NORMAL => Stretch::Normal,
            ffi::PANGO_STRETCH_SEMI_EXPANDED => Stretch::SemiExpanded,
            ffi::PANGO_STRETCH_EXPANDED => Stretch::Expanded,
            ffi::PANGO_STRETCH_EXTRA_EXPANDED => Stretch::ExtraExpanded,
            ffi::PANGO_STRETCH_ULTRA_EXPANDED => Stretch::UltraExpanded,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Style {
    Normal,
    Oblique,
    Italic,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for Style {
    type GlibType = ffi::PangoStyle;

    fn to_glib(&self) -> ffi::PangoStyle {
        match *self {
            Style::Normal => ffi::PANGO_STYLE_NORMAL,
            Style::Oblique => ffi::PANGO_STYLE_OBLIQUE,
            Style::Italic => ffi::PANGO_STYLE_ITALIC,
            Style::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoStyle> for Style {
    fn from_glib(value: ffi::PangoStyle) -> Self {
        match value {
            ffi::PANGO_STYLE_NORMAL => Style::Normal,
            ffi::PANGO_STYLE_OBLIQUE => Style::Oblique,
            ffi::PANGO_STYLE_ITALIC => Style::Italic,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Variant {
    Normal,
    SmallCaps,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for Variant {
    type GlibType = ffi::PangoVariant;

    fn to_glib(&self) -> ffi::PangoVariant {
        match *self {
            Variant::Normal => ffi::PANGO_VARIANT_NORMAL,
            Variant::SmallCaps => ffi::PANGO_VARIANT_SMALL_CAPS,
            Variant::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoVariant> for Variant {
    fn from_glib(value: ffi::PangoVariant) -> Self {
        match value {
            ffi::PANGO_VARIANT_NORMAL => Variant::Normal,
            ffi::PANGO_VARIANT_SMALL_CAPS => Variant::SmallCaps,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Weight {
    Thin,
    Ultralight,
    Light,
    Semilight,
    Book,
    Normal,
    Medium,
    Semibold,
    Bold,
    Ultrabold,
    Heavy,
    Ultraheavy,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for Weight {
    type GlibType = ffi::PangoWeight;

    fn to_glib(&self) -> ffi::PangoWeight {
        match *self {
            Weight::Thin => ffi::PANGO_WEIGHT_THIN,
            Weight::Ultralight => ffi::PANGO_WEIGHT_ULTRALIGHT,
            Weight::Light => ffi::PANGO_WEIGHT_LIGHT,
            Weight::Semilight => ffi::PANGO_WEIGHT_SEMILIGHT,
            Weight::Book => ffi::PANGO_WEIGHT_BOOK,
            Weight::Normal => ffi::PANGO_WEIGHT_NORMAL,
            Weight::Medium => ffi::PANGO_WEIGHT_MEDIUM,
            Weight::Semibold => ffi::PANGO_WEIGHT_SEMIBOLD,
            Weight::Bold => ffi::PANGO_WEIGHT_BOLD,
            Weight::Ultrabold => ffi::PANGO_WEIGHT_ULTRABOLD,
            Weight::Heavy => ffi::PANGO_WEIGHT_HEAVY,
            Weight::Ultraheavy => ffi::PANGO_WEIGHT_ULTRAHEAVY,
            Weight::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoWeight> for Weight {
    fn from_glib(value: ffi::PangoWeight) -> Self {
        match value {
            ffi::PANGO_WEIGHT_THIN => Weight::Thin,
            ffi::PANGO_WEIGHT_ULTRALIGHT => Weight::Ultralight,
            ffi::PANGO_WEIGHT_LIGHT => Weight::Light,
            ffi::PANGO_WEIGHT_SEMILIGHT => Weight::Semilight,
            ffi::PANGO_WEIGHT_BOOK => Weight::Book,
            ffi::PANGO_WEIGHT_NORMAL => Weight::Normal,
            ffi::PANGO_WEIGHT_MEDIUM => Weight::Medium,
            ffi::PANGO_WEIGHT_SEMIBOLD => Weight::Semibold,
            ffi::PANGO_WEIGHT_BOLD => Weight::Bold,
            ffi::PANGO_WEIGHT_ULTRABOLD => Weight::Ultrabold,
            ffi::PANGO_WEIGHT_HEAVY => Weight::Heavy,
            ffi::PANGO_WEIGHT_ULTRAHEAVY => Weight::Ultraheavy,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum WrapMode {
    Word,
    Char,
    WordChar,
    #[doc(hidden)]
    __Nonexhaustive(()),
}

#[doc(hidden)]
impl ToGlib for WrapMode {
    type GlibType = ffi::PangoWrapMode;

    fn to_glib(&self) -> ffi::PangoWrapMode {
        match *self {
            WrapMode::Word => ffi::PANGO_WRAP_WORD,
            WrapMode::Char => ffi::PANGO_WRAP_CHAR,
            WrapMode::WordChar => ffi::PANGO_WRAP_WORD_CHAR,
            WrapMode::__Nonexhaustive(_) => panic!(),
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::PangoWrapMode> for WrapMode {
    fn from_glib(value: ffi::PangoWrapMode) -> Self {
        match value {
            ffi::PANGO_WRAP_WORD => WrapMode::Word,
            ffi::PANGO_WRAP_CHAR => WrapMode::Char,
            ffi::PANGO_WRAP_WORD_CHAR => WrapMode::WordChar,
        }
    }
}
