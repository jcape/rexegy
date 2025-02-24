//! Exegy "Country" Code (feed group) support

use ref_cast::RefCast;

/// Exegy "country" code -- actually a feed ID group
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd, RefCast)]
#[repr(transparent)]
pub struct Id(rexegy_sys::XC_COUNTRY_ID);

/// A group of feeds
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Group {
    /// This group represents a country
    Country(Country),
    /// This group represents a global vendor feed
    Corporate(Corporate),
}

impl TryFrom<[u8; 2]> for Group {
    type Error = ();

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        Country::try_from(value).map_or_else(
            |_e| Corporate::try_from(value).map(Group::Corporate),
            |country| Ok(Group::Country(country)),
        )
    }
}

impl TryFrom<[i8; 2]> for Group {
    type Error = ();

    fn try_from(value: [i8; 2]) -> Result<Self, Self::Error> {
        Self::try_from([value[0] as u8, value[1] as u8])
    }
}

impl TryFrom<Id> for Group {
    type Error = ();

    fn try_from(value: Id) -> Result<Self, Self::Error> {
        Self::try_from(value.0.xcc_ch)
    }
}

/// An enumeration of country codes in use in Exegy
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(i16)]
pub enum Country {
    /// United Arab Emirates
    UnitedArabEmirates = i16::from_le_bytes(*b"AE"),
    /// Antigua And Barbuda
    AntiguaAndBarbuda = i16::from_le_bytes(*b"AG"),
    /// Anguilla
    Anguilla = i16::from_le_bytes(*b"AI"),
    /// Netherlands Antilles
    NetherlandsAntilles = i16::from_le_bytes(*b"AN"),
    /// Argentina
    Argentina = i16::from_le_bytes(*b"AR"),
    /// Austria
    Austria = i16::from_le_bytes(*b"AT"),
    /// Australia
    Australia = i16::from_le_bytes(*b"AU"),
    /// Barbados
    Barbados = i16::from_le_bytes(*b"BB"),
    /// Bangladesh
    Bangladesh = i16::from_le_bytes(*b"BD"),
    /// Belgium
    Belgium = i16::from_le_bytes(*b"BE"),
    /// Bahrain
    Bahrain = i16::from_le_bytes(*b"BH"),
    /// Bermuda
    Bermuda = i16::from_le_bytes(*b"BM"),
    /// Brazil
    Brazil = i16::from_le_bytes(*b"BR"),
    /// Bahamas
    Bahamas = i16::from_le_bytes(*b"BS"),
    /// Botswana
    Botswana = i16::from_le_bytes(*b"BW"),
    /// Benelux
    Benelux = i16::from_le_bytes(*b"BX"),
    /// Belize
    Belize = i16::from_le_bytes(*b"BZ"),
    /// Canada
    Canada = i16::from_le_bytes(*b"CA"),
    /// Congo
    Congo = i16::from_le_bytes(*b"CG"),
    /// Switzerland
    Switzerland = i16::from_le_bytes(*b"CH"),
    /// Cote D'ivoire
    CoteDivoire = i16::from_le_bytes(*b"CI"),
    /// Chile
    Chile = i16::from_le_bytes(*b"CL"),
    /// Cameroon
    Cameroon = i16::from_le_bytes(*b"CM"),
    /// China
    China = i16::from_le_bytes(*b"CN"),
    /// Colombia
    Colombia = i16::from_le_bytes(*b"CO"),
    /// Cuba
    Cuba = i16::from_le_bytes(*b"CU"),
    /// Curacao
    Curacao = i16::from_le_bytes(*b"CW"),
    /// Cyprus
    Cyprus = i16::from_le_bytes(*b"CY"),
    /// Czech Republic
    CzechRepublic = i16::from_le_bytes(*b"CZ"),
    /// Germany
    Germany = i16::from_le_bytes(*b"DE"),
    /// Denmark
    Denmark = i16::from_le_bytes(*b"DK"),
    /// Algeria
    Algeria = i16::from_le_bytes(*b"DZ"),
    /// Estonia
    Estonia = i16::from_le_bytes(*b"EE"),
    /// Egypt
    Egypt = i16::from_le_bytes(*b"EG"),
    /// Spain
    Spain = i16::from_le_bytes(*b"ES"),
    /// European Union
    EuropeanUnion = i16::from_le_bytes(*b"EU"),
    /// Finland
    Finland = i16::from_le_bytes(*b"FI"),
    /// Falkland Islands
    FalklandIslands = i16::from_le_bytes(*b"FK"),
    /// Faroe Islands
    FaroeIslands = i16::from_le_bytes(*b"FO"),
    /// France
    France = i16::from_le_bytes(*b"FR"),
    /// Gabon
    Gabon = i16::from_le_bytes(*b"GA"),
    /// United Kingdom
    UnitedKingdom = i16::from_le_bytes(*b"GB"),
    /// Guernsey
    Guernsey = i16::from_le_bytes(*b"GG"),
    /// Gibraltar
    Gibraltar = i16::from_le_bytes(*b"GI"),
    /// Greece
    Greece = i16::from_le_bytes(*b"GR"),
    /// Hong Kong
    HongKong = i16::from_le_bytes(*b"HK"),
    /// Croatia
    Croatia = i16::from_le_bytes(*b"HR"),
    /// Hungary
    Hungary = i16::from_le_bytes(*b"HU"),
    /// Indonesia
    Indonesia = i16::from_le_bytes(*b"ID"),
    /// Ireland
    Ireland = i16::from_le_bytes(*b"IE"),
    /// Israel
    Israel = i16::from_le_bytes(*b"IL"),
    /// India
    India = i16::from_le_bytes(*b"IN"),
    /// Iraq
    Iraq = i16::from_le_bytes(*b"IQ"),
    /// Iran
    Iran = i16::from_le_bytes(*b"IR"),
    /// Iceland
    Iceland = i16::from_le_bytes(*b"IS"),
    /// Italy
    Italy = i16::from_le_bytes(*b"IT"),
    /// Jersey
    Jersey = i16::from_le_bytes(*b"JE"),
    /// Jamaica
    Jamaica = i16::from_le_bytes(*b"JM"),
    /// Jordan
    Jordan = i16::from_le_bytes(*b"JO"),
    /// Japan
    Japan = i16::from_le_bytes(*b"JP"),
    /// Kenya
    Kenya = i16::from_le_bytes(*b"KE"),
    /// Saint Kitts and Nevis
    SaintKittsAndNevis = i16::from_le_bytes(*b"KN"),
    /// Republic Of Korea
    RepublicOfKorea = i16::from_le_bytes(*b"KR"),
    /// Kuwait
    Kuwait = i16::from_le_bytes(*b"KW"),
    /// Cayman Islands
    CaymanIslands = i16::from_le_bytes(*b"KY"),
    /// Liechtenstein
    Liechtenstein = i16::from_le_bytes(*b"LI"),
    /// Liberia
    Liberia = i16::from_le_bytes(*b"LR"),
    /// Lesotho
    Lesotho = i16::from_le_bytes(*b"LS"),
    /// Lithuania
    Lithuania = i16::from_le_bytes(*b"LT"),
    /// Luxembourg
    Luxembourg = i16::from_le_bytes(*b"LU"),
    /// Latvia
    Latvia = i16::from_le_bytes(*b"LV"),
    /// Morocco
    Morocco = i16::from_le_bytes(*b"MA"),
    /// Monaco
    Monaco = i16::from_le_bytes(*b"MC"),
    /// Marshall Islands
    MarshallIslands = i16::from_le_bytes(*b"MH"),
    /// Malta
    Malta = i16::from_le_bytes(*b"MT"),
    /// Mauritius
    Mauritius = i16::from_le_bytes(*b"MU"),
    /// Mexico
    Mexico = i16::from_le_bytes(*b"MX"),
    /// Malaysia
    Malaysia = i16::from_le_bytes(*b"MY"),
    /// Namibia
    Namibia = i16::from_le_bytes(*b"NA"),
    /// Nigeria
    Nigeria = i16::from_le_bytes(*b"NG"),
    /// Netherlands
    Netherlands = i16::from_le_bytes(*b"NL"),
    /// Norway
    Norway = i16::from_le_bytes(*b"NO"),
    /// New Zealand
    NewZealand = i16::from_le_bytes(*b"NZ"),
    /// Panama
    Panama = i16::from_le_bytes(*b"PA"),
    /// Peru
    Peru = i16::from_le_bytes(*b"PE"),
    /// Papua New Guinea
    PapuaNewGuinea = i16::from_le_bytes(*b"PG"),
    /// Philippines
    Philippines = i16::from_le_bytes(*b"PH"),
    /// Pakistan
    Pakistan = i16::from_le_bytes(*b"PK"),
    /// Poland
    Poland = i16::from_le_bytes(*b"PL"),
    /// Puerto Rico
    PuertoRico = i16::from_le_bytes(*b"PR"),
    /// Portugal
    Portugal = i16::from_le_bytes(*b"PT"),
    /// Qatar
    Qatar = i16::from_le_bytes(*b"QA"),
    /// Kosovo
    Kosovo = i16::from_le_bytes(*b"QO"),
    /// Romania
    Romania = i16::from_le_bytes(*b"RO"),
    /// Russian Federation
    RussianFederation = i16::from_le_bytes(*b"RU"),
    /// Saudi Arabia
    SaudiArabia = i16::from_le_bytes(*b"SA"),
    /// Sweden
    Sweden = i16::from_le_bytes(*b"SE"),
    /// Singapore
    Singapore = i16::from_le_bytes(*b"SG"),
    /// Slovenia
    Slovenia = i16::from_le_bytes(*b"SI"),
    /// Slovak Republic
    SlovakRepublic = i16::from_le_bytes(*b"SK"),
    /// Senegal
    Senegal = i16::from_le_bytes(*b"SN"),
    /// Thailand
    Thailand = i16::from_le_bytes(*b"TH"),
    /// Turkey
    Turkey = i16::from_le_bytes(*b"TR"),
    /// Ukraine
    Ukraine = i16::from_le_bytes(*b"UA"),
    /// United States
    UnitedStates = i16::from_le_bytes(*b"US"),
    /// Venezuela
    Venezuela = i16::from_le_bytes(*b"VE"),
    /// British Virgin Islands
    BritishVirginIslands = i16::from_le_bytes(*b"VG"),
    /// VietNam
    VietNam = i16::from_le_bytes(*b"VN"),
    /// Pan-European
    PanEuropean = i16::from_le_bytes(*b"XS"),
    /// South Africa
    SouthAfrica = i16::from_le_bytes(*b"ZA"),
    /// Zambia
    Zambia = i16::from_le_bytes(*b"ZM"),
    /// Zimbabwe
    Zimbabwe = i16::from_le_bytes(*b"ZW"),
}

