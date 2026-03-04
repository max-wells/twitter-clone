use icons::{Ellipsis, Heart, MessageCircle, Share2, Trash2};
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_router::components::A;
use time::OffsetDateTime;

use crate::components::ui::avatar::Avatar;
use crate::components::ui::card::Card;
use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem,
    DropdownMenuTrigger,
};
use backend::domain::tweet_db::Tweet;
use crate::domain::tweet::tweet_services::DeleteTweet;
use crate::domain::user::routing::routes::UserRoutes;

#[component]
pub fn TweetCard(tweet: Tweet, delete_tweet: ServerAction<DeleteTweet>) -> impl IntoView {
    let unid = tweet.unid;
    let is_mine = tweet.is_mine;
    let profile_href = UserRoutes::profile_url(&tweet.author_username);
    let time_label = format_time(&tweet.created_at);
    let initial = tweet.author_display_name.chars().next().unwrap_or('?').to_string();

    let on_delete = move |ev: MouseEvent| {
        ev.stop_propagation();
        delete_tweet.dispatch(DeleteTweet { unid });
    };

    view! {
        <Card class="relative flex gap-3 p-4">
            // "..." dropdown — top-right corner, own tweets only
            {is_mine.then(move || {
                view! {
                    <div class="absolute top-2 right-2">
                        <DropdownMenu>
                            <DropdownMenuTrigger>
                                <Ellipsis class="size-4" />
                            </DropdownMenuTrigger>
                            <DropdownMenuContent>
                                <DropdownMenuGroup>
                                    <DropdownMenuItem>
                                        <DropdownMenuAction
                                            class="text-destructive hover:text-destructive"
                                            on:click=on_delete
                                        >
                                            <Trash2 class="size-4" />
                                            "Delete"
                                        </DropdownMenuAction>
                                    </DropdownMenuItem>
                                </DropdownMenuGroup>
                            </DropdownMenuContent>
                        </DropdownMenu>
                    </div>
                }
            })}

            // Left: avatar
            <Avatar src=tweet.author_avatar_url fallback=initial />

            // Right: content column
            <div class="flex flex-col gap-1 min-w-0 flex-1">
                // Top row: author info
                <div class="flex items-baseline gap-1 text-sm min-w-0 pr-8">
                    <A href=profile_href attr:class="font-semibold hover:underline truncate">
                        {tweet.author_display_name.clone()}
                    </A>
                    <span class="text-muted-foreground shrink-0">
                        {"@"}{tweet.author_username.clone()}
                    </span>
                    <span class="text-muted-foreground shrink-0">"·"</span>
                    <span class="text-muted-foreground shrink-0">{time_label}</span>
                </div>

                // Tweet content
                <p class="text-sm leading-relaxed">{tweet.content.clone()}</p>

                // Action bar
                <div class="flex items-center gap-4 mt-1 text-muted-foreground">
                    <span class="flex items-center gap-1 text-xs">
                        <MessageCircle class="size-3.5" />
                        "0"
                    </span>
                    <span class="flex items-center gap-1 text-xs">
                        <Heart class="size-3.5" />
                        "0"
                    </span>
                    <span class="flex items-center gap-1 text-xs">
                        <Share2 class="size-3.5" />
                    </span>
                </div>
            </div>
        </Card>
    }
}

fn format_time(dt: &OffsetDateTime) -> String {
    format!("{} {}", dt.month(), dt.day())
}
