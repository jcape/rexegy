//! US Feed Codes

use crate::group::{Country, Group};

use super::{Feed as FeedTrait, Id};

// US feed codes used by Exegy
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
    AmexOptionsPillarTop = i16::from_le_bytes([b'A', b'P']),
    /// NYSE Amex Trades (Level 1)
    AmexTrades = i16::from_le_bytes([b'A', b'T']),
    /// NYSE Arca Integrated (Level 2)
    ArcaIntegrated = i16::from_le_bytes([b'A', b'X']),
    /// NYSE American Options Pillar - Deep (Level 2)
    AmexOptionsPillarDeep = i16::from_le_bytes([b'A', b'Z']),
    /// NYSE Amex Options via OPRA (Level 1)
    AmexOptionsViaOpra = i16::from_le_bytes([b'A', b'_']),
    /// Nasdaq BX via CTA/UTP (Level 1)
    BxViaSip = i16::from_le_bytes([b'B', 0]),
    /// BOX Options Exchange via HSVF (Level 2)
    BoxLevel2ViaHsvf = i16::from_le_bytes([b'B', b'B']),
    /// Nasdaq BX Options Depth of Market (Level 2)
    BxOptionsDepthOfMarket = i16::from_le_bytes([b'B', b'D']),
    /// BOX Options Exchange via HSVF (Level 1)
    BoxLevel1ViaHsvf = i16::from_le_bytes([b'B', b'O']),
    /// Nasdaq BX Options Top of Market (Level 1)
    BxOptionsTopOfMarket = i16::from_le_bytes([b'B', b'P']),
    /// Nasdaq BX MatchView (Level 1)
    BxMatchView = i16::from_le_bytes([b'B', b'Q']),
    /// Nasdaq BX via TotalView (Level 2)
    BxTotalView = i16::from_le_bytes([b'B', b'X']),
    /// CBOT (Level 2)
    CbotY = i16::from_le_bytes([b'B', b'Y']),
    /// CBOT (Level 2)
    CbotZ = i16::from_le_bytes([b'B', b'Z']),
    /// BOX Options Exchange via OPRA (Level 1)
    BoxViaOpra = i16::from_le_bytes([b'B', b'_']),
    /// National Stock Exchange via CTA/UTP (Level 1)
    NyseNationalViaSip = i16::from_le_bytes([b'C', 0]),
    /// NSE National BBO (Level 1)
    NyseNationalBbo = i16::from_le_bytes([b'C', b'C']),
    /// Citadel Execution Services IOI (Level 1)
    CitadelIoi = i16::from_le_bytes([b'C', b'E']),
    /// Citadel Securities Fixed Income (Level 1)
    CitadelFixedIncome = i16::from_le_bytes([b'C', b'F']),
    /// CME Indices (Level 1)
    CmeIndices = i16::from_le_bytes([b'C', b'I']),
    /// CME Pit (Level 1)
    CmePit = i16::from_le_bytes([b'C', b'M']),
    /// Cboe C2 Options via PITCH (Level 2)
    CboeC2ViaPitch = i16::from_le_bytes([b'C', b'O']),
    /// CBOE Market Data Indices (Level 1)
    CboeIndices = i16::from_le_bytes([b'C', b'S']),
    /// NSE National Trades (Level 1)
    NyseNationalTrades = i16::from_le_bytes([b'C', b'T']),
    /// NSE National Integrated (Level 2)
    NyseNationalDepth = i16::from_le_bytes([b'C', b'X']),
    /// CME (Level 2)
    CmeY = i16::from_le_bytes([b'C', b'Y']),
    /// CME (Level 2)
    CmeZ = i16::from_le_bytes([b'C', b'Z']),
    /// CBOE via OPRA (Level 1)
    CboeViaOpra = i16::from_le_bytes([b'C', b'_']),
    /// FINRA ADF/TRF (Level 1)
    FinraViaSip = i16::from_le_bytes([b'D', 0]),
    /// FINRA / Nasdaq TRF Chicago (Level 1)
    NasdaqChicagoTrf = i16::from_le_bytes([b'D', b'B']),
    /// Dow Jones Indices (Level 1)
    DowIndices = i16::from_le_bytes([b'D', b'I']),
    /// FINRA / NYSE TRF (Level 1)
    NyseTrf = i16::from_le_bytes([b'D', b'N']),
    /// FINRA / Nasdaq TRF Carteret (Level 1)
    NasdaqCarteretTrf = i16::from_le_bytes([b'D', b'Q']),
    /// MIAX Emerald via OPRA (Level 1)
    MiaxEmeraldViaOpra = i16::from_le_bytes([b'D', b'_']),
    /// IEX (Level 1)
    IexViaSip = i16::from_le_bytes([b'E', 0]),
    /// Bats EDGX Options via OPRA (Level 1)
    EdgxOptionsViaOpra2 = i16::from_le_bytes([b'E', b'D']),
    /// COMEX Pit (Level 1)
    ComexPit = i16::from_le_bytes([b'E', b'F']),
    /// MIAX Emerald via Tom/AIS (Level 1)
    MiaxEmeraldViaTomAis = i16::from_le_bytes([b'E', b'M']),
    /// eSpeed ITCH (Level 2)
    EspeedItch = i16::from_le_bytes([b'E', b'S']),
    /// Bats EDGX Options via Multicast (Level 2)
    EdgxOptionsViaMulticast = i16::from_le_bytes([b'E', b'X']),
    /// Bats EDGX Options via OPRA (Level 1)
    EdgxOptionsViaOpra = i16::from_le_bytes([b'E', b'_']),
    /// Fenics UST BIMP - Visibility Group 1 (Level 2)
    FenicsGroup1 = i16::from_le_bytes([b'F', b'C']),
    /// Fenics UST BIMP - Global (Level 2)
    FenicsGlobal = i16::from_le_bytes([b'F', b'G']),
    /// Fenics UST BIMP - Visibility Group 2 (Level 2)
    FenicsGroup2 = i16::from_le_bytes([b'F', b'N']),
    /// Cboe Futures Exchange via PITCH Level 2
    CboeFuturesPitch = i16::from_le_bytes([b'F', b'O']),
    /// Cboe Futures Exchange via TOP Level 1
    CboeFuturesTop = i16::from_le_bytes([b'F', b'T']),
    /// Fenics UST BIMP - Custom (Level 2)
    FenicsCustom = i16::from_le_bytes([b'F', b'X']),
    /// Nasdaq Global Index Dissemination Service (Level 1)
    NasdaqGids = i16::from_le_bytes([b'G', b'D']),
    /// NYSE Global Index Feed (Level 1)
    NyseGlobalIndexes = i16::from_le_bytes([b'G', b'I']),
    /// ISE Gemini via OPRA (Level 1)
    GeminiOpraLegacy = i16::from_le_bytes([b'H', 0]),
    /// Miax PEARL Equities via DOM (Level 2)
    MiaxPerlViaDom = i16::from_le_bytes([b'H', b'D']),
    /// Nasdaq GEMX Options Top of Market (Level 1)
    NasdaqGemxTopOfMarket = i16::from_le_bytes([b'H', b'H']),
    /// Miax PEARL Equities via TOM (Level 1)
    MiaxPerlViaTom = i16::from_le_bytes([b'H', b'T']),
    /// Hanweck Risk (Level 2)
    HanweckRisk = i16::from_le_bytes([b'H', b'W']),
    /// Nasdaq GEMX Options Depth of Market (Level 2)
    NasdaqGemxDepthOfMarket = i16::from_le_bytes([b'H', b'X']),
    /// ISE Gemini via OPRA (Level 1)
    GeminiViaOpra = i16::from_le_bytes([b'H', b'_']),
    /// International Securities Exchange via CTA/UTP (Level 1)
    IseViaSip = i16::from_le_bytes([b'I', 0]),
    /// FINRA ADF / IntelligentCross (Level 1)
    IntelligentCrossAdf = i16::from_le_bytes([b'I', b'A']),
    /// Intelligent Cross ASPEN (Level 2)
    IntelligentCrossAspen = i16::from_le_bytes([b'I', b'C']),
    /// IEX DEEP (Level 2 - price book)
    IexDeep = i16::from_le_bytes([b'I', b'D']),
    /// IEX TOPS (Level 1)
    IexTops = i16::from_le_bytes([b'I', b'E']),
    /// Nasdaq ISE Options Top of Market (Level 1)
    NasdaqIseTopOfMarket = i16::from_le_bytes([b'I', b'I']),
    /// Intelligent Cross ASPEN Maker/Taker (Level 2)
    IntelligentCrossAspenMakerTaker = i16::from_le_bytes([b'I', b'M']),
    /// Nasdaq ISE Options Depth of Market (Level 2)
    NasdaqIseOptionsDepthOfMarket = i16::from_le_bytes([b'I', b'O']),
    /// ICE US, Market by Price (Level 2)
    IceUsMarketByPrice = i16::from_le_bytes([b'I', b'P']),
    /// ICE US, Market by Price (non-full implied) (Level 2)
    IceUsMarketByPriceNonFullImplied = i16::from_le_bytes([b'I', b'Q']),
    /// International Securities Exchange via T7 - Test (Level 2)
    IseT7 = i16::from_le_bytes([b'I', b'S']),
    /// Intelligent Cross ASPEN Taker/Maker (Level 2)
    IntelligentCrossTakerMaker = i16::from_le_bytes([b'I', b'T']),
    /// International Securities Exchange via INET (Level 2)
    IseInet = i16::from_le_bytes([b'I', b'X']),
    /// ICE US, Market by Order (Level 2)
    IceUsMarketByOrder = i16::from_le_bytes([b'I', b'Y']),
    /// ICE US, Market by Order (non-full implied) (Level 2)
    IceUsMarketByOrderNonFullImplied = i16::from_le_bytes([b'I', b'Z']),
    /// International Securities Exchange via OPRA (Level 1)
    IseViaOpra = i16::from_le_bytes([b'I', b'_']),
    /// Bats EDGA via CTA/UTP (Level 1)
    EdgaViaSip = i16::from_le_bytes([b'J', 0]),
    /// ISE Mercury via OPRA (Level 1)
    MercuryViaOpraLegacy = i16::from_le_bytes([b'J', b'M']),
    /// Nasdaq MRX Options Depth of Market (Level 2)
    NasdaqMrxOptionsDepthOfMarket = i16::from_le_bytes([b'J', b'O']),
    /// Jane Street IOI (Level 1)
    JaneStreetIoi = i16::from_le_bytes([b'J', b'S']),
    /// JP Morgan US Treasuries (Level 2)
    JpmUsTreasuries = i16::from_le_bytes([b'J', b'T']),
    /// Bats EDGA via Multicast (Level 2)
    EdgaViaMulticast = i16::from_le_bytes([b'J', b'X']),
    /// ISE Mercury via OPRA (Level 1)
    MercuryViaOpra = i16::from_le_bytes([b'J', b'_']),
    /// Bats EDGX via CTA/UTP (Level 1)
    EdgxViaSip = i16::from_le_bytes([b'K', 0]),
    /// KCG Americas Equity IOI (Level 1)
    KcgEquityIoi = i16::from_le_bytes([b'K', b'G']),
    /// Bats EDGX via Multicast (Level 2)
    EdgxViaMulticast = i16::from_le_bytes([b'K', b'X']),
    /// KCBT (Level 2)
    Kcbt = i16::from_le_bytes([b'K', b'Y']),
    /// Long-Term Stock Exchange via CTA/UTP (Level 1)
    LtseViaSip = i16::from_le_bytes([b'L', 0]),
    /// Credit Suisse Light Pool (Level 2)
    LightPoolDepth = i16::from_le_bytes([b'L', b'H']),
    /// ICE Liffe US, Market by Order (Level 2)
    LiffeUsMarketByOrder = i16::from_le_bytes([b'L', b'I']),
    /// ICE Liffe US, Market by Order (non-full implied) (Level 2)
    LiffeUsMarketByOrderNonFullImplied = i16::from_le_bytes([b'L', b'J']),
    /// LTSE MEMOIR Top of Book (Level 1)
    LtseMemoirTopOfBook = i16::from_le_bytes([b'L', b'L']),
    /// ICE Liffe US, Market by Price (Level 2)
    LiffeUsMarketByPrice = i16::from_le_bytes([b'L', b'P']),
    /// ICE Liffe US, Market by Price (non-full implied) (Level 2)
    LiffeUsMarketByPriceNonFullImplied = i16::from_le_bytes([b'L', b'R']),
    /// LTSE MEMOIR Last Sale (Level 1)
    LtseMemoirLastSale = i16::from_le_bytes([b'L', b'T']),
    /// NYSE Liffe US, Top (Level 1)
    LiffeUsTop = i16::from_le_bytes([b'L', b'U']),
    /// LTSE MEMOIR Depth (Level 2)
    LtseMemoirDepth = i16::from_le_bytes([b'L', b'X']),
    /// NYSE Liffe US, Depth (Level 2 - price book)
    LiffeUsDepth = i16::from_le_bytes([b'L', b'Y']),
    /// Chicago Stock Exchange via CTA/UTP (Level 1)
    NyseChicagoViaSip = i16::from_le_bytes([b'M', 0]),
    /// MIAX Emerald via OPRA (Level 1)
    MiaxEmeraldViaOpraLegacy = i16::from_le_bytes([b'M', b'D']),
    /// MIAX Options via OPRA (Level 1)
    MiaxViaOpraLegacy = i16::from_le_bytes([b'M', b'I']),
    /// NYSE Chicago BBO (Level 1)
    NyseChicagoBbo = i16::from_le_bytes([b'M', b'M']),
    /// MIAX Options via ToM/AIS (Level 1)
    MiaxViaTomAis = i16::from_le_bytes([b'M', b'O']),
    /// NYSE Chicago Trades (Level 1)
    NyseChicagoTrades = i16::from_le_bytes([b'M', b'T']),
    /// NYSE Chicago Integrated (Level 2)
    NyseChicagoIntegrated = i16::from_le_bytes([b'M', b'X']),
    /// MGEX (Level 2)
    MgexY = i16::from_le_bytes([b'M', b'Y']),
    /// MGEX (Level 2)
    MgexZ = i16::from_le_bytes([b'M', b'Z']),
    /// MIAX Options via OPRA (Level 1)
    MiaxViaOpra = i16::from_le_bytes([b'M', b'_']),
    /// NYSE via CTA/UTP (Level 1)
    NyseViaSip = i16::from_le_bytes([b'N', 0]),
    /// NYSE MKT BBO (Level 1)
    NyseMktBbo = i16::from_le_bytes([b'N', b'A']),
    /// NYSE Arca Options (Level 1)
    NyseArcaOptionsLevel1 = i16::from_le_bytes([b'N', b'C']),
    /// Nasdaq Fixed Income ITCH (Level 2)
    NasdaqFixedIncomeItch = i16::from_le_bytes([b'N', b'F']),
    /// NYSE Integrated (Level 2)
    NyseIntegrated = i16::from_le_bytes([b'N', b'G']),
    /// NYSE Order Imbalances (Level 2 - price book)
    NyseImbalances = i16::from_le_bytes([b'N', b'I']),
    /// NYSE Alerts (Level 1)
    NyseAlerts = i16::from_le_bytes([b'N', b'L']),
    /// NYSE BBO (Level 1)
    NyseBbo = i16::from_le_bytes([b'N', b'N']),
    /// NYSE TRF (Level 1)
    NyseTrfLevel1 = i16::from_le_bytes([b'N', b'R']),
    /// NYSE Trades (Level 1)
    NyseTrades = i16::from_le_bytes([b'N', b'T']),
    /// NYSE via OpenBook Ultra (Level 2 - price book)
    NyseOpenBookUltra = i16::from_le_bytes([b'N', b'X']),
    /// NYMEX (Level 2)
    NymexY = i16::from_le_bytes([b'N', b'Y']),
    /// NYMEX (Level 2)
    NymexZ = i16::from_le_bytes([b'N', b'Z']),
    /// MIAX PEARL via OPRA (Level 1)
    MiaxPearlViaOpraLegacy = i16::from_le_bytes([b'O', 0]),
    /// Nasdaq OTC Montage Data Feed (Level 1)
    NasdaqOtcMontage = i16::from_le_bytes([b'O', b'D']),
    /// OTC Markets via Quote Inside - Expert (Level 1)
    OtcMarketsViaQuoteInsideExpert = i16::from_le_bytes([b'O', b'E']),
    /// OTC Markets via Quote Inside (Level 1)
    OtcMarketsViaQuoteInside = i16::from_le_bytes([b'O', b'T']),
    /// CBOE BZX Options via Multicast (Level 2)
    BzxOptionsViaMulticast = i16::from_le_bytes([b'O', b'Z']),
    /// MIAX PEARL via OPRA (Level 1)
    MiaxPearlViaOpra = i16::from_le_bytes([b'O', b'_']),
    /// NYSE Arca via CTA/UTP (Level 1)
    ArcaViaSip = i16::from_le_bytes([b'P', 0]),
    /// Nasdaq PHLX Depth of Market (Level 2)
    NasdaqPhlxDepth = i16::from_le_bytes([b'P', b'D']),
    /// OTC Markets via Quote Book - Expert (Level 2)
    OtcMarketsViaQuoteBookExpert = i16::from_le_bytes([b'P', b'E']),
    /// NYSE Arca Imbalances (Level 2 - price book)
    ArcaImbalances = i16::from_le_bytes([b'P', b'I']),
    /// MIAX PEARL via Tom/AIS (Level 1)
    PearlViaTomAis = i16::from_le_bytes([b'P', b'M']),
    /// Nasdaq PHLX Orders (Level 2)
    PhlxOrders = i16::from_le_bytes([b'P', b'O']),
    /// Top of PHLX Options (Level 1)
    PhlxOptionsTop = i16::from_le_bytes([b'P', b'P']),
    /// Nasdaq PSX MatchView (Level 1)
    PsxMatchView = i16::from_le_bytes([b'P', b'Q']),
    /// OTC Markets (Level 1)
    OtcMarkets = i16::from_le_bytes([b'P', b'S']),
    /// NYSE Arca Trades (Level 1)
    ArcaTrades = i16::from_le_bytes([b'P', b'T']),
    /// OTC Markets via Quote Book (Level 2)
    OtcMarketsViaQuoteBook = i16::from_le_bytes([b'P', b'X']),
    /// NYSE Arca Options via OPRA (Level 1)
    ArcaOptionsViaOpra = i16::from_le_bytes([b'P', b'_']),
    /// Nasdaq via CTA/UTP (Level 1)
    NasdaqViaSip = i16::from_le_bytes([b'Q', 0]),
    /// Quincy CBOT Equity Futures (Level 2)
    QuincyCbotEquityFutures = i16::from_le_bytes([b'Q', b'B']),
    /// Quincy CME Equity Futures (Level 2)
    QuincyCmeEquityFutures = i16::from_le_bytes([b'Q', b'C']),
    /// Nasdaq Trade Reporting Facility (Level 1)
    NasdaqTrf = i16::from_le_bytes([b'Q', b'F']),
    /// Best of Nasdaq Options (Level 1)
    BestOfNasdaqOptions = i16::from_le_bytes([b'Q', b'P']),
    /// Nasdaq Execution System (Level 1)
    NasdaqExecutionSystem = i16::from_le_bytes([b'Q', b'T']),
    /// Nasdaq Options Market ITTO (Level 2)
    NasdaqOptionsItto = i16::from_le_bytes([b'Q', b'X']),
    /// Nasdaq Options Market via OPRA (Level 1)
    NasdaqOptionsViaOpra = i16::from_le_bytes([b'Q', b'_']),
    /// NYSE Arca Options Pillar - Deep (Level 2)
    ArcaOptionsDeep = i16::from_le_bytes([b'R', b'D']),
    /// NYSE Arca Options Pillar - Top - Imbalances (Level 1)
    ArcaOptionsImbalances = i16::from_le_bytes([b'R', b'I']),
    /// NYSE Arca Options Pillar - Top - BBO (Level 1)
    ArcaOptionsBbo = i16::from_le_bytes([b'R', b'R']),
    /// NYSE Arca Options Pillar - Top - Trades (Level 1)
    ArcaOptionsTrades = i16::from_le_bytes([b'R', b'T']),
    /// Members Exchange via CTA/UTP (Level 1)
    MemxViaSip = i16::from_le_bytes([b'S', 0]),
    /// MIAX Sapphire via ToM (Level 1)
    SapphireViaTom = i16::from_le_bytes([b'S', b'H']),
    /// Sun IOI (Level 1)
    SunIoi = i16::from_le_bytes([b'S', b'N']),
    /// MEMX Top (Level 1)
    MemxTop = i16::from_le_bytes([b'S', b'S']),
    /// MEMX Last Sale (Level 1)
    MemxLastSale = i16::from_le_bytes([b'S', b'T']),
    /// MEMX Depth (Level 2)
    MemxDepth = i16::from_le_bytes([b'S', b'X']),
    /// MIAX Sapphire via OPRA (Level 1)
    SapphireViaOpra = i16::from_le_bytes([b'S', b'_']),
    /// Nasdaq BX Options via OPRA (Level 1)
    BxOptionsViaOpraLegacy = i16::from_le_bytes([b'T', 0]),
    /// Nasdaq via TotalView (Level 2)
    NasdaqViaTotalView = i16::from_le_bytes([b'T', b'X']),
    /// CME BrokerTec (Level 2)
    CmeBrokerTecY = i16::from_le_bytes([b'T', b'Y']),
    /// CME BrokerTec (Level 2)
    CmeBrokerTecZ = i16::from_le_bytes([b'T', b'Z']),
    /// Nasdaq BX Options via OPRA (Level 1)
    BxOptionsViaOpra = i16::from_le_bytes([b'T', b'_']),
    /// OTC Bulletin Board (Level 1)
    OtcBulletinBoard = i16::from_le_bytes([b'U', 0]),
    /// MEMX Options Depth (Level 2)
    MemxOptionsDepth = i16::from_le_bytes([b'U', b'Z']),
    /// Members Exchange MEMX via OPRA (Level 1)
    MemxOptionsViaOpra = i16::from_le_bytes([b'U', b'_']),
    /// OTC Other (Level 1)
    OtcOther = i16::from_le_bytes([b'V', 0]),
    /// NASDAQ Market Velocity and Forces (Level 1)
    NasdasqMarketVelocityAndForces = i16::from_le_bytes([b'V', b'F']),
    /// Virtu Fixed Income (Level 2)
    VirtuFixedIncome = i16::from_le_bytes([b'V', b'I']),
    /// Virtu Financial IOI (Level 1)
    VirtuIoi = i16::from_le_bytes([b'V', b'U']),
    /// CBOE Stock Exchange (Level 1)
    CboeStock = i16::from_le_bytes([b'W', 0]),
    /// Cboe C1 Options via PITCH (Level 2)
    CboeC1ViaPitch = i16::from_le_bytes([b'W', b'O']),
    /// CBOE Equities C2 (Level 1)
    CboeEquitiesC2 = i16::from_le_bytes([b'W', b'W']),
    /// CBOE C2 via OPRA (Level 1)
    CboeC2ViaOpra = i16::from_le_bytes([b'W', b'_']),
    /// Nasdaq PSX via CTA/UTP (Level 1)
    PsxViaSip = i16::from_le_bytes([b'X', 0]),
    /// CME ITC (Level 1)
    CmeItcF = i16::from_le_bytes([b'X', b'F']),
    /// CME ITC S&P 100 and S&P 500 (Level 1)
    CmeItcSp100Sp500 = i16::from_le_bytes([b'X', b'L']),
    /// Nasdaq PHLX (Level 1)
    PhlxLevel1 = i16::from_le_bytes([b'X', b'O']),
    /// CME ITC (Level 1)
    CmeItcW = i16::from_le_bytes([b'X', b'W']),
    /// Nasdaq PSX via TotalView (Level 2)
    PsxTotalView = i16::from_le_bytes([b'X', b'X']),
    /// COMEX (Level 2)
    ComexY = i16::from_le_bytes([b'X', b'Y']),
    /// COMEX (Level 2)
    ComexZ = i16::from_le_bytes([b'X', b'Z']),
    /// Nasdaq PHLX via OPRA (Level 1)
    PhxlOptionsViaOpra = i16::from_le_bytes([b'X', b'_']),
    /// Bats BYX via CTA/UTP (Level 1)
    ByxViaSip = i16::from_le_bytes([b'Y', 0]),
    /// Bats BYX via Multicast (Level 2)
    ByxViaMulticast = i16::from_le_bytes([b'Y', b'X']),
    /// Bats BZX via CTA/UTP (Level 1)
    BzxViaSip = i16::from_le_bytes([b'Z', 0]),
    /// Bats BZX (Level 1)
    BzxLevel1 = i16::from_le_bytes([b'Z', b'Q']),
    /// Bats BZX via Multicast (Level 2)
    BzxLevel2 = i16::from_le_bytes([b'Z', b'X']),
    /// Bats BZX Options via OPRA (Level 1)
    BzxOptionsViaOpra = i16::from_le_bytes([b'Z', b'_']),
    ///  (Quote Montage (Level 1)
    QuoteMontage = i16::from_le_bytes([b'\\', b'N']),
    ///  (Symbol Reference - US (Level 2)
    SymbolReference = i16::from_le_bytes([b'\\', b'R']),
    ///  (User-defined Composite (Level 1) Exegy generated
    UserComposite = i16::from_le_bytes([b'\\', b'U']),
    /// NBBO (Level 1) Equity
    Nbbo = i16::from_le_bytes([b'\\', b'\\']),
    /// Customer Proprietary Feed - Connection 1 (Level 2)
    Connection1 = i16::from_le_bytes([b'_', b'1']),
    /// Customer Proprietary Feed - Connection 2 (Level 2)
    Connection2 = i16::from_le_bytes([b'_', b'2']),
    /// Customer Proprietary Feed - Connection 3 (Level 2)
    Connection3 = i16::from_le_bytes([b'_', b'3']),
    /// Customer Proprietary Feed - Connection 4 (Level 2)
    Connection4 = i16::from_le_bytes([b'_', b'4']),
    /// Customer Proprietary Feed - Connection 5 (Level 2)
    Connection5 = i16::from_le_bytes([b'_', b'5']),
    /// Customer Proprietary Feed - Connection 6 (Level 2)
    Connection6 = i16::from_le_bytes([b'_', b'6']),
    /// Customer Proprietary Feed - Connection 7 (Level 2)
    Connection7 = i16::from_le_bytes([b'_', b'7']),
    /// Customer Proprietary Feed - Connection 8 (Level 2)
    Connection8 = i16::from_le_bytes([b'_', b'8']),
}

