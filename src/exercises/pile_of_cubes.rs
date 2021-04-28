// Your task is to construct a building which will be a pile of n cubes. The cube at the bottom will have a volume of n^3, the cube above will have volume of (n-1)^3 and so on until the top which will have a volume of 1^3.

// You are given the total volume m of the building. Being given m can you find the number n of cubes you will have to build?

// The parameter of the function findNb (find_nb, find-nb, findNb, ...) will be an integer m and you have to return the integer n such as n^3 + (n-1)^3 + ... + 1^3 = m if such a n exists or -1 if there is no such n.

// Examples:
// findNb(1071225) --> 45

// findNb(91716553919377) --> -1


#[allow(dead_code)]
fn find_nb(n: u64) -> i32 {
  let mut m: u64 = 0;
  let mut volume: u64 = 0;
  // println!("n: {}", n);

  while volume < n {
    m+= 1;
    // println!("{}", m);
    // println!("volume: {}", volume);
    volume += m.pow(3);
  }
  if volume == n {
    m as i32
  }
  else {
    -1
  }
}

// Other people solutions
#[allow(dead_code)]
fn find_nb_2(n: u64) -> i32 {
  let mut sum = 0_u64;
  let l = (0_u64..).take_while(|&x| {sum+=x.pow(3); sum<n}).count() as i32;
  if sum==n {l}
  else {-1}
}


#[allow(dead_code)]
fn find_nb_3(n: u64) -> i32 {
  let k: u64 = ((4.0*n as f64).powf(0.25) - 0.5).ceil() as u64;
  if k*k*(k + 1)*(k + 1) == 4*n {k as i32} else {-1}
}


//Sum of n ^ 3 = n ^ 2 * (n + 1) ^ 2 / 4
//Solve n ^ 2 * (n + 1) ^ 2 / 4 = m, for m
//m = (sqrt(8 * sqrt(n) + 1) - 1) / 2
#[allow(dead_code)]
fn find_nb_4(n: u64) -> i32 {
  let solution = (((n as f64).sqrt() * 8. + 1.).sqrt() - 1.) / 2.;

  if solution.round() == solution {
    solution as i32
  } else {
    -1
  }
}

#[allow(dead_code)]
fn testing(n: u64, exp: i32) -> () {
  assert_eq!(find_nb(n), exp);
}

#[test]
fn basics_find_nb() {
    testing(4183059834009, 2022);
    // testing(24723578342962, -1);
    // testing(135440716410000, 4824);
    // testing(40539911473216, 3568);
    // testing(26825883955641, 3218);
}
