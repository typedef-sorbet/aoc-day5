use std::{collections::HashMap, fs::File, io::{BufReader, BufRead}, thread::current, mem::discriminant};

#[derive(Debug)]
struct FarmMapping {
    dest_start: i64,
    src_start: i64,
    range: i64
}

type Almanac = HashMap<(Resource, Resource), Vec<FarmMapping>>;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Hash)]
#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Debug)]
enum Resource {
    Seed(i64),
    Soil(i64),
    Fertilizer(i64),
    Water(i64),
    Light(i64),
    Temperature(i64),
    Humidity(i64),
    Location(i64)
}

// The vec reference should live as long as the almanac does
fn find_mappings_for_dest_resource<'a>(resource: &Resource, conversion_table: &'a Almanac) -> Option<&'a Vec<FarmMapping>> {
    conversion_table.iter()
                    .filter(|((_r_src, r_dest), v)| std::mem::discriminant(resource) == std::mem::discriminant(r_dest))     // filter for any (should be only one) entry where the destination resource enum _variant_ matches that of the given resource
                    .map(|(k, v)| v)                                                                                        // just grab the value
                    .next()                                                                                                 // return the option of the "next" (read: only or None) value    
}

fn get_resource_num(resource: &Resource) -> &i64 {
    match resource {
        Resource::Seed(x) |
        Resource::Soil(x) |
        Resource::Fertilizer(x) |
        Resource::Water(x) |
        Resource::Light(x) |
        Resource::Temperature(x) |
        Resource::Humidity(x) |
        Resource::Location(x) => x
    }
}

fn to_previous_resource(resource: Resource, new_num: Option<i64>) -> Resource {
    match resource {
        Resource::Seed(x)           => panic!("Cannot back-convert from a Seed"),
        Resource::Soil(x)           => Resource::Seed(new_num.unwrap_or(x)),
        Resource::Fertilizer(x)     => Resource::Soil(new_num.unwrap_or(x)),
        Resource::Water(x)          => Resource::Fertilizer(new_num.unwrap_or(x)),
        Resource::Light(x)          => Resource::Water(new_num.unwrap_or(x)),
        Resource::Temperature(x)    => Resource::Light(new_num.unwrap_or(x)),
        Resource::Humidity(x)       => Resource::Temperature(new_num.unwrap_or(x)),
        Resource::Location(x)       => Resource::Humidity(new_num.unwrap_or(x))
    }
}

fn create_conversion_table() -> Result<(Vec<i64>, Almanac), &'static str> {
    if let Ok(file) = File::open("./day5.txt") {
        let reader = BufReader::new(file);
        let mut almanac: Almanac = HashMap::new();
        let mut seeds: Vec<i64> = Vec::new();

        // Start parsing the file
        let mut current_resource: Option<(Resource, Resource)> = None;

        for line in reader.lines().flatten() {
            match line.as_str() {
                // Handle state transitions
                "seed-to-soil map:"             => current_resource = Some((Resource::Seed(0), Resource::Soil(0))),
                "soil-to-fertilizer map:"       => current_resource = Some((Resource::Soil(0), Resource::Fertilizer(0))),
                "fertilizer-to-water map:"      => current_resource = Some((Resource::Fertilizer(0), Resource::Water(0))),
                "water-to-light map:"           => current_resource = Some((Resource::Water(0), Resource::Light(0))),
                "light-to-temperature map:"     => current_resource = Some((Resource::Water(0), Resource::Temperature(0))),
                "temperature-to-humidity map:"  => current_resource = Some((Resource::Temperature(0), Resource::Humidity(0))),
                "humidity-to-location map:"     => current_resource = Some((Resource::Humidity(0), Resource::Location(0))),
                
                // Handle general lines
                _ => {
                    match current_resource {
                        None => {
                            // We must be on the very first line, or the first empty line. If non-empty, parse it as a list of seed numbers.
                            if !line.is_empty() {
                                seeds = line.split(" ")
                                            .filter(|s| *s != "seeds:")     // Toss the list header
                                            .map(|s| s.parse::<i64>())      // &str -> i64
                                            .flatten()                      // Toss any Err
                                            .collect::<Vec<i64>>();         // Collect as vec of i64
                            }
                        }
                        Some(resource_tuple) => {
                            // This is a mapping line, or an empty line.
                            if !line.is_empty() {
                               let mut tokens = line.split(" ")
                                                    .map(|s| s.parse::<i64>())
                                                    .flatten()
                                                    .collect::<Vec<i64>>();

                                if !almanac.contains_key(&resource_tuple) {
                                    almanac.insert(resource_tuple.clone(), Vec::new());
                                }

                                almanac.get_mut(&resource_tuple).unwrap().push(FarmMapping {
                                    dest_start: tokens.remove(0),
                                    src_start: tokens.remove(0),
                                    range: tokens.remove(0)
                                });
                            }
                        }
                    }
                }
            }
        }
        
        return Ok((seeds, almanac));
    }

    // ...
    Err("Unable to open file ./day5.txt")
}

// Converts resource *backwards* through the conversion table -- so Locations get converted to Humidity, Humidity to Temperature, etc.
fn convert_resource(resource: Resource, conversion_table: &Almanac) -> Resource {
    if let Some(mappings) = find_mappings_for_dest_resource(&resource, conversion_table) {
        // We have a mappings vec. See if any of the ranges apply.
        let resource_num = get_resource_num(&resource);

        for FarmMapping{dest_start, src_start, range} in mappings {
            if dest_start <= resource_num && *resource_num < (dest_start + range) {
                return to_previous_resource(resource, Some(src_start + (resource_num - dest_start)))
            }
        }

        // No mapping applied -- use default
        return to_previous_resource(resource, None);
    }
    else {
        println!("Unable to find mappings for destination resource with discriminant {:?}", std::mem::discriminant(&resource));
    }

    Resource::Seed(0)
}

fn main() {
    println!("Hello, world!");

    if let Ok((seeds, almanac)) = create_conversion_table() {
        println!("Seeds: {:?}", seeds);
        println!("Almanac: {:?}", almanac);
    }
}
