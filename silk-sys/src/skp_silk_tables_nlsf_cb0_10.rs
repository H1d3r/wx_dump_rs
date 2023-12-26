use crate::SKP_Silk_NLSF_MSVQ_decode::{SKP_Silk_NLSF_CBS, SKP_Silk_NLSF_CB_struct};
use once_cell::sync::Lazy;

pub static SKP_SILK_NLSF_MSVQ_CB0_10_CDF: [u16; 126] = [
    0u16, 2658u16, 4420u16, 6107u16, 7757u16, 9408u16, 10955u16, 12502u16, 13983u16, 15432u16,
    16882u16, 18331u16, 19750u16, 21108u16, 22409u16, 23709u16, 25010u16, 26256u16, 27501u16,
    28747u16, 29965u16, 31158u16, 32351u16, 33544u16, 34736u16, 35904u16, 36997u16, 38091u16,
    39185u16, 40232u16, 41280u16, 42327u16, 43308u16, 44290u16, 45271u16, 46232u16, 47192u16,
    48132u16, 49032u16, 49913u16, 50775u16, 51618u16, 52462u16, 53287u16, 54095u16, 54885u16,
    55675u16, 56449u16, 57222u16, 57979u16, 58688u16, 59382u16, 60076u16, 60726u16, 61363u16,
    61946u16, 62505u16, 63052u16, 63543u16, 63983u16, 64396u16, 64766u16, 65023u16, 65279u16,
    65535u16, 0u16, 4977u16, 9542u16, 14106u16, 18671u16, 23041u16, 27319u16, 31596u16, 35873u16,
    39969u16, 43891u16, 47813u16, 51652u16, 55490u16, 59009u16, 62307u16, 65535u16, 0u16, 8571u16,
    17142u16, 25529u16, 33917u16, 42124u16, 49984u16, 57844u16, 65535u16, 0u16, 8732u16, 17463u16,
    25825u16, 34007u16, 42189u16, 50196u16, 58032u16, 65535u16, 0u16, 8948u16, 17704u16, 25733u16,
    33762u16, 41791u16, 49821u16, 57678u16, 65535u16, 0u16, 4374u16, 8655u16, 12936u16, 17125u16,
    21313u16, 25413u16, 29512u16, 33611u16, 37710u16, 41809u16, 45820u16, 49832u16, 53843u16,
    57768u16, 61694u16, 65535u16,
];
pub static SKP_SILK_NLSF_MSVQ_CB0_10_CDF_START_PTR: Lazy<Vec<&'static [u16]>> = Lazy::new(|| {
    vec![
        &SKP_SILK_NLSF_MSVQ_CB0_10_CDF[0..],
        &SKP_SILK_NLSF_MSVQ_CB0_10_CDF[65..],
        &SKP_SILK_NLSF_MSVQ_CB0_10_CDF[82..],
        &SKP_SILK_NLSF_MSVQ_CB0_10_CDF[91..],
        &SKP_SILK_NLSF_MSVQ_CB0_10_CDF[100..],
        &SKP_SILK_NLSF_MSVQ_CB0_10_CDF[109..],
    ]
});
pub static SKP_SILK_NLSF_MSVQ_CB0_10_CDF_MIDDLE_IDX: [i32; 6] = [23, 8, 5, 5, 5, 9];

