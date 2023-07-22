use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run();
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup)
        .add_systems(Update, print_names)
        .add_systems(Update, people_with_jobs)
        .add_systems(Update, people_ready_for_hire)
        .add_systems(Update, person_does_job);
    }
}

pub fn setup(mut commands: Commands){

    commands.spawn(
        (Person {
            name: "Luisal".to_string(),
        },
        Employed {
            job: Job::Doctor,
        }) 
    );

    commands.spawn(
        (Person {
            name: "Igoritmos".to_string(),
        },
        Employed {
            job: Job::FireFighter
        }) 
    );

    commands.spawn(
        Person {
            name: "Pão com ovo".to_string(),
        }
    );
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>){
    for person in person_query.iter() {
        println!("{} has a job", person.name);
    }
}

pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>){
    for person in person_query.iter() {
        println!("{} ir ready for hire", person.name);
    }
}

pub fn person_does_job(person_query: Query<(&Person, &Employed)>){
    for (person, employed)in person_query.iter() {
        let job_name: &str = match employed.job {
            Job::Doctor => "Doctor",
            Job::FireFighter => "FireFighter",
            Job::Lawyer => "Lawyer",
        };
        println!("{0} is a {1}.", person.name, job_name)
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String
}

#[derive(Component)]
pub struct Employed {
    pub job: Job
}

pub enum Job {
    Doctor,
    FireFighter,
    Lawyer
}
