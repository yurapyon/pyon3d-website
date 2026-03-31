use leptos::prelude::*;

// ===

#[component]
pub fn Info() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col px-2 gap-4">
            <div class="flex flex-col">
                <div class="font-semibold">Project info</div>
                <div class="text-sm">_TODO_</div>
            </div>
            <div class="flex flex-col">
                <div class="font-semibold">Questions</div>
                <div class="flex flex-col gap-2 text-sm">
                    <div>
                        <span class="italic">"Who is the team? "</span>
                        <br />
                        "Its just parcel_ch for now"
                    </div>
                    <div>
                        <span class="italic">"Will there be a cute epic mascot? "</span>
                        <br />
                        "Yes, I'm working on it"
                    </div>
                    <div>
                        <span class="italic">"How much will it cost? "</span>
                        <br />
                        "Not sure yet, but it will be affordable. "
                        "There will also be a Patreon with events and perks and all that"
                    </div>
                    <div>
                        <span class="italic">"When will it be released? "</span>
                        <br />
                        "Hopefully an alpha will be available for Patreon members soon"
                    </div>
                    <div>
                        <span class="italic">"Will it be open source? "</span>
                        <br />
                        "Maybe not... but parts of it may be turned into an open-source library and shared"
                    </div>
                    <div>
                        <span class="italic">"What tech does pyon3d use? "</span>
                        <br />
                        "pyon3d is written in Rust! There is a custom UI engine made with hecs and kasuari, "
                        "parry helps with collisions, and the graphics are SDL2 and OpenGL. This website is "
                        "also written in Rust, using Leptos."
                    </div>
                </div>
            </div>
            <div class="flex flex-col">
                <div class="font-semibold">AI statement</div>
                <div class="flex flex-col gap-2 text-sm">
                    <div>This is a <span class="font-semibold">" no AI zone"</span></div>
                    <span class="underline">
                        "Generative AI tools are not used in developing the app"
                    </span>
                    <ul>
                        <li>"Not for writing code"</li>
                        <li>"Not for planning/internal operations"</li>
                        <li>"Not for research"</li>
                    </ul>
                    <div>
                        <span class="underline">
                            "Generative AI tools will not be integrated into the app out of the box"
                        </span>
                        <br />
                        " I'm planning on adding a plugin system in the future, "
                        "so you can do whatever you want, but third-party AI plugins will not be endorsed by Pyon3D"
                    </div>
                </div>
            </div>
        </div>
    }
}
