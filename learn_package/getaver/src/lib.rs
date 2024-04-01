pub struct AverCollect {
    list: Vec<i32>,
    aver: f64,
}

impl AverCollect {
    
    pub fn new() -> AverCollect {
        AverCollect{
            list: vec![],
            aver: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.updata_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.updata_average();
                Some(value)
            },
            None => None
        }
    }

    pub fn average(&self) -> f64 {
        self.aver
    }

    pub fn updata_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.aver = total as f64 / self.list.len() as f64;
    }
}