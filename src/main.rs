use std::io::{self};
use console::style;
use structopt::StructOpt;
use std::process;
use core::f64::consts::PI;
use core::f64::consts::E;

#[derive(Debug, StructOpt)]
#[structopt(name = "flexulator", about = "A flexible, terminal based calculator written in Rust.")]
struct Flag { // flag struct for use when calling flexulator w/out args
    /// Examples of syntax that works in flexulator 
    #[structopt(short = "x", long = "examples")]
        ex: bool,
    /// Examples of unacceptable syntax that do not work in flexulator
    #[structopt(short, long)]
        dont: bool,
    /// What's new with flexulator
    #[structopt(short, long)]
        new: bool,
    /// Turns off the intro message
    #[structopt(short, long)]
        message: bool,
}

fn main() -> io::Result<()> {
    let flag = Flag::from_args();
    let flexulator = style("Flexulator").green().bold();
    let line_break = style("----------").green().dim().bold();

    if flag.message == false {
        println!("{}: A flexible, terminal based calculator {}", 
            flexulator, "written in Rust.");
        println!("{} accepts a decent amount of syntax {} {}", 
            flexulator, "options depending on where you're from, or how lazy",
            "you are.");
        print!("You may also call {} from the terminal. ",
            flexulator);
        println!("For now, though, {} will accept your input through", flexulator);
        print!("this instance. If there is any math you would");
        println!("like to be done, please type it in {}", style("now.").bold());
        println!("{} {} {} {}.", style("If you would like to quit, type").yellow(), style("q").yellow().bold(), 
            style("or").yellow(), style("quit").yellow().bold());
    } else {
        println!("");
        println!("{}", flexulator);
    }

    if flag.ex == true {
        print_examples();
    }
    if flag.dont == true {
        print_dont();
    }
    if flag.new == true {
        print_new();
    }

    let mut buffer = String::new();
    while &buffer != "q" || &buffer != "quit" { // 
        println!("{}", line_break);
        io::stdin().read_line(&mut buffer)?;
        basic_math(&buffer, 0, 0);
        buffer = String::new();
    }

    Ok(())
}

