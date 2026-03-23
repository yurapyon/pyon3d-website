use leptos::prelude::*;

// ===

// Tailwind example
// https://github.com/leptos-rs/leptos/blob/main/examples/tailwind_csr/src/app.rs

// Thaw ui
// https://thawui.vercel.app/

fn main() {
    leptos::mount::mount_to_body(|| {
        view! {
            <div class="w-screen h-screen flex flex-row items-center justify-center bg-[#141414] font-mono">
                <div class="w-60 flex flex-col gap-2">
                    <div class="font-bold text-xl bg-white px-2 py-1 w-full">
                        pyon3d
                    </div>
                    <div class="bg-white px-2 py-1 w-full">
                        a comfy 3d editor
                    </div>
                    <div class="font-bold text-xl bg-yellow-500 px-2 py-1 w-full">
                        under construction
                    </div>
                </div>
            </div>
        }
    })
}
