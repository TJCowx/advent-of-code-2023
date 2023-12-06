use crate::file_reader::file_reader;

pub fn solve() {
    let mut lines = file_reader::read_as_vec("inputs/day06.txt");

    let time_line = lines.remove(0);
    let distance_line = lines.remove(0);

    // parse the time line into the array of numbers
    let record_times = time_line.split(' ').fold(vec![], |mut acc, x| {
        match x.parse::<u64>() {
            Ok(_) => acc.push(x.parse::<u64>().unwrap()),
            Err(_) => (),
        }
        acc
    });

    let distances = distance_line.split(' ').fold(vec![], |mut acc, x| {
        match x.parse::<u64>() {
            Ok(_) => acc.push(x.parse::<u64>().unwrap()),
            Err(_) => (),
        }
        acc
    });

    println!("Record Times: \n_______________\n {:#?}", record_times);
    println!("Distances: \n_______________\n {:#?}", distances);

    let mut res = 0;

    for (i, record) in record_times.iter().enumerate() {
        let mut num_rec_beaten = 0;
        let distance = distances.get(i).unwrap();

        // Lets brute force this
        for hold_time in 1..*record {
            let time_remaining = record - hold_time;

            let distance_traveled = hold_time * time_remaining;

            if distance_traveled > *distance {
                num_rec_beaten += 1;
            }
        }
        println!("{record} Times beaten: {}", num_rec_beaten);

        if res == 0 {
            res = num_rec_beaten;
        } else {
            res = res * num_rec_beaten;
        }
    }

    println!("\n\nRESULT: {}", res);
}
