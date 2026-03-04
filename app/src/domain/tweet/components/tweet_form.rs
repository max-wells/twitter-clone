use icons::{Image, MapPin, Smile};
use leptos::prelude::*;

use crate::components::ui::avatar::Avatar;
use crate::components::ui::button::Button;
use crate::components::ui::textarea::Textarea;
use crate::domain::tweet::tweet_services::PostTweet;

const ALICE_AVATAR: &str = "https://api.dicebear.com/9.x/pixel-art/svg?seed=alice";

#[component]
pub fn TweetForm(post_tweet: ServerMultiAction<PostTweet>) -> impl IntoView {
    let content = RwSignal::new(String::new());
    let char_count = move || content.get().chars().count();
    let is_over_limit = move || char_count() > 260;

    // Reset textarea after successful submission
    Effect::new(move |_| {
        post_tweet.version().get();
        content.set(String::new());
    });

    view! {
        <MultiActionForm action=post_tweet>
            <div class="flex gap-3 px-4 py-3 border-b">
                <Avatar
                    src=Some(ALICE_AVATAR.to_string())
                    alt="Your avatar"
                    class="shrink-0 mt-1"
                />

                <div class="flex flex-col flex-1 gap-3 min-w-0">
                    // Twitter-style borderless textarea
                    <Textarea
                        name="content"
                        placeholder="What's happening?"
                        rows=2u32
                        maxlength=280u32
                        bind_value=content
                        class="resize-none text-lg border-0 shadow-none focus-visible:ring-0 \
                               focus-visible:border-0 px-0 py-0 min-h-0 rounded-none"
                    />

                    // Bottom row: icon strip + char count + Post button
                    <div class="flex items-center justify-between">
                        <div class="flex items-center gap-3 text-primary">
                            <Image class="size-5 cursor-pointer" />
                            <Smile class="size-5 cursor-pointer" />
                            <MapPin class="size-5 cursor-pointer" />
                        </div>
                        <div class="flex items-center gap-3">
                            <span class=move || {
                                if is_over_limit() {
                                    "text-xs text-red-500"
                                } else {
                                    "text-xs text-muted-foreground"
                                }
                            }>{move || format!("{}/280", char_count())}</span>
                            <Button attr:r#type="submit" class="rounded-full px-4">
                                "Post"
                            </Button>
                        </div>
                    </div>
                </div>
            </div>
        </MultiActionForm>
    }
}
