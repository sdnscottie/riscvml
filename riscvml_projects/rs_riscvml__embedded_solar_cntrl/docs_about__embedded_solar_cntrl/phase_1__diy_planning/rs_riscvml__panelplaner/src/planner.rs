// rs_riscvml__panelplaner — Core computation engine
//
// All math uses integer arithmetic (millivolts, milliamps, millimetres)
// so it runs on ESP32-P4 without FPU/float overhead.
//
// RISCVML — riscvml.org

#![allow(dead_code)]

/// Solar panel specifications (from datasheet)
pub struct PanelSpec<'a> {
    pub name: &'a str,
    pub width_mm: u32,           // landscape width
    pub height_mm: u32,          // landscape height
    pub depth_mm: u32,
    pub weight_g: u32,           // grams
    pub watt_peak: u32,          // Wp
    pub vmp_mv: u32,             // Vmp in millivolts
    pub imp_ma: u32,             // Imp in milliamps
    pub voc_mv: u32,             // Voc in millivolts
    pub isc_ma: u32,             // Isc in milliamps
    pub efficiency_pct_x10: u32, // efficiency × 10 (e.g. 227 = 22.7%)
}

/// Roof dimensions and orientation
pub struct RoofSpec<'a> {
    pub name: &'a str,
    pub width_mm: u32,
    pub depth_mm: u32,
    pub base_tilt_deg: u32,      // fixed roof incline
    pub max_tilt_deg: u32,       // actuator max tilt
    pub latitude_deg_x10: u32,   // latitude × 10 (e.g. 501 = 50.1°)
    pub azimuth_deg: u32,        // 180 = due south
    pub gap_mm: u32,             // gap between panels
}

/// String (series) configuration
pub struct StringConfig {
    pub panels_per_string: u32,
    pub num_strings: u32,
}

/// Planner output — all results in integer units
pub struct PlannerResult {
    // Layout
    pub panels_across: u32,
    pub rows_deep: u32,
    pub max_panels_fit: u32,
    pub total_panels: u32,
    pub array_width_mm: u32,
    pub array_depth_mm: u32,
    pub margin_left_mm: u32,
    pub margin_top_mm: u32,

    // Electrical (per string)
    pub string_vmp_mv: u32,
    pub string_voc_mv: u32,
    pub string_imp_ma: u32,
    pub string_isc_ma: u32,
    pub string_watt_peak: u32,

    // Electrical (total array)
    pub total_watt_peak: u32,

    // Weight
    pub total_weight_g: u32,
    pub weight_per_sqm_g: u32,   // g per m²

    // Energy yield
    pub peak_sun_hours_x10: u32, // × 10
    pub daily_kwh_x10: u32,      // × 10
    pub annual_kwh: u32,
    pub specific_yield: u32,     // kWh/kWp/year

    // Shading
    pub min_row_spacing_mm: u32,
    pub shading_ok: bool,
}

