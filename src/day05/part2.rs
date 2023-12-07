use crate::file_reader::file_reader;

#[derive(Debug)]
struct ValRange {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug)]
struct ConversionRange {
    pub source_start: u64,
    pub destination_start: u64,
    pub length: u64,
}

impl ConversionRange {
    pub fn get_source_end(&self) -> u64 {
        self.source_start + self.length
    }

    pub fn get_offset(&self, val: u64) -> u64 {
        val - self.source_start
    }

    pub fn get_destination_val(&self, val: u64) -> u64 {
        self.destination_start + self.get_offset(val)
    }
}

#[derive(Debug)]
struct ConversionMap {
    pub min_conversion: u64,
    pub max_conversion: u64,
    pub conversions: Vec<ConversionRange>,
}

impl ConversionMap {
    fn new(data: &Vec<String>) -> Self {
        let mut iter = data.iter();
        println!("\nProcessing {}\n___________", iter.next().unwrap());

        let mut ranges = vec![];

        while let Some(group) = iter.next() {
            let items = group.split_whitespace().collect::<Vec<&str>>();
            let conversion = ConversionRange {
                destination_start: items[0].parse::<u64>().unwrap(),
                source_start: items[1].parse::<u64>().unwrap(),
                length: items[2].parse::<u64>().unwrap(),
            };

            ranges.push(conversion);
        }

        ranges.sort_by(|a, b| a.source_start.cmp(&b.source_start));

        println!("\nDone processing");

        let min_conversion = ranges[0].source_start;
        let max_conversion = ranges[ranges.len() - 1].get_source_end();

        ConversionMap {
            conversions: ranges,
            min_conversion,
            max_conversion,
        }
    }

    // TODO: Logic in here is off, figure it out
    fn get_new_ranges(&self, in_ranges: &mut Vec<ValRange>) -> Vec<ValRange> {
        let mut new_ranges = vec![];

        for range in in_ranges {
            // First find if the range starts before the min conversion
            if range.start > self.min_conversion && range.end < self.min_conversion {
                new_ranges.push(ValRange {
                    start: range.start,
                    end: range.end,
                });
            } else if range.start > self.max_conversion {
                new_ranges.push(ValRange {
                    start: range.start,
                    end: range.end,
                });
            } else {
                for conversion in &self.conversions {
                    // Skip this if the start is after the end
                    if range.start > conversion.get_source_end() {
                        continue;
                    }

                    if range.end < conversion.source_start {
                        new_ranges.push(ValRange {
                            start: range.start,
                            end: range.end,
                        });
                        continue;
                    }
                    // If the range starts before the conversion, we need to split it
                    if range.start < conversion.source_start {
                        // Push anything that is before the start of conversion and don't convert
                        new_ranges.push(ValRange {
                            start: range.start,
                            end: conversion.source_start - 1,
                        });

                        range.start = conversion.source_start;
                    }

                    // If the range ends after the conversion we need to split what's in the conversion
                    if range.end > conversion.get_source_end() {
                        new_ranges.push(ValRange {
                            start: conversion.get_destination_val(range.start),
                            end: conversion.get_destination_val(conversion.get_source_end()),
                        });

                        range.start = conversion.get_source_end() + 1;

                        continue;
                    }

                    // If the whole range is in the conversion, just convert it
                    if range.start >= conversion.source_start
                        && range.end <= conversion.get_source_end()
                    {
                        new_ranges.push(ValRange {
                            start: conversion.get_destination_val(range.start),
                            end: conversion.get_destination_val(range.end),
                        });

                        break;
                    }

                    println!("I feel like this should never happen");
                }
            }
        }

        new_ranges.sort_by(|a, b| a.start.cmp(&b.start));

        new_ranges
    }
}

fn get_seeds(line: String) -> Vec<ValRange> {
    let nums: Vec<u64> = line
        .replace("seeds: ", "")
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut ranges = vec![];
    for chunk in nums.chunks(2) {
        println!("Chunk: {:#?}", chunk);
        ranges.push(ValRange {
            start: chunk[0],
            end: chunk[0] + chunk[1] - 1, // Include the start
        });
    }

    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    ranges
}

pub fn solve() {
    println!("Day 5 Part 2");
    let lines = file_reader::read_as_vec("inputs/day05-test.txt");

    // let mut lowest: u64 = u64::MAX;

    let data: Vec<Vec<String>> = lines.split(|s| s.is_empty()).map(|g| g.to_vec()).collect();

    let mut iter = data.iter();

    let seed_string = iter.next().unwrap().get(0).unwrap().to_string();
    let mut seeds = get_seeds(seed_string);

    println!("Found Seeds");
    println!("______________________");
    println!("{:#?}", seeds);

    println!("Processing maps...");
    let seed_to_soil = ConversionMap::new(iter.next().unwrap());
    let soil_to_fertilizer = ConversionMap::new(iter.next().unwrap());
    let fertilizer_to_water = ConversionMap::new(iter.next().unwrap());
    let water_to_light = ConversionMap::new(iter.next().unwrap());
    let light_to_temp = ConversionMap::new(iter.next().unwrap());
    let temp_to_humidity = ConversionMap::new(iter.next().unwrap());
    let humidity_to_location = ConversionMap::new(iter.next().unwrap());
    println!("Done processing maps");

    println!("Generating Seed to Soil");
    let mut soil_ranges = seed_to_soil.get_new_ranges(&mut seeds);
    println!("Soil Ranges: {:#?}", soil_ranges);
    println!("Generating Soil to Fertilizer");
    let mut fertilizer_ranges = soil_to_fertilizer.get_new_ranges(&mut soil_ranges);
    println!("Fertilizer Ranges: {:#?}", fertilizer_ranges);
    println!("Generating Fertilizer to Water");
    let mut water_ranges = fertilizer_to_water.get_new_ranges(&mut fertilizer_ranges);
    println!("Water Ranges: {:#?}", water_ranges);
    println!("Generating Water to Light");
    let mut light_ranges = water_to_light.get_new_ranges(&mut water_ranges);
    println!("Light Ranges: {:#?}", light_ranges);
    println!("Generating Light to Temp");
    let mut temp_ranges = light_to_temp.get_new_ranges(&mut light_ranges);
    println!("Temp Ranges: {:#?}", temp_ranges);
    println!("Generating Temp to Humidity");
    let mut humidity_ranges = temp_to_humidity.get_new_ranges(&mut temp_ranges);
    println!("Humidity Ranges: {:#?}", humidity_ranges);
    println!("Generating Humidity to Location");
    let mut location_ranges = humidity_to_location.get_new_ranges(&mut humidity_ranges);
    println!("Final Ranges: {:#?}", location_ranges);
}
