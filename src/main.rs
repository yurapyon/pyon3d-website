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

/*
  return (
    <a
      class={[
        "px-1.5 py-1 text-center text-lmn-slate bg-white",
        props.deactivated
          ? "font-bold bg-lmn-slate text-white pointer-events-none cursor-default"
          : "",
        props.class,
      ].join(" ")}
      href={props.href}
      target={props.target || ""}
    >
      {props.text}
    </a>
  );
*/

#[component]
fn Button<'a>(text: &'a str, href: &'a str) -> impl IntoView {
    view! {
        // <button class="px-2 py-0.5 text-center">{text}</button>
        //
        <a
            class="px-2 py-0.5"
            // [
            // props.deactivated
            // ? "font-bold bg-lmn-slate text-white pointer-events-none cursor-default"
            // : "",
            // props.class,
            // ].join(" ")}
            href=href
        >
            // target={props.target || ""}
            {text}
        </a>
    }
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

#[component]
fn Home() -> impl IntoView {
    // TODO choose a random logo on page refresh
    view! {
        <div class="flex flex-row w-full aspect-video bg-black text-white relative">
            <div class="absolute bg-yellow-500 text-black px-2 py-0.5 bottom-3 right-3 z-10">
                "NOTE: Not a real screenshot of pyon3d"
            </div>
            <img class="object-fill w-full grayscale-60 blur-[1px]" src="blender-screenshot.png" />
        </div>
        <div class="w-full px-3 flex flex-col gap-2">
            <div class="grid grid-cols-3 gap-1">
                <Feature title="Quick to use">
                    "\"Sculpting for low-poly\". Workflow inspried by 2D art tools."
                </Feature>
                <Feature title="For game developers">
                    "Preview shaders live. Automatically reload assets as you work on them. Game loop and interaction
                    scripting. Extensible ECS-based data model."
                </Feature>
                <Feature title="Easy to learn">
                    "Clean UI. Straightforward user experience. Good documentation."
                </Feature>
            </div>
            <div class="w-full flex flex-row items-center bg-[#c0c0c0]">
                <div class="bg-[#b0b0b0] px-1 font-semibold h-full">Planned</div>
                <div class="grow min-w-0 px-1 text-sm h-full">
                    "multilanguage support, high-poly modelling tools, iOS/android/tablet, plugin system, themable UI"
                </div>
            </div>
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
            <Router>
                <div class="w-screen h-screen flex flex-col items-center font-sans relative overflow-clip">
                    <img src="ilp.jpg" class="absolute top-0 left-0 w-full -z-10 brightness-90 contrast-90" />
                    <div class="bg-[#e0e0e0] max-w-[800px] min-h-full flex flex-col gap-3">
                        <div class="flex flex-row gap-2 px-1 pt-1">
                            <div class="flex flex-row border-2 divide-x-2 grow min-w-0">
                                <div class="px-2 py-0.5">"status: prealpha"</div>
                                <div class="grow min-w-0" />
                                <Button text="info" href="/info" />
                                <Button text="manual" href="" />
                                <Button text="try/buy" href="" />
                            </div>
                            <Button text="P" href="" />
                            <Button text="B" href="" />
                        </div>
                        <div class="flex flex-row w-full gap-5 items-center px-2">
                            <a class="text-2xl ml-2" href="/">
                                pyon3d
                            </a>
                            <div>a comfy 3d editor</div>
                            <div class="grow min-w-0" />
                            <a
                                class="text-white bg-blue-500 rounded-full px-2 py-0.5"
                                href="/donate"
                            >
                                donate
                            </a>
                        </div>
                        <Routes fallback=|| "Not found.">
                            <Route path=path!("/") view=Home />
                            <Route path=path!("/info") view=Info />
                            <Route path=path!("/donate") view=Donate />
                        </Routes>
                        <div class="grow min-h-0" />
                        <div class="w-full text-center pb-1">
                            made with love by a human in Chicago {" 🌭 "}(c) pyondotmoe 2026
                        </div>
                    </div>
                </div>

            </Router>
        }
    })
}