/// Compute the full solar plan from inputs.
/// All math is integer — safe for no_std / ESP32-P4.
pub fn compute(panel: &PanelSpec, roof: &RoofSpec, strings: &StringConfig) -> PlannerResult {
    // ── Layout ────────────────────────────────────────────────────
    let panel_pitch_w = panel.width_mm + roof.gap_mm;
    let panel_pitch_h = panel.height_mm + roof.gap_mm;

    let panels_across = roof.width_mm / panel_pitch_w;
    let rows_deep = roof.depth_mm / panel_pitch_h;
    let max_panels_fit = panels_across * rows_deep;

    let total_panels = strings.num_strings * strings.panels_per_string;

    let array_width_mm = panels_across * panel.width_mm + (panels_across - 1) * roof.gap_mm;
    let array_depth_mm = rows_deep * panel.height_mm + (rows_deep - 1) * roof.gap_mm;

    let margin_left_mm = (roof.width_mm - array_width_mm) / 2;
    let margin_top_mm = (roof.depth_mm - array_depth_mm) / 2;

    // ── Electrical (per string: panels in series) ─────────────────
    let string_vmp_mv = panel.vmp_mv * strings.panels_per_string;
    let string_voc_mv = panel.voc_mv * strings.panels_per_string;
    let string_imp_ma = panel.imp_ma;  // series: current stays same
    let string_isc_ma = panel.isc_ma;
    let string_watt_peak = panel.watt_peak * strings.panels_per_string;

    // Total array power
    let total_watt_peak = panel.watt_peak * total_panels;

    // ── Weight ────────────────────────────────────────────────────
    let total_weight_g = panel.weight_g * total_panels;

    // Weight per m² — (total_g * 1_000_000) / (width_mm * depth_mm)
    // to get g/m² without overflow: use u64
    let roof_area_mm2 = roof.width_mm as u64 * roof.depth_mm as u64;
    let weight_per_sqm_g = if roof_area_mm2 > 0 {
        ((total_weight_g as u64) * 1_000_000 / roof_area_mm2) as u32
    } else {
        0
    };

    // ── Energy yield estimate ─────────────────────────────────────
    // Peak sun hours for central Germany (~50°N): ~2.8 h/day annual average
    // (accounts for clouds, winter, angle losses)
    // Better estimate: use latitude to approximate
    let peak_sun_hours_x10 = estimate_peak_sun_hours(roof.latitude_deg_x10);

    // System losses: ~80% (inverter, cable, temperature, soiling, mismatch)
    // daily_kwh = total_watt_peak * peak_sun_hours / 1000 * 0.80
    // In integer: daily_kwh_x10 = total_watt_peak * psh_x10 * 8 / 10000
    let daily_kwh_x10 = (total_watt_peak as u64 * peak_sun_hours_x10 as u64 * 8 / 10000) as u32;

    // Annual: daily × 365
    let annual_kwh = daily_kwh_x10 * 365 / 10;

    // Specific yield: kWh per kWp per year
    let specific_yield = if total_watt_peak > 0 {
        annual_kwh * 1000 / total_watt_peak
    } else {
        0
    };

    // ── Inter-row shading ─────────────────────────────────────────
    // At winter solstice (~21 Dec), sun elevation at solar noon:
    //   elevation = 90 - latitude + 23.45 ≈ 90 - 50.1 + 23.45 = 63.35° (from zenith)
    //   → sun elevation = 90 - 63.35 = 26.65°... wait:
    //   elevation = 90 - latitude - 23.45 (winter) = 90 - 50.1 - 23.45 = 16.45°
    //
    // Min row spacing = panel_height * sin(tilt) / tan(sun_elevation)
    // Using lookup: sin(24°) ≈ 0.407, tan(16.45°) ≈ 0.295
    // spacing = 1134 * 407 / 1000 * 1000 / 295 = 1134 * 407 / 295 ≈ 1564 mm
    let min_row_spacing_mm = compute_min_row_spacing(
        panel.height_mm,
        roof.max_tilt_deg,
        roof.latitude_deg_x10,
    );

    let actual_row_pitch = panel.height_mm + roof.gap_mm;
    let shading_ok = actual_row_pitch >= min_row_spacing_mm;

    PlannerResult {
        panels_across,
        rows_deep,
        max_panels_fit,
        total_panels,
        array_width_mm,
        array_depth_mm,
        margin_left_mm,
        margin_top_mm,
        string_vmp_mv,
        string_voc_mv,
        string_imp_ma,
        string_isc_ma,
        string_watt_peak,
        total_watt_peak,
        total_weight_g,
        weight_per_sqm_g,
        peak_sun_hours_x10,
        daily_kwh_x10,
        annual_kwh,
        specific_yield,
        min_row_spacing_mm,
        shading_ok,
    }
}

/// Estimate average daily peak sun hours from latitude.
/// Uses a simple linear model for central Europe (40°N–60°N).
/// Returns hours × 10 (e.g. 28 = 2.8 h/day).
fn estimate_peak_sun_hours(latitude_deg_x10: u32) -> u32 {
    // Rough model: PSH decreases ~0.07 h per degree latitude in Europe
    // Reference: 48°N ≈ 3.0 h, 52°N ≈ 2.7 h, 55°N ≈ 2.4 h
    // Formula: PSH_x10 = 64 - (latitude_x10 - 400) * 7 / 100
    let lat = latitude_deg_x10;
    if lat <= 400 {
        35 // southern Europe: ~3.5 h
    } else if lat >= 600 {
        20 // northern Europe: ~2.0 h
    } else {
        // Linear interpolation 40°N=3.5h to 60°N=2.0h
        (35 - (lat - 400) * 15 / 200) as u32
    }
}

/// Compute minimum inter-row spacing to avoid shading at winter solstice.
/// Returns spacing in mm.
fn compute_min_row_spacing(panel_height_mm: u32, tilt_deg: u32, latitude_deg_x10: u32) -> u32 {
    // Winter solstice sun elevation = 90 - latitude - 23.45
    // sun_elev_x10 = 900 - latitude_x10 - 234
    let sun_elev_x10 = if 900 > latitude_deg_x10 + 234 {
        900 - latitude_deg_x10 - 234
    } else {
        10 // minimum 1°
    };

    // sin(tilt) lookup — integer sin×1000 for 0-45°
    let sin_tilt = sin_lookup(tilt_deg);
    // tan(sun_elevation) lookup — integer tan×1000
    let tan_sun = tan_lookup(sun_elev_x10 / 10);

    if tan_sun == 0 {
        return panel_height_mm * 5; // very low sun — huge spacing needed
    }

    // spacing = panel_height * sin(tilt) / tan(sun_elev)
    (panel_height_mm as u64 * sin_tilt as u64 / tan_sun as u64) as u32
}

