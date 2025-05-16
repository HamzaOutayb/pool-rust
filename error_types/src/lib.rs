pub type Utc = chrono::Utc;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let date = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(); // rigle format
        Self {
            form_values: (field_name, field_value),
            date,
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        let mut alphabitic = false;
        let mut symbols = false;
        let mut numeric = false;

        for c in self.password.chars() {
            if c.is_ascii_lowercase() || c.is_ascii_uppercase() {
                alphabitic = true;
            } else if c.is_ascii_punctuation() {
                symbols = true;
            } else if c.is_ascii_digit(){
                numeric = true;
            }
        };
        
        if self.name.is_empty() {
            return Err(FormError::new("name",self.name.clone(),"Username is empty"));
        };

        if alphabitic && symbols && numeric && self.password.len() >= 8 {
            return Ok(());
        }; 


        if self.password.len() < 8 {
            return Err(FormError::new("password", self.password.clone(), "Password should be at least 8 characters long"));
        };

        if !alphabitic || !symbols || !numeric {
            return Err(FormError::new("password", self.password.clone(), "Password should be a combination of ASCII numbers, letters and symbols"));
        };

        Ok(())
    }
}