use aep_schedule_generator::{
    algorithm::{generation::SchedulesOptions, scores::EvaluationOption},
    data::{course::Course, course_type::CourseType, groups::Groups, time::week::Week},
};
use compact_str::CompactString;
use leptos::*;

#[derive(Copy, Clone)]
pub struct OptionState {
    pub selections: (
        ReadSignal<Vec<ReactiveCourse>>,
        WriteSignal<Vec<ReactiveCourse>>,
    ),
    pub week: [RwSignal<u64>; 5],
    pub max_nb_conflicts: RwSignal<u8>,
    pub day_off: RwSignal<u8>,
    pub morning: RwSignal<i8>,
    pub finish_early: RwSignal<u8>,
}

#[derive(Clone, Debug)]
pub enum ReactiveCourseType {
    TheoOnly {
        theo_open: Vec<RwSignal<bool>>,
        theo_groups: Groups,
    },
    LabOnly {
        lab_open: Vec<RwSignal<bool>>,
        lab_groups: Groups,
    },
    Both {
        theo_open: Vec<RwSignal<bool>>,
        theo_groups: Groups,
        lab_open: Vec<RwSignal<bool>>,
        lab_groups: Groups,
    },
    Linked {
        both_open: Vec<RwSignal<bool>>,
        theo_groups: Groups,
        lab_groups: Groups,
    },
}

#[derive(Clone)]
pub struct ReactiveCourse {
    pub sigle: CompactString,
    pub name: String,
    pub course_type: ReactiveCourseType,
    pub nb_credit: usize,
}

impl Default for OptionState {
    fn default() -> Self {
        Self {
            selections: create_signal(vec![]),
            max_nb_conflicts: create_rw_signal(0),
            week: std::array::from_fn(|_i| create_rw_signal(0)),
            day_off: create_rw_signal(3),
            morning: create_rw_signal(0),
            finish_early: create_rw_signal(1),
        }
    }
}

impl From<ReactiveCourse> for Course {
    fn from(value: ReactiveCourse) -> Self {
        let course_type = match value.course_type {
            ReactiveCourseType::TheoOnly { theo_open, mut theo_groups } => {
                for (i, theo) in theo_open.iter().enumerate() {
                    theo_groups.get_mut(i.into()).unwrap().open = theo.get();
                }
                CourseType::TheoOnly { theo_groups }
            },
            ReactiveCourseType::LabOnly { lab_open, mut lab_groups } => {
                for (i, lab) in lab_open.iter().enumerate() {
                    lab_groups.get_mut(i.into()).unwrap().open = lab.get();
                }
                CourseType::LabOnly { lab_groups }
            },
            ReactiveCourseType::Both { theo_open, mut theo_groups, lab_open, mut lab_groups } => {
                for (i, theo) in theo_open.iter().enumerate() {
                    theo_groups.get_mut(i.into()).unwrap().open = theo.get();
                }
                for (i, lab) in lab_open.iter().enumerate() {
                    lab_groups.get_mut(i.into()).unwrap().open = lab.get();
                }
                CourseType::Both { theo_groups, lab_groups }
            },
            ReactiveCourseType::Linked { both_open, mut theo_groups, mut lab_groups } => {
                for (i, group) in both_open.iter().enumerate() {
                    lab_groups.get_mut(i.into()).unwrap().open = group.get();
                    theo_groups.get_mut(i.into()).unwrap().open = group.get();
                }
                CourseType::Linked { theo_groups, lab_groups }
            },
        };
        Self {
            sigle: value.sigle,
            name: value.name,
            course_type,
            nb_credit: value.nb_credit,
        }
    }
}

impl From<Course> for ReactiveCourse {
    fn from(value: Course) -> Self {
        let course_type = match value.course_type {
            CourseType::TheoOnly { theo_groups } => {
                let theo_open = theo_groups.iter().map(|g| RwSignal::new(g.open)).collect();
                ReactiveCourseType::TheoOnly { theo_open, theo_groups }
            },
            CourseType::LabOnly { lab_groups } => {
                let lab_open = lab_groups.iter().map(|g| RwSignal::new(g.open)).collect();
                ReactiveCourseType::LabOnly { lab_open, lab_groups }
            },
            CourseType::Both { theo_groups, lab_groups } => {
                let theo_open = theo_groups.iter().map(|g| RwSignal::new(g.open)).collect();
                let lab_open = lab_groups.iter().map(|g| RwSignal::new(g.open)).collect();
                ReactiveCourseType::Both { theo_open, theo_groups, lab_open, lab_groups }
            },
            CourseType::Linked { theo_groups, lab_groups } => {
                let both_open = theo_groups.iter().map(|g| RwSignal::new(g.open)).collect();
                ReactiveCourseType::Linked { both_open, theo_groups, lab_groups }
            },
        };

        Self {
            sigle: value.sigle,
            name: value.name,
            course_type,
            nb_credit: value.nb_credit,
        }
    }
}

impl From<&OptionState> for SchedulesOptions {
    fn from(state: &OptionState) -> Self {
        let courses_to_take = state
            .selections
            .0
            .get()
            .into_iter()
            .map(|c| c.into())
            .collect();
        let max_nb_conflicts = state.max_nb_conflicts.get();
        let evaluation = EvaluationOption {
            day_off: state.day_off.get(),
            morning: state.morning.get(),
            finish_early: state.finish_early.get(),
        };
        let user_conflicts = Week::new(state.week.map(|s| s.get()));
        Self {
            courses_to_take,
            max_nb_conflicts,
            evaluation,
            user_conflicts,
            max_size: 5,
        }
    }
}
