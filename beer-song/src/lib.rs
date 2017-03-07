fn upper_first(s: &str) -> String {
  let mut c = s.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + c.as_str()
  }
}

fn do_verse1(num: u32) -> String {
  match num {
    0 => "no more bottles".to_string(),
    1 => "1 bottle".to_string(),
    _ => num.to_string() + " bottles",
  }
}

fn do_verse2(num: u32) -> (String, String) {
  match num {
    0 => 
      ("Go to the store and buy some more".to_string(), "99 bottles".to_string()),
    _ => 
      {
        let sth = if num == 1 { "it" } else { "one" };
        ("Take ".to_string() + sth + " down and pass it around", do_verse1(num - 1))
      },
  }
}

pub fn verse(num: u32) -> String {
  let str1 = do_verse1(num);
  let (str2, str3) = do_verse2(num);

  upper_first(&str1) + " of beer on the wall, " + 
  &str1 + " of beer.\n" + 
  &str2 + ", " + 
  &str3 + " of beer on the wall.\n"
}


pub fn sing(num1: u32, num2: u32) -> String {
  let result = verse(num1);
  if num1 == num2 {
    result
  } else {
    result + "\n" + &sing(num1 - 1, num2)
  }
}