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
            <div class="font-semibold px-1">{title}</div>
            <div class="bg-[#c0c0c0] h-full text-sm px-1 pb-2">{children()}</div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(|| {
        view! {
            <div class="w-screen h-screen flex flex-col items-center font-sans">
                <div class="bg-[#e0e0e0] max-w-[800px] min-h-full flex flex-col p-1 gap-4">
                    <div class="flex flex-row border-2 divide-x-2">
                        <div class="px-1 py-0.5">status: prealpha</div>
                        <div class="grow min-w-0" />
                        <Button text="info" />
                        <Button text="manual" />
                        <Button text="donate" />
                    </div>
                    <div class="flex flex-row w-full justify-center gap-6 items-center">
                        <div class="text-2xl">pyon3d</div>
                        <div>a comfy 3d editor</div>
                    </div>
                    <div class="flex flex-row w-full aspect-video bg-black text-white relative">
                        <div class="absolute top-1 left-1 bg-yellow-500 text-black px-1 py-0.5">
                            NOTE: Not a real screenshot of pyon3d a comfy 3d editor
                        </div>
                        <img class="object-fill w-full" src="public/blender-screenshot.png" />
                    </div>
                    <div class="grid grid-cols-3 px-2 gap-1">
                        <Feature title="Quick to use">{"\"Sculpting for low-poly\""}</Feature>
                        <Feature title="For gamedevs">
                            Preview shaders live. Automatically reload assets as you work on them. Scripting.
                            Extensible data model.
                        </Feature>
                        <Feature title="Easy to learn">
                            Clean UI. Straightforward user experience
                        </Feature>
                    </div>
                    <div class="flex flex-row gap-4">
                        planned: multilanguage support, highpoly, iOS/android, plugin system
                    </div>
                    <div class="flex flex-row justify-center w-full">
                        <div class="flex flex-row border-2 divide-x-2 ">
                            <Button text="try/buy" />
                            <Button text="subscribe" />
                        </div>
                    </div>
                    <div class="grow min-h-0" />
                    <div class="w-full text-center">
                        made with love BY HUMANS in Chicago {" 🌭"}
                    </div>
                </div>
            </div>
        }
    })
}
