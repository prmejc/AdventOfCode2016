use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn main(){
  part1();
  println!("\n-----");
  part2();
}

fn part2(){
  let (mut x, mut y) = (0,2);
  let keypad = vec!{
                    vec!{' ', ' ', '1', ' ', ' '},
                    vec!{' ', '2', '3', '4', ' '},
                    vec!{'5', '6', '7', '8', '9'},
                    vec!{' ', 'A', 'B', 'C', ' '},
                    vec!{' ', ' ', 'D', ' ', ' '}
                  };
  let dimension = keypad.len();

  let instructions = get_file_text("./input/keypad_instructions.txt");
  
  for instruction in instructions.lines(){
    for step in instruction.chars(){
      let (a, b) = match step {
        'U' => if x == 0 || keypad[x-1][y] == ' ' { (x, y) } else { ((x-1), y) },
        'L' => if y == 0 || keypad[x][y-1] == ' ' { (x, y) } else { (x, (y-1)) },
        'R' => if y == (dimension-1) || keypad[x][y] == ' ' { (x, y) } else { (x, (y+1)) },
        'D' => if x == (dimension-1) || keypad[x+1][y] == ' ' { (x, y) } else { ((x+1),y) },
         _  => panic!("Unknown step: {}", step),
      };
      x = a;
      y = b;
    };
    print!("{}", keypad[x][y]); 
  }
}

fn part1() {
  let (mut x, mut y) = (1,1);
  let keypad = vec!{vec!{1,2,3},
                    vec!{4,5,6},
                    vec!{7,8,9}};
  let dimension = keypad.len();

  let instructions = get_file_text("./input/keypad_instructions.txt");
  
  for instruction in instructions.lines(){
    for step in instruction.chars(){
      let (a, b) = match step {
        'U' => if x == 0 { (x, y) } else { ((x-1), y) },
        'L' => if y == 0 { (x, y) } else { (x, (y-1)) },
        'R' => if y == (dimension-1) { (x, y) } else { (x, (y+1)) },
        'D' => if x == (dimension-1) { (x, y) } else { ((x+1),y) },
         _  => panic!("Unknown step: {}", step),
      };
      x = a;
      y = b;
    };
    print!("{}", keypad[x][y]); 
  }
}


fn get_file_text(file_name: &str) -> String {
  let path = Path::new(file_name);
  let display = path.display();     
  // Open the path in read-only mode, returns `io::Result<File>`
  let mut file = match File::open(&path) {
      // The `description` method of `io::Error` returns a string that
      // describes the error
      Err(why) => panic!("couldn't open {}: {}", display,
                                                 why.description()),
      Ok(file) => file,
  };

  // Read the file contents into a string, returns `io::Result<usize>`
  let mut s = String::new();
  match file.read_to_string(&mut s) {
      Err(why) => panic!("couldn't read {}: {}", display,
                                                 why.description()),
      Ok(_) => (),
  }
  return s;
}