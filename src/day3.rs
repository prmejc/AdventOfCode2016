use util;

pub fn part1(){
  let triangles = util::get_file_text("./input/triangles.txt");

  let mut triangles_count = 0;

  for instruction in triangles.lines(){
    let mut vec = instruction.split(" ").collect::<Vec<&str>>();
    vec.retain(|&x| x != "");
    let ints = vec.iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    if ints[0] + ints[1] > ints[2] &&
       ints[0] + ints[2] > ints[1] &&
       ints[1] + ints[2] > ints[0]
    {
      triangles_count += 1;
    }   
  }
  println!("{:?}", triangles_count);
}

pub fn part2(){
  let triangles = util::get_file_text("./input/triangles.txt");

  let mut triangles_count = 0;

  let mut line_count = 0;
  let mut ints1 = vec![];
  let mut ints2 = vec![];
  for instruction in triangles.lines(){
    let mut vec = instruction.split(" ").collect::<Vec<&str>>();
    vec.retain(|&x| x != "");
    let ints = vec.iter().map(|&x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    if line_count % 3 == 0 {
      ints1 = ints;
    }
    else if line_count % 3 == 1 {
      ints2 = ints;
    }
    else {
       for col in 0..3 {
         if ints1[col] + ints2[col] > ints[col] &&
            ints1[col] + ints[col]  > ints2[col]  &&
            ints[col]  + ints2[col] > ints1[col]
          {
            triangles_count += 1;
          }
      }
    }
    line_count+=1;
  }
  println!("{:?}", triangles_count);
}