impl TryFrom<[u8; 2]> for Country {
    type Error = ();

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        match value {
            [b'A', b'E'] => Ok(Country::UnitedArabEmirates),
            [b'A', b'G'] => Ok(Country::AntiguaAndBarbuda),
            [b'A', b'I'] => Ok(Country::Anguilla),
            [b'A', b'N'] => Ok(Country::NetherlandsAntilles),
            [b'A', b'R'] => Ok(Country::Argentina),
            [b'A', b'T'] => Ok(Country::Austria),
            [b'A', b'U'] => Ok(Country::Australia),
            [b'B', b'B'] => Ok(Country::Barbados),
            [b'B', b'D'] => Ok(Country::Bangladesh),
            [b'B', b'E'] => Ok(Country::Belgium),
            [b'B', b'H'] => Ok(Country::Bahrain),
            [b'B', b'M'] => Ok(Country::Bermuda),
            [b'B', b'R'] => Ok(Country::Brazil),
            [b'B', b'S'] => Ok(Country::Bahamas),
            [b'B', b'W'] => Ok(Country::Botswana),
            [b'B', b'X'] => Ok(Country::Benelux),
            [b'B', b'Z'] => Ok(Country::Belize),
            [b'C', b'A'] => Ok(Country::Canada),
            [b'C', b'G'] => Ok(Country::Congo),
            [b'C', b'H'] => Ok(Country::Switzerland),
            [b'C', b'I'] => Ok(Country::CoteDivoire),
            [b'C', b'L'] => Ok(Country::Chile),
            [b'C', b'M'] => Ok(Country::Cameroon),
            [b'C', b'N'] => Ok(Country::China),
            [b'C', b'O'] => Ok(Country::Colombia),
            [b'C', b'U'] => Ok(Country::Cuba),
            [b'C', b'W'] => Ok(Country::Curacao),
            [b'C', b'Y'] => Ok(Country::Cyprus),
            [b'C', b'Z'] => Ok(Country::CzechRepublic),
            [b'D', b'E'] => Ok(Country::Germany),
            [b'D', b'K'] => Ok(Country::Denmark),
            [b'D', b'Z'] => Ok(Country::Algeria),
            [b'E', b'E'] => Ok(Country::Estonia),
            [b'E', b'G'] => Ok(Country::Egypt),
            [b'E', b'S'] => Ok(Country::Spain),
            [b'E', b'U'] => Ok(Country::EuropeanUnion),
            [b'F', b'I'] => Ok(Country::Finland),
            [b'F', b'K'] => Ok(Country::FalklandIslands),
            [b'F', b'O'] => Ok(Country::FaroeIslands),
            [b'F', b'R'] => Ok(Country::France),
            [b'G', b'A'] => Ok(Country::Gabon),
            [b'G', b'B'] => Ok(Country::UnitedKingdom),
            [b'G', b'G'] => Ok(Country::Guernsey),
            [b'G', b'I'] => Ok(Country::Gibraltar),
            [b'G', b'R'] => Ok(Country::Greece),
            [b'H', b'K'] => Ok(Country::HongKong),
            [b'H', b'R'] => Ok(Country::Croatia),
            [b'H', b'U'] => Ok(Country::Hungary),
            [b'I', b'D'] => Ok(Country::Indonesia),
            [b'I', b'E'] => Ok(Country::Ireland),
            [b'I', b'L'] => Ok(Country::Israel),
            [b'I', b'N'] => Ok(Country::India),
            [b'I', b'Q'] => Ok(Country::Iraq),
            [b'I', b'R'] => Ok(Country::Iran),
            [b'I', b'S'] => Ok(Country::Iceland),
            [b'I', b'T'] => Ok(Country::Italy),
            [b'J', b'E'] => Ok(Country::Jersey),
            [b'J', b'M'] => Ok(Country::Jamaica),
            [b'J', b'O'] => Ok(Country::Jordan),
            [b'J', b'P'] => Ok(Country::Japan),
            [b'K', b'E'] => Ok(Country::Kenya),
            [b'K', b'N'] => Ok(Country::SaintKittsAndNevis),
            [b'K', b'R'] => Ok(Country::RepublicOfKorea),
            [b'K', b'W'] => Ok(Country::Kuwait),
            [b'K', b'Y'] => Ok(Country::CaymanIslands),
            [b'L', b'I'] => Ok(Country::Liechtenstein),
            [b'L', b'R'] => Ok(Country::Liberia),
            [b'L', b'S'] => Ok(Country::Lesotho),
            [b'L', b'T'] => Ok(Country::Lithuania),
            [b'L', b'U'] => Ok(Country::Luxembourg),
            [b'L', b'V'] => Ok(Country::Latvia),
            [b'M', b'A'] => Ok(Country::Morocco),
            [b'M', b'C'] => Ok(Country::Monaco),
            [b'M', b'H'] => Ok(Country::MarshallIslands),
            [b'M', b'T'] => Ok(Country::Malta),
            [b'M', b'U'] => Ok(Country::Mauritius),
            [b'M', b'X'] => Ok(Country::Mexico),
            [b'M', b'Y'] => Ok(Country::Malaysia),
            [b'N', b'A'] => Ok(Country::Namibia),
            [b'N', b'G'] => Ok(Country::Nigeria),
            [b'N', b'L'] => Ok(Country::Netherlands),
            [b'N', b'O'] => Ok(Country::Norway),
            [b'N', b'Z'] => Ok(Country::NewZealand),
            [b'P', b'A'] => Ok(Country::Panama),
            [b'P', b'E'] => Ok(Country::Peru),
            [b'P', b'G'] => Ok(Country::PapuaNewGuinea),
            [b'P', b'H'] => Ok(Country::Philippines),
            [b'P', b'K'] => Ok(Country::Pakistan),
            [b'P', b'L'] => Ok(Country::Poland),
            [b'P', b'R'] => Ok(Country::PuertoRico),
            [b'P', b'T'] => Ok(Country::Portugal),
            [b'Q', b'A'] => Ok(Country::Qatar),
            [b'Q', b'O'] => Ok(Country::Kosovo),
            [b'R', b'O'] => Ok(Country::Romania),
            [b'R', b'U'] => Ok(Country::RussianFederation),
            [b'S', b'A'] => Ok(Country::SaudiArabia),
            [b'S', b'E'] => Ok(Country::Sweden),
            [b'S', b'G'] => Ok(Country::Singapore),
            [b'S', b'I'] => Ok(Country::Slovenia),
            [b'S', b'K'] => Ok(Country::SlovakRepublic),
            [b'S', b'N'] => Ok(Country::Senegal),
            [b'T', b'H'] => Ok(Country::Thailand),
            [b'T', b'R'] => Ok(Country::Turkey),
            [b'U', b'A'] => Ok(Country::Ukraine),
            [b'U', b'S'] => Ok(Country::UnitedStates),
            [b'V', b'E'] => Ok(Country::Venezuela),
            [b'V', b'G'] => Ok(Country::BritishVirginIslands),
            [b'V', b'N'] => Ok(Country::VietNam),
            [b'X', b'S'] => Ok(Country::PanEuropean),
            [b'Z', b'A'] => Ok(Country::SouthAfrica),
            [b'Z', b'M'] => Ok(Country::Zambia),
            [b'Z', b'W'] => Ok(Country::Zimbabwe),
            _ => Err(()),
        }
    }
}

