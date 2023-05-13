use bevy::prelude::*;

// == Main ==
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PeoplePlugin)
        .run();
}

// == Plugins ==
struct PeoplePlugin;
impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(print_names)
            .add_system(people_with_jobs)
            .add_system(people_without_jobs)
            .add_system(person_does_job);
    }
}

// == Components ==
#[allow(dead_code)]
#[derive(Component)]
struct Person {
    pub name: String,
}

#[derive(Component)]
struct Employed {
    pub job: Job,
}

#[derive(Component, Debug)]
enum Job {
    Engineer,
    Scientist,
    Manager,
}

// == Systems ==
fn setup(mut commands: Commands) {
    println!("Setting up...");
    commands.spawn((
        Person {
            name: "Elaina Proctor".to_string(),
        },
        Employed { job: Job::Engineer },
    ));
    commands.spawn((
        Person {
            name: "Alice Smith".to_string(),
        },
        Employed {
            job: Job::Scientist,
        },
    ));
    commands.spawn((Person {
        name: "Bob Johnson".to_string(),
    },));
    commands.spawn((
        Person {
            name: "Elliot Alderson".to_string(),
        },
        Employed { job: Job::Manager },
    ));
    commands.spawn((
        Person {
            name: "Darlene Alderson".to_string(),
        },
        Employed { job: Job::Engineer },
    ));
}

/// Simple Query example.
fn print_names(query: Query<&Person>) {
    for person in query.iter() {
        println!("Name: {}", person.name);
    }
}

/// Query with filter.
fn people_with_jobs(query: Query<&Person, With<Employed>>) {
    for person in query.iter() {
        println!("Has job: {}", person.name);
    }
}

/// Query with negative filter.
fn people_without_jobs(query: Query<&Person, Without<Employed>>) {
    for person in query.iter() {
        println!("Needs job: {}", person.name);
    }
}

/// Query with multiple components.
fn person_does_job(query: Query<(&Person, &Employed)>) {
    for (person, job) in query.iter() {
        println!("Job: {} - {:?}", person.name, job.job);
    }
}