impl TryFrom<[u8; 2]> for Feed {
    type Error = ();

    fn try_from(value: [u8; 2]) -> Result<Self, Self::Error> {
        match value {
            [b'A', 0] => Ok(Feed::NyseMktViaSip),
            [b'A', b'A'] => Ok(Feed::ArcaBbo),
            [b'A', b'B'] => Ok(Feed::ArcaBook),
            [b'A', b'C'] => Ok(Feed::AmexOptionsComplex),
            [b'A', b'D'] => Ok(Feed::NyseMktOpenBookUltra),
            [b'A', b'G'] => Ok(Feed::NyseMktIntegrated),
            [b'A', b'I'] => Ok(Feed::NyseMktImbalances),
            [b'A', b'L'] => Ok(Feed::NyseMktAlerts),
            [b'A', b'P'] => Ok(Feed::AmexOptionsPillarTop),
            [b'A', b'T'] => Ok(Feed::AmexTrades),
            [b'A', b'X'] => Ok(Feed::ArcaIntegrated),
            [b'A', b'Z'] => Ok(Feed::AmexOptionsPillarDeep),
            [b'A', b'_'] => Ok(Feed::AmexOptionsViaOpra),
            [b'B', 0] => Ok(Feed::BxViaSip),
            [b'B', b'B'] => Ok(Feed::BoxLevel2ViaHsvf),
            [b'B', b'D'] => Ok(Feed::BxOptionsDepthOfMarket),
            [b'B', b'O'] => Ok(Feed::BoxLevel1ViaHsvf),
            [b'B', b'P'] => Ok(Feed::BxOptionsTopOfMarket),
            [b'B', b'Q'] => Ok(Feed::BxMatchView),
            [b'B', b'X'] => Ok(Feed::BxTotalView),
            [b'B', b'Y'] => Ok(Feed::CbotY),
            [b'B', b'Z'] => Ok(Feed::CbotZ),
            [b'B', b'_'] => Ok(Feed::BoxViaOpra),
            [b'C', 0] => Ok(Feed::NyseNationalViaSip),
            [b'C', b'C'] => Ok(Feed::NyseNationalBbo),
            [b'C', b'E'] => Ok(Feed::CitadelIoi),
            [b'C', b'F'] => Ok(Feed::CitadelFixedIncome),
            [b'C', b'I'] => Ok(Feed::CmeIndices),
            [b'C', b'M'] => Ok(Feed::CmePit),
            [b'C', b'O'] => Ok(Feed::CboeC2ViaPitch),
            [b'C', b'S'] => Ok(Feed::CboeIndices),
            [b'C', b'T'] => Ok(Feed::NyseNationalTrades),
            [b'C', b'X'] => Ok(Feed::NyseNationalDepth),
            [b'C', b'Y'] => Ok(Feed::CmeY),
            [b'C', b'Z'] => Ok(Feed::CmeZ),
            [b'C', b'_'] => Ok(Feed::CboeViaOpra),
            [b'D', 0] => Ok(Feed::FinraViaSip),
            [b'D', b'B'] => Ok(Feed::NasdaqChicagoTrf),
            [b'D', b'I'] => Ok(Feed::DowIndices),
            [b'D', b'N'] => Ok(Feed::NyseTrf),
            [b'D', b'Q'] => Ok(Feed::NasdaqCarteretTrf),
            [b'D', b'_'] => Ok(Feed::MiaxEmeraldViaOpra),
            [b'E', 0] => Ok(Feed::IexViaSip),
            [b'E', b'D'] => Ok(Feed::EdgxOptionsViaOpra2),
            [b'E', b'F'] => Ok(Feed::ComexPit),
            [b'E', b'M'] => Ok(Feed::MiaxEmeraldViaTomAis),
            [b'E', b'S'] => Ok(Feed::EspeedItch),
            [b'E', b'X'] => Ok(Feed::EdgxOptionsViaMulticast),
            [b'E', b'_'] => Ok(Feed::EdgxOptionsViaOpra),
            [b'F', b'C'] => Ok(Feed::FenicsGroup1),
            [b'F', b'G'] => Ok(Feed::FenicsGlobal),
            [b'F', b'N'] => Ok(Feed::FenicsGroup2),
            [b'F', b'O'] => Ok(Feed::CboeFuturesPitch),
            [b'F', b'T'] => Ok(Feed::CboeFuturesTop),
            [b'F', b'X'] => Ok(Feed::FenicsCustom),
            [b'G', b'D'] => Ok(Feed::NasdaqGids),
            [b'G', b'I'] => Ok(Feed::NyseGlobalIndexes),
            [b'H', 0] => Ok(Feed::GeminiOpraLegacy),
            [b'H', b'D'] => Ok(Feed::MiaxPerlViaDom),
            [b'H', b'H'] => Ok(Feed::NasdaqGemxTopOfMarket),
            [b'H', b'T'] => Ok(Feed::MiaxPerlViaTom),
            [b'H', b'W'] => Ok(Feed::HanweckRisk),
            [b'H', b'X'] => Ok(Feed::NasdaqGemxDepthOfMarket),
            [b'H', b'_'] => Ok(Feed::GeminiViaOpra),
            [b'I', 0] => Ok(Feed::IseViaSip),
            [b'I', b'A'] => Ok(Feed::IntelligentCrossAdf),
            [b'I', b'C'] => Ok(Feed::IntelligentCrossAspen),
            [b'I', b'D'] => Ok(Feed::IexDeep),
            [b'I', b'E'] => Ok(Feed::IexTops),
            [b'I', b'I'] => Ok(Feed::NasdaqIseTopOfMarket),
            [b'I', b'M'] => Ok(Feed::IntelligentCrossAspenMakerTaker),
            [b'I', b'O'] => Ok(Feed::NasdaqIseOptionsDepthOfMarket),
            [b'I', b'P'] => Ok(Feed::IceUsMarketByPrice),
            [b'I', b'Q'] => Ok(Feed::IceUsMarketByPriceNonFullImplied),
            [b'I', b'S'] => Ok(Feed::IseT7),
            [b'I', b'T'] => Ok(Feed::IntelligentCrossTakerMaker),
            [b'I', b'X'] => Ok(Feed::IseInet),
            [b'I', b'Y'] => Ok(Feed::IceUsMarketByOrder),
            [b'I', b'Z'] => Ok(Feed::IceUsMarketByOrderNonFullImplied),
            [b'I', b'_'] => Ok(Feed::IseViaOpra),
            [b'J', 0] => Ok(Feed::EdgaViaSip),
            [b'J', b'M'] => Ok(Feed::MercuryViaOpraLegacy),
            [b'J', b'O'] => Ok(Feed::NasdaqMrxOptionsDepthOfMarket),
            [b'J', b'S'] => Ok(Feed::JaneStreetIoi),
            [b'J', b'T'] => Ok(Feed::JpmUsTreasuries),
            [b'J', b'X'] => Ok(Feed::EdgaViaMulticast),
            [b'J', b'_'] => Ok(Feed::MercuryViaOpra),
            [b'K', 0] => Ok(Feed::EdgxViaSip),
            [b'K', b'G'] => Ok(Feed::KcgEquityIoi),
            [b'K', b'X'] => Ok(Feed::EdgxViaMulticast),
            [b'K', b'Y'] => Ok(Feed::Kcbt),
            [b'L', 0] => Ok(Feed::LtseViaSip),
            [b'L', b'H'] => Ok(Feed::LightPoolDepth),
            [b'L', b'I'] => Ok(Feed::LiffeUsMarketByOrder),
            [b'L', b'J'] => Ok(Feed::LiffeUsMarketByOrderNonFullImplied),
            [b'L', b'L'] => Ok(Feed::LtseMemoirTopOfBook),
            [b'L', b'P'] => Ok(Feed::LiffeUsMarketByPrice),
            [b'L', b'R'] => Ok(Feed::LiffeUsMarketByPriceNonFullImplied),
            [b'L', b'T'] => Ok(Feed::LtseMemoirLastSale),
            [b'L', b'U'] => Ok(Feed::LiffeUsTop),
            [b'L', b'X'] => Ok(Feed::LtseMemoirDepth),
            [b'L', b'Y'] => Ok(Feed::LiffeUsDepth),
            [b'M', 0] => Ok(Feed::NyseChicagoViaSip),
            [b'M', b'D'] => Ok(Feed::MiaxEmeraldViaOpraLegacy),
            [b'M', b'I'] => Ok(Feed::MiaxViaOpraLegacy),
            [b'M', b'M'] => Ok(Feed::NyseChicagoBbo),
            [b'M', b'O'] => Ok(Feed::MiaxViaTomAis),
            [b'M', b'T'] => Ok(Feed::NyseChicagoTrades),
            [b'M', b'X'] => Ok(Feed::NyseChicagoIntegrated),
            [b'M', b'Y'] => Ok(Feed::MgexY),
            [b'M', b'Z'] => Ok(Feed::MgexZ),
            [b'M', b'_'] => Ok(Feed::MiaxViaOpra),
            [b'N', 0] => Ok(Feed::NyseViaSip),
            [b'N', b'A'] => Ok(Feed::NyseMktBbo),
            [b'N', b'C'] => Ok(Feed::NyseArcaOptionsLevel1),
            [b'N', b'F'] => Ok(Feed::NasdaqFixedIncomeItch),
            [b'N', b'G'] => Ok(Feed::NyseIntegrated),
            [b'N', b'I'] => Ok(Feed::NyseImbalances),
            [b'N', b'L'] => Ok(Feed::NyseAlerts),
            [b'N', b'N'] => Ok(Feed::NyseBbo),
            [b'N', b'R'] => Ok(Feed::NyseTrfLevel1),
            [b'N', b'T'] => Ok(Feed::NyseTrades),
            [b'N', b'X'] => Ok(Feed::NyseOpenBookUltra),
            [b'N', b'Y'] => Ok(Feed::NymexY),
            [b'N', b'Z'] => Ok(Feed::NymexZ),
            [b'O', 0] => Ok(Feed::MiaxPearlViaOpraLegacy),
            [b'O', b'D'] => Ok(Feed::NasdaqOtcMontage),
            [b'O', b'E'] => Ok(Feed::OtcMarketsViaQuoteInsideExpert),
            [b'O', b'T'] => Ok(Feed::OtcMarketsViaQuoteInside),
            [b'O', b'Z'] => Ok(Feed::BzxOptionsViaMulticast),
            [b'O', b'_'] => Ok(Feed::MiaxPearlViaOpra),
            [b'P', 0] => Ok(Feed::ArcaViaSip),
            [b'P', b'D'] => Ok(Feed::NasdaqPhlxDepth),
            [b'P', b'E'] => Ok(Feed::OtcMarketsViaQuoteBookExpert),
            [b'P', b'I'] => Ok(Feed::ArcaImbalances),
            [b'P', b'M'] => Ok(Feed::PearlViaTomAis),
            [b'P', b'O'] => Ok(Feed::PhlxOrders),
            [b'P', b'P'] => Ok(Feed::PhlxOptionsTop),
            [b'P', b'Q'] => Ok(Feed::PsxMatchView),
            [b'P', b'S'] => Ok(Feed::OtcMarkets),
            [b'P', b'T'] => Ok(Feed::ArcaTrades),
            [b'P', b'X'] => Ok(Feed::OtcMarketsViaQuoteBook),
            [b'P', b'_'] => Ok(Feed::ArcaOptionsViaOpra),
            [b'Q', 0] => Ok(Feed::NasdaqViaSip),
            [b'Q', b'B'] => Ok(Feed::QuincyCbotEquityFutures),
            [b'Q', b'C'] => Ok(Feed::QuincyCmeEquityFutures),
            [b'Q', b'F'] => Ok(Feed::NasdaqTrf),
            [b'Q', b'P'] => Ok(Feed::BestOfNasdaqOptions),
            [b'Q', b'T'] => Ok(Feed::NasdaqExecutionSystem),
            [b'Q', b'X'] => Ok(Feed::NasdaqOptionsItto),
            [b'Q', b'_'] => Ok(Feed::NasdaqOptionsViaOpra),
            [b'R', b'D'] => Ok(Feed::ArcaOptionsDeep),
            [b'R', b'I'] => Ok(Feed::ArcaOptionsImbalances),
            [b'R', b'R'] => Ok(Feed::ArcaOptionsBbo),
            [b'R', b'T'] => Ok(Feed::ArcaOptionsTrades),
            [b'S', 0] => Ok(Feed::MemxViaSip),
            [b'S', b'H'] => Ok(Feed::SapphireViaTom),
            [b'S', b'N'] => Ok(Feed::SunIoi),
            [b'S', b'S'] => Ok(Feed::MemxTop),
            [b'S', b'T'] => Ok(Feed::MemxLastSale),
            [b'S', b'X'] => Ok(Feed::MemxDepth),
            [b'S', b'_'] => Ok(Feed::SapphireViaOpra),
            [b'T', 0] => Ok(Feed::BxOptionsViaOpraLegacy),
            [b'T', b'X'] => Ok(Feed::NasdaqViaTotalView),
            [b'T', b'Y'] => Ok(Feed::CmeBrokerTecY),
            [b'T', b'Z'] => Ok(Feed::CmeBrokerTecZ),
            [b'T', b'_'] => Ok(Feed::BxOptionsViaOpra),
            [b'U', 0] => Ok(Feed::OtcBulletinBoard),
            [b'U', b'Z'] => Ok(Feed::MemxOptionsDepth),
            [b'U', b'_'] => Ok(Feed::MemxOptionsViaOpra),
            [b'V', 0] => Ok(Feed::OtcOther),
            [b'V', b'F'] => Ok(Feed::NasdasqMarketVelocityAndForces),
            [b'V', b'I'] => Ok(Feed::VirtuFixedIncome),
            [b'V', b'U'] => Ok(Feed::VirtuIoi),
            [b'W', 0] => Ok(Feed::CboeStock),
            [b'W', b'O'] => Ok(Feed::CboeC1ViaPitch),
            [b'W', b'W'] => Ok(Feed::CboeEquitiesC2),
            [b'W', b'_'] => Ok(Feed::CboeC2ViaOpra),
            [b'X', 0] => Ok(Feed::PsxViaSip),
            [b'X', b'F'] => Ok(Feed::CmeItcF),
            [b'X', b'L'] => Ok(Feed::CmeItcSp100Sp500),
            [b'X', b'O'] => Ok(Feed::PhlxLevel1),
            [b'X', b'W'] => Ok(Feed::CmeItcW),
            [b'X', b'X'] => Ok(Feed::PsxTotalView),
            [b'X', b'Y'] => Ok(Feed::ComexY),
            [b'X', b'Z'] => Ok(Feed::ComexZ),
            [b'X', b'_'] => Ok(Feed::PhxlOptionsViaOpra),
            [b'Y', 0] => Ok(Feed::ByxViaSip),
            [b'Y', b'X'] => Ok(Feed::ByxViaMulticast),
            [b'Z', 0] => Ok(Feed::BzxViaSip),
            [b'Z', b'Q'] => Ok(Feed::BzxLevel1),
            [b'Z', b'X'] => Ok(Feed::BzxLevel2),
            [b'Z', b'_'] => Ok(Feed::BzxOptionsViaOpra),
            [b'\\', b'N'] => Ok(Feed::QuoteMontage),
            [b'\\', b'R'] => Ok(Feed::SymbolReference),
            [b'\\', b'U'] => Ok(Feed::UserComposite),
            [b'\\', b'\\'] => Ok(Feed::Nbbo),
            [b'_', b'1'] => Ok(Feed::Connection1),
            [b'_', b'2'] => Ok(Feed::Connection2),
            [b'_', b'3'] => Ok(Feed::Connection3),
            [b'_', b'4'] => Ok(Feed::Connection4),
            [b'_', b'5'] => Ok(Feed::Connection5),
            [b'_', b'6'] => Ok(Feed::Connection6),
            [b'_', b'7'] => Ok(Feed::Connection7),
            [b'_', b'8'] => Ok(Feed::Connection8),
            _ => Err(()),
        }
    }
}

