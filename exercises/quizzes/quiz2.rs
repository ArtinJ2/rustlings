// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
//
// The exact form of this will be:
// - The input is going to be a Vector of 2-length tuples,
//   the first element is the string, the second one is the command.
// - The output element is going to be a vector of strings.
enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;
    pub fn transformer(input: (String, Command)) -> String { 
        let mut i = 0;
       match input.1 {
        Command::Uppercase => input.0.to_uppercase(),
        Command::Trim => input.0.trim().to_string(),
        Command::Append(count) => {
            let mut s = input.0;
            while i < count {
            i = i+1;    
            s = s + "bar";
            }
            s.to_string()
        }
        
       }
       
    }
    }
    // TODO: Complete the function as described above.
    // pub fn transformer(input: ???) -> ??? { ??? }


fn main() {
    use my_module::transformer;
    let result = transformer(("bar".to_string(), Command::Append(5)));
    println!("{result}");
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    // use ???;
    use super::Command;
    use my_module::transformer;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
