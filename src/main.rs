use std::collections::HashSet;
use std::iter::FromIterator;
use serde_json::Result;


fn validate_flights(flights: Vec<Vec<String>>) -> Result<String> {
    let mut source_master_list: Vec<String> = Vec::new();
    let mut destination_master_list: Vec<String> = Vec::new();

    for flight in flights {
        source_master_list.push(flight[0].to_string());
        destination_master_list.push(flight[1].to_string());
    }

    // Get the symmetric difference for flights
    let final_source: HashSet<String> = HashSet::from_iter(source_master_list);
    let final_destination: HashSet<String> = HashSet::from_iter(destination_master_list);

    let difference = final_source.symmetric_difference(&final_destination);
    return  Ok(format!("{:?}", difference));
}

fn main() {
    // Process user input
    let mut input = String::new();
    println!("Enter the flight sequence (use format [[\"source code\",\"destination code\"], ...] ): ");
    _ = std::io::stdin().read_line(&mut input).expect("Invalid user input");
    let flights: Vec< Vec<String>> = serde_json::from_str(&input).expect("Invalid flight trip data");
    print!("{:?}", flights);
    let res = validate_flights(flights).expect("Invalid flight");
    println!(" => {}", res);
}


// The next line exists to trick play.rust-lang.org into running our code as a
// test:
// fn main

#[cfg(test)]
mod tests {
    use super::validate_flights;

    #[test]
    fn one_flight_works() {
        let one_flight = r#"[["IND", "EWR"], ["SFO", "ATL"], ["GSO", "IND"], ["ATL", "GSO"]]"#;
        let flights: Vec< Vec<String>> = serde_json::from_str(one_flight).expect("Invalid flight trip data");
        let res = validate_flights(flights).expect("Invalid flight");
        let expected = String::from("[\"SFO\", \"EWR\"]");
        assert_eq!(expected, res);
    }

    #[test]
    fn two_flights_works() {
        let two_flights = r#"[["IND", "EWR"], ["SFO", "ATL"], ["GSO", "IND"], ["ATL", "GSO"]]"#;
        let flights: Vec< Vec<String>> = serde_json::from_str(two_flights).expect("Invalid flight trip data");
        let res = validate_flights(flights).expect("Invalid flight");
        let expected = String::from("[\"SFO\", \"EWR\"]");
        assert_eq!(expected, res);
    }

    #[test]
    fn multiple_flights_works() {
        let multiple_flights = r#"[["IND", "EWR"], ["SFO", "ATL"], ["GSO", "IND"], ["ATL", "GSO"]]"#;
        let flights: Vec< Vec<String>> = serde_json::from_str(multiple_flights).expect("Invalid flight trip data");
        let res = validate_flights(flights).expect("Invalid flight");
        let expected = String::from("[\"SFO\", \"EWR\"]");
        assert_eq!(expected, res);
    }
}
