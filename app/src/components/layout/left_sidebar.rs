use icons::{Bell, House, Mail, Pen};
use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::hooks::use_is_current_path::use_is_current_path;
use crate::components::ui::avatar::Avatar;
use crate::routing::home::routes::HomeRoutes;

#[component]
pub fn LeftSidebar() -> impl IntoView {
    let is_current_path = use_is_current_path();

    let home_active = {
        let icp = is_current_path.clone();
        move || icp(HomeRoutes::base_url()) == "page"
    };

    view! {
        <aside class="hidden sm:flex flex-col w-[72px] xl:w-[275px] h-full border-r py-3 px-2 shrink-0 overflow-y-auto">
            // Logo
            <div class="flex justify-center xl:justify-start px-3 mb-4">
                <A href="/" attr:class="flex items-center gap-3 w-fit">
                    <svg viewBox="0 0 1200 1227" fill="none" xmlns="http://www.w3.org/2000/svg" class="size-7 shrink-0">
                        <path d="M714.163 519.284L1160.89 0H1055.03L667.137 450.887L357.328 0H0L468.492 681.821L0 1226.37H105.866L515.491 750.218L842.672 1226.37H1200L714.137 519.284H714.163ZM569.165 687.828L521.697 619.934L144.011 79.6944H306.615L611.412 515.685L658.88 583.579L1055.08 1150.3H892.476L569.165 687.854V687.828Z" fill="currentColor"/>
                    </svg>
                    <span class="hidden xl:block text-xl font-bold tracking-tight">"Twitter Clone"</span>
                </A>
            </div>

            // Nav items
            <nav class="flex flex-col gap-1">
                <A
                    href=HomeRoutes::base_url()
                    attr:class=move || {
                        let bold = if home_active() { " font-bold" } else { "" };
                        format!(
                            "flex items-center justify-center xl:justify-start gap-4 px-3 py-3 rounded-full hover:bg-muted{bold}"
                        )
                    }
                >
                    <House class="size-6 shrink-0" />
                    <span class="hidden xl:block text-base">"Home"</span>
                </A>

                // Notifications — stub
                <div class="flex items-center justify-center xl:justify-start gap-4 px-3 py-3 rounded-full hover:bg-muted cursor-pointer">
                    <Bell class="size-6 shrink-0" />
                    <span class="hidden xl:block text-base">"Notifications"</span>
                </div>

                // Messages — stub
                <div class="flex items-center justify-center xl:justify-start gap-4 px-3 py-3 rounded-full hover:bg-muted cursor-pointer">
                    <Mail class="size-6 shrink-0" />
                    <span class="hidden xl:block text-base">"Messages"</span>
                </div>
            </nav>

            // Tweet button
            <div class="mt-4 px-2">
                // xl+: full text
                <A
                    href=HomeRoutes::base_url()
                    attr:class="hidden xl:flex w-full rounded-full bg-primary text-primary-foreground font-bold py-3 items-center justify-center hover:bg-primary/90"
                >
                    "Tweet"
                </A>
                // sm–xl: icon only
                <A
                    href=HomeRoutes::base_url()
                    attr:class="xl:hidden flex items-center justify-center rounded-full size-12 bg-primary text-primary-foreground hover:bg-primary/90 mx-auto"
                >
                    <Pen class="size-5" />
                </A>
            </div>

            // Spacer
            <div class="flex-1" />

            // Current user (Alice)
            <A href="/alice" attr:class="flex items-center justify-center xl:justify-start gap-3 p-3 rounded-full">
                <Avatar src=None fallback="A" class="size-9" />
                <div class="hidden xl:block min-w-0">
                    <p class="font-semibold truncate">"Alice"</p>
                    <p class="text-sm text-muted-foreground truncate">"@alice"</p>
                </div>
            </A>
        </aside>
    }
}
