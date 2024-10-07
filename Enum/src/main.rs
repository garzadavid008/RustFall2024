#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}
#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}
#[derive(Debug)]
struct Student {
    name: String,
    grade: GradeLevel,
    major: Major,
}

impl Student {
    fn new(name: String, grade: GradeLevel,major: Major) -> Self {
        Student {
            name: name,
            grade: grade,
            major: major,
        }
    }

    fn introduce_yourself(&self) {
        let grade_msg = match self.grade {
            GradeLevel::Bachelor => "I am bachelor",
            GradeLevel::Master => "I am Master",
            GradeLevel::PhD => "I am PhD",
        };
        let major_msg = match self.major{
            Major::ComputerScience => "I am in computer science",
            Major::ElectricalEngineering => "I am in electrical enginnering",
        };
    
        println!("My name is {:?}", self.name);
        println!("{:?}", grade_msg);
        println!("{:?}", major_msg);
    }
} 



fn main() {
    let s1 = Student::new("John".to_string(), GradeLevel::Bachelor, Major::ComputerScience);
    println!("{:?}", s1);

    Student::introduce_yourself(&s1);
}