impl TryFrom<[i8; 2]> for Feed {
    type Error = ();

    fn try_from(value: [i8; 2]) -> Result<Self, Self::Error> {
        Self::try_from([value[0] as u8, value[1] as u8])
    }
}

impl TryFrom<Id> for Feed {
    type Error = ();

    fn try_from(value: Id) -> Result<Self, Self::Error> {
        Self::try_from(value.0.xex_ch)
    }
}

impl TryFrom<i16> for Feed {
    type Error = ();

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        Self::try_from(value.to_le_bytes())
    }
}

impl FeedTrait for Feed {
    fn mic(&self) -> Option<i32> {
        match *self {
            Feed::NyseMktAlerts
            | Feed::NyseMktBbo
            | Feed::NyseMktImbalances
            | Feed::NyseMktIntegrated
            | Feed::NyseMktOpenBookUltra
            | Feed::NyseMktViaSip => Some(i32::from_be_bytes(*b"????")),

            Feed::ArcaBbo
            | Feed::ArcaBook
            | Feed::ArcaImbalances
            | Feed::ArcaIntegrated
            | Feed::ArcaTrades
            | Feed::ArcaViaSip => Some(i32::from_be_bytes(*b"ARCX")),

            Feed::AmexOptionsComplex => todo!(),
            Feed::AmexOptionsPillarTop => todo!(),
            Feed::AmexTrades => todo!(),
            Feed::AmexOptionsPillarDeep => todo!(),
            Feed::AmexOptionsViaOpra => todo!(),
            Feed::BxViaSip => todo!(),
            Feed::BoxLevel2ViaHsvf => todo!(),
            Feed::BxOptionsDepthOfMarket => todo!(),
            Feed::BoxLevel1ViaHsvf => todo!(),
            Feed::BxOptionsTopOfMarket => todo!(),
            Feed::BxMatchView => todo!(),
            Feed::BxTotalView => todo!(),
            Feed::CbotY => todo!(),
            Feed::CbotZ => todo!(),
            Feed::BoxViaOpra => todo!(),
            Feed::NyseNationalViaSip => todo!(),
            Feed::NyseNationalBbo => todo!(),
            Feed::CitadelIoi => todo!(),
            Feed::CitadelFixedIncome => todo!(),
            Feed::CmeIndices => todo!(),
            Feed::CmePit => todo!(),
            Feed::CboeC2ViaPitch => todo!(),
            Feed::CboeIndices => todo!(),
            Feed::NyseNationalTrades => todo!(),
            Feed::NyseNationalDepth => todo!(),
            Feed::CmeY => todo!(),
            Feed::CmeZ => todo!(),
            Feed::CboeViaOpra => todo!(),
            Feed::FinraViaSip => todo!(),
            Feed::NasdaqChicagoTrf => todo!(),
            Feed::DowIndices => todo!(),
            Feed::NyseTrf => todo!(),
            Feed::NasdaqCarteretTrf => todo!(),
            Feed::MiaxEmeraldViaOpra => todo!(),
            Feed::IexViaSip => todo!(),
            Feed::EdgxOptionsViaOpra2 => todo!(),
            Feed::ComexPit => todo!(),
            Feed::MiaxEmeraldViaTomAis => todo!(),
            Feed::EspeedItch => todo!(),
            Feed::EdgxOptionsViaMulticast => todo!(),
            Feed::EdgxOptionsViaOpra => todo!(),
            Feed::FenicsGroup1 => todo!(),
            Feed::FenicsGlobal => todo!(),
            Feed::FenicsGroup2 => todo!(),
            Feed::CboeFuturesPitch => todo!(),
            Feed::CboeFuturesTop => todo!(),
            Feed::FenicsCustom => todo!(),
            Feed::NasdaqGids => todo!(),
            Feed::NyseGlobalIndexes => todo!(),
            Feed::GeminiOpraLegacy => todo!(),
            Feed::MiaxPerlViaDom => todo!(),
            Feed::NasdaqGemxTopOfMarket => todo!(),
            Feed::MiaxPerlViaTom => todo!(),
            Feed::HanweckRisk => todo!(),
            Feed::NasdaqGemxDepthOfMarket => todo!(),
            Feed::GeminiViaOpra => todo!(),
            Feed::IseViaSip => todo!(),
            Feed::IntelligentCrossAdf => todo!(),
            Feed::IntelligentCrossAspen => todo!(),
            Feed::IexDeep => todo!(),
            Feed::IexTops => todo!(),
            Feed::NasdaqIseTopOfMarket => todo!(),
            Feed::IntelligentCrossAspenMakerTaker => todo!(),
            Feed::NasdaqIseOptionsDepthOfMarket => todo!(),
            Feed::IceUsMarketByPrice => todo!(),
            Feed::IceUsMarketByPriceNonFullImplied => todo!(),
            Feed::IseT7 => todo!(),
            Feed::IntelligentCrossTakerMaker => todo!(),
            Feed::IseInet => todo!(),
            Feed::IceUsMarketByOrder => todo!(),
            Feed::IceUsMarketByOrderNonFullImplied => todo!(),
            Feed::IseViaOpra => todo!(),
            Feed::EdgaViaSip => todo!(),
            Feed::MercuryViaOpraLegacy => todo!(),
            Feed::NasdaqMrxOptionsDepthOfMarket => todo!(),
            Feed::JaneStreetIoi => todo!(),
            Feed::JpmUsTreasuries => todo!(),
            Feed::EdgaViaMulticast => todo!(),
            Feed::MercuryViaOpra => todo!(),
            Feed::EdgxViaSip => todo!(),
            Feed::KcgEquityIoi => todo!(),
            Feed::EdgxViaMulticast => todo!(),
            Feed::Kcbt => todo!(),
            Feed::LtseViaSip => todo!(),
            Feed::LightPoolDepth => todo!(),
            Feed::LiffeUsMarketByOrder => todo!(),
            Feed::LiffeUsMarketByOrderNonFullImplied => todo!(),
            Feed::LtseMemoirTopOfBook => todo!(),
            Feed::LiffeUsMarketByPrice => todo!(),
            Feed::LiffeUsMarketByPriceNonFullImplied => todo!(),
            Feed::LtseMemoirLastSale => todo!(),
            Feed::LiffeUsTop => todo!(),
            Feed::LtseMemoirDepth => todo!(),
            Feed::LiffeUsDepth => todo!(),
            Feed::NyseChicagoViaSip => todo!(),
            Feed::MiaxEmeraldViaOpraLegacy => todo!(),
            Feed::MiaxViaOpraLegacy => todo!(),
            Feed::NyseChicagoBbo => todo!(),
            Feed::MiaxViaTomAis => todo!(),
            Feed::NyseChicagoTrades => todo!(),
            Feed::NyseChicagoIntegrated => todo!(),
            Feed::MgexY => todo!(),
            Feed::MgexZ => todo!(),
            Feed::MiaxViaOpra => todo!(),
            Feed::NyseViaSip => todo!(),
            Feed::NyseArcaOptionsLevel1 => todo!(),
            Feed::NasdaqFixedIncomeItch => todo!(),
            Feed::NyseIntegrated => todo!(),
            Feed::NyseImbalances => todo!(),
            Feed::NyseAlerts => todo!(),
            Feed::NyseBbo => todo!(),
            Feed::NyseTrfLevel1 => todo!(),
            Feed::NyseTrades => todo!(),
            Feed::NyseOpenBookUltra => todo!(),
            Feed::NymexY => todo!(),
            Feed::NymexZ => todo!(),
            Feed::MiaxPearlViaOpraLegacy => todo!(),
            Feed::NasdaqOtcMontage => todo!(),
            Feed::OtcMarketsViaQuoteInsideExpert => todo!(),
            Feed::OtcMarketsViaQuoteInside => todo!(),
            Feed::BzxOptionsViaMulticast => todo!(),
            Feed::MiaxPearlViaOpra => todo!(),
            Feed::NasdaqPhlxDepth => todo!(),
            Feed::OtcMarketsViaQuoteBookExpert => todo!(),
            Feed::PearlViaTomAis => todo!(),
            Feed::PhlxOrders => todo!(),
            Feed::PhlxOptionsTop => todo!(),
            Feed::PsxMatchView => todo!(),
            Feed::OtcMarkets => todo!(),
            Feed::OtcMarketsViaQuoteBook => todo!(),
            Feed::ArcaOptionsViaOpra => todo!(),
            Feed::NasdaqViaSip => todo!(),
            Feed::QuincyCbotEquityFutures => todo!(),
            Feed::QuincyCmeEquityFutures => todo!(),
            Feed::NasdaqTrf => todo!(),
            Feed::BestOfNasdaqOptions => todo!(),
            Feed::NasdaqExecutionSystem => todo!(),
            Feed::NasdaqOptionsItto => todo!(),
            Feed::NasdaqOptionsViaOpra => todo!(),
            Feed::ArcaOptionsDeep => todo!(),
            Feed::ArcaOptionsImbalances => todo!(),
            Feed::ArcaOptionsBbo => todo!(),
            Feed::ArcaOptionsTrades => todo!(),
            Feed::MemxViaSip => todo!(),
            Feed::SapphireViaTom => todo!(),
            Feed::SunIoi => todo!(),
            Feed::MemxTop => todo!(),
            Feed::MemxLastSale => todo!(),
            Feed::MemxDepth => todo!(),
            Feed::SapphireViaOpra => todo!(),
            Feed::BxOptionsViaOpraLegacy => todo!(),
            Feed::NasdaqViaTotalView => todo!(),
            Feed::CmeBrokerTecY => todo!(),
            Feed::CmeBrokerTecZ => todo!(),
            Feed::BxOptionsViaOpra => todo!(),
            Feed::OtcBulletinBoard => todo!(),
            Feed::MemxOptionsDepth => todo!(),
            Feed::MemxOptionsViaOpra => todo!(),
            Feed::OtcOther => todo!(),
            Feed::NasdasqMarketVelocityAndForces => todo!(),
            Feed::VirtuFixedIncome => todo!(),
            Feed::VirtuIoi => todo!(),
            Feed::CboeStock => todo!(),
            Feed::CboeC1ViaPitch => todo!(),
            Feed::CboeEquitiesC2 => todo!(),
            Feed::CboeC2ViaOpra => todo!(),
            Feed::PsxViaSip => todo!(),
            Feed::CmeItcF => todo!(),
            Feed::CmeItcSp100Sp500 => todo!(),
            Feed::PhlxLevel1 => todo!(),
            Feed::CmeItcW => todo!(),
            Feed::PsxTotalView => todo!(),
            Feed::ComexY => todo!(),
            Feed::ComexZ => todo!(),
            Feed::PhxlOptionsViaOpra => todo!(),
            Feed::ByxViaSip => todo!(),
            Feed::ByxViaMulticast => todo!(),
            Feed::BzxViaSip => todo!(),
            Feed::BzxLevel1 => todo!(),
            Feed::BzxLevel2 => todo!(),
            Feed::BzxOptionsViaOpra => todo!(),
            _ => None,
        }
    }

    /// Retrieve the group
    fn group(&self) -> Group {
        Group::Country(Country::UnitedStates)
    }
}
