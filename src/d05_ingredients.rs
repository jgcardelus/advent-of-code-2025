use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    ops::Range,
    path::Path,
};

#[derive(Debug, PartialEq)]
enum SearchResult {
    Inside(usize),
    Outside(usize),
}

impl SearchResult {
    fn unwrap(&self) -> usize {
        match self {
            SearchResult::Inside(i) | SearchResult::Outside(i) => *i,
        }
    }
}

enum RelativeToRange {
    Left,
    In,
    Right,
}

fn position_relative_to_range(id: u128, range: &Range<u128>) -> RelativeToRange {
    match range {
        _ if id >= range.start && id <= range.end => RelativeToRange::In,
        _ if id < range.start => RelativeToRange::Left,
        _ => RelativeToRange::Right,
    }
}

pub fn find_id(id: u128, ids: &[Range<u128>]) -> SearchResult {
    find_id_from_range(
        id,
        ids,
        Range {
            start: 0,
            end: ids.len(),
        },
    )
}

fn find_id_from_range(id: u128, ids: &[Range<u128>], sliced: Range<usize>) -> SearchResult {
    assert!(sliced.start <= ids.len());
    assert!(sliced.end <= ids.len());

    if sliced.start == sliced.end {
        let checked_id = match ids.get(sliced.start) {
            Some(id) => id,
            None => return SearchResult::Outside(sliced.start),
        };

        return match position_relative_to_range(id, checked_id) {
            RelativeToRange::In => SearchResult::Inside(sliced.start),
            RelativeToRange::Left => SearchResult::Outside(sliced.start),
            RelativeToRange::Right => SearchResult::Outside(sliced.start + 1),
        };
    }

    let middle = ((sliced.end - sliced.start) / 2usize) + sliced.start;
    let middle_id = &ids[middle];

    match position_relative_to_range(id, middle_id) {
        RelativeToRange::In => SearchResult::Inside(middle),
        RelativeToRange::Left => find_id_from_range(
            id,
            ids,
            Range {
                start: sliced.start,
                end: middle,
            },
        ),
        RelativeToRange::Right => find_id_from_range(
            id,
            ids,
            Range {
                start: middle + 1,
                end: sliced.end,
            },
        ),
    }
}

fn insert_range(ids: &mut Vec<Range<u128>>, range: Range<u128>) {
    assert!(range.start <= range.end);

    let start = find_id(range.start, ids);
    let end = find_id(range.end, ids);

    match (&start, &end) {
        (SearchResult::Inside(start), SearchResult::Inside(end)) => {
            ids[*start].end = ids[*end].end;
        }
        (SearchResult::Inside(start), SearchResult::Outside(_)) => {
            ids[*start].end = range.end;
        }
        (SearchResult::Outside(_), SearchResult::Inside(end)) => {
            ids[*end].start = range.start;
        }
        (SearchResult::Outside(start), SearchResult::Outside(_)) => {
            ids.insert(*start, range);
        }
    }

    if &start.unwrap() < &end.unwrap() {
        match (&start, &end) {
            (SearchResult::Inside(start), SearchResult::Inside(end))
            | (SearchResult::Outside(start), SearchResult::Outside(end)) => {
                ids.drain((*start + 1)..(*end + 1));
            }
            (_, _) => {
                ids.drain((start.unwrap() + 1)..end.unwrap());
            }
        }
    }
}

pub fn find_valid_ids(ids: Vec<u128>, ranges: &Vec<Range<u128>>) -> u128 {
    ids.iter()
        .map(|id| match find_id(*id, &ranges) {
            SearchResult::Inside(_) => 1,
            SearchResult::Outside(_) => 0,
        })
        .reduce(|acc, result| acc + result)
        .unwrap()
}

pub fn get_total_fresh(ranges: &Vec<Range<u128>>) -> u128 {
    ranges
        .iter()
        .map(|range| range.end - range.start + 1)
        .reduce(|acc, result| acc + result)
        .unwrap()
}

pub fn read_ids(path: &Path) -> (Vec<Range<u128>>, Vec<u128>) {
    let file = File::open(&path).expect("Error while opening file");
    let reader = BufReader::new(file);

    let mut ranges: Vec<Range<u128>> = Vec::new();
    let mut ids: Vec<u128> = Vec::new();

    let mut reading_ranges = true;
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let line = line.trim();
        if line.is_empty() {
            reading_ranges = false;
            continue;
        }

        if reading_ranges {
            let chunks = line.split("-").collect::<Vec<&str>>();
            insert_range(
                &mut ranges,
                Range {
                    start: chunks[0].parse().unwrap(),
                    end: chunks[1].parse().unwrap(),
                },
            );
        } else {
            ids.push(line.parse().unwrap())
        }
    }

    (ranges, ids)
}

#[cfg(test)]
mod test {
    use std::ops::Range;

    use crate::d05_ingredients::{SearchResult, find_id, insert_range};

    #[test]
    fn test_find_id() {
        let ids = vec![
            Range { start: 3, end: 5 },
            Range { start: 7, end: 9 },
            Range { start: 11, end: 15 },
            Range { start: 17, end: 20 },
        ];

        assert_eq!(find_id(4, &ids), SearchResult::Inside(0));
        assert_eq!(find_id(8, &ids), SearchResult::Inside(1));
        assert_eq!(find_id(12, &ids), SearchResult::Inside(2));
        assert_eq!(find_id(17, &ids), SearchResult::Inside(3));
        assert_eq!(find_id(21, &ids), SearchResult::Outside(4));
        assert_eq!(find_id(0, &ids), SearchResult::Outside(0));
        assert_eq!(find_id(10, &ids), SearchResult::Outside(2));
    }

    #[test]
    fn test_insert_range() {
        let mut ids = vec![Range { start: 3, end: 5 }];

        insert_range(&mut ids, Range { start: 4, end: 6 });
        assert_eq!(ids, vec![Range { start: 3, end: 6 }]);
        insert_range(&mut ids, Range { start: 1, end: 2 });
        assert_eq!(
            ids,
            vec![Range { start: 1, end: 2 }, Range { start: 3, end: 6 }]
        );
        insert_range(&mut ids, Range { start: 7, end: 8 });
        assert_eq!(
            ids,
            vec![
                Range { start: 1, end: 2 },
                Range { start: 3, end: 6 },
                Range { start: 7, end: 8 }
            ]
        );
        insert_range(&mut ids, Range { start: 0, end: 10 });
        assert_eq!(ids, vec![Range { start: 0, end: 10 },]);
        insert_range(&mut ids, Range { start: 12, end: 14 });
        insert_range(&mut ids, Range { start: 14, end: 18 });
        assert_eq!(
            ids,
            vec![Range { start: 0, end: 10 }, Range { start: 12, end: 18 }]
        );
        insert_range(&mut ids, Range { start: 13, end: 19 });
        assert_eq!(
            ids,
            vec![Range { start: 0, end: 10 }, Range { start: 12, end: 19 }]
        );
    }
}
