mod student;
use student::Student;


fn main() {
    let mut student = Student::create_student("Iris", "Computer Science");

    student.change_name("Iris Seaman");
    student.introduce_yourself();

    student.change_major("Computer Science PhD");
    student.introduce_yourself();


}
