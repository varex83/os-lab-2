use std::cmp::{Ordering};
use std::io::Read;

struct File {
    _name: String,
    size: u64
}

impl Eq for File {}

impl PartialEq<Self> for File {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size
    }
}

impl PartialOrd<Self> for File {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for File {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

fn main() {
    // get argv
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} --parts <NUMBER_OF_PARTS>", args[0]);
        std::process::exit(1);
    }

    let parts = args[2].clone();
    let parts: u64 = parts.parse().unwrap();

    // read all lines in stdin
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    // split input by lines
    let lines: Vec<&str> = input.trim().split("\n").collect();

    let files = lines.iter().map(|line| {
        let parts: Vec<&str> = line.split("::").collect();
        File {
            _name: parts[0].to_string(),
            size: parts[1].parse().unwrap()
        }
    }).collect::<Vec<File>>();


    let mut files: Vec<_> = files.iter().skip_while(|file| file.size == 0).collect();

    let binding = &File {
        _name: "END".to_string(),
        size: 0
    };
    files.push(binding);

    files.sort();

    let mut dp = vec![vec![0f64; files.len()]; parts as usize + 1];
    let mut answer = vec![vec![(-1i32, -1i32); files.len()]; parts as usize + 1];

    // (r - l + 1) / (a[r] - a[l]) -> max
    for i in 1..=parts as usize {
        for j in i..files.len() {
            for k in 0..j {
                let add = (j - k + 1) as f64 / (files[j].size as f64 - files[k].size as f64 + 0.000000001);
                let a = if k > 0 {
                    dp[i - 1][k - 1] + add
                } else {
                    add
                };

                if a > dp[i][j] {
                    dp[i][j] = a;
                    answer[i][j] = (i as i32 - 1, if k > 0 {
                        k as i32 - 1
                    } else {
                        0
                    });
                }
            }
        }
    }

    // re-construct the result and print from a[l] to a[r] there are r - l + 1 files
    let mut result = vec![];
    let mut l = files.len() - 1; // L
    let mut r = files.len() - 1; // R
    let mut pp = parts as usize;
    while l > 0 && r > 0 {
        let (_ll, rr) = answer[pp][r];
        l = r;
        r = rr as usize;
        pp -= 1;
        result.push((r, l));
    }

    for (l, r) in result.iter().rev() {
        let percent = (*r - *l + 1) as f64 / files.len() as f64 * 100.0;
        println!("[{:.2}%]: From {} to {} there are {} files", percent, files[*l].size, files[*r].size, *r - *l + 1);
    }

    println!("Total files: {}", files.len());

    println!("Total size: {}", files.iter().map(|file| file.size).sum::<u64>());
}
