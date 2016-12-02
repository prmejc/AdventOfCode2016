use std::collections::btree_map::{BTreeMap, Entry};
#[derive(Debug)]
struct Position{
  x: i32,
  y: i32,
}

impl Position{
  fn id(&self) -> String {
    format!("{},{}", &self.x, &self.y)
  }
}

fn printSolution(startPosition:Position, endPosition:Position){
    let distance = (startPosition.x - endPosition.x).abs() + 
                   (startPosition.y - endPosition.y).abs();
    println!("Starting point: {:?}", startPosition);
    println!("   Finished at: {:?}", endPosition);
    println!("   Distance is: {:?}", distance);
}

pub fn main() {
    
    let directions = "R4, R4, L1, R3, L5, R2, R5, R1, L4, R3, L5, R2, L3, L4, L3, R1, R5, R1, L3, L1, R3, L1, R2, R2, L2, R5, L3, L4, R4, R4, R2, L4, L1, R5, L1, L4, R4, L1, R1, L2, R5, L2, L3, R2, R1, L194, R2, L4, R49, R1, R3, L5, L4, L1, R4, R2, R1, L5, R3, L5, L4, R4, R4, L2, L3, R78, L5, R4, R191, R4, R3, R1, L2, R1, R3, L1, R3, R4, R2, L2, R1, R4, L5, R2, L2, L4, L2, R1, R2, L3, R5, R2, L3, L3, R3, L1, L1, R5, L4, L4, L2, R5, R1, R4, L3, L5, L4, R5, L4, R5, R4, L3, L2, L5, R4, R3, L3, R1, L5, R5, R1, L3, R2, L5, R5, L3, R1, R4, L5, R4, R2, R3, L4, L5, R3, R4, L5, L5, R4, L4, L4, R1, R5, R3, L1, L4, L3, L4, R1, L5, L1, R2, R2, R4, R4, L5, R4, R1, L1, L1, L3, L5, L2, R4, L3, L5, L4, L1, R3";


    let mut orientation = 'N';
    let startPosition = Position{x:0,y:0};
    let mut position = Position{x: startPosition.x, y: startPosition.y};
    let mut visitedLocations = BTreeMap::new();
    visitedLocations.insert(position.id(), 0);
    'outer: 
    for direction in directions.split(", ") {

      orientation = match (direction.chars().next().expect("Input error"), orientation) {
        ('L','N') => 'W',
        ('L','W') => 'S',
        ('L','S') => 'E',
        ('L','E') => 'N',
        ('R','N') => 'E',
        ('R','E') => 'S',
        ('R','S') => 'W',
        ('R','W') => 'N',
        uknownDirection => panic!("Direction uknown! {:?}", uknownDirection)
      };

      let steps = direction[1..].parse::<i32>().unwrap();
      for _ in 0..steps {   
        position = match orientation {
          'N' => Position{x: position.x, y: position.y + 1},
          'E' => Position{x: position.x + 1, y: position.y},
          'S' => Position{x: position.x, y: position.y - 1},
          'W' => Position{x: position.x - 1, y: position.y},
          x => panic!("Orientation uknown! {:?}", x)
        };

        if let Entry::Occupied(_) = visitedLocations.entry(position.id()){
          break 'outer;
        }
        visitedLocations.insert(position.id(), 0);
      }
    }

    printSolution(startPosition, position);
  
}                  

