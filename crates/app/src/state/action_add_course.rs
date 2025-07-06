use leptos::prelude::*;

use crate::routes::get_course;

use super::OptionState;

#[derive(Clone)]
pub struct ActionAddCourse(pub Action<String, ()>);

impl ActionAddCourse {
    pub fn from_context() -> Self {
        use_context().unwrap()
    }

    pub fn new(state: OptionState) -> Self {
        let action_courses = Action::new(move |sigle: &String| {
            let sigle = sigle.clone();
            async move {
                if let Ok(c) = get_course(sigle.clone()).await {
                    state.courses.update(|courses| {
                        if !courses.iter().any(|c| c.sigle == sigle) {
                            courses.push(c.into());
                        }
                    });
                    state.submit();
                }
            }
        });
        Self(action_courses)
    }
}
