// 定义一个 Student 结构体，包含以下字段：name、age、score
// 实现以下功能：
// - new(name: &str, age: u8, score: f32) -> Student：返回一个新的学生实例。
// - show(&self)：打印 Student 的信息，格式如 Name: Alice, Age: 18, Score: 95.5。
// - is_passed(&self) -> bool：如果 score >= 60.0 则返回 true，否则返回 false。
// 定义 Student 结构体
struct Student {
    name: String,  
    age: u8,       
    score: f32, 
}

impl Student {
    fn new(name: &str, age: u8, score: f32) -> Student {
        Student {
            name: name.to_string(),  // 将 &str 转换为 String
            age,                     // 直接使用传入的 age
            score,                   // 直接使用传入的 score
        }
    }
    
    fn show(&self) {
        println!("Name: {}, Age: {}, Score: {}", self.name, self.age, self.score);
    }
    
    fn is_passed(&self) -> bool {
        self.score >= 60.0  // 返回布尔值，score >= 60.0 为 true，否则为 false
    }
}

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_new() {
        let student = Student::new("Alice", 18, 95.5);
        assert_eq!(student.name, "Alice"); 
        assert_eq!(student.age, 18);
        assert_eq!(student.score, 95.5);
    }
    
    #[test]
    fn test_is_passed() {
        let student1 = Student::new("Bob", 20, 70.0);
        assert!(student1.is_passed());  // score = 70.0，应返回 true
        
        let student2 = Student::new("Charlie", 22, 50.0);
        assert!(!student2.is_passed()); // score = 50.0，应返回 false
    }
}

fn main() {
    let student = Student::new("Alice", 18, 95.5);
    student.show();
}