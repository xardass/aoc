fn round_score_1(req: &str, res: &str) -> u32 {
    let mut score: u32 = 0;

    match (req, res) {
        ("A", "X") => score += 3,
        ("A", "Y") => score += 6,
        ("A", "Z") => score += 0,
        ("B", "X") => score += 0,
        ("B", "Y") => score += 3,
        ("B", "Z") => score += 6,
        ("C", "X") => score += 6,
        ("C", "Y") => score += 0,
        ("C", "Z") => score += 3,
        _ => (),
    }

    match res {
        "X" => score += 1,
        "Y" => score += 2,
        "Z" => score += 3,
        _ => (),
    }

    score
}

fn round_score_2(req: &str, res: &str) -> u32 {
    let mut score: u32 = 0;

    match (req, res) {
        ("A", "X") => score += 3,
        ("A", "Y") => score += 1,
        ("A", "Z") => score += 2,
        ("B", "X") => score += 1,
        ("B", "Y") => score += 2,
        ("B", "Z") => score += 3,
        ("C", "X") => score += 2,
        ("C", "Y") => score += 3,
        ("C", "Z") => score += 1,
        _ => (),
    }

    match res {
        "X" => score += 0,
        "Y" => score += 3,
        "Z" => score += 6,
        _ => (),
    }

    score
}

pub fn part1(input: &str) -> u32 {
    let mut total_score: u32 = 0;

    let rounds = input.split("\n");

    for round in rounds {
        let v: Vec<&str> = round.split(' ').collect();
        total_score += round_score_1(v.first().unwrap(), v.last().unwrap());
    }
    total_score
}

pub fn part2(input: &str) -> u32 {
    let mut total_score: u32 = 0;

    let rounds = input.split("\n");

    for round in rounds {
        let v: Vec<&str> = round.split(' ').collect();
        total_score += round_score_2(v.first().unwrap(), v.last().unwrap());
    }
    total_score
}
