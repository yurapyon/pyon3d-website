use leptos::prelude::*;

// ===

#[component]
pub fn Info() -> impl IntoView {
    view! {
        <div class="w-full px-2 grid grid-cols-2 gap-2">
            <div class="flex flex-col gap-4">
                <div class="flex flex-col">
                    <div class="font-semibold">Info</div>
                    <div class="text-sm">
                        "pyon3d is a 3d model editor with the goal of making low\u{2011}poly modelling "
                        "as natural and easy as 2d art or digital sculpting. "
                    </div>
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
                            <span class="italic">"Will it be open source? "</span>
                            <br />
                            "Probably not... but parts of it may be turned into an open-source library and shared"
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
                            <span class="italic">"What tech does pyon3d use? "</span>
                            <br />
                            "pyon3d is written in Rust! There is a custom UI engine made using kasuari, "
                            "hecs and parry help with 3d stuff, and the graphics in general is SDL2 and OpenGL. "
                            "This website is also written in Rust, using Leptos."
                        </div>
                    </div>
                </div>
            </div>
            <div class="flex flex-col">
                <div class="font-semibold">AI statement</div>
                <div class="flex flex-col gap-2 text-sm">
                    <div>This is a <span class="font-semibold">" no AI zone"</span></div>
                    <div>
                        <span class="underline">
                            "Generative AI tools are not used in developing the app"
                        </span>
                        <ul class="list-disc list-inside">
                            <li>"Not for writing code"</li>
                            <li>"Not for planning/internal operations"</li>
                            <li>"Not for research"</li>
                            <li>"Not even for making this website"</li>
                        </ul>
                    </div>
                    <div>
                        <span class="underline">
                            "Generative AI tools will not be integrated into the app out\u{2011}of\u{2011}the\u{2011}box"
                        </span>
                        <br />
                        " I'm planning on adding a plugin system in the future, "
                        "so you can do whatever you want, but third-party AI plugins will never be endorsed by Pyon3D"
                    </div>
                    <div>
                        "Humans have been making things for hundreds of thousands of years without "
                        "needing to ask billionaire sex\u{2011}fiends with no friends, billionaire incestuous rapists, "
                        "or Mark\u{2011}fucking\u{2011}Zuckerburg for permission. "
                        <div class="italic mt-0.5 ml-1">"Heres to a hundred thousand more!"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}
