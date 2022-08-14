use std::{fs::*, io::{Write}, io::{ErrorKind, Read}, mem::MaybeUninit, string::FromUtf16Error};

#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate widestring;
use winapi::um::winbase::GetComputerNameW;
use winapi::shared::minwindef::{BOOL, DWORD, LPDWORD};
use winapi::um::winnt::{LPWSTR};
use winapi::shared::ntdef::HANDLE;

#[derive(Debug)]
struct TestStruct(i32);

#[derive(Debug)]
struct TestStruct1(i64);

#[derive(Debug)]
struct Person<'a> {
  fname: &'a str,
  lname: &'a str,
  age: i32,
  addr: &'a str,
  sub: &'a str,
  post: i32,
}

#[derive(Debug)]
enum Make_Err {
  ThisIsATest,
  NotARealError
}

// The derive(Debug) trait allows us to use the standard debug {:?} print in a custom struct.
// The <'a> is a lifetime specifier that tells rust that the struct and each member must live as long as each other.
#[derive(Debug)]
struct UselessStruct<'a> {
  a: &'a str,
  b: &'a str,
  c: &'a str,
}

/**Function calls a Windows api to get the computer name
 * Params:
 *  None
 * Returns void.
 */
fn unsafe_get_windows_host_name() -> () {
  let mut computer_name = "".to_owned();        // Creates a string.
  unsafe {                                              // Windows apis must be called in an unsafe block.
    let mut buffer: [u16; 32] = Default::default();     // Creates a static array of 32 characters.
    let mut len: DWORD = 32;                            
    
    // Calls GetComputerNameW and casts buffer to an LPWSTR pointer.
    // The len must be casted to a pointer to an int as the API will need to modify the length.
    GetComputerNameW(buffer.as_mut_ptr() as LPWSTR, (&mut len as *mut DWORD) as LPDWORD);
    
    // {} is println! is used for inserting values for standard rust types.
    // {:?} is used for debug printing.
    // {:#?} is used for pretty debug printing.
    println!("{:?}{}", buffer, len);

    // Convert the buffer from GetComputerNameW from u16 array to a string.
    let comp_string = String::from_utf16(&buffer);
    match comp_string {
      // As from_utf16 returns a result, a we need to grab the success value and handle the error (if any).
      Ok(t) => { computer_name = t; },
      Err(e) => {println!("{:?}", e);}
    }
  }

  println!("[{}]", computer_name);
}

/**Functi0n gets the current file name of the running executable
 * Params:
 *  None.
 * Returns String.
 */
pub fn get_module_file_name() -> String {
  let mut output = "".to_owned();       // The output buffer.

  unsafe {
    let mut buffer: [u16; 260] = [0; 260];      // A static buffer to hold the module path and name..
    let len: DWORD = 260;                       // The length of the static buffer.

    // Populates the static buffer.
    let ret_len = GetModuleFileNameW(ptr::null_mut(), buffer.as_mut_ptr(), len);
    if ret_len < 1 {
      println!("Unable to retrieve the current module file name");
    }

    // Converts static buffer to String.
    match String::from_utf16(&buffer) {
      Ok(s) => {output = s},
      
      Err(e) => {
        println!("Unable to convert u16 string to rust_string {e}");
      }
    }
  }
  
  output
}

/**Function writes the content of a buffer to a file
 * Params:
 *  file_name: &str {The name of the file.}
 *  content:   &str {The content to write to the file.}
 * Returns void.
 */
