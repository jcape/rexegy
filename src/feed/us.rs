//! US Feed Codes

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(i16)]
#[non_exhaustive]
pub enum Feed {
    /// NYSE MKT via CTA/UTP (Level 1)
    NyseMktViaSip = i16::from_le_bytes([b'A', 0]),
    /// NYSE Arca BBO (Level 1)
    ArcaBbo = i16::from_le_bytes([b'A', b'A']),
    /// NYSE ArcaBook (Level 2)
    ArcaBook = i16::from_le_bytes([b'A', b'B']),
    /// NYSE American Options Pillar - Complex (Level 1)
    AmexOptionsComplex = i16::from_le_bytes([b'A', b'C']),
    /// NYSE MKT OpenBook Ultra (Level 2 - price book)
    NyseMktOpenBookUltra = i16::from_le_bytes([b'A', b'D']),
    /// NYSE MKT Integrated (Level 2)
    NyseMktIntegrated = i16::from_le_bytes([b'A', b'G']),
    /// NYSE MKT Order Imbalances (Level 2 - price book)
    NyseMktImbalances = i16::from_le_bytes([b'A', b'I']),
    /// NYSE MKT Alerts (Level 1)
    NyseMktAlerts = i16::from_le_bytes([b'A', b'L']),
    /// NYSE American Options Pillar - Top (Level 1)
    AmexOptionsTop = i16::from_le_bytes([b'A', b'P']),
    /// NYSE Amex Trades (Level 1)
    AmexTrades = i16::from_le_bytes([b'A', b'T']),
    /// NYSE Arca Integrated (Level 2)
    ArcaIntegrated = i16::from_le_bytes([b'A', b'X']),
    /// NYSE American Options Pillar - Deep (Level 2)
    AmexOptionsDeep = i16::from_le_bytes([b'A', b'Z']),
    /// NYSE Amex Options via OPRA (Level 1)
    AmexOptionsOpra = i16::from_le_bytes([b'A', b'_']),
    /// Nasdaq BX via CTA/UTP (Level 1)
    B = i16::from_le_bytes([b'B', 0]),
    /// BOX Options Exchange via HSVF (Level 2)
    BB = i16::from_le_bytes([b'B', b'B']),
    /// Nasdaq BX Options Depth of Market (Level 2)
    BD = i16::from_le_bytes([b'B', b'D']),
    /// BOX Options Exchange via HSVF (Level 1)
    BO = i16::from_le_bytes([b'B', b'O']),
    /// Nasdaq BX Options Top of Market (Level 1)
    BP = i16::from_le_bytes([b'B', b'P']),
    /// Nasdaq BX MatchView (Level 1)
    BQ = i16::from_le_bytes([b'B', b'Q']),
    /// Nasdaq BX via TotalView (Level 2)
    BX = i16::from_le_bytes([b'B', b'X']),
    /// CBOT (Level 2)
    BY = i16::from_le_bytes([b'B', b'Y']),
    /// CBOT (Level 2)
    BZ = i16::from_le_bytes([b'B', b'Z']),
    /// BOX Options Exchange via OPRA (Level 1)
    B_ = i16::from_le_bytes([b'B', b'_']),
    /// National Stock Exchange via CTA/UTP (Level 1)
    C = i16::from_le_bytes([b'C', 0]),
    /// NSE National BBO (Level 1)
    CC = i16::from_le_bytes([b'C', b'C']),
    /// Citadel Execution Services IOI (Level 1)
    CE = i16::from_le_bytes([b'C', b'E']),
    /// Citadel Securities Fixed Income (Level 1)
    CF = i16::from_le_bytes([b'C', b'F']),
    /// CME Indices (Level 1)
    CI = i16::from_le_bytes([b'C', b'I']),
    /// CME Pit (Level 1)
    CM = i16::from_le_bytes([b'C', b'M']),
    /// Cboe C2 Options via PITCH (Level 2)
    CO = i16::from_le_bytes([b'C', b'O']),
    /// CBOE Market Data Indices (Level 1)
    CS = i16::from_le_bytes([b'C', b'S']),
    /// NSE National Trades (Level 1)
    CT = i16::from_le_bytes([b'C', b'T']),
    /// NSE National Integrated (Level 2)
    CX = i16::from_le_bytes([b'C', b'X']),
    /// CME (Level 2)
    CY = i16::from_le_bytes([b'C', b'Y']),
    /// CME (Level 2)
    CZ = i16::from_le_bytes([b'C', b'Z']),
    /// CBOE via OPRA (Level 1)
    C_ = i16::from_le_bytes([b'C', b'_']),
    /// FINRA ADF/TRF (Level 1)
    D = i16::from_le_bytes([b'D', 0]),
    /// FINRA / Nasdaq TRF Chicago (Level 1)
    DB = i16::from_le_bytes([b'D', b'B']),
    /// Dow Jones Indices (Level 1)
    DI = i16::from_le_bytes([b'D', b'I']),
    /// FINRA / NYSE TRF (Level 1)
    DN = i16::from_le_bytes([b'D', b'N']),
    /// FINRA / Nasdaq TRF Carteret (Level 1)
    DQ = i16::from_le_bytes([b'D', b'Q']),
    /// MIAX Emerald via OPRA (Level 1)
    D_ = i16::from_le_bytes([b'D', b'_']),
    /// IEX (Level 1)
    E = i16::from_le_bytes([b'E', 0]),
    /// Bats EDGX Options via OPRA (Level 1)
    ED = i16::from_le_bytes([b'E', b'D']),
    /// COMEX Pit (Level 1)
    EF = i16::from_le_bytes([b'E', b'F']),
    /// MIAX Emerald via Tom/AIS (Level 1)
    EM = i16::from_le_bytes([b'E', b'M']),
    /// eSpeed ITCH (Level 2)
    ES = i16::from_le_bytes([b'E', b'S']),
    /// Bats EDGX Options via Multicast (Level 2)
    EX = i16::from_le_bytes([b'E', b'X']),
    /// Bats EDGX Options via OPRA (Level 1)
    E_ = i16::from_le_bytes([b'E', b'_']),
    /// Fenics UST BIMP - Visibility Group 1 (Level 2)
    FC = i16::from_le_bytes([b'F', b'C']),
    /// Fenics UST BIMP - Global (Level 2)
    FG = i16::from_le_bytes([b'F', b'G']),
    /// Fenics UST BIMP - Visibility Group 2 (Level 2)
    FN = i16::from_le_bytes([b'F', b'N']),
    /// Cboe Futures Exchange via PITCH Level 2
    FO = i16::from_le_bytes([b'F', b'O']),
    /// Cboe Futures Exchange via TOP Level 1
    FT = i16::from_le_bytes([b'F', b'T']),
    /// Fenics UST BIMP - Custom (Level 2)
    FX = i16::from_le_bytes([b'F', b'X']),
    /// Nasdaq Global Index Dissemination Service (Level 1)
    GD = i16::from_le_bytes([b'G', b'D']),
    /// NYSE Global Index Feed (Level 1)
    GI = i16::from_le_bytes([b'G', b'I']),
    /// ISE Gemini via OPRA (Level 1)
    H = i16::from_le_bytes([b'H', 0]),
    /// Miax PEARL Equities via DOM (Level 2)
    HD = i16::from_le_bytes([b'H', b'D']),
    /// Nasdaq GEMX Options Top of Market (Level 1)
    HH = i16::from_le_bytes([b'H', b'H']),
    /// Miax PEARL Equities via TOM (Level 1)
    HT = i16::from_le_bytes([b'H', b'T']),
    /// Hanweck Risk (Level 2)
    HW = i16::from_le_bytes([b'H', b'W']),
    /// Nasdaq GEMX Options Depth of Market (Level 2)
    HX = i16::from_le_bytes([b'H', b'X']),
    /// ISE Gemini via OPRA (Level 1)
    H_ = i16::from_le_bytes([b'H', b'_']),
    /// International Securities Exchange via CTA/UTP (Level 1)
    I = i16::from_le_bytes([b'I', 0]),
    /// FINRA ADF / IntelligentCross (Level 1)
    IA = i16::from_le_bytes([b'I', b'A']),
    /// Intelligent Cross ASPEN (Level 2)
    IC = i16::from_le_bytes([b'I', b'C']),
    /// IEX DEEP (Level 2 - price book)
    ID = i16::from_le_bytes([b'I', b'D']),
    /// IEX TOPS (Level 1)
    IE = i16::from_le_bytes([b'I', b'E']),
    /// Nasdaq ISE Options Top of Market (Level 1)
    II = i16::from_le_bytes([b'I', b'I']),
    /// Intelligent Cross ASPEN Maker/Taker (Level 2)
    IM = i16::from_le_bytes([b'I', b'M']),
    /// Nasdaq ISE Options Depth of Market (Level 2)
    IO = i16::from_le_bytes([b'I', b'O']),
    /// ICE US, Market by Price (Level 2)
    IP = i16::from_le_bytes([b'I', b'P']),
    /// ICE US, Market by Price (non-full implied) (Level 2)
    IQ = i16::from_le_bytes([b'I', b'Q']),
    /// International Securities Exchange via T7 - Test (Level 2)
    IS = i16::from_le_bytes([b'I', b'S']),
    /// Intelligent Cross ASPEN Taker/Maker (Level 2)
    IT = i16::from_le_bytes([b'I', b'T']),
    /// International Securities Exchange via INET (Level 2)
    IX = i16::from_le_bytes([b'I', b'X']),
    /// ICE US, Market by Order (Level 2)
    IY = i16::from_le_bytes([b'I', b'Y']),
    /// ICE US, Market by Order (non-full implied) (Level 2)
    IZ = i16::from_le_bytes([b'I', b'Z']),
    /// International Securities Exchange via OPRA (Level 1)
    I_ = i16::from_le_bytes([b'I', b'_']),
    /// Bats EDGA via CTA/UTP (Level 1)
    J = i16::from_le_bytes([b'J', 0]),
    /// ISE Mercury via OPRA (Level 1)
    JM = i16::from_le_bytes([b'J', b'M']),
    /// Nasdaq MRX Options Depth of Market (Level 2)
    JO = i16::from_le_bytes([b'J', b'O']),
    /// Jane Street IOI (Level 1)
    JS = i16::from_le_bytes([b'J', b'S']),
    /// JP Morgan US Treasuries (Level 2)
    JT = i16::from_le_bytes([b'J', b'T']),
    /// Bats EDGA via Multicast (Level 2)
    JX = i16::from_le_bytes([b'J', b'X']),
    /// ISE Mercury via OPRA (Level 1)
    J_ = i16::from_le_bytes([b'J', b'_']),
    /// Bats EDGX via CTA/UTP (Level 1)
    K = i16::from_le_bytes([b'K', 0]),
    /// KCG Americas Equity IOI (Level 1)
    KG = i16::from_le_bytes([b'K', b'G']),
    /// Bats EDGX via Multicast (Level 2)
    KX = i16::from_le_bytes([b'K', b'X']),
    /// KCBT (Level 2)
    KY = i16::from_le_bytes([b'K', b'Y']),
    /// Long-Term Stock Exchange via CTA/UTP (Level 1)
    L = i16::from_le_bytes([b'L', 0]),
    /// Credit Suisse Light Pool (Level 2)
    LH = i16::from_le_bytes([b'L', b'H']),
    /// ICE Liffe US, Market by Order (Level 2)
    LI = i16::from_le_bytes([b'L', b'I']),
    /// ICE Liffe US, Market by Order (non-full implied) (Level 2)
    LJ = i16::from_le_bytes([b'L', b'J']),
    /// LTSE MEMOIR Top of Book (Level 1)
    LL = i16::from_le_bytes([b'L', b'L']),
    /// ICE Liffe US, Market by Price (Level 2)
    LP = i16::from_le_bytes([b'L', b'P']),
    /// ICE Liffe US, Market by Price (non-full implied) (Level 2)
    LR = i16::from_le_bytes([b'L', b'R']),
    /// LTSE MEMOIR Last Sale (Level 1)
    LT = i16::from_le_bytes([b'L', b'T']),
    /// NYSE Liffe US, Top (Level 1)
    LU = i16::from_le_bytes([b'L', b'U']),
    /// LTSE MEMOIR Depth (Level 2)
    LX = i16::from_le_bytes([b'L', b'X']),
    /// NYSE Liffe US, Depth (Level 2 - price book)
    LY = i16::from_le_bytes([b'L', b'Y']),
    /// Chicago Stock Exchange via CTA/UTP (Level 1)
    M = i16::from_le_bytes([b'M', 0]),
    /// MIAX Emerald via OPRA (Level 1)
    MD = i16::from_le_bytes([b'M', b'D']),
    /// MIAX Options via OPRA (Level 1)
    MI = i16::from_le_bytes([b'M', b'I']),
    /// NYSE Chicago BBO (Level 1)
    MM = i16::from_le_bytes([b'M', b'M']),
    /// MIAX Options via ToM/AIS (Level 1)
    MO = i16::from_le_bytes([b'M', b'O']),
    /// NYSE Chicago Trades (Level 1)
    MT = i16::from_le_bytes([b'M', b'T']),
    /// NYSE Chicago Integrated (Level 2)
    MX = i16::from_le_bytes([b'M', b'X']),
    /// MGEX (Level 2)
    MY = i16::from_le_bytes([b'M', b'Y']),
    /// MGEX (Level 2)
    MZ = i16::from_le_bytes([b'M', b'Z']),
    /// MIAX Options via OPRA (Level 1)
    M_ = i16::from_le_bytes([b'M', b'_']),
    /// NYSE via CTA/UTP (Level 1)
    N = i16::from_le_bytes([b'N', 0]),
    /// NYSE MKT BBO (Level 1)
    NA = i16::from_le_bytes([b'N', b'A']),
    /// NYSE Arca Options (Level 1)
    NC = i16::from_le_bytes([b'N', b'C']),
    /// Nasdaq Fixed Income ITCH (Level 2)
    NF = i16::from_le_bytes([b'N', b'F']),
    /// NYSE Integrated (Level 2)
    NG = i16::from_le_bytes([b'N', b'G']),
    /// NYSE Order Imbalances (Level 2 - price book)
    NI = i16::from_le_bytes([b'N', b'I']),
    /// NYSE Alerts (Level 1)
    NL = i16::from_le_bytes([b'N', b'L']),
    /// NYSE BBO (Level 1)
    NN = i16::from_le_bytes([b'N', b'N']),
    /// NYSE TRF (Level 1)
    NR = i16::from_le_bytes([b'N', b'R']),
    /// NYSE Trades (Level 1)
    NT = i16::from_le_bytes([b'N', b'T']),
    /// NYSE via OpenBook Ultra (Level 2 - price book)
    NX = i16::from_le_bytes([b'N', b'X']),
    /// NYMEX (Level 2)
    NY = i16::from_le_bytes([b'N', b'Y']),
    /// NYMEX (Level 2)
    NZ = i16::from_le_bytes([b'N', b'Z']),
    /// MIAX PEARL via OPRA (Level 1)
    O = i16::from_le_bytes([b'O', 0]),
    /// Nasdaq OTC Montage Data Feed (Level 1)
    OD = i16::from_le_bytes([b'O', b'D']),
    /// OTC Markets via Quote Inside - Expert (Level 1)
    OE = i16::from_le_bytes([b'O', b'E']),
    /// OTC Markets via Quote Inside (Level 1)
    OT = i16::from_le_bytes([b'O', b'T']),
    /// CBOE BZX Options via Multicast (Level 2)
    OZ = i16::from_le_bytes([b'O', b'Z']),
    /// MIAX PEARL via OPRA (Level 1)
    O_ = i16::from_le_bytes([b'O', b'_']),
    /// NYSE Arca via CTA/UTP (Level 1)
    P = i16::from_le_bytes([b'P', 0]),
    /// Nasdaq PHLX Depth of Market (Level 2)
    PD = i16::from_le_bytes([b'P', b'D']),
    /// OTC Markets via Quote Book - Expert (Level 2)
    PE = i16::from_le_bytes([b'P', b'E']),
    /// NYSE Arca Imbalances (Level 2 - price book)
    PI = i16::from_le_bytes([b'P', b'I']),
    /// MIAX PEARL via Tom/AIS (Level 1)
    PM = i16::from_le_bytes([b'P', b'M']),
    /// Nasdaq PHLX Orders (Level 2)
    PO = i16::from_le_bytes([b'P', b'O']),
    /// Top of PHLX Options (Level 1)
    PP = i16::from_le_bytes([b'P', b'P']),
    /// Nasdaq PSX MatchView (Level 1)
    PQ = i16::from_le_bytes([b'P', b'Q']),
    /// OTC Markets (Level 1)
    PS = i16::from_le_bytes([b'P', b'S']),
    /// NYSE Arca Trades (Level 1)
    PT = i16::from_le_bytes([b'P', b'T']),
    /// OTC Markets via Quote Book (Level 2)
    PX = i16::from_le_bytes([b'P', b'X']),
    /// NYSE Arca Options via OPRA (Level 1)
    P_ = i16::from_le_bytes([b'P', b'_']),
    /// Nasdaq via CTA/UTP (Level 1)
    Q = i16::from_le_bytes([b'Q', 0]),
    /// Quincy CBOT Equity Futures (Level 2)
    QB = i16::from_le_bytes([b'Q', b'B']),
    /// Quincy CME Equity Futures (Level 2)
    QC = i16::from_le_bytes([b'Q', b'C']),
    /// Nasdaq Trade Reporting Facility (Level 1)
    QF = i16::from_le_bytes([b'Q', b'F']),
    /// Best of Nasdaq Options (Level 1)
    QP = i16::from_le_bytes([b'Q', b'P']),
    /// Nasdaq Execution System (Level 1)
    QT = i16::from_le_bytes([b'Q', b'T']),
    /// Nasdaq Options Market ITTO (Level 2)
    QX = i16::from_le_bytes([b'Q', b'X']),
    /// Nasdaq Options Market via OPRA (Level 1)
    Q_ = i16::from_le_bytes([b'Q', b'_']),
    /// NYSE Arca Options Pillar - Deep (Level 2)
    RD = i16::from_le_bytes([b'R', b'D']),
    /// NYSE Arca Options Pillar - Top - Imbalances (Level 1)
    RI = i16::from_le_bytes([b'R', b'I']),
    /// NYSE Arca Options Pillar - Top - BBO (Level 1)
    RR = i16::from_le_bytes([b'R', b'R']),
    /// NYSE Arca Options Pillar - Top - Trades (Level 1)
    RT = i16::from_le_bytes([b'R', b'T']),
    /// Members Exchange via CTA/UTP (Level 1)
    S = i16::from_le_bytes([b'S', 0]),
    /// MIAX Sapphire via ToM (Level 1)
    SH = i16::from_le_bytes([b'S', b'H']),
    /// Sun IOI (Level 1)
    SN = i16::from_le_bytes([b'S', b'N']),
    /// MEMX Top (Level 1)
    SS = i16::from_le_bytes([b'S', b'S']),
    /// MEMX Last Sale (Level 1)
    ST = i16::from_le_bytes([b'S', b'T']),
    /// MEMX Depth (Level 2)
    SX = i16::from_le_bytes([b'S', b'X']),
    /// MIAX Sapphire via OPRA (Level 1)
    S_ = i16::from_le_bytes([b'S', b'_']),
    /// Nasdaq BX Options via OPRA (Level 1)
    T = i16::from_le_bytes([b'T', 0]),
    /// Nasdaq via TotalView (Level 2)
    TX = i16::from_le_bytes([b'T', b'X']),
    /// CME BrokerTec (Level 2)
    TY = i16::from_le_bytes([b'T', b'Y']),
    /// CME BrokerTec (Level 2)
    TZ = i16::from_le_bytes([b'T', b'Z']),
    /// Nasdaq BX Options via OPRA (Level 1)
    T_ = i16::from_le_bytes([b'T', b'_']),
    /// OTC Bulletin Board (Level 1)
    U = i16::from_le_bytes([b'U', 0]),
    /// MEMX Options Depth (Level 2)
    UZ = i16::from_le_bytes([b'U', b'Z']),
    /// Members Exchange MEMX via OPRA (Level 1)
    U_ = i16::from_le_bytes([b'U', b'_']),
    /// OTC Other (Level 1)
    V = i16::from_le_bytes([b'V', 0]),
    /// NASDAQ Market Velocity and Forces (Level 1)
    VF = i16::from_le_bytes([b'V', b'F']),
    /// Virtu Fixed Income (Level 2)
    VI = i16::from_le_bytes([b'V', b'I']),
    /// Virtu Financial IOI (Level 1)
    VU = i16::from_le_bytes([b'V', b'U']),
    /// CBOE Stock Exchange (Level 1)
    W = i16::from_le_bytes([b'W', 0]),
    /// Cboe C1 Options via PITCH (Level 2)
    WO = i16::from_le_bytes([b'W', b'O']),
    /// CBOE Equities C2 (Level 1)
    WW = i16::from_le_bytes([b'W', b'W']),
    /// CBOE C2 via OPRA (Level 1)
    W_ = i16::from_le_bytes([b'W', b'_']),
    /// Nasdaq PSX via CTA/UTP (Level 1)
    X = i16::from_le_bytes([b'X', 0]),
    /// CME ITC (Level 1)
    XF = i16::from_le_bytes([b'X', b'F']),
    /// CME ITC S&P 100 and S&P 500 (Level 1)
    XL = i16::from_le_bytes([b'X', b'L']),
    /// Nasdaq PHLX (Level 1)
    XO = i16::from_le_bytes([b'X', b'O']),
    /// CME ITC (Level 1)
    XW = i16::from_le_bytes([b'X', b'W']),
    /// Nasdaq PSX via TotalView (Level 2)
    XX = i16::from_le_bytes([b'X', b'X']),
    /// COMEX (Level 2)
    XY = i16::from_le_bytes([b'X', b'Y']),
    /// COMEX (Level 2)
    XZ = i16::from_le_bytes([b'X', b'Z']),
    /// Nasdaq PHLX via OPRA (Level 1)
    X_ = i16::from_le_bytes([b'X', b'_']),
    /// Bats BYX via CTA/UTP (Level 1)
    Y = i16::from_le_bytes([b'Y', 0]),
    /// Bats BYX via Multicast (Level 2)
    YX = i16::from_le_bytes([b'Y', b'X']),
    /// Bats BZX via CTA/UTP (Level 1)
    Z = i16::from_le_bytes([b'Z', 0]),
    /// Bats BZX (Level 1)
    ZQ = i16::from_le_bytes([b'Z', b'Q']),
    /// Bats BZX via Multicast (Level 2)
    ZX = i16::from_le_bytes([b'Z', b'X']),
    /// Bats BZX Options via OPRA (Level 1)
    Z_ = i16::from_le_bytes([b'Z', b'_']),
    ///  (Quote Montage (Level 1)
    QuoteMontage = i16::from_le_bytes([b'\\', b'N']),
    ///  (Symbol Reference - US (Level 2)
    SymbolReference = i16::from_le_bytes([b'\\', b'R']),
    ///  (User-defined Composite (Level 1) Exegy generated
    UserComposite = i16::from_le_bytes([b'\\', b'U']),
    /// NBBO (Level 1) Equity
    Nbbo = i16::from_le_bytes([b'\\', b'\\']),
    /// Customer Proprietary Feed - Connection 1 (Level 2)
    _1 = i16::from_le_bytes([b'_', b'1']),
    /// Customer Proprietary Feed - Connection 2 (Level 2)
    _2 = i16::from_le_bytes([b'_', b'2']),
    /// Customer Proprietary Feed - Connection 3 (Level 2)
    _3 = i16::from_le_bytes([b'_', b'3']),
    /// Customer Proprietary Feed - Connection 4 (Level 2)
    _4 = i16::from_le_bytes([b'_', b'4']),
    /// Customer Proprietary Feed - Connection 5 (Level 2)
    _5 = i16::from_le_bytes([b'_', b'5']),
    /// Customer Proprietary Feed - Connection 6 (Level 2)
    _6 = i16::from_le_bytes([b'_', b'6']),
    /// Customer Proprietary Feed - Connection 7 (Level 2)
    _7 = i16::from_le_bytes([b'_', b'7']),
    /// Customer Proprietary Feed - Connection 8 (Level 2)
    _8 = i16::from_le_bytes([b'_', b'8']),
}
