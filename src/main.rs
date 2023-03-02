#![feature(test)]
#![feature(portable_simd)]
#![feature(type_ascription)]

use std::thread;
use std::time::Duration;
use std::simd;
use std::simd::i32x2;

// The task was as follows: There were a number of sensors at a given XY coordinate, and they pick
// up the closest beacon, and give the distance based on Manhattan geometry (e.g. the distance to#
// the beacon when only traversing the grid at right angles, like a taxi in Manhattan).
//
// The goal is to find the one grid cell which is not in the range of any beacon.

fn main() {

    // This is the list of sensors, and the beacons they picked up.
    // SIMD means Single Instruction Multiple Data, and allows for simultaneous processing of
    // multiple datasets of the same type, which I used to slightly speed up the computation.
    // They are sorted by size from biggest to smallest to speed up computation time, as a bigger
    // beacon will cover more squares and as such result in a faster negative result, because we need
    // to find the one square which is in the range of no beacon.
    let sensors: [(i32x2, u32); 27] = [
        (simd::i32x2::from([2594265, 638715]), 1621697),
    (simd::i32x2::from([3633714, 1229193]), 1579240),
    (simd::i32x2::from([282831, 991087]), 1538758),
    (simd::i32x2::from([3999451, 15688]), 1485109),
    (simd::i32x2::from([2281504, 3945638]), 1334463),
    (simd::i32x2::from([1446898, 1674290]), 1212665),
    (simd::i32x2::from([822012, 3898848]), 1202376),
    (simd::i32x2::from([1139483, 2716286]), 1036910),
    (simd::i32x2::from([111006, 2376713]), 1030765),
    (simd::i32x2::from([3954504, 3606495]), 923686),
    (simd::i32x2::from([2482128, 2934657]), 746580),
    (simd::i32x2::from([1290563, 46916]), 681921),
    (simd::i32x2::from([3961416, 2485266]), 650869),
    (simd::i32x2::from([3280991, 2338486]), 648452),
    (simd::i32x2::from([3002132, 3500345]), 645002),
    (simd::i32x2::from([89817, 3512049]), 612800),
    (simd::i32x2::from([1473740, 3283213]), 610364),
    (simd::i32x2::from([3137614, 2929987]), 569712),
    (simd::i32x2::from([2667083, 2286333]), 544471),
    (simd::i32x2::from([424237, 2718408]), 515261),
    (simd::i32x2::from([1944813, 2557878]), 457284),
    (simd::i32x2::from([3699264, 2920959]), 352294),
    (simd::i32x2::from([833202, 92320]), 269964),
    (simd::i32x2::from([2227536, 2152792]), 230525),
    (simd::i32x2::from([3713985, 2744503]), 190559),
    (simd::i32x2::from([2275050, 2067292]), 126095),
    (simd::i32x2::from([3523437, 2746095]), 47939)];

    // Here I spawn the eight threads for my eight-core processor
    let handle1 = thread::spawn(move || main_task(sensors, 0));
    let handle2 = thread::spawn(move || main_task(sensors, 50000));
    let handle3 = thread::spawn(move || main_task(sensors, 1000000));
    let handle4 = thread::spawn(move || main_task(sensors, 1500000));
    let handle5 = thread::spawn(move || main_task(sensors, 2000000));
    let handle6 = thread::spawn(move || main_task(sensors, 2500000));
    let handle7 = thread::spawn(move || main_task(sensors, 3000000));
    let handle8 = thread::spawn(move || main_task(sensors, 3500000));

    // Not quite sure what this does, but I think it's necessary
    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
    handle4.join().unwrap();
    handle5.join().unwrap();
    handle6.join().unwrap();
    handle7.join().unwrap();
    handle8.join().unwrap();
}

// This is the task which gets distributed among the eight cores
// It always processes a 4000000 by 500000 area of the grid, with each thread passing a different
// offset/start index.
fn main_task(beacon_list: [(i32x2, u32); 27], start_index: i32){

    let mut coords: i32x2;
    let mut in_range: bool;
    let first_beacon = beacon_list[0];
    let mut index:usize;

    // This is the main loop of the thread, where the actual computation happens.
    for y in (start_index..start_index+500000).rev(){
        for x in 0:i32..4000001{
            coords = simd::i32x2::from([x, y]);

            // We run a first test of if a beacon is in range instead of initializing the loop value
            // to say that no beacon is in range to save on computation time.
            in_range = is_manhattan(coords, first_beacon);
            index = 0;

            // As long as the square is not in the range of a beacon, we iterate through this loop.
            // Also, the index [of the beacon list] needs to be lower than 26, as that is its length.
            while !in_range && index < 26 {
                index += 1;
                in_range = is_manhattan(coords, beacon_list[index])
            }

            // If we iterated through all beacons and still no one is in range, we have found our square.
            if !in_range {
                println!("HEEEEEEYYYY");
                println!("{}", coords[0]);
                println!("{}", coords[1]);
                println!("{}", simd::SimdInt::reduce_product(coords).to_string());
            }

        }
    }

}

// And this is the function that computes whether a given coordinate is within range of a given beacon
// This is why I used SIMD, so that the X and Y coordinates can be processed in a single step.
fn is_manhattan(own_coords: i32x2, beacon: (i32x2, u32)) -> bool {
    return simd::SimdInt::reduce_sum(simd::SimdInt::abs(own_coords-beacon.0)) <= beacon.1 as i32;
}