//fn basic_math(s: &str) { // this is the main function that does all the math.
fn basic_math(s: &str, nums_index: usize, oper_index: usize) {
    // Basics
    let mut nums: Vec<f64> = Vec::new(); // number vector 
    let mut oper: Vec<&str> = Vec::new(); // operator vector
    let mut res: Vec<f64> = Vec::new(); // result vector

    let statement: Vec<&str> = s.split_whitespace().collect();

    // separates nums from operators    
    for i in statement {
        if i == "+" || i == "-" || i == "/" || i == "*" || i == "^" || i == "!" || i == "%" || i == "ln" || i == "log" 
            || i == "sin" || i == "cos" || i == "tan" || i == "arcsin" || i == "arccos" || i == "arctan" || i == "||" 
            || i == "n" || i == "new" || i == "d" || i == "dont" || i == "x" || i == "examples" || i == "rt" 
            || i == "root" || i == "sqrt" || i == "sqr" || i == "e" {
            oper.push(i);
        } else if i == "q" || i == "quit" {
            process::exit(0x0100);

        } else if i == "pi" {
            nums.push(PI);
        } else if i == "e" {
            nums.push(E);
        } else {
            nums.push(i.parse().expect("not an f64!"));
        }
    }

    let mut nums_index = nums_index;
    let mut oper_index = oper_index;
    let bounds = oper.len();
    if bounds == 0 { // prints the number if there are no operators. If there are multiple numbers, it prints the first number.
        res.push(nums[0]);
    }

    while oper_index < bounds {
        match oper[oper_index] {
            "+" => {
                if res.len() == 0 {
                   res.push(nums[nums_index] + nums[nums_index + 1]); 
                } else {
                    res.push(res[nums_index - 1] + nums[nums_index]);
                }
            },
            "-" => {
                if res.len() == 0 {
                   res.push(nums[nums_index] - nums[nums_index + 1]); 
                } else {
                    res.push(res[nums_index - 1] - nums[nums_index]);
                }
            },
            "*" => {
                if res.len() == 0 {
                   res.push(nums[nums_index] * nums[nums_index + 1]); 
                } else {
                    res.push(res[nums_index - 1] * nums[nums_index]);
                }
            },
            "/" => {
                if nums[nums_index + 1] == 0.0 {
                    panic!("Error: Can't divide by zero!");
                }
                if res.len() == 0 {
                   res.push(nums[nums_index] / nums[nums_index + 1]); 
                } else {
                    res.push(res[nums_index - 1] / nums[nums_index]);
                }
            },
            "^" => {
                if res.len() == 0 {
                    res.push(exponent(nums[nums_index], nums[nums_index + 1]));
                } else {
                    res.push(exponent(res[nums_index - 1], nums[nums_index]));
                }
            },
            "!" => { // note: factorial only works on a single number, so it will just take the number and spit it onto res
                if res.len() == 0 {
                    res.push(factorial(nums[nums_index]));
                } else {
                    res.push(factorial(res[nums_index - 1]));
                }
            },
            "%" => {
                if res.len() == 0 {
                    res.push(nums[nums_index] % nums[nums_index + 1]);
                } else {
                    res.push(res[nums_index - 1] % nums[nums_index]);
                }
            },
            "n" | "new" => {
                print_new();
            },
            "x" | "examples" => {
                print_examples();
            },
            "d" | "dont" => {
                print_dont();
            },
            "ln" => {
                if res.len() == 0 {
                   res.push(nums[nums_index].ln()); 
                } else {
                    res.push(res[nums_index - 1].ln());
                }
            },
            "log" => {
                if res.len() == 0 {
                    res.push(nums[nums_index].log(nums[nums_index + 1]));
                } else {
                    res.push(res[nums_index - 1].log(nums[nums_index]));
                }
            },
            "sin" => {
                if res.len() == 0 {
                    res.push(nums[nums_index].sin());
                } else {
                    res.push(res[nums_index - 1].sin());
                }
            },
            "cos" => {
                if res.len() == 0 {
                    res.push(nums[nums_index].cos());
                } else {
                    res.push(res[nums_index - 1].cos());
                }
            },
            "tan" => {
                if res.len() == 0 {
                    res.push(nums[nums_index].tan());
                } else {
                    res.push(res[nums_index - 1].tan());
                }
            },
            "arcsin" => {
                if res.len() == 0 {
                    res.push(nums[nums_index].asin());
                } else {
                    res.push(res[nums_index - 1].asin());
                }
            },
            "arccos" => {
                if res.len() == 0 {
                    res.push(nums[nums_index].acos());
                } else {
                    res.push(res[nums_index - 1].acos());
                }
            },
            "arctan" => {
                if res.len() == 0 {
                    res.push(nums[nums_index].atan());
                } else {
                    res.push(res[nums_index - 1].atan());
                }
            },
            "||" => {
                if res.len() == 0 {
                    res.push(nums[nums_index].abs());
                } else {
                    res.push(res[nums_index - 1].abs());
                }
            },
            "rt" | "root" => {
                if res.len() == 0 {
                    res.push(root(nums[nums_index], nums[nums_index + 1]));
                } else {
                    res.push(root(res[nums_index - 1], nums[nums_index]));
                }
            },
            "sqrt" => {
                if res.len() == 0 {
                    res.push(root(nums[nums_index], 2.0));
                } else {
                    res.push(root(res[nums_index - 1], 2.0));
                }
            },
            "sqr" => {
                if res.len() == 0 {
                    res.push(exponent(nums[nums_index], 2.0));
                } else {
                    res.push(exponent(res[nums_index - 1], 2.0));
                }
            },
            _ => {
                eprintln!("Error: expected operator");
            }
        }
        nums_index = nums_index + 1;
        oper_index = oper_index + 1;
    }

    // fancy syntax
    if res.len() != 0 {
        println!("");
        println!("{} {}", style("=").bold(), style(res[res.len() - 1]).bold()); 
    }
}

fn factorial(n: f64) -> f64 {
    let mut temp_n : f64 = n;
    let mut answer : f64 = 1.0; 

    while temp_n >= 1.0 {
        answer = answer * temp_n;
        temp_n = temp_n - 1.0;
    }
    answer
} 

fn exponent(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        return 1.0;
    } else if y == 1.0 {
        return x;
    }
    x.powf(y) // refactored original function with this: should work with values between 0 and 1
}

fn root(x: f64, y: f64) -> f64 {
    if y == 0.0 {
        eprintln!("Error: 0th root!");
    } else if y == 1.0 {
        return x;
    }
    x.powf(1.0 / y) // easier for user
}

