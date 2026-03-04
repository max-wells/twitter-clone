use icons::House;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::hooks::use_is_current_path::use_is_current_path;
use crate::components::ui::bottom_nav::{
    BottomNav, BottomNavButton, BottomNavGrid, BottomNavLabel,
};
use crate::routing::home::routes::HomeRoutes;

#[component]
pub fn AppBottomNav() -> impl IntoView {
    let navigate = use_navigate();
    let is_current_path = use_is_current_path();

    view! {
        <BottomNav class="fixed right-0 bottom-0 left-0 sm:hidden">
            <BottomNavGrid>
                <BottomNavButton
                    on:click={
                        let navigate = navigate.clone();
                        move |_| navigate(HomeRoutes::base_url(), Default::default())
                    }
                    attr:aria-current=move || is_current_path(HomeRoutes::base_url())
                >
                    <House />
                    <BottomNavLabel>"Home"</BottomNavLabel>
                </BottomNavButton>
            </BottomNavGrid>
        </BottomNav>
    }
}