impl TryFrom<[i8; 2]> for Country {
    type Error = ();

    fn try_from(value: [i8; 2]) -> Result<Self, Self::Error> {
        Self::try_from([value[0] as u8, value[1] as u8])
    }
}

impl TryFrom<Id> for Country {
    type Error = ();

    fn try_from(value: Id) -> Result<Self, Self::Error> {
        Self::try_from(value.0.xcc_ch)
    }
}

/// An enumeration of corporate feeds that Exegy calls "countries"
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(i16)]
pub enum Corporate {
    Aquis = i16::from_le_bytes(*b"AQ"),
    Barclays = i16::from_le_bytes(*b"BC"),
    Bgc = i16::from_le_bytes(*b"BG"),
    Citadel = i16::from_le_bytes(*b"CE"),
    Citi = i16::from_le_bytes(*b"CP"),
    Curex = i16::from_le_bytes(*b"CR"),
    CitadelEu = i16::from_le_bytes(*b"CS"),
    Currenex = i16::from_le_bytes(*b"CX"),
    DeutscheBank = i16::from_le_bytes(*b"DB"),
    UnknownDi = i16::from_le_bytes(*b"DI"),
    DealerWeb = i16::from_le_bytes(*b"DW"),
    UnknownDx = i16::from_le_bytes(*b"DX"),
    Ebs = i16::from_le_bytes(*b"EB"),
    EbsCreditScreened = i16::from_le_bytes(*b"EC"),
    Fenics = i16::from_le_bytes(*b"FC"),
    FastMatch = i16::from_le_bytes(*b"FM"),
    FlowTraders = i16::from_le_bytes(*b"FT"),
    ForeignExchange = i16::from_le_bytes(*b"FX"),
    Gfi = i16::from_le_bytes(*b"GF"),
    GainCapital = i16::from_le_bytes(*b"GT"),
    Hotspot = i16::from_le_bytes(*b"HF"),
    Missing1 = i16::from_le_bytes(*b"HS"),
    HudsonRiverTrading = i16::from_le_bytes(*b"HT"),
    Imc = i16::from_le_bytes(*b"IM"),
    IntegralFx = i16::from_le_bytes(*b"IN"),
    JaneStreet = i16::from_le_bytes(*b"JS"),
    Jump = i16::from_le_bytes(*b"JU"),
    Kcg = i16::from_le_bytes(*b"KG"),
    LiquidityEdge = i16::from_le_bytes(*b"LE"),
    Lmax = i16::from_le_bytes(*b"LH"),
    LmaxPro = i16::from_le_bytes(*b"LP"),
    MorganStanley = i16::from_le_bytes(*b"MS"),
    NasdaqOcean = i16::from_le_bytes(*b"NS"),
    ParFx = i16::from_le_bytes(*b"PX"),
    UnknownQq = i16::from_le_bytes(*b"QQ"),
    UnknownQs = i16::from_le_bytes(*b"QS"),
    UnknownQx = i16::from_le_bytes(*b"QX"),
    TestQy = i16::from_le_bytes(*b"QY"),
    TestQz = i16::from_le_bytes(*b"QZ"),
    Rbc = i16::from_le_bytes(*b"RB"),
    ThomsonReutersRfaGlobalData = i16::from_le_bytes(*b"RF"),
    SpotStream = i16::from_le_bytes(*b"SP"),
    StateStreet = i16::from_le_bytes(*b"ST"),
    Tower = i16::from_le_bytes(*b"TW"),
    Ubs = i16::from_le_bytes(*b"UB"),
    ExegyVirtu = i16::from_le_bytes(*b"VT"),
    BloombergEquities = i16::from_le_bytes(*b"XA"),
    BloombergCommodities = i16::from_le_bytes(*b"XB"),
    ExegyCustomerInternal = i16::from_le_bytes(*b"XC"),
    UnknownXd = i16::from_le_bytes(*b"XD"),
    Xtx = i16::from_le_bytes(*b"XT"),
    ExegyInternal = i16::from_le_bytes(*b"XX"),
    TestXy = i16::from_le_bytes(*b"XY"),
    TestXz = i16::from_le_bytes(*b"XZ"),
    UnknownZy = i16::from_le_bytes(*b"ZY"),
    TestZz = i16::from_le_bytes(*b"ZZ"),
}

