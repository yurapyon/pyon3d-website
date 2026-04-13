use leptos::prelude::*;

// ===

#[component]
pub fn Donate() -> impl IntoView {
    // TODO add email and links
    view! {
        <div class="py-1 px-6 flex flex-col gap-3">
            <div class="font-semibold">"Hi!"</div>
            <div>"This is parcel_ch, the developer working on pyon3d. "</div>
            <div>
                "I've been making art for a long time and I want to build the kind of tools that would've helped me when I was just starting out. "
                "The systems we live under do not afford the time necessary to learn massive enigmas of software. "
                "We need a future where its easy and fun to share our ideas. "
            </div>
            <div>
                "I'm currently unemployed due to the AI craze and devaluing of quality human-made work :-) "
                "There's no way I can make this happen without monetary support. "
                "If you want to contribute to this project's future, I have a Patreon link below. "
                "Outside of that I'm looking for formal sponsorships or grants! Or even a job! "
                "If you have any info about that kind of stuff, you can contact me on Bluesky."
            </div>
            <div>
                "I have big dreams and want to make a positve impact on... something! " <br />
                "Let's do this! "
            </div>
        </div>
    }
}
