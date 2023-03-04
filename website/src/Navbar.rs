use yew::prelude::*;

#[function_component]
pub fn NavBar() -> Html {
    html! {
        <div class = "navbar">
            <ul>
                <li class = "li"><a class = "a" href = "index.html">{"Home"} </a></li>
                <li class = "li"><a class = "a" href = "#gmap">{"Map"} </a></li>
                <li class = "li"><a class = "a" href = "https://www.ohio.edu/international-student-scholar-services/health-care">{"Resources"} </a></li>
            </ul>
        </div>
    }
}