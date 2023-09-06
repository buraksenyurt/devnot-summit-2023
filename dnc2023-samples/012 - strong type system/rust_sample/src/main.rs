use chrono::{DateTime, Utc};

fn main() {
    let mario = User::Inactive {
        name: "Can Cey Mario".to_string(),
    };
    println!("{:#?}", mario);

    if let Ok(m) = mario.activate(Utc::now()) {
        println!("Mario etkinleştirildi");
        println!("{:#?}", m);
    }
}
#[derive(Debug)]
enum User {
    Inactive {
        name: String,
    },
    Active {
        name: String,
        active: bool,
        activation_date: DateTime<Utc>,
    },
}

impl User {
    fn activate(&self, activation_date: DateTime<Utc>) -> Result<Self> {
        match self {
            User::Inactive { name } => {
                let created = User::Active {
                    name: name.clone(),
                    active: true,
                    activation_date: activation_date,
                };
                let _ = created.save();
                Ok(created)
            }
            User::Active { .. } => Err(AlreadyActiveError),
        }
    }

    fn save(&self) -> Result<()> {
        println!("Veri tabanına yazılıyor");
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct AlreadyActiveError;

#[derive(Debug, Clone)]
struct SaveError;

type Result<T> = std::result::Result<T, AlreadyActiveError>;
