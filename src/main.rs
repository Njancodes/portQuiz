use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::fs::File;
use std::io;


#[derive(Serialize,Deserialize, Debug)]
struct QuizElement{
    protocol: String,
    ports: u32,
    protocol_type: String,
    full_form: String,
    function: String,
}

struct OptElement{
    a:String,
    b:String,
    c:String,
    d:String,
}

//Make the function to read all objects from the json file - DONE
//Get an array from it and then a random object to print - DONE

//Get the four random objects, pick the answer object and the three wrong ones randomly - DONE
//Frame a question with the answer object - DONE
//Give options with the correct and wrong objects - DONE

//Return a question - DONE
//Random Options - DONE

//Recieve input from user - DONE
//For number of questions - DONE
//Answer - DONE

fn main() {

    let mut n = String::new();
    let mut score = 0;

    println!("Welcome to quiz test of ports from 2.2");
    println!("How many questions do you want ??");

    io::stdin().read_line(&mut n).expect("Something went wrong with input");

    let n: u32 = n.trim().parse().expect("Something went wrong with input type");


    //I am mutating it so it would remove(so the ownership is transfered to the variables) correct object and the wrong objects from the vector
    for n in 1..n+1{
        let file = File::open("obj.json").unwrap();
        let mut eles: Vec<QuizElement> = from_reader(file).unwrap();
        let mut option_eles = option_ele(&mut eles);
        let mut qs_ele = create_qs(&mut option_eles);

        let mut ans = String::new();



        let question = qs_ele.remove(0);
        let answer = qs_ele[0].clone();
        let opt_ele = create_opt(&mut qs_ele);

        println!("{}. {}",n, question);
        println!("Answer is {}",answer);
        println!("A. {}",opt_ele.a);
        println!("B. {}",opt_ele.b);
        println!("C. {}",opt_ele.c);
        println!("D. {}",opt_ele.d);
        io::stdin().read_line(&mut ans).expect("Something went wrong with input");

        match ans.trim() {
            "A" => {
                if opt_ele.a == answer{
                    println!("The answer is correct");
                    score = score + 1;
                }else {
                    println!("The answer aint correct");
                }
            },
            "B" => {
                if opt_ele.b == answer{
                    println!("The answer is correct");
                    score = score + 1;
                }else {
                    println!("The answer aint correct");
                }
            },
            "C" => {
                if opt_ele.c == answer{
                    println!("The answer is correct");
                    score = score + 1;
                }else {
                    println!("The answer aint correct");
                }
            },
            "D" => {
                if opt_ele.d == answer{
                    println!("The answer is correct");
                    score = score + 1;
                }else {
                    println!("The answer aint correct");
                }
            },
            _ => {
                println!("Teach me how you got here, Sensei");
            }
        }

    }

    println!("Your score is {}/{}",score,n);

}

fn option_ele(eles: &mut Vec<QuizElement>) -> Vec<QuizElement>{
    let mut option_ele: Vec<QuizElement> = Vec::new();
    let mut limit = eles.len() - 1;
    let mut chooser = rand_number(limit);
    let correct_obj = eles.remove(chooser);
    limit = eles.len() - 1;
    option_ele.push(correct_obj);
    for _i in 0..3{
        chooser = rand_number(limit);
        let wrong_obj = eles.remove(chooser);
        limit = eles.len() - 1;
        option_ele.push(wrong_obj);
    };
    option_ele
}

fn create_opt(qs_ele: &mut Vec<String>) -> OptElement{
    let opt_alpha = ["A","B","C","D"];
    let mut opt_ele = OptElement{
        a:String::new(),
        b:String::new(),
        c:String::new(),
        d:String::new(),
    };

    for i in opt_alpha{
        let ele = qs_ele.remove(rand_number(qs_ele.len() - 1));
        match i {
            "A" => opt_ele.a = ele.clone(),
            "B" => opt_ele.b = ele.clone(),
            "C" => opt_ele.c = ele.clone(),
            "D" => opt_ele.d = ele.clone(),
            _ => println!("How the hell did you break this ??"),
        }
    }

    opt_ele
}

fn create_qs(opt_eles: &mut Vec<QuizElement>) -> Vec<String>{
    let type_qs = rand_number(2);
    match type_qs {
        0 => {
            //Which [port] is used for {function/protocol} ?
            let mut question = String::new();
            question = format!("Which port is used for {}", opt_eles[0].function);

            let answer_obj = opt_eles.remove(0);
            let wrong_obj_vec = vec![opt_eles.remove(0),opt_eles.remove(0),opt_eles.remove(0)];
            let mut wrong_vec = Vec::new();

            for i in wrong_obj_vec{
                wrong_vec.push(i.ports.to_string()); 
            }

            let answer:String = answer_obj.ports.to_string();

            let quiz_ele = vec![question,answer,wrong_vec[0].clone(),wrong_vec[1].clone(),wrong_vec[2].clone()];

            quiz_ele

        },
        1 => {
            //Is it [tcp/udp] for {function/protocol} ?
            let mut question = String::new();
            question = format!("Is it tcp/udp to {}?", opt_eles[0].function);


            let answer_obj = opt_eles.remove(0);
            let wrong_obj_vec = vec![opt_eles.remove(0),opt_eles.remove(0),opt_eles.remove(0)];
            let mut wrong_vec = Vec::new();

            for i in wrong_obj_vec{
                wrong_vec.push(i.protocol_type); 
            }

            let answer:String = answer_obj.protocol_type;

            let quiz_ele = vec![question,answer,wrong_vec[0].clone(),wrong_vec[1].clone(),wrong_vec[2].clone()];

            quiz_ele
            
        },
        2 => {
            //{tcp/udp with port} is used by which [protocol] ?
            let mut question = String::new();
            question = format!("{}/{} is used by which protocol", opt_eles[0].protocol_type,opt_eles[0].ports);

            let answer_obj = opt_eles.remove(0);
            let wrong_obj_vec = vec![opt_eles.remove(0),opt_eles.remove(0),opt_eles.remove(0)];
            let mut wrong_vec = Vec::new();

            for i in wrong_obj_vec{
                wrong_vec.push(i.protocol); 
            }

            let answer:String = answer_obj.protocol;
            let quiz_ele = vec![question,answer,wrong_vec[0].clone(),wrong_vec[1].clone(),wrong_vec[2].clone()];

            quiz_ele
        },
        3 => {
            //What is the function of this {tcp/udp with port || port} [function]
            let mut question = String::new();
            question = format!("What is the function of this {}/{}", opt_eles[0].protocol_type,opt_eles[0].ports);

            let answer_obj = opt_eles.remove(0);
            let wrong_obj_vec = vec![opt_eles.remove(0),opt_eles.remove(0),opt_eles.remove(0)];
            let mut wrong_vec = Vec::new();

            for i in wrong_obj_vec{
                wrong_vec.push(i.function); 
            }

            let answer:String = answer_obj.function;

            let quiz_ele = vec![question,answer,wrong_vec[0].clone(),wrong_vec[1].clone(),wrong_vec[2].clone()];

            quiz_ele
        }
        _ => {
            vec![String::from("How did you get here ??"),String::from("How did you get here ??"),String::from("How did you get here ??"),String::from("How did you get here ??")]

        }
    } 
}

fn rand_number(limit:usize)->usize{
    //Not inclusive
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=limit).try_into().unwrap()
}