pub static SKP_SILK_NLSF_MSVQ_CB0_10_RATES_Q5: [i16; 120] = [
    148i16, 167i16, 169i16, 170i16, 170i16, 173i16, 173i16, 175i16, 176i16, 176i16, 176i16, 177i16,
    179i16, 181i16, 181i16, 181i16, 183i16, 183i16, 183i16, 184i16, 185i16, 185i16, 185i16, 185i16,
    186i16, 189i16, 189i16, 189i16, 191i16, 191i16, 191i16, 194i16, 194i16, 194i16, 195i16, 195i16,
    196i16, 198i16, 199i16, 200i16, 201i16, 201i16, 202i16, 203i16, 204i16, 204i16, 205i16, 205i16,
    206i16, 209i16, 210i16, 210i16, 213i16, 214i16, 218i16, 220i16, 221i16, 226i16, 231i16, 234i16,
    239i16, 256i16, 256i16, 256i16, 119i16, 123i16, 123i16, 123i16, 125i16, 126i16, 126i16, 126i16,
    128i16, 130i16, 130i16, 131i16, 131i16, 135i16, 138i16, 139i16, 94i16, 94i16, 95i16, 95i16,
    96i16, 98i16, 98i16, 99i16, 93i16, 93i16, 95i16, 96i16, 96i16, 97i16, 98i16, 100i16, 92i16,
    93i16, 97i16, 97i16, 97i16, 97i16, 98i16, 98i16, 125i16, 126i16, 126i16, 127i16, 127i16,
    128i16, 128i16, 128i16, 128i16, 128i16, 129i16, 129i16, 129i16, 130i16, 130i16, 131i16,
];

pub static SKP_SILK_NLSF_MSVQ_CB0_10_N_DELTA_MIN_Q15: [i32; 11] =
    [563, 3, 22, 20, 3, 3, 132, 119, 358, 86, 964];

