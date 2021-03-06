use std::ops::Range;
use std::process::Command;

pub fn find_subsequence<T>(haystack: &[T], needle: &[T], wild_ranges: Option<&Vec<Range<usize>>>) -> Option<usize>
    where T: Eq + Copy
{
    let empty_vec = Vec::new();
    let wild_ranges = &wild_ranges.unwrap_or(&empty_vec);
    haystack.windows(needle.len()).position(|window| {
        matches_with_wildcard(window, needle, wild_ranges)
    })
}

fn matches_with_wildcard<T>(window: &[T], needle: &[T], wild_ranges: &Vec<Range<usize>>) -> bool
    where T: Eq + Copy
{

    if wild_ranges.len() > 0 {
        needle
            .iter()
            .enumerate()
            .filter(|needle_byte|  {
                wild_ranges
                    .iter()
                    .all(|wild_range| !wild_range.contains(&needle_byte.0))
            })
            .all(|needle_byte| window[needle_byte.0] == *needle_byte.1)
    } else {
        window == needle
    }

}

pub fn find_ffxiv() -> Option<i32> {
    let output = Command::new("pgrep")
        .arg("ffxiv_dx11.exe")
        .output()
        .expect("Unable to start pgrep to find ffxiv.");
    if output.status.success() {
        let mut str_pid = String::from_utf8(output.stdout).expect("Unable to parse pgrep output");
        str_pid.remove(str_pid.len() - 1);
        let pid = str_pid.parse::<i32>().expect("Unable to parse pgrep output");
        Some(pid)
    } else {
        None
    }

}