extern {
    fn BrotliAllocate(
        m : *mut MemoryManager, n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn BrotliFree(
        m : *mut MemoryManager, p : *mut ::std::os::raw::c_void
    );
    fn BrotliPopulationCostCommand(
        arg1 : *const HistogramCommand
    ) -> f64;
    fn BrotliPopulationCostDistance(
        arg1 : *const HistogramDistance
    ) -> f64;
    fn BrotliPopulationCostLiteral(
        arg1 : *const HistogramLiteral
    ) -> f64;
    fn log2(__x : f64) -> f64;
    fn memcpy(
        __dest : *mut ::std::os::raw::c_void,
        __src : *const ::std::os::raw::c_void,
        __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn memmove(
        __dest : *mut ::std::os::raw::c_void,
        __src : *const ::std::os::raw::c_void,
        __n : usize
    ) -> *mut ::std::os::raw::c_void;
    fn memset(
        __s : *mut ::std::os::raw::c_void, __c : i32, __n : usize
    ) -> *mut ::std::os::raw::c_void;
}

static mut kLog2Table
    : [f32; 256]
    = [   0.0000000000000000f32,
          0.0000000000000000f32,
          1.0000000000000000f32,
          1.5849625007211563f32,
          2.0000000000000000f32,
          2.3219280948873622f32,
          2.5849625007211561f32,
          2.8073549220576042f32,
          3.0000000000000000f32,
          3.1699250014423126f32,
          3.3219280948873626f32,
          3.4594316186372978f32,
          3.5849625007211565f32,
          3.7004397181410922f32,
          3.8073549220576037f32,
          3.9068905956085187f32,
          4.0000000000000000f32,
          4.0874628412503400f32,
          4.1699250014423122f32,
          4.2479275134435852f32,
          4.3219280948873626f32,
          4.3923174227787607f32,
          4.4594316186372973f32,
          4.5235619560570131f32,
          4.5849625007211570f32,
          4.6438561897747244f32,
          4.7004397181410926f32,
          4.7548875021634691f32,
          4.8073549220576037f32,
          4.8579809951275728f32,
          4.9068905956085187f32,
          4.9541963103868758f32,
          5.0000000000000000f32,
          5.0443941193584534f32,
          5.0874628412503400f32,
          5.1292830169449664f32,
          5.1699250014423122f32,
          5.2094533656289501f32,
          5.2479275134435852f32,
          5.2854022188622487f32,
          5.3219280948873626f32,
          5.3575520046180838f32,
          5.3923174227787607f32,
          5.4262647547020979f32,
          5.4594316186372973f32,
          5.4918530963296748f32,
          5.5235619560570131f32,
          5.5545888516776376f32,
          5.5849625007211570f32,
          5.6147098441152083f32,
          5.6438561897747244f32,
          5.6724253419714961f32,
          5.7004397181410926f32,
          5.7279204545631996f32,
          5.7548875021634691f32,
          5.7813597135246599f32,
          5.8073549220576046f32,
          5.8328900141647422f32,
          5.8579809951275719f32,
          5.8826430493618416f32,
          5.9068905956085187f32,
          5.9307373375628867f32,
          5.9541963103868758f32,
          5.9772799234999168f32,
          6.0000000000000000f32,
          6.0223678130284544f32,
          6.0443941193584534f32,
          6.0660891904577721f32,
          6.0874628412503400f32,
          6.1085244567781700f32,
          6.1292830169449672f32,
          6.1497471195046822f32,
          6.1699250014423122f32,
          6.1898245588800176f32,
          6.2094533656289510f32,
          6.2288186904958804f32,
          6.2479275134435861f32,
          6.2667865406949019f32,
          6.2854022188622487f32,
          6.3037807481771031f32,
          6.3219280948873617f32,
          6.3398500028846252f32,
          6.3575520046180847f32,
          6.3750394313469254f32,
          6.3923174227787598f32,
          6.4093909361377026f32,
          6.4262647547020979f32,
          6.4429434958487288f32,
          6.4594316186372982f32,
          6.4757334309663976f32,
          6.4918530963296748f32,
          6.5077946401986964f32,
          6.5235619560570131f32,
          6.5391588111080319f32,
          6.5545888516776376f32,
          6.5698556083309478f32,
          6.5849625007211561f32,
          6.5999128421871278f32,
          6.6147098441152092f32,
          6.6293566200796095f32,
          6.6438561897747253f32,
          6.6582114827517955f32,
          6.6724253419714952f32,
          6.6865005271832185f32,
          6.7004397181410917f32,
          6.7142455176661224f32,
          6.7279204545631988f32,
          6.7414669864011465f32,
          6.7548875021634691f32,
          6.7681843247769260f32,
          6.7813597135246599f32,
          6.7944158663501062f32,
          6.8073549220576037f32,
          6.8201789624151887f32,
          6.8328900141647422f32,
          6.8454900509443757f32,
          6.8579809951275719f32,
          6.8703647195834048f32,
          6.8826430493618416f32,
          6.8948177633079437f32,
          6.9068905956085187f32,
          6.9188632372745955f32,
          6.9307373375628867f32,
          6.9425145053392399f32,
          6.9541963103868758f32,
          6.9657842846620879f32,
          6.9772799234999168f32,
          6.9886846867721664f32,
          7.0000000000000000f32,
          7.0112272554232540f32,
          7.0223678130284544f32,
          7.0334230015374501f32,
          7.0443941193584534f32,
          7.0552824355011898f32,
          7.0660891904577721f32,
          7.0768155970508317f32,
          7.0874628412503400f32,
          7.0980320829605272f32,
          7.1085244567781700f32,
          7.1189410727235076f32,
          7.1292830169449664f32,
          7.1395513523987937f32,
          7.1497471195046822f32,
          7.1598713367783891f32,
          7.1699250014423130f32,
          7.1799090900149345f32,
          7.1898245588800176f32,
          7.1996723448363644f32,
          7.2094533656289492f32,
          7.2191685204621621f32,
          7.2288186904958804f32,
          7.2384047393250794f32,
          7.2479275134435861f32,
          7.2573878426926521f32,
          7.2667865406949019f32,
          7.2761244052742384f32,
          7.2854022188622487f32,
          7.2946207488916270f32,
          7.3037807481771031f32,
          7.3128829552843557f32,
          7.3219280948873617f32,
          7.3309168781146177f32,
          7.3398500028846243f32,
          7.3487281542310781f32,
          7.3575520046180847f32,
          7.3663222142458151f32,
          7.3750394313469254f32,
          7.3837042924740528f32,
          7.3923174227787607f32,
          7.4008794362821844f32,
          7.4093909361377026f32,
          7.4178525148858991f32,
          7.4262647547020979f32,
          7.4346282276367255f32,
          7.4429434958487288f32,
          7.4512111118323299f32,
          7.4594316186372973f32,
          7.4676055500829976f32,
          7.4757334309663976f32,
          7.4838157772642564f32,
          7.4918530963296748f32,
          7.4998458870832057f32,
          7.5077946401986964f32,
          7.5156998382840436f32,
          7.5235619560570131f32,
          7.5313814605163119f32,
          7.5391588111080319f32,
          7.5468944598876373f32,
          7.5545888516776376f32,
          7.5622424242210728f32,
          7.5698556083309478f32,
          7.5774288280357487f32,
          7.5849625007211561f32,
          7.5924570372680806f32,
          7.5999128421871278f32,
          7.6073303137496113f32,
          7.6147098441152075f32,
          7.6220518194563764f32,
          7.6293566200796095f32,
          7.6366246205436488f32,
          7.6438561897747244f32,
          7.6510516911789290f32,
          7.6582114827517955f32,
          7.6653359171851765f32,
          7.6724253419714952f32,
          7.6794800995054464f32,
          7.6865005271832185f32,
          7.6934869574993252f32,
          7.7004397181410926f32,
          7.7073591320808825f32,
          7.7142455176661224f32,
          7.7210991887071856f32,
          7.7279204545631996f32,
          7.7347096202258392f32,
          7.7414669864011465f32,
          7.7481928495894596f32,
          7.7548875021634691f32,
          7.7615512324444795f32,
          7.7681843247769260f32,
          7.7747870596011737f32,
          7.7813597135246608f32,
          7.7879025593914317f32,
          7.7944158663501062f32,
          7.8008998999203047f32,
          7.8073549220576037f32,
          7.8137811912170374f32,
          7.8201789624151887f32,
          7.8265484872909159f32,
          7.8328900141647422f32,
          7.8392037880969445f32,
          7.8454900509443757f32,
          7.8517490414160571f32,
          7.8579809951275719f32,
          7.8641861446542798f32,
          7.8703647195834048f32,
          7.8765169465650002f32,
          7.8826430493618425f32,
          7.8887432488982601f32,
          7.8948177633079446f32,
          7.9008668079807496f32,
          7.9068905956085187f32,
          7.9128893362299619f32,
          7.9188632372745955f32,
          7.9248125036057813f32,
          7.9307373375628867f32,
          7.9366379390025719f32,
          7.9425145053392399f32,
          7.9483672315846778f32,
          7.9541963103868758f32,
          7.9600019320680806f32,
          7.9657842846620870f32,
          7.9715435539507720f32,
          7.9772799234999168f32,
          7.9829935746943104f32,
          7.9886846867721664f32,
          7.9943534368588578f32
      ];

static mut kInsBase
    : [u32; 24]
    = [   0i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          8i32 as (u32),
          10i32 as (u32),
          14i32 as (u32),
          18i32 as (u32),
          26i32 as (u32),
          34i32 as (u32),
          50i32 as (u32),
          66i32 as (u32),
          98i32 as (u32),
          130i32 as (u32),
          194i32 as (u32),
          322i32 as (u32),
          578i32 as (u32),
          1090i32 as (u32),
          2114i32 as (u32),
          6210i32 as (u32),
          22594i32 as (u32)
      ];

static mut kInsExtra
    : [u32; 24]
    = [   0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          1i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          7i32 as (u32),
          8i32 as (u32),
          9i32 as (u32),
          10i32 as (u32),
          12i32 as (u32),
          14i32 as (u32),
          24i32 as (u32)
      ];

static mut kCopyBase
    : [u32; 24]
    = [   2i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          7i32 as (u32),
          8i32 as (u32),
          9i32 as (u32),
          10i32 as (u32),
          12i32 as (u32),
          14i32 as (u32),
          18i32 as (u32),
          22i32 as (u32),
          30i32 as (u32),
          38i32 as (u32),
          54i32 as (u32),
          70i32 as (u32),
          102i32 as (u32),
          134i32 as (u32),
          198i32 as (u32),
          326i32 as (u32),
          582i32 as (u32),
          1094i32 as (u32),
          2118i32 as (u32)
      ];

static mut kCopyExtra
    : [u32; 24]
    = [   0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          0i32 as (u32),
          1i32 as (u32),
          1i32 as (u32),
          2i32 as (u32),
          2i32 as (u32),
          3i32 as (u32),
          3i32 as (u32),
          4i32 as (u32),
          4i32 as (u32),
          5i32 as (u32),
          5i32 as (u32),
          6i32 as (u32),
          7i32 as (u32),
          8i32 as (u32),
          9i32 as (u32),
          10i32 as (u32),
          24i32 as (u32)
      ];

static kBrotliMinWindowBits : i32 = 10i32;

static kBrotliMaxWindowBits : i32 = 24i32;

static mut kUTF8ContextLookup
    : [u8; 512]
    = [   0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          4i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          8i32 as (u8),
          12i32 as (u8),
          16i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          20i32 as (u8),
          12i32 as (u8),
          16i32 as (u8),
          24i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          32i32 as (u8),
          12i32 as (u8),
          36i32 as (u8),
          12i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          44i32 as (u8),
          32i32 as (u8),
          32i32 as (u8),
          24i32 as (u8),
          40i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          48i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          52i32 as (u8),
          24i32 as (u8),
          12i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          12i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          56i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          60i32 as (u8),
          24i32 as (u8),
          12i32 as (u8),
          28i32 as (u8),
          12i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          0i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8)
      ];

static mut kSigned3BitContextLookup
    : [u8; 256]
    = [   0i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          1i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          2i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          3i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          4i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          5i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          6i32 as (u8),
          7i32 as (u8)
      ];

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramLiteral {
    pub data_ : [u32; 256],
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramPair {
    pub idx1 : u32,
    pub idx2 : u32,
    pub cost_combo : f64,
    pub cost_diff : f64,
}

unsafe extern fn FastLog2(mut v : usize) -> f64 {
    if v < ::std::mem::size_of::<[f32; 256]>().wrapping_div(
               ::std::mem::size_of::<f32>()
           ) {
        kLog2Table[v] as (f64)
    } else {
        log2(v as (f64))
    }
}

unsafe extern fn ClusterCostDiff(
    mut size_a : usize, mut size_b : usize
) -> f64 {
    let mut size_c : usize = size_a.wrapping_add(size_b);
    size_a as (f64) * FastLog2(size_a) + size_b as (f64) * FastLog2(
                                                               size_b
                                                           ) - size_c as (f64) * FastLog2(size_c)
}

unsafe extern fn brotli_max_double(
    mut a : f64, mut b : f64
) -> f64 {
    if a > b { a } else { b }
}

unsafe extern fn HistogramAddHistogramLiteral(
    mut self : *mut HistogramLiteral, mut v : *const HistogramLiteral
) {
    let mut i : usize;
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               (*v).total_count_
                           );
    i = 0i32 as (usize);
    'loop1: loop {
        if i < 256i32 as (usize) {
            {
                let _rhs = (*v).data_[i];
                let _lhs = &mut (*self).data_[i];
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

unsafe extern fn HistogramPairIsLess(
    mut p1 : *const HistogramPair, mut p2 : *const HistogramPair
) -> i32 {
    if (*p1).cost_diff != (*p2).cost_diff {
        if !!((*p1).cost_diff > (*p2).cost_diff) { 1i32 } else { 0i32 }
    } else if !!((*p1).idx2.wrapping_sub(
                     (*p1).idx1
                 ) > (*p2).idx2.wrapping_sub((*p2).idx1)) {
        1i32
    } else {
        0i32
    }
}

#[no_mangle]
pub unsafe extern fn BrotliCompareAndPushToQueueLiteral(
    mut out : *const HistogramLiteral,
    mut cluster_size : *const u32,
    mut idx1 : u32,
    mut idx2 : u32,
    mut max_num_pairs : usize,
    mut pairs : *mut HistogramPair,
    mut num_pairs : *mut usize
) {
    let mut is_good_pair : i32 = 0i32;
    let mut p : HistogramPair;
    if idx1 == idx2 {
    } else {
        if idx2 < idx1 {
            let mut t : u32 = idx2;
            idx2 = idx1;
            idx1 = t;
        }
        p.idx1 = idx1;
        p.idx2 = idx2;
        p.cost_diff = 0.5f64 * ClusterCostDiff(
                                   *cluster_size.offset(idx1 as (isize)) as (usize),
                                   *cluster_size.offset(idx2 as (isize)) as (usize)
                               );
        p.cost_diff = p.cost_diff - (*out.offset(
                                          idx1 as (isize)
                                      )).bit_cost_;
        p.cost_diff = p.cost_diff - (*out.offset(
                                          idx2 as (isize)
                                      )).bit_cost_;
        if (*out.offset(idx1 as (isize))).total_count_ == 0i32 as (usize) {
            p.cost_combo = (*out.offset(idx2 as (isize))).bit_cost_;
            is_good_pair = 1i32;
        } else if (*out.offset(
                        idx2 as (isize)
                    )).total_count_ == 0i32 as (usize) {
            p.cost_combo = (*out.offset(idx1 as (isize))).bit_cost_;
            is_good_pair = 1i32;
        } else {
            let mut threshold
                : f64
                = if *num_pairs == 0i32 as (usize) {
                      1e99f64
                  } else {
                      brotli_max_double(
                          0.0f64,
                          (*pairs.offset(0i32 as (isize))).cost_diff
                      )
                  };
            let mut combo : HistogramLiteral = *out.offset(idx1 as (isize));
            let mut cost_combo : f64;
            HistogramAddHistogramLiteral(
                &mut combo as (*mut HistogramLiteral),
                &*out.offset(idx2 as (isize)) as (*const HistogramLiteral)
            );
            cost_combo = BrotliPopulationCostLiteral(
                             &mut combo as (*mut HistogramLiteral) as (*const HistogramLiteral)
                         );
            if cost_combo < threshold - p.cost_diff {
                p.cost_combo = cost_combo;
                is_good_pair = 1i32;
            }
        }
        if is_good_pair != 0 {
            p.cost_diff = p.cost_diff + p.cost_combo;
            if *num_pairs > 0i32 as (usize) && (HistogramPairIsLess(
                                                    &mut *pairs.offset(
                                                              0i32 as (isize)
                                                          ) as (*mut HistogramPair) as (*const HistogramPair),
                                                    &mut p as (*mut HistogramPair) as (*const HistogramPair)
                                                ) != 0) {
                if *num_pairs < max_num_pairs {
                    *pairs.offset(*num_pairs as (isize)) = *pairs.offset(
                                                                0i32 as (isize)
                                                            );
                    *num_pairs = (*num_pairs).wrapping_add(1 as (usize));
                }
                *pairs.offset(0i32 as (isize)) = p;
            } else if *num_pairs < max_num_pairs {
                *pairs.offset(*num_pairs as (isize)) = p;
                *num_pairs = (*num_pairs).wrapping_add(1 as (usize));
            }
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliHistogramCombineLiteral(
    mut out : *mut HistogramLiteral,
    mut cluster_size : *mut u32,
    mut symbols : *mut u32,
    mut clusters : *mut u32,
    mut pairs : *mut HistogramPair,
    mut num_clusters : usize,
    mut symbols_size : usize,
    mut max_clusters : usize,
    mut max_num_pairs : usize
) -> usize {
    let mut cost_diff_threshold : f64 = 0.0f64;
    let mut min_cluster_size : usize = 1i32 as (usize);
    let mut num_pairs : usize = 0i32 as (usize);
    let mut idx1 : usize;
    idx1 = 0i32 as (usize);
    'loop1: loop {
        if idx1 < num_clusters {
            let mut idx2 : usize;
            idx2 = idx1.wrapping_add(1i32 as (usize));
            'loop29: loop {
                if idx2 < num_clusters {
                    BrotliCompareAndPushToQueueLiteral(
                        out as (*const HistogramLiteral),
                        cluster_size as (*const u32),
                        *clusters.offset(idx1 as (isize)),
                        *clusters.offset(idx2 as (isize)),
                        max_num_pairs,
                        &mut *pairs.offset(0i32 as (isize)) as (*mut HistogramPair),
                        &mut num_pairs as (*mut usize)
                    );
                    idx2 = idx2.wrapping_add(1 as (usize));
                    continue 'loop29;
                } else {
                    break 'loop29;
                }
            }
            idx1 = idx1.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    'loop2: loop {
        if num_clusters > min_cluster_size {
            let mut best_idx1 : u32;
            let mut best_idx2 : u32;
            let mut i : usize;
            if (*pairs.offset(
                     0i32 as (isize)
                 )).cost_diff >= cost_diff_threshold {
                cost_diff_threshold = 1e99f64;
                min_cluster_size = max_clusters;
                continue 'loop2;
            } else {
                best_idx1 = (*pairs.offset(0i32 as (isize))).idx1;
                best_idx2 = (*pairs.offset(0i32 as (isize))).idx2;
                HistogramAddHistogramLiteral(
                    &mut *out.offset(best_idx1 as (isize)) as (*mut HistogramLiteral),
                    &mut *out.offset(
                              best_idx2 as (isize)
                          ) as (*mut HistogramLiteral) as (*const HistogramLiteral)
                );
                (*out.offset(best_idx1 as (isize))).bit_cost_ = (*pairs.offset(
                                                                      0i32 as (isize)
                                                                  )).cost_combo;
                {
                    let _rhs = *cluster_size.offset(best_idx2 as (isize));
                    let _lhs = &mut *cluster_size.offset(best_idx1 as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                }
                i = 0i32 as (usize);
                'loop6: loop {
                    if i < symbols_size {
                        if *symbols.offset(i as (isize)) == best_idx2 {
                            *symbols.offset(i as (isize)) = best_idx1;
                        }
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop6;
                    } else {
                        break 'loop6;
                    }
                }
                i = 0i32 as (usize);
                'loop8: loop {
                    if i < num_clusters {
                        if *clusters.offset(i as (isize)) == best_idx2 {
                            memmove(
                                &mut *clusters.offset(
                                          i as (isize)
                                      ) as (*mut u32) as (*mut ::std::os::raw::c_void),
                                &mut *clusters.offset(
                                          i.wrapping_add(1i32 as (usize)) as (isize)
                                      ) as (*mut u32) as (*const ::std::os::raw::c_void),
                                num_clusters.wrapping_sub(i).wrapping_sub(
                                    1i32 as (usize)
                                ).wrapping_mul(
                                    ::std::mem::size_of::<u32>()
                                )
                            );
                        } else {
                            i = i.wrapping_add(1 as (usize));
                            continue 'loop8;
                        }
                    } else {
                        break 'loop8;
                    }
                }
                num_clusters = num_clusters.wrapping_sub(1 as (usize));
                let mut copy_to_idx : usize = 0i32 as (usize);
                i = 0i32 as (usize);
                'loop13: loop {
                    if i < num_pairs {
                        let mut p
                            : *mut HistogramPair
                            = &mut *pairs.offset(i as (isize)) as (*mut HistogramPair);
                        if !((*p).idx1 == best_idx1 || (*p).idx2 == best_idx1 || (*p).idx1 == best_idx2 || (*p).idx2 == best_idx2) {
                            if HistogramPairIsLess(
                                   &mut *pairs.offset(
                                             0i32 as (isize)
                                         ) as (*mut HistogramPair) as (*const HistogramPair),
                                   p as (*const HistogramPair)
                               ) != 0 {
                                let mut front : HistogramPair = *pairs.offset(0i32 as (isize));
                                *pairs.offset(0i32 as (isize)) = *p;
                                *pairs.offset(copy_to_idx as (isize)) = front;
                            } else {
                                *pairs.offset(copy_to_idx as (isize)) = *p;
                            }
                            copy_to_idx = copy_to_idx.wrapping_add(1 as (usize));
                        }
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop13;
                    } else {
                        break 'loop13;
                    }
                }
                num_pairs = copy_to_idx;
                i = 0i32 as (usize);
                'loop15: loop {
                    if i < num_clusters {
                        BrotliCompareAndPushToQueueLiteral(
                            out as (*const HistogramLiteral),
                            cluster_size as (*const u32),
                            best_idx1,
                            *clusters.offset(i as (isize)),
                            max_num_pairs,
                            &mut *pairs.offset(0i32 as (isize)) as (*mut HistogramPair),
                            &mut num_pairs as (*mut usize)
                        );
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop15;
                    } else {
                        continue 'loop2;
                    }
                }
            }
        } else {
            break 'loop2;
        }
    }
    num_clusters
}

#[no_mangle]
pub unsafe extern fn BrotliHistogramBitCostDistanceLiteral(
    mut histogram : *const HistogramLiteral,
    mut candidate : *const HistogramLiteral
) -> f64 {
    if (*histogram).total_count_ == 0i32 as (usize) {
        0.0f64
    } else {
        let mut tmp : HistogramLiteral = *histogram;
        HistogramAddHistogramLiteral(
            &mut tmp as (*mut HistogramLiteral),
            candidate
        );
        BrotliPopulationCostLiteral(
            &mut tmp as (*mut HistogramLiteral) as (*const HistogramLiteral)
        ) - (*candidate).bit_cost_
    }
}

unsafe extern fn HistogramClearLiteral(
    mut self : *mut HistogramLiteral
) {
    memset(
        (*self).data_.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 256]>()
    );
    (*self).total_count_ = 0i32 as (usize);
    (*self).bit_cost_ = 3.402e+38f64;
}

#[no_mangle]
pub unsafe extern fn BrotliHistogramRemapLiteral(
    mut in_ : *const HistogramLiteral,
    mut in_size : usize,
    mut clusters : *const u32,
    mut num_clusters : usize,
    mut out : *mut HistogramLiteral,
    mut symbols : *mut u32
) {
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < in_size {
            let mut best_out
                : u32
                = if i == 0i32 as (usize) {
                      *symbols.offset(0i32 as (isize))
                  } else {
                      *symbols.offset(i.wrapping_sub(1i32 as (usize)) as (isize))
                  };
            let mut best_bits
                : f64
                = BrotliHistogramBitCostDistanceLiteral(
                      &*in_.offset(i as (isize)) as (*const HistogramLiteral),
                      &mut *out.offset(
                                best_out as (isize)
                            ) as (*mut HistogramLiteral) as (*const HistogramLiteral)
                  );
            let mut j : usize;
            j = 0i32 as (usize);
            'loop12: loop {
                if j < num_clusters {
                    let cur_bits
                        : f64
                        = BrotliHistogramBitCostDistanceLiteral(
                              &*in_.offset(i as (isize)) as (*const HistogramLiteral),
                              &mut *out.offset(
                                        *clusters.offset(j as (isize)) as (isize)
                                    ) as (*mut HistogramLiteral) as (*const HistogramLiteral)
                          );
                    if cur_bits < best_bits {
                        best_bits = cur_bits;
                        best_out = *clusters.offset(j as (isize));
                    }
                    j = j.wrapping_add(1 as (usize));
                    continue 'loop12;
                } else {
                    break 'loop12;
                }
            }
            *symbols.offset(i as (isize)) = best_out;
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    i = 0i32 as (usize);
    'loop3: loop {
        if i < num_clusters {
            HistogramClearLiteral(
                &mut *out.offset(
                          *clusters.offset(i as (isize)) as (isize)
                      ) as (*mut HistogramLiteral)
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    i = 0i32 as (usize);
    'loop5: loop {
        if i < in_size {
            HistogramAddHistogramLiteral(
                &mut *out.offset(
                          *symbols.offset(i as (isize)) as (isize)
                      ) as (*mut HistogramLiteral),
                &*in_.offset(i as (isize)) as (*const HistogramLiteral)
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop5;
        } else {
            break 'loop5;
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryManager {
    pub alloc_func : unsafe extern fn(*mut ::std::os::raw::c_void, usize) -> *mut ::std::os::raw::c_void,
    pub free_func : unsafe extern fn(*mut ::std::os::raw::c_void, *mut ::std::os::raw::c_void),
    pub opaque : *mut ::std::os::raw::c_void,
}

#[no_mangle]
pub unsafe extern fn BrotliHistogramReindexLiteral(
    mut m : *mut MemoryManager,
    mut out : *mut HistogramLiteral,
    mut symbols : *mut u32,
    mut length : usize
) -> usize {
    static kInvalidIndex : u32 = !(0i32 as (u32));
    let mut new_index
        : *mut u32
        = if length != 0 {
              BrotliAllocate(
                  m,
                  length.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut next_index : u32;
    let mut tmp : *mut HistogramLiteral;
    let mut i : usize;
    if !(0i32 == 0) {
        0i32 as (usize)
    } else {
        i = 0i32 as (usize);
        'loop2: loop {
            if i < length {
                *new_index.offset(i as (isize)) = kInvalidIndex;
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        next_index = 0i32 as (u32);
        i = 0i32 as (usize);
        'loop4: loop {
            if i < length {
                if *new_index.offset(
                        *symbols.offset(i as (isize)) as (isize)
                    ) == kInvalidIndex {
                    *new_index.offset(
                         *symbols.offset(i as (isize)) as (isize)
                     ) = next_index;
                    next_index = next_index.wrapping_add(1 as (u32));
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop4;
            } else {
                break 'loop4;
            }
        }
        tmp = if next_index != 0 {
                  BrotliAllocate(
                      m,
                      (next_index as (usize)).wrapping_mul(
                          ::std::mem::size_of::<HistogramLiteral>()
                      )
                  ) as (*mut HistogramLiteral)
              } else {
                  0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral)
              };
        if !(0i32 == 0) {
            0i32 as (usize)
        } else {
            next_index = 0i32 as (u32);
            i = 0i32 as (usize);
            'loop7: loop {
                if i < length {
                    if *new_index.offset(
                            *symbols.offset(i as (isize)) as (isize)
                        ) == next_index {
                        *tmp.offset(next_index as (isize)) = *out.offset(
                                                                  *symbols.offset(
                                                                       i as (isize)
                                                                   ) as (isize)
                                                              );
                        next_index = next_index.wrapping_add(1 as (u32));
                    }
                    *symbols.offset(i as (isize)) = *new_index.offset(
                                                         *symbols.offset(i as (isize)) as (isize)
                                                     );
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop7;
                } else {
                    break 'loop7;
                }
            }
            BrotliFree(m,new_index as (*mut ::std::os::raw::c_void));
            new_index = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
            i = 0i32 as (usize);
            'loop9: loop {
                if i < next_index as (usize) {
                    *out.offset(i as (isize)) = *tmp.offset(i as (isize));
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop9;
                } else {
                    break 'loop9;
                }
            }
            BrotliFree(m,tmp as (*mut ::std::os::raw::c_void));
            tmp = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramLiteral);
            next_index as (usize)
        }
    }
}

unsafe extern fn brotli_min_size_t(
    mut a : usize, mut b : usize
) -> usize {
    if a < b { a } else { b }
}

#[no_mangle]
pub unsafe extern fn BrotliClusterHistogramsLiteral(
    mut m : *mut MemoryManager,
    mut in_ : *const HistogramLiteral,
    in_size : usize,
    mut max_histograms : usize,
    mut out : *mut HistogramLiteral,
    mut out_size : *mut usize,
    mut histogram_symbols : *mut u32
) {
    let mut cluster_size
        : *mut u32
        = if in_size != 0 {
              BrotliAllocate(
                  m,
                  in_size.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut clusters
        : *mut u32
        = if in_size != 0 {
              BrotliAllocate(
                  m,
                  in_size.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut num_clusters : usize = 0i32 as (usize);
    let max_input_histograms : usize = 64i32 as (usize);
    let mut pairs_capacity
        : usize
        = max_input_histograms.wrapping_mul(
              max_input_histograms
          ).wrapping_div(
              2i32 as (usize)
          );
    let mut pairs
        : *mut HistogramPair
        = if pairs_capacity.wrapping_add(1i32 as (usize)) != 0 {
              BrotliAllocate(
                  m,
                  pairs_capacity.wrapping_add(1i32 as (usize)).wrapping_mul(
                      ::std::mem::size_of::<HistogramPair>()
                  )
              ) as (*mut HistogramPair)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
          };
    let mut i : usize;
    if !(0i32 == 0) {
    } else {
        i = 0i32 as (usize);
        'loop2: loop {
            if i < in_size {
                *cluster_size.offset(i as (isize)) = 1i32 as (u32);
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        i = 0i32 as (usize);
        'loop4: loop {
            if i < in_size {
                *out.offset(i as (isize)) = *in_.offset(i as (isize));
                (*out.offset(
                      i as (isize)
                  )).bit_cost_ = BrotliPopulationCostLiteral(
                                     &*in_.offset(i as (isize)) as (*const HistogramLiteral)
                                 );
                *histogram_symbols.offset(i as (isize)) = i as (u32);
                i = i.wrapping_add(1 as (usize));
                continue 'loop4;
            } else {
                break 'loop4;
            }
        }
        i = 0i32 as (usize);
        'loop6: loop {
            if i < in_size {
                let mut num_to_combine
                    : usize
                    = brotli_min_size_t(in_size.wrapping_sub(i),max_input_histograms);
                let mut num_new_clusters : usize;
                let mut j : usize;
                j = 0i32 as (usize);
                'loop20: loop {
                    if j < num_to_combine {
                        *clusters.offset(
                             num_clusters.wrapping_add(j) as (isize)
                         ) = i.wrapping_add(j) as (u32);
                        j = j.wrapping_add(1 as (usize));
                        continue 'loop20;
                    } else {
                        break 'loop20;
                    }
                }
                num_new_clusters = BrotliHistogramCombineLiteral(
                                       out,
                                       cluster_size,
                                       &mut *histogram_symbols.offset(i as (isize)) as (*mut u32),
                                       &mut *clusters.offset(num_clusters as (isize)) as (*mut u32),
                                       pairs,
                                       num_to_combine,
                                       num_to_combine,
                                       max_histograms,
                                       pairs_capacity
                                   );
                num_clusters = num_clusters.wrapping_add(num_new_clusters);
                i = i.wrapping_add(max_input_histograms);
                continue 'loop6;
            } else {
                break 'loop6;
            }
        }
        let mut max_num_pairs
            : usize
            = brotli_min_size_t(
                  (64i32 as (usize)).wrapping_mul(num_clusters),
                  num_clusters.wrapping_div(2i32 as (usize)).wrapping_mul(
                      num_clusters
                  )
              );
        if pairs_capacity < max_num_pairs.wrapping_add(1i32 as (usize)) {
            let mut _new_size
                : usize
                = if pairs_capacity == 0i32 as (usize) {
                      max_num_pairs.wrapping_add(1i32 as (usize))
                  } else {
                      pairs_capacity
                  };
            let mut new_array : *mut HistogramPair;
            'loop9: loop {
                if _new_size < max_num_pairs.wrapping_add(1i32 as (usize)) {
                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                    continue 'loop9;
                } else {
                    break 'loop9;
                }
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(::std::mem::size_of::<HistogramPair>())
                            ) as (*mut HistogramPair)
                        } else {
                            0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
                        };
            if !!(0i32 == 0) && (pairs_capacity != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut ::std::os::raw::c_void),
                    pairs as (*const ::std::os::raw::c_void),
                    pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>())
                );
            }
            BrotliFree(m,pairs as (*mut ::std::os::raw::c_void));
            pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
            pairs = new_array;
            pairs_capacity = _new_size;
        }
        if !(0i32 == 0) {
        } else {
            num_clusters = BrotliHistogramCombineLiteral(
                               out,
                               cluster_size,
                               histogram_symbols,
                               clusters,
                               pairs,
                               num_clusters,
                               in_size,
                               max_histograms,
                               max_num_pairs
                           );
            BrotliFree(m,pairs as (*mut ::std::os::raw::c_void));
            pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
            BrotliFree(m,cluster_size as (*mut ::std::os::raw::c_void));
            cluster_size = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
            BrotliHistogramRemapLiteral(
                in_,
                in_size,
                clusters as (*const u32),
                num_clusters,
                out,
                histogram_symbols
            );
            BrotliFree(m,clusters as (*mut ::std::os::raw::c_void));
            clusters = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
            *out_size = BrotliHistogramReindexLiteral(
                            m,
                            out,
                            histogram_symbols,
                            in_size
                        );
            if !(0i32 == 0) { }
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramCommand {
    pub data_ : [u32; 704],
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

unsafe extern fn HistogramAddHistogramCommand(
    mut self : *mut HistogramCommand, mut v : *const HistogramCommand
) {
    let mut i : usize;
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               (*v).total_count_
                           );
    i = 0i32 as (usize);
    'loop1: loop {
        if i < 704i32 as (usize) {
            {
                let _rhs = (*v).data_[i];
                let _lhs = &mut (*self).data_[i];
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliCompareAndPushToQueueCommand(
    mut out : *const HistogramCommand,
    mut cluster_size : *const u32,
    mut idx1 : u32,
    mut idx2 : u32,
    mut max_num_pairs : usize,
    mut pairs : *mut HistogramPair,
    mut num_pairs : *mut usize
) {
    let mut is_good_pair : i32 = 0i32;
    let mut p : HistogramPair;
    if idx1 == idx2 {
    } else {
        if idx2 < idx1 {
            let mut t : u32 = idx2;
            idx2 = idx1;
            idx1 = t;
        }
        p.idx1 = idx1;
        p.idx2 = idx2;
        p.cost_diff = 0.5f64 * ClusterCostDiff(
                                   *cluster_size.offset(idx1 as (isize)) as (usize),
                                   *cluster_size.offset(idx2 as (isize)) as (usize)
                               );
        p.cost_diff = p.cost_diff - (*out.offset(
                                          idx1 as (isize)
                                      )).bit_cost_;
        p.cost_diff = p.cost_diff - (*out.offset(
                                          idx2 as (isize)
                                      )).bit_cost_;
        if (*out.offset(idx1 as (isize))).total_count_ == 0i32 as (usize) {
            p.cost_combo = (*out.offset(idx2 as (isize))).bit_cost_;
            is_good_pair = 1i32;
        } else if (*out.offset(
                        idx2 as (isize)
                    )).total_count_ == 0i32 as (usize) {
            p.cost_combo = (*out.offset(idx1 as (isize))).bit_cost_;
            is_good_pair = 1i32;
        } else {
            let mut threshold
                : f64
                = if *num_pairs == 0i32 as (usize) {
                      1e99f64
                  } else {
                      brotli_max_double(
                          0.0f64,
                          (*pairs.offset(0i32 as (isize))).cost_diff
                      )
                  };
            let mut combo : HistogramCommand = *out.offset(idx1 as (isize));
            let mut cost_combo : f64;
            HistogramAddHistogramCommand(
                &mut combo as (*mut HistogramCommand),
                &*out.offset(idx2 as (isize)) as (*const HistogramCommand)
            );
            cost_combo = BrotliPopulationCostCommand(
                             &mut combo as (*mut HistogramCommand) as (*const HistogramCommand)
                         );
            if cost_combo < threshold - p.cost_diff {
                p.cost_combo = cost_combo;
                is_good_pair = 1i32;
            }
        }
        if is_good_pair != 0 {
            p.cost_diff = p.cost_diff + p.cost_combo;
            if *num_pairs > 0i32 as (usize) && (HistogramPairIsLess(
                                                    &mut *pairs.offset(
                                                              0i32 as (isize)
                                                          ) as (*mut HistogramPair) as (*const HistogramPair),
                                                    &mut p as (*mut HistogramPair) as (*const HistogramPair)
                                                ) != 0) {
                if *num_pairs < max_num_pairs {
                    *pairs.offset(*num_pairs as (isize)) = *pairs.offset(
                                                                0i32 as (isize)
                                                            );
                    *num_pairs = (*num_pairs).wrapping_add(1 as (usize));
                }
                *pairs.offset(0i32 as (isize)) = p;
            } else if *num_pairs < max_num_pairs {
                *pairs.offset(*num_pairs as (isize)) = p;
                *num_pairs = (*num_pairs).wrapping_add(1 as (usize));
            }
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliHistogramCombineCommand(
    mut out : *mut HistogramCommand,
    mut cluster_size : *mut u32,
    mut symbols : *mut u32,
    mut clusters : *mut u32,
    mut pairs : *mut HistogramPair,
    mut num_clusters : usize,
    mut symbols_size : usize,
    mut max_clusters : usize,
    mut max_num_pairs : usize
) -> usize {
    let mut cost_diff_threshold : f64 = 0.0f64;
    let mut min_cluster_size : usize = 1i32 as (usize);
    let mut num_pairs : usize = 0i32 as (usize);
    let mut idx1 : usize;
    idx1 = 0i32 as (usize);
    'loop1: loop {
        if idx1 < num_clusters {
            let mut idx2 : usize;
            idx2 = idx1.wrapping_add(1i32 as (usize));
            'loop29: loop {
                if idx2 < num_clusters {
                    BrotliCompareAndPushToQueueCommand(
                        out as (*const HistogramCommand),
                        cluster_size as (*const u32),
                        *clusters.offset(idx1 as (isize)),
                        *clusters.offset(idx2 as (isize)),
                        max_num_pairs,
                        &mut *pairs.offset(0i32 as (isize)) as (*mut HistogramPair),
                        &mut num_pairs as (*mut usize)
                    );
                    idx2 = idx2.wrapping_add(1 as (usize));
                    continue 'loop29;
                } else {
                    break 'loop29;
                }
            }
            idx1 = idx1.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    'loop2: loop {
        if num_clusters > min_cluster_size {
            let mut best_idx1 : u32;
            let mut best_idx2 : u32;
            let mut i : usize;
            if (*pairs.offset(
                     0i32 as (isize)
                 )).cost_diff >= cost_diff_threshold {
                cost_diff_threshold = 1e99f64;
                min_cluster_size = max_clusters;
                continue 'loop2;
            } else {
                best_idx1 = (*pairs.offset(0i32 as (isize))).idx1;
                best_idx2 = (*pairs.offset(0i32 as (isize))).idx2;
                HistogramAddHistogramCommand(
                    &mut *out.offset(best_idx1 as (isize)) as (*mut HistogramCommand),
                    &mut *out.offset(
                              best_idx2 as (isize)
                          ) as (*mut HistogramCommand) as (*const HistogramCommand)
                );
                (*out.offset(best_idx1 as (isize))).bit_cost_ = (*pairs.offset(
                                                                      0i32 as (isize)
                                                                  )).cost_combo;
                {
                    let _rhs = *cluster_size.offset(best_idx2 as (isize));
                    let _lhs = &mut *cluster_size.offset(best_idx1 as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                }
                i = 0i32 as (usize);
                'loop6: loop {
                    if i < symbols_size {
                        if *symbols.offset(i as (isize)) == best_idx2 {
                            *symbols.offset(i as (isize)) = best_idx1;
                        }
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop6;
                    } else {
                        break 'loop6;
                    }
                }
                i = 0i32 as (usize);
                'loop8: loop {
                    if i < num_clusters {
                        if *clusters.offset(i as (isize)) == best_idx2 {
                            memmove(
                                &mut *clusters.offset(
                                          i as (isize)
                                      ) as (*mut u32) as (*mut ::std::os::raw::c_void),
                                &mut *clusters.offset(
                                          i.wrapping_add(1i32 as (usize)) as (isize)
                                      ) as (*mut u32) as (*const ::std::os::raw::c_void),
                                num_clusters.wrapping_sub(i).wrapping_sub(
                                    1i32 as (usize)
                                ).wrapping_mul(
                                    ::std::mem::size_of::<u32>()
                                )
                            );
                        } else {
                            i = i.wrapping_add(1 as (usize));
                            continue 'loop8;
                        }
                    } else {
                        break 'loop8;
                    }
                }
                num_clusters = num_clusters.wrapping_sub(1 as (usize));
                let mut copy_to_idx : usize = 0i32 as (usize);
                i = 0i32 as (usize);
                'loop13: loop {
                    if i < num_pairs {
                        let mut p
                            : *mut HistogramPair
                            = &mut *pairs.offset(i as (isize)) as (*mut HistogramPair);
                        if !((*p).idx1 == best_idx1 || (*p).idx2 == best_idx1 || (*p).idx1 == best_idx2 || (*p).idx2 == best_idx2) {
                            if HistogramPairIsLess(
                                   &mut *pairs.offset(
                                             0i32 as (isize)
                                         ) as (*mut HistogramPair) as (*const HistogramPair),
                                   p as (*const HistogramPair)
                               ) != 0 {
                                let mut front : HistogramPair = *pairs.offset(0i32 as (isize));
                                *pairs.offset(0i32 as (isize)) = *p;
                                *pairs.offset(copy_to_idx as (isize)) = front;
                            } else {
                                *pairs.offset(copy_to_idx as (isize)) = *p;
                            }
                            copy_to_idx = copy_to_idx.wrapping_add(1 as (usize));
                        }
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop13;
                    } else {
                        break 'loop13;
                    }
                }
                num_pairs = copy_to_idx;
                i = 0i32 as (usize);
                'loop15: loop {
                    if i < num_clusters {
                        BrotliCompareAndPushToQueueCommand(
                            out as (*const HistogramCommand),
                            cluster_size as (*const u32),
                            best_idx1,
                            *clusters.offset(i as (isize)),
                            max_num_pairs,
                            &mut *pairs.offset(0i32 as (isize)) as (*mut HistogramPair),
                            &mut num_pairs as (*mut usize)
                        );
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop15;
                    } else {
                        continue 'loop2;
                    }
                }
            }
        } else {
            break 'loop2;
        }
    }
    num_clusters
}

#[no_mangle]
pub unsafe extern fn BrotliHistogramBitCostDistanceCommand(
    mut histogram : *const HistogramCommand,
    mut candidate : *const HistogramCommand
) -> f64 {
    if (*histogram).total_count_ == 0i32 as (usize) {
        0.0f64
    } else {
        let mut tmp : HistogramCommand = *histogram;
        HistogramAddHistogramCommand(
            &mut tmp as (*mut HistogramCommand),
            candidate
        );
        BrotliPopulationCostCommand(
            &mut tmp as (*mut HistogramCommand) as (*const HistogramCommand)
        ) - (*candidate).bit_cost_
    }
}

unsafe extern fn HistogramClearCommand(
    mut self : *mut HistogramCommand
) {
    memset(
        (*self).data_.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 704]>()
    );
    (*self).total_count_ = 0i32 as (usize);
    (*self).bit_cost_ = 3.402e+38f64;
}

#[no_mangle]
pub unsafe extern fn BrotliHistogramRemapCommand(
    mut in_ : *const HistogramCommand,
    mut in_size : usize,
    mut clusters : *const u32,
    mut num_clusters : usize,
    mut out : *mut HistogramCommand,
    mut symbols : *mut u32
) {
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < in_size {
            let mut best_out
                : u32
                = if i == 0i32 as (usize) {
                      *symbols.offset(0i32 as (isize))
                  } else {
                      *symbols.offset(i.wrapping_sub(1i32 as (usize)) as (isize))
                  };
            let mut best_bits
                : f64
                = BrotliHistogramBitCostDistanceCommand(
                      &*in_.offset(i as (isize)) as (*const HistogramCommand),
                      &mut *out.offset(
                                best_out as (isize)
                            ) as (*mut HistogramCommand) as (*const HistogramCommand)
                  );
            let mut j : usize;
            j = 0i32 as (usize);
            'loop12: loop {
                if j < num_clusters {
                    let cur_bits
                        : f64
                        = BrotliHistogramBitCostDistanceCommand(
                              &*in_.offset(i as (isize)) as (*const HistogramCommand),
                              &mut *out.offset(
                                        *clusters.offset(j as (isize)) as (isize)
                                    ) as (*mut HistogramCommand) as (*const HistogramCommand)
                          );
                    if cur_bits < best_bits {
                        best_bits = cur_bits;
                        best_out = *clusters.offset(j as (isize));
                    }
                    j = j.wrapping_add(1 as (usize));
                    continue 'loop12;
                } else {
                    break 'loop12;
                }
            }
            *symbols.offset(i as (isize)) = best_out;
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    i = 0i32 as (usize);
    'loop3: loop {
        if i < num_clusters {
            HistogramClearCommand(
                &mut *out.offset(
                          *clusters.offset(i as (isize)) as (isize)
                      ) as (*mut HistogramCommand)
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    i = 0i32 as (usize);
    'loop5: loop {
        if i < in_size {
            HistogramAddHistogramCommand(
                &mut *out.offset(
                          *symbols.offset(i as (isize)) as (isize)
                      ) as (*mut HistogramCommand),
                &*in_.offset(i as (isize)) as (*const HistogramCommand)
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop5;
        } else {
            break 'loop5;
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliHistogramReindexCommand(
    mut m : *mut MemoryManager,
    mut out : *mut HistogramCommand,
    mut symbols : *mut u32,
    mut length : usize
) -> usize {
    static kInvalidIndex : u32 = !(0i32 as (u32));
    let mut new_index
        : *mut u32
        = if length != 0 {
              BrotliAllocate(
                  m,
                  length.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut next_index : u32;
    let mut tmp : *mut HistogramCommand;
    let mut i : usize;
    if !(0i32 == 0) {
        0i32 as (usize)
    } else {
        i = 0i32 as (usize);
        'loop2: loop {
            if i < length {
                *new_index.offset(i as (isize)) = kInvalidIndex;
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        next_index = 0i32 as (u32);
        i = 0i32 as (usize);
        'loop4: loop {
            if i < length {
                if *new_index.offset(
                        *symbols.offset(i as (isize)) as (isize)
                    ) == kInvalidIndex {
                    *new_index.offset(
                         *symbols.offset(i as (isize)) as (isize)
                     ) = next_index;
                    next_index = next_index.wrapping_add(1 as (u32));
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop4;
            } else {
                break 'loop4;
            }
        }
        tmp = if next_index != 0 {
                  BrotliAllocate(
                      m,
                      (next_index as (usize)).wrapping_mul(
                          ::std::mem::size_of::<HistogramCommand>()
                      )
                  ) as (*mut HistogramCommand)
              } else {
                  0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand)
              };
        if !(0i32 == 0) {
            0i32 as (usize)
        } else {
            next_index = 0i32 as (u32);
            i = 0i32 as (usize);
            'loop7: loop {
                if i < length {
                    if *new_index.offset(
                            *symbols.offset(i as (isize)) as (isize)
                        ) == next_index {
                        *tmp.offset(next_index as (isize)) = *out.offset(
                                                                  *symbols.offset(
                                                                       i as (isize)
                                                                   ) as (isize)
                                                              );
                        next_index = next_index.wrapping_add(1 as (u32));
                    }
                    *symbols.offset(i as (isize)) = *new_index.offset(
                                                         *symbols.offset(i as (isize)) as (isize)
                                                     );
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop7;
                } else {
                    break 'loop7;
                }
            }
            BrotliFree(m,new_index as (*mut ::std::os::raw::c_void));
            new_index = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
            i = 0i32 as (usize);
            'loop9: loop {
                if i < next_index as (usize) {
                    *out.offset(i as (isize)) = *tmp.offset(i as (isize));
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop9;
                } else {
                    break 'loop9;
                }
            }
            BrotliFree(m,tmp as (*mut ::std::os::raw::c_void));
            tmp = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramCommand);
            next_index as (usize)
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliClusterHistogramsCommand(
    mut m : *mut MemoryManager,
    mut in_ : *const HistogramCommand,
    in_size : usize,
    mut max_histograms : usize,
    mut out : *mut HistogramCommand,
    mut out_size : *mut usize,
    mut histogram_symbols : *mut u32
) {
    let mut cluster_size
        : *mut u32
        = if in_size != 0 {
              BrotliAllocate(
                  m,
                  in_size.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut clusters
        : *mut u32
        = if in_size != 0 {
              BrotliAllocate(
                  m,
                  in_size.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut num_clusters : usize = 0i32 as (usize);
    let max_input_histograms : usize = 64i32 as (usize);
    let mut pairs_capacity
        : usize
        = max_input_histograms.wrapping_mul(
              max_input_histograms
          ).wrapping_div(
              2i32 as (usize)
          );
    let mut pairs
        : *mut HistogramPair
        = if pairs_capacity.wrapping_add(1i32 as (usize)) != 0 {
              BrotliAllocate(
                  m,
                  pairs_capacity.wrapping_add(1i32 as (usize)).wrapping_mul(
                      ::std::mem::size_of::<HistogramPair>()
                  )
              ) as (*mut HistogramPair)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
          };
    let mut i : usize;
    if !(0i32 == 0) {
    } else {
        i = 0i32 as (usize);
        'loop2: loop {
            if i < in_size {
                *cluster_size.offset(i as (isize)) = 1i32 as (u32);
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        i = 0i32 as (usize);
        'loop4: loop {
            if i < in_size {
                *out.offset(i as (isize)) = *in_.offset(i as (isize));
                (*out.offset(
                      i as (isize)
                  )).bit_cost_ = BrotliPopulationCostCommand(
                                     &*in_.offset(i as (isize)) as (*const HistogramCommand)
                                 );
                *histogram_symbols.offset(i as (isize)) = i as (u32);
                i = i.wrapping_add(1 as (usize));
                continue 'loop4;
            } else {
                break 'loop4;
            }
        }
        i = 0i32 as (usize);
        'loop6: loop {
            if i < in_size {
                let mut num_to_combine
                    : usize
                    = brotli_min_size_t(in_size.wrapping_sub(i),max_input_histograms);
                let mut num_new_clusters : usize;
                let mut j : usize;
                j = 0i32 as (usize);
                'loop20: loop {
                    if j < num_to_combine {
                        *clusters.offset(
                             num_clusters.wrapping_add(j) as (isize)
                         ) = i.wrapping_add(j) as (u32);
                        j = j.wrapping_add(1 as (usize));
                        continue 'loop20;
                    } else {
                        break 'loop20;
                    }
                }
                num_new_clusters = BrotliHistogramCombineCommand(
                                       out,
                                       cluster_size,
                                       &mut *histogram_symbols.offset(i as (isize)) as (*mut u32),
                                       &mut *clusters.offset(num_clusters as (isize)) as (*mut u32),
                                       pairs,
                                       num_to_combine,
                                       num_to_combine,
                                       max_histograms,
                                       pairs_capacity
                                   );
                num_clusters = num_clusters.wrapping_add(num_new_clusters);
                i = i.wrapping_add(max_input_histograms);
                continue 'loop6;
            } else {
                break 'loop6;
            }
        }
        let mut max_num_pairs
            : usize
            = brotli_min_size_t(
                  (64i32 as (usize)).wrapping_mul(num_clusters),
                  num_clusters.wrapping_div(2i32 as (usize)).wrapping_mul(
                      num_clusters
                  )
              );
        if pairs_capacity < max_num_pairs.wrapping_add(1i32 as (usize)) {
            let mut _new_size
                : usize
                = if pairs_capacity == 0i32 as (usize) {
                      max_num_pairs.wrapping_add(1i32 as (usize))
                  } else {
                      pairs_capacity
                  };
            let mut new_array : *mut HistogramPair;
            'loop9: loop {
                if _new_size < max_num_pairs.wrapping_add(1i32 as (usize)) {
                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                    continue 'loop9;
                } else {
                    break 'loop9;
                }
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(::std::mem::size_of::<HistogramPair>())
                            ) as (*mut HistogramPair)
                        } else {
                            0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
                        };
            if !!(0i32 == 0) && (pairs_capacity != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut ::std::os::raw::c_void),
                    pairs as (*const ::std::os::raw::c_void),
                    pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>())
                );
            }
            BrotliFree(m,pairs as (*mut ::std::os::raw::c_void));
            pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
            pairs = new_array;
            pairs_capacity = _new_size;
        }
        if !(0i32 == 0) {
        } else {
            num_clusters = BrotliHistogramCombineCommand(
                               out,
                               cluster_size,
                               histogram_symbols,
                               clusters,
                               pairs,
                               num_clusters,
                               in_size,
                               max_histograms,
                               max_num_pairs
                           );
            BrotliFree(m,pairs as (*mut ::std::os::raw::c_void));
            pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
            BrotliFree(m,cluster_size as (*mut ::std::os::raw::c_void));
            cluster_size = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
            BrotliHistogramRemapCommand(
                in_,
                in_size,
                clusters as (*const u32),
                num_clusters,
                out,
                histogram_symbols
            );
            BrotliFree(m,clusters as (*mut ::std::os::raw::c_void));
            clusters = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
            *out_size = BrotliHistogramReindexCommand(
                            m,
                            out,
                            histogram_symbols,
                            in_size
                        );
            if !(0i32 == 0) { }
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct HistogramDistance {
    pub data_ : [u32; 520],
    pub total_count_ : usize,
    pub bit_cost_ : f64,
}

unsafe extern fn HistogramAddHistogramDistance(
    mut self : *mut HistogramDistance, mut v : *const HistogramDistance
) {
    let mut i : usize;
    (*self).total_count_ = (*self).total_count_.wrapping_add(
                               (*v).total_count_
                           );
    i = 0i32 as (usize);
    'loop1: loop {
        if i < 520i32 as (usize) {
            {
                let _rhs = (*v).data_[i];
                let _lhs = &mut (*self).data_[i];
                *_lhs = (*_lhs).wrapping_add(_rhs);
            }
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliCompareAndPushToQueueDistance(
    mut out : *const HistogramDistance,
    mut cluster_size : *const u32,
    mut idx1 : u32,
    mut idx2 : u32,
    mut max_num_pairs : usize,
    mut pairs : *mut HistogramPair,
    mut num_pairs : *mut usize
) {
    let mut is_good_pair : i32 = 0i32;
    let mut p : HistogramPair;
    if idx1 == idx2 {
    } else {
        if idx2 < idx1 {
            let mut t : u32 = idx2;
            idx2 = idx1;
            idx1 = t;
        }
        p.idx1 = idx1;
        p.idx2 = idx2;
        p.cost_diff = 0.5f64 * ClusterCostDiff(
                                   *cluster_size.offset(idx1 as (isize)) as (usize),
                                   *cluster_size.offset(idx2 as (isize)) as (usize)
                               );
        p.cost_diff = p.cost_diff - (*out.offset(
                                          idx1 as (isize)
                                      )).bit_cost_;
        p.cost_diff = p.cost_diff - (*out.offset(
                                          idx2 as (isize)
                                      )).bit_cost_;
        if (*out.offset(idx1 as (isize))).total_count_ == 0i32 as (usize) {
            p.cost_combo = (*out.offset(idx2 as (isize))).bit_cost_;
            is_good_pair = 1i32;
        } else if (*out.offset(
                        idx2 as (isize)
                    )).total_count_ == 0i32 as (usize) {
            p.cost_combo = (*out.offset(idx1 as (isize))).bit_cost_;
            is_good_pair = 1i32;
        } else {
            let mut threshold
                : f64
                = if *num_pairs == 0i32 as (usize) {
                      1e99f64
                  } else {
                      brotli_max_double(
                          0.0f64,
                          (*pairs.offset(0i32 as (isize))).cost_diff
                      )
                  };
            let mut combo : HistogramDistance = *out.offset(idx1 as (isize));
            let mut cost_combo : f64;
            HistogramAddHistogramDistance(
                &mut combo as (*mut HistogramDistance),
                &*out.offset(idx2 as (isize)) as (*const HistogramDistance)
            );
            cost_combo = BrotliPopulationCostDistance(
                             &mut combo as (*mut HistogramDistance) as (*const HistogramDistance)
                         );
            if cost_combo < threshold - p.cost_diff {
                p.cost_combo = cost_combo;
                is_good_pair = 1i32;
            }
        }
        if is_good_pair != 0 {
            p.cost_diff = p.cost_diff + p.cost_combo;
            if *num_pairs > 0i32 as (usize) && (HistogramPairIsLess(
                                                    &mut *pairs.offset(
                                                              0i32 as (isize)
                                                          ) as (*mut HistogramPair) as (*const HistogramPair),
                                                    &mut p as (*mut HistogramPair) as (*const HistogramPair)
                                                ) != 0) {
                if *num_pairs < max_num_pairs {
                    *pairs.offset(*num_pairs as (isize)) = *pairs.offset(
                                                                0i32 as (isize)
                                                            );
                    *num_pairs = (*num_pairs).wrapping_add(1 as (usize));
                }
                *pairs.offset(0i32 as (isize)) = p;
            } else if *num_pairs < max_num_pairs {
                *pairs.offset(*num_pairs as (isize)) = p;
                *num_pairs = (*num_pairs).wrapping_add(1 as (usize));
            }
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliHistogramCombineDistance(
    mut out : *mut HistogramDistance,
    mut cluster_size : *mut u32,
    mut symbols : *mut u32,
    mut clusters : *mut u32,
    mut pairs : *mut HistogramPair,
    mut num_clusters : usize,
    mut symbols_size : usize,
    mut max_clusters : usize,
    mut max_num_pairs : usize
) -> usize {
    let mut cost_diff_threshold : f64 = 0.0f64;
    let mut min_cluster_size : usize = 1i32 as (usize);
    let mut num_pairs : usize = 0i32 as (usize);
    let mut idx1 : usize;
    idx1 = 0i32 as (usize);
    'loop1: loop {
        if idx1 < num_clusters {
            let mut idx2 : usize;
            idx2 = idx1.wrapping_add(1i32 as (usize));
            'loop29: loop {
                if idx2 < num_clusters {
                    BrotliCompareAndPushToQueueDistance(
                        out as (*const HistogramDistance),
                        cluster_size as (*const u32),
                        *clusters.offset(idx1 as (isize)),
                        *clusters.offset(idx2 as (isize)),
                        max_num_pairs,
                        &mut *pairs.offset(0i32 as (isize)) as (*mut HistogramPair),
                        &mut num_pairs as (*mut usize)
                    );
                    idx2 = idx2.wrapping_add(1 as (usize));
                    continue 'loop29;
                } else {
                    break 'loop29;
                }
            }
            idx1 = idx1.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    'loop2: loop {
        if num_clusters > min_cluster_size {
            let mut best_idx1 : u32;
            let mut best_idx2 : u32;
            let mut i : usize;
            if (*pairs.offset(
                     0i32 as (isize)
                 )).cost_diff >= cost_diff_threshold {
                cost_diff_threshold = 1e99f64;
                min_cluster_size = max_clusters;
                continue 'loop2;
            } else {
                best_idx1 = (*pairs.offset(0i32 as (isize))).idx1;
                best_idx2 = (*pairs.offset(0i32 as (isize))).idx2;
                HistogramAddHistogramDistance(
                    &mut *out.offset(best_idx1 as (isize)) as (*mut HistogramDistance),
                    &mut *out.offset(
                              best_idx2 as (isize)
                          ) as (*mut HistogramDistance) as (*const HistogramDistance)
                );
                (*out.offset(best_idx1 as (isize))).bit_cost_ = (*pairs.offset(
                                                                      0i32 as (isize)
                                                                  )).cost_combo;
                {
                    let _rhs = *cluster_size.offset(best_idx2 as (isize));
                    let _lhs = &mut *cluster_size.offset(best_idx1 as (isize));
                    *_lhs = (*_lhs).wrapping_add(_rhs);
                }
                i = 0i32 as (usize);
                'loop6: loop {
                    if i < symbols_size {
                        if *symbols.offset(i as (isize)) == best_idx2 {
                            *symbols.offset(i as (isize)) = best_idx1;
                        }
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop6;
                    } else {
                        break 'loop6;
                    }
                }
                i = 0i32 as (usize);
                'loop8: loop {
                    if i < num_clusters {
                        if *clusters.offset(i as (isize)) == best_idx2 {
                            memmove(
                                &mut *clusters.offset(
                                          i as (isize)
                                      ) as (*mut u32) as (*mut ::std::os::raw::c_void),
                                &mut *clusters.offset(
                                          i.wrapping_add(1i32 as (usize)) as (isize)
                                      ) as (*mut u32) as (*const ::std::os::raw::c_void),
                                num_clusters.wrapping_sub(i).wrapping_sub(
                                    1i32 as (usize)
                                ).wrapping_mul(
                                    ::std::mem::size_of::<u32>()
                                )
                            );
                        } else {
                            i = i.wrapping_add(1 as (usize));
                            continue 'loop8;
                        }
                    } else {
                        break 'loop8;
                    }
                }
                num_clusters = num_clusters.wrapping_sub(1 as (usize));
                let mut copy_to_idx : usize = 0i32 as (usize);
                i = 0i32 as (usize);
                'loop13: loop {
                    if i < num_pairs {
                        let mut p
                            : *mut HistogramPair
                            = &mut *pairs.offset(i as (isize)) as (*mut HistogramPair);
                        if !((*p).idx1 == best_idx1 || (*p).idx2 == best_idx1 || (*p).idx1 == best_idx2 || (*p).idx2 == best_idx2) {
                            if HistogramPairIsLess(
                                   &mut *pairs.offset(
                                             0i32 as (isize)
                                         ) as (*mut HistogramPair) as (*const HistogramPair),
                                   p as (*const HistogramPair)
                               ) != 0 {
                                let mut front : HistogramPair = *pairs.offset(0i32 as (isize));
                                *pairs.offset(0i32 as (isize)) = *p;
                                *pairs.offset(copy_to_idx as (isize)) = front;
                            } else {
                                *pairs.offset(copy_to_idx as (isize)) = *p;
                            }
                            copy_to_idx = copy_to_idx.wrapping_add(1 as (usize));
                        }
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop13;
                    } else {
                        break 'loop13;
                    }
                }
                num_pairs = copy_to_idx;
                i = 0i32 as (usize);
                'loop15: loop {
                    if i < num_clusters {
                        BrotliCompareAndPushToQueueDistance(
                            out as (*const HistogramDistance),
                            cluster_size as (*const u32),
                            best_idx1,
                            *clusters.offset(i as (isize)),
                            max_num_pairs,
                            &mut *pairs.offset(0i32 as (isize)) as (*mut HistogramPair),
                            &mut num_pairs as (*mut usize)
                        );
                        i = i.wrapping_add(1 as (usize));
                        continue 'loop15;
                    } else {
                        continue 'loop2;
                    }
                }
            }
        } else {
            break 'loop2;
        }
    }
    num_clusters
}

#[no_mangle]
pub unsafe extern fn BrotliHistogramBitCostDistanceDistance(
    mut histogram : *const HistogramDistance,
    mut candidate : *const HistogramDistance
) -> f64 {
    if (*histogram).total_count_ == 0i32 as (usize) {
        0.0f64
    } else {
        let mut tmp : HistogramDistance = *histogram;
        HistogramAddHistogramDistance(
            &mut tmp as (*mut HistogramDistance),
            candidate
        );
        BrotliPopulationCostDistance(
            &mut tmp as (*mut HistogramDistance) as (*const HistogramDistance)
        ) - (*candidate).bit_cost_
    }
}

unsafe extern fn HistogramClearDistance(
    mut self : *mut HistogramDistance
) {
    memset(
        (*self).data_.as_mut_ptr() as (*mut ::std::os::raw::c_void),
        0i32,
        ::std::mem::size_of::<[u32; 520]>()
    );
    (*self).total_count_ = 0i32 as (usize);
    (*self).bit_cost_ = 3.402e+38f64;
}

#[no_mangle]
pub unsafe extern fn BrotliHistogramRemapDistance(
    mut in_ : *const HistogramDistance,
    mut in_size : usize,
    mut clusters : *const u32,
    mut num_clusters : usize,
    mut out : *mut HistogramDistance,
    mut symbols : *mut u32
) {
    let mut i : usize;
    i = 0i32 as (usize);
    'loop1: loop {
        if i < in_size {
            let mut best_out
                : u32
                = if i == 0i32 as (usize) {
                      *symbols.offset(0i32 as (isize))
                  } else {
                      *symbols.offset(i.wrapping_sub(1i32 as (usize)) as (isize))
                  };
            let mut best_bits
                : f64
                = BrotliHistogramBitCostDistanceDistance(
                      &*in_.offset(i as (isize)) as (*const HistogramDistance),
                      &mut *out.offset(
                                best_out as (isize)
                            ) as (*mut HistogramDistance) as (*const HistogramDistance)
                  );
            let mut j : usize;
            j = 0i32 as (usize);
            'loop12: loop {
                if j < num_clusters {
                    let cur_bits
                        : f64
                        = BrotliHistogramBitCostDistanceDistance(
                              &*in_.offset(i as (isize)) as (*const HistogramDistance),
                              &mut *out.offset(
                                        *clusters.offset(j as (isize)) as (isize)
                                    ) as (*mut HistogramDistance) as (*const HistogramDistance)
                          );
                    if cur_bits < best_bits {
                        best_bits = cur_bits;
                        best_out = *clusters.offset(j as (isize));
                    }
                    j = j.wrapping_add(1 as (usize));
                    continue 'loop12;
                } else {
                    break 'loop12;
                }
            }
            *symbols.offset(i as (isize)) = best_out;
            i = i.wrapping_add(1 as (usize));
            continue 'loop1;
        } else {
            break 'loop1;
        }
    }
    i = 0i32 as (usize);
    'loop3: loop {
        if i < num_clusters {
            HistogramClearDistance(
                &mut *out.offset(
                          *clusters.offset(i as (isize)) as (isize)
                      ) as (*mut HistogramDistance)
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop3;
        } else {
            break 'loop3;
        }
    }
    i = 0i32 as (usize);
    'loop5: loop {
        if i < in_size {
            HistogramAddHistogramDistance(
                &mut *out.offset(
                          *symbols.offset(i as (isize)) as (isize)
                      ) as (*mut HistogramDistance),
                &*in_.offset(i as (isize)) as (*const HistogramDistance)
            );
            i = i.wrapping_add(1 as (usize));
            continue 'loop5;
        } else {
            break 'loop5;
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliHistogramReindexDistance(
    mut m : *mut MemoryManager,
    mut out : *mut HistogramDistance,
    mut symbols : *mut u32,
    mut length : usize
) -> usize {
    static kInvalidIndex : u32 = !(0i32 as (u32));
    let mut new_index
        : *mut u32
        = if length != 0 {
              BrotliAllocate(
                  m,
                  length.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut next_index : u32;
    let mut tmp : *mut HistogramDistance;
    let mut i : usize;
    if !(0i32 == 0) {
        0i32 as (usize)
    } else {
        i = 0i32 as (usize);
        'loop2: loop {
            if i < length {
                *new_index.offset(i as (isize)) = kInvalidIndex;
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        next_index = 0i32 as (u32);
        i = 0i32 as (usize);
        'loop4: loop {
            if i < length {
                if *new_index.offset(
                        *symbols.offset(i as (isize)) as (isize)
                    ) == kInvalidIndex {
                    *new_index.offset(
                         *symbols.offset(i as (isize)) as (isize)
                     ) = next_index;
                    next_index = next_index.wrapping_add(1 as (u32));
                }
                i = i.wrapping_add(1 as (usize));
                continue 'loop4;
            } else {
                break 'loop4;
            }
        }
        tmp = if next_index != 0 {
                  BrotliAllocate(
                      m,
                      (next_index as (usize)).wrapping_mul(
                          ::std::mem::size_of::<HistogramDistance>()
                      )
                  ) as (*mut HistogramDistance)
              } else {
                  0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance)
              };
        if !(0i32 == 0) {
            0i32 as (usize)
        } else {
            next_index = 0i32 as (u32);
            i = 0i32 as (usize);
            'loop7: loop {
                if i < length {
                    if *new_index.offset(
                            *symbols.offset(i as (isize)) as (isize)
                        ) == next_index {
                        *tmp.offset(next_index as (isize)) = *out.offset(
                                                                  *symbols.offset(
                                                                       i as (isize)
                                                                   ) as (isize)
                                                              );
                        next_index = next_index.wrapping_add(1 as (u32));
                    }
                    *symbols.offset(i as (isize)) = *new_index.offset(
                                                         *symbols.offset(i as (isize)) as (isize)
                                                     );
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop7;
                } else {
                    break 'loop7;
                }
            }
            BrotliFree(m,new_index as (*mut ::std::os::raw::c_void));
            new_index = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
            i = 0i32 as (usize);
            'loop9: loop {
                if i < next_index as (usize) {
                    *out.offset(i as (isize)) = *tmp.offset(i as (isize));
                    i = i.wrapping_add(1 as (usize));
                    continue 'loop9;
                } else {
                    break 'loop9;
                }
            }
            BrotliFree(m,tmp as (*mut ::std::os::raw::c_void));
            tmp = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramDistance);
            next_index as (usize)
        }
    }
}

#[no_mangle]
pub unsafe extern fn BrotliClusterHistogramsDistance(
    mut m : *mut MemoryManager,
    mut in_ : *const HistogramDistance,
    in_size : usize,
    mut max_histograms : usize,
    mut out : *mut HistogramDistance,
    mut out_size : *mut usize,
    mut histogram_symbols : *mut u32
) {
    let mut cluster_size
        : *mut u32
        = if in_size != 0 {
              BrotliAllocate(
                  m,
                  in_size.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut clusters
        : *mut u32
        = if in_size != 0 {
              BrotliAllocate(
                  m,
                  in_size.wrapping_mul(::std::mem::size_of::<u32>())
              ) as (*mut u32)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut u32)
          };
    let mut num_clusters : usize = 0i32 as (usize);
    let max_input_histograms : usize = 64i32 as (usize);
    let mut pairs_capacity
        : usize
        = max_input_histograms.wrapping_mul(
              max_input_histograms
          ).wrapping_div(
              2i32 as (usize)
          );
    let mut pairs
        : *mut HistogramPair
        = if pairs_capacity.wrapping_add(1i32 as (usize)) != 0 {
              BrotliAllocate(
                  m,
                  pairs_capacity.wrapping_add(1i32 as (usize)).wrapping_mul(
                      ::std::mem::size_of::<HistogramPair>()
                  )
              ) as (*mut HistogramPair)
          } else {
              0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
          };
    let mut i : usize;
    if !(0i32 == 0) {
    } else {
        i = 0i32 as (usize);
        'loop2: loop {
            if i < in_size {
                *cluster_size.offset(i as (isize)) = 1i32 as (u32);
                i = i.wrapping_add(1 as (usize));
                continue 'loop2;
            } else {
                break 'loop2;
            }
        }
        i = 0i32 as (usize);
        'loop4: loop {
            if i < in_size {
                *out.offset(i as (isize)) = *in_.offset(i as (isize));
                (*out.offset(
                      i as (isize)
                  )).bit_cost_ = BrotliPopulationCostDistance(
                                     &*in_.offset(i as (isize)) as (*const HistogramDistance)
                                 );
                *histogram_symbols.offset(i as (isize)) = i as (u32);
                i = i.wrapping_add(1 as (usize));
                continue 'loop4;
            } else {
                break 'loop4;
            }
        }
        i = 0i32 as (usize);
        'loop6: loop {
            if i < in_size {
                let mut num_to_combine
                    : usize
                    = brotli_min_size_t(in_size.wrapping_sub(i),max_input_histograms);
                let mut num_new_clusters : usize;
                let mut j : usize;
                j = 0i32 as (usize);
                'loop20: loop {
                    if j < num_to_combine {
                        *clusters.offset(
                             num_clusters.wrapping_add(j) as (isize)
                         ) = i.wrapping_add(j) as (u32);
                        j = j.wrapping_add(1 as (usize));
                        continue 'loop20;
                    } else {
                        break 'loop20;
                    }
                }
                num_new_clusters = BrotliHistogramCombineDistance(
                                       out,
                                       cluster_size,
                                       &mut *histogram_symbols.offset(i as (isize)) as (*mut u32),
                                       &mut *clusters.offset(num_clusters as (isize)) as (*mut u32),
                                       pairs,
                                       num_to_combine,
                                       num_to_combine,
                                       max_histograms,
                                       pairs_capacity
                                   );
                num_clusters = num_clusters.wrapping_add(num_new_clusters);
                i = i.wrapping_add(max_input_histograms);
                continue 'loop6;
            } else {
                break 'loop6;
            }
        }
        let mut max_num_pairs
            : usize
            = brotli_min_size_t(
                  (64i32 as (usize)).wrapping_mul(num_clusters),
                  num_clusters.wrapping_div(2i32 as (usize)).wrapping_mul(
                      num_clusters
                  )
              );
        if pairs_capacity < max_num_pairs.wrapping_add(1i32 as (usize)) {
            let mut _new_size
                : usize
                = if pairs_capacity == 0i32 as (usize) {
                      max_num_pairs.wrapping_add(1i32 as (usize))
                  } else {
                      pairs_capacity
                  };
            let mut new_array : *mut HistogramPair;
            'loop9: loop {
                if _new_size < max_num_pairs.wrapping_add(1i32 as (usize)) {
                    _new_size = _new_size.wrapping_mul(2i32 as (usize));
                    continue 'loop9;
                } else {
                    break 'loop9;
                }
            }
            new_array = if _new_size != 0 {
                            BrotliAllocate(
                                m,
                                _new_size.wrapping_mul(::std::mem::size_of::<HistogramPair>())
                            ) as (*mut HistogramPair)
                        } else {
                            0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair)
                        };
            if !!(0i32 == 0) && (pairs_capacity != 0i32 as (usize)) {
                memcpy(
                    new_array as (*mut ::std::os::raw::c_void),
                    pairs as (*const ::std::os::raw::c_void),
                    pairs_capacity.wrapping_mul(::std::mem::size_of::<HistogramPair>())
                );
            }
            BrotliFree(m,pairs as (*mut ::std::os::raw::c_void));
            pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
            pairs = new_array;
            pairs_capacity = _new_size;
        }
        if !(0i32 == 0) {
        } else {
            num_clusters = BrotliHistogramCombineDistance(
                               out,
                               cluster_size,
                               histogram_symbols,
                               clusters,
                               pairs,
                               num_clusters,
                               in_size,
                               max_histograms,
                               max_num_pairs
                           );
            BrotliFree(m,pairs as (*mut ::std::os::raw::c_void));
            pairs = 0i32 as (*mut ::std::os::raw::c_void) as (*mut HistogramPair);
            BrotliFree(m,cluster_size as (*mut ::std::os::raw::c_void));
            cluster_size = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
            BrotliHistogramRemapDistance(
                in_,
                in_size,
                clusters as (*const u32),
                num_clusters,
                out,
                histogram_symbols
            );
            BrotliFree(m,clusters as (*mut ::std::os::raw::c_void));
            clusters = 0i32 as (*mut ::std::os::raw::c_void) as (*mut u32);
            *out_size = BrotliHistogramReindexDistance(
                            m,
                            out,
                            histogram_symbols,
                            in_size
                        );
            if !(0i32 == 0) { }
        }
    }
}
