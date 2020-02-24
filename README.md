# vega-embed-shim

This is a utility crate to let vega-embed-3.rs to call the vega-embed.js library to render it's spec.

The purpose of this crate:
1. Allow user to reuse their vega-lite chart generated from the vega-lite-3 crate,
2. Render the chart onto their rust/frontend using web_sys::Element as a target.

Additional features:
1. Detecting container size change to live-update the chart size by converting the Promise callback to a rust futures.

Prerequisite:
1. Make sure the required library for vega-embed is loaded in the header of your html file.
2. Make sure you have allocated enough stack memory so that the vega-lite-3 chart won't stuff your wasm memory pool.

---

Run the example:
1. The example is guarded under the example feature, so please enable it.
2. use wasm-pack to build the package: ```wasm-pack build -t web -d public/pkg -- --all-features```. For simplicity purpose, it's using the web target so no bundler is involved.
3. Start a webserver of your choice on the public folder, ```serve public```

[rust wasm frontend demo](https://imgur.com/vPKXTiB)
