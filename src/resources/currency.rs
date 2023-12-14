use serde::{Deserialize, Serialize};

use crate::params::to_snakecase;

/// Currency is the list of supported currencies.
///
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub enum Currency {
    AED, // United Arab Emirates Dirham
    AFN, // Afghan Afghani
    ALL, // Albanian Lek
    AMD, // Armenian Dram
    ANG, // Netherlands Antillean Gulden
    AOA, // Angolan Kwanza
    ARS, // Argentine Peso
    AUD, // Australian Dollar
    AWG, // Aruban Florin
    AZN, // Azerbaijani Manat
    BAM, // Bosnia & Herzegovina Convertible Mark
    BBD, // Barbadian Dollar
    BDT, // Bangladeshi Taka
    BGN, // Bulgarian Lev
    BIF, // Burundian Franc
    BMD, // Bermudian Dollar
    BND, // Brunei Dollar
    BOB, // Bolivian Boliviano
    BRL, // Brazilian Real
    BSD, // Bahamian Dollar
    BWP, // Botswana Pula
    BZD, // Belize Dollar
    CAD, // Canadian Dollar
    CDF, // Congolese Franc
    CHF, // Swiss Franc
    CLP, // Chilean Peso
    CNY, // Chinese Renminbi Yuan
    COP, // Colombian Peso
    CRC, // Costa Rican Colón
    CVE, // Cape Verdean Escudo
    CZK, // Czech Koruna
    DJF, // Djiboutian Franc
    DKK, // Danish Krone
    DOP, // Dominican Peso
    DZD, // Algerian Dinar
    EEK, // Estonian Kroon
    EGP, // Egyptian Pound
    ETB, // Ethiopian Birr
    EUR, // Euro
    FJD, // Fijian Dollar
    FKP, // Falkland Islands Pound
    GBP, // British Pound
    GEL, // Georgian Lari
    GIP, // Gibraltar Pound
    GMD, // Gambian Dalasi
    GNF, // Guinean Franc
    GTQ, // Guatemalan Quetzal
    GYD, // Guyanese Dollar
    HKD, // Hong Kong Dollar
    HNL, // Honduran Lempira
    HRK, // Croatian Kuna
    HTG, // Haitian Gourde
    HUF, // Hungarian Forint
    IDR, // Indonesian Rupiah
    ILS, // Israeli New Sheqel
    INR, // Indian Rupee
    ISK, // Icelandic Króna
    JMD, // Jamaican Dollar
    JPY, // Japanese Yen
    KES, // Kenyan Shilling
    KGS, // Kyrgyzstani Som
    KHR, // Cambodian Riel
    KMF, // Comorian Franc
    KRW, // South Korean Won
    KYD, // Cayman Islands Dollar
    KZT, // Kazakhstani Tenge
    LAK, // Lao Kip
    LBP, // Lebanese Pound
    LKR, // Sri Lankan Rupee
    LRD, // Liberian Dollar
    LSL, // Lesotho Loti
    LTL, // Lithuanian Litas
    LVL, // Latvian Lats
    MAD, // Moroccan Dirham
    MDL, // Moldovan Leu
    MGA, // Malagasy Ariary
    MKD, // Macedonian Denar
    MNT, // Mongolian Tögrög
    MOP, // Macanese Pataca
    MRO, // Mauritanian Ouguiya
    MUR, // Mauritian Rupee
    MVR, // Maldivian Rufiyaa
    MWK, // Malawian Kwacha
    MXN, // Mexican Peso
    MYR, // Malaysian Ringgit
    MZN, // Mozambican Metical
    NAD, // Namibian Dollar
    NGN, // Nigerian Naira
    NIO, // Nicaraguan Córdoba
    NOK, // Norwegian Krone
    NPR, // Nepalese Rupee
    NZD, // New Zealand Dollar
    PAB, // Panamanian Balboa
    PEN, // Peruvian Nuevo Sol
    PGK, // Papua New Guinean Kina
    PHP, // Philippine Peso
    PKR, // Pakistani Rupee
    PLN, // Polish Złoty
    PYG, // Paraguayan Guaraní
    QAR, // Qatari Riyal
    RON, // Romanian Leu
    RSD, // Serbian Dinar
    RUB, // Russian Ruble
    RWF, // Rwandan Franc
    SAR, // Saudi Riyal
    SBD, // Solomon Islands Dollar
    SCR, // Seychellois Rupee
    SEK, // Swedish Krona
    SGD, // Singapore Dollar
    SHP, // Saint Helenian Pound
    SLL, // Sierra Leonean Leone
    SOS, // Somali Shilling
    SRD, // Surinamese Dollar
    STD, // São Tomé and Príncipe Dobra
    SVC, // Salvadoran Colón
    SZL, // Swazi Lilangeni
    THB, // Thai Baht
    TJS, // Tajikistani Somoni
    TOP, // Tongan Paʻanga
    TRY, // Turkish Lira
    TTD, // Trinidad and Tobago Dollar
    TWD, // New Taiwan Dollar
    TZS, // Tanzanian Shilling
    UAH, // Ukrainian Hryvnia
    UGX, // Ugandan Shilling
    USD, // United States Dollar
    UYU, // Uruguayan Peso
    UZS, // Uzbekistani Som
    VEF, // Venezuelan Bolívar
    VND, // Vietnamese Đồng
    VUV, // Vanuatu Vatu
    WST, // Samoan Tala
    XAF, // Central African Cfa Franc
    XCD, // East Caribbean Dollar
    XOF, // West African Cfa Franc
    XPF, // Cfp Franc
    YER, // Yemeni Rial
    ZAR, // South African Rand
    ZMW, // Zambian Kwacha
}

