use astro::*;

fn main() {
    let day_of_month = time::DayOfMonth {
        day: 9,
        hr: 18,
        min: 43,
        sec: 4.0,
        time_zone: 0.0,
    };

    let date = time::Date {
        year: 2025,
        month: 9,
        decimal_day: time::decimal_day(&day_of_month),
        cal_type: time::CalType::Gregorian,
    };

    let julian_day = time::julian_day(&date);
    let (moon_ecl_point, rad_vec_moon) = lunar::geocent_ecl_pos(julian_day);
    print!("lon: {} | lat: {}", moon_ecl_point.long, moon_ecl_point.lat);
    println!("{} km", rad_vec_moon * 149597870.691);
}
