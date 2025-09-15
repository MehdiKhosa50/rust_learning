// Description: This is the main entry point for the Rust application.
// Task: Implement a custom iterator for a collection of educational courses, allowing iteration over course names and their respective credits.

fn main() {
    println!("Hello, world!");

    #[derive(Debug)]
    struct Course {
        name: String,
        credits: u8,
    }

    struct CourseCollection {
        courses: Vec<Course>,
    }

    struct CourseIterator<'a> {
        courses: &'a [Course],
        index: usize,
    }

    impl CourseCollection {
        fn new() -> Self {
            CourseCollection { courses: Vec::new() }
        }

        fn add_course(&mut self, name: String, credits: u8) {
            self.courses.push(Course { name, credits });
        }

        fn iter(&self) -> CourseIterator<'_> {
            CourseIterator {
                courses: &self.courses,
                index: 0,
            }
        }
    }

    impl<'a> Iterator for CourseIterator<'a> {
        type Item = (&'a str, u8);

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.courses.len() {
                let course = &self.courses[self.index];
                self.index += 1;
                Some((&course.name, course.credits))
            } else {
                None
            }
        }
    }

    let mut course_collection = CourseCollection::new();
    course_collection.add_course(String::from("Rust Programming"), 3);
    course_collection.add_course(String::from("AI in Modern Applications"), 2);
    course_collection.add_course(String::from("Web Development"), 4);
    course_collection.add_course(String::from("Data Science"), 3);
    course_collection.add_course(String::from("Machine Learning"), 4);
    course_collection.add_course(String::from("Cloud Computing"), 3);
    course_collection.add_course(String::from("Cybersecurity"), 3);
    course_collection.add_course(String::from("Blockchain Technology"), 2);

    for (name, credits) in course_collection.iter() {
        println!("Course: {}, Credits: {}", name, credits);
    }
}
