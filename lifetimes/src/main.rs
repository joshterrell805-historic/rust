fn main() {
  let x = 5;
  // this is not borrowing
  let mut y = x;
  y += 1;

  let a = &x;
  let ref b = 4;

  println!("{}, {}, {}", y, a, b);

  let mut c = 4;
  f(c);
  println!("c: {}", c);
  g(&mut c);
  println!("c: {}", c);
  h(b);
  h(&x);

  let mut p = 17;
  println!("p: {}", p);
  {
    let q = i(&mut p);
    *q += 2;
    println!("q: {}", q);
  }
  p += 3;
  println!("p: {}", p);

  let d = 7;
  let e = &mut &x;
  println!("e: {}, d: {}", e, d);
  j(e, &d);
  println!("e: {}, d: {}", e, d);
  *e = &p;
  println!("e: {}, d: {}", e, d);
}

fn f(ref mut a : i32) {
  *a = 32;
}

fn g(a : &mut i32) {
  *a = 47;
}

fn h(a : &i32) {
  println!("h(a): {}", a);
}

fn i<'a>(a : &'a mut i32) -> &'a mut i32 {
  a
}

fn j<'a>(a : &mut &'a i32, b : &'a i32) {
  *a = b;
}
