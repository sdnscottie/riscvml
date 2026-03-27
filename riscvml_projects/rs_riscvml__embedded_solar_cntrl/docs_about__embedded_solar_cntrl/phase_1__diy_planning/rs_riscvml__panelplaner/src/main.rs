// rs_riscvml__panelplaner — Solar Panel Layout + Engineering Planner
//
// Primary target: ESP32-P4-WIFI6 Kit A (no_std capable)
// Optional: Raspberry Pi, desktop
//
// RISCVML — riscvml.org
// Author: Scottie von Bruchhausen

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

mod planner;

#[cfg(feature = "std")]
fn main() {
    use planner::{PanelSpec, RoofSpec, StringConfig, PlannerResult};

    // ── TrinaSolar TSM-620NEG19RC.20 ──────────────────────────────
    let panel = PanelSpec {
        name: "TrinaSolar TSM-620NEG19RC.20",
        width_mm: 2278,        // landscape width
        height_mm: 1134,       // landscape height
        depth_mm: 30,
        weight_g: 29500,       // 29.5 kg
        watt_peak: 620,        // Wp
        vmp_mv: 41400,         // Vmp = 41.40V (millivolts)
        imp_ma: 14990,         // Imp = 14.99A (milliamps)
        voc_mv: 49200,         // Voc = 49.20V
        isc_ma: 15920,         // Isc = 15.92A
        efficiency_pct_x10: 227, // 22.7% (×10 for integer math)
    };

    // ── Wintergarden Roof ─────────────────────────────────────────
    let roof = RoofSpec {
        name: "Wintergarden South Roof",
        width_mm: 9530,        // 9.53m
        depth_mm: 3400,        // 3.40m
        base_tilt_deg: 8,      // roof incline
        max_tilt_deg: 24,      // actuator max
        latitude_deg_x10: 501, // 50.1°N (×10)
        azimuth_deg: 180,      // due south
        gap_mm: 20,            // gap between panels
    };

    // ── String Configuration ──────────────────────────────────────
    let strings = StringConfig {
        panels_per_string: 4,
        num_strings: 3,
    };

    let result = planner::compute(&panel, &roof, &strings);
    print_result(&panel, &roof, &strings, &result);
}

