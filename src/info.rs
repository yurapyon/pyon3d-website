use leptos::prelude::*;

// ===

#[component]
pub fn Info() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3 px-6 py-1">
            <div class="flex flex-col">
                <div class="font-semibold bg-pyon-blue-mid px-2 py-0.5">Info</div>
                <div class="text-sm border border-pyon-blue-mid border-t-0 px-2 pt-1 pb-2">
                    "pyon3d is a 3d model editor with the goal of making low\u{2011}poly modelling "
                    "as natural and easy as 2d art or digital sculpting. "
                </div>
            </div>
            <div class="flex flex-col">
                <div class="font-semibold bg-pyon-green-mid px-2 py-0.5">Questions</div>
                <div class="flex flex-col gap-2 text-sm border border-pyon-green-mid border-t-0 px-2 pt-1 pb-2">
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
                <div class="flex flex-row bg-pyon-red-mid px-2 py-0.5 items-center">
                    <div class="font-semibold text-base">AI statement</div>
                    <div class="grow min-w-0" />
                    <div class="text-sm">
                        This is a <span class="font-semibold">" no AI zone"</span>
                    </div>
                </div>
                <div class="grid grid-cols-[auto_40%] text-sm border border-pyon-red-mid border-t-0 px-2 pt-1 pb-2 gap-2">
                    <div class="flex flex-col gap-2">
                        <div>
                            <div class="underline">
                                "Generative AI tools are not used in developing the app"
                            </div>
                            <ul class="list-disc list-inside">
                                <li>"Not for writing code"</li>
                                <li>"Not for planning/internal operations"</li>
                                <li>"Not for research"</li>
                                <li>"Not even for making this website"</li>
                            </ul>
                        </div>
                        <div>
                            <div class="underline">
                                "Generative AI tools will not be integrated into the app by default"
                            </div>
                            <div class="">
                                " I'm planning on adding a plugin system in the future, "
                                "so you can do whatever you want, but third-party AI plugins will never be endorsed by Pyon3D"
                            </div>
                        </div>
                    </div>
                    <div class="rounded-lg bg-pyon-yellow-light px-2 py-1 self-start">
                        "Humans have been making things for hundreds of thousands of years without "
                        "needing to ask billionaire sex fiends with no friends "
                        "or Mark-fucking-Zuckerburg for permission. "
                        <div class="italic mt-0.5 ml-1">"Heres to a hundred thousand more!"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}