/// Integer sin(degrees) × 1000 lookup for 0–90°
fn sin_lookup(deg: u32) -> u32 {
    const TABLE: [u32; 91] = [
        0, 17, 35, 52, 70, 87, 105, 122, 139, 156,
        174, 191, 208, 225, 242, 259, 276, 292, 309, 326,
        342, 358, 375, 391, 407, 423, 438, 454, 469, 485,
        500, 515, 530, 545, 559, 574, 588, 602, 616, 629,
        643, 656, 669, 682, 695, 707, 719, 731, 743, 755,
        766, 777, 788, 799, 809, 819, 829, 839, 848, 857,
        866, 875, 883, 891, 899, 906, 914, 921, 927, 934,
        940, 946, 951, 956, 961, 966, 970, 974, 978, 982,
        985, 988, 990, 993, 995, 996, 998, 999, 999, 1000,
        1000,
    ];
    if deg > 90 { 1000 } else { TABLE[deg as usize] }
}

/// Integer tan(degrees) × 1000 lookup for 0–89°
fn tan_lookup(deg: u32) -> u32 {
    const TABLE: [u32; 90] = [
        0, 17, 35, 52, 70, 87, 105, 123, 141, 158,
        176, 194, 213, 231, 249, 268, 287, 306, 325, 344,
        364, 384, 404, 424, 445, 466, 488, 510, 532, 554,
        577, 601, 625, 649, 675, 700, 727, 754, 781, 810,
        839, 869, 900, 933, 966, 1000, 1036, 1072, 1111, 1150,
        1192, 1235, 1280, 1327, 1376, 1428, 1483, 1540, 1600, 1664,
        1732, 1804, 1881, 1963, 2050, 2145, 2246, 2356, 2475, 2605,
        2747, 2904, 3078, 3271, 3487, 3732, 4011, 4331, 4705, 5145,
        5671, 6314, 7115, 8144, 9514, 11430, 14301, 19081, 28636, 57290,
    ];
    if deg >= 90 { 57290 } else { TABLE[deg as usize] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wintergarden_layout() {
        let panel = PanelSpec {
            name: "Trina 620W",
            width_mm: 2278,
            height_mm: 1134,
            depth_mm: 30,
            weight_g: 29500,
            watt_peak: 620,
            vmp_mv: 41400,
            imp_ma: 14990,
            voc_mv: 49200,
            isc_ma: 15920,
            efficiency_pct_x10: 227,
        };
        let roof = RoofSpec {
            name: "Wintergarden",
            width_mm: 9530,
            depth_mm: 3400,
            base_tilt_deg: 8,
            max_tilt_deg: 24,
            latitude_deg_x10: 501,
            azimuth_deg: 180,
            gap_mm: 20,
        };
        let strings = StringConfig {
            panels_per_string: 4,
            num_strings: 3,
        };

        let r = compute(&panel, &roof, &strings);

        assert_eq!(r.panels_across, 4);     // 9530 / 2298 = 4.14 → 4
        assert_eq!(r.rows_deep, 2);         // 3400 / 1154 = 2.94 → 2
        assert_eq!(r.max_panels_fit, 8);    // 4 × 2 = 8
        assert_eq!(r.total_panels, 12);     // 3 strings × 4
        assert_eq!(r.total_watt_peak, 7440);// 12 × 620
        assert_eq!(r.string_vmp_mv, 165600);// 4 × 41400
        assert_eq!(r.string_voc_mv, 196800);// 4 × 49200

        // Weight
        assert_eq!(r.total_weight_g, 354000); // 12 × 29500

        // Energy: should be in reasonable range for 50°N
        assert!(r.annual_kwh > 5000);
        assert!(r.annual_kwh < 10000);
    }

    #[test]
    fn test_sin_lookup() {
        assert_eq!(sin_lookup(0), 0);
        assert_eq!(sin_lookup(30), 500);
        assert_eq!(sin_lookup(90), 1000);
    }

    #[test]
    fn test_tan_lookup() {
        assert_eq!(tan_lookup(0), 0);
        assert_eq!(tan_lookup(45), 1000);
    }

    #[test]
    fn test_peak_sun_hours() {
        let psh_501 = estimate_peak_sun_hours(501);
        assert!(psh_501 >= 25 && psh_501 <= 32); // ~2.5-3.2 for 50.1°N
    }
}
