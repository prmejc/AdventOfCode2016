use std::collections::btree_map::BTreeMap;
use util;

pub fn part1(){
  let mut sum = 0;
  let rooms = util::get_file_text("./input/day4.txt");
  for room in rooms.lines(){
     sum += is_a_room(room).1;
  }
  println!("{:?}", sum);
}


fn is_a_room(room: &str) -> (bool, i32) {

  let name = room.split("[").next().unwrap()
                 .split("-")
                 .filter(|&e| match e.parse::<i32>(){
                                                      Ok(_) => false,
                                                      _ => true
                                                    })
                 .collect::<String>();
  let id = room.split("[").next().unwrap().split("-").last().unwrap().parse::<i32>().unwrap();
  let checksum = room.split("[").last().unwrap().replace("]","");

  let mut char_count = BTreeMap::new();
  for letter in name.chars(){
    *char_count.entry(letter).or_insert(1) += 1;
  }

  let mut char_vector: Vec<(char, u16)> = char_count.iter().map(|(&letter, &count)| (letter, count*1000 + (200-(letter as u16)))).collect();
  char_vector.sort_by(|a,b| (b.1).cmp(&a.1));
  let calculated_checksum = char_vector.iter().map(|c| char::to_string(&c.0)).collect::<String>();
  if checksum == calculated_checksum[0..5] {
    return (true, id)
  }
  return (false, 0);
}