#[cfg(feature = "std")]
fn print_result(
    panel: &planner::PanelSpec,
    roof: &planner::RoofSpec,
    strings: &planner::StringConfig,
    r: &planner::PlannerResult,
) {
    println!();
    println!("╔══════════════════════════════════════════════════════════╗");
    println!("║  RISCVML — Solar Panel Planner                         ║");
    println!("║  rs_riscvml__panelplaner v0.1.0                        ║");
    println!("║  riscvml.org                                           ║");
    println!("╚══════════════════════════════════════════════════════════╝");
    println!();

    println!("── PANEL: {} ──", panel.name);
    println!("  Dimensions:   {}×{}×{} mm (landscape)",
             panel.width_mm, panel.height_mm, panel.depth_mm);
    println!("  Weight:       {}.{} kg", panel.weight_g / 1000, (panel.weight_g % 1000) / 100);
    println!("  Power:        {} Wp", panel.watt_peak);
    println!("  Vmp/Imp:      {}.{} V / {}.{} A",
             panel.vmp_mv / 1000, (panel.vmp_mv % 1000) / 10,
             panel.imp_ma / 1000, (panel.imp_ma % 1000) / 10);
    println!("  Voc/Isc:      {}.{} V / {}.{} A",
             panel.voc_mv / 1000, (panel.voc_mv % 1000) / 10,
             panel.isc_ma / 1000, (panel.isc_ma % 1000) / 10);
    println!("  Efficiency:   {}.{}%",
             panel.efficiency_pct_x10 / 10, panel.efficiency_pct_x10 % 10);
    println!();

    println!("── ROOF: {} ──", roof.name);
    println!("  Dimensions:   {}.{} m × {}.{} m",
             roof.width_mm / 1000, (roof.width_mm % 1000) / 10,
             roof.depth_mm / 1000, (roof.depth_mm % 1000) / 10);
    println!("  Tilt range:   {}° – {}°", roof.base_tilt_deg, roof.max_tilt_deg);
    println!("  Latitude:     {}.{}°N", roof.latitude_deg_x10 / 10, roof.latitude_deg_x10 % 10);
    println!("  Azimuth:      {}° (south = 180°)", roof.azimuth_deg);
    println!("  Panel gap:    {} mm", roof.gap_mm);
    println!();

    println!("── LAYOUT ──");
    println!("  Panels across:   {}", r.panels_across);
    println!("  Rows deep:       {}", r.rows_deep);
    println!("  Total panels:    {} (of {} max fit)", r.total_panels, r.max_panels_fit);
    println!("  Used vs config:  {} panels configured ({} strings × {} panels)",
             strings.num_strings * strings.panels_per_string,
             strings.num_strings, strings.panels_per_string);
    println!("  Array width:     {}.{} m",
             r.array_width_mm / 1000, (r.array_width_mm % 1000) / 10);
    println!("  Array depth:     {}.{} m",
             r.array_depth_mm / 1000, (r.array_depth_mm % 1000) / 10);
    println!("  Margin left:     {} mm", r.margin_left_mm);
    println!("  Margin top:      {} mm", r.margin_top_mm);
    println!();

    println!("── ELECTRICAL ──");
    println!("  String Vmp:      {}.{} V ({} panels × {}.{} V)",
             r.string_vmp_mv / 1000, (r.string_vmp_mv % 1000) / 10,
             strings.panels_per_string,
             panel.vmp_mv / 1000, (panel.vmp_mv % 1000) / 10);
    println!("  String Voc:      {}.{} V",
             r.string_voc_mv / 1000, (r.string_voc_mv % 1000) / 10);
    println!("  String Imp:      {}.{} A (same as panel Imp)",
             r.string_imp_ma / 1000, (r.string_imp_ma % 1000) / 10);
    println!("  String Isc:      {}.{} A",
             r.string_isc_ma / 1000, (r.string_isc_ma % 1000) / 10);
    println!("  String power:    {} Wp", r.string_watt_peak);
    println!("  Total array:     {} Wp ({}.{} kWp)",
             r.total_watt_peak, r.total_watt_peak / 1000, (r.total_watt_peak % 1000) / 100);
    println!();

    println!("── WEIGHT ──");
    println!("  Panel weight:    {}.{} kg × {} = {}.{} kg",
             panel.weight_g / 1000, (panel.weight_g % 1000) / 100,
             r.total_panels,
             r.total_weight_g / 1000, (r.total_weight_g % 1000) / 100);
    println!("  Per sq metre:    {}.{} kg/m²",
             r.weight_per_sqm_g / 1000, (r.weight_per_sqm_g % 1000) / 100);
    println!();

    println!("── ENERGY YIELD ESTIMATE ──");
    println!("  Peak sun hours:  ~{}.{} h/day (Bad Schwalbach avg)",
             r.peak_sun_hours_x10 / 10, r.peak_sun_hours_x10 % 10);
    println!("  Daily yield:     ~{}.{} kWh",
             r.daily_kwh_x10 / 10, r.daily_kwh_x10 % 10);
    println!("  Annual yield:    ~{} kWh ({}.{} MWh)",
             r.annual_kwh, r.annual_kwh / 1000, (r.annual_kwh % 1000) / 100);
    println!("  Specific yield:  ~{} kWh/kWp/year", r.specific_yield);
    println!();

    println!("── INTER-ROW SHADING ──");
    println!("  Min row spacing at {}°: {} mm",
             roof.max_tilt_deg, r.min_row_spacing_mm);
    println!("  Actual row pitch:      {} mm (panel height + gap)",
             panel.height_mm + roof.gap_mm);
    if r.shading_ok {
        println!("  Status:                OK — no inter-row shading at winter solstice");
    } else {
        println!("  Status:                WARNING — inter-row shading possible at low sun angles");
    }
    println!();
}

// ── no_std entry point for ESP32-P4 ────────────────────────────────
#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(not(feature = "std"))]
#[no_mangle]
pub extern "C" fn main() -> ! {
    // ESP32-P4: compute and output via UART
    // TODO: integrate with esp-hal serial output
    loop {}
}
