/*
A couple things this assumes
This file assumes that the headers of the CSV are the same as given in test files
This assumes the file are in the src folder
To Run Install rust
Cd To hatchwaysRS and run cargo run
*/
#[macro_use]
extern crate serde_derive;
extern crate csv;

use std::fs::{File,OpenOptions};
use std::collections::HashMap;
use std::io::Write;


#[derive(Debug,Deserialize)]
struct Course {
    id: usize,
    name: String,
    teacher: String,
}
//struct for holding course imformation

#[derive(Debug, Copy, Clone)]
struct Mark {
    test_id: usize,
    mark: usize,
}
//struct for holding mark information

#[derive(Debug)]
struct Student {
    id: usize,
    name: String,
    list_of_marks: Vec<Mark>,
}
//struct for holding information on student contains a list of thier marks thier Id and name

#[derive(Debug,Deserialize)]
struct Test {
    id: usize,
    course_id: usize,
    weight: usize,
}
//struct for holding test information

#[derive(Debug)]
struct Class {
    id: usize,
    final_mark: f32,
    teacher: String,
    class_name: String,
}
//struct for holding Class information for a student

impl Mark {
    //mark constructor
    fn new(test_id: usize, mark: usize) -> Mark {
        Mark{test_id: test_id, mark: mark}
    }
}

impl Student {
    //Student contructor assumes marks.csv exists
    fn new(id: usize, name: String) -> Student {
        Student{ id: id, name: name, list_of_marks: Student::get_marks(id,String::from("marks.csv"))}


    }
    //creates a list of marks with the id of the student
    fn get_marks(id: usize, file_name: String) -> Vec<Mark> {
        let mut vec = Vec::new();
        let file = File::open(file_name).unwrap();
        let mut rdr = csv::Reader::from_reader(file);
        for result in rdr.records() {
            let record = result.unwrap();
            if record.get(1).unwrap().parse::<usize>().unwrap() == id {
                let mark = Mark::new(record.get(0).unwrap().parse::<usize>().unwrap(),record.get(2).unwrap().parse::<usize>().unwrap());
                vec.push(mark);
            }
        }
        return vec;
    }
}


impl Class {
    //class constuctor
    fn new(id: usize,final_mark: f32, teacher: String, class_name: String) -> Class{
        Class{id:id, final_mark: final_mark, teacher: teacher, class_name: class_name}
    }
}

//function to get teacher of a course based on its id
fn get_teacher(id: usize, courses: &Vec<Course>) -> Option<String> {
    for i in courses.iter(){
        if i.id == id
        {
            return Some(i.teacher.clone());
        }
    }
    None //if this happens something went wrong program will chrash
}
//function to get course based on its id
fn get_course_name(id: usize, courses: &Vec<Course>) -> Option<String> {
    for i in courses.iter(){
        if i.id == id
        {
            return Some(i.name.clone());
        }
    }
    None //If this happens something went wrong program will chrash
}


//this is the main function to genrate the report card 
fn report_card(student: &Student, courses: &Vec<Course>, tests: &Vec<Test>){
    let mut class_list: HashMap<usize, Class> = HashMap::new();// Use a hasmap for easy lookup based on Id

    for mark in student.list_of_marks.iter() {
        for test in tests.iter() {
            if mark.test_id == test.id {
                class_list.entry(test.course_id) // see if class_list has a entry with the key test.course_id
                    .and_modify(|z| {z.final_mark += test.weight as f32 * 0.01 * mark.mark as f32}) //if it exists change final mark to add the new mark and weight
                    .or_insert(Class::new(test.course_id,test.weight as f32* 0.01 * mark.mark as f32, get_teacher(test.course_id,&courses).unwrap(),get_course_name(test.course_id,&courses).unwrap())); // if it doesn't exist create new Class struct with required information
            }
        }
    }

    let mut total_mark: f32 = 0.0;
    let mut print_class_list = Vec::new();//create vector so I can print to file in order of Id
    for (_key,val) in class_list.iter() {
        total_mark += &val.final_mark; // get the sum of the marks
        print_class_list.push(val);
    }
    total_mark /= class_list.len() as f32;
    print_class_list.sort_by_key(|x| x.id);

    let mut file = OpenOptions::new() // Create file reader to create if a file doesnt exist and append if it does
        .write(true)
        .append(true)
        .create(true)
        .open("reportcard.txt")
        .unwrap();

    // the next few line are what write to the file
    if let Err(e) = writeln!(file,"Student Id: {}, name: {}", &student.id,&student.name) {
        eprintln!("error printing {}",e);
    }

    if let Err(e) = writeln!(file,"Total Average:\t{:.2}%\n",&total_mark) {
        eprintln!("error printing {}",e);
    }

    for val in print_class_list.iter(){
        if let Err(e) = writeln!(file,"\tCourse: {}, Teacher: {}\n\tFinal Grade:\t{:.2}%\n",val.class_name,val.teacher,val.final_mark){
            eprintln!("error printing {}",e);
        }
    }
}

fn test() {
    let mut tests = Vec::new();
    let mut courses = Vec::new();
    let mut students = Vec::new();
    // create vector of type Test from tests.csv assumes that it exists and the headers are in order given by test file
    let file = File::open("tests.csv").unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {//Takes CSV file and creates test struct based on headers of csv
        let test: Test = result.unwrap();
        tests.push(test);
    }
    // create vector of type Course from course.csv assumes that it exists and the headers are in order given by test file
    let file_two = File::open("courses.csv").unwrap();
    let mut rdr_two = csv::Reader::from_reader(file_two);
    for result in rdr_two.deserialize() { //Takes CSV file and creates Course struct based on headers of csv
        let course: Course = result.unwrap();
        courses.push(course);
    }
    let student_file = File::open("students.csv").unwrap();
    let mut student_rdr = csv::Reader::from_reader(student_file);
    for result in student_rdr.records() { //Creates student struct from students.csv and marks.csv(as there is a vector or marks in the Student Struct)
        let record = result.unwrap();
        let student = Student::new(record.get(0).unwrap().parse::<usize>().unwrap(),String::from(record.get(1).unwrap()));
        students.push(student);
    }

    for x in students.iter() {
        report_card(x,&courses,&tests)
    }

}


fn main() {
    test();
}