pub static SKP_SILK_NLSF_MSVQ_CB0_10_Q15: [i16; 1200] = [
    2210i16, 4023i16, 6981i16, 9260i16, 12573i16, 15687i16, 19207i16, 22383i16, 25981i16, 29142i16,
    3285i16, 4172i16, 6116i16, 10856i16, 15289i16, 16826i16, 19701i16, 22010i16, 24721i16,
    29313i16, 1554i16, 2511i16, 6577i16, 10337i16, 13837i16, 16511i16, 20086i16, 23214i16,
    26480i16, 29464i16, 3062i16, 4017i16, 5771i16, 10037i16, 13365i16, 14952i16, 20140i16,
    22891i16, 25229i16, 29603i16, 2085i16, 3457i16, 5934i16, 8718i16, 11501i16, 13670i16, 17997i16,
    21817i16, 24935i16, 28745i16, 2776i16, 4093i16, 6421i16, 10413i16, 15111i16, 16806i16,
    20825i16, 23826i16, 26308i16, 29411i16, 2717i16, 4034i16, 5697i16, 8463i16, 14301i16, 16354i16,
    19007i16, 23413i16, 25812i16, 28506i16, 2872i16, 3702i16, 5881i16, 11034i16, 17141i16,
    18879i16, 21146i16, 23451i16, 25817i16, 29600i16, 2999i16, 4015i16, 7357i16, 11219i16,
    12866i16, 17307i16, 20081i16, 22644i16, 26774i16, 29107i16, 2942i16, 3866i16, 5918i16,
    11915i16, 13909i16, 16072i16, 20453i16, 22279i16, 27310i16, 29826i16, 2271i16, 3527i16,
    6606i16, 9729i16, 12943i16, 17382i16, 20224i16, 22345i16, 24602i16, 28290i16, 2207i16, 3310i16,
    5844i16, 9339i16, 11141i16, 15651i16, 18576i16, 21177i16, 25551i16, 28228i16, 3963i16, 4975i16,
    6901i16, 11588i16, 13466i16, 15577i16, 19231i16, 21368i16, 25510i16, 27759i16, 2749i16,
    3549i16, 6966i16, 13808i16, 15653i16, 17645i16, 20090i16, 22599i16, 26467i16, 28537i16,
    2126i16, 3504i16, 5109i16, 9954i16, 12550i16, 14620i16, 19703i16, 21687i16, 26457i16, 29106i16,
    3966i16, 5745i16, 7442i16, 9757i16, 14468i16, 16404i16, 19135i16, 23048i16, 25375i16, 28391i16,
    3197i16, 4751i16, 6451i16, 9298i16, 13038i16, 14874i16, 17962i16, 20627i16, 23835i16, 28464i16,
    3195i16, 4081i16, 6499i16, 12252i16, 14289i16, 16040i16, 18357i16, 20730i16, 26980i16,
    29309i16, 1533i16, 2471i16, 4486i16, 7796i16, 12332i16, 15758i16, 19567i16, 22298i16, 25673i16,
    29051i16, 2002i16, 2971i16, 4985i16, 8083i16, 13181i16, 15435i16, 18237i16, 21517i16, 24595i16,
    28351i16, 3808i16, 4925i16, 6710i16, 10201i16, 12011i16, 14300i16, 18457i16, 20391i16,
    26525i16, 28956i16, 2281i16, 3418i16, 4979i16, 8726i16, 15964i16, 18104i16, 20250i16, 22771i16,
    25286i16, 28954i16, 3051i16, 5479i16, 7290i16, 9848i16, 12744i16, 14503i16, 18665i16, 23684i16,
    26065i16, 28947i16, 2364i16, 3565i16, 5502i16, 9621i16, 14922i16, 16621i16, 19005i16, 20996i16,
    26310i16, 29302i16, 4093i16, 5212i16, 6833i16, 9880i16, 16303i16, 18286i16, 20571i16, 23614i16,
    26067i16, 29128i16, 2941i16, 3996i16, 6038i16, 10638i16, 12668i16, 14451i16, 16798i16,
    19392i16, 26051i16, 28517i16, 3863i16, 5212i16, 7019i16, 9468i16, 11039i16, 13214i16, 19942i16,
    22344i16, 25126i16, 29539i16, 4615i16, 6172i16, 7853i16, 10252i16, 12611i16, 14445i16,
    19719i16, 22441i16, 24922i16, 29341i16, 3566i16, 4512i16, 6985i16, 8684i16, 10544i16, 16097i16,
    18058i16, 22475i16, 26066i16, 28167i16, 4481i16, 5489i16, 7432i16, 11414i16, 13191i16,
    15225i16, 20161i16, 22258i16, 26484i16, 29716i16, 3320i16, 4320i16, 6621i16, 9867i16, 11581i16,
    14034i16, 21168i16, 23210i16, 26588i16, 29903i16, 3794i16, 4689i16, 6916i16, 8655i16, 10143i16,
    16144i16, 19568i16, 21588i16, 27557i16, 29593i16, 2446i16, 3276i16, 5918i16, 12643i16,
    16601i16, 18013i16, 21126i16, 23175i16, 27300i16, 29634i16, 2450i16, 3522i16, 5437i16, 8560i16,
    15285i16, 19911i16, 21826i16, 24097i16, 26567i16, 29078i16, 2580i16, 3796i16, 5580i16, 8338i16,
    9969i16, 12675i16, 18907i16, 22753i16, 25450i16, 29292i16, 3325i16, 4312i16, 6241i16, 7709i16,
    9164i16, 14452i16, 21665i16, 23797i16, 27096i16, 29857i16, 3338i16, 4163i16, 7738i16, 11114i16,
    12668i16, 14753i16, 16931i16, 22736i16, 25671i16, 28093i16, 3840i16, 4755i16, 7755i16,
    13471i16, 15338i16, 17180i16, 20077i16, 22353i16, 27181i16, 29743i16, 2504i16, 4079i16,
    8351i16, 12118i16, 15046i16, 18595i16, 21684i16, 24704i16, 27519i16, 29937i16, 5234i16,
    6342i16, 8267i16, 11821i16, 15155i16, 16760i16, 20667i16, 23488i16, 25949i16, 29307i16,
    2681i16, 3562i16, 6028i16, 10827i16, 18458i16, 20458i16, 22303i16, 24701i16, 26912i16,
    29956i16, 3374i16, 4528i16, 6230i16, 8256i16, 9513i16, 12730i16, 18666i16, 20720i16, 26007i16,
    28425i16, 2731i16, 3629i16, 8320i16, 12450i16, 14112i16, 16431i16, 18548i16, 22098i16,
    25329i16, 27718i16, 3481i16, 4401i16, 7321i16, 9319i16, 11062i16, 13093i16, 15121i16, 22315i16,
    26331i16, 28740i16, 3577i16, 4945i16, 6669i16, 8792i16, 10299i16, 12645i16, 19505i16, 24766i16,
    26996i16, 29634i16, 4058i16, 5060i16, 7288i16, 10190i16, 11724i16, 13936i16, 15849i16,
    18539i16, 26701i16, 29845i16, 4262i16, 5390i16, 7057i16, 8982i16, 10187i16, 15264i16, 20480i16,
    22340i16, 25958i16, 28072i16, 3404i16, 4329i16, 6629i16, 7946i16, 10121i16, 17165i16, 19640i16,
    22244i16, 25062i16, 27472i16, 3157i16, 4168i16, 6195i16, 9319i16, 10771i16, 13325i16, 15416i16,
    19816i16, 24672i16, 27634i16, 2503i16, 3473i16, 5130i16, 6767i16, 8571i16, 14902i16, 19033i16,
    21926i16, 26065i16, 28728i16, 4133i16, 5102i16, 7553i16, 10054i16, 11757i16, 14924i16,
    17435i16, 20186i16, 23987i16, 26272i16, 4972i16, 6139i16, 7894i16, 9633i16, 11320i16, 14295i16,
    21737i16, 24306i16, 26919i16, 29907i16, 2958i16, 3816i16, 6851i16, 9204i16, 10895i16, 18052i16,
    20791i16, 23338i16, 27556i16, 29609i16, 5234i16, 6028i16, 8034i16, 10154i16, 11242i16,
    14789i16, 18948i16, 20966i16, 26585i16, 29127i16, 5241i16, 6838i16, 10526i16, 12819i16,
    14681i16, 17328i16, 19928i16, 22336i16, 26193i16, 28697i16, 3412i16, 4251i16, 5988i16, 7094i16,
    9907i16, 18243i16, 21669i16, 23777i16, 26969i16, 29087i16, 2470i16, 3217i16, 7797i16, 15296i16,
    17365i16, 19135i16, 21979i16, 24256i16, 27322i16, 29442i16, 4939i16, 5804i16, 8145i16,
    11809i16, 13873i16, 15598i16, 17234i16, 19423i16, 26476i16, 29645i16, 5051i16, 6167i16,
    8223i16, 9655i16, 12159i16, 17995i16, 20464i16, 22832i16, 26616i16, 28462i16, 4987i16, 5907i16,
    9319i16, 11245i16, 13132i16, 15024i16, 17485i16, 22687i16, 26011i16, 28273i16, 5137i16,
    6884i16, 11025i16, 14950i16, 17191i16, 19425i16, 21807i16, 24393i16, 26938i16, 29288i16,
    7057i16, 7884i16, 9528i16, 10483i16, 10960i16, 14811i16, 19070i16, 21675i16, 25645i16,
    28019i16, 6759i16, 7160i16, 8546i16, 11779i16, 12295i16, 13023i16, 16627i16, 21099i16,
    24697i16, 28287i16, 3863i16, 9762i16, 11068i16, 11445i16, 12049i16, 13960i16, 18085i16,
    21507i16, 25224i16, 28997i16, 397i16, 335i16, 651i16, 1168i16, 640i16, 765i16, 465i16, 331i16,
    214i16, -194i16, -578i16, -647i16, -657i16, 750i16, 564i16, 613i16, 549i16, 630i16, 304i16,
    -52i16, 828i16, 922i16, 443i16, 111i16, 138i16, 124i16, 169i16, 14i16, 144i16, 83i16, 132i16,
    58i16, -413i16, -752i16, 869i16, 336i16, 385i16, 69i16, 56i16, 830i16, -227i16, -266i16,
    -368i16, -440i16, -1195i16, 163i16, 126i16, -228i16, 802i16, 156i16, 188i16, 120i16, 376i16,
    59i16, -358i16, -558i16, -1326i16, -254i16, -202i16, -789i16, 296i16, 92i16, -70i16, -129i16,
    -718i16, -1135i16, 292i16, -29i16, -631i16, 487i16, -157i16, -153i16, -279i16, 2i16, -419i16,
    -342i16, -34i16, -514i16, -799i16, -1571i16, -687i16, -609i16, -546i16, -130i16, -215i16,
    -252i16, -446i16, -574i16, -1337i16, 207i16, -72i16, 32i16, 103i16, -642i16, 942i16, 733i16,
    187i16, 29i16, -211i16, -814i16, 143i16, 225i16, 20i16, 24i16, -268i16, -377i16, 1623i16,
    1133i16, 667i16, 164i16, 307i16, 366i16, 187i16, 34i16, 62i16, -313i16, -832i16, -1482i16,
    -1181i16, 483i16, -42i16, -39i16, -450i16, -1406i16, -587i16, -52i16, -760i16, 334i16, 98i16,
    -60i16, -500i16, -488i16, -1058i16, 299i16, 131i16, -250i16, -251i16, -703i16, 1037i16, 568i16,
    -413i16, -265i16, 1687i16, 573i16, 345i16, 323i16, 98i16, 61i16, -102i16, 31i16, 135i16,
    149i16, 617i16, 365i16, -39i16, 34i16, -611i16, 1201i16, 1421i16, 736i16, -414i16, -393i16,
    -492i16, -343i16, -316i16, -532i16, 528i16, 172i16, 90i16, 322i16, -294i16, -319i16, -541i16,
    503i16, 639i16, 401i16, 1i16, -149i16, -73i16, -167i16, 150i16, 118i16, 308i16, 218i16, 121i16,
    195i16, -143i16, -261i16, -1013i16, -802i16, 387i16, 436i16, 130i16, -427i16, -448i16, -681i16,
    123i16, -87i16, -251i16, -113i16, 274i16, 310i16, 445i16, 501i16, 354i16, 272i16, 141i16,
    -285i16, 569i16, 656i16, 37i16, -49i16, 251i16, -386i16, -263i16, 1122i16, 604i16, 606i16,
    336i16, 95i16, 34i16, 0i16, 85i16, 180i16, 207i16, -367i16, -622i16, 1070i16, -6i16, -79i16,
    -160i16, -92i16, -137i16, -276i16, -323i16, -371i16, -696i16, -1036i16, 407i16, 102i16, -86i16,
    -214i16, -482i16, -647i16, -28i16, -291i16, -97i16, -180i16, -250i16, -435i16, -18i16, -76i16,
    -332i16, 410i16, 407i16, 168i16, 539i16, 411i16, 254i16, 111i16, 58i16, -145i16, 200i16, 30i16,
    187i16, 116i16, 131i16, -367i16, -475i16, 781i16, -559i16, 561i16, 195i16, -115i16, 8i16,
    -168i16, 30i16, 55i16, -122i16, 131i16, 82i16, -5i16, -273i16, -50i16, -632i16, 668i16, 4i16,
    32i16, -26i16, -279i16, 315i16, 165i16, 197i16, 377i16, 155i16, -41i16, -138i16, -324i16,
    -109i16, -617i16, 360i16, 98i16, -53i16, -319i16, -114i16, -245i16, -82i16, 507i16, 468i16,
    263i16, -137i16, -389i16, 652i16, 354i16, -18i16, -227i16, -462i16, -135i16, 317i16, 53i16,
    -16i16, 66i16, -72i16, -126i16, -356i16, -347i16, -328i16, -72i16, -337i16, 324i16, 152i16,
    349i16, 169i16, -196i16, 179i16, 254i16, 260i16, 325i16, -74i16, -80i16, 75i16, -31i16, 270i16,
    275i16, 87i16, 278i16, -446i16, -301i16, 309i16, 71i16, -25i16, -242i16, 516i16, 161i16,
    -162i16, -83i16, 329i16, 230i16, -311i16, -259i16, 177i16, -26i16, -462i16, 89i16, 257i16,
    6i16, -130i16, -93i16, -456i16, -317i16, -221i16, -206i16, -417i16, -182i16, -74i16, 234i16,
    48i16, 261i16, 359i16, 231i16, 258i16, 85i16, -282i16, 252i16, -147i16, -222i16, 251i16,
    -207i16, 443i16, 123i16, -417i16, -36i16, 273i16, -241i16, 240i16, -112i16, 44i16, -167i16,
    126i16, -124i16, -77i16, 58i16, -401i16, 333i16, -118i16, 82i16, 126i16, 151i16, -433i16,
    359i16, -130i16, -102i16, 131i16, -244i16, 86i16, 85i16, -462i16, 414i16, -240i16, 16i16,
    145i16, 28i16, -205i16, -481i16, 373i16, 293i16, -72i16, -174i16, 62i16, 259i16, -8i16, -18i16,
    362i16, 233i16, 185i16, 43i16, 278i16, 27i16, 193i16, 570i16, -248i16, 189i16, 92i16, 31i16,
    -275i16, -3i16, 243i16, 176i16, 438i16, 209i16, 206i16, -51i16, 79i16, 109i16, 168i16, -185i16,
    -308i16, -68i16, -618i16, 385i16, -310i16, -108i16, -164i16, 165i16, 61i16, -152i16, -101i16,
    -412i16, -268i16, -257i16, -40i16, -20i16, -28i16, -158i16, -301i16, 271i16, 380i16, -338i16,
    -367i16, -132i16, 64i16, 114i16, -131i16, -225i16, -156i16, -260i16, -63i16, -116i16, 155i16,
    -586i16, -202i16, 254i16, -287i16, 178i16, 227i16, -106i16, -294i16, 164i16, 298i16, -100i16,
    185i16, 317i16, 193i16, -45i16, 28i16, 80i16, -87i16, -433i16, 22i16, -48i16, 48i16, -237i16,
    -229i16, -139i16, 120i16, -364i16, 268i16, -136i16, 396i16, 125i16, 130i16, -89i16, -272i16,
    118i16, -256i16, -68i16, -451i16, 488i16, 143i16, -165i16, -48i16, -190i16, 106i16, 219i16,
    47i16, 435i16, 245i16, 97i16, 75i16, -418i16, 121i16, -187i16, 570i16, -200i16, -351i16,
    225i16, -21i16, -217i16, 234i16, -111i16, 194i16, 14i16, 242i16, 118i16, 140i16, -397i16,
    355i16, 361i16, -45i16, -195i16,
];

