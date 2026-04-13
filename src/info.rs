use leptos::prelude::*;

// ===

#[component]
pub fn Info() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 px-6 py-1">
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
                        <span class="italic">"Who is the team?"</span>
                        <br />
                        "Its just parcel_ch for now"
                    </div>
                    <div>
                        <span class="italic">"How is it pronounced?"</span>
                        <br />
                        "It's \"pyohn\", 'oh' like the 'o' in \"phone\". Its from the Japanese 'jumping' sound: ぴょん"
                    </div>
                    <div>
                        <span class="italic">"Will there be a cute epic mascot?"</span>
                        <br />
                        "Yes, I'm working on it"
                    </div>
                    <div>
                        <span class="italic">"Will it be open source?"</span>
                        <br />
                        "Maybe not... but parts of it may be turned into an open-source library and shared"
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
                        "pyon3d is written in Rust! It's event-based overall, using Tokio. "
                        "Theres a custom UI engine using kasuari. For now the graphics is SDL3 and OpenGL, "
                        "but it's planned to move to wgpu in the future. "
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
                        "needing to ask billionaire sex\u{2011}fiends with no friends "
                        "or Mark\u{2011}fucking\u{2011}Zuckerburg for permission. "
                        <div class="italic mt-0.5 ml-1">"Heres to a hundred thousand more!"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}
