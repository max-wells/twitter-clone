use icons::Search;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::layout::theme_toggle::ThemeToggle;
use crate::components::ui::avatar::Avatar;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::domain::follow::follow_services::ToggleFollow;
use backend::domain::user_db::UserProfile;
use crate::domain::user::user_services::get_users_to_follow;

#[component]
pub fn RightSidebar() -> impl IntoView {
    let toggle_follow = ServerAction::<ToggleFollow>::new();

    let users_resource = Resource::new(
        move || toggle_follow.version().get(),
        |_| get_users_to_follow(),
    );

    view! {
        <aside class="hidden lg:flex flex-col w-[350px] h-full border-l px-4 py-3 gap-6 overflow-y-auto shrink-0">
            // Search + theme toggle
            <div class="flex items-center gap-4">
                <div class="relative flex-1">
                    <Search class="absolute left-3 top-2.5 size-4 text-muted-foreground pointer-events-none" />
                    <input
                        type="text"
                        placeholder="Search"
                        class="w-full pl-9 pr-4 py-2 rounded-full bg-muted border-0 outline-none text-sm"
                    />
                </div>
                <ThemeToggle />
            </div>

            // Trending
            <div class="rounded-2xl border p-4 flex flex-col gap-3">
                <h2 class="font-bold text-lg">"What's happening"</h2>
                <TrendingItem tag="#leptos" count="1.2K Tweets" />
                <TrendingItem tag="#rust" count="45.3K Tweets" />
                <TrendingItem tag="#tauri" count="8.9K Tweets" />
            </div>

            // Who to follow
            <div class="rounded-2xl border p-4 flex flex-col gap-3">
                <h2 class="font-bold text-lg">"Who to follow"</h2>
                <Transition fallback=|| {
                    view! { <p class="text-sm text-muted-foreground">"Loading..."</p> }
                }>
                    {move || {
                        users_resource
                            .and_then(|users| {
                                users
                                    .iter()
                                    .map(|u| {
                                        view! {
                                            <WhoToFollowItem
                                                user=u.clone()
                                                toggle_follow=toggle_follow
                                            />
                                        }
                                    })
                                    .collect_view()
                            })
                    }}
                </Transition>
            </div>
        </aside>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn TrendingItem(tag: &'static str, count: &'static str) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-0.5 cursor-pointer hover:bg-muted rounded-lg p-2 -mx-2">
            <p class="text-xs text-muted-foreground">"Trending"</p>
            <p class="font-semibold text-sm">{tag}</p>
            <p class="text-xs text-muted-foreground">{count}</p>
        </div>
    }
}

#[component]
fn WhoToFollowItem(
    user: UserProfile,
    toggle_follow: ServerAction<ToggleFollow>,
) -> impl IntoView {
    let following_id = user.unid;
    let is_following = user.is_following;
    let avatar_url = user.avatar_url.clone();
    let display_name = user.display_name.clone();
    let username = user.username.clone();
    let initial = display_name.chars().next().unwrap_or('?').to_string();

    let on_follow = move |ev: leptos::ev::MouseEvent| {
        ev.stop_propagation();
        toggle_follow.dispatch(ToggleFollow { following_id });
    };

    view! {
        <div class="flex items-center gap-3">
            <A href=format!("/{username}") attr:class="shrink-0">
                <Avatar src=avatar_url fallback=initial />
            </A>

            <div class="flex-1 min-w-0">
                <p class="font-semibold text-sm truncate">{display_name}</p>
                <p class="text-xs text-muted-foreground truncate">"@"{username}</p>
            </div>

            <Button
                on:click=on_follow
                variant=if is_following { ButtonVariant::Outline } else { ButtonVariant::Default }
                class="shrink-0"
                size=ButtonSize::Sm
            >
                {if is_following { "Unfollow" } else { "Follow" }}
            </Button>
        </div>
    }
}
