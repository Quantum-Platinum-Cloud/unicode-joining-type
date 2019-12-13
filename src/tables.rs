// DO NOT EDIT THIS FILE. IT WAS AUTOMATICALLY GENERATED BY:
//
//  ucd-generate joining-type --rust-enum /home/wmoore/Downloads/ucd-12.1
//
// ucd-generate is available on crates.io.

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum JoiningType {
    DualJoining,
    JoinCausing,
    LeftJoining,
    NonJoining,
    RightJoining,
    Transparent,
}

#[allow(dead_code)]
pub const JOINING_TYPE: &'static [(u32, u32, JoiningType)] = &[
    (0, 172, JoiningType::NonJoining),
    (173, 173, JoiningType::Transparent),
    (174, 767, JoiningType::NonJoining),
    (768, 879, JoiningType::Transparent),
    (880, 1154, JoiningType::NonJoining),
    (1155, 1161, JoiningType::Transparent),
    (1162, 1424, JoiningType::NonJoining),
    (1425, 1469, JoiningType::Transparent),
    (1470, 1470, JoiningType::NonJoining),
    (1471, 1471, JoiningType::Transparent),
    (1472, 1472, JoiningType::NonJoining),
    (1473, 1474, JoiningType::Transparent),
    (1475, 1475, JoiningType::NonJoining),
    (1476, 1477, JoiningType::Transparent),
    (1478, 1478, JoiningType::NonJoining),
    (1479, 1479, JoiningType::Transparent),
    (1480, 1551, JoiningType::NonJoining),
    (1552, 1562, JoiningType::Transparent),
    (1563, 1563, JoiningType::NonJoining),
    (1564, 1564, JoiningType::Transparent),
    (1565, 1567, JoiningType::NonJoining),
    (1568, 1568, JoiningType::DualJoining),
    (1569, 1569, JoiningType::NonJoining),
    (1570, 1573, JoiningType::RightJoining),
    (1574, 1574, JoiningType::DualJoining),
    (1575, 1575, JoiningType::RightJoining),
    (1576, 1576, JoiningType::DualJoining),
    (1577, 1577, JoiningType::RightJoining),
    (1578, 1582, JoiningType::DualJoining),
    (1583, 1586, JoiningType::RightJoining),
    (1587, 1599, JoiningType::DualJoining),
    (1600, 1600, JoiningType::JoinCausing),
    (1601, 1607, JoiningType::DualJoining),
    (1608, 1608, JoiningType::RightJoining),
    (1609, 1610, JoiningType::DualJoining),
    (1611, 1631, JoiningType::Transparent),
    (1632, 1645, JoiningType::NonJoining),
    (1646, 1647, JoiningType::DualJoining),
    (1648, 1648, JoiningType::Transparent),
    (1649, 1651, JoiningType::RightJoining),
    (1652, 1652, JoiningType::NonJoining),
    (1653, 1655, JoiningType::RightJoining),
    (1656, 1671, JoiningType::DualJoining),
    (1672, 1689, JoiningType::RightJoining),
    (1690, 1727, JoiningType::DualJoining),
    (1728, 1728, JoiningType::RightJoining),
    (1729, 1730, JoiningType::DualJoining),
    (1731, 1739, JoiningType::RightJoining),
    (1740, 1740, JoiningType::DualJoining),
    (1741, 1741, JoiningType::RightJoining),
    (1742, 1742, JoiningType::DualJoining),
    (1743, 1743, JoiningType::RightJoining),
    (1744, 1745, JoiningType::DualJoining),
    (1746, 1747, JoiningType::RightJoining),
    (1748, 1748, JoiningType::NonJoining),
    (1749, 1749, JoiningType::RightJoining),
    (1750, 1756, JoiningType::Transparent),
    (1757, 1758, JoiningType::NonJoining),
    (1759, 1764, JoiningType::Transparent),
    (1765, 1766, JoiningType::NonJoining),
    (1767, 1768, JoiningType::Transparent),
    (1769, 1769, JoiningType::NonJoining),
    (1770, 1773, JoiningType::Transparent),
    (1774, 1775, JoiningType::RightJoining),
    (1776, 1785, JoiningType::NonJoining),
    (1786, 1788, JoiningType::DualJoining),
    (1789, 1790, JoiningType::NonJoining),
    (1791, 1791, JoiningType::DualJoining),
    (1792, 1806, JoiningType::NonJoining),
    (1807, 1807, JoiningType::Transparent),
    (1808, 1808, JoiningType::RightJoining),
    (1809, 1809, JoiningType::Transparent),
    (1810, 1812, JoiningType::DualJoining),
    (1813, 1817, JoiningType::RightJoining),
    (1818, 1821, JoiningType::DualJoining),
    (1822, 1822, JoiningType::RightJoining),
    (1823, 1831, JoiningType::DualJoining),
    (1832, 1832, JoiningType::RightJoining),
    (1833, 1833, JoiningType::DualJoining),
    (1834, 1834, JoiningType::RightJoining),
    (1835, 1835, JoiningType::DualJoining),
    (1836, 1836, JoiningType::RightJoining),
    (1837, 1838, JoiningType::DualJoining),
    (1839, 1839, JoiningType::RightJoining),
    (1840, 1866, JoiningType::Transparent),
    (1867, 1868, JoiningType::NonJoining),
    (1869, 1869, JoiningType::RightJoining),
    (1870, 1880, JoiningType::DualJoining),
    (1881, 1883, JoiningType::RightJoining),
    (1884, 1898, JoiningType::DualJoining),
    (1899, 1900, JoiningType::RightJoining),
    (1901, 1904, JoiningType::DualJoining),
    (1905, 1905, JoiningType::RightJoining),
    (1906, 1906, JoiningType::DualJoining),
    (1907, 1908, JoiningType::RightJoining),
    (1909, 1911, JoiningType::DualJoining),
    (1912, 1913, JoiningType::RightJoining),
    (1914, 1919, JoiningType::DualJoining),
    (1920, 1957, JoiningType::NonJoining),
    (1958, 1968, JoiningType::Transparent),
    (1969, 1993, JoiningType::NonJoining),
    (1994, 2026, JoiningType::DualJoining),
    (2027, 2035, JoiningType::Transparent),
    (2036, 2041, JoiningType::NonJoining),
    (2042, 2042, JoiningType::JoinCausing),
    (2043, 2044, JoiningType::NonJoining),
    (2045, 2045, JoiningType::Transparent),
    (2046, 2069, JoiningType::NonJoining),
    (2070, 2073, JoiningType::Transparent),
    (2074, 2074, JoiningType::NonJoining),
    (2075, 2083, JoiningType::Transparent),
    (2084, 2084, JoiningType::NonJoining),
    (2085, 2087, JoiningType::Transparent),
    (2088, 2088, JoiningType::NonJoining),
    (2089, 2093, JoiningType::Transparent),
    (2094, 2111, JoiningType::NonJoining),
    (2112, 2112, JoiningType::RightJoining),
    (2113, 2117, JoiningType::DualJoining),
    (2118, 2119, JoiningType::RightJoining),
    (2120, 2120, JoiningType::DualJoining),
    (2121, 2121, JoiningType::RightJoining),
    (2122, 2131, JoiningType::DualJoining),
    (2132, 2132, JoiningType::RightJoining),
    (2133, 2133, JoiningType::DualJoining),
    (2134, 2136, JoiningType::NonJoining),
    (2137, 2139, JoiningType::Transparent),
    (2140, 2143, JoiningType::NonJoining),
    (2144, 2144, JoiningType::DualJoining),
    (2145, 2145, JoiningType::NonJoining),
    (2146, 2149, JoiningType::DualJoining),
    (2150, 2150, JoiningType::NonJoining),
    (2151, 2151, JoiningType::RightJoining),
    (2152, 2152, JoiningType::DualJoining),
    (2153, 2154, JoiningType::RightJoining),
    (2155, 2207, JoiningType::NonJoining),
    (2208, 2217, JoiningType::DualJoining),
    (2218, 2220, JoiningType::RightJoining),
    (2221, 2221, JoiningType::NonJoining),
    (2222, 2222, JoiningType::RightJoining),
    (2223, 2224, JoiningType::DualJoining),
    (2225, 2226, JoiningType::RightJoining),
    (2227, 2228, JoiningType::DualJoining),
    (2229, 2229, JoiningType::NonJoining),
    (2230, 2232, JoiningType::DualJoining),
    (2233, 2233, JoiningType::RightJoining),
    (2234, 2237, JoiningType::DualJoining),
    (2238, 2258, JoiningType::NonJoining),
    (2259, 2273, JoiningType::Transparent),
    (2274, 2274, JoiningType::NonJoining),
    (2275, 2306, JoiningType::Transparent),
    (2307, 2361, JoiningType::NonJoining),
    (2362, 2362, JoiningType::Transparent),
    (2363, 2363, JoiningType::NonJoining),
    (2364, 2364, JoiningType::Transparent),
    (2365, 2368, JoiningType::NonJoining),
    (2369, 2376, JoiningType::Transparent),
    (2377, 2380, JoiningType::NonJoining),
    (2381, 2381, JoiningType::Transparent),
    (2382, 2384, JoiningType::NonJoining),
    (2385, 2391, JoiningType::Transparent),
    (2392, 2401, JoiningType::NonJoining),
    (2402, 2403, JoiningType::Transparent),
    (2404, 2432, JoiningType::NonJoining),
    (2433, 2433, JoiningType::Transparent),
    (2434, 2491, JoiningType::NonJoining),
    (2492, 2492, JoiningType::Transparent),
    (2493, 2496, JoiningType::NonJoining),
    (2497, 2500, JoiningType::Transparent),
    (2501, 2508, JoiningType::NonJoining),
    (2509, 2509, JoiningType::Transparent),
    (2510, 2529, JoiningType::NonJoining),
    (2530, 2531, JoiningType::Transparent),
    (2532, 2557, JoiningType::NonJoining),
    (2558, 2558, JoiningType::Transparent),
    (2559, 2560, JoiningType::NonJoining),
    (2561, 2562, JoiningType::Transparent),
    (2563, 2619, JoiningType::NonJoining),
    (2620, 2620, JoiningType::Transparent),
    (2621, 2624, JoiningType::NonJoining),
    (2625, 2626, JoiningType::Transparent),
    (2627, 2630, JoiningType::NonJoining),
    (2631, 2632, JoiningType::Transparent),
    (2633, 2634, JoiningType::NonJoining),
    (2635, 2637, JoiningType::Transparent),
    (2638, 2640, JoiningType::NonJoining),
    (2641, 2641, JoiningType::Transparent),
    (2642, 2671, JoiningType::NonJoining),
    (2672, 2673, JoiningType::Transparent),
    (2674, 2676, JoiningType::NonJoining),
    (2677, 2677, JoiningType::Transparent),
    (2678, 2688, JoiningType::NonJoining),
    (2689, 2690, JoiningType::Transparent),
    (2691, 2747, JoiningType::NonJoining),
    (2748, 2748, JoiningType::Transparent),
    (2749, 2752, JoiningType::NonJoining),
    (2753, 2757, JoiningType::Transparent),
    (2758, 2758, JoiningType::NonJoining),
    (2759, 2760, JoiningType::Transparent),
    (2761, 2764, JoiningType::NonJoining),
    (2765, 2765, JoiningType::Transparent),
    (2766, 2785, JoiningType::NonJoining),
    (2786, 2787, JoiningType::Transparent),
    (2788, 2809, JoiningType::NonJoining),
    (2810, 2815, JoiningType::Transparent),
    (2816, 2816, JoiningType::NonJoining),
    (2817, 2817, JoiningType::Transparent),
    (2818, 2875, JoiningType::NonJoining),
    (2876, 2876, JoiningType::Transparent),
    (2877, 2878, JoiningType::NonJoining),
    (2879, 2879, JoiningType::Transparent),
    (2880, 2880, JoiningType::NonJoining),
    (2881, 2884, JoiningType::Transparent),
    (2885, 2892, JoiningType::NonJoining),
    (2893, 2893, JoiningType::Transparent),
    (2894, 2901, JoiningType::NonJoining),
    (2902, 2902, JoiningType::Transparent),
    (2903, 2913, JoiningType::NonJoining),
    (2914, 2915, JoiningType::Transparent),
    (2916, 2945, JoiningType::NonJoining),
    (2946, 2946, JoiningType::Transparent),
    (2947, 3007, JoiningType::NonJoining),
    (3008, 3008, JoiningType::Transparent),
    (3009, 3020, JoiningType::NonJoining),
    (3021, 3021, JoiningType::Transparent),
    (3022, 3071, JoiningType::NonJoining),
    (3072, 3072, JoiningType::Transparent),
    (3073, 3075, JoiningType::NonJoining),
    (3076, 3076, JoiningType::Transparent),
    (3077, 3133, JoiningType::NonJoining),
    (3134, 3136, JoiningType::Transparent),
    (3137, 3141, JoiningType::NonJoining),
    (3142, 3144, JoiningType::Transparent),
    (3145, 3145, JoiningType::NonJoining),
    (3146, 3149, JoiningType::Transparent),
    (3150, 3156, JoiningType::NonJoining),
    (3157, 3158, JoiningType::Transparent),
    (3159, 3169, JoiningType::NonJoining),
    (3170, 3171, JoiningType::Transparent),
    (3172, 3200, JoiningType::NonJoining),
    (3201, 3201, JoiningType::Transparent),
    (3202, 3259, JoiningType::NonJoining),
    (3260, 3260, JoiningType::Transparent),
    (3261, 3262, JoiningType::NonJoining),
    (3263, 3263, JoiningType::Transparent),
    (3264, 3269, JoiningType::NonJoining),
    (3270, 3270, JoiningType::Transparent),
    (3271, 3275, JoiningType::NonJoining),
    (3276, 3277, JoiningType::Transparent),
    (3278, 3297, JoiningType::NonJoining),
    (3298, 3299, JoiningType::Transparent),
    (3300, 3327, JoiningType::NonJoining),
    (3328, 3329, JoiningType::Transparent),
    (3330, 3386, JoiningType::NonJoining),
    (3387, 3388, JoiningType::Transparent),
    (3389, 3392, JoiningType::NonJoining),
    (3393, 3396, JoiningType::Transparent),
    (3397, 3404, JoiningType::NonJoining),
    (3405, 3405, JoiningType::Transparent),
    (3406, 3425, JoiningType::NonJoining),
    (3426, 3427, JoiningType::Transparent),
    (3428, 3529, JoiningType::NonJoining),
    (3530, 3530, JoiningType::Transparent),
    (3531, 3537, JoiningType::NonJoining),
    (3538, 3540, JoiningType::Transparent),
    (3541, 3541, JoiningType::NonJoining),
    (3542, 3542, JoiningType::Transparent),
    (3543, 3632, JoiningType::NonJoining),
    (3633, 3633, JoiningType::Transparent),
    (3634, 3635, JoiningType::NonJoining),
    (3636, 3642, JoiningType::Transparent),
    (3643, 3654, JoiningType::NonJoining),
    (3655, 3662, JoiningType::Transparent),
    (3663, 3760, JoiningType::NonJoining),
    (3761, 3761, JoiningType::Transparent),
    (3762, 3763, JoiningType::NonJoining),
    (3764, 3772, JoiningType::Transparent),
    (3773, 3783, JoiningType::NonJoining),
    (3784, 3789, JoiningType::Transparent),
    (3790, 3863, JoiningType::NonJoining),
    (3864, 3865, JoiningType::Transparent),
    (3866, 3892, JoiningType::NonJoining),
    (3893, 3893, JoiningType::Transparent),
    (3894, 3894, JoiningType::NonJoining),
    (3895, 3895, JoiningType::Transparent),
    (3896, 3896, JoiningType::NonJoining),
    (3897, 3897, JoiningType::Transparent),
    (3898, 3952, JoiningType::NonJoining),
    (3953, 3966, JoiningType::Transparent),
    (3967, 3967, JoiningType::NonJoining),
    (3968, 3972, JoiningType::Transparent),
    (3973, 3973, JoiningType::NonJoining),
    (3974, 3975, JoiningType::Transparent),
    (3976, 3980, JoiningType::NonJoining),
    (3981, 3991, JoiningType::Transparent),
    (3992, 3992, JoiningType::NonJoining),
    (3993, 4028, JoiningType::Transparent),
    (4029, 4037, JoiningType::NonJoining),
    (4038, 4038, JoiningType::Transparent),
    (4039, 4140, JoiningType::NonJoining),
    (4141, 4144, JoiningType::Transparent),
    (4145, 4145, JoiningType::NonJoining),
    (4146, 4151, JoiningType::Transparent),
    (4152, 4152, JoiningType::NonJoining),
    (4153, 4154, JoiningType::Transparent),
    (4155, 4156, JoiningType::NonJoining),
    (4157, 4158, JoiningType::Transparent),
    (4159, 4183, JoiningType::NonJoining),
    (4184, 4185, JoiningType::Transparent),
    (4186, 4189, JoiningType::NonJoining),
    (4190, 4192, JoiningType::Transparent),
    (4193, 4208, JoiningType::NonJoining),
    (4209, 4212, JoiningType::Transparent),
    (4213, 4225, JoiningType::NonJoining),
    (4226, 4226, JoiningType::Transparent),
    (4227, 4228, JoiningType::NonJoining),
    (4229, 4230, JoiningType::Transparent),
    (4231, 4236, JoiningType::NonJoining),
    (4237, 4237, JoiningType::Transparent),
    (4238, 4252, JoiningType::NonJoining),
    (4253, 4253, JoiningType::Transparent),
    (4254, 4956, JoiningType::NonJoining),
    (4957, 4959, JoiningType::Transparent),
    (4960, 5905, JoiningType::NonJoining),
    (5906, 5908, JoiningType::Transparent),
    (5909, 5937, JoiningType::NonJoining),
    (5938, 5940, JoiningType::Transparent),
    (5941, 5969, JoiningType::NonJoining),
    (5970, 5971, JoiningType::Transparent),
    (5972, 6001, JoiningType::NonJoining),
    (6002, 6003, JoiningType::Transparent),
    (6004, 6067, JoiningType::NonJoining),
    (6068, 6069, JoiningType::Transparent),
    (6070, 6070, JoiningType::NonJoining),
    (6071, 6077, JoiningType::Transparent),
    (6078, 6085, JoiningType::NonJoining),
    (6086, 6086, JoiningType::Transparent),
    (6087, 6088, JoiningType::NonJoining),
    (6089, 6099, JoiningType::Transparent),
    (6100, 6108, JoiningType::NonJoining),
    (6109, 6109, JoiningType::Transparent),
    (6110, 6150, JoiningType::NonJoining),
    (6151, 6151, JoiningType::DualJoining),
    (6152, 6153, JoiningType::NonJoining),
    (6154, 6154, JoiningType::JoinCausing),
    (6155, 6157, JoiningType::Transparent),
    (6158, 6175, JoiningType::NonJoining),
    (6176, 6264, JoiningType::DualJoining),
    (6265, 6276, JoiningType::NonJoining),
    (6277, 6278, JoiningType::Transparent),
    (6279, 6312, JoiningType::DualJoining),
    (6313, 6313, JoiningType::Transparent),
    (6314, 6314, JoiningType::DualJoining),
    (6315, 6431, JoiningType::NonJoining),
    (6432, 6434, JoiningType::Transparent),
    (6435, 6438, JoiningType::NonJoining),
    (6439, 6440, JoiningType::Transparent),
    (6441, 6449, JoiningType::NonJoining),
    (6450, 6450, JoiningType::Transparent),
    (6451, 6456, JoiningType::NonJoining),
    (6457, 6459, JoiningType::Transparent),
    (6460, 6678, JoiningType::NonJoining),
    (6679, 6680, JoiningType::Transparent),
    (6681, 6682, JoiningType::NonJoining),
    (6683, 6683, JoiningType::Transparent),
    (6684, 6741, JoiningType::NonJoining),
    (6742, 6742, JoiningType::Transparent),
    (6743, 6743, JoiningType::NonJoining),
    (6744, 6750, JoiningType::Transparent),
    (6751, 6751, JoiningType::NonJoining),
    (6752, 6752, JoiningType::Transparent),
    (6753, 6753, JoiningType::NonJoining),
    (6754, 6754, JoiningType::Transparent),
    (6755, 6756, JoiningType::NonJoining),
    (6757, 6764, JoiningType::Transparent),
    (6765, 6770, JoiningType::NonJoining),
    (6771, 6780, JoiningType::Transparent),
    (6781, 6782, JoiningType::NonJoining),
    (6783, 6783, JoiningType::Transparent),
    (6784, 6831, JoiningType::NonJoining),
    (6832, 6846, JoiningType::Transparent),
    (6847, 6911, JoiningType::NonJoining),
    (6912, 6915, JoiningType::Transparent),
    (6916, 6963, JoiningType::NonJoining),
    (6964, 6964, JoiningType::Transparent),
    (6965, 6965, JoiningType::NonJoining),
    (6966, 6970, JoiningType::Transparent),
    (6971, 6971, JoiningType::NonJoining),
    (6972, 6972, JoiningType::Transparent),
    (6973, 6977, JoiningType::NonJoining),
    (6978, 6978, JoiningType::Transparent),
    (6979, 7018, JoiningType::NonJoining),
    (7019, 7027, JoiningType::Transparent),
    (7028, 7039, JoiningType::NonJoining),
    (7040, 7041, JoiningType::Transparent),
    (7042, 7073, JoiningType::NonJoining),
    (7074, 7077, JoiningType::Transparent),
    (7078, 7079, JoiningType::NonJoining),
    (7080, 7081, JoiningType::Transparent),
    (7082, 7082, JoiningType::NonJoining),
    (7083, 7085, JoiningType::Transparent),
    (7086, 7141, JoiningType::NonJoining),
    (7142, 7142, JoiningType::Transparent),
    (7143, 7143, JoiningType::NonJoining),
    (7144, 7145, JoiningType::Transparent),
    (7146, 7148, JoiningType::NonJoining),
    (7149, 7149, JoiningType::Transparent),
    (7150, 7150, JoiningType::NonJoining),
    (7151, 7153, JoiningType::Transparent),
    (7154, 7211, JoiningType::NonJoining),
    (7212, 7219, JoiningType::Transparent),
    (7220, 7221, JoiningType::NonJoining),
    (7222, 7223, JoiningType::Transparent),
    (7224, 7375, JoiningType::NonJoining),
    (7376, 7378, JoiningType::Transparent),
    (7379, 7379, JoiningType::NonJoining),
    (7380, 7392, JoiningType::Transparent),
    (7393, 7393, JoiningType::NonJoining),
    (7394, 7400, JoiningType::Transparent),
    (7401, 7404, JoiningType::NonJoining),
    (7405, 7405, JoiningType::Transparent),
    (7406, 7411, JoiningType::NonJoining),
    (7412, 7412, JoiningType::Transparent),
    (7413, 7415, JoiningType::NonJoining),
    (7416, 7417, JoiningType::Transparent),
    (7418, 7615, JoiningType::NonJoining),
    (7616, 7673, JoiningType::Transparent),
    (7674, 7674, JoiningType::NonJoining),
    (7675, 7679, JoiningType::Transparent),
    (7680, 8202, JoiningType::NonJoining),
    (8203, 8203, JoiningType::Transparent),
    (8204, 8204, JoiningType::NonJoining),
    (8205, 8205, JoiningType::JoinCausing),
    (8206, 8207, JoiningType::Transparent),
    (8208, 8233, JoiningType::NonJoining),
    (8234, 8238, JoiningType::Transparent),
    (8239, 8287, JoiningType::NonJoining),
    (8288, 8292, JoiningType::Transparent),
    (8293, 8297, JoiningType::NonJoining),
    (8298, 8303, JoiningType::Transparent),
    (8304, 8399, JoiningType::NonJoining),
    (8400, 8432, JoiningType::Transparent),
    (8433, 11502, JoiningType::NonJoining),
    (11503, 11505, JoiningType::Transparent),
    (11506, 11646, JoiningType::NonJoining),
    (11647, 11647, JoiningType::Transparent),
    (11648, 11743, JoiningType::NonJoining),
    (11744, 11775, JoiningType::Transparent),
    (11776, 12329, JoiningType::NonJoining),
    (12330, 12333, JoiningType::Transparent),
    (12334, 12440, JoiningType::NonJoining),
    (12441, 12442, JoiningType::Transparent),
    (12443, 42606, JoiningType::NonJoining),
    (42607, 42610, JoiningType::Transparent),
    (42611, 42611, JoiningType::NonJoining),
    (42612, 42621, JoiningType::Transparent),
    (42622, 42653, JoiningType::NonJoining),
    (42654, 42655, JoiningType::Transparent),
    (42656, 42735, JoiningType::NonJoining),
    (42736, 42737, JoiningType::Transparent),
    (42738, 43009, JoiningType::NonJoining),
    (43010, 43010, JoiningType::Transparent),
    (43011, 43013, JoiningType::NonJoining),
    (43014, 43014, JoiningType::Transparent),
    (43015, 43018, JoiningType::NonJoining),
    (43019, 43019, JoiningType::Transparent),
    (43020, 43044, JoiningType::NonJoining),
    (43045, 43046, JoiningType::Transparent),
    (43047, 43071, JoiningType::NonJoining),
    (43072, 43121, JoiningType::DualJoining),
    (43122, 43122, JoiningType::LeftJoining),
    (43123, 43203, JoiningType::NonJoining),
    (43204, 43205, JoiningType::Transparent),
    (43206, 43231, JoiningType::NonJoining),
    (43232, 43249, JoiningType::Transparent),
    (43250, 43262, JoiningType::NonJoining),
    (43263, 43263, JoiningType::Transparent),
    (43264, 43301, JoiningType::NonJoining),
    (43302, 43309, JoiningType::Transparent),
    (43310, 43334, JoiningType::NonJoining),
    (43335, 43345, JoiningType::Transparent),
    (43346, 43391, JoiningType::NonJoining),
    (43392, 43394, JoiningType::Transparent),
    (43395, 43442, JoiningType::NonJoining),
    (43443, 43443, JoiningType::Transparent),
    (43444, 43445, JoiningType::NonJoining),
    (43446, 43449, JoiningType::Transparent),
    (43450, 43451, JoiningType::NonJoining),
    (43452, 43453, JoiningType::Transparent),
    (43454, 43492, JoiningType::NonJoining),
    (43493, 43493, JoiningType::Transparent),
    (43494, 43560, JoiningType::NonJoining),
    (43561, 43566, JoiningType::Transparent),
    (43567, 43568, JoiningType::NonJoining),
    (43569, 43570, JoiningType::Transparent),
    (43571, 43572, JoiningType::NonJoining),
    (43573, 43574, JoiningType::Transparent),
    (43575, 43586, JoiningType::NonJoining),
    (43587, 43587, JoiningType::Transparent),
    (43588, 43595, JoiningType::NonJoining),
    (43596, 43596, JoiningType::Transparent),
    (43597, 43643, JoiningType::NonJoining),
    (43644, 43644, JoiningType::Transparent),
    (43645, 43695, JoiningType::NonJoining),
    (43696, 43696, JoiningType::Transparent),
    (43697, 43697, JoiningType::NonJoining),
    (43698, 43700, JoiningType::Transparent),
    (43701, 43702, JoiningType::NonJoining),
    (43703, 43704, JoiningType::Transparent),
    (43705, 43709, JoiningType::NonJoining),
    (43710, 43711, JoiningType::Transparent),
    (43712, 43712, JoiningType::NonJoining),
    (43713, 43713, JoiningType::Transparent),
    (43714, 43755, JoiningType::NonJoining),
    (43756, 43757, JoiningType::Transparent),
    (43758, 43765, JoiningType::NonJoining),
    (43766, 43766, JoiningType::Transparent),
    (43767, 44004, JoiningType::NonJoining),
    (44005, 44005, JoiningType::Transparent),
    (44006, 44007, JoiningType::NonJoining),
    (44008, 44008, JoiningType::Transparent),
    (44009, 44012, JoiningType::NonJoining),
    (44013, 44013, JoiningType::Transparent),
    (44014, 64285, JoiningType::NonJoining),
    (64286, 64286, JoiningType::Transparent),
    (64287, 65023, JoiningType::NonJoining),
    (65024, 65039, JoiningType::Transparent),
    (65040, 65055, JoiningType::NonJoining),
    (65056, 65071, JoiningType::Transparent),
    (65072, 65278, JoiningType::NonJoining),
    (65279, 65279, JoiningType::Transparent),
    (65280, 65528, JoiningType::NonJoining),
    (65529, 65531, JoiningType::Transparent),
    (65532, 66044, JoiningType::NonJoining),
    (66045, 66045, JoiningType::Transparent),
    (66046, 66271, JoiningType::NonJoining),
    (66272, 66272, JoiningType::Transparent),
    (66273, 66421, JoiningType::NonJoining),
    (66422, 66426, JoiningType::Transparent),
    (66427, 68096, JoiningType::NonJoining),
    (68097, 68099, JoiningType::Transparent),
    (68100, 68100, JoiningType::NonJoining),
    (68101, 68102, JoiningType::Transparent),
    (68103, 68107, JoiningType::NonJoining),
    (68108, 68111, JoiningType::Transparent),
    (68112, 68151, JoiningType::NonJoining),
    (68152, 68154, JoiningType::Transparent),
    (68155, 68158, JoiningType::NonJoining),
    (68159, 68159, JoiningType::Transparent),
    (68160, 68287, JoiningType::NonJoining),
    (68288, 68292, JoiningType::DualJoining),
    (68293, 68293, JoiningType::RightJoining),
    (68294, 68294, JoiningType::NonJoining),
    (68295, 68295, JoiningType::RightJoining),
    (68296, 68296, JoiningType::NonJoining),
    (68297, 68298, JoiningType::RightJoining),
    (68299, 68300, JoiningType::NonJoining),
    (68301, 68301, JoiningType::LeftJoining),
    (68302, 68306, JoiningType::RightJoining),
    (68307, 68310, JoiningType::DualJoining),
    (68311, 68311, JoiningType::LeftJoining),
    (68312, 68316, JoiningType::DualJoining),
    (68317, 68317, JoiningType::RightJoining),
    (68318, 68320, JoiningType::DualJoining),
    (68321, 68321, JoiningType::RightJoining),
    (68322, 68323, JoiningType::NonJoining),
    (68324, 68324, JoiningType::RightJoining),
    (68325, 68326, JoiningType::Transparent),
    (68327, 68330, JoiningType::NonJoining),
    (68331, 68334, JoiningType::DualJoining),
    (68335, 68335, JoiningType::RightJoining),
    (68336, 68479, JoiningType::NonJoining),
    (68480, 68480, JoiningType::DualJoining),
    (68481, 68481, JoiningType::RightJoining),
    (68482, 68482, JoiningType::DualJoining),
    (68483, 68485, JoiningType::RightJoining),
    (68486, 68488, JoiningType::DualJoining),
    (68489, 68489, JoiningType::RightJoining),
    (68490, 68491, JoiningType::DualJoining),
    (68492, 68492, JoiningType::RightJoining),
    (68493, 68493, JoiningType::DualJoining),
    (68494, 68495, JoiningType::RightJoining),
    (68496, 68496, JoiningType::DualJoining),
    (68497, 68497, JoiningType::RightJoining),
    (68498, 68520, JoiningType::NonJoining),
    (68521, 68524, JoiningType::RightJoining),
    (68525, 68526, JoiningType::DualJoining),
    (68527, 68863, JoiningType::NonJoining),
    (68864, 68864, JoiningType::LeftJoining),
    (68865, 68897, JoiningType::DualJoining),
    (68898, 68898, JoiningType::RightJoining),
    (68899, 68899, JoiningType::DualJoining),
    (68900, 68903, JoiningType::Transparent),
    (68904, 69423, JoiningType::NonJoining),
    (69424, 69426, JoiningType::DualJoining),
    (69427, 69427, JoiningType::RightJoining),
    (69428, 69444, JoiningType::DualJoining),
    (69445, 69445, JoiningType::NonJoining),
    (69446, 69456, JoiningType::Transparent),
    (69457, 69459, JoiningType::DualJoining),
    (69460, 69460, JoiningType::RightJoining),
    (69461, 69632, JoiningType::NonJoining),
    (69633, 69633, JoiningType::Transparent),
    (69634, 69687, JoiningType::NonJoining),
    (69688, 69702, JoiningType::Transparent),
    (69703, 69758, JoiningType::NonJoining),
    (69759, 69761, JoiningType::Transparent),
    (69762, 69810, JoiningType::NonJoining),
    (69811, 69814, JoiningType::Transparent),
    (69815, 69816, JoiningType::NonJoining),
    (69817, 69818, JoiningType::Transparent),
    (69819, 69887, JoiningType::NonJoining),
    (69888, 69890, JoiningType::Transparent),
    (69891, 69926, JoiningType::NonJoining),
    (69927, 69931, JoiningType::Transparent),
    (69932, 69932, JoiningType::NonJoining),
    (69933, 69940, JoiningType::Transparent),
    (69941, 70002, JoiningType::NonJoining),
    (70003, 70003, JoiningType::Transparent),
    (70004, 70015, JoiningType::NonJoining),
    (70016, 70017, JoiningType::Transparent),
    (70018, 70069, JoiningType::NonJoining),
    (70070, 70078, JoiningType::Transparent),
    (70079, 70088, JoiningType::NonJoining),
    (70089, 70092, JoiningType::Transparent),
    (70093, 70190, JoiningType::NonJoining),
    (70191, 70193, JoiningType::Transparent),
    (70194, 70195, JoiningType::NonJoining),
    (70196, 70196, JoiningType::Transparent),
    (70197, 70197, JoiningType::NonJoining),
    (70198, 70199, JoiningType::Transparent),
    (70200, 70205, JoiningType::NonJoining),
    (70206, 70206, JoiningType::Transparent),
    (70207, 70366, JoiningType::NonJoining),
    (70367, 70367, JoiningType::Transparent),
    (70368, 70370, JoiningType::NonJoining),
    (70371, 70378, JoiningType::Transparent),
    (70379, 70399, JoiningType::NonJoining),
    (70400, 70401, JoiningType::Transparent),
    (70402, 70458, JoiningType::NonJoining),
    (70459, 70460, JoiningType::Transparent),
    (70461, 70463, JoiningType::NonJoining),
    (70464, 70464, JoiningType::Transparent),
    (70465, 70501, JoiningType::NonJoining),
    (70502, 70508, JoiningType::Transparent),
    (70509, 70511, JoiningType::NonJoining),
    (70512, 70516, JoiningType::Transparent),
    (70517, 70711, JoiningType::NonJoining),
    (70712, 70719, JoiningType::Transparent),
    (70720, 70721, JoiningType::NonJoining),
    (70722, 70724, JoiningType::Transparent),
    (70725, 70725, JoiningType::NonJoining),
    (70726, 70726, JoiningType::Transparent),
    (70727, 70749, JoiningType::NonJoining),
    (70750, 70750, JoiningType::Transparent),
    (70751, 70834, JoiningType::NonJoining),
    (70835, 70840, JoiningType::Transparent),
    (70841, 70841, JoiningType::NonJoining),
    (70842, 70842, JoiningType::Transparent),
    (70843, 70846, JoiningType::NonJoining),
    (70847, 70848, JoiningType::Transparent),
    (70849, 70849, JoiningType::NonJoining),
    (70850, 70851, JoiningType::Transparent),
    (70852, 71089, JoiningType::NonJoining),
    (71090, 71093, JoiningType::Transparent),
    (71094, 71099, JoiningType::NonJoining),
    (71100, 71101, JoiningType::Transparent),
    (71102, 71102, JoiningType::NonJoining),
    (71103, 71104, JoiningType::Transparent),
    (71105, 71131, JoiningType::NonJoining),
    (71132, 71133, JoiningType::Transparent),
    (71134, 71218, JoiningType::NonJoining),
    (71219, 71226, JoiningType::Transparent),
    (71227, 71228, JoiningType::NonJoining),
    (71229, 71229, JoiningType::Transparent),
    (71230, 71230, JoiningType::NonJoining),
    (71231, 71232, JoiningType::Transparent),
    (71233, 71338, JoiningType::NonJoining),
    (71339, 71339, JoiningType::Transparent),
    (71340, 71340, JoiningType::NonJoining),
    (71341, 71341, JoiningType::Transparent),
    (71342, 71343, JoiningType::NonJoining),
    (71344, 71349, JoiningType::Transparent),
    (71350, 71350, JoiningType::NonJoining),
    (71351, 71351, JoiningType::Transparent),
    (71352, 71452, JoiningType::NonJoining),
    (71453, 71455, JoiningType::Transparent),
    (71456, 71457, JoiningType::NonJoining),
    (71458, 71461, JoiningType::Transparent),
    (71462, 71462, JoiningType::NonJoining),
    (71463, 71467, JoiningType::Transparent),
    (71468, 71726, JoiningType::NonJoining),
    (71727, 71735, JoiningType::Transparent),
    (71736, 71736, JoiningType::NonJoining),
    (71737, 71738, JoiningType::Transparent),
    (71739, 72147, JoiningType::NonJoining),
    (72148, 72151, JoiningType::Transparent),
    (72152, 72153, JoiningType::NonJoining),
    (72154, 72155, JoiningType::Transparent),
    (72156, 72159, JoiningType::NonJoining),
    (72160, 72160, JoiningType::Transparent),
    (72161, 72192, JoiningType::NonJoining),
    (72193, 72202, JoiningType::Transparent),
    (72203, 72242, JoiningType::NonJoining),
    (72243, 72248, JoiningType::Transparent),
    (72249, 72250, JoiningType::NonJoining),
    (72251, 72254, JoiningType::Transparent),
    (72255, 72262, JoiningType::NonJoining),
    (72263, 72263, JoiningType::Transparent),
    (72264, 72272, JoiningType::NonJoining),
    (72273, 72278, JoiningType::Transparent),
    (72279, 72280, JoiningType::NonJoining),
    (72281, 72283, JoiningType::Transparent),
    (72284, 72329, JoiningType::NonJoining),
    (72330, 72342, JoiningType::Transparent),
    (72343, 72343, JoiningType::NonJoining),
    (72344, 72345, JoiningType::Transparent),
    (72346, 72751, JoiningType::NonJoining),
    (72752, 72758, JoiningType::Transparent),
    (72759, 72759, JoiningType::NonJoining),
    (72760, 72765, JoiningType::Transparent),
    (72766, 72766, JoiningType::NonJoining),
    (72767, 72767, JoiningType::Transparent),
    (72768, 72849, JoiningType::NonJoining),
    (72850, 72871, JoiningType::Transparent),
    (72872, 72873, JoiningType::NonJoining),
    (72874, 72880, JoiningType::Transparent),
    (72881, 72881, JoiningType::NonJoining),
    (72882, 72883, JoiningType::Transparent),
    (72884, 72884, JoiningType::NonJoining),
    (72885, 72886, JoiningType::Transparent),
    (72887, 73008, JoiningType::NonJoining),
    (73009, 73014, JoiningType::Transparent),
    (73015, 73017, JoiningType::NonJoining),
    (73018, 73018, JoiningType::Transparent),
    (73019, 73019, JoiningType::NonJoining),
    (73020, 73021, JoiningType::Transparent),
    (73022, 73022, JoiningType::NonJoining),
    (73023, 73029, JoiningType::Transparent),
    (73030, 73030, JoiningType::NonJoining),
    (73031, 73031, JoiningType::Transparent),
    (73032, 73103, JoiningType::NonJoining),
    (73104, 73105, JoiningType::Transparent),
    (73106, 73108, JoiningType::NonJoining),
    (73109, 73109, JoiningType::Transparent),
    (73110, 73110, JoiningType::NonJoining),
    (73111, 73111, JoiningType::Transparent),
    (73112, 73458, JoiningType::NonJoining),
    (73459, 73460, JoiningType::Transparent),
    (73461, 78895, JoiningType::NonJoining),
    (78896, 78904, JoiningType::Transparent),
    (78905, 92911, JoiningType::NonJoining),
    (92912, 92916, JoiningType::Transparent),
    (92917, 92975, JoiningType::NonJoining),
    (92976, 92982, JoiningType::Transparent),
    (92983, 94030, JoiningType::NonJoining),
    (94031, 94031, JoiningType::Transparent),
    (94032, 94094, JoiningType::NonJoining),
    (94095, 94098, JoiningType::Transparent),
    (94099, 113820, JoiningType::NonJoining),
    (113821, 113822, JoiningType::Transparent),
    (113823, 113823, JoiningType::NonJoining),
    (113824, 113827, JoiningType::Transparent),
    (113828, 119142, JoiningType::NonJoining),
    (119143, 119145, JoiningType::Transparent),
    (119146, 119154, JoiningType::NonJoining),
    (119155, 119170, JoiningType::Transparent),
    (119171, 119172, JoiningType::NonJoining),
    (119173, 119179, JoiningType::Transparent),
    (119180, 119209, JoiningType::NonJoining),
    (119210, 119213, JoiningType::Transparent),
    (119214, 119361, JoiningType::NonJoining),
    (119362, 119364, JoiningType::Transparent),
    (119365, 121343, JoiningType::NonJoining),
    (121344, 121398, JoiningType::Transparent),
    (121399, 121402, JoiningType::NonJoining),
    (121403, 121452, JoiningType::Transparent),
    (121453, 121460, JoiningType::NonJoining),
    (121461, 121461, JoiningType::Transparent),
    (121462, 121475, JoiningType::NonJoining),
    (121476, 121476, JoiningType::Transparent),
    (121477, 121498, JoiningType::NonJoining),
    (121499, 121503, JoiningType::Transparent),
    (121504, 121504, JoiningType::NonJoining),
    (121505, 121519, JoiningType::Transparent),
    (121520, 122879, JoiningType::NonJoining),
    (122880, 122886, JoiningType::Transparent),
    (122887, 122887, JoiningType::NonJoining),
    (122888, 122904, JoiningType::Transparent),
    (122905, 122906, JoiningType::NonJoining),
    (122907, 122913, JoiningType::Transparent),
    (122914, 122914, JoiningType::NonJoining),
    (122915, 122916, JoiningType::Transparent),
    (122917, 122917, JoiningType::NonJoining),
    (122918, 122922, JoiningType::Transparent),
    (122923, 123183, JoiningType::NonJoining),
    (123184, 123190, JoiningType::Transparent),
    (123191, 123627, JoiningType::NonJoining),
    (123628, 123631, JoiningType::Transparent),
    (123632, 125135, JoiningType::NonJoining),
    (125136, 125142, JoiningType::Transparent),
    (125143, 125183, JoiningType::NonJoining),
    (125184, 125251, JoiningType::DualJoining),
    (125252, 125259, JoiningType::Transparent),
    (125260, 917504, JoiningType::NonJoining),
    (917505, 917505, JoiningType::Transparent),
    (917506, 917535, JoiningType::NonJoining),
    (917536, 917631, JoiningType::Transparent),
    (917632, 917759, JoiningType::NonJoining),
    (917760, 917999, JoiningType::Transparent),
    (918000, 1114111, JoiningType::NonJoining),
];
