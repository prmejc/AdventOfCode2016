use util;

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

  let instructions = util::get_file_text("./input/keypad_instructions.txt");
  
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

  let instructions = util::get_file_text("./input/keypad_instructions.txt");
  
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


