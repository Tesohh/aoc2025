use aoc::{Grid, GridExt, Vec2i, Vec2u};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Tachyon {
    pos: Vec2i,
    encounters: usize,
}

pub fn part2(grid: &Grid<char>, start: &Vec2u) -> usize {
    let start: Vec2i = (*start).try_into().unwrap();

    let mut tachyons: Vec<Tachyon> = vec![Tachyon {
        pos: start + Vec2i::DOWN,
        encounters: 1,
    }];

    for _ in 0..grid.height() {
        let mut new_tachyons: Vec<Tachyon> = vec![];

        for tachyon in &tachyons {
            let candidate = tachyon.pos + Vec2i::DOWN;

            match grid.safe_at(candidate) {
                Some(&'^') => {
                    let candidate_right = candidate + Vec2i::RIGHT;
                    let candidate_left = candidate + Vec2i::LEFT;

                    for candidate in [candidate_right, candidate_left] {
                        match new_tachyons.iter_mut().find(|t| t.pos == candidate) {
                            Some(t) => t.encounters += 1,
                            None => {
                                new_tachyons.push(Tachyon {
                                    pos: candidate,
                                    encounters: 1,
                                });
                            }
                        }
                    }
                }
                Some(&'.') => {
                    new_tachyons.push(Tachyon {
                        pos: candidate,
                        encounters: tachyon.encounters,
                    });
                }
                None => break,
                _ => panic!("invalid char found"),
            };
        }

        if !new_tachyons.is_empty() {
            dbg!(&new_tachyons);
            tachyons = new_tachyons;
        }
    }

    tachyons.iter().map(|t| t.encounters).sum()
}
