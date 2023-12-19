use anyhow::Result;

use super::day_15a::my_hash;

pub fn solve(input: &String) -> Result<String> {
    let mut hash_map: Vec<Vec<(&str, usize)>> = vec![vec![];256];
    input.split(',').for_each(|op| {
        let (label, focal) = op.split_once(['=','-']).unwrap();
        match focal {
            "" => {
                let this_box = &mut hash_map[my_hash(label)];
                let pos = this_box.iter().position(|(l, _f)| *l == label);
                match pos {
                    Some(pos) => {this_box.remove(pos);},
                    None => {}
                }
            },
            num => {
                let this_box = &mut hash_map[my_hash(label)];
                let pos = this_box.iter().position(|(l, _f)| *l == label);
                match pos {
                    Some(pos) => {this_box[pos] = (label, num.parse().expect(""))},
                    None => {this_box.push((label, num.parse().expect("")))}
                }
            }
        }
    });

    let focus: usize = hash_map.iter().enumerate().map(|(i, this_box)| {
        this_box.iter().enumerate().map(|(j, (_label, lens))| {
            (i + 1) * (j + 1) * lens
        }).sum::<usize>()
    }).sum();
    Ok(focus.to_string())
}