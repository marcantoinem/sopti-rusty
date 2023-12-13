// use crate::backend::routes::get_courses;
// use leptos::*;

// #[component]
// fn CoursesSelector() -> impl IntoView {
//     let (results, set_results) = create_signal(vec![]);
//     let courses = create_resource(|| (), |_| async move { get_courses().await });
//     view! {
//         <div>
//             {move || match once.get().map() {
//                 None => view! { <p>"Loading..."</p> }.into_view(),
//                 Some(data) => view! { <ShowData data/> }.into_view()
//             }}

//             <ul>
//                 {results.iter().map(|s| view!{<li>{s.sigle} {s.name}</li>}).collect_view()}
//             </ul>
//         </div>
//     }
// }
