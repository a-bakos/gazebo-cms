// This will be the admin (management) area

use yew::prelude::*;

use crate::components::admin_bar::AdminBar;
use crate::components::nav::Nav;

use crate::app::MainNavigationRoute;
use yew_router::prelude::Link;

#[function_component(GazeboAdminArea)]
pub fn gazebo_admin_area() -> Html {
    html! {
        <main id={"gb-gui-admin"}>
            <AdminBar />

            <div class={"gb-admin-panel"}>
                <nav class={"gb-admin-menu"}>
                    <ul>
                        <li>
                            <Link<MainNavigationRoute> classes={classes!("testclass")} to={MainNavigationRoute::Admin}>
                                {"admin"}
                            </Link<MainNavigationRoute>>
                        </li>
                        <li><a href="">{"Posts"}</a></li>
                        <li><a href="">{"Media"}</a></li>
                        <li><a href="">{"Settings"}</a></li>
                        <li><a href="">{"Users"}</a></li>
                    </ul>
                </nav>

                <div class={"gb-admin-main"}>
                   <h1>{"Posts"}</h1>
                    <table class={"gb-admin-table"}>
                        <thead>
                            <tr>
                                <th>{"Title"}</th>
                                <th>{"Status"}</th>
                                <th>{"Author"}</th>
                                <th>{"Category"}</th>
                                <th>{"Published"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>{"Title of post 1"}</td>
                                <td>{"Published"}</td>
                                <td>{"admin-xyz"}</td>
                                <td>{"None"}</td>
                                <td>{"2023-06-06 22:49"}</td>
                            </tr>
                            <tr>
                                <td>{"Gazebo Post Example Title"}</td>
                                <td>{"Draft"}</td>
                                <td>{"editor-xyz"}</td>
                                <td>{"Announcements"}</td>
                                <td>{"2023-06-02 11:53"}</td>
                            </tr>
                            <tr>
                                <td>{"Title of post 1"}</td>
                                <td>{"Private"}</td>
                                <td>{"admin-xyz"}</td>
                                <td>{"None"}</td>
                                <td>{"2023-06-06 22:49"}</td>
                            </tr>
                            <tr>
                                <td>{"Gazebo Post Example Title"}</td>
                                <td>{"Private"}</td>
                                <td>{"editor-xyz"}</td>
                                <td>{"Announcements"}</td>
                                <td>{"2023-06-02 11:53"}</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>

        </main>
    }
}