fn print_examples() {
    let flexulator = style("Flexulator").green().bold();
    let line_break = style("----------").green().dim().bold();
    print!("Here are a couple of examples to help you on");
    println!(" your {} journey.", flexulator);
    println!("{}", line_break);
    println!("{}", style("! 5 * - 2 3"));
    println!("");
    println!("{}", style("= 237").bold());
    print!("Every operator and number must have spaces");
    println!(" in between them or else {} doesn't know what", flexulator);
    print!("to do! It's an unfortunate issue that I hope to fix");
    println!(" in future updates.");
    print!("{} currently accepts the basic operators {}", flexulator, style("+, -, *, /").underlined());
    println!(" but it also accepts {}.", style("^, !, %").underlined());
    println!("{}", line_break);
    println!("5 -3 %");
    println!("");
    println!("{}", style("= 2").bold());
    println!("With {} Version 0.1.1, the inclusion of logarithms and scientific functions (sin,", flexulator);
    println!("cos, tan) have been added. Here's how to use them in the context of {}", flexulator);
    println!("{}", line_break);
    println!("{} is pretty self explanatory. Natural log only uses one argument, so don't try to overwhelm it.", style("ln 28").underlined());
    println!("{}", style("= 3.3322046").bold());
    println!("");
    println!("{} is a little more confusing. The first number is the log's base, while the second is the one", style("log 2 10").underlined());
    println!("that gets the logarithm applied to it.");
    println!("{}", style("= 0.30102998").bold());
    println!("Much like the rest of {}. you can use these new functions in \"flexible\" ways.", flexulator);
    println!("{} gives the same result as the previous statement.", style("2 10 log").underlined());
}

fn print_dont() {
    let flexulator = style("Flexulator").green().bold();
    let line_break = style("----------").green().dim().bold();
    println!("Here are some examples that {} work:", style("don't").bold());
    println!("{}", line_break);
    println!("- - - 5 2");
    print!("There are too many operators here. The number of operators needs to be less");
    println!(" than the amount of numbers.");
    print!("There is one exception to this, and that is {}.", style("!").bold());
    println!(" Factorial can be used on every number"); 
    println!("as long as their aren't more Factorial being used than numbers.");
    println!("{}", line_break);
    println!("{} works while {} doesn't.", style("! 5 - 2 !").underlined(), style("! ! ! 9 * 110").underlined());
    println!("");
    println!("There is no 'style' per se that isn't accepted by {}, but there are some rules that {}", flexulator, flexulator);
    println!("follows in order to accurately produce a result. Specifically, it reads from left to right. In the future");
    println!("there may be a way to change this in case you'd prefer to work from right to left. To help show this:");
    println!("{}", line_break);
    println!("5 / 2");
    println!("{}", style("= 2.5").bold());
    println!("");
    println!("Flip it around and...");
    println!("");
    println!("2 / 5");
    println!("{}", style("= 0.4").bold());
    println!("Unlike a standard calculator, {} does not utilize the previous entry. So, for example:", flexulator);
    println!("{}", line_break);
    println!("110 - 109");
    println!("{}", style("= 1").bold());
    println!("{}", style("1 -").underlined());
    println!("The statement directly above will not apply to the previous return value of 1. In the future there will be a");
    println!("flag inwhich you can utilize the previous return value. For now, though, you can't rely on it.");
    println!("{} does not recognize {} as valid characters, so please don't try to use parenthesis", flexulator, style("()").yellow());
    println!("just yet.");
    println!("");
    println!("New to Version 0.1.1, {} now has absolute value!", flexulator);
    println!("The absolute value gets used via {}, which does not apply to the final result unless used last.", style("||").yellow());
    println!("{}", line_break);
    println!("|| 4 - 10");
    println!("{}", style("= -6").bold());
    println!("When you move the absolute value to the end of the statement, however:");
    println!("4 - 10 ||");
    println!("{}", style("= 6").bold());
    println!("Perhaps in the future, a flag will be implemented where absolute value will be applied to the final result");
    println!("whenever used.");
}

fn print_new() {
    let flexulator = style("Flexulator").green().bold();
    let line_break = style("----------").green().dim().bold();
    println!("What's new?");
    println!("Effectively, the update notes for {}!", flexulator);
    println!("{} Versions 0.1.1", flexulator);
    println!("{}", line_break);
    println!("Small update, add a couple of scientific functions that work similarly to Factorial");
    println!("as well as some others that do not. Be careful when calling log, as it will take the first value it sees");
    println!("as the base, and the second as the value it's applying to. sin, cos, tan and their arc varients have");
    println!("been added. Absolute value {} has also been added. Changed functions from f32 to f64 for accuracy.", style("||").yellow());
    println!("");
    println!("{} Version 0.1.0", flexulator);
    println!("{}", line_break);
    println!("Created {}. Reads from left to right, but in any order you wish. Utilizes StructOpt", flexulator);
    println!("for flags. Works with 7 basic operators, but more will be added as time goes on.");
    println!("Written in Rust, by Aaron McKenna (aaronmckenna)");
}
