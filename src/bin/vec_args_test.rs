// Define the AddArgs struct
#[derive(Debug, PartialEq)]
struct AddArgs {
    a: i32,
    b: i32,
}

// Type alias for AddArgs
type Args = AddArgs;

impl Args {
    // Method to perform addition
    fn add(&self) -> i32 {
        self.a + self.b
    }
}

fn main(){
        let args_vec = vec![
            Args { a: 1, b: 2 },
            Args { a: 3, b: 4 },
            Args { a: 5, b: 6 },
        ];
        let results: Vec<i32> = args_vec.iter().map(|args| args.add()).collect();

        println!("{:?}", results);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_of_args() {
        // Create a vector of Args
        let args_vec = vec![
            Args { a: 1, b: 2 },
            Args { a: 3, b: 4 },
            Args { a: 5, b: 6 },
        ];

        // Collect results of addition
        let results: Vec<i32> = args_vec.iter().map(|args| args.add()).collect();

        // Verify the results
        assert_eq!(results, vec![3, 7, 11]);
    }
}