impl Default for Currency {
    fn default() -> Self {
        Currency::USD
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_snakecase(&format!("{:?}", self)))
    }
}

impl std::str::FromStr for Currency {
    type Err = ParseCurrencyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AED" => Ok(Currency::AED),
            "AFN" => Ok(Currency::AFN),
            "ALL" => Ok(Currency::ALL),
            "AMD" => Ok(Currency::AMD),
            "ANG" => Ok(Currency::ANG),
            "AOA" => Ok(Currency::AOA),
            "ARS" => Ok(Currency::ARS),
            "AUD" => Ok(Currency::AUD),
            "AWG" => Ok(Currency::AWG),
            "AZN" => Ok(Currency::AZN),
            "BAM" => Ok(Currency::BAM),
            "BBD" => Ok(Currency::BBD),
            "BDT" => Ok(Currency::BDT),
            "BGN" => Ok(Currency::BGN),
            "BIF" => Ok(Currency::BIF),
            "BMD" => Ok(Currency::BMD),
            "BND" => Ok(Currency::BND),
            "BOB" => Ok(Currency::BOB),
            "BRL" => Ok(Currency::BRL),
            "BSD" => Ok(Currency::BSD),
            "BWP" => Ok(Currency::BWP),
            "BZD" => Ok(Currency::BZD),
            "CAD" => Ok(Currency::CAD),
            "CDF" => Ok(Currency::CDF),
            "CHF" => Ok(Currency::CHF),
            "CLP" => Ok(Currency::CLP),
            "CNY" => Ok(Currency::CNY),
            "COP" => Ok(Currency::COP),
            "CRC" => Ok(Currency::CRC),
            "CVE" => Ok(Currency::CVE),
            "CZK" => Ok(Currency::CZK),
            "DJF" => Ok(Currency::DJF),
            "DKK" => Ok(Currency::DKK),
            "DOP" => Ok(Currency::DOP),
            "DZD" => Ok(Currency::DZD),
            "EEK" => Ok(Currency::EEK),
            "EGP" => Ok(Currency::EGP),
            "ETB" => Ok(Currency::ETB),
            "EUR" => Ok(Currency::EUR),
            "FJD" => Ok(Currency::FJD),
            "FKP" => Ok(Currency::FKP),
            "GBP" => Ok(Currency::GBP),
            "GEL" => Ok(Currency::GEL),
            "GIP" => Ok(Currency::GIP),
            "GMD" => Ok(Currency::GMD),
            "GNF" => Ok(Currency::GNF),
            "GTQ" => Ok(Currency::GTQ),
            "GYD" => Ok(Currency::GYD),
            "HKD" => Ok(Currency::HKD),
            "HNL" => Ok(Currency::HNL),
            "HRK" => Ok(Currency::HRK),
            "HTG" => Ok(Currency::HTG),
            "HUF" => Ok(Currency::HUF),
            "IDR" => Ok(Currency::IDR),
            "ILS" => Ok(Currency::ILS),
            "INR" => Ok(Currency::INR),
            "ISK" => Ok(Currency::ISK),
            "JMD" => Ok(Currency::JMD),
            "JPY" => Ok(Currency::JPY),
            "KES" => Ok(Currency::KES),
            "KGS" => Ok(Currency::KGS),
            "KHR" => Ok(Currency::KHR),
            "KMF" => Ok(Currency::KMF),
            "KRW" => Ok(Currency::KRW),
            "KYD" => Ok(Currency::KYD),
            "KZT" => Ok(Currency::KZT),
            "LAK" => Ok(Currency::LAK),
            "LBP" => Ok(Currency::LBP),
            "LKR" => Ok(Currency::LKR),
            "LRD" => Ok(Currency::LRD),
            "LSL" => Ok(Currency::LSL),
            "LTL" => Ok(Currency::LTL),
            "LVL" => Ok(Currency::LVL),
            "MAD" => Ok(Currency::MAD),
            "MDL" => Ok(Currency::MDL),
            "MGA" => Ok(Currency::MGA),
            "MKD" => Ok(Currency::MKD),
            "MNT" => Ok(Currency::MNT),
            "MOP" => Ok(Currency::MOP),
            "MRO" => Ok(Currency::MRO),
            "MUR" => Ok(Currency::MUR),
            "MVR" => Ok(Currency::MVR),
            "MWK" => Ok(Currency::MWK),
            "MXN" => Ok(Currency::MXN),
            "MYR" => Ok(Currency::MYR),
            "MZN" => Ok(Currency::MZN),
            "NAD" => Ok(Currency::NAD),
            "NGN" => Ok(Currency::NGN),
            "NIO" => Ok(Currency::NIO),
            "NOK" => Ok(Currency::NOK),
            "NPR" => Ok(Currency::NPR),
            "NZD" => Ok(Currency::NZD),
            "PAB" => Ok(Currency::PAB),
            "PEN" => Ok(Currency::PEN),
            "PGK" => Ok(Currency::PGK),
            "PHP" => Ok(Currency::PHP),
            "PKR" => Ok(Currency::PKR),
            "PLN" => Ok(Currency::PLN),
            "PYG" => Ok(Currency::PYG),
            "QAR" => Ok(Currency::QAR),
            "RON" => Ok(Currency::RON),
            "RSD" => Ok(Currency::RSD),
            "RUB" => Ok(Currency::RUB),
            "RWF" => Ok(Currency::RWF),
            "SAR" => Ok(Currency::SAR),
            "SBD" => Ok(Currency::SBD),
            "SCR" => Ok(Currency::SCR),
            "SEK" => Ok(Currency::SEK),
            "SGD" => Ok(Currency::SGD),
            "SHP" => Ok(Currency::SHP),
            "SLL" => Ok(Currency::SLL),
            "SOS" => Ok(Currency::SOS),
            "SRD" => Ok(Currency::SRD),
            "STD" => Ok(Currency::STD),
            "SVC" => Ok(Currency::SVC),
            "SZL" => Ok(Currency::SZL),
            "THB" => Ok(Currency::THB),
            "TJS" => Ok(Currency::TJS),
            "TOP" => Ok(Currency::TOP),
            "TRY" => Ok(Currency::TRY),
            "TTD" => Ok(Currency::TTD),
            "TWD" => Ok(Currency::TWD),
            "TZS" => Ok(Currency::TZS),
            "UAH" => Ok(Currency::UAH),
            "UGX" => Ok(Currency::UGX),
            "USD" => Ok(Currency::USD),
            "UYU" => Ok(Currency::UYU),
            "UZS" => Ok(Currency::UZS),
            "VEF" => Ok(Currency::VEF),
            "VND" => Ok(Currency::VND),
            "VUV" => Ok(Currency::VUV),
            "WST" => Ok(Currency::WST),
            "XAF" => Ok(Currency::XAF),
            "XCD" => Ok(Currency::XCD),
            "XOF" => Ok(Currency::XOF),
            "XPF" => Ok(Currency::XPF),
            "YER" => Ok(Currency::YER),
            "ZAR" => Ok(Currency::ZAR),
            "ZMW" => Ok(Currency::ZMW),
            _ => Err(ParseCurrencyError(())),
        }
    }
}

#[derive(Debug)]
pub struct ParseCurrencyError(/* private */ ());

impl std::fmt::Display for ParseCurrencyError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(deprecated)]
        fmt.write_str(::std::error::Error::description(self))
    }
}

impl std::error::Error for ParseCurrencyError {
    fn description(&self) -> &str {
        "unknown currency code"
    }
}