pub static SKP_SILK_NLSF_CB0_10_STAGE_INFO: Lazy<Vec<SKP_Silk_NLSF_CBS>> = Lazy::new(|| {
    vec![
        SKP_Silk_NLSF_CBS {
            nVectors: 64,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_10_Q15[0 as usize..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_10_RATES_Q5[0..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 16,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_10_Q15[640 as usize..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_10_RATES_Q5[64..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_10_Q15[800 as usize..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_10_RATES_Q5[80..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_10_Q15[880 as usize..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_10_RATES_Q5[88..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 8,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_10_Q15[960 as usize..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_10_RATES_Q5[96..],
        },
        SKP_Silk_NLSF_CBS {
            nVectors: 16,
            CB_NLSF_Q15: &SKP_SILK_NLSF_MSVQ_CB0_10_Q15[1040 as usize..],
            Rates_Q5: &SKP_SILK_NLSF_MSVQ_CB0_10_RATES_Q5[104..],
        },
    ]
});

pub static SKP_SILK_NLSF_CB0_10: SKP_Silk_NLSF_CB_struct = SKP_Silk_NLSF_CB_struct {
    nStages: 6,
    CBStages: &SKP_SILK_NLSF_CB0_10_STAGE_INFO,
    NDeltaMin_Q15: &SKP_SILK_NLSF_MSVQ_CB0_10_N_DELTA_MIN_Q15,
    CDF: &SKP_SILK_NLSF_MSVQ_CB0_10_CDF,
    StartPtr: &SKP_SILK_NLSF_MSVQ_CB0_10_CDF_START_PTR,
    MiddleIx: &SKP_SILK_NLSF_MSVQ_CB0_10_CDF_MIDDLE_IDX,
};