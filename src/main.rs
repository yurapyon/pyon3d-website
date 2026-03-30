use leptos::prelude::*;

// ===

// Tailwind example
// https://github.com/leptos-rs/leptos/blob/main/examples/tailwind_csr/src/app.rs

// Thaw ui
// https://thawui.vercel.app/

#[component]
fn Button<'a>(text: &'a str) -> impl IntoView {
    view! { <button class="px-1 py-0.5 text-center">{text}</button> }
}

#[component]
fn Feature<'a>(title: &'a str, children: Children) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full h-full bg-[#b0b0b0]">
            <div class="font-semibold px-1 whitespace-nowrap">{title}</div>
            <div class="bg-[#c0c0c0] h-full text-sm px-1 pb-2">{children()}</div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    // Social links order
    // Patreon, bsky

    /*
    <div class="flex flex-row justify-center w-full">
        <div class="flex flex-row border-2 divide-x-2 ">
            <Button text="try/buy" />
            <Button text="subscribe" />
        </div>
    </div>
    */

    leptos::mount::mount_to_body(|| {
        view! {
            <div class="w-screen h-screen flex flex-col items-center font-sans">
                <div class="bg-[#e0e0e0] max-w-[800px] min-h-full flex flex-col gap-3">
                    <div class="flex flex-row gap-2 px-1 pt-1">
                        <div class="flex flex-row border-2 divide-x-2 grow min-w-0">
                            <div class="px-1 py-0.5">"status: prealpha"</div>
                            <div class="grow min-w-0" />
                            <Button text="info" />
                            <Button text="manual" />
                            <Button text="try/buy" />
                        </div>
                        <Button text="P" />
                        <Button text="B" />
                    </div>
                    <div class="flex flex-row w-full justify-center gap-6 items-center">
                        <div class="text-2xl">pyon3d</div>
                        <div>a comfy 3d editor</div>
                    </div>
                    <div class="flex flex-row w-full aspect-video bg-black text-white relative">
                        <div class="absolute bg-yellow-500 text-black px-1 py-0.5">
                            "NOTE: Not a real screenshot of pyon3d"
                        </div>
                        <img class="object-fill w-full" src="blender-screenshot.png" />
                    </div>
                    <div class="w-full px-3 flex flex-col gap-2">
                        <div class="grid grid-cols-3 gap-1">
                            <Feature title="Quick to use">
                                "\"Sculpting for low-poly\". Workflow inspried by 2D art tools."
                            </Feature>
                            <Feature title="For gamedevs">
                                "Preview shaders live. Automatically reload assets as you work on them. Game loop and interaction
                                scripting. Extensible data model."
                            </Feature>
                            <Feature title="Easy to learn">
                                "Clean UI. Straightforward user experience. Good documentation."
                            </Feature>
                        </div>
                        <div class="w-full flex flex-row items-center bg-[#c0c0c0]">
                            <div class="bg-[#b0b0b0] px-1 font-semibold">Planned</div>
                            <div class="grow min-w-0 px-1 text-sm">
                                "multilanguage support, high-poly modelling tools, iOS/android/tablet, plugin system, themable UI"
                            </div>
                        </div>
                    </div>
                    <div class="grow min-h-0" />
                    <div class="w-full text-center pb-1">
                        made with love by humans in Chicago {" 🌭 "}(c) pyondotmoe 2026
                    </div>
                </div>
            </div>
        }
    })
}
