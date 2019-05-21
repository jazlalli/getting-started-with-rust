use std::io;

fn convert (opt: i32, value: f32) -> f32 {
  let mut result: f32 = 0.0;

  if opt == 1 {
    // Celsius to Fahrenheit
    result = (value * 9.0/5.0) + 32.0;
  } else if opt == 2 {
    // Fahrenheit to Celsius
    result = (value - 32.0) * 5.0/9.0;
  };

  result
}

fn get_unit(opt: i32, switch: bool) -> String {
  let mut unit = String::new();

  if switch {
    if opt == 1 {
      unit = "Fahrenheit".to_string()
    } else if opt == 2 {
      unit = "Celsius".to_string()
    }
  } else {
    if opt == 1 {
      unit = "Celsius".to_string()
    } else if opt == 2 {
      unit = "Fahrenheit".to_string()
    }
  }

  unit
}

fn main() {
    // which way do you want to convert
    println!("Select conversion:");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");

    let mut conversion = String::new();

    io::stdin().read_line(&mut conversion)
        .expect("Failed to read option for conversion");

    let _conversion: i32 = conversion.trim().parse().unwrap();

    if _conversion < 1 || _conversion > 2 {
      println!("Invalid option. Exiting...");
      return;
    }

    // get value and do the conversion
    let mut value = String::new();
    println!("Enter the temperature in {}:", get_unit(_conversion, false));
    io::stdin().read_line(&mut value)
        .expect("Failed to read input value");

    let _value = value.trim().parse::<f32>().unwrap();

    println!("The converted value is: {} {}", convert(_conversion, _value), get_unit(_conversion, true));
}
