#[no_mangle]
pub extern "C" fn entry() {
    {
        use jiff::Timestamp;
        let now = Timestamp::now();
        println!("Timestamp now: {:?}", now);
    }

    {
        use jiff::tz::TimeZone;
        let tz_result = TimeZone::try_system();
        assert!(tz_result.is_err());
        println!("System tz result: {:?}", tz_result); // failure
    }

    {
        use jiff::{civil::date, tz::TimeZone};
        let tz_name = "America/New_York";
        let tz_data = android_tzdata::find_tz_data(tz_name).unwrap();
        let tz = TimeZone::tzif(tz_name, &tz_data).unwrap();
        let zdt = date(2023, 12, 31).at(18, 30, 0, 0).to_zoned(tz).unwrap();
        assert_eq!(
            zdt.to_string(),
            "2023-12-31T18:30:00-05:00[America/New_York]"
        );

        println!("{:?}", zdt);
    }

    {
        use jiff::{tz::TimeZone, Timestamp};
        let tz_name = iana_time_zone::get_timezone().unwrap();
        let tz_data = android_tzdata::find_tz_data(&tz_name).unwrap();
        let tz = TimeZone::tzif(&tz_name, &tz_data).unwrap();
        let zdt_now = Timestamp::now().to_zoned(tz);
        println!("The current time is: {}", zdt_now);
    }
}
