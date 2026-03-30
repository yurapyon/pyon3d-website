use leptos::prelude::*;

// ===

#[component]
pub fn Donate() -> impl IntoView {
    // TODO add email and links
    view! {
        <div class="px-2 flex flex-col gap-2">
            <div class="w-full">"Hi! "</div>
            <div class="w-full">
                "This is parcel_ch, the (currently) solo developer working on pyon3d. "
                "I've been making art for a long time and I want to build the kind of tools that would've helped me when I was just starting out. "
                "The systems we live under do not afford the time necessary to learn massive enigmas of software. "
                "We need a future where its easy and fun to share your ideas. "
            // "We need to " <span class="italic">actually</span> " democratize art. "
            // "Not just pretend we are by giving money to billionarie sex-fiends who only want to make you dumber. "
            </div>
            <div class="w-full">
                "I'm currently unemployed due to the AI craze and devaluing of quality human-made work :-) "
                "There's no possible way I could make this happen without some monetary support. "
                "If you want to contribute to this project's future, I have Kofi and Patreon links below. "
                "I'm also looking for long-term sponsors or grants! "
                "If you have any info about that kind of stuff, you can email me at "
            </div>
            <div class="w-full">
                "I have big dreams and want to take this project as far as I can! "
                "Let's do this! "
            </div>
        </div>
    }
}
