/// Prints "Hello, world!" and demonstrates using traits and structs.
///
/// It creates two structs, `Workshop` and `Seminar`, and a trait `Course` that
/// both structs implement. It then creates instances of `Workshop` and `Seminar` and
/// prints out the details of each course using the trait methods.
fn main() {
    println!("Hello, world!");

    struct Workshop {
        name: String,
        instructor: String,
        duration_hours: u8,
        participants: u32,
    }

    struct Seminar {
        title: String,
        speaker: String,
        topic: String,
        date: String,
        location: String,
    }
    trait Course {
        fn course_name(&self) -> &str;
        fn course_code(&self) -> &str;
        fn credits(&self) -> u8;
    }

    impl Course for Workshop {
        fn course_name(&self) -> &str {
            &self.name
        }
        fn course_code(&self) -> &str {
            "WS101"
        }
        fn credits(&self) -> u8 {
            3
        }
    }

    impl Course for Seminar {
        fn course_name(&self) -> &str {
            &self.title
        }
        fn course_code(&self) -> &str {
            "SM202"
        }
        fn credits(&self) -> u8 {
            2
        }
    }

    let rust_workshop = Workshop {
        name: String::from("Rust Programming"),
        instructor: String::from("Alice"),
        duration_hours: 8,
        participants: 20,
    };
    let ai_seminar = Seminar {
        title: String::from("AI in Modern Applications"),
        speaker: String::from("Bob"),
        topic: String::from("Artificial Intelligence"),
        date: String::from("2024-05-15"),
        location: String::from("Conference Room A"),
    };
    println!(
        "Workshop: {}, Instructor: {}, Duration: {} hours, Participants: {}, Course Code: {}, Credits: {}",
        rust_workshop.course_name(),
        rust_workshop.instructor,
        rust_workshop.duration_hours,
        rust_workshop.participants,
        rust_workshop.course_code(),
        rust_workshop.credits()
    );
    println!(
        "Seminar: {}, Speaker: {}, Topic: {}, Date: {}, Location: {}, Course Code: {}, Credits: {}",
        ai_seminar.course_name(),
        ai_seminar.speaker,
        ai_seminar.topic,
        ai_seminar.date,
        ai_seminar.location,
        ai_seminar.course_code(),
        ai_seminar.credits()
    );

    fn print_course_details<T: Course>(course: &T) {
        println!(
            "Course Name: {}, Course Code: {}, Credits: {}",
            course.course_name(),
            course.course_code(),
            course.credits()
        );
    }
    print_course_details(&rust_workshop);
    print_course_details(&ai_seminar);

    let courses: Vec<Box<dyn Course>> = vec![Box::new(rust_workshop), Box::new(ai_seminar)];

    for course in courses {
        println!(
            "Course Name: {}, Course Code: {}, Credits: {}",
            course.course_name(),
            course.course_code(),
            course.credits()
        );
    }
}
