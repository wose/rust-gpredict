extern crate gpredict;

use gpredict::{Predict, Location, Tle};

use std::thread;
use std::time::Duration;

fn main() {
    // start processing
    println!("predict example started");

    let tle: Tle = Tle {
        name: "GRIFEX".to_string(),
        line1: "1 40379U 15003D   15243.42702278  .00003367  00000-0  17130-3 0  9993".to_string(),
        line2: "2 40379  99.1124 290.6779 0157088   8.9691 351.4280 15.07659299 31889".to_string()
    };

    let location: Location = Location{lat_deg:58.64560, lon_deg: 23.15163, alt_m: 8.};
    let mut predict: Predict = Predict::new(&tle, &location);

    loop {
        // these two are the same:
        predict.update(None);

        println!("aos        : {:}", predict.sat.aos.expect("do not have AOS with this satellite").as_gregorian_utc_str());
        println!("los        : {:}", predict.sat.los.expect("do not have LOS with this satellite").as_gregorian_utc_str());
        println!("az         : {:.2}°", predict.sat.az_deg);
        println!("el         : {:.2}°", predict.sat.el_deg);
        println!("range      : {:.0} km", predict.sat.range_km);
        println!("range rate : {:.3} km/sec\n", predict.sat.range_rate_km_sec);

        thread::sleep(Duration::from_secs(1));
    }
}
