pub mod common;
use common::Timer;

fn solution(path: &str) -> String {
    use intmap::IntMap;
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::*;

    let cpu_count = num_cpus::get_physical();
    let section_count = cpu_count * 2;
    let file_length = std::fs::metadata(path).unwrap().len() as usize;
    let section_size = file_length / section_count;
    let mut map: IntMap<(usize, usize, usize, usize)> = IntMap::new();
    let mut names = HashSet::new();

    std::thread::scope(|s| {
        let threads: Vec<_> = (0..section_count)
            .map(|i| {
                s.spawn(move || unsafe {
                    let mut map: IntMap<(usize, usize, usize, usize)> = IntMap::new();
                    let mut names = HashSet::new();
                    let mut file = File::open(path).unwrap_unchecked();
                    file.seek(SeekFrom::Start((i * section_size) as u64))
                        .unwrap_unchecked();
                    let mut c = [0u8];
                    if i != 0 {
                        while *c.get_unchecked(0) != b'\n' {
                            file.read(&mut c).unwrap_unchecked();
                        }
                    }

                    let mut is_name = true;
                    let mut position = file.stream_position().unwrap_unchecked() as usize;
                    let mut now_name = [0u8; 200];
                    let mut now_name_len = 0;
                    let mut now_num = 0;
                    let to = if i == section_count - 1 {
                        file_length
                    } else {
                        (i + 1) * section_size
                    };
                    loop {
                        let readed_size = file.read(&mut c).unwrap_unchecked();
                        position += 1;
                        match c.get_unchecked(0) {
                            b'\n' => {
                                is_name = true;
                                let name_hash = hash_string(&now_name);
                                match map.entry(name_hash) {
                                    intmap::Entry::Occupied(mut e) => {
                                        let (min, max, sum, count) = e.get_mut();
                                        *min = (*min).min(now_num);
                                        *max = (*max).max(now_num);
                                        *sum += now_num;
                                        *count += 1;
                                    }
                                    intmap::Entry::Vacant(e) => {
                                        names
                                            .insert(String::from_utf8_lossy(&now_name).to_string());
                                        e.insert((now_num, now_num, now_num, 1));
                                    }
                                }
                                now_name_len = 0;
                                now_num = 0;
                                if position >= to || readed_size == 0 {
                                    break;
                                }
                            }
                            b';' => {
                                is_name = false;
                            }
                            _ => {
                                if is_name {
                                    *now_name.get_unchecked_mut(now_name_len) = *c.get_unchecked(0);
                                    now_name_len += 1;
                                } else {
                                    now_num = now_num
                                        .wrapping_mul(10)
                                        .wrapping_add((*c.get_unchecked(0) - b'0') as usize);
                                }
                            }
                        }
                    }
                    (map, names)
                })
            })
            .collect();

        for t in threads {
            let (local_map, local_names) = unsafe { t.join().unwrap_unchecked() };
            println!("joined");
            names.extend(local_names);
            for (hash, (min, max, sum, count)) in local_map {
                match map.entry(hash) {
                    intmap::Entry::Occupied(mut e) => {
                        let (min_, max_, sum_, count_) = e.get_mut();
                        *min_ = (*min_).min(min);
                        *max_ = (*max_).max(max);
                        *sum_ += sum;
                        *count_ += count;
                    }
                    intmap::Entry::Vacant(e) => {
                        e.insert((min, max, sum, count));
                    }
                }
            }
        }
    });

    let mut result = String::new();
    for name in names {
        let (min, max, sum, count) =
            unsafe { map.get(hash_string(name.as_bytes())).unwrap_unchecked() };
        result.push_str(&format!(
            "{}={};{};{}({},{})\n",
            name,
            min,
            max,
            sum / count,
            sum,
            count
        ));
    }

    result
}

fn hash_string(s: &[u8]) -> u64 {
    let mut hash = 0u64;
    for c in s {
        if c == &0 {
            break;
        }
        hash = hash.wrapping_mul(33).wrapping_add(*c as u64);
    }
    hash
}

fn main() {
    let expect_output = std::fs::read_to_string(common::OUTPUT_PATH).unwrap();

    let timer = Timer::new();
    let got = solution(common::MEASUREMENTS_PATH);
    println!("Elapsed: {}ms", timer.elapsed_as_millis());

    assert_eq!(expect_output, got);
}