fn write_content(file_name: &str, content: &str) -> () {
  let text = content.as_bytes();      // Converts the content to a pointer to a u8 array.
  
  // Opens a file with read and write permissions.
  let write_file = OpenOptions::new().append(true).read(true).open(file_name);

  // Check the Result<File, Error> type for an error.
  match write_file {
    Ok(mut f) => {
      
      // If successful, attempt to write to file.
      match f.write(text) {
        Ok(_) => { println!("Successfully wrote to file"); },
        Err(e) => { println!("Unable to write to file {:?}", e); }
      }
    },
    
    Err(e) => {

      // If unsuccessful, attempt to create the file.
      match File::create("TestFile.txt") {
        Ok(_) => { println!("Successfully created file"); },
        Err(e) => { println!("Unable to create file {:?}", e); }
      }

      // Attempt to open file after it has been created.
      let write_file = OpenOptions::new().append(true).read(true).open("TestFile.txt");
      
      match write_file {
        Ok(mut f) => {

          // If successful, write to the file.
          match f.write(text) {
            Ok(_) => { println!("Successfully wrote to file"); },
            Err(e) => { println!("Unable to write to file {:?}", e); }
          }
        },
        Err(e) => { println!("Unable to open file"); }
      }
    }
  }
}

/**Function reads the content of a file
 * Params:
 *  file_name: &str {The name of the file.}
 * Returns void.
 */
fn read_content(file_name: &str) -> () {
  let read_file = File::open(file_name);
  
  match read_file {
    Ok(mut f) => {
      let mut buffer = "".to_owned();
      
      match f.read_to_string(&mut buffer) {
        Ok(_) => { println!("Buffer = {}", buffer); },
        Err(e) => { println!("Unable to write to file {:?}", e); }
      }
    },
    Err(e) => {}
  }
}

/**Functiob checks if a file exists and uses an if statatement to check for an error instead of a match statement
 * Params:
 *  file_name &str {The name of the file.}
 * Returns bool.
 */
fn read_content_two(file_name: &str) -> bool {
  let read_file = OpenOptions::new().read(true).open(file_name);
  
  if read_file.is_err() { return false; }
  true  // Values can be returned from functions either like this, or by typing {return <VALUE>}
}

/**Function takes a slice, number of characters and an indivudal char as input and outputs a string that looks like the following.
 * ===<Example slice>===
 * Params:
 *  input:  &str {The slice to modify.}
 *  num_ch: u32  {The number of chars to add to the output string.}
 *  ch:     char {The char to repeat.}
 * Returns String.
 */
fn add_line_seps(input: &str, num_ch: u32, ch: char) -> String {
  let mut output = "".to_owned();
  
  // Function uses a closure which is like a mini function can be defined sort of like the javascript function
  // often seen in function params. Eg: function doStuff(function heyThere() {console.log('do stuff');});
  // Parameters in a closure are defined between the || characters.
  let repeat = |chars: u32, chr: char| -> String {
    let mut output = "".to_owned();
    
    for ch in 0..chars {
      output.push(chr);
    }

    return output;
  };

  // Here we concatenate the characters we want to add to the 'input' slice.
  output.push_str(repeat(num_ch, ch).as_str());
  output.push_str(input);
  output.push_str(repeat(num_ch, ch).as_str());
  return output;
}

fn main() {
  let res: bool = read_content_two("TestFile.txt");
  println!("res = {:?}", res);

  write_content("TestFile.txt", "Hey there mate whats your name?\n");
  write_content("TestFile.txt", "I like grapes do you?\n");
  read_content("TestFile.txt");

  // Defines a test struct and print it out using the standard debug print.
  let structure = TestStruct1(284656);
  println!("TestStruct1 = {:?}", structure);

  // Just like any other struct.
  let p = Person {
    fname: "Bob", lname: "Boyle", age: 32, addr: "6/909-102 Happy Road",
    sub: "no se, AUS", post: 3000
  };

  // Prints out the Person struct with pretty debug and standard debug printing.
  println!("p = {:#?}\np = {:?}", p, p);

  // Simpel test closure just formats a string.
  let test_string = |name: &str| -> String {
    let mut output = String::from("Name: [");
    output.push_str(name);
    output.push_str("]");
    return output;
  };

  let name = test_string("That guy");
  println!("The following name was produced by a closure:\n\t{}\n\n", name);

  let random_var = add_line_seps("George the Octopus!! WE LOVE YOU!!!!", 25, '=');
  println!("Another test closure within a function:\n\t{}\n", random_var);

  // Calls the winapi function GetComputerNameW.
  unsafe_get_windows_host_name();
}
