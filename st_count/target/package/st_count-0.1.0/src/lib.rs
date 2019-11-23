
pub mod count {
    pub fn space (data:String)->i32{
        let mut counts = 0;
        for i in data.chars() {
            match i {
                ' ' => counts+=1,
                _   => counts=counts,
            }
        }
        counts
    }

    pub fn vovandcon(data:String)->(i32,i32){
        let mut v_counts = 0;
        let mut c_counts = 0;
        for i in data.chars() {
            match i {
                'a'|'e'|'i'|'o'|'u' => v_counts+=1,
                'A'|'E'|'I'|'O'|'U' => v_counts+=1,
                _   => c_counts+=1,
            }
        }
        (v_counts,c_counts)
    }

    pub fn capital(data:String)->i32{
        let mut counts = 0;
        for i in data.chars() {
            if i.is_uppercase() {
                counts+=1;
            }
        }
        counts
    }

    pub fn small(data:String)->i32{
        let mut counts = 0;
        for i in data.chars() {
            if i.is_lowercase() {
                counts+=1;
            }
        }
        counts
    }

    
}