impl TryFrom<[u8; 2]> for Corporate {
    type Error = ();

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        match value {
            [b'A', b'Q'] => Ok(Corporate::Aquis),
            [b'B', b'C'] => Ok(Corporate::Barclays),
            [b'B', b'G'] => Ok(Corporate::Bgc),
            [b'C', b'E'] => Ok(Corporate::Citadel),
            [b'C', b'P'] => Ok(Corporate::Citi),
            [b'C', b'R'] => Ok(Corporate::Curex),
            [b'C', b'S'] => Ok(Corporate::CitadelEu),
            [b'C', b'X'] => Ok(Corporate::Currenex),
            [b'D', b'B'] => Ok(Corporate::DeutscheBank),
            [b'D', b'I'] => Ok(Corporate::UnknownDi),
            [b'D', b'W'] => Ok(Corporate::DealerWeb),
            [b'D', b'X'] => Ok(Corporate::UnknownDx),
            [b'E', b'B'] => Ok(Corporate::Ebs),
            [b'E', b'C'] => Ok(Corporate::EbsCreditScreened),
            [b'F', b'C'] => Ok(Corporate::Fenics),
            [b'F', b'M'] => Ok(Corporate::FastMatch),
            [b'F', b'T'] => Ok(Corporate::FlowTraders),
            [b'F', b'X'] => Ok(Corporate::ForeignExchange),
            [b'G', b'F'] => Ok(Corporate::Gfi),
            [b'G', b'T'] => Ok(Corporate::GainCapital),
            [b'H', b'F'] => Ok(Corporate::Hotspot),
            [b'H', b'S'] => Ok(Corporate::Missing1),
            [b'H', b'T'] => Ok(Corporate::HudsonRiverTrading),
            [b'I', b'M'] => Ok(Corporate::Imc),
            [b'I', b'N'] => Ok(Corporate::IntegralFx),
            [b'J', b'S'] => Ok(Corporate::JaneStreet),
            [b'J', b'U'] => Ok(Corporate::Jump),
            [b'K', b'G'] => Ok(Corporate::Kcg),
            [b'L', b'E'] => Ok(Corporate::LiquidityEdge),
            [b'L', b'H'] => Ok(Corporate::Lmax),
            [b'L', b'P'] => Ok(Corporate::LmaxPro),
            [b'M', b'S'] => Ok(Corporate::MorganStanley),
            [b'N', b'S'] => Ok(Corporate::NasdaqOcean),
            [b'P', b'X'] => Ok(Corporate::ParFx),
            [b'Q', b'Q'] => Ok(Corporate::UnknownQq),
            [b'Q', b'S'] => Ok(Corporate::UnknownQs),
            [b'Q', b'X'] => Ok(Corporate::UnknownQx),
            [b'Q', b'Y'] => Ok(Corporate::TestQy),
            [b'Q', b'Z'] => Ok(Corporate::TestQz),
            [b'R', b'B'] => Ok(Corporate::Rbc),
            [b'R', b'F'] => Ok(Corporate::ThomsonReutersRfaGlobalData),
            [b'S', b'P'] => Ok(Corporate::SpotStream),
            [b'S', b'T'] => Ok(Corporate::StateStreet),
            [b'T', b'W'] => Ok(Corporate::Tower),
            [b'U', b'B'] => Ok(Corporate::Ubs),
            [b'V', b'T'] => Ok(Corporate::ExegyVirtu),
            [b'X', b'A'] => Ok(Corporate::BloombergEquities),
            [b'X', b'B'] => Ok(Corporate::BloombergCommodities),
            [b'X', b'C'] => Ok(Corporate::ExegyCustomerInternal),
            [b'X', b'D'] => Ok(Corporate::UnknownXd),
            [b'X', b'T'] => Ok(Corporate::Xtx),
            [b'X', b'X'] => Ok(Corporate::ExegyInternal),
            [b'X', b'Y'] => Ok(Corporate::TestXy),
            [b'X', b'Z'] => Ok(Corporate::TestXz),
            [b'Z', b'Y'] => Ok(Corporate::UnknownZy),
            [b'Z', b'Z'] => Ok(Corporate::TestZz),
            _ => Err(()),
        }
    }
}

impl TryFrom<[i8; 2]> for Corporate {
    type Error = ();

    fn try_from(value: [i8; 2]) -> Result<Self, Self::Error> {
        Self::try_from([value[0] as u8, value[1] as u8])
    }
}

impl TryFrom<i16> for Corporate {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Self::try_from(value.to_le_bytes())
    }
}

impl TryFrom<Id> for Corporate {
    type Error = ();

    fn try_from(value: Id) -> Result<Self, Self::Error> {
        Self::try_from(value.0.xcc_ch)
    }
}
