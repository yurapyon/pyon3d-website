use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

mod donate;
mod info;

use donate::Donate;
use info::Info;

// ===

// Tailwind example
// https://github.com/leptos-rs/leptos/blob/main/examples/tailwind_csr/src/app.rs

// Thaw ui
// https://thawui.vercel.app/

#[component]
fn Link<'a>(text: &'a str, href: &'a str, deactivated: bool) -> impl IntoView {
    let class = format!(
        "px-2 py-0.5 {}",
        if deactivated {
            "hover:bg-[#707070] hover:text-[#c0c0c0] cursor-not-allowed"
        } else {
            "hover:bg-black hover:text-white"
        }
    );

    view! {
        <a class=class href=href>
            {text}
        </a>
    }
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

#[component]
fn Home() -> impl IntoView {
    // TODO choose a random logo on page refresh
    view! {
        <div class="flex flex-col gap-3">
            <div class="flex flex-row w-full aspect-video bg-black text-white relative">
                <div class="absolute bg-yellow-500 text-black px-2 py-0.5 bottom-3 right-3 z-10">
                    "NOTE: Not a real screenshot of pyon3d"
                </div>
                <img
                    class="object-fill w-full grayscale-60 blur-[1px]"
                    src="blender-screenshot.png"
                />
            </div>
            <div class="w-full px-3 flex flex-col gap-2">
                <div class="grid grid-cols-3 gap-1">
                    <Feature title="Chill experience">
                        "\"Sculpting for low-poly\". Workflow inspried by 2D art tools. "
                        "Clean UIX. Good documentation. "
                    </Feature>
                    <Feature title="For gamedev and art">
                        "Preview shaders and scripts live. Automatically reload assets as you work on them. "
                    </Feature>
                    <Feature title="Not slop">
                        "Quality over quantity. Aim to be responsive to bug reports and requests. "
                        "Human-made everything, no GenAI. "
                    </Feature>
                </div>
                <div class="flex flex-row bg-[#c0c0c0]">
                    <div class="bg-[#b0b0b0] px-1 font-semibold flex flex-row items-center">
                        Roadmap
                    </div>
                    <div class="grow min-w-0 px-1 text-sm h-full flex flex-row items-center">
                        "multilanguage support, high-poly modelling tools, iOS/android/tablet support, plugin system, themable UI"
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();

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
            <Router>
                <div class="w-screen h-screen flex flex-col items-center font-sans relative overflow-clip">
                    <img
                        src="ilp.jpg"
                        class="absolute top-0 left-0 w-full -z-10 brightness-90 contrast-90"
                    />
                    <div class="bg-[#e0e0e0] max-w-[800px] min-h-full flex flex-col gap-3">
                        <div class="flex flex-row gap-2 px-1 pt-1">
                            <div class="flex flex-row border-2 divide-x-2 divide-black grow min-w-0">
                                <div class="px-2 py-0.5 select-none">"status: prealpha"</div>
                                <div class="grow min-w-0" />
                                <Link text="info" href="/info" deactivated=false />
                                <Link text="manual" href="" deactivated=true />
                                <Link text="try/buy" href="" deactivated=true />
                            </div>
                            <div class="flex flex-row items-center gap-2">
                                <a class="w-[20px]" href="/">
                                    <img class="brightness-0" src="patreon_logo.png" />
                                </a>
                                <a
                                    class="w-[20px]"
                                    href="https://bsky.app/profile/pyon3d.com"
                                    target="_blank"
                                >
                                    <img class="brightness-0" src="bsky_logo.png" />
                                </a>
                            </div>
                        </div>
                        <div class="flex flex-row w-full gap-5 items-center px-2">
                            <a class="text-2xl ml-2 w-[100px] group" href="/">
                                <img
                                    class="group-hover:block hidden"
                                    src="color_logo.png"
                                    alt="pyon3d"
                                />
                                <img class="group-hover:hidden" src="logo.png" alt="pyon3d" />
                            </a>
                            <div>"a comfy low-poly editor"</div>
                            <div class="grow min-w-0" />
                            <a
                                class="text-white bg-blue-500 rounded-full px-2 py-0.5 hover:bg-blue-600"
                                href="/donate"
                            >
                                donate
                            </a>
                        </div>
                        <div class="overflow-y-auto">
                            <Routes fallback=|| "Not found.">
                                <Route path=path!("/") view=Home />
                                <Route path=path!("/info") view=Info />
                                <Route path=path!("/donate") view=Donate />
                            </Routes>
                        </div>
                        <div class="grow min-h-0" />
                        <div class="w-full text-center pb-1">
                            made with love by a person in Chicago {" 🌭 "}(c) pyondotmoe 2026
                        </div>
                    </div>
                </div>

            </Router>
        }
    